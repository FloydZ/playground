#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::cluster_last_ran::{ClusterLastRun};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

use handlebars::Handlebars;


#[get("/cluster_last_runs")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let users = ClusterLastRun::find_all()?;
    Ok(HttpResponse::Ok().json(users))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
}
