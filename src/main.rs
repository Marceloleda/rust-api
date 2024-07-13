pub mod controllers;
mod model;
pub mod repositories;
mod routes;
pub mod schema;
pub mod services;

use actix_web::{web, App, HttpServer};

use dotenv::dotenv;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;
    println!("Starting sever on port {port}!");
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL MUST TO BE SET");
    let pool = match PgPoolOptions::new().max_connections(10).connect(&database_url).await {
        Ok(pool) => {
            println!("Connection DB resolved");
            pool
        }
        Err(error) => {
            println!("Failed to connect to the database {:?}", error);
            std::process::exit(1)
        }
    };

    HttpServer::new(move || App::new()
        .app_data(web::Data::new(AppState { db: pool.clone()}))
        .configure(routes::config
        ))  
        .bind(("127.0.0.1", port))?
        // .workers(2) para controlar as threads
        .run()
        .await
}
