use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};

use url_shortener::routes::{self, pages::get_home};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1";
    let port = 8080;

    println!("->> Listening on {}", port);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(serve_home))
            .route(
                "/healthcheck",
                web::get().to(|| async { HttpResponse::Ok() }),
            )
            .route("/url", web::get().to(routes::url::get_url))
            .route("/api/url", web::post().to(routes::url::create_url))
    })
    .bind((addr, port))?
    .run()
    .await?;

    Ok(())
}

// TODO: serve homepage
async fn serve_home() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(get_home())
}
