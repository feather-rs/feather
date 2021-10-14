use std::io::{ErrorKind, Write};

use colored::Colorize;
use fern::Output;
use flume::Sender;
use log::{Level, LevelFilter};

pub fn init(level: LevelFilter, stdout: Sender<u8>) {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let level_string = match record.level() {
                Level::Error => record.level().to_string().red(),
                Level::Warn => record.level().to_string().yellow(),
                Level::Info => record.level().to_string().cyan(),
                Level::Debug => record.level().to_string().purple(),
                Level::Trace => record.level().to_string().normal(),
            };
            let target = if !record.target().is_empty() {
                record.target()
            } else {
                record.module_path().unwrap_or_default()
            };
            out.finish(format_args!(
                "{} {:<5} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S,%3f"),
                level_string,
                target,
                message,
            ));
        })
        .level(level)
        // cranelift_codegen spams debug-level logs
        .level_for("cranelift_codegen", LevelFilter::Info)
        .level_for("regalloc", LevelFilter::Off)
        .level_for("rustyline", LevelFilter::Off)
        .chain(FlumeSenderLogger(stdout))
        .apply()
        .unwrap();
}

struct FlumeSenderLogger(Sender<u8>);

impl From<FlumeSenderLogger> for Output {
    fn from(f: FlumeSenderLogger) -> Self {
        Output::writer(Box::new(f), "\n")
    }
}

impl Write for FlumeSenderLogger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for b in buf {
            self.0
                .send(*b)
                .map_err(|e| std::io::Error::new(ErrorKind::BrokenPipe, e.to_string()))?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
