#![allow(unused_must_use, unused_assignments)]
#![feature(proc_macro_hygiene, decl_macro, arc_unwrap_or_clone, async_closure)]

#[macro_use]
extern crate rocket;

pub mod database;
pub mod errors;
pub mod logging;
pub mod middleware;
pub mod routes;
pub mod server;
pub mod state;
pub mod util;

use mayhem::database::prepare_connection;
use mayhem_db::Client;
use rocket::Error;
use routes::{handle_error, handle_index, handle_login, handle_register};

#[main]
pub async fn main() -> Result<(), Error> {
    let database_connection = prepare_connection();
    let client = Client::connect(database_connection).await;

    info_!("Connected to the database!");

    let mut server = rocket::build();

    server = server.manage(client.clone());

    server = server.mount("/", routes![handle_index, handle_login, handle_register]);
    server = server.register("/", catchers![handle_error]);

    server = server.attach(routes::server::stage());

    let app = server.ignite().await.unwrap();

    client.run_migrations().await.unwrap();

    info_!("Migrations succeeded!");

    app.launch().await.unwrap();

    return Ok(());
}
