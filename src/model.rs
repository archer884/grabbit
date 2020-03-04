pub mod iter;

use hashbrown::HashMap;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostsResponse {
    token: String,
    post_ids: Vec<String>,
    posts: HashMap<String, Post>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    id: String,
    title: String,
    author: String,
    media: Option<Media>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    content: String,
    #[serde(rename = "type")]
    media_type: MediaType,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaType {
    Image,
}

impl PostsResponse {
    pub fn posts(self) -> impl Iterator<Item = Post> {
        self.into_iter()
    }
}

impl Post {
    pub fn url(&self) -> Option<&str> {
        self.media.as_ref().map(|media| media.content.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::PostsResponse;
    use serde_json;

    #[test]
    fn can_deserialize() -> serde_json::Result<()> {
        let content = include_str!("../resource/posts_response.json");
        let _: PostsResponse = serde_json::from_str(content)?;

        Ok(())
    }
}
