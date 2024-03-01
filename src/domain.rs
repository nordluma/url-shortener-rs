use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::routes::url::UrlRequest;

pub enum ValidationError {
    InvalidShortId,
}

#[derive(Clone)]
pub struct AppState {
    pub url: String,
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
            short_id: nanoid!(8, &nanoid::alphabet::SAFE),
            url: value.url,
            created_at: now,
            last_accessed: None,
            request_count: 0,
        }
    }
}

pub fn validate_short_id(short_id: &str) -> Result<(), ValidationError> {
    if !short_id.len() == 8 && short_id.chars().find(|c| !c.is_alphanumeric()).is_some() {
        return Err(ValidationError::InvalidShortId);
    }

    Ok(())
}
