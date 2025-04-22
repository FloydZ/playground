#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::cluster_step::{ClusterStep};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/cluster_steps")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = ClusterStep::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_steps/job_db_inx/{job_db_inx}")]
async fn findByDb(job_db_inx: web::Path<u64>) -> Result<HttpResponse, ApiError> {
    let a = ClusterStep::findIdDB(job_db_inx.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_steps/id_step/{id_step}")]
async fn findByJobId(id_step: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let a = ClusterStep::findIdStep(id_step.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(findByDb);
    cfg.service(findByJobId);
}