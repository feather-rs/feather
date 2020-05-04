use log::Level;

/// Level of a log message to print.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<log::Level> for LogLevel {
    fn from(level: Level) -> Self {
        match level {
            Level::Debug => LogLevel::Debug,
            Level::Error => LogLevel::Error,
            Level::Info => LogLevel::Info,
            Level::Trace => LogLevel::Trace,
            Level::Warn => LogLevel::Warn,
        }
    }
}

impl From<LogLevel> for log::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => Level::Error,
            LogLevel::Warn => Level::Warn,
            LogLevel::Info => Level::Info,
            LogLevel::Debug => Level::Debug,
            LogLevel::Trace => Level::Trace,
        }
    }
}
