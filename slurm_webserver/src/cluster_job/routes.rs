#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::cluster_job::{ClusterJob};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/cluster_jobs")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = ClusterJob::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_jobs/time_start/{time_start}")]
async fn find(job_db_inx: web::Path<u64>) -> Result<HttpResponse, ApiError> {
    let a = ClusterJob::find(job_db_inx.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
}