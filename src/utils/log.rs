use crate::utils::Error;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};
use slog::{o, Drain, Level, LevelFilter, FnValue, PushFnValue, Record, Logger, Serializer, KV};
use slog_async::Async;
use std::sync::Arc;

#[derive(Debug, EnumString, Deserialize, Serialize, Display)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum LogFormat {
    #[strum(serialize = "json")]
    Json,
    #[strum(serialize = "console")]
    Console,
}

#[derive(Debug, EnumString, Deserialize, Serialize, Clone, Copy, Display)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum LogLevel {
    #[strum(serialize = "critical")]
    Critical,
    #[strum(serialize = "error")]
    Error,
    #[strum(serialize = "warning")]
    Warning,
    #[strum(serialize = "info")]
    Info,
    #[strum(serialize = "debug")]
    Debug,
    #[strum(serialize = "trace")]
    Trace,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LogConfig {
    pub format: LogFormat,
    pub level:  LogLevel,
}

impl KV for LogConfig {
    fn serialize(&self, _record: &Record, serializer: &mut dyn Serializer) -> slog::Result {
        serializer.emit_str("LogConfig.format", self.format.to_string().as_str())?;
        serializer.emit_str("LogConfig.level", self.level.to_string().as_str())
    }
}

impl From<LogLevel> for Level {
    fn from(config: LogLevel) -> Self {
        match config {
            LogLevel::Trace => Level::Trace,
            LogLevel::Critical => Level::Critical,
            LogLevel::Error => Level::Error,
            LogLevel::Warning => Level::Warning,
            LogLevel::Info => Level::Info,
            LogLevel::Debug => Level::Debug
        }
    }
}

pub fn new_logger(config: &LogConfig) -> Result<Logger, Error> {
    let level: Level = config.level.into();

    let drain = match config.format {
        LogFormat::Console => {
            let drain = slog_json::Json::new(std::io::stdout()).build().fuse();

            Async::new(LevelFilter(drain, level).fuse())
                .chan_size(100)
                .build()
                .fuse()
        },

        LogFormat::Json => {
            let drain = slog_term::TermDecorator::new().build();
            let drain = slog_term::FullFormat::new(drain).build().fuse();

            Async::new(LevelFilter(drain, level).fuse())
                .chan_size(100)
                .build()
                .fuse()
        },
    };

    let logger = slog::Logger::root_typed(
        Arc::new(drain),
        o!(
            "timestamp" => PushFnValue(|_: &Record, ser| {
                ser.emit(chrono::Local::now().to_rfc3339())
            }),
            "level" => FnValue(|rec: &Record| {
                rec.level().as_short_str()
            }),
            "message" => PushFnValue(|rec: &Record, ser| {
                ser.emit(rec.msg())
            }),
        ),
    );

    let erased_logger = logger.into_erased();
    let _scope_guard = slog_scope::set_global_logger(erased_logger.clone());
    let _log_guard = slog_stdlog::init().unwrap();

    Ok(erased_logger)
}
