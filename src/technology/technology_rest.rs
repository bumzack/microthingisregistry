// based on
// https://github.com/seanmonstar/warp/blob/master/examples/rejections.rs
// and
// https://github.com/seanmonstar/warp/blob/master/examples/todos.rs

pub mod filters_technology {
    use diesel::MysqlConnection;
    use diesel::r2d2::ConnectionManager;
    use r2d2::Pool;
    use warp::Filter;

    use crate::models::Technology;
    use crate::technology::technology_rest::models::NewTechnologyPost;

    use super::handlers_technology;

    pub fn technology(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        let api = warp::path("api");
        api.and(technology_list(connection_pool.clone())
            .or(technology_create(connection_pool.clone())))
        // .or(todos_update(db.clone()))
        // .or(todos_delete(db))
    }

    /// GET /technology
    pub fn technology_list(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        warp::path!("technology")
            .and(warp::get())
//            .and(warp::query::<ListOptions>())
            .and(with_db(connection_pool.clone()))
            .and_then(handlers_technology::list_technologies)
    }

    // POST /technology with JSON body
    pub fn technology_create(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        warp::path!("technology")
            .and(warp::post())
            .and(json_body_new_technology())
            .and(with_db(connection_pool))
            .and_then(handlers_technology::create_technology)
    }
    //
    // /// PUT /todos/:id with JSON body
    // pub fn todos_update(
    //     db: Db,
    // ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //     warp::path!("todos" / u64)
    //         .and(warp::put())
    //         .and(json_body())
    //         .and(with_db(db))
    //         .and_then(handlers::update_todo)
    // }
    //
    // /// DELETE /todos/:id
    // pub fn todos_delete(
    //     db: Db,
    // ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //     // We'll make one of our endpoints admin-only to show how authentication filters are used
    //     let admin_only = warp::header::exact("authorization", "Bearer admin");
    //
    //     warp::path!("todos" / u64)
    //         // It is important to put the auth check _after_ the path filters.
    //         // If we put the auth check before, the request `PUT /todos/invalid-string`
    //         // would try this filter and reject because the authorization header doesn't match,
    //         // rather because the param is wrong for that other path.
    //         .and(admin_only)
    //         .and(warp::delete())
    //         .and(with_db(db))
    //         .and_then(handlers::delete_todo)
    // }

    fn with_db(db: Pool<ConnectionManager<MysqlConnection>>) -> impl Filter<Extract=(Pool<ConnectionManager<MysqlConnection>>, ), Error=std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }


    fn json_body_new_technology() -> impl Filter<Extract=(NewTechnologyPost, ), Error=warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers_technology {
    use std::convert::Infallible;

    use diesel::{MysqlConnection, RunQueryDsl};
    use diesel::r2d2::ConnectionManager;
    use r2d2::Pool;
    use serde::Serialize;
    use warp::http::StatusCode;
    use warp::log;

    use crate::models::{NewTechnology, Technology};
    use crate::read_data::print_technologies;
    use crate::technology::technology_rest::models::{ErrorMessage, NewTechnologyPost};

    // opts: ListOptions,
    pub async fn list_technologies(db: Pool<ConnectionManager<MysqlConnection>>) -> Result<impl warp::Reply, Infallible> {
        // Just return a JSON array of todos, applying the limit and offset.
        let connection = &mut db.get().unwrap();
        let techs: Vec<Technology> = print_technologies(connection);
        // log::info!("    -> todo id not found!");
        //  ("found {} technologies ", techs.size);
        Ok(warp::reply::json(&techs))
    }

    pub async fn create_technology(new_tec: NewTechnologyPost, pool: Pool<ConnectionManager<MysqlConnection>>) -> Result<impl warp::Reply, Infallible> {
        use crate::schema::technology;

        //  log::info!("create_technology: {:?}", create);
        let connection = &mut pool.get().unwrap();

        let new_tec = NewTechnology {
            name: new_tec.name.as_str()
        };

        match diesel::insert_into(technology::table)
            .values(&new_tec)
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
                let message = format!("an error occurred inserting a new technology which we are ignoring '{}'", e);
                let code = StatusCode::INTERNAL_SERVER_ERROR;

                let json = warp::reply::json(&ErrorMessage {
                    code: code.as_u16(),
                    message: message.into(),
                });

                Ok(warp::reply::with_status(json, code))
            }
        }
    }
    //
    // pub async fn update_todo(
    //     id: u64,
    //     update: Todo,
    //     db: Db,
    // ) -> Result<impl warp::Reply, Infallible> {
    //     log::debug!("update_todo: id={}, todo={:?}", id, update);
    //     let mut vec = db.lock().await;
    //
    //     // Look for the specified Todo...
    //     for todo in vec.iter_mut() {
    //         if todo.id == id {
    //             *todo = update;
    //             return Ok(StatusCode::OK);
    //         }
    //     }
    //
    //     log::debug!("    -> todo id not found!");
    //
    //     // If the for loop didn't return OK, then the ID doesn't exist...
    //     Ok(StatusCode::NOT_FOUND)
    // }
    //
    // pub async fn delete_todo(id: u64, db: Db) -> Result<impl warp::Reply, Infallible> {
    //     log::debug!("delete_todo: id={}", id);
    //
    //     let mut vec = db.lock().await;
    //
    //     let len = vec.len();
    //     vec.retain(|todo| {
    //         // Retain all Todos that aren't this id...
    //         // In other words, remove all that *are* this id...
    //         todo.id != id
    //     });
    //
    //     // If the vec is smaller, we found and deleted a Todo!
    //     let deleted = vec.len() != len;
    //
    //     if deleted {
    //         // respond with a `204 No Content`, which means successful,
    //         // yet no body expected...
    //         Ok(StatusCode::NO_CONTENT)
    //     } else {
    //         log::debug!("    -> todo id not found!");
    //         Ok(StatusCode::NOT_FOUND)
    //     }
    // }
}

mod models {
    use std::sync::Arc;

    use serde_derive::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct NewTechnologyPost {
        pub name: String,
    }

    /// An API error serializable to JSON.
    #[derive(Serialize)]
    pub struct ErrorMessage {
        pub(crate) code: u16,
        pub(crate) message: String,
    }


    //
    // // The query parameters for list_todos.
    // #[derive(Debug, Deserialize)]
    // pub struct ListOptions {
    //     pub offset: Option<usize>,
    //     pub limit: Option<usize>,
    // }
}
