use ::errors::*;

use log;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= log::LogLevel::Debug
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}

pub fn init(level: log::LogLevelFilter) -> Result<()> {
    log::set_logger(|max_level| {
            max_level.set(level);
            Box::new(SimpleLogger)
        })
        .map_err(|e| e.into())
}
