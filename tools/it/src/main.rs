use anyhow::{bail, format_err, Error, Result};
use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::stream::StreamExt as _;

/// Print the differences between the two provided paths.
fn write_differences<W>(
    mut w: W,
    source: &Path,
    target: &Path,
    errors: &[it::utils::Diff],
) -> Result<()>
where
    W: std::io::Write,
{
    use it::utils::Diff::*;

    writeln!(
        w,
        "differences between {} and {}",
        source.display(),
        target.display(),
    )?;

    for e in errors {
        match *e {
            MissingDir(ref loc, ref dir) => {
                writeln!(w, "missing dir in {}:{}", loc.display(), dir)?;
            }
            ExpectedDir(ref loc, ref file) => {
                writeln!(w, "expected dir in {}:{}", loc.display(), file)?;
            }
            MissingFile(ref loc, ref file) => {
                writeln!(w, "missing file in {}:{}", loc.display(), file)?;
            }
            ExpectedFile(ref loc, ref file) => {
                writeln!(w, "expected file in {}:{}", loc.display(), file)?;
            }
            Mismatch(ref src, _, ref mismatch) => {
                let added = mismatch
                    .iter()
                    .map(|m| match m {
                        &diff::Result::Right(_) => 1,
                        _ => 0,
                    })
                    .sum::<u32>();

                let removed = mismatch
                    .iter()
                    .map(|m| match m {
                        &diff::Result::Left(_) => 1,
                        _ => 0,
                    })
                    .sum::<u32>();

                writeln!(w, "{}: +{}, -{}", src, added, removed)?;

                for m in mismatch {
                    match m {
                        &diff::Result::Left(ref l) => writeln!(w, "-{}", l)?,
                        &diff::Result::Right(ref r) => writeln!(w, "+{}", r)?,
                        &diff::Result::Both(ref l, _) => writeln!(w, " {}", l)?,
                    }
                }
            }
        }
    }

    Ok(())
}

const WIDTH: usize = 10;

struct AnimalVisual {
    pub step: usize,
    total: usize,
    buf: String,
    last: Option<Instant>,
    runner: char,
    food: char,
    food_width: usize,
}

impl AnimalVisual {
    pub fn new(total: usize) -> Self {
        use unicode_width::UnicodeWidthChar as _;

        let runner = Self::pick_runner();
        let food = Self::pick_food();

        Self {
            step: 0,
            total,
            buf: String::new(),
            last: None,
            runner,
            food,
            food_width: food.width().unwrap_or(1),
        }
    }

    fn pick_runner() -> char {
        let choices = vec![
            '🐎', '🐤', '🦖', '🦆', '🐛', '🐟', '🐿', '🐪', '🐖', '🦙', '🦒', '🐘', '🐌',
        ];
        Self::pick(&choices)
    }

    fn pick_food() -> char {
        let choices = vec![
            '🧆', '🍩', '🧁', '🍍', '🍎', '🥔', '🌶', '🍕', '🌮', '🍄', '🥦', '🥑', '🍑', '🍒',
        ];
        Self::pick(&choices)
    }

    fn pick(choices: &[char]) -> char {
        choices[rand::random::<usize>() % choices.len()]
    }

    /// Get the next dot.
    pub fn dots(&mut self) -> &str {
        let pos = if self.total == 0 {
            0
        } else {
            (self.step * WIDTH) / self.total
        };

        let now = Instant::now();

        if let Some(last) = self.last {
            if now.duration_since(last) < Duration::from_millis(50) {
                return &self.buf;
            }
        }

        self.buf.clear();

        for _ in pos..WIDTH {
            self.buf.push(self.food);
        }

        self.buf.push(self.runner);

        for _ in 0..(pos * self.food_width) {
            self.buf.push('.');
        }

        self.buf.push(' ');

        self.buf.push(match self.step % 4 {
            0 => '◐',
            1 => '◓',
            2 => '◑',
            _ => '◒',
        });

        self.last = Some(now);
        &self.buf
    }
}

struct SimpleVisual {
    pub step: usize,
    buf: String,
    last: Option<Instant>,
}

impl SimpleVisual {
    pub fn new() -> Self {
        Self {
            step: 0,
            buf: String::new(),
            last: None,
        }
    }

    /// Get the next dot.
    pub fn render(&mut self) -> &str {
        let now = Instant::now();

        if let Some(last) = self.last {
            if now.duration_since(last) < Duration::from_millis(50) {
                return &self.buf;
            }
        }

        self.buf.clear();
        self.buf.push(match self.step % 4 {
            0 => '◐',
            1 => '◓',
            2 => '◑',
            _ => '◒',
        });

        self.last = Some(now);
        self.step += 1;
        &self.buf
    }
}

