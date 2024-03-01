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

        /*
        connection
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        */

        connection.use_ns("surreal").use_db("short-url-db").await?;

        Ok(Self { connection })
    }

    pub async fn insert_url(&self, new_url: Url) -> surrealdb::Result<Option<Url>> {
        println!("->> DATABASE - insert_url, {:?}", new_url);
        let created_url = self
            .connection
            .create(("short-url", "test"))
            .content(new_url)
            .await?;

        Ok(created_url)
    }

    pub async fn get_urls(&self) -> surrealdb::Result<Vec<Url>> {
        println!("->> DATABASE - get_urls");
        self.connection.select("short-url").await
    }
}
