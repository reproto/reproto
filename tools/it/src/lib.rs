use anyhow::{Context as _, Result};
use std::collections::HashSet;
use std::fmt;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::io;

pub use self::build_yaml::PreBuild;

mod build_yaml;
pub mod docker;
mod lang_yaml;
pub mod languages;
pub mod line;
pub mod progress;
pub mod project;
pub mod reproto;
mod run;
pub mod structure;
pub mod suite;
pub mod ui;
pub mod utils;

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>;

#[derive(Debug)]
pub struct Test(String);

impl fmt::Display for Test {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

#[derive(Debug)]
pub enum JsonMismatch {
    Error {
        index: usize,
        expected: serde_json::Value,
        error: anyhow::Error,
    },
    Mismatch {
        index: usize,
        expected: serde_json::Value,
        actual: serde_json::Value,
    },
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("check failed")]
    CheckFailed {
        expected: Option<crate::reproto::CheckResult>,
        actual: crate::reproto::CheckResult,
    },

    #[error("differences between two directories")]
    Differences {
        from: PathBuf,
        to: PathBuf,
        errors: Vec<utils::Diff>,
    },

    #[error("mismatches in json documents")]
    JsonMismatches { mismatches: Vec<JsonMismatch> },
}

pub struct TimedRun {
    pub id: String,
    duration: Duration,
}

impl TimedRun {
    /// Format the duration to be human readable.
    pub fn format_duration(&self) -> FormatDuration {
        FormatDuration(self.duration)
    }
}

#[derive(Debug)]
pub struct FormatDuration(Duration);

impl FormatDuration {
    /// Construct a new formatted duration.
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }
}

impl fmt::Display for FormatDuration {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let d = self.0;

        let (secs, ns) = (d.as_secs(), d.subsec_nanos());

        if secs > 0 {
            return write!(fmt, "{}.{}s", secs, (ns / 10_000_000) % 100);
        }

        match ns {
            ns if ns / 1_000_000 > 0 => write!(fmt, "{}ms", ns / 1_000_000),
            ns if ns / 1_000 > 0 => write!(fmt, "{}us", ns / 1_000),
            ns => write!(fmt, "{}ns", ns),
        }
    }
}

/// Perform a timed run of the given segment and report its results.
pub async fn timed_run<T>(id: String, task: T) -> (TimedRun, Result<()>)
where
    T: Future<Output = Result<()>>,
{
    let before = Instant::now();
    let res = task.await.context(Test(id.to_string()));
    let duration = Instant::now().duration_since(before);
    let timed_run = TimedRun { id, duration };
    (timed_run, res)
}

pub trait Runner: Send + Sync {
    /// Run the current runner.
    fn run<'o>(&'o self) -> BoxFuture<'o, (TimedRun, Result<()>)>;

    /// Extract all containers used by the specified runner so that they can
    /// be pulled before they are used.
    fn containers(&self, _: &mut HashSet<String>) {}
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    /// Update the suite.
    Update,
    /// Verify the suite.
    Verify,
}
