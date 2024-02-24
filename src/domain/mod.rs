pub mod get;
pub mod post;

pub use get::*;
pub use post::*;

pub struct Url {
    url: String,
    created_at: String,
    last_modified: String,
    request_count: usize,
}

impl From<NewUrl> for Url {
    fn from(value: NewUrl) -> Self {
        let now = "NOW".to_string();

        Self {
            url: value.url,
            created_at: now.clone(),
            last_modified: now,
            request_count: 0,
        }
    }
}
