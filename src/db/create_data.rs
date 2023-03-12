use diesel::mysql;
use diesel::prelude::*;
use dotenvy::dotenv;

use crate::diesel::associations::HasTable;
use crate::models::models::{NewBackend, NewFrontend, NewHost, NewMicroService, NewTechnology};

pub fn create_technology(conn: &mut MysqlConnection, other_name: &str) -> usize {
    use crate::schema::technology;
    let new_tec = NewTechnology { name: other_name };
    match diesel::insert_into(technology::table)
        .values(&new_tec)
        .execute(conn) {
        Ok(iedee) => iedee,
        Err(e) => {
            println!("an error occurred inserting a new technology which we are ignoring '{}'", e);
            0
        }
    }
}

pub fn create_host(conn: &mut MysqlConnection, new_host_name: &str, new_ip: &str, new_port: i32) -> usize {
    use crate::schema::host;
    let new_host = NewHost {
        hostname: new_host_name,
        ip: new_ip,
        port: new_port,
    };
    match diesel::insert_into(host::table)
        .values(&new_host)
        .execute(conn) {
        Ok(iedee) => iedee,
        Err(e) => {
            println!("an error occurred inserting a new host which we are ignoring '{}'", e);
            0
        }
    }
}

pub fn create_service(conn: &mut MysqlConnection, new_service_id: &str) -> usize {
    use crate::schema::microservice;
    let new_service = NewMicroService {
        microservice_id: new_service_id,
    };
    match diesel::insert_into(microservice::table)
        .values(&new_service)
        .execute(conn) {
        Ok(iedee) => iedee,
        Err(e) => {
            println!("an error occurred inserting a new serivce which we are ignoring '{}'", e);
            0
        }
    }
}

pub fn create_backend(conn: &mut MysqlConnection,
                      new_service_id: &str,
                      new_service_url: &str,
                      new_openapi_url: &str,
                      new_local_repo_path: &str,
                      new_technology_id: i32) -> usize {
    use crate::schema::backend;
    let new_backend = NewBackend {
        microservice_id: new_service_id,
        service_url: new_service_url,
        openapi_url: new_openapi_url,
        local_repo_path: new_local_repo_path,
        technology_id: new_technology_id,
        api_client_prefix: "apiclientprefix",
        publish_as_frontend_package: false,
    };
    match diesel::insert_into(backend::table)
        .values(&new_backend)
        .execute(conn) {
        Ok(iedee) => iedee,
        Err(e) => {
            println!("an error occurred inserting a new backend which we are ignoring '{}'", e);
            0
        }
    }
}

pub fn create_frontend(conn: &mut MysqlConnection,
                       new_service_id: &str,
                       new_service_url: &str,
                       new_local_repo_path: &str,
                       new_url: &str,
                       new_technology_id: i32) -> usize {
    use crate::schema::frontend;
    let new_frontend = NewFrontend {
        microservice_id: new_service_id,
        service_url: new_service_url,
        local_repo_path: new_local_repo_path,
        technology_id: new_technology_id,
        url: new_url,
    };
    match diesel::insert_into(frontend::table)
        .values(&new_frontend)
        .execute(conn) {
        Ok(iedee) => iedee,
        Err(e) => {
            println!("an error occurred inserting a new frontend which we are ignoring '{}'", e);
            0
        }
    }
}

