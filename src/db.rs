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
use serde::{de::DeserializeOwned, ser::Serialize};
pub struct Db;
impl Db {
    async fn connect() -> Result<Database> {
        let mut client_options = ClientOptions::parse(&std::env::var("MONGODB_URL")?).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(&std::env::var("MONGODB_DB_NAME")?);
        Ok(db)
    }
    pub async fn insert_one(
        collection: &str,
        doc: Document,
        options: impl Into<Option<InsertOneOptions>>,
    ) -> Result<InsertOneResult> {
        let db = Self::connect().await?;
        let coll = db.collection(collection);
        println!("{:#?}", coll);
        let res = coll.insert_one(doc, options).await?;
        Ok(res)
    }
    pub async fn delete_one(
        collection: &str,
        filter: Document,
        options: impl Into<Option<FindOneAndDeleteOptions>>,
    ) -> Result<Option<Document>> {
        let db = Self::connect().await?;
        let coll = db.collection(collection);
        Ok(coll.find_one_and_delete(filter, options).await?)
    }
    pub async fn find_one(
        collection: String,
        filter: Document,
        options: impl Into<Option<FindOneOptions>>,
    ) -> Result<Option<Document>> {
        let db = Self::connect().await?;
        let coll = db.collection(&collection);
        Ok(coll.find_one(filter, options).await?)
    }
    pub async fn update(
        collection: &str,
        filter: Document,
        update: impl Into<UpdateModifications>,
        options: impl Into<Option<FindOneAndUpdateOptions>>,
    ) -> Result<Option<Document>> {
        let db = Self::connect().await?;
        let coll = db.collection(collection);
        Ok(coll.find_one_and_update(filter, update, options).await?)
    }
    pub async fn find(
        collection: &str,
        filter: Document,
        options: impl Into<Option<FindOptions>>,
    ) -> Result<Cursor> {
        let db = Self::connect().await?;
        let coll = db.collection(collection);
        Ok(coll.find(filter, options).await?)
    }
}
