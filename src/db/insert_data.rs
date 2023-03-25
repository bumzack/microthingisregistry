use diesel::mysql;
use diesel::prelude::*;
use dotenvy::dotenv;

use crate::db::create_data::{
    create_backend, create_frontend, create_host, create_service, create_technology,
};
use crate::diesel::associations::HasTable;

pub fn insert_backends(connection: &mut MysqlConnection) {
    let backends = vec![
        (
            "solrsearchproduct",
            "/search/product",
            "/swagger.html",
            "/Users/bumzack/repo1",
            4,
        ),
        (
            "solrsearchcategory",
            "/search/category",
            "/swagger.html",
            "/Users/bumzack/repo2",
            4,
        ),
        (
            "solrsearchimage",
            "/search/image",
            "/swagger.html",
            "/Users/bumzack/repo3",
            4,
        ),
    ];
    backends.into_iter().for_each(|s| {
        let x = create_backend(connection, s.0, s.1, s.2, s.3, s.4);
        println!("inserted backend id {:?}", x);
    });
}

pub fn insert_frontends(connection: &mut MysqlConnection) {
    let frontends = vec![
        (
            "searchcomponent",
            "/search/product",
            "/Users/bumzack/repo1",
            "/dist/index.js",
            2,
        ),
        (
            "articelistcomponent",
            "/search/category",
            "/Users/bumzack/repo2",
            "/dist/index.js",
            2,
        ),
        (
            "solrsearchimage",
            "/search/image",
            "/Users/bumzack/repo3",
            "/dist/index.js",
            2,
        ),
    ];
    frontends.into_iter().for_each(|s| {
        let x = create_frontend(connection, s.0, s.1, s.2, s.3, s.4);
        println!("inserted frontend id {:?}", x);
    });
}

pub fn insert_hosts(connection: &mut MysqlConnection) {
    let hosts = vec![
        (
            "solrsearchproduct.foryouandyourfakewebshop.wtf",
            "127.0.0.1",
            9000,
        ),
        (
            "solrsearchcategory.foryouandyourfakewebshop.wtf",
            "127.0.0.1",
            9001,
        ),
        (
            "solrsearchimage.foryouandyourfakewebshop.wtf",
            "127.0.0.1",
            9002,
        ),
    ];
    hosts.into_iter().for_each(|s| {
        let x = create_host(connection, s.0, s.1, s.2);
        println!("inserted hosts id {:?}", x);
    });
}

pub fn insert_services(connection: &mut MysqlConnection) {
    let names = vec!["solrsearchproduct", "solrsearchcategory", "solrsearchimage"];
    names.into_iter().for_each(|s| {
        let x = create_service(connection, s);
        println!("inserted services id {:?}", x);
    });
}

pub fn insert_technologies(connection: &mut MysqlConnection) {
    let names = vec!["rust", "typescript", "webflux", "java8"];
    names.into_iter().for_each(|s| {
        let x = create_technology(connection, s);
        println!("inserted services id {:?}", x);
    });
}
