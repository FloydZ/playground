#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::cluster_suspend::{ClusterSuspend};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/cluster_suspends")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = ClusterSuspend::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_suspends/time_start/{time_start}")]
async fn findByTimeStart(time_start: web::Path<u64>) -> Result<HttpResponse, ApiError> {
    let a = ClusterSuspend::findByTimeStart(time_start.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_suspends/node_name/{node_name}")]
async fn findById(job_db_inx: web::Path<u64>) -> Result<HttpResponse, ApiError> {
    let a = ClusterSuspend::findById(job_db_inx.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}



pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(findByTimeStart);
    cfg.service(findById);
}