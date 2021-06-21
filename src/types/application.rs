use serde::{Deserialize, Serialize};

use serde_with::*;

use super::components::ComponentType;
use super::Snowflake;
use serde_repr::*;

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// AKA a 'slash command'.
pub struct ApplicationCommand {
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// ID of command
    pub id: Option<Snowflake>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    application_id: Option<Snowflake>,
    pub name: String,
    description: String,
    options: Option<Vec<ApplicationCommandOption>>,
}

impl Default for ApplicationCommand {
    fn default() -> Self {
        Self {
            id: None,
            application_id: None,
            name: String::new(),
            description: String::new(),
            options: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Command option
pub struct ApplicationCommandOption {
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Command option choice
pub struct ApplicationCommandOptionChoice {
    name: String,
    // This can be int
    value: String,
}

#[serde_as]
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Representing a bunch of options for slash commands
pub struct ApplicationCommandInteractionDataOption {
    /// Name of the option
    pub name: String,
    /// Value of the option
    pub value: String,
    /// More options
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
}

#[derive(Clone, Debug)]
#[cfg(feature = "extended-handler")]
#[cfg_attr(docsrs, doc(cfg(feature = "extended-handler")))]
/// Simple builder for defining SlashCommands
pub struct SlashCommandDefinitionBuilder {
    obj: ApplicationCommand,
}

#[cfg(feature = "extended-handler")]
impl Default for SlashCommandDefinitionBuilder {
    fn default() -> Self {
        Self {
            obj: ApplicationCommand::default(),
        }
    }
}

#[cfg(feature = "extended-handler")]
impl SlashCommandDefinitionBuilder {
    /// Name of slash command
    pub fn name(mut self, name: impl ToString) -> Self {
        let n = name.to_string();

        self.obj.name = n;
        self
    }
    /// Command description
    pub fn description(mut self, desc: impl ToString) -> Self {
        let d = desc.to_string();

        self.obj.description = d;
        self
    }

    /// Adds an option ([`ApplicationCommandOption`]) to the slash command definition
    pub fn add_option(mut self, opt: ApplicationCommandOption) -> Self{
        match self.obj.options.as_mut(){
            None => {
                self.obj.options = Some(vec![opt]);
            }
            Some(o) => {
                o.push(opt);
            }
        }
        self
    }

    /// Finish building slash command
    pub fn finish(self) -> ApplicationCommand {
        self.obj
    }
}
