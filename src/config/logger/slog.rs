use std::fs::OpenOptions;


use slog_async::{self, Async, OverflowStrategy};
use slog_term::{self, FullFormat, TermDecorator};

use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use slog::{o, Drain, Level as SlogLevel, Logger};

use super::main::LoggerProvider;

pub struct SlogEngine;
impl LoggerProvider for SlogEngine {
    fn init(&self) {
        init_logger().expect("Failed to initialize logger");
    }
}


struct SlogLogger {
    logger: Logger,
}

impl log::Log for SlogLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.logger
            .is_enabled(convert_level_to_slog(metadata.level()))
    }

    fn log(&self, record: &Record) {
        let message = record.args();

        match record.level() {
            Level::Error => {
                slog::error!(self.logger, "{}", message);
            }
            Level::Warn => {
                slog::warn!(self.logger, "{}", message);
            }
            Level::Info => {
                slog::info!(self.logger, "{}", message);
            }
            Level::Debug => {
                slog::debug!(self.logger, "{}", message);
            }
            Level::Trace => {
                slog::trace!(self.logger, "{}", message);
            }
        }
    }

    fn flush(&self) {}
}

fn convert_level_to_slog(level: Level) -> SlogLevel {
    match level {
        Level::Error => SlogLevel::Error,
        Level::Warn => SlogLevel::Warning,
        Level::Info => SlogLevel::Info,
        Level::Debug => SlogLevel::Debug,
        Level::Trace => SlogLevel::Trace,
    }
}

fn init_logger() -> Result<(), SetLoggerError> {
    // STDOUT
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let async_drain = Async::new(drain)
        // .chan_size(usize::MAX)
        .overflow_strategy(OverflowStrategy::Block)
        .build()
        .fuse();

    // FILE
    let log_path = "logs/current.log";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)
        .unwrap();
    let file_decorator = slog_term::PlainSyncDecorator::new(file);
    let file_drain = slog_term::FullFormat::new(file_decorator).build().fuse();
    let async_file_drain = Async::new(file_drain)
        // .chan_size(usize::MAX)
        .overflow_strategy(OverflowStrategy::Block)
        .build()
        .fuse();

    let logger = slog::Logger::root(
        slog::Duplicate::new(async_drain, async_file_drain).fuse(),
        o!(),
    );

    let slog_logger = SlogLogger { logger };
    log::set_boxed_logger(Box::new(slog_logger))?;
    log::set_max_level(LevelFilter::Info);
    Ok(())
}
