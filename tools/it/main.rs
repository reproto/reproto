extern crate diff;
#[macro_use]
extern crate failure;
extern crate it;
extern crate rayon;

use it::{Action, Instance, Language, Project, Reproto, Result};
use rayon::prelude::*;
use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};
use std::time::Instant;

/// Test if the given command successfully runs.
fn test(command: &str, args: &[&str]) -> bool {
    Command::new(command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(args)
        .status()
        .is_ok()
}

/// Detect supported languages
fn detect() -> HashSet<Language> {
    let mut out = HashSet::new();
    out.extend(vec![Language::Json, Language::Reproto]);

    if test("mvn", &["--version"]) {
        out.insert(Language::Java);
    } else {
        println!("WARN: `mvn --version` failed, not building Java projects");
    }

    if test("python", &["--version"]) {
        out.insert(Language::Python);
    } else {
        println!("WARN: `python --version` failed, not building Python projects");
    }

    if test("python3", &["--version"]) {
        out.insert(Language::Python3);
    } else {
        println!("WARN: `python3 --version` failed, not building Python 3 projects");
    }

    if test("cargo", &["--version"]) {
        out.insert(Language::Rust);
    } else {
        println!("WARN: `cargo --version` failed, not building Rust projects");
    }

    if test("node", &["--version"]) && test("babel", &["--version"]) {
        out.insert(Language::JavaScript);
    } else {
        println!(
            "WARN: `node --version` or `babel --version` failed, not building JavaScript projects"
        );
    }

    if test("dotnet", &["--version"]) {
        out.insert(Language::Csharp);
    } else {
        println!("WARN: `dotnet --version` failed, not building C# projects");
    }

    if test("swift", &["--version"]) {
        out.insert(Language::Swift);
    } else {
        println!("WARN: `swift --version` failed, not building Swift projects");
    }

    if test("go", &["version"]) {
        out.insert(Language::Go);
    } else {
        println!("WARN: `go version` failed, not building Go projects");
    }

    out
}

/// Print the differences between the two provided paths.
fn print_differences(source: &Path, target: &Path, errors: &[it::utils::Diff]) {
    use it::utils::Diff::*;

    println!(
        "differences between {} and {}",
        source.display(),
        target.display(),
    );

    for e in errors {
        match *e {
            MissingDir(ref loc, ref dir) => {
                println!("missing dir in {}:{}", loc.display(), dir.display());
            }
            ExpectedDir(ref loc, ref file) => {
                println!("expected dir in {}:{}", loc.display(), file.display());
            }
            MissingFile(ref loc, ref file) => {
                println!("missing file in {}:{}", loc.display(), file.display());
            }
            ExpectedFile(ref loc, ref file) => {
                println!("expected file in {}:{}", loc.display(), file.display());
            }
            Mismatch(ref src, _, ref mismatch) => {
                let added = mismatch
                    .iter()
                    .map(|m| match m {
                        &diff::Result::Right(_) => 1,
                        _ => 0,
                    }).sum::<u32>();

                let removed = mismatch
                    .iter()
                    .map(|m| match m {
                        &diff::Result::Left(_) => 1,
                        _ => 0,
                    }).sum::<u32>();

                println!("{}: +{}, -{}", src.display(), added, removed);

                for m in mismatch {
                    match m {
                        &diff::Result::Left(ref l) => println!("-{}", l),
                        &diff::Result::Right(ref r) => println!("+{}", r),
                        &diff::Result::Both(ref l, _) => println!(" {}", l),
                    }
                }
            }
        }
    }
}

fn try_main() -> Result<()> {
    let all_languages = vec![
        it::Language::Csharp,
        it::Language::Go,
        it::Language::Java,
        it::Language::JavaScript,
        it::Language::Json,
        it::Language::OpenApi,
        it::Language::Python,
        it::Language::Python3,
        it::Language::Reproto,
        it::Language::Rust,
        it::Language::Swift,
    ];

    let mut root = env::current_dir()?;
    let mut args = env::args();
    args.next();

    let mut do_checks = false;
    let mut do_structures = false;
    let mut do_project = false;
    let mut debug = false;
    let mut action = Action::Verify;
    let mut filters = HashSet::new();

    while let Some(opt) = args.next() {
        match opt.as_str() {
            "--update" => {
                action = Action::Update;
            }
            "--debug" => {
                debug = true;
            }
            "--check" => {
                do_checks = true;
            }
            "--structure" => {
                do_structures = true;
            }
            "--project" => {
                do_project = true;
            }
            "--root" => {
                let arg = args
                    .next()
                    .ok_or_else(|| format_err!("expected argument to `--root`"))?;
                root = PathBuf::from(arg);
            }
            other => {
                filters.insert(other.to_string());
            }
        }
    }

    let parent = root
        .parent()
        .ok_or_else(|| format_err!("no parent directory"))?;

    let cli = parent.join("cli");

    let reproto = Reproto::from_project(cli, debug)?;

    let project_languages = if do_project { detect() } else { HashSet::new() };

    let mut project = Project::new(
        &project_languages,
        &all_languages,
        &reproto,
        do_checks,
        do_structures,
        do_project,
        action,
    );

    project.arg(Language::Go, &["-m", "encoding/json"]);
    project.arg(Language::Java, &["-m", "builder", "-m", "jackson"]);
    project.arg(Language::Csharp, &["-m", "Json.NET"]);
    project.arg(Language::Python, &["-m", "python2"]);

    project.add(Language::Swift, {
        let mut i = Instance::new("simple");
        i.args(&["-m", "simple"]);
        i
    });

    project.add(Language::Swift, {
        let mut i = Instance::new("codable");
        i.args(&["-m", "codable"]);
        i
    });

    it::entry(&mut project);

    let before = Instant::now();

    let res = project
        .runners(&root)?
        .into_par_iter()
        .filter(|s| {
            filters
                .iter()
                .all(|term| s.keywords().into_iter().any(|k| k.contains(term)))
        }).map(|s| s.run())
        .collect::<Vec<_>>();

    let res = res
        .into_iter()
        .flat_map(|res| match res {
            Ok(_) => None.into_iter(),
            Err(e) => Some(e).into_iter(),
        }).collect::<Vec<_>>();

    let duration = Instant::now() - before;
    println!("Finished in {}", it::DurationFmt(duration));

    if res.len() > 0 {
        for (i, e) in res.iter().enumerate() {
            eprintln!("error #{}: {}", i, e);

            if let Some(error) = e.find_root_cause().downcast_ref::<it::Error>() {
                match *error {
                    it::Error::CheckFailed {
                        ref expected,
                        ref actual,
                    } => {
                        if let Some(expected) = expected.as_ref() {
                            println!("Expected:");
                            print!("{}", expected);
                        } else {
                            println!("# Expected *Nothing* (no file)");
                        }

                        println!("# Actual:");
                        print!("{}", actual);
                    }
                    it::Error::Differences {
                        ref source,
                        ref target,
                        ref errors,
                    } => {
                        print_differences(source, target, errors);
                    }
                    it::Error::JsonMismatches { ref mismatches } => {
                        for mismatch in mismatches {
                            println!(
                                "#{}: {} (expected) is not same as {} (actual)",
                                mismatch.index, mismatch.expected, mismatch.actual
                            );
                        }
                    }
                }

                continue;
            }

            for cause in e.iter_chain() {
                println!("Caused by: {}", cause);
            }

            continue;
        }

        bail!("Encountered {} error(s)", res.len());
    }

    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        process::exit(1)
    }

    process::exit(0);
}
