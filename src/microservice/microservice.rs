use diesel::{MysqlConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::r2d2::ConnectionManager;
use log::{error, info};
use r2d2::Pool;

use crate::models::models::{Backend, MicroService};

pub fn find_microservice_by_name(
    db: Pool<ConnectionManager<MysqlConnection>>,
    name: &str,
) -> Option<MicroService> {
    use diesel::ExpressionMethods;

    let connection = &mut db.get().unwrap();

    use crate::schema::microservice;
    let result = microservice::table
        .filter(microservice::microservice_id.eq(name))
        .select(MicroService::as_select())
        .get_result(connection);
    match result {
        Ok(r) => Some(r),
        Err(e) => {
            error!("cant find microservice {name}, {e}");
            None
        }
    }
}

pub fn find_backend_by_name(
    db: Pool<ConnectionManager<MysqlConnection>>,
    name: &str,
) -> Option<Backend> {
    use diesel::ExpressionMethods;
    let connection = &mut db.get().unwrap();

    info!("looking for backend {name}");
    use crate::schema::backend;
    let result = backend::table
        .filter(backend::microservice_id.eq(name))
        .select(Backend::as_select())
        .get_result(connection);
    match result {
        Ok(r) => Some(r),
        Err(e) => {
            error!("cant find backend {name}. err {e}",);
            None
        }
    }
}
