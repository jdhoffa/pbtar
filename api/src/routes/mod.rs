use actix_web::web;

mod items;
mod health;
mod scenarios;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/api")
                .configure(items::config)
                .configure(health::config)
                .configure(scenarios::config)
        );
}