use std::io::Error;

use diesel::{MysqlConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

use crate::db::read_data::print_backends;
use crate::diesel::ExpressionMethods;
use crate::models::models::MicroService;

pub fn find_microservice_by_name(db: Pool<ConnectionManager<MysqlConnection>>, name: &str) -> Option<MicroService> {
    let connection = &mut db.get().unwrap();

    use crate::schema::microservice;
    let x = microservice::table
        .filter(crate::schema::microservice::microservice_id.eq(name))
        .select(MicroService::as_select())
        .get_result(connection)
        .expect("should find it");
    Some(x)
}
