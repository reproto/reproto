use crate::build_yaml;
use crate::languages::{Instance, Language, Languages};
use crate::reproto::{Manifest, Reproto};
use crate::suite::Suite;
use crate::utils;
use crate::{timed_run, BoxFuture, Error, JsonMismatch, PreBuild, Runner, TimedRun};
use anyhow::{bail, format_err, Context as _, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use tokio::fs;
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt as _, AsyncReadExt as _, AsyncWriteExt as _};
use tokio::process::{ChildStderr, ChildStdin, ChildStdout};
use tokio::time;

/// Setup projects.
pub async fn setup(
    foreground: bool,
    root: &Path,
    hbs: &Arc<handlebars::Handlebars<'static>>,
    reproto: &Reproto,
    languages: &Languages,
    suites: &[Suite],
    runners: &mut Vec<Box<dyn Runner>>,
    prebuilds: &mut Vec<PreBuild>,
    filter: impl Fn(&[&str]) -> bool,
) -> Result<()> {
    for language in &languages.languages {
        if language.no_project {
            continue;
        }

        let mut any_project = false;
        // If any project builds are happening.
        let source_languages = root.join("languages").join(&language.name);
        let build_yaml = source_languages.join("build.yaml");

        if !build_yaml.is_file() {
            bail!("missing build.yaml: {}", build_yaml.display());
        }

        let build_yaml = build_yaml::load_path(&build_yaml)
            .with_context(|| format_err!("failed to load: {}", build_yaml.display()))?;

        let shared_languages = root
            .join("target")
            .join("projects")
            .join(&language.name)
            .join("shared");

        for suite in suites {
            if !suite.supports_language(&language.name) {
                log::trace!(
                    "language `{}` not supported by suite `{}`",
                    language.name,
                    suite.name
                );
                continue;
            }

            for instance in &language.instances {
                if filter(&["project", &suite.name, &instance.name, &language.name]) {
                    any_project = true;

                    let target_languages = root
                        .join("target")
                        .join("projects")
                        .join(&language.name)
                        .join(&format!("{}-{}", suite.name, instance.name));

                    let vars = Vars {
                        language: &language.name,
                        test: &suite.name,
                        instance: &instance.name,
                    };

                    let build_yaml = build_yaml.compile(&hbs, &vars)?;

                    runners.push(Box::new(ProjectRunner {
                        foreground,
                        root: root.to_owned(),
                        build_yaml,
                        suite: suite.clone(),
                        instance: instance.clone(),
                        language: language.clone(),
                        reproto: reproto.clone(),
                        shared_target: root.join("target"),
                        source_languages: source_languages.clone(),
                        target_languages,
                    }));
                }
            }
        }

        if any_project {
            if let Some(prebuild) =
                build_yaml.prebuild(&source_languages, &shared_languages, language)
            {
                prebuilds.push(prebuild);
            }
        }
    }

    Ok(())
}

#[derive(Serialize)]
struct Vars<'a> {
    language: &'a str,
    test: &'a str,
    instance: &'a str,
}

#[derive(Debug)]
struct ProjectRunner {
    foreground: bool,
    /// Root directory.
    root: PathBuf,
    /// Builder used with project.
    build_yaml: build_yaml::BuildYaml,
    /// Current project directory.
    suite: Suite,
    /// Instance of this test.
    instance: Instance,
    /// Language-specific options.
    language: Language,
    /// Reproto command wrapper.
    reproto: Reproto,
    /// Shared target directory for projects.
    shared_target: PathBuf,
    /// Source directory from where to build project.
    source_languages: PathBuf,
    /// Target directory to build project.
    target_languages: PathBuf,
}

