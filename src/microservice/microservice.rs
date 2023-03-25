use std::io::Error;

use diesel::r2d2::ConnectionManager;
use diesel::{MysqlConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use r2d2::Pool;

use crate::db::read_data::print_backends;
use crate::diesel::ExpressionMethods;
use crate::models::models::{Backend, MicroService};

pub fn find_microservice_by_name(
    db: Pool<ConnectionManager<MysqlConnection>>,
    name: &str,
) -> Option<MicroService> {
    let connection = &mut db.get().unwrap();

    use crate::schema::microservice;
    let result = microservice::table
        .filter(crate::schema::microservice::microservice_id.eq(name))
        .select(MicroService::as_select())
        .get_result(connection);
    match result {
        Ok(r) => Some(r),
        Err(e) => {
            println!("cant find microservice {}", name);
            None
        }
    }
}

pub fn find_backend_by_name(
    db: Pool<ConnectionManager<MysqlConnection>>,
    name: &str,
) -> Option<Backend> {
    let connection = &mut db.get().unwrap();

    println!("looking for backend {}", name);
    use crate::schema::backend;
    let result = backend::table
        .filter(crate::schema::backend::microservice_id.eq(name))
        .select(Backend::as_select())
        .get_result(connection);
    match result {
        Ok(r) => Some(r),
        Err(e) => {
            println!("cant find backend {}", name);
            None
        }
    }
}
