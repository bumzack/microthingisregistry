use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::backend;
use crate::schema::frontend;
use crate::schema::host;
use crate::schema::microservice;
use crate::schema::technology;

#[derive(Queryable, Identifiable, Associations, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = backend)]
#[diesel(belongs_to(MicroService, foreign_key = microservice_id))]
#[diesel(belongs_to(Host))]
#[diesel(belongs_to(Technology))]
pub struct Backend {
    pub id: i32,
    pub openapiclient: Option<String>,
    pub service_url: String,
    pub openapi_url: String,
    pub local_repo_path: String,
    pub host_id: Option<i32>,
    pub microservice_id: String,
    pub technology_id: i32,
    pub publish_as_frontend_package: bool,
    pub api_client_prefix: String,
}


#[derive(Queryable, Identifiable, Associations, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = frontend)]
#[diesel(belongs_to(MicroService, foreign_key = microservice_id))]
#[diesel(belongs_to(Host))]
#[diesel(belongs_to(Technology))]
pub struct Frontend {
    pub id: i32,
    pub url: String,
    pub version_major: i32,
    pub version_minor: i32,
    pub version_patch: i32,
    pub service_url: String,
    pub local_repo_path: String,
    pub host_id: Option<i32>,
    pub microservice_id: String,
    pub technology_id: i32,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = host)]
pub struct Host {
    pub id: i32,
    pub hostname: String,
    pub ip: String,
    pub port: i32,
}


#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Deserialize, Serialize)]
#[diesel(table_name = microservice)]
pub struct MicroService {
    pub id: i32,
    pub microservice_id: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Deserialize, Serialize)]
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


#[derive(Insertable)]
#[diesel(table_name = host)]
pub struct NewHost<'a> {
    pub hostname: &'a str,
    pub ip: &'a str,
    pub port: i32,
}

#[derive(Insertable)]
#[diesel(table_name = microservice)]
pub struct NewMicroService<'a> {
    pub microservice_id: &'a str,
}


#[derive(Insertable)]
#[diesel(table_name = backend)]
pub struct NewBackend<'a> {
    pub microservice_id: &'a str,
    pub service_url: &'a str,
    pub openapi_url: &'a str,
    pub local_repo_path: &'a str,
    pub technology_id: i32,
    pub api_client_prefix: &'a str,
    pub publish_as_frontend_package: bool,
}

#[derive(Insertable)]
#[diesel(table_name = frontend)]
pub struct NewFrontend<'a> {
    pub microservice_id: &'a str,
    pub service_url: &'a str,
    pub local_repo_path: &'a str,
    pub technology_id: i32,
    pub url: &'a str,
}

