use teloxide::prelude::*;
use crate::Post;
pub async fn random(cx: UpdateWithCx<Message>){
Post::getRandomPost();
}
