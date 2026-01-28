use clap::ValueEnum;
use std::error::Error;
use tracing::error;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Clone, ValueEnum)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }
}

pub fn init_logging(json_format: bool, log_level: LogLevel) {
    let env_filter = EnvFilter::new(format!("{}={}", env!("CARGO_PKG_NAME"), log_level.as_str()));

    if json_format {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                fmt::layer()
                    .json()
                    .with_current_span(false)
                    .with_span_list(false)
                    .with_target(true)
                    .with_level(true)
                    .with_thread_ids(false)
                    .with_thread_names(false)
                    .with_file(false)
                    .with_line_number(false),
            )
            .init();
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                fmt::layer()
                    .with_target(false)
                    .with_thread_ids(false)
                    .with_thread_names(false)
                    .with_file(false)
                    .with_line_number(false)
                    .compact(),
            )
            .init();
    }
}

pub fn chain_error(context: &str, err: &dyn Error) {
    error!("{}: {}", context, err);

    let mut source = err.source();
    let mut level = 0;

    while let Some(err) = source {
        level += 1;
        error!("  Caused by ({}): {}", level, err);
        source = err.source();
    }
}
