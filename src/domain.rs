use crate::routes::url::UrlRequest;

pub struct Url {
    id: String,
    url: String,
    created_at: String,
    last_modified: String,
    request_count: usize,
}

impl From<UrlRequest> for Url {
    fn from(value: UrlRequest) -> Self {
        let now = "NOW".to_string();

        Self {
            id: "some".to_string(),
            url: value.url,
            created_at: now.clone(),
            last_modified: now,
            request_count: 0,
        }
    }
}
