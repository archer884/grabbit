pub mod iter;

use hashbrown::HashMap;
use serde::Deserialize;
use std::borrow::Cow;

type Items<'a> = smallvec::SmallVec<[Cow<'a, str>; 4]>;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostsResponse {
    token: Option<String>,
    post_ids: Vec<String>,
    pub posts: HashMap<String, Post>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    id: String,
    title: String,
    author: String,
    media: Option<Media>,
    source: Option<Source>,
    permalink: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    display_text: String,
    url: String,
}

// This type is NOT actually untagged, but I'm not interested
// in implementing a half dozen different types that are all
// identical with the exception of the gallery type.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Media {
    Standard(MediaSource),
    Gallery(GallerySource),

    // Hack to get around the fact that obfuscated reddit videos
    // contain no useful goddamn information.
    Obfuscated(ObfuscatedSource),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSource {
    #[serde(rename = "type")]
    pub media_type: MediaType,
    content: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GallerySource {
    gallery: Gallery,
    #[serde(rename = "mediaMetadata")]
    metadata: HashMap<String, MediaMetadata>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObfuscatedSource {
    obfuscated: String,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaType {
    Embed,
    Gallery,
    #[serde(rename = "gifvideo")]
    GifVideo,
    Image,
    /// Rich Text JSON ??!!1? wtfbbq
    #[serde(rename = "rtjson")]
    RtJson,
    Video,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Gallery {
    items: Vec<Item>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Item {
    #[serde(rename = "mediaId")]
    id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MediaMetadata {
    #[serde(rename = "m")]
    format: String,
}

impl PostsResponse {
    pub fn last_id(&self) -> Option<&String> {
        self.post_ids.last()
    }

    pub fn media(&self) -> impl Iterator<Item = Cow<str>> {
        self.posts
            .iter()
            .filter_map(|(_, post)| post.media())
            .flatten()
    }
}

impl Post {
    pub fn media_type(&self) -> Option<MediaType> {
        match self.media.as_ref()? {
            Media::Standard(m) => Some(m.media_type),
            Media::Gallery(_) => Some(MediaType::Gallery),
            Media::Obfuscated(_) => None
        }
    }

    // pub fn content(&self) -> Option<&str> {
    //     Some(self.media.as_ref()?.content.as_ref()?.as_ref())
    // }

    pub fn source(&self) -> Option<&str> {
        Some(self.source.as_ref()?.url.as_ref())
    }

    pub fn media(&self) -> Option<Items> {
        match self.media.as_ref()? {
            Media::Standard(media) => {
                let mut items = Items::new();
                items.push(Cow::from(&media.content));
                Some(items)
            }
            Media::Gallery(media) => gallery(media),
            Media::Obfuscated(_) => {
                let mut items = Items::new();
                items.push(Cow::from(&self.permalink));
                Some(items)
            }
        }
    }
}

fn gallery(media: &GallerySource) -> Option<Items> {
    let items = &media.gallery.items;
    let types = &media.metadata;

    if items.is_empty() {
        return None;
    }

    Some(
        items
            .into_iter()
            .filter_map(move |item| {
                types.get(&item.id).map(|x| match x.format.as_ref() {
                    "image/jpg" => format!("https://i.redd.it/{}.jpg", item.id),
                    x => panic!("Unknown media format: {}", x),
                })
            })
            .map(Cow::from)
            .collect(),
    )
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
