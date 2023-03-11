extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::env;

use diesel::mysql;
use diesel::prelude::*;
use dotenvy::dotenv;

use crate::diesel::associations::HasTable;
use crate::models::{Backend, Frontend, Host, MicroService, NewBackend, NewFrontend, NewHost, NewMicroService, NewTechnology, Technology};

// use self::schema::backend::dsl::*;
// use self::schema::frontend::dsl::*;
// use self::schema::host::dsl::*;
// use self::schema::microservice::dsl::*;
// use self::schema::technology::dsl::*;

pub mod models;
pub mod schema;

//
// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

//
//
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// pub fn create_technology(conn: &mut MysqlConnection, other_name: &str) -> usize {
//     use crate::schema::technology;
//
//     let new_tec = NewTechnology { name: other_name };
//
//     diesel::insert_into(technology::table)
//         .values(&new_tec)
//         .execute(conn)
//         .expect("Error saving new technology")
// }
//
//
// fn main2() {
//     let mut connection = &mut establish_connection();
//
//     let string = "java".to_string();
//     let java = string.as_str() ;
//     let string1 = "typescript".to_string();
//     let typescript = string1.as_str();
//     let x = create_technology(&mut connection, &java);
//     println!("insert id {:?}", x);
//     let y = create_technology(&mut connection, &typescript);
//     println!("insert id {:?}", y);
//
//
//     let results = technology
//         //   .filter(published.eq(true))
//         .limit(5)
//         .load::<Technology>(connection)
//         .expect("Error loading technologies");
//
//     println!("Displaying {} technologies", results.len());
//     for tech in results {
//         println!("{}", tech.id);
//         println!("-----------\n");
//         println!("{}", tech.name);
//     }
// }

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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = establish_connection();
    //
    // println!("Migrating...");
    // connection.run_pending_migrations(MIGRATIONS).unwrap();
    // Ok(())

    insert_technologies(&mut connection);
    insert_hosts(&mut connection);
    insert_services(&mut connection);

    print_technologies(&mut connection);
    print_services(&mut connection);
    print_hosts(&mut connection);
    print_frontends(&mut connection);
    print_backends(&mut connection);

    insert_frontends(&mut connection);
    insert_backends(&mut connection);

    Ok(())
}

fn insert_backends(connection: &mut MysqlConnection) {
    let backends = vec![
        ("solrsearchproduct", "/search/product", "/swagger.html", "/Users/bumzack/repo1", 4),
        ("solrsearchcategory", "/search/category", "/swagger.html", "/Users/bumzack/repo2", 4),
        ("solrsearchimage", "/search/image", "/swagger.html", "/Users/bumzack/repo3", 4),
    ];
    backends.into_iter().for_each(|s| {
        let x = create_backend(connection, s.0, s.1, s.2, s.3, s.4);
        println!("inserted backend id {:?}", x);
    });
}


fn insert_frontends(connection: &mut MysqlConnection) {
    let frontends = vec![
        ("solrsearchproduct", "/search/product", "/Users/bumzack/repo1", "/dist/index.js", 4),
        ("solrsearchcategory", "/search/category", "/Users/bumzack/repo2", "/dist/index.js", 4),
        ("solrsearchimage", "/search/image", "/Users/bumzack/repo3", "/dist/index.js", 4),
    ];
    frontends.into_iter().for_each(|s| {
        let x = create_frontend(connection, s.0, s.1, s.2, s.3, s.4);
        println!("inserted frontend id {:?}", x);
    });
}

fn insert_hosts(connection: &mut MysqlConnection) {
    let hosts = vec![
        ("solrsearchproduct.foryouandyourfakewebshop.wtf", "127.0.0.1", 9000),
        ("solrsearchcategory.foryouandyourfakewebshop.wtf", "127.0.0.1", 9001),
        ("solrsearchimage.foryouandyourfakewebshop.wtf", "127.0.0.1", 9002),
    ];
    hosts.into_iter().for_each(|s| {
        let x = create_host(connection, s.0, s.1, s.2);
        println!("inserted hosts id {:?}", x);
    });
}

fn insert_services(connection: &mut MysqlConnection) {
    let names = vec!["solrsearchproduct", "solrsearchcategory", "solrsearchimage"];
    names.into_iter().for_each(|s| {
        let x = create_service(connection, s);
        println!("inserted services id {:?}", x);
    });
}

fn insert_technologies(connection: &mut MysqlConnection) {
    let names = vec!["rust", "typescript", "webflux", "java8"];
    names.into_iter().for_each(|s| {
        let x = create_technology(connection, s);
        println!("inserted services id {:?}", x);
    });
}

fn print_technologies(connection: &mut MysqlConnection) {
    use crate::schema::technology;
    let technologies = crate::schema::technology::dsl::technology
        //   .filter(published.eq(true))
        .order(crate::schema::technology::id.asc())
        .load::<Technology>(connection)
        .expect("Error loading technologies");

    println!("Displaying {} technologies", technologies.len());
    for tech in technologies {
        println!("technology  {} / {} ", tech.id, tech.name);
    }
}

fn print_services(connection: &mut MysqlConnection) {
    use crate::schema::microservice;
    let services = crate::schema::microservice::dsl::microservice
        .order(crate::schema::microservice::id.asc())

        //   .filter(published.eq(true))
        .load::<MicroService>(connection)
        .expect("Error loading technologies");

    println!("Displaying {} services", services.len());
    for s in services {
        println!("service  {} / {}   ", s.id, s.microservice_id);
    }
}

fn print_hosts(connection: &mut MysqlConnection) {
    use crate::schema::host;
    let hosts = crate::schema::host::dsl::host
        //   .filter(published.eq(true))
        .order(crate::schema::host::id.asc())
        .load::<Host>(connection)
        .expect("Error loading technologies");

    println!("Displaying {} services", hosts.len());
    for h in hosts {
        println!("host  {} / {} / {} / {}   ", h.id, h.hostname, h.ip, h.port);
    }
}


fn print_frontends(connection: &mut MysqlConnection) {
    use crate::schema::frontend;
    let frontends = crate::schema::frontend::dsl::frontend
        //   .filter(published.eq(true))
        .order(crate::schema::frontend::id.asc())
        .load::<Frontend>(connection)
        .expect("Error loading technologies");

    println!("Displaying {} services", frontends.len());
    for fe in frontends {
        println!("{}", fe.id);
        println!("-----------\n");
        println!("frontend  {} / {} / {}   ", fe.id, fe.service_url, fe.url);
    }
}

fn print_backends(connection: &mut MysqlConnection) {
    use crate::schema::backend;
    let backends = crate::schema::backend::dsl::backend
        //   .filter(published.eq(true))
        .order(crate::schema::backend::id.asc())
        .load::<Backend>(connection)
        .expect("Error loading backend");

    println!("Displaying {} backends", backends.len());
    for b in backends {
        println!("backend {} / {} / {} / {}   ", b.id, b.service_url, b.openapi_url, b.local_repo_path);
    }
}
