use actix_web::{
    http::header::LOCATION,
    web::{self, Data},
    HttpResponse,
};
use serde::Deserialize;

use crate::{
    domain::{validate_short_id, AppState, Url},
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
    state: Data<AppState>,
) -> actix_web::Result<HttpResponse> {
    let Ok(()) = validate_short_id(&short_id.short_id) else {
        return Ok(HttpResponse::BadRequest().finish());
    };

    // TODO: query url from db
    println!("{}/{}", state.url, short_id.short_id);

    Ok(HttpResponse::SeeOther().finish())
}