fn print_help() {
    println!("Please read 'Testing' in README.md");
    println!("");
    println!("cargo it [--all] [--ui] [--structures] [--projects] [--update] [filters]");
    println!("");
    println!("Arguments:");
    println!(
        "  --fg            - Run each project, one at a time in the foreground (troubleshooting)."
    );
    println!("  --root <path>   - Use the specified root location of the project.");
    println!("  --all           - Run all integration tests.");
    println!("  --ui            - Run all UI tests.");
    println!("  --structures    - Run all structures tests.");
    println!("  --projects      - Run (slow) projects tests.");
    println!(
        "  --update        - When combined with --ui or --structures, updates the reference files."
    );
    println!("  --report <path> - Write a report to the specified path.");
    println!(
        "   [filters]      - Additional arguments which act as filters. All specified filters \
         must match."
    );
    println!("");
    println!("Examples:");
    println!("  Run all tests (very fast):");
    println!("    cargo it --structures --ui");
    println!("  A single set of suites:");
    println!("    cargo it --structures --ui basic");
    println!("  A single set of projects:");
    println!("    cargo it --projects basic");
    println!("");
}

fn write_report<W>(out: &mut W, res: &[Error]) -> Result<()>
where
    W: std::io::Write,
{
    for (i, e) in res.iter().enumerate() {
        writeln!(out, "error #{}: {}", i, e)?;

        if let Some(error) = e.root_cause().downcast_ref::<it::Error>() {
            match error {
                it::Error::CheckFailed { expected, actual } => {
                    if let Some(expected) = expected.as_ref() {
                        writeln!(out, "Expected:")?;
                        write!(out, "{}", expected)?;
                    } else {
                        writeln!(out, "# Expected *Nothing* (no file)")?;
                    }

                    writeln!(out, "# Actual:")?;
                    write!(out, "{}", actual)?;
                }
                it::Error::Differences { from, to, errors } => {
                    write_differences(&mut *out, from, to, errors)?;
                }
                it::Error::JsonMismatches { ref mismatches } => {
                    for mismatch in mismatches {
                        match mismatch {
                            it::JsonMismatch::Mismatch {
                                index,
                                expected,
                                actual,
                            } => {
                                writeln!(
                                    out,
                                    "#{}: {} (expected) is not same as {} (actual)",
                                    index, expected, actual
                                )?;
                            }
                            it::JsonMismatch::Error {
                                index,
                                expected,
                                error,
                            } => {
                                writeln!(
                                    out,
                                    "#{}: {} (expected) while failed to decode json {}",
                                    index, expected, error
                                )?;
                            }
                        }
                    }
                }
            }

            continue;
        }

        for cause in e.chain() {
            writeln!(out, "Caused by: {}", cause)?;
        }
    }

    Ok(())
}

