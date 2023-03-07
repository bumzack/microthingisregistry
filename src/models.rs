use diesel::prelude::*;

use crate::schema::technology;

#[derive(Queryable)]
pub struct Backend {
    pub id: i32,
    pub backendid: String,
    pub openapiclient: Option<String>,
}

#[derive(Queryable)]
pub struct Frontend {
    pub id: i32,
    pub frontendid: String,
    pub url: String,
    pub version_major: i32,
    pub version_minor: i32,
    pub verion_patch: i32,
}


#[derive(Queryable)]
pub struct Technology {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = technology)]
pub struct NewTechnology<'a> {
    pub name: &'a str,
}
