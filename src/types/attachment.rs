use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use crate::types::Snowflake;

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Attachment {
    pub id: Snowflake,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: usize,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<usize>,
    pub width: Option<usize>,
    pub ephemeral: Option<bool>,
    pub duration_secs: Option<f64>,
    pub waveform: Option<String>,
    pub flags: Option<i32>,
}

impl PartialEq for Attachment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}