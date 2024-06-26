pub mod user_routes;

use actix_web::web::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/v1")
            .configure(user_routes::config)
            // .configure(other_routes::config) // Adicione outras configurações de rotas conforme necessário
    );
}
