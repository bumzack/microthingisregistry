use std::env;

use diesel::mysql;
use diesel::prelude::*;
use dotenvy::dotenv;

use crate::models::{NewTechnology, Technology};

use self::schema::technology::dsl::*;

pub mod models;
pub mod schema;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_technology(conn: &mut MysqlConnection, other_name: &str) -> usize {
    use crate::schema::technology;

    let new_tec = NewTechnology { name: other_name };

    diesel::insert_into(technology::table)
        .values(&new_tec)
        .execute(conn)
        .expect("Error saving new technology")
}


fn main() {
    let mut connection = &mut establish_connection();

    let string = "java".to_string();
    let java = string.as_str() ;
    let string1 = "typescript".to_string();
    let typescript = string1.as_str();
    let x = create_technology(&mut connection, &java);
    println!("insert id {:?}", x);
    let y = create_technology(&mut connection, &typescript);
    println!("insert id {:?}", y);


    let results = technology
        //   .filter(published.eq(true))
        .limit(5)
        .load::<Technology>(connection)
        .expect("Error loading technologies");

    println!("Displaying {} technologies", results.len());
    for tech in results {
        println!("{}", tech.id);
        println!("-----------\n");
        println!("{}", tech.name);
    }
}

