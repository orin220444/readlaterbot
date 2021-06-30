


use mongodb::bson::{de::from_document, doc, ser::to_document};
use serde_derive::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Post {
    id: String,
    pub original_url: String,
    pub real_url: String,
    pub read: bool,
    pub created: String,
}

impl Post {
    pub fn id(&self) -> String {
        self.id.clone()
    }
}
#[derive(TypedBuilder, Serialize)]
pub struct PostBuilder {
    pub original_url: String,
    pub real_url: String,
    pub read: bool,
    pub created: String,
}
