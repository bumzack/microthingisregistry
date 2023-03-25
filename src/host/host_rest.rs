// based on
// https://github.com/seanmonstar/warp/blob/master/examples/rejections.rs
// and
// https://github.com/seanmonstar/warp/blob/master/examples/todos.rs

pub mod filters_host {
    use diesel::r2d2::ConnectionManager;
    use diesel::MysqlConnection;

    use r2d2::Pool;
    use warp::Filter;

    use crate::db::db::with_db;
    use crate::models::rest_models::rest_models::NewHostPost;

    use super::handlers_host;

    pub fn host(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let api = warp::path("api");
        api.and(host_list(connection_pool.clone()).or(host_create(connection_pool.clone())))
    }

    /// GET /host
    pub fn host_list(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("host")
            .and(warp::get())
            .and(with_db(connection_pool.clone()))
            .and_then(handlers_host::list_hosts)
    }

    // POST /host with JSON body
    pub fn host_create(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("host")
            .and(warp::post())
            .and(json_body_new_host())
            .and(with_db(connection_pool))
            .and_then(handlers_host::create_host)
    }

    fn json_body_new_host() -> impl Filter<Extract = (NewHostPost,), Error = warp::Rejection> + Clone
    {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers_host {
    use std::convert::Infallible;

    use diesel::r2d2::ConnectionManager;
    use diesel::{MysqlConnection, RunQueryDsl};

    use r2d2::Pool;
    use serde::Serialize;
    use warp::http::StatusCode;
    use warp::log;

    use crate::db::read_data::print_hosts;
    use crate::models::models::{Host, NewHost, NewTechnology, Technology};
    use crate::models::rest_models::rest_models::{ErrorMessage, NewHostPost, NewTechnologyPost};

    // opts: ListOptions,
    pub async fn list_hosts(
        db: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        // Just return a JSON array of todos, applying the limit and offset.
        let connection = &mut db.get().unwrap();
        let hosts: Vec<Host> = print_hosts(connection);
        // log::info!("    -> todo id not found!");
        //  ("found {} hosts ", hosts.size);
        Ok(warp::reply::json(&hosts))
    }

    pub async fn create_host(
        new_host: NewHostPost,
        pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        use crate::schema::host;

        //  log::info!("create_technology: {:?}", create);
        let connection = &mut pool.get().unwrap();

        let new_host = NewHost {
            hostname: new_host.hostname.as_str(),
            ip: new_host.ip.as_str(),
            port: new_host.port,
        };

        match diesel::insert_into(host::table)
            .values(&new_host)
            .execute(connection)
        {
            Ok(iedee) => {
                let message = format!("created");
                let code = StatusCode::CREATED;
                let json = warp::reply::json(&ErrorMessage {
                    code: code.as_u16(),
                    message: message.into(),
                });
                Ok(warp::reply::with_status(json, code))
            }
            Err(e) => {
                let message = format!(
                    "an error occurred inserting a new host which we are ignoring '{}'",
                    e
                );

                let code = StatusCode::INTERNAL_SERVER_ERROR;

                let json = warp::reply::json(&ErrorMessage {
                    code: code.as_u16(),
                    message: message.into(),
                });

                Ok(warp::reply::with_status(json, code))
            }
        }
    }
}
