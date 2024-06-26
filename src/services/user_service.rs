use crate::{repositories::user_repository, schema::AppState};
use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn index(data: AppState) -> String {
    format!("Hello {}!", data.name)
}

pub async fn hello(user_id: String) -> impl Responder {
    // Chame o repositório para obter dados
    let user_data = user_repository::get_user_data(&user_id).await;
    println!("teste");
    HttpResponse::Ok().json(json!({
        "message": user_data,
        "ponteiro": &user_id,
        "sem comercial": user_id
    }))
}

pub async fn agreet() -> impl Responder {
    format!("Oi mundo com API em Rust! agreet")
}
