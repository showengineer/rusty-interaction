use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use crate::types::Snowflake;

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Attachment {
    id: Snowflake,
    filename: String,
    description: Option<String>,
    content_type: Option<String>,
    size: usize,
    url: String,
    proxy_url: String,
    height: Option<usize>,
    width: Option<usize>,
    ephemeral: Option<bool>,
    duration_secs: Option<f64>,
    waveform: Option<String>,
    flags: Option<i32>,
}

impl PartialEq for Attachment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}