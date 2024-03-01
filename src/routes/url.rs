use actix_web::{
    http::header::LOCATION,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::{domain::Url, storage::database::Database};

#[derive(Debug, Deserialize)]
pub struct UrlRequest {
    pub url: String,
}

// TODO: insert url into db and return shortened url
pub async fn create_url(
    url: web::Form<UrlRequest>,
    conn: Data<Database>,
) -> actix_web::Result<HttpResponse> {
    let url: Url = url.into_inner().into();
    println!("{:#?}", url);

    if let Err(err) = conn.insert_url(url).await {
        eprintln!("->> {}", err);
        return Ok(HttpResponse::InternalServerError().finish());
    }

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish())
}

pub async fn get_urls(conn: Data<Database>) -> actix_web::Result<HttpResponse> {
    let Ok(urls) = conn.get_urls().await else {
        return Ok(HttpResponse::InternalServerError().finish());
    };

    for url in urls {
        println!("{:#?}", url);
    }

    Ok(HttpResponse::Ok().finish())
}

pub async fn get_url() -> impl Responder {
    // TODO: get url from db
    let _url = "https://youtube.com";
    HttpResponse::SeeOther()
}
