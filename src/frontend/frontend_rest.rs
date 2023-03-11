// based on
// https://github.com/seanmonstar/warp/blob/master/examples/rejections.rs
// and
// https://github.com/seanmonstar/warp/blob/master/examples/todos.rs

pub mod filters_frontend{
    use diesel::MysqlConnection;
    use diesel::r2d2::ConnectionManager;
    use r2d2::Pool;
    use warp::Filter;

    use crate::db::db::with_db;
    use crate::models::rest_models::rest_models::NewFrontendPost;

    use super::handlers_frontend;

    pub fn frontend (
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        let api = warp::path("api");
        api.and(frontend_list(connection_pool.clone())
            .or(frontend_create(connection_pool.clone())))
    }

    /// GET /frontend
    pub fn frontend_list(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        warp::path!("frontend")
            .and(warp::get())
            .and(with_db(connection_pool.clone()))
            .and_then(handlers_frontend::list_frontend )
    }

    // POST /frontend  with JSON body
    pub fn frontend_create(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        warp::path!("frontend")
            .and(warp::post())
            .and(json_body_new_frontend ())
            .and(with_db(connection_pool))
            .and_then(handlers_frontend::create_frontend)
    }

    fn json_body_new_frontend () -> impl Filter<Extract=(NewFrontendPost, ), Error=warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers_frontend {
    use std::convert::Infallible;

    use diesel::{MysqlConnection, RunQueryDsl};
    use diesel::r2d2::ConnectionManager;
    use r2d2::Pool;
    use serde::Serialize;
    use warp::http::StatusCode;
    use warp::log;
    use crate::db::read_data::{print_backends, print_frontends};

    use crate::models::models::{Frontend, NewFrontend};
    use crate::models::rest_models::rest_models::{ErrorMessage, NewFrontendPost};

    // opts: ListOptions,
    pub async fn list_frontend (db: Pool<ConnectionManager<MysqlConnection>>) -> Result<impl warp::Reply, Infallible> {
        // Just return a JSON array of todos, applying the limit and offset.
        let connection = &mut db.get().unwrap();
        let frontends: Vec<Frontend> = print_frontends(connection);
        // log::info!("    -> todo id not found!");
        //  ("found {} frontends ", frontends.size);
        Ok(warp::reply::json(&frontends))
    }

    pub async fn create_frontend(new_tec: NewFrontendPost, pool: Pool<ConnectionManager<MysqlConnection>>) -> Result<impl warp::Reply, Infallible> {
        use crate::schema::frontend;

        //  log::info!("create_frontend: {:?}", create);
        let connection = &mut pool.get().unwrap();

        let new_frontend = NewFrontend {
            microservice_id: new_tec.microservice_id.as_str(),
            service_url: new_tec.service_url.as_str(),
            local_repo_path: new_tec.local_repo_path.as_str(),
            technology_id: new_tec.technology_id,
            url: new_tec.url.as_str(),
        };

        match diesel::insert_into(frontend::table)
            .values(&new_frontend)
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
                let message = format!("an error occurred inserting a new frontend which we are ignoring '{}'", e);
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
