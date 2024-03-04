use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::routes::url::UrlRequest;

const SHORT_ID_LEN: usize = 8;

pub enum ValidationError {
    InvalidShortId,
}

#[derive(Clone)]
pub struct AppState {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
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
    pub fn parse(short_id: String) -> Result<Self, ValidationError> {
        if !short_id.len() == SHORT_ID_LEN
            && short_id.chars().find(|c| !c.is_alphanumeric()).is_some()
        {
            return Err(ValidationError::InvalidShortId);
        }

        Ok(Self(short_id))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Url {
    pub short_id: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub request_count: usize,
}

impl From<UrlRequest> for Url {
    fn from(value: UrlRequest) -> Self {
        let now = chrono::Utc::now();

        Self {
            short_id: nanoid!(SHORT_ID_LEN, &nanoid::alphabet::SAFE),
            url: value.url,
            created_at: now,
            last_accessed: None,
            request_count: 0,
        }
    }
}
