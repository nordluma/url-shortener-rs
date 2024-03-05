use actix_web::web;

use crate::routes;

pub mod pages;
pub mod url;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/url")
            .route(web::get().to(routes::url::get_url))
            .route(web::post().to(routes::url::create_url)),
    );
}
