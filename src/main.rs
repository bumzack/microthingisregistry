extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate pretty_env_logger;

use std::env;

use diesel::mysql;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use warp::Filter;

use crate::insert_data::{insert_backends, insert_frontends, insert_hosts, insert_services, insert_technologies};
use crate::read_data::{print_backends, print_frontends, print_hosts, print_services, print_technologies};
use crate::technology::technology_rest::filters_technology;
use crate::utils::{establish_connection, get_connection_pool};

mod utils;
mod create_data;
mod insert_data;
mod read_data;
mod technology;
pub mod models;
pub mod schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_connection_pool();    //
    // println!("Migrating...");
    // connection.run_pending_migrations(MIGRATIONS).unwrap();
    // Ok(())

    let connection = &mut pool.get().unwrap();
    insert_technologies(connection);
    insert_hosts(connection);
    insert_services(connection);

    print_technologies(connection);
    print_services(connection);
    print_hosts(connection);

    insert_frontends(connection);
    insert_backends(connection);

    print_frontends(connection);
    print_backends(connection);

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "technology=info");
    }
    pretty_env_logger::init();

    log::info!("bla");

    let api = filters_technology::technology(pool.clone());

    // View access logs by setting `RUST_LOG=todos`.
    let routes = api.with(warp::log("technology"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
