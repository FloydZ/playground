#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::cluster::{Cluster};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/clusters")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = Cluster::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/clusters/{time_start}")]
async fn find(name: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let a = Cluster::find(name.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}



pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
}