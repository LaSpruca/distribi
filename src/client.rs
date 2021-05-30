pub mod logger;
pub mod reactors;
pub mod structures;

use std::collections::HashMap;

use crate::{logger::log_setup, reactors::Reactor, structures::FieldType};
use actix_web::{client::Client, web::Buf};
use log::info;

#[actix_web::main]
async fn main() -> actix_web::Result<()> {
    log_setup();
    let client = Client::new();
    let mut setup_request = client
        .get("http://localhost:8080/distribi/setup")
        .send()
        .await?;

    let bytes = setup_request.body().await?.bytes().to_owned();
    let result: HashMap<String, (HashMap<String, FieldType>, Vec<Reactor>)> =
        bincode::deserialize_from(bytes.as_slice()).unwrap();

    info!("{:#?}", result);
    Ok(())
}
