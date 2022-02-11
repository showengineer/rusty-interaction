use serde::{Serialize, Deserialize}

use serde_with::*;
use super::Components;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Modal{
    custom_id: String,
    title: String,
    components: Vec<MessageComponent>,
}
