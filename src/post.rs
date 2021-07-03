use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Post {
    #[serde(rename(deserialize = "_id"))]
    id: ObjectId,
    pub original_url: String,
    pub real_url: String,
    pub read: bool,
    pub created: String,
}

impl Post {
    pub fn id(&self) -> String {
        self.id.clone().to_string()
    }
}
#[derive(TypedBuilder, Serialize)]
pub struct PostBuilder {
    pub original_url: String,
    pub real_url: String,
    pub read: bool,
    pub created: String,
}
