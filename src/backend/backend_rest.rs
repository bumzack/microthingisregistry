// based on
// https://github.com/seanmonstar/warp/blob/master/examples/rejections.rs
// and
// https://github.com/seanmonstar/warp/blob/master/examples/todos.rs

pub mod filters_backend {
    use diesel::r2d2::ConnectionManager;
    use diesel::MysqlConnection;
    use r2d2::Pool;
    use warp::Filter;

    use crate::backend::backend_rest::handlers_backend::get_backend_by_name;
    use crate::db::db::with_db;
    use crate::models::rest_modelss::rest_models::{NewBackendPost, UpdateBackendOpenApiPut};

    use super::handlers_backend;

    pub fn backend(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let api = warp::path("api");
        api.and(
            backend_list(connection_pool.clone())
                .or(backend_create(connection_pool.clone()))
                .or(backend_update_openapi(connection_pool.clone()))
                .or(get_backend_openapi_client(connection_pool.clone()))
                .or(update_openapi_clients(connection_pool.clone()))
                .or(backend_by_name(connection_pool.clone()))
                .or(get_backend_openapi_client_name(connection_pool.clone()))
                .or(get_backend_openapi_package_name(connection_pool.clone()))
                .or(get_backend_as_frontend(connection_pool.clone())),
        )
    }

    /// GET /backend
    pub fn backend_list(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("backend")
            .and(warp::get())
            .and(with_db(connection_pool.clone()))
            .and_then(handlers_backend::list_backend)
    }

    /// GET /backend/:name
    pub fn backend_by_name(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("backend" / String)
            .and(warp::get())
            .and(with_db(connection_pool.clone()))
            .and_then(handlers_backend::get_backend_by_name)
    }

    /// GET /backend
    pub fn update_openapi_clients(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("openapiclient" / "update")
            .and(warp::get())
            .and(with_db(connection_pool.clone()))
            .and_then(handlers_backend::update_openapi_clients)
    }

    pub fn get_backend_openapi_client(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("backend" / "openapiclient" / String)
            .and(warp::get())
            .and(with_db(connection_pool.clone()))
            .and_then(handlers_backend::backend_openapi_client)
    }

    pub fn get_backend_openapi_client_name(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("backend" / "apiclientprefix" / String)
            .and(warp::get())
            .and(with_db(connection_pool.clone()))
            .and_then(handlers_backend::backend_apiclientprefix)
    }

    pub fn get_backend_openapi_package_name(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("backend" / "apiclientpackage" / String)
            .and(warp::get())
            .and(with_db(connection_pool.clone()))
            .and_then(handlers_backend::backend_apiclientpackage)
    }

    pub fn get_backend_as_frontend(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("backend" / "asfrontend" / "all")
            .and(warp::get())
            .and(with_db(connection_pool.clone()))
            .and_then(handlers_backend::backend_as_frontend)
    }

    // POST /backend with JSON body
    pub fn backend_create(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("backend")
            .and(warp::post())
            .and(json_body_new_backend())
            .and(with_db(connection_pool))
            .and_then(handlers_backend::create_backend)
    }

    // POST /backend with JSON body
    pub fn backend_update_openapi(
        connection_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("backend" / String)
            .and(warp::put())
            .and(json_body_update_openapi())
            .and(with_db(connection_pool))
            .and_then(handlers_backend::update_backend)
    }

