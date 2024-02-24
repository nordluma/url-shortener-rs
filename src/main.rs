use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};

use url_shortener::routes::pages::get_home;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1";
    let port = 8080;

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(serve_home))
            .route("/healthcheck", web::get().to(|| async { HttpResponse::Ok() }))
            .route("/url", web::get().to(get_url))
            .route("/api/url", web::post().to(create_url))
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

// TODO: insert url into db and return shortened url
async fn create_url() -> impl Responder {
    HttpResponse::Ok().body("Hello, world")
}

async fn get_url() -> impl Responder {
    // TODO: get url from db
    let url = "https://youtube.com";
    HttpResponse::SeeOther()
}
