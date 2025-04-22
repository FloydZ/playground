#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::cluster_wckey::{ClusterWckey};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/cluster_wckeys")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = ClusterWckey::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_wckeys/{id_assoc}")]
async fn find(id_assoc: web::Path<u32>) -> Result<HttpResponse, ApiError> {
    let a = ClusterWckey::find(id_assoc.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
}