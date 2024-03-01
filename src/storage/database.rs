use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

use crate::domain::Url;

#[derive(Clone)]
pub struct Database {
    connection: Surreal<Client>,
}

impl Database {
    pub async fn connect() -> surrealdb::Result<Self> {
        let connection = Surreal::new::<Ws>("localhost:8000").await?;
        connection.use_ns("surreal").use_db("short-url-db").await?;

        Ok(Self { connection })
    }

    pub async fn insert_url(&self, new_url: Url) -> surrealdb::Result<Option<Url>> {
        println!("->> DATABASE - insert_url, {:?}", new_url);
        let mut res = self
            .connection
            .query("SELECT * FROM short_url WHERE url = ($url)")
            .bind(("url", &new_url.url))
            .await?;

        let existing_url: Option<Url> = res.take(0)?;
        let url = if let Some(existing_url) = existing_url {
            println!("->> DATABASE - URL already exists in database, returning existing url");

            Some(existing_url)
        } else {
            self.connection
                .create(("short_url", &new_url.short_id))
                .content(new_url)
                .await?
        };

        Ok(url)
    }

    pub async fn get_urls(&self) -> surrealdb::Result<Vec<Url>> {
        println!("->> DATABASE - get_urls");
        self.connection.select("short_url").await
    }
}
