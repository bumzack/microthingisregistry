extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate pretty_env_logger;

use std::env;

use diesel::mysql;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use warp::Filter;

use crate::backend::backend_rest::filters_backend;
use crate::db::db::get_connection_pool;
use crate::db::insert_data::{insert_backends, insert_frontends, insert_hosts, insert_services, insert_technologies};
use crate::db::read_data::{print_backends, print_frontends, print_hosts, print_services};
use crate::frontend::frontend_rest::filters_frontend;
use crate::host::host_rest::filters_host;
use crate::technology::technology_rest::filters_technology;

mod utils;
mod technology;
mod host;
mod models;
mod schema;
mod db;
mod backend;
mod frontend;
mod microservice;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_connection_pool();
    let connection = &mut pool.get().unwrap();

    // insert_technologies(connection);
    // insert_hosts(connection);
    // insert_services(connection);
    //
    // print_hosts(connection);
    // print_services(connection);
    // print_hosts(connection);
    //
    // // insert_frontends(connection);
    // // insert_backends(connection);
    //
    // print_frontends(connection);
    // print_backends(connection);

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "technology=info");
    }
    pretty_env_logger::init();

    log::info!("bla");

    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let root = root
        .or(filters_technology::technology(pool.clone()))
        .or(filters_backend::backend(pool.clone()))
        .or(filters_frontend::frontend(pool.clone()))
        .or(filters_host::host(pool.clone()));

    // View access logs by setting `RUST_LOG=todos`.
    let routes = root.with(warp::log("technology"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
