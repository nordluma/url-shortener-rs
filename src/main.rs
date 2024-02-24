use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1";
    let port = 8080;

    HttpServer::new(|| App::new().route("/", web::get().to(homepage)).route("/api/url", web::post().to(create_url)))
        .bind((addr, port))?
        .run()
        .await?;

    Ok(())
}

// TODO: serve homepage
async fn homepage() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("../static/index.html"))
}

// TODO: insert url into db and return shortened url
async fn create_url() -> impl Responder {
    HttpResponse::Ok().body("Hello, world")
}
