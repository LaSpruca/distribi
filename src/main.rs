pub mod reactors;
pub mod structures;

use structures::parse_structure;

use crate::reactors::load_reactors;
use std::{
    collections::HashMap,
    env::current_dir,
    fs::{self, create_dir},
    io::Write,
    path::Path,
    process::exit,
};

fn main() {
    println!("=> Loading reactors");
    let loaded_reactors = match load_reactors() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}", err);
            exit(-1);
        }
    };

    println!("=> Loaded reactors\n=>Loading structures");

    let mut data = HashMap::new();
    let mut structures_root = current_dir().unwrap();
    structures_root.push("structures");

    for a in loaded_reactors.keys() {
        let mut structures_root = structures_root.clone();
        structures_root.push(format!("{}.dst", a));

        let structure_def = match parse_structure(structures_root.to_str().unwrap()) {
            Ok(v) => v,
            Err(err) => {
                eprintln!("{}", err);
                exit(-1);
            }
        };

        data.insert(
            a.to_owned(),
            (structure_def, loaded_reactors.get(a).unwrap()),
        );
    }

    println!("=> Loaded reactors\nWriting binary");

    if !Path::new("out").is_dir() {
        create_dir("out").unwrap();
    }

    let serialized = bincode::serialize(&data).unwrap();
    let mut output_file = fs::File::create("out/data.bin").unwrap();
    output_file.write_all(serialized.as_slice()).unwrap();

    println!("Written binary")
}
