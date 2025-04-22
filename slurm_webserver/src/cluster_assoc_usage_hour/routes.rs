#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::cluster_assoc_usage_hour::{ClusterAssocUsageHour};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/cluster_assoc_usage_hours")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = ClusterAssocUsageHour::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_assoc_usage_hours/id/{id}")]
async fn findbyId(id: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let a = ClusterAssocUsageHour::findById(id.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_assoc_usage_hours/id_tres/{id_tres}")]
async fn findByTres(id_tres: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let a = ClusterAssocUsageHour::findByIdTres(id_tres.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_assoc_usage_hours/time_start/{time_start}")]
async fn findbyTimeStart(time_start: web::Path<i64>) -> Result<HttpResponse, ApiError> {
    let a = ClusterAssocUsageHour::findByTimeStarts(time_start.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(findbyId);
    cfg.service(findByTres);
    cfg.service(findbyTimeStart);

}