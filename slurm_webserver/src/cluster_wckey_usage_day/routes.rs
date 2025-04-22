#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::cluster_wckey_usage_day::{ClusterWckeyUsageDay};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/cluster_usage_days")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = ClusterWckeyUsageDay::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_usage_days/id/{id}")]
async fn findById(id: web::Path<u32>) -> Result<HttpResponse, ApiError> {
    let a = ClusterWckeyUsageDay::findById(id.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_usage_days/id_tres/{id}")]
async fn findByIdTres(id_tres: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let a = ClusterWckeyUsageDay::findByIdTres(id_tres.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}



#[get("/cluster_usage_days/time_start/{time_start}")]
async fn findbyStartTime(time_start: web::Path<u64>) -> Result<HttpResponse, ApiError> {
    let a = ClusterWckeyUsageDay::findByTimeStarts(time_start.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}



pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(findById);
    cfg.service(findByIdTres);
    cfg.service(findbyStartTime);

}