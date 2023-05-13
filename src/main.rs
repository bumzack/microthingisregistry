use std::env;

use dotenvy::dotenv;
use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Builder;
use warp::Filter;

use crate::backend::backend_rest::filters_backend;
use crate::db::db::get_connection_pool;
use crate::db::insert_data::insert_technologies;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::new().filter_level(LevelFilter::Debug).init();


    let p = dotenv().unwrap();
    info!("path {:?}", &p);
    for (key, value) in env::vars() {
        info!("{key}: {value}");
    }

    let pool = get_connection_pool();

    insert_technologies(&mut pool.clone().get().unwrap());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "content-type",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Access-Control-Allow-Headers",
            "Access-Control-Allow-Methods",
            "Access-Control-Allow-Origin",
            "Access-Control-Expose-Headers",
            "Access-Control-Request-Headers",
            "Access-Control-Request-Methods",
            "Accept-Encoding",
            "Accept-Language",
            "Accept-Post",
            "Access-Control-Allow-Credentials",
            "keep-alive",
        ])
        .allow_methods(vec!["POST", "GET", "OPTIONS", "PUT", "DELETE", "HEAD"]);

    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let root = root
        .or(filters_technology::technology(pool.clone()))
        .or(filters_backend::backend(pool.clone()))
        .or(filters_frontend::frontend(pool.clone()))
        .or(filters_host::host(pool.clone()))
        .with(cors);

    let routes = root.with(warp::log("microthingisregistry"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
