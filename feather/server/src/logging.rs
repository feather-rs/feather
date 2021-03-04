use colored::Colorize;
use log::{Level, LevelFilter};

pub fn init(level: LevelFilter) {
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
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}
