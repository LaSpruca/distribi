use chrono::SecondsFormat;
use colored::*;
use std::io::Write;

pub fn log_setup() {
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::builder()
        .format(|f, record| {
            writeln!(
                f,
                "[ {1} {0} ] {2}",
                chrono::Local::now()
                    .to_rfc3339_opts(SecondsFormat::Millis, true)
                    .as_str()
                    .bright_black(),
                // Color the log levels
                match record.level() {
                    log::Level::Error => {
                        " ERROR   ".red().bold()
                    }
                    log::Level::Warn => {
                        " WARNING ".yellow().bold()
                    }
                    log::Level::Info => {
                        " INFO    ".blue().bold()
                    }
                    log::Level::Debug => {
                        " DEBUG   ".white().bold()
                    }
                    log::Level::Trace => {
                        " TRACE   ".black().bold()
                    }
                },
                record.args()
            )
        })
        .init();
}
