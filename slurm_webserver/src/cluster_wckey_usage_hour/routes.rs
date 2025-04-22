#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::cluster_wckey_usage_hour::{ClusterWckeyUsageHour};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/cluster_wckey_usage_hours")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = ClusterWckeyUsageHour::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_wckey_usage_hours/id/{id}")]
async fn findById(id: web::Path<u32>) -> Result<HttpResponse, ApiError> {
    let a = ClusterWckeyUsageHour::findById(id.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_wckey_usage_hours/id_tres/{id}")]
async fn findByIdTres(id_tres: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let a = ClusterWckeyUsageHour::findByIdTres(id_tres.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}



#[get("/cluster_wckey_usage_hours/time_start/{time_start}")]
async fn findbyStartTime(time_start: web::Path<u64>) -> Result<HttpResponse, ApiError> {
    let a = ClusterWckeyUsageHour::findByTimeStarts(time_start.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}



pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(findById);
    cfg.service(findByIdTres);
    cfg.service(findbyStartTime);

}