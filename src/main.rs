pub mod controllers;
mod model;
pub mod repositories;
mod routes;
pub mod schema;
pub mod services;

use actix_web::{App, HttpServer};

use dotenv::dotenv;

use sqlx::{
    Postgres,
    Pool,
    postgres::PgPoolOptions
};

pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;
    println!("Starting sever on port {port}!");
    dotenv().ok();
    HttpServer::new(|| App::new().configure(routes::config))
        .bind(("127.0.0.1", port))?
        // .workers(2) para controlar as threads
        .run()
        .await
}
