use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};

use url_shortener::{
    domain::AppState,
    routes::{self, pages::get_home},
    storage::database::Database,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "localhost";
    let port = 8080;

    let state = web::Data::new(AppState {
        url: addr.to_string(),
    });
    let connection = web::Data::new(Database::connect().await?);

    println!("->> Listening on {}", port);
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(serve_home))
            .route(
                "/healthcheck",
                web::get().to(|| async { HttpResponse::Ok() }),
            )
            .route("/{short_id}", web::get().to(routes::url::get_url))
            .service(web::scope("/api").configure(api_config))
            .app_data(state.clone())
            .app_data(connection.clone())
    })
    .bind((addr, port))?
    .run()
    .await?;

    Ok(())
}

fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/url")
            .route(web::get().to(routes::url::get_urls))
            .route(web::post().to(routes::url::create_url)),
    );
}

async fn serve_home() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(get_home())
}
