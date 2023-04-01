use diesel::prelude::*;

use crate::models::models::{NewMicroService, NewTechnology};

pub fn create_technology(conn: &mut MysqlConnection, other_name: &str) -> usize {
    use crate::schema::technology;
    let new_tec = NewTechnology { name: other_name };
    match diesel::insert_into(technology::table)
        .values(&new_tec)
        .execute(conn)
    {
        Ok(iedee) => iedee,
        Err(e) => {
            println!("an error occurred inserting a new technology which we are ignoring '{e}'");

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
        .execute(conn)
    {
        Ok(iedee) => iedee,
        Err(e) => {
            println!("an error occurred inserting a new serivce which we are ignoring '{e}'",);

            0
        }
    }
}
