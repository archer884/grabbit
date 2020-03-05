pub mod iter;

use hashbrown::HashMap;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostsResponse {
    token: Option<String>,
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
    source: Option<Source>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    display_text: String,
    url: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    content: Option<String>,
    #[serde(rename = "type")]
    pub media_type: MediaType,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaType {
    Embed,
    #[serde(rename = "gifvideo")]
    GifVideo,
    Image,
    /// Rich Text JSON ??!!1? wtfbbq
    #[serde(rename = "rtjson")]
    RtJson,
    Video,
}

impl PostsResponse {
    pub fn last_id(&self) -> Option<&String> {
        self.post_ids.last()
    }

    pub fn posts(self) -> impl Iterator<Item = Post> {
        self.into_iter()
    }
}

impl Post {
    pub fn media_type(&self) -> Option<MediaType> {
        Some(self.media.as_ref()?.media_type)
    }
    
    pub fn content(&self) -> Option<&str> {
        Some(self.media.as_ref()?.content.as_ref()?.as_ref())
    }

    pub fn source(&self) -> Option<&str> {
        Some(self.source.as_ref()?.url.as_ref())
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
