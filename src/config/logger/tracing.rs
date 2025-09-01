use lazy_static::lazy_static;

use std::path::Path;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};

use super::main::LoggerProvider;

use chrono::Local;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::{fmt, EnvFilter};

pub struct TracingEngine;
impl LoggerProvider for TracingEngine {
    fn init(&self) {
        LogTracer::builder()
            .ignore_crate("actix_server")
            .init()
            .expect("Failed to set logger");

        let _ = &*LOGGER_GUARD;
    }
}

lazy_static! {
    static ref LOGGER_GUARD: WorkerGuard = setup_tracing();
}

struct CustomTimeFormat;

impl FormatTime for CustomTimeFormat {
    fn format_time(
        &self,
        writer: &mut tracing_subscriber::fmt::format::Writer<'_>,
    ) -> std::fmt::Result {
        write!(writer, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

fn setup_tracing() -> WorkerGuard {
    let (_non_blocking_stdout, _guard_stdout) = get_stdout();
    let (_non_blocking_rolling, _guard_rolling) = get_rolling();


    let custom_format = fmt::format()
        .with_target(true)
        .with_timer(CustomTimeFormat)
        .with_ansi(true)
        .with_source_location(false)
        .with_level(true);

    let subscriber = fmt::Subscriber::builder()
        .with_writer(_non_blocking_stdout)
        .with_writer(_non_blocking_rolling)
        .with_env_filter(EnvFilter::from_default_env())
        .event_format(custom_format)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    // guard_stdout
    _guard_rolling
}

fn get_stdout() -> (NonBlocking, WorkerGuard) {
    tracing_appender::non_blocking(std::io::stdout())
}
fn get_rolling() -> (NonBlocking, WorkerGuard) {
    let log_dir = "./logs";
    std::fs::create_dir_all(log_dir).unwrap();
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY, // Rotate daily
        Path::new(log_dir),
        format!("current.log"),
    );
    tracing_appender::non_blocking(file_appender)

}