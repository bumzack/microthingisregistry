// based on
// https://github.com/seanmonstar/warp/blob/master/examples/rejections.rs
// and
// https://github.com/seanmonstar/warp/blob/master/examples/todos.rs

pub mod filters_frontend {
    use diesel::r2d2::ConnectionManager;
    use diesel::MysqlConnection;
    use r2d2::Pool;
    use warp::Filter;

    use crate::db::db::with_db;
    use crate::models::rest_modelss::rest_models::NewFrontendPost;

    use super::handlers_frontend;

    pub fn frontend(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let api = warp::path("api");
        api.and(frontend_list(connection_pool.clone()).or(frontend_create(connection_pool)))
    }

    /// GET /frontend
    pub fn frontend_list(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("frontend")
            .and(warp::get())
            .and(with_db(connection_pool))
            .and_then(handlers_frontend::list_frontend)
    }

    // POST /frontend  with JSON body
    pub fn frontend_create(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("frontend")
            .and(warp::post())
            .and(json_body_new_frontend())
            .and(with_db(connection_pool))
            .and_then(handlers_frontend::create_frontend)
    }

    fn json_body_new_frontend(
    ) -> impl Filter<Extract = (NewFrontendPost,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers_frontend {
    use std::convert::Infallible;

    use diesel::r2d2::ConnectionManager;
    use diesel::{MysqlConnection, RunQueryDsl};

    use crate::db::create_data::create_service;
    use r2d2::Pool;
    use warp::http::StatusCode;

    use crate::db::read_data::print_frontends;
    use crate::microservice::microservice::find_microservice_by_name;
    use crate::models::models::{Frontend, NewFrontend};
    use crate::models::rest_modelss::rest_models::{ErrorMessage, NewFrontendPost};

    // opts: ListOptions,
    pub async fn list_frontend(
        db: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        // Just return a JSON array of todos, applying the limit and offset.
        let connection = &mut db.get().unwrap();
        let frontends: Vec<Frontend> = print_frontends(connection);
        // log::info!("    -> todo id not found!");
        //  ("found {} frontends ", frontends.size);
        Ok(warp::reply::json(&frontends))
    }

    pub async fn create_frontend(
        new_tec: NewFrontendPost,
        pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        use crate::schema::frontend;

        //  log::info!("create_frontend: {:?}", create);
        let connection = &mut pool.get().unwrap();

        // insert value into microservice table
        let _id = create_service(connection, new_tec.microservice_id.as_str());
        let result = find_microservice_by_name(pool, new_tec.microservice_id.as_str());
        if result.is_none() {
            let message = format!(
                "can't find microservice id '{}'",
                new_tec.microservice_id.as_str()
            );

            let code = StatusCode::NOT_FOUND;
            let json = warp::reply::json(&ErrorMessage {
                code: code.as_u16(),
                message,
            });
            return Ok(warp::reply::with_status(json, code));
        }

        let new_frontend = NewFrontend {
            microservice_id: new_tec.microservice_id.as_str(),
            service_url: new_tec.service_url.as_str(),
            local_repo_path: new_tec.local_repo_path.as_str(),
            technology_id: new_tec.technology_id,
            url: new_tec.url.as_str(),
        };

        match diesel::insert_into(frontend::table)
            .values(&new_frontend)
            .execute(connection)
        {
            Ok(_iedee) => {
                let message = "created".to_string();
                let code = StatusCode::CREATED;
                let json = warp::reply::json(&ErrorMessage {
                    code: code.as_u16(),
                    message,
                });
                Ok(warp::reply::with_status(json, code))
            }
            Err(e) => {
                let message = format!(
                    "an error occurred inserting a new frontend which we are ignoring '{e}'"
                );

                let code = StatusCode::INTERNAL_SERVER_ERROR;

                let json = warp::reply::json(&ErrorMessage {
                    code: code.as_u16(),
                    message,
                });

                Ok(warp::reply::with_status(json, code))
            }
        }
    }
}