async fn try_main() -> Result<()> {
    use std::io::Write as _;

    env_logger::init();

    let mut root = PathBuf::from("");
    let mut args = env::args();
    args.next();

    let mut do_ui = false;
    let mut do_structures = false;
    let mut do_project = false;
    let mut debug = false;
    let mut progress = true;
    let mut action = it::Action::Verify;
    let mut filters = HashSet::new();
    let mut report = None;
    let mut num_tasks = num_cpus::get() * 4;
    let mut foreground = false;

    while let Some(opt) = args.next() {
        match opt.as_str() {
            "--help" => {
                print_help();
                return Ok(());
            }
            "--update" => {
                action = it::Action::Update;
            }
            "--fg" => {
                foreground = true;
            }
            "--debug" => {
                debug = true;
            }
            "--all" => {
                do_ui = true;
                do_structures = true;
                do_project = true;
            }
            "--ui" => {
                do_ui = true;
            }
            "--structures" => {
                do_structures = true;
            }
            "--projects" => {
                do_project = true;
            }
            "--root" => {
                let arg = args
                    .next()
                    .ok_or_else(|| format_err!("expected argument to `--root`"))?;
                root = PathBuf::from(arg);
            }
            "--report" => {
                let arg = args
                    .next()
                    .ok_or_else(|| format_err!("expected argument to `--report`"))?;
                report = Some(PathBuf::from(arg));
            }
            "--no-progress" => {
                progress = false;
            }
            "--tasks" => {
                let arg = args
                    .next()
                    .ok_or_else(|| format_err!("expected argument to `--tasks`"))?;
                num_tasks = str::parse(&arg)?;
            }
            other if other.starts_with('-') => {
                bail!("bad option: {}", other);
            }
            other => {
                filters.insert(other.to_string());
            }
        }
    }

    if foreground {
        num_tasks = 1;
        progress = false;
    }

    let it_root = root.join("it");
    let reproto = it::reproto::from_project(root.join("cli"), debug).await?;

    let reg = Arc::new(handlebars::Handlebars::new());

    let suites = it::suite::discover_suites(&it_root)?;
    let languages = it::languages::discover_languages(&it_root)?;

    let filter = |keywords: &[&str]| {
        if filters.is_empty() {
            return true;
        }

        filters
            .iter()
            .all(|term| keywords.into_iter().any(|k| k == term))
    };

    if log::log_enabled!(log::Level::Trace) {
        for language in &languages.languages {
            log::trace!("registered language: {:?}", language);
        }
    }

    let mut runners = Vec::new();
    let mut prebuilds = Vec::new();

    if do_ui {
        it::ui::setup(&it_root, action, filter, &reproto, &mut runners).await?;
    }

    if do_structures {
        it::structure::setup(
            &it_root,
            action,
            filter,
            &reproto,
            &languages,
            &suites,
            &mut runners,
        )
        .await?;
    }

    if do_project {
        it::project::setup(
            foreground,
            &it_root,
            &reg,
            &reproto,
            &languages,
            &suites,
            &mut runners,
            &mut prebuilds,
            filter,
        )
        .await?;
    }

    let mut containers = HashSet::new();

    for runner in &runners {
        runner.containers(&mut containers);
    }

    // Run pre-builds which sets up base containers.
    if !prebuilds.is_empty() {
        if !containers.is_empty() {
            it::docker::install_containers(&containers).await?;
        }

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let count = prebuilds.len();

        let mut visual = SimpleVisual::new();
        write!(
            stdout,
            "Building containers... {} (0/{})",
            visual.render(),
            count
        )?;
        stdout.flush()?;

        let mut futures = futures::stream::FuturesUnordered::new();
        let mut prebuilds_it = prebuilds.into_iter();

        for prebuild in (&mut prebuilds_it).take(num_tasks) {
            futures.push(tokio::spawn(
                async move { prebuild.build(foreground).await },
            ));
        }

        while let Some(result) = futures.next().await.transpose()? {
            let step = visual.step;
            write!(
                stdout,
                "\x1B[1K\rBuilding containers... {} ({}/{}) {}",
                visual.render(),
                step,
                count,
                result?
            )?;
            stdout.flush()?;

            if let Some(prebuild) = prebuilds_it.next() {
                futures.push(tokio::spawn(
                    async move { prebuild.build(foreground).await },
                ));
            }
        }

        write!(stdout, "\x1B[1K\rBuilding containers... done!")?;
        stdout.flush()?;
        writeln!(stdout)?;
    }

    let before = Instant::now();

    let mut futures = futures::stream::FuturesUnordered::new();
    let count = runners.len();

    let mut runners_it = runners.into_iter();

    for task in (&mut runners_it).take(num_tasks) {
        futures.push(tokio::spawn(async move { task.run().await }));
    }

    let mut visual = None;

    let mut res = Vec::new();

    let mut c = 0usize;
    let stdout = std::io::stdout();

    if progress {
        let visual = visual.get_or_insert_with(|| AnimalVisual::new(count));

        let mut stdout = stdout.lock();
        write!(stdout, "\x1B[1K\r")?;
        write!(stdout, "{} ({}/{})", visual.dots(), c, count,)?;
        stdout.flush()?;
    }

    while !futures.is_empty() {
        if let Some((timed_run, r)) = futures.next().await.transpose()? {
            if progress {
                let visual = visual.get_or_insert_with(|| AnimalVisual::new(count));
                visual.step += 1;

                let mut stdout = stdout.lock();
                write!(stdout, "\x1B[1K\r")?;
                write!(
                    stdout,
                    "{} ({}/{}): {} ({})",
                    visual.dots(),
                    c,
                    count,
                    timed_run.id,
                    timed_run.format_duration(),
                )?;
                stdout.flush()?;
            } else {
                if r.is_err() {
                    println!("FAIL: {} ({})", timed_run.id, timed_run.format_duration());
                } else {
                    println!("  OK: {} ({})", timed_run.id, timed_run.format_duration());
                }
            }

            if let Err(e) = r {
                res.push(e);
            }

            if let Some(task) = runners_it.next() {
                futures.push(tokio::spawn(async move { task.run().await }))
            }

            c += 1;
        }
    }

    write!(stdout.lock(), "\x1B[1K\r")?;

    let duration = Instant::now().duration_since(before);

    if res.len() > 0 {
        if let Some(report) = report {
            let mut out = std::fs::File::create(&report)?;
            write_report(&mut out, &res)?;

            println!(
                "Encountered {errors} errors from {count} tests in {duration}, wrote report to \
                 `{report}`.",
                errors = res.len(),
                count = count,
                duration = it::FormatDuration::new(duration),
                report = report.display(),
            );
        } else {
            let stdout = std::io::stdout();
            write_report(&mut stdout.lock(), &res)?;

            println!(
                "Encountered {errors} errors from {count} tests in {duration}.",
                errors = res.len(),
                count = count,
                duration = it::FormatDuration::new(duration),
            );
        }
    } else {
        println!(
            "Finished {count} tests in {duration} without issues!",
            count = count,
            duration = it::FormatDuration::new(duration)
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = try_main().await {
        eprintln!("Error: {}", e);

        for e in e.chain().skip(1) {
            eprintln!("Caused by: {}", e);
        }

        std::process::exit(1)
    }

    std::process::exit(0);
}
