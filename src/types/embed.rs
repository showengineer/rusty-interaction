use serde::{Deserialize, Serialize};

use serde_with::*;

use std::time::SystemTime;

// ======== Structures =========
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Embed {
    title: String,
    r#type: String,
    description: String,
    url: String,
    timestamp: SystemTime,
    color: i32,
    footer: EmbedFooter,
    image: EmbedImage,
    thumbnail: EmbedThumbnail,
    video: EmbedVideo,
    provider: EmbedProvider,
    author: EmbedAuthor,
    fields: Vec<EmbedField>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EmbedThumbnail {
    url: String,
    proxy_url: String,
    height: i32,
    width: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EmbedVideo {
    url: String,
    proxy_url: String,
    height: i32,
    witdh: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EmbedImage {
    url: String,
    proxy_url: String,
    height: i32,
    width: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EmbedProvider {
    name: String,
    url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EmbedAuthor {
    name: String,
    url: String,
    icon_url: String,
    proxy_icon_url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EmbedFooter {
    text: String,
    icon_url: String,
    proxy_icon_url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EmbedField {
    name: String,
    value: String,
    inline: bool,
}

// ========== IMPLS ===========
