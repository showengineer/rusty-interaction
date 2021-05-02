use serde::{Deserialize, Serialize};

use serde_with::*;

use super::Snowflake;
use serde_repr::*;

#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
struct ApplicationCommand {
    #[serde_as(as = "DisplayFromStr")]
    id: Snowflake,
    #[serde_as(as = "DisplayFromStr")]
    application_id: Snowflake,
    name: String,
    description: String,
    options: Vec<ApplicationCommandOption>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct ApplicationCommandOption {
    r#type: i8,
    name: String,
    description: String,
    required: bool,
    choices: Vec<ApplicationCommandOptionChoice>,
    options: Vec<ApplicationCommandOption>,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, Debug)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum ApplicationCommandOptionType {
    SUB_COMMAND = 1,
    SUB_COMMAND_GROUP = 2,
    STRING = 3,
    INTEGER = 4,
    BOOLEAN = 5,
    USER = 6,
    CHANNEL = 7,
    ROLE = 8,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct ApplicationCommandOptionChoice {
    name: String,
    // This can be int
    value: String,
}
