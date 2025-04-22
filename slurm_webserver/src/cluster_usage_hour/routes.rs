#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::cluster_usage_hour::{ClusterUsageHour};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/cluster_usage_hours")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = ClusterUsageHour::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_usage_hours/id/{id}")]
async fn findById(id: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let a = ClusterUsageHour::findById(id.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_usage_hours/time_start/{time_start}")]
async fn findbyStartTime(time_start: web::Path<u64>) -> Result<HttpResponse, ApiError> {
    let a = ClusterUsageHour::findByTimeStarts(time_start.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}



pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(findById);

    cfg.service(findbyStartTime);

}