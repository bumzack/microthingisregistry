// based on
// https://github.com/seanmonstar/warp/blob/master/examples/rejections.rs
// and
// https://github.com/seanmonstar/warp/blob/master/examples/todos.rs

pub mod filters_backend {
    use diesel::MysqlConnection;
    use diesel::r2d2::ConnectionManager;
    use r2d2::Pool;
    use warp::Filter;

    use crate::db::db::with_db;
    use crate::models::rest_models::rest_models::NewBackendPost;

    use super::handlers_backend;

    pub fn backend(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        let api = warp::path("api");
        api.and(backend_list(connection_pool.clone())
            .or(backend_create(connection_pool.clone())))
    }

    /// GET /backend
    pub fn backend_list(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        warp::path!("backend")
            .and(warp::get())
            .and(with_db(connection_pool.clone()))
            .and_then(handlers_backend::list_backend)
    }

    // POST /backend with JSON body
    pub fn backend_create(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        warp::path!("technology")
            .and(warp::post())
            .and(json_body_new_backend())
            .and(with_db(connection_pool))
            .and_then(handlers_backend::create_technology)
    }

    fn json_body_new_backend() -> impl Filter<Extract=(NewBackendPost, ), Error=warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers_backend {
    use std::convert::Infallible;

    use diesel::{MysqlConnection, RunQueryDsl};
    use diesel::r2d2::ConnectionManager;
    use r2d2::Pool;
    use serde::Serialize;
    use warp::http::StatusCode;
    use warp::log;
    use crate::db::read_data::print_backends;
    use crate::models::models::{Backend, NewBackend};

    use crate::models::rest_models::rest_models::{ErrorMessage, NewBackendPost, NewTechnologyPost};

    // opts: ListOptions,
    pub async fn list_backend(db: Pool<ConnectionManager<MysqlConnection>>) -> Result<impl warp::Reply, Infallible> {
        // Just return a JSON array of todos, applying the limit and offset.
        let connection = &mut db.get().unwrap();
        let techs: Vec<Backend> = print_backends(connection);
        // log::info!("    -> todo id not found!");
        //  ("found {} technologies ", techs.size);
        Ok(warp::reply::json(&techs))
    }

    pub async fn create_technology(new_tec: NewBackendPost, pool: Pool<ConnectionManager<MysqlConnection>>) -> Result<impl warp::Reply, Infallible> {
        use crate::schema::backend;

        //  log::info!("create_technology: {:?}", create);
        let connection = &mut pool.get().unwrap();

        let new_backend= NewBackend {
            microservice_id: new_tec.microservice_id.as_str(),
            service_url: new_tec.service_url.as_str(),
            openapi_url: new_tec.openapi_url.as_str(),
            local_repo_path: new_tec.local_repo_path.as_str(),
            technology_id: new_tec.technology_id,
        };

        match diesel::insert_into(backend::table)
            .values(&new_backend)
            .execute(connection) {
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
                let message = format!("an error occurred inserting a new backend which we are ignoring '{}'", e);
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