impl ProjectRunner {
    fn manifest<'a>(&'a self) -> Manifest<'a> {
        Manifest {
            suite: &self.suite,
            output: self.language.output.to_path(&self.target_languages),
            language: &self.language,
            instance: &self.instance,
            package_prefix: self.language.package_prefix.as_deref(),
        }
    }

    async fn try_run(&self) -> Result<()> {
        let deadline = time::Instant::now() + self.build_yaml.deadline;

        utils::copy_dir(&self.source_languages, &self.target_languages)?;

        let name = format!("{}-{}", self.suite.name, self.instance.name);

        self.reproto.build(self.manifest()).await?;

        let run = self
            .build_yaml
            .build(
                self.foreground,
                deadline,
                &self.target_languages,
                &self.language,
                &self.suite,
                &self.instance,
            )
            .await?;

        log::debug!("{name}: starting project: {run:?}", name = name, run = run);

        let mut command = run.command()?;

        log::trace!(
            "{name}: running command: {command:?}",
            name = name,
            command = command
        );

        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let mut actual: Vec<Result<serde_json::Value>> = Vec::new();
        let mut expected = Vec::new();
        let mut stdout_buf = String::new();
        let mut stderr_buf = String::new();

        {
            let stdin = child.stdin.take().expect("missing stdin");
            let stdout = child.stdout.take().expect("missing stdout");
            let stderr = child.stderr.take().expect("missing stderr");

            let timed_out = self
                .perform_tests(
                    &name,
                    deadline,
                    stdin,
                    stdout,
                    stderr,
                    &mut expected,
                    &mut actual,
                    &mut stdout_buf,
                    &mut stderr_buf,
                )
                .await?;

            if timed_out {
                log::warn!("{name} - timed out, killing", name = name);
                child.kill()?;
            }

            log::trace!("{name}: waiting for process to exit...", name = name);
            let status = child.await?;
            log::trace!("{name}: exited with {status}", name = name, status = status);

            if !status.success() {
                bail!(
                    "Child exited with non-zero exit: {status}\ntimed_out: \
                     {timed_out}\nstdout:\n{stdout}stderr:\n{stderr}",
                    status = status,
                    stdout = stdout_buf,
                    stderr = stderr_buf,
                    timed_out = timed_out,
                );
            }
        }

        if actual.len() != expected.len() {
            bail!(
                "number of JSON documents ({}) do not match expected ({})",
                actual.len(),
                expected.len(),
            );
        }

        let mut mismatches = Vec::new();

        for (i, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
            let actual = match actual {
                Ok(actual) => actual,
                Err(error) => {
                    mismatches.push(JsonMismatch::Error {
                        index: i,
                        expected,
                        error,
                    });

                    continue;
                }
            };

            if !similar(&actual, &expected) {
                mismatches.push(JsonMismatch::Mismatch {
                    index: i,
                    actual,
                    expected,
                });
            }
        }

        if !mismatches.is_empty() {
            return Err(Error::JsonMismatches { mismatches }.into());
        }

        return Ok(());

        /// Check if the two documents are similar enough to be considered equal.
        fn similar(left: &serde_json::Value, right: &serde_json::Value) -> bool {
            use serde_json::Value::*;

            match (left, right) {
                (&Null, &Null) => true,
                (&Bool(ref left), &Bool(ref right)) => left == right,
                (&Number(ref left), &Number(ref right)) => match (left, right) {
                    (l, r) if l.is_u64() && r.is_u64() => {
                        l.as_u64().unwrap() == r.as_u64().unwrap()
                    }
                    (l, r) if l.is_i64() && r.is_i64() => {
                        l.as_i64().unwrap() == r.as_i64().unwrap()
                    }
                    (l, r) if l.is_f64() && r.is_f64() => {
                        let l = l.as_f64().unwrap();
                        let r = r.as_f64().unwrap();

                        (l.fract() - r.fract()).abs() < 0.0001f64
                    }
                    _ => false,
                },
                (&String(ref left), &String(ref right)) => left == right,
                (&Array(ref left), &Array(ref right)) => {
                    if left.len() != right.len() {
                        return false;
                    }

                    left.iter().zip(right.iter()).all(|(l, r)| similar(l, r))
                }
                (&Object(ref left), &Object(ref right)) => {
                    if left.len() != right.len() {
                        return false;
                    }

                    for (key, left) in left {
                        match right.get(key) {
                            None => return false,
                            Some(right) => {
                                if !similar(left, right) {
                                    return false;
                                }
                            }
                        }
                    }

                    return true;
                }
                _ => false,
            }
        }
    }

    /// Write inputs to the stdin of the process and collect expected documents.
    ///
    /// Returns bool indicating if the test timed out.
    async fn perform_tests(
        &self,
        name: &str,
        deadline: time::Instant,
        stdin: ChildStdin,
        stdout: ChildStdout,
        stderr: ChildStderr,
        expected: &mut Vec<serde_json::Value>,
        actual: &mut Vec<Result<serde_json::Value>>,
        stdout_buf: &mut String,
        stderr_buf: &mut String,
    ) -> Result<bool> {
        for input in &self.suite.json {
            let f = fs::File::open(&input)
                .await
                .map_err(|e| format_err!("failed to open: {}: {}", input.display(), e))?;
            let mut f = BufReader::new(f);

            let mut line = String::new();

            loop {
                line.clear();
                let n = f.read_line(&mut line).await?;

                if n == 0 {
                    break;
                }

                // skip comments.
                if line.starts_with('#') {
                    continue;
                }

                // skip empty lines
                if line.trim() == "" {
                    continue;
                }

                expected.push(serde_json::from_str(&line)?);
            }
        }

        let write_all = write_all(stdin, expected.iter());
        tokio::pin!(write_all);

        let read_stdout = read_stdout(name, stdout, stdout_buf, actual, expected.len());
        tokio::pin!(read_stdout);

        let read_stderr = read_stderr(stderr, stderr_buf);
        tokio::pin!(read_stderr);

        let mut timeout = time::delay_until(deadline);

        let mut stdout_ended = false;
        let mut stderr_ended = false;
        let mut writing_ended = false;
        let mut timed_out = false;

        while !stdout_ended || !stderr_ended || !writing_ended {
            tokio::select! {
                _ = &mut read_stdout, if !stdout_ended => {
                    stdout_ended = true;
                }
                _ = &mut read_stderr, if !stderr_ended => {
                    stderr_ended = true;
                }
                _ = &mut write_all, if !writing_ended => {
                    writing_ended = true;
                }
                _ = &mut timeout => {
                    timed_out = true;
                    break;
                }
            }
        }

        if timed_out {
            if !stdout_ended {
                drop(read_stdout);
            }

            if !stderr_ended {
                drop(read_stderr);
            }

            if !writing_ended {
                drop(write_all);
            }
        }

        return Ok(timed_out);

        async fn read_stdout(
            name: &str,
            mut stdout: ChildStdout,
            stdout_buf: &mut String,
            actual: &mut Vec<Result<serde_json::Value>>,
            len: usize,
        ) -> Result<()> {
            /// line marker.
            const MARK: &[u8] = b"#<>";
            const COMMENT: &[u8] = b"###";

            let mut read_buf = [0u8; 256];

            let mut buf = Vec::new();
            // the current read cursor.
            let mut cursor = 0;
            // if we are at a start of document.
            let mut end = None;

            loop {
                let n = stdout.read(&mut read_buf).await?;

                if n == 0 {
                    break;
                }

                buf.extend(&read_buf[..n]);

                while len != actual.len() {
                    match end.take() {
                        None => {
                            let o = match memchr::memchr(MARK[0], &buf[cursor..]) {
                                Some(o) => o,
                                None => {
                                    cursor = buf.len();
                                    break;
                                }
                            };

                            let s = &buf[cursor..];

                            if s.len() < MARK.len() {
                                cursor += o;
                                break;
                            }

                            match &s[..MARK.len()] {
                                MARK => {
                                    cursor += MARK.len();
                                    end = Some((Parsing::Json, cursor));
                                }
                                COMMENT => {
                                    cursor += COMMENT.len();
                                    end = Some((Parsing::Comment, cursor));
                                }
                                _ => {
                                    cursor += 1;
                                    continue;
                                }
                            }
                        }
                        Some((parsing, e)) => {
                            let o = match memchr::memchr(b'\n', &buf[e..]) {
                                Some(o) => o,
                                None => {
                                    end = Some((parsing, buf.len()));
                                    break;
                                }
                            };

                            let data = &buf[cursor..(e + o)];
                            cursor = e + o + 1;

                            match parsing {
                                Parsing::Json => {
                                    let json = serde_json::from_slice(data).map_err(Into::into);

                                    if log::log_enabled!(log::Level::Trace) {
                                        match &json {
                                            Ok(json) => {
                                                log::trace!(
                                                    "{name} - json: {json}",
                                                    name = name,
                                                    json = serde_json::to_string(json)?
                                                );
                                            }
                                            Err(e) => {
                                                log::trace!(
                                                    "{name} - error: {error}",
                                                    name = name,
                                                    error = e
                                                );
                                            }
                                        }
                                    }

                                    actual.push(json);
                                }
                                Parsing::Comment => {
                                    log::trace!(
                                        "{name} - comment:{comment}",
                                        name = name,
                                        comment = String::from_utf8_lossy(data)
                                    );
                                }
                            }
                        }
                    }
                }
            }

            if !buf.is_empty() {
                *stdout_buf = String::from_utf8_lossy(&buf).to_string();
            }

            return Ok(());

            #[derive(Debug, Clone, Copy)]
            enum Parsing {
                Comment,
                Json,
            }
        }

        async fn read_stderr(mut stderr: ChildStderr, error: &mut String) -> Result<()> {
            let mut read_buf = [0u8; 512];
            let mut buf = Vec::<u8>::new();

            loop {
                let n = stderr.read(&mut read_buf).await?;

                if n == 0 {
                    break;
                }

                buf.extend(&read_buf[..n]);
            }

            if !buf.is_empty() {
                *error = String::from_utf8_lossy(&buf).to_string();
            }

            Ok(())
        }

        /// Construct a future to write the next item
        async fn write_all<'a, I>(mut stdin: ChildStdin, mut it: I) -> Result<()>
        where
            I: Iterator<Item = &'a serde_json::Value>,
        {
            while let Some(value) = it.next() {
                let value = serde_json::to_vec(&value)?;
                stdin.write_all(&value).await?;
                stdin.write_u8(b'\n').await?;
                stdin.flush().await?;
            }

            Ok(())
        }
    }
}

impl Runner for ProjectRunner {
    /// Run the suite.
    fn run<'a>(&'a self) -> BoxFuture<'a, (TimedRun, Result<()>)> {
        Box::pin(async move {
            let id = format!(
                "project {} (lang: {}, instance: {})",
                self.suite.name, self.language.name, self.instance.name,
            );

            timed_run(id, self.try_run()).await
        })
    }

    fn containers(&self, containers: &mut std::collections::HashSet<String>) {
        self.build_yaml.containers(containers);
    }
}
