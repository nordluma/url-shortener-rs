use moka::future::Cache;

use crate::domain::ShortId;

pub struct CacheStorage {
    pub cache: Cache<ShortId, String>,
}

impl CacheStorage {
    pub fn build(max_capacity: u64) -> Self {
        Self {
            cache: Cache::new(max_capacity),
        }
    }
}
