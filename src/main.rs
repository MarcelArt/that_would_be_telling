use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use db::surreal::{self};
use dotenv::dotenv;
use dotenv_codegen::dotenv;
use routes::{permission, user, role};
use env_logger::Env;

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
    dotenv().ok();
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    let port = dotenv!("PORT");
    let port = port.parse::<u16>().expect("PORT must be a number");

    let db = surreal::connect().await?;
    let repos = repos::Combined::new(db);

    println!("Starting server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(repos.clone()))
            .wrap(Logger::default())
            .service(permission::setup_routes())
            .service(user::setup_routes())
            .service(role::setup_routes())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
