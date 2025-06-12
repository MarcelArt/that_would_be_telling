
use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web::{self, Data}, App, HttpServer};
use db::surreal::{self};
use dotenv::dotenv;
use routes::{permission, user, role};
use env_logger::Env;

use crate::{routes::{environment, project, variable}, utils::{args, env::get_env}};

mod handlers;
mod models;
mod routes;
mod db;
mod error;
mod repos;
mod utils;
mod middlewares;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_env = args::get_var(1).unwrap_or_default();
    if server_env != "prod" {
        println!("Running in development mode");
        println!("Reading environment variables from .env file");
        dotenv().ok();
    }
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    let port = get_env("PORT").unwrap_or_default();
    let port = port.parse::<u16>().expect("PORT must be a number");

    let db = surreal::connect().await?;
    let repos = repos::Combined::new(db);

    println!("Starting server at http://localhost:{}", port);
    HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);


        App::new()
            .app_data(Data::new(repos.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .service(
                web::scope("/api")
                    .service(permission::setup_routes())
                    .service(user::setup_routes())
                    .service(role::setup_routes())
                    .service(project::setup_routes())
                    .service(environment::setup_routes())
                    .service(variable::setup_routes())
            )
            
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
