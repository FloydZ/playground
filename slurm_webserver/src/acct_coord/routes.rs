#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::acct_coord::{AcctCoord};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/acct_coords")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let a = AcctCoord::find_all()?;
    Ok(HttpResponse::Ok().json(a))
}

// Find by user
#[get("/acct_coords/user/{user}")]
async fn findByUser(user: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let a = AcctCoord::findByUser(user.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}

// Find by acct
#[get("/acct_coords/acct/{acct}")]
async fn findbyAcct(acct: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let a = AcctCoord::findByAcct(acct.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}



pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(findByUser);

    cfg.service(findbyAcct);
}
