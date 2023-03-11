extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::env;

use diesel::mysql;
use diesel::prelude::*;

use crate::insert_data::{insert_backends, insert_frontends, insert_hosts, insert_services, insert_technologies};
use crate::read_data::{print_backends, print_frontends, print_hosts, print_services, print_technologies};
use crate::utils::establish_connection;

mod utils;
mod create_data;
mod insert_data;
mod read_data;

pub mod models;
pub mod schema;


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

    insert_frontends(&mut connection);
    insert_backends(&mut connection);

    print_frontends(&mut connection);
    print_backends(&mut connection);

    Ok(())
}
