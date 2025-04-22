#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::cluster_assoc_usage_day::{ClusterAssocUsageDay};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/cluster_assoc_usage_days")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = ClusterAssocUsageDay::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_assoc_usage_days/id/{id}")]
async fn findById(id: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let a = ClusterAssocUsageDay::findById(id.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_assoc_usage_days/id_tres/{id_tres}")]
async fn findbyTres(id_tres: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let a = ClusterAssocUsageDay::findByIdTres(id_tres.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_assoc_usage_days/time_start/{time_start}")]
async fn findbyStartTime(time_start: web::Path<i64>) -> Result<HttpResponse, ApiError> {
    let a = ClusterAssocUsageDay::findByTimeStarts(time_start.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}



pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(findById);
    cfg.service(findbyTres);
    cfg.service(findbyStartTime);

}