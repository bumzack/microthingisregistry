extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::env;

use diesel::mysql;
use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

use crate::models::{Backend, Frontend, Host, NewTechnology, Service, Technology};

use self::schema::backend::dsl::*;
use self::schema::frontend::dsl::*;
use self::schema::host::dsl::*;
use self::schema::service::dsl::*;
//
use self::schema::technology::dsl::*;

//
pub mod models;
pub mod schema;

//
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = establish_connection();
    //
    // println!("Migrating...");
    // connection.run_pending_migrations(MIGRATIONS).unwrap();
    // Ok(())

    print_technologies(&mut connection);
    print_services(&mut connection);
    print_hosts(&mut connection);
    print_frontends(&mut connection);
    print_backends(&mut connection);

    Ok(())
}

fn print_technologies(connection: &mut MysqlConnection) {
    let technologies = technology
        //   .filter(published.eq(true))
        .load::<Technology>(connection)
        .expect("Error loading technologies");

    println!("Displaying {} technologies", technologies.len());
    for tech in technologies {
        println!("{}", tech.id);
        println!("-----------\n");
        println!("{}", tech.name);
    }
}

fn print_services(connection: &mut MysqlConnection) {
    let services = service
        //   .filter(published.eq(true))
        .load::<Service>(connection)
        .expect("Error loading technologies");

    println!("Displaying {} services", services.len());
    for s in services {
        println!("service  {} / {}   ", s.id, s.service_id);
    }
}

fn print_hosts(connection: &mut MysqlConnection) {
    let hosts = host
        //   .filter(published.eq(true))
        .load::<Host>(connection)
        .expect("Error loading technologies");

    println!("Displaying {} services", hosts.len());
    for h in hosts {
        println!("host  {} / {} / {} / {}   ", h.id, h.hostname, h.ip, h.port);
    }
}


fn print_frontends(connection: &mut MysqlConnection) {
    let frontends = frontend
        //   .filter(published.eq(true))
        .load::<Frontend>(connection)
        .expect("Error loading technologies");

    println!("Displaying {} services", frontends.len());
    for fe in frontends {
        println!("{}", fe.id);
        println!("-----------\n");
        println!("frontend  {} / {} / {} / {}   ", fe.id, fe.service_url, fe.openapi_url, fe.url);
    }
}

fn print_backends(connection: &mut MysqlConnection) {
    let backends = backend
        //   .filter(published.eq(true))
        .load::<Backend>(connection)
        .expect("Error loading backend");

    println!("Displaying {} backends", backends.len());
    for b in backends {
        println!("backend {} / {} / {} / {}   ", b.id, b.service_url, b.openapi_url, b.local_repo_path);
    }
}
