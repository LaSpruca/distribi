pub mod reactors;
pub mod structures;

use crate::reactors::load_reactors;
use chrono::SecondsFormat;
use colored::*;
use log::{error, info};
use std::{
    collections::HashMap,
    env::current_dir,
    fs::{self, create_dir},
    io::Write,
    path::Path,
    process::exit,
};
use structures::parse_structure;

fn setup_distribi() {
    info!("Loading reactors");
    let loaded_reactors = match load_reactors() {
        Ok(v) => v,
        Err(err) => {
            error!("{}", err);
            exit(-1);
        }
    };

    info!("Loaded reactors");
    info!("Loading structures");

    let mut data = HashMap::new();
    let mut structures_root = current_dir().unwrap();
    structures_root.push("structures");

    for a in loaded_reactors.keys() {
        let mut structures_root = structures_root.clone();
        structures_root.push(format!("{}.dst", a));

        let structure_def = match parse_structure(structures_root.to_str().unwrap()) {
            Ok(v) => v,
            Err(err) => {
                error!("{}", err);
                exit(-1);
            }
        };

        data.insert(
            a.to_owned(),
            (structure_def, loaded_reactors.get(a).unwrap()),
        );
    }

    info!("Loaded reactors");
    info!("Writing binary");

    if !Path::new("out").is_dir() {
        create_dir("out").unwrap();
    }

    let serialized = bincode::serialize(&data).unwrap();
    let mut output_file = fs::File::create("out/data.bin").unwrap();
    output_file.write_all(serialized.as_slice()).unwrap();

    info!("Written binary")
}

fn main() {
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "[ {1} {0} ] {2}",
                chrono::Local::now()
                    .to_rfc3339_opts(SecondsFormat::Millis, true)
                    .as_str()
                    .bright_black(),
                // Color the log levels
                match record.level() {
                    log::Level::Error => {
                        "  ERROR  ".red().bold()
                    }
                    log::Level::Warn => {
                        " WARNING ".yellow().bold()
                    }
                    log::Level::Info => {
                        "   INFO  ".blue().bold()
                    }
                    log::Level::Debug => {
                        "  DEBUG  ".white().bold()
                    }
                    log::Level::Trace => {
                        "  TRACE  ".black().bold()
                    }
                },
                record.args()
            )
        })
        .init();
    setup_distribi();
}
