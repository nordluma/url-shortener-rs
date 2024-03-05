use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};

use url_shortener::{
    domain::AppState,
    routes::{self, api_config, pages::get_home},
    storage::{cache::CacheStorage, database::Database},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "localhost";
    let port = 8080;

    let state = web::Data::new(AppState {
        url: addr.to_string(),
    });
    let connection = web::Data::new(Database::connect().await?);
    let cache = web::Data::new(CacheStorage::build(1000));

    println!("->> Listening on {}", port);
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(serve_home))
            .route(
                "/healthcheck",
                web::get().to(|| async { HttpResponse::Ok() }),
            )
            .service(web::scope("/api").configure(api_config))
            .route("/{short_id}", web::get().to(routes::url::get_url))
            .app_data(state.clone())
            .app_data(connection.clone())
            .app_data(cache.clone())
    })
    .bind((addr, port))?
    .run()
    .await?;

    Ok(())
}

async fn serve_home() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(get_home())
}
