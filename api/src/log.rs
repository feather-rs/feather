use crate::vtable::vtable;
use fapi_common::log::LogLevel;
use fapi_common::util::FStr;
use log::{Log, Metadata, Record};

/// A logging instance which delegates log messages
/// to the host's logging infrastructure.
pub struct HostLogger;

pub static HOST: HostLogger = HostLogger;

impl Log for HostLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let level = LogLevel::from(record.level());
        let msg = format!("{}", record.args());
        let module_path = record.module_path_static().unwrap_or("");

        unsafe {
            vtable().log(level, FStr::from(msg.as_str()), module_path.into());
        }
    }

    fn flush(&self) {
        // empty
    }
}
