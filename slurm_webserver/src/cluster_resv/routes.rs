#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::cluster_resv::{ClusterResv};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/cluster_resvs")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = ClusterResv::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_resvs/id/{id}")]
async fn findById(id: web::Path<u32>) -> Result<HttpResponse, ApiError> {
    let a = ClusterResv::findById(id.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_resvs/time_start/{time_start}")]
async fn findByTimeStart(time_start: web::Path<u64>) -> Result<HttpResponse, ApiError> {
    let a = ClusterResv::findByTimeStart(time_start.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(findByTimeStart);
    cfg.service(findById);
}