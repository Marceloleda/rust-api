use crate::{schema::AppState, services::user_service};
use actix_web::{web, Responder};

pub async fn index(data: web::Path<AppState>) -> impl Responder {
    user_service::index(data.into_inner()).await
}

pub async fn hello(user_id: web::Path<String>) -> impl Responder {
    user_service::hello(user_id.into_inner()).await
}

pub async fn agreet() -> impl Responder {
    user_service::agreet().await
}
