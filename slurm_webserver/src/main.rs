#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

use actix_web::web;
use actix_web::{App, HttpServer};
use handlebars::Handlebars;
use listenfd::ListenFd;

use std::env;
use dotenv::dotenv;

// Normal Include
mod api_error;
mod db;
mod schema;
mod user;

// table includes
mod acct_coord;
mod acct;

mod cluster;
mod cluster_assoc;
mod cluster_assoc_usage_day;
mod cluster_assoc_usage_hour;
mod cluster_assoc_usage_month;
mod cluster_event;
mod cluster_job;
mod cluster_last_ran;
mod cluster_resv;
mod cluster_step;
mod cluster_suspend;
mod cluster_usage_day;
mod cluster_usage_hour;
mod cluster_usage_month;
mod cluster_wckey;
mod cluster_wckey_usage_day;
mod cluster_wckey_usage_hour;
mod cluster_wckey_usage_month;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // read the .env file
    dotenv().ok();

    // start the logging
    env_logger::init();

    // start the db
    db::init();

    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    // automatic recompile
    // run with `systemfd --no-pid -s http::5000 -- cargo watch -x run`
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move ||
        App::new()
            .app_data(handlebars_ref.clone())
            .configure(user::init_routes)
            .configure(acct_coord::init_routes)
            .configure(acct::init_routes)
            .configure(cluster_assoc::init_routes)
            .configure(cluster_assoc_usage_day::init_routes)
            .configure(cluster_assoc_usage_hour::init_routes)
            .configure(cluster_assoc_usage_month::init_routes)
            .configure(cluster_event::init_routes)
            .configure(cluster_job::init_routes)
            .configure(cluster_last_ran::init_routes)
            .configure(cluster::init_routes)
            .configure(cluster_resv::init_routes)
            .configure(cluster_step::init_routes)
            .configure(cluster_suspend::init_routes)
            .configure(cluster_usage_day::init_routes)
            .configure(cluster_usage_hour::init_routes)
            .configure(cluster_usage_month::init_routes)
            .configure(cluster_wckey::init_routes)
            .configure(cluster_wckey_usage_day::init_routes)
            .configure(cluster_wckey_usage_hour::init_routes)
            .configure(cluster_wckey_usage_month::init_routes)

    );

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };


    info!("Starting server");
    server.run().await
}
