use serde::{Deserialize, Serialize};

use serde_with::*;

#[cfg(feature = "builder")]
use crate::Builder;

use super::components::ComponentType;
use super::user::*;
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

    /// the type of command, defaults `1` if not set
    pub r#type: Option<ApplicationCommandType>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    application_id: Option<Snowflake>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// guild id of the command, if not global
    pub guild_id: Option<Snowflake>,

    /// Command name
    pub name: String,
    /// Command description
    description: String,
    /// Command options
    options: Option<Vec<ApplicationCommandOption>>,

    /// Whether the command is enabled by default when the app is added to a guild
    default_permission: Option<bool>,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
/// Type of `ApplicationCommand`
pub enum ApplicationCommandType {
    /// Slash commands; a text-based command that shows up when a user types `/`
    ChatInput = 1,
    /// A UI-based command that shows up when you right click or tap on a user
    User = 2,
    /// A UI-based command that shows up when you right click or tap on a messages
    Message = 3,
}

impl Default for ApplicationCommand {
    fn default() -> Self {
        Self {
            id: None,
            r#type: Some(ApplicationCommandType::ChatInput),
            application_id: None,
            guild_id: None,
            name: String::new(),
            description: String::new(),
            options: None,
            default_permission: Some(true),
        }
    }
}

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
/// Type of permission override
pub enum ApplicationCommandPermissionType {
    /// A guild role
    Role = 1,
    /// A user
    User = 2,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Used for specifying a batch of [`ApplicationCommandPermission`]s
pub struct ApplicationCommandPermissionBatch {
    /// ID of the command
    pub id: Snowflake,
    /// Permissions (see [`ApplicationCommandPermission`])
    pub permissions: Vec<ApplicationCommandPermission>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// A permission override for a [`ApplicationCommand`]
pub struct ApplicationCommandPermission {
    /// Role or user ID
    pub id: Snowflake,
    /// Type of override. See [`ApplicationCommandPermissionType`]
    pub r#type: ApplicationCommandPermissionType,

    /// Allow or disallow for this override
    pub permission: bool,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Command option
pub struct ApplicationCommandOption {
    r#type: ApplicationCommandOptionType,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    name: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    description: Option<String>,
    #[serde_as(as = "Option<_>")]
    #[serde(default)]
    required: Option<bool>,
    #[serde_as(as = "Option<Vec<_>>")]
    #[serde(default)]
    choices: Option<Vec<ApplicationCommandOptionChoice>>,

    #[serde_as(as = "Option<Vec<_>>")]
    #[serde(default)]
    options: Option<Vec<ApplicationCommandOption>>,
}

impl Default for ApplicationCommandOption {
    fn default() -> Self {
        Self {
            r#type: ApplicationCommandOptionType::String,
            name: None,
            description: None,
            required: None,
            choices: None,
            options: None,
        }
    }
}

impl ApplicationCommandOption {
    /// Set the type
    pub fn option_type(mut self, ty: &ApplicationCommandOptionType) -> Self {
        self.r#type = ty.clone();
        self
    }
    /// Set the option name
    ///
    /// This can only be lower case and may not contain spaces and special characters
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the option description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Sets whether this option is required to be filled in
    pub fn required(mut self, req: &bool) -> Self {
        self.required = Some(*req);
        self
    }

    /// Add a choice
    pub fn add_choice(mut self, choice: &ApplicationCommandOptionChoice) -> Self {
        match self.choices.as_mut() {
            None => {
                self.choices = Some(vec![choice.clone()]);
            }
            Some(o) => {
                o.push(choice.clone());
            }
        }
        self
    }

    /// Add another option
    ///
    /// Can only be used with the `SubCommand` and `SubCommandGroup` types.
    pub fn add_option(mut self, opt: &ApplicationCommandOption) -> Self {
        match self.options.as_mut() {
            None => {
                self.options = Some(vec![opt.clone()]);
            }
            Some(o) => {
                o.push(opt.clone());
            }
        }
        self
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
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
    /// Name
    pub name: String,
    // This can be int
    /// Value
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Stripped down version of ResolvedData
pub struct ResolvedData {
    /// User map
    pub users: Option<User>,
    /// Member map
    pub members: Option<Member>,
}

#[serde_as]
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Representing a slash command
pub struct ApplicationCommandInteractionData {
    /// The unique id of the command
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub id: Option<Snowflake>,
    /// The name of the command
    pub name: Option<String>,

    /// The type of the invoked command
    pub r#type: Option<ApplicationCommandType>,

    /// An array of [`ApplicationCommandInteractionDataOption`]
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,

    /// converted users + roles + channels
    // Not including this yet
    //pub resolved: Option<ResolvedData>,

    /// For components, the component type
    pub component_type: Option<ComponentType>,

    /// For components, the custom identifier for the developer
    pub custom_id: Option<String>,

    /// For Select Menus, the selected values
    pub values: Option<Vec<ApplicationCommandOption>>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// For User- and Message Commands, the id of the user or message targeted.
    pub target_id: Option<Snowflake>,
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
#[cfg(feature = "builder")]
#[cfg_attr(docsrs, doc(cfg(feature = "builder")))]
/// Simple builder for defining SlashCommands
pub struct SlashCommandDefinitionBuilder {
    obj: ApplicationCommand,
}

#[cfg(feature = "builder")]
impl Default for SlashCommandDefinitionBuilder {
    fn default() -> Self {
        Self {
            obj: ApplicationCommand::default(),
        }
    }
}

#[cfg(feature = "builder")]
impl SlashCommandDefinitionBuilder {
    /// Name of the application command
    pub fn name(mut self, name: impl ToString) -> Self {
        let n = name.to_string();

        self.obj.name = n;
        self
    }

    /// Sets the type of command you're defining. See [`ApplicationCommandType`]
    pub fn command_type(mut self, c_type: ApplicationCommandType) -> Self {
        self.obj.r#type = Some(c_type);
        self
    }

    /// Command description
    pub fn description(mut self, desc: impl ToString) -> Self {
        let d = desc.to_string();

        self.obj.description = d;
        self
    }

    /// Adds an option ([`ApplicationCommandOption`]) to the slash command definition
    pub fn add_option(mut self, opt: ApplicationCommandOption) -> Self {
        match self.obj.options.as_mut() {
            None => {
                self.obj.options = Some(vec![opt]);
            }
            Some(o) => {
                o.push(opt);
            }
        }
        self
    }

    /// Sets the default permission. If false, this command can't be used unless some
    /// permission override is set.
    pub fn default_permission(mut self, permission: bool) -> Self {
        self.obj.default_permission = Some(permission);
        self
    }

    #[deprecated(since = "0.1.9", note = "Use the `build()` function instead")]
    /// Finish building slash command
    pub fn finish(self) -> ApplicationCommand {
        self.obj
    }
}

#[cfg(feature = "builder")]
impl Builder<ApplicationCommand> for SlashCommandDefinitionBuilder {
    type Error = std::convert::Infallible;

    fn build(self) -> Result<ApplicationCommand, Self::Error> {
        Ok(self.obj)
    }
}