    fn json_body_new_backend(
    ) -> impl Filter<Extract = (NewBackendPost,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    fn json_body_update_openapi(
    ) -> impl Filter<Extract = (UpdateBackendOpenApiPut,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers_backend {
    use std::convert::Infallible;

    use diesel::r2d2::ConnectionManager;
    use diesel::{MysqlConnection, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};
    use r2d2::Pool;
    use warp::http::StatusCode;

    use crate::db::create_data::create_service;
    use crate::db::read_data::print_backends;
    use crate::diesel::ExpressionMethods;
    use crate::microservice::microservice::{find_backend_by_name, find_microservice_by_name};
    use crate::models::models::{Backend, NewBackend};
    use crate::models::rest_modelss::rest_models::{
        ErrorMessage, NewBackendPost, UpdateBackendOpenApiPut,
    };

    // opts: ListOptions,
    pub async fn list_backend(
        db: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        // Just return a JSON array of todos, applying the limit and offset.
        let connection = &mut db.get().unwrap();
        let techs: Vec<Backend> = print_backends(connection);
        // log::info!("    -> todo id not found!");
        //  ("found {} technologies ", techs.size);
        Ok(warp::reply::json(&techs))
    }

    pub async fn get_backend_by_name(
        ms_id: String,
        db: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let be = find_backend_by_name(db, ms_id.as_str());
        match be {
            Some(b) => Ok(warp::reply::with_status(
                warp::reply::json(&b),
                StatusCode::OK,
            )),

            None => {
                let message = format!("can not find  backend {}", &ms_id);
                let code = StatusCode::NOT_FOUND;
                let json = warp::reply::json(&ErrorMessage {
                    code: code.as_u16(),
                    message: message.into(),
                });
                Ok(warp::reply::with_status(json, code))
            }
        }
    }

    pub async fn backend_as_frontend(
        db: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let connection = &mut db.get().unwrap();
        println!("looking for backends  publish_as_frontend_package = true ");
        use crate::schema::backend;

        let backends = backend::dsl::backend
            .filter(backend::publish_as_frontend_package.eq(true))
            .order(backend::microservice_id.asc())
            .load::<Backend>(connection);

        let list: Vec<String> = match backends {
            Ok(b) => b.into_iter().map(|b| b.microservice_id).collect(),
            Err(_) => {
                vec![]
            }
        };
        let resp = list.join("\n");
        Ok(warp::reply::with_status(resp, StatusCode::OK))
    }

    pub async fn backend_openapi_client(
        ms_id: String,
        db: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let be = find_backend_by_name(db, ms_id.as_str()).unwrap();
        Ok(warp::reply::with_status(
            be.openapiclient.unwrap(),
            StatusCode::OK,
        ))
    }

    pub async fn backend_apiclientpackage(
        ms_id: String,
        db: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let be = find_backend_by_name(db, ms_id.as_str()).unwrap();
        Ok(warp::reply::with_status(
            be.api_client_prefix,
            StatusCode::OK,
        ))
    }

    pub async fn backend_apiclientprefix(
        ms_id: String,
        db: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let be = find_backend_by_name(db, ms_id.as_str()).unwrap();
        Ok(warp::reply::with_status(
            be.api_client_package,
            StatusCode::OK,
        ))
    }

    pub async fn update_openapi_clients(
        db: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let connection = &mut db.get().unwrap();
        let backends = print_backends(connection);

        let total = backends.len();
        let mut successes: i32 = 0;

        for be in &backends {
            let url = be.openapi_url.clone();
            println!("openapi url for ms {}: {}", be.microservice_id, &url);
            let res = reqwest::get(url).await.unwrap();

            println!("Response: {:?} {}", res.version(), res.status());
            println!("Headers: {:#?}\n", res.headers());
            let body = res.text().await.unwrap();

            let result = match update_openapi_client(&be.microservice_id, body, db.clone()) {
                Ok(iedee) => 1,
                Err(e) => {
                    println!(
                        "an error occurred while updating  backend {}  which we are ignoring '{}'",
                        &be.microservice_id, e
                    );

                    0
                }
            };
            successes += result;
        }

        let message = format!(
            "updated OpenAPI clients. total backends {total}. successes updating {successes}"
        );

        let code = StatusCode::OK;
        let json = warp::reply::json(&ErrorMessage {
            code: code.as_u16(),
            message: message.into(),
        });
        Ok(warp::reply::with_status(json, code))
    }

    pub async fn create_backend(
        new_tec: NewBackendPost,
        pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        use crate::schema::backend;

        println!("adding new service {:?}", &new_tec);
        //  log::info!("create_technology: {:?}", create);
        let connection = &mut pool.get().unwrap();

        // insert value into microservice table
        let id = create_service(connection, new_tec.microservice_id.as_str());
        let result = find_microservice_by_name(pool, new_tec.microservice_id.as_str());
        if result.is_none() {
            let message = format!(
                "can't find microservice id '{}'",
                new_tec.microservice_id.as_str()
            );

            let code = StatusCode::NOT_FOUND;
            let json = warp::reply::json(&ErrorMessage {
                code: code.as_u16(),
                message: message.into(),
            });
            return Ok(warp::reply::with_status(json, code));
        }

        let new_backend = NewBackend {
            microservice_id: new_tec.microservice_id.as_str(),
            service_url: new_tec.service_url.as_str(),
            openapi_url: new_tec.openapi_url.as_str(),
            local_repo_path: new_tec.local_repo_path.as_str(),
            technology_id: new_tec.technology_id,
            api_client_prefix: new_tec.api_client_prefix.as_str(),
            publish_as_frontend_package: new_tec.publish_as_frontend_package,
            api_client_package: new_tec.api_client_package.as_str(),
        };

        match diesel::insert_into(backend::table)
            .values(&new_backend)
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
                    "an error occurred inserting a new backend which we are ignoring '{e}'"
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

    pub async fn update_backend(
        ms_id: String,
        update: UpdateBackendOpenApiPut,
        pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<impl warp::Reply, Infallible> {
        let result = update_openapi_client(&ms_id, update.openapiclient, pool);

        match result {
            Ok(_iedee) => {
                let message = format!("updated open api client for {}", &ms_id);
                let code = StatusCode::OK;
                let json = warp::reply::json(&ErrorMessage {
                    code: code.as_u16(),
                    message: message.into(),
                });
                Ok(warp::reply::with_status(json, code))
            }
            Err(e) => {
                let message = format!(
                    "an error occurred while updating  backend {}  which we are ignoring '{}'",
                    &ms_id, e
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

    pub fn update_openapi_client(
        ms_id: &String,
        openapi: String,
        pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> QueryResult<usize> {
        use crate::schema::backend::dsl::backend;
        use crate::schema::backend::microservice_id;
        use crate::schema::backend::openapiclient;
        println!(
            "updating microservice new service {:?} with data {:?}",
            &ms_id, openapi
        );

        let connection = &mut pool.get().unwrap();

        diesel::update(backend)
            .filter(microservice_id.eq(&ms_id))
            .set(openapiclient.eq(openapi))
            .execute(connection)
    }
}
