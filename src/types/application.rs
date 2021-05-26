use serde::{Deserialize, Serialize};

use serde_with::*;

use super::components::ComponentType;
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
#[repr(u8)]
/// Representing a type of [`ApplicationCommandOption`]
pub enum ApplicationCommandOptionType {
    /// A subcommand
    SubCommand = 1,
    /// A group of subcommands
    SubCommandGroup = 2,
    /// A string
    String = 3,
    /// An integer
    Integer = 4,
    /// A boolean
    Boolean = 5,
    /// A user
    User = 6,
    /// A channel
    Channel = 7,
    /// A role
    Role = 8,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct ApplicationCommandOptionChoice {
    name: String,
    // This can be int
    value: String,
}

#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
/// Representing a slash command
pub struct ApplicationCommandInteractionData {
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// The unique id of the command
    pub id: Option<Snowflake>,
    /// The name of the command
    pub name: Option<String>,
    /// An array of [`ApplicationCommandInteractionDataOption`]
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
    
    /// For components, the component type
    pub component_type: Option<ComponentType>,

    /// For components, the custom identifier for the developer
    pub custom_id: Option<String>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
/// Representing a bunch of options for slash commands
pub struct ApplicationCommandInteractionDataOption {
    /// Name of the option
    pub name: String,
    /// Value of the option
    pub value: String,
    /// More options
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
}
