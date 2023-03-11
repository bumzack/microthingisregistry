extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::env;

use diesel::mysql;
use diesel::prelude::*;

use crate::insert_data::{insert_backends, insert_frontends, insert_hosts, insert_services, insert_technologies};
use crate::read_data::{print_backends, print_frontends, print_hosts, print_services, print_technologies};
use crate::utils::{establish_connection, get_connection_pool};

mod utils;
mod create_data;
mod insert_data;
mod read_data;

pub mod models;
pub mod schema;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_connection_pool();    //
    // println!("Migrating...");
    // connection.run_pending_migrations(MIGRATIONS).unwrap();
    // Ok(())

    let connection = &mut pool.get().unwrap();
    insert_technologies(connection);
    insert_hosts(connection);
    insert_services(connection);

    print_technologies(connection);
    print_services(connection);
    print_hosts(connection);

    insert_frontends(connection);
    insert_backends(connection);

    print_frontends(connection);
    print_backends(connection);

    Ok(())
}
