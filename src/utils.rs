
use std::env;

use diesel::mysql;
use diesel::prelude::*;
use dotenvy::dotenv;
    use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

pub fn establish_connection() -> MysqlConnection {
    let database_url = database_url_for_env();
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn database_url_for_env() -> String {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    database_url
}

pub fn get_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    let url = database_url_for_env();
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

