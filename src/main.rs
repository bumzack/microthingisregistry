extern crate diesel;
extern crate futures;
extern crate pretty_env_logger;

use std::env;

use diesel::mysql;
use diesel::prelude::*;
use dotenvy::dotenv;

use serde::{Deserialize, Serialize};
use warp::Filter;

use crate::backend::backend_rest::filters_backend;
use crate::db::db::get_connection_pool;
use crate::db::insert_data::{
    insert_frontends, insert_hosts, insert_services, insert_technologies,
};

use crate::db::read_data::{print_backends, print_frontends, print_hosts, print_services};
use crate::frontend::frontend_rest::filters_frontend;
use crate::host::host_rest::filters_host;
use crate::technology::technology_restt::filters_technology;

mod backend;
mod db;
mod frontend;
mod host;
mod microservice;
mod models;
mod schema;
mod technology;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p = dotenv().unwrap();
    println!("path {:?}", &p);
    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    let pool = get_connection_pool();

    insert_technologies(&mut pool.clone().get().unwrap());

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "microthingisregistry=info");
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
