use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::routes::url::UrlRequest;

#[derive(Debug, Deserialize, Serialize)]
pub struct Url {
    id: String,
    url: String,
    created_at: DateTime<Utc>,
    last_accessed: Option<DateTime<Utc>>,
    request_count: usize,
}

impl From<UrlRequest> for Url {
    fn from(value: UrlRequest) -> Self {
        let now = chrono::Utc::now();

        Self {
            id: nanoid!(8),
            url: value.url,
            created_at: now,
            last_accessed: None,
            request_count: 0,
        }
    }
}
