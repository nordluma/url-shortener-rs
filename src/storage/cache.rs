use moka::future::Cache;

pub struct CacheStorage {
    cache: Cache<String, String>,
}

impl CacheStorage {
    pub fn build(max_capacity: u64) -> Self {
        Self {
            cache: Cache::new(max_capacity),
        }
    }
}
