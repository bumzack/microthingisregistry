use std::env;

use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;

use dotenvy::dotenv;
use r2d2::Pool;
use warp::Filter;

fn database_url_for_env() -> String {
    let p = dotenv().unwrap();
    println!("path {:?}", &p);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DATABASE URL {database_url}");

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

pub fn with_db(
    db: Pool<ConnectionManager<MysqlConnection>>,
) -> impl Filter<
    Extract = (Pool<ConnectionManager<MysqlConnection>>,),
    Error = std::convert::Infallible,
> + Clone {
    warp::any().map(move || db.clone())
}
