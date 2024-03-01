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
}
