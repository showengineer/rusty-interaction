use serde::{Deserialize, Serialize};

use super::components::MessageComponent;
use serde_with::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Modal {
    custom_id: String,
    title: String,
    components: Vec<MessageComponent>,
}
