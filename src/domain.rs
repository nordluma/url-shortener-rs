use chrono::{DateTime, Utc};
use nanoid::nanoid;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::routes::url::UrlRequest;

const SHORT_ID_LEN: usize = 8;

lazy_static::lazy_static! {
    static ref ACCEPTED_CHARS: Regex = Regex::new(r"[a-zA-Z0-9\-\_]{8}").unwrap();
}

#[derive(Debug)]
pub enum ValidationError {
    InvalidShortId,
}

#[derive(Clone)]
pub struct AppState {
    pub url: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ShortId(String);

impl std::ops::Deref for ShortId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for ShortId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ShortId {
    pub fn new() -> Self {
        Self(nanoid!(SHORT_ID_LEN, &nanoid::alphabet::SAFE))
    }

    pub fn parse(short_id: String) -> Result<Self, ValidationError> {
        println!("->> DOMAIN - parse: {}", short_id);
        if !ACCEPTED_CHARS.is_match(&short_id) {
            return Err(ValidationError::InvalidShortId);
        }

        Ok(Self(short_id))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Url {
    pub short_id: ShortId,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub request_count: usize,
}

impl From<UrlRequest> for Url {
    fn from(value: UrlRequest) -> Self {
        let now = chrono::Utc::now();

        Self {
            short_id: ShortId::new(),
            url: value.url,
            created_at: now,
            last_accessed: None,
            request_count: 0,
        }
    }
}
