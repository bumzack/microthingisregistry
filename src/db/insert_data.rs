use diesel::prelude::*;
use log::info;

use crate::db::create_data::create_technology;

pub fn insert_technologies(connection: &mut MysqlConnection) {
    let names = vec!["rust", "typescript", "webflux", "java8"];
    names.into_iter().for_each(|s| {
        let x = create_technology(connection, s);
        info!("inserted services id {x}");
    });
}
