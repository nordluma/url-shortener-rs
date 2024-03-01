use actix_web::{http::header::LOCATION, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UrlRequest {
    url: String,
}

// TODO: insert url into db and return shortened url
pub async fn create_url(url: web::Form<UrlRequest>) -> impl Responder {
    println!("{}", url.url);

    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}

pub async fn get_url() -> impl Responder {
    // TODO: get url from db
    let _url = "https://youtube.com";
    HttpResponse::SeeOther()
}
