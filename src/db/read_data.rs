use std::env;

use diesel::mysql;
use diesel::prelude::*;
use dotenvy::dotenv;

use crate::diesel::associations::HasTable;
use crate::models::models::{Backend, Frontend, Host, MicroService, Technology};

pub fn print_technologies(connection: &mut MysqlConnection) -> Vec<Technology> {
    use crate::schema::technology;
    let technologies = technology::dsl::technology
        //   .filter(published.eq(true))
        .order(technology::id.asc())
        .load::<Technology>(connection)
        .expect("Error loading technologies");

    println!("Displaying {} technologies", technologies.len());
    for tech in &technologies {
        println!("technology  {} / {} ", tech.id, tech.name);
    }
    technologies
}

pub fn print_services(connection: &mut MysqlConnection) -> Vec<MicroService> {
    use crate::schema::microservice;
    let services = microservice::dsl::microservice
        .order(microservice::id.asc())
        .load::<MicroService>(connection)
        .expect("Error loading technologies");

    println!("Displaying {} services", services.len());
    for s in &services {
        println!("service  {} / {}   ", s.id, s.microservice_id);
    }
    services
}

pub fn print_hosts(connection: &mut MysqlConnection) -> Vec<Host> {
    use crate::schema::host;
    let hosts = crate::schema::host::dsl::host
        //   .filter(published.eq(true))
        .order(host::id.asc())
        .load::<Host>(connection)
        .expect("Error loading technologies");

    println!("Displaying {} services", hosts.len());
    for h in &hosts {
        println!("host  {} / {} / {} / {}   ", h.id, h.hostname, h.ip, h.port);
    }
    hosts
}

pub fn print_frontends(connection: &mut MysqlConnection) -> Vec<Frontend> {
    use crate::schema::frontend;
    let frontends = frontend::dsl::frontend
        //   .filter(published.eq(true))
        .order(frontend::id.asc())
        .load::<Frontend>(connection)
        .expect("Error loading technologies");

    println!("Displaying {} services", frontends.len());
    for fe in &frontends {
        println!("frontend  {} / {} / {}   ", fe.id, fe.service_url, fe.url);
    }
    frontends
}

pub fn print_backends(connection: &mut MysqlConnection) -> Vec<Backend> {
    use crate::schema::backend;
    let backends = backend::dsl::backend
        //   .filter(published.eq(true))
        .order(backend::id.asc())
        .load::<Backend>(connection)
        .expect("Error loading backend");

    println!("Displaying {} backends", backends.len());
    for b in &backends {
        println!(
            "backend {} / {} / {} / {}   ",
            b.id, b.service_url, b.openapi_url, b.local_repo_path
        );
    }
    backends
}
