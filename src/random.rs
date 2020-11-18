use crate::Post;
use teloxide::prelude::*;
pub async fn random(cx: UpdateWithCx<Message>) {
    Post::getRandomPost();
}
