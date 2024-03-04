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
    state: Data<AppState>,
    conn: Data<Database>,
) -> actix_web::Result<HttpResponse> {
    println!("->> HANDLER - create_url: {:?}", url);
    let url: Url = url.into_inner().into();

    let created_url = match conn.insert_url(url).await {
        Ok(Some(url)) => url.short_id,
        Ok(None) => return Ok(HttpResponse::InternalServerError().finish()),
        Err(err) => {
            eprintln!("->> DB ERROR: {}", err);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    println!(
        "->> HANDLER - create_url: shortened url -> {}/{}",
        state.url, created_url
    );

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
    println!("->> HANDLER - get_url: {:?}", short_id);
    let Ok(short_id) = ShortId::parse(short_id.short_id.clone()) else {
        eprintln!("->> HANDLER - get_url: invalid url");
        return Ok(HttpResponse::BadRequest().finish());
    };

    let Ok(Some(url)) = conn.get_url(short_id).await else {
        return Ok(HttpResponse::NotFound().finish());
    };

    println!(
        "->> HANDLER - get_url: found url, redirecting to -> {}",
        url.url
    );

    Ok(HttpResponse::Found()
        .insert_header((LOCATION, url.url))
        .finish())
}
