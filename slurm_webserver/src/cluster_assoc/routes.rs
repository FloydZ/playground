#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::cluster_assoc::{ClusterAssoc};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/cluster_assocs")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = ClusterAssoc::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_assocs/{id_assoc}")]
async fn find(id_assoc: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let a = ClusterAssoc::find(id_assoc.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
}