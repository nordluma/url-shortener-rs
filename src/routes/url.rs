use actix_web::{
    http::header::LOCATION,
    web::{self, Data},
    HttpResponse,
};
use serde::Deserialize;

use crate::{
    domain::{AppState, ShortId, Url},
    storage::database::Database,
};

#[derive(Debug, Deserialize)]
pub struct UrlRequest {
    pub url: String,
}

// TODO: insert url into db and return shortened url
pub async fn create_url(
    url: web::Form<UrlRequest>,
    conn: Data<Database>,
) -> actix_web::Result<HttpResponse> {
    println!("->> HANDLER - create_url: {:?}", url);
    let url: Url = url.into_inner().into();

    if let Err(err) = conn.insert_url(url).await {
        eprintln!("->> DB ERROR: {}", err);
        return Ok(HttpResponse::InternalServerError().finish());
    }

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish())
}

pub async fn get_urls(conn: Data<Database>) -> actix_web::Result<HttpResponse> {
    println!("->> HANDLER - get_urls");
    let Ok(urls) = conn.get_urls().await else {
        return Ok(HttpResponse::InternalServerError().finish());
    };

    Ok(HttpResponse::Ok().json(urls))
}

#[derive(Debug, Deserialize)]
pub struct ShortIdRequest {
    short_id: String,
}

pub async fn get_url(
    short_id: web::Path<ShortIdRequest>,
    conn: Data<Database>,
) -> actix_web::Result<HttpResponse> {
    let Ok(short_id) = ShortId::parse(short_id.short_id.clone()) else {
        return Ok(HttpResponse::BadRequest().finish());
    };

    let Ok(Some(url)) = conn.get_url(short_id).await else {
        return Ok(HttpResponse::NotFound().finish());
    };

    Ok(HttpResponse::Found()
        .insert_header((LOCATION, url.url))
        .finish())
}
