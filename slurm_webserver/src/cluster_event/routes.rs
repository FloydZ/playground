#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::cluster_event::{ClusterEvent};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/cluster_events")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = ClusterEvent::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_events/time_start/{time_start}")]
async fn findByTimeStart(time_start: web::Path<i64>) -> Result<HttpResponse, ApiError> {
    let a = ClusterEvent::findByTimeStart(time_start.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/cluster_eventsn/node_name/{node_name}")]
async fn findByNodeName(node_name: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let a = ClusterEvent::findByNodeName(node_name.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}



pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(findByTimeStart);
    cfg.service(findByNodeName);
}