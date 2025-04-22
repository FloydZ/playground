#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::acct::{Acct};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/accts")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = Acct::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

#[get("/accts")]
async fn find(name: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let a = Acct::find(name.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}



pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
}
