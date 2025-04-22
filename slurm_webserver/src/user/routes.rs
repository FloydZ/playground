#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::api_error::ApiError;
use crate::user::{User};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

use handlebars::Handlebars;


#[get("/users")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let users = User::find_all()?;
    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{name}")]
async fn find(name: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let a = User::find(name.into_inner())?;
    Ok(HttpResponse::Ok().json(a))
}


// NORMAL SITES
#[get("/user/{name}")]
async fn user(hb: web::Data<Handlebars>, name: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let user = User::find(name.into_inner())?;
    let body = hb.render("user", &json!(user)).unwrap();
    Ok(HttpResponse::Ok().body(body))
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);

    cfg.service(user);

}
