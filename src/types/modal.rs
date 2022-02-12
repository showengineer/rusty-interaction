use serde::{Serialize, Deserialize};

use serde_with::*;
use super::components::MessageComponent;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Modal{
    custom_id: String,
    title: String,
    components: Vec<MessageComponent>,
}
