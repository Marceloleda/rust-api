use actix_web::{web, web::ServiceConfig};
use crate::controllers::user_controller;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/test/{name}").route(web::get().to(user_controller::index))
    )
    .service(
        web::resource("/hello/{id}").route(web::get().to(user_controller::hello))
    )
    .service(
        web::resource("/hello-2").route(web::get().to(user_controller::agreet))
    );
}
