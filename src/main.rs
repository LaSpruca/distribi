pub mod logger;
pub mod reactors;
pub mod structures;

use crate::{logger::log_setup, reactors::load_reactors};
use actix_web::{dev::Body, get, web::Bytes, App, HttpResponse, HttpServer, Result, Scope};
use log::{error, info};
use std::{
    collections::HashMap,
    env::current_dir,
    fs::{self, create_dir, File},
    io::{Read, Write},
    iter::FromIterator,
    path::Path,
    process::exit,
};
use structures::parse_structure;

#[get("/setup")]
fn setup() -> HttpResponse {
    let mut file = File::open("out/data.bin").unwrap();
    let metadata = file.metadata().unwrap();
    let mut bytes = vec![0; metadata.len() as usize];
    file.read(&mut bytes).unwrap();
    HttpResponse::Ok().message_body(Body::Bytes(Bytes::from_iter(bytes)))
}

#[actix_web::main]
async fn main() -> Result<()> {
    println!("no");
    log_setup();
    setup_distribi();

    HttpServer::new(|| App::new().service(Scope::new("/distribi").service(setup)))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}

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
