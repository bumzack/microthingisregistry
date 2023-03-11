use diesel::prelude::*;

use crate::schema::technology;
use crate::schema::backend;
use crate::schema::frontend;
use crate::schema::host;
use crate::schema::service;

#[derive(Queryable, Identifiable,Associations, Selectable, Debug, PartialEq)]
#[diesel(table_name = backend)]
#[diesel(belongs_to(Service))]
#[diesel(belongs_to(Host))]
#[diesel(belongs_to(Technology))]
pub struct Backend {
    pub id: i32,
    pub openapiclient: Option<String>,
    pub service_url: String,
    pub openapi_url: String,
    pub local_repo_path: String,
    pub host_id: Option<i32>,
    pub service_id: String,
    pub technology_id: i32,
}


#[derive(Queryable, Identifiable,Associations, Selectable, Debug, PartialEq)]
#[diesel(table_name = frontend)]
#[diesel(belongs_to(Service))]
#[diesel(belongs_to(Host))]
#[diesel(belongs_to(Technology))]
pub struct Frontend {
    pub id: i32,
    pub url: String,
    pub version_major: i32,
    pub version_minor: i32,
    pub version_patch: i32,
    pub service_url: String,
    pub openapi_url: String,
    pub local_repo_path: String,
    pub host_id: Option<i32>,
    pub service_id: String,
    pub technology_id: i32,

}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = host)]
pub struct Host {
    pub id: i32,
    pub hostname: String,
    pub ip: String,
    pub port: i32,
}


#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = service)]
pub struct Service {
    pub id: i32,
    pub service_id: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = technology)]
pub struct Technology {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = technology)]
pub struct NewTechnology<'a> {
    pub name: &'a str,
}
