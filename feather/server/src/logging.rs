use colored::Colorize;
use log::{Level, LevelFilter};
use time::macros::format_description;
use time::OffsetDateTime;

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

            let datetime: OffsetDateTime = match OffsetDateTime::now_local() {
                Ok(x) => x,
                Err(_) => OffsetDateTime::now_utc(),
            };
            out.finish(format_args!(
                "{} {:<5} [{}] {}",
                datetime
                    .format(format_description!(
                        "[year]-[month]-[day] [hour]:[minute]:[second],[subsecond digits:3]"
                    ))
                    .unwrap(),
                level_string,
                target,
                message,
            ));
        })
        .level(level)
        // cranelift_codegen spams debug-level logs
        .level_for("cranelift_codegen", LevelFilter::Info)
        .level_for("regalloc", LevelFilter::Off)
        .level_for("wasmer_wasi::syscalls", LevelFilter::Info)
        .level_for("wasmer_compiler_cranelift::translator", LevelFilter::Warn)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}
