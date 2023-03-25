// based on
// https://github.com/seanmonstar/warp/blob/master/examples/rejections.rs
// and
// https://github.com/seanmonstar/warp/blob/master/examples/todos.rs

pub mod filters_technology {
    use diesel::r2d2::ConnectionManager;
    use diesel::MysqlConnection;
    use r2d2::Pool;
    use warp::Filter;

    use crate::db::db::with_db;
    use crate::models::rest_models::rest_models::NewTechnologyPost;

    use super::handlers_technology;

    pub fn technology(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let api = warp::path("api");
        api.and(
            technology_list(connection_pool.clone())
                .or(technology_create(connection_pool.clone()))
                .or(technology_find_by_name(connection_pool.clone())),
        )
    }

    /// GET /technology
    pub fn technology_list(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("technology")
            .and(warp::get())
            .and(with_db(connection_pool.clone()))
            .and_then(handlers_technology::list_technologies)
    }

    // POST /technology with JSON body
    pub fn technology_create(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("technology")
            .and(warp::post())
            .and(json_body_new_technology())
            .and(with_db(connection_pool))
            .and_then(handlers_technology::create_technology)
    }

    // GET /technology/name
    pub fn technology_find_by_name(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("technology" / String)
            .and(warp::get())
            .and(with_db(connection_pool))
            .and_then(handlers_technology::find_by_name_technology)
    }

    fn json_body_new_technology(
    ) -> impl Filter<Extract = (NewTechnologyPost,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers_technology {
    use std::convert::Infallible;

    use diesel::r2d2::ConnectionManager;
    use diesel::{MysqlConnection, QueryDsl, RunQueryDsl, SelectableHelper};
    use r2d2::Pool;
    use serde::Serialize;
    use warp::http::StatusCode;
    use warp::log;

    use crate::db::read_data::{print_hosts, print_technologies};
    use crate::diesel::ExpressionMethods;
    use crate::models::models::{NewTechnology, Technology};
    use crate::models::rest_models::rest_models::{ErrorMessage, NewTechnologyPost};

    // opts: ListOptions,
    pub async fn list_technologies(
        db: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let connection = &mut db.get().unwrap();
        let techs: Vec<Technology> = print_technologies(connection);
        // log::info!("    -> todo id not found!");
        //  ("found {} technologies ", techs.size);
        Ok(warp::reply::json(&techs))
    }

    pub async fn find_by_name_technology(
        name: String,
        db: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let connection = &mut db.get().unwrap();
        use crate::schema::technology;
        let x = technology::table
            .filter(crate::schema::technology::name.eq(name))
            .select(Technology::as_select())
            .get_result(connection)
            .expect("expect to find it");
        println!("x {:?}", x);
        Ok(warp::reply::json(&x))
    }

    pub async fn create_technology(
        new_tec: NewTechnologyPost,
        pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        use crate::schema::technology;

        //  log::info!("create_technology: {:?}", create);
        let connection = &mut pool.get().unwrap();

        let new_tec = NewTechnology {
            name: new_tec.name.as_str(),
        };

        match diesel::insert_into(technology::table)
            .values(&new_tec)
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
                    "an error occurred inserting a new technology which we are ignoring '{}'",
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
