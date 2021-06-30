 use anyhow::Result;
use mongodb::{
    bson::Document,
    options::{
        ClientOptions, FindOneAndDeleteOptions, FindOneAndUpdateOptions, FindOneOptions,
        FindOptions, InsertOneOptions, UpdateModifications,
    },
    results::InsertOneResult,
    Client, Cursor, Database,
};

    pub async fn connect_to_db() -> Result<Database> {
        let client_options = ClientOptions::parse(&std::env::var("MONGODB_URL")?).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(&std::env::var("MONGODB_DB_NAME")?);
        Ok(db)
    }
