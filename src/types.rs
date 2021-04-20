use serde::{Deserialize, Serialize};
#[macro_use]
use serde_with::*;

use serde_repr::*;


mod embed;
use embed::*;

mod user;
use user::*;

/// Discord's 'snowflake'. It's a 64bit integer that is mainly used for identifying anything Discord.  
type Snowflake = i64;

/// HTTP Error return struct
#[derive(Clone, Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
}


impl Error {
    pub fn new(code: i32, msg: String) -> Error {
        Error {
            code: code,
            message: msg,
        }
    }
}
/// Lame Message Error structure
#[derive(Clone, Serialize, Deserialize)]
pub struct MessageError{
    pub message: String,
}

impl MessageError{
    pub fn new(msg: String) -> MessageError{
        MessageError{
            message: msg,
        }
    }
}

impl From<Error> for MessageError{
    fn from(e: Error) -> MessageError{
        MessageError{
            message: e.message,
        }
    }
}


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
#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Interaction {

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub application_id: Option<Snowflake>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub id: Option<Snowflake>,
    pub r#type: InteractionType,
    pub data: Option<ApplicationCommandInteractionData>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub channel_id: Option<Snowflake>,
    pub member: Option<Member>,
    pub user: Option<User>,
    pub token: Option<String>,
    pub version: Option<i8>,
}


impl Interaction{
    pub fn response(&self, rtype: InteractionResponseType) -> InteractionResponse{
        InteractionResponse{
            r#type: rtype,
            data: None
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum InteractionType {
    PING = 1,
    APPLICATION_COMMAND = 2,
}
#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApplicationCommandInteractionData {
    #[serde_as(as = "DisplayFromStr")]
    pub id: Snowflake,
    pub name: String,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApplicationCommandInteractionDataOption {
    pub name: String,
    pub value: String,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
}

// InteractonResponse

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InteractionResponse {
    pub r#type: InteractionResponseType,
    pub data: Option<InteractionApplicationCommandCallbackData>,
}

impl InteractionResponse{
    pub fn new(rtype: InteractionResponseType, data: Option<InteractionApplicationCommandCallbackData>) -> InteractionResponse{
        InteractionResponse{
            r#type: rtype,
            data: data,
        }
    }

    /// Fills the `InteractionResponse` with some `InteractionApplicationCommandCallbackData`
    pub fn data(&mut self, d: InteractionApplicationCommandCallbackData) -> &Self{
        self.data = Some(d);
        self
    }

    /// Sets the Text-To-Speech value of this `InteractionResponse` to `true`
    pub fn tts(&mut self) -> &Self{
        // Does data exist?
        if self.data.is_none(){
            let mut d = InteractionApplicationCommandCallbackData::new();
            d.tts = Some(true);
            self.data = Some(d);
        }
        else{
            self.data.as_mut().unwrap().tts = Some(true);
        }
        self
    }
    /// This sets the `content` for an `InteractionResponse`
    pub fn content(&mut self, c: &String) -> &Self{
        match self.data.as_mut(){
            None => {
                let mut d = InteractionApplicationCommandCallbackData::new();
                d.content = Some(c.to_string());
                self.data = Some(d);
            },
            Some(mut d) => {
                d.content = Some(c.to_string());
            },
        }
        self
    }

    /// Sets the `content` for an `InteractionResponse`. Alias for `content()`
    pub fn message(&mut self, c: &String) -> &Self{
        self.content(c);
        self
    }

    pub fn add_embed(&mut self, e: Embed) -> &Self{
        match self.data.as_mut(){
            None => {
                let mut d = InteractionApplicationCommandCallbackData::new();
                d.embeds = Some(vec![e]);
                self.data = Some(d);
            },
            Some(mut d) => {
                if d.embeds.is_none(){
                    d.embeds = Some(vec![e]);
                }
                else{
                    let v = d.embeds.as_mut().unwrap();
                    v.push(e);
                }
            },
        }
        self
    }

    // Returns a copy of the current `InteractionResponse`, consuming itself.
    pub fn finish(&self) -> Self{
        self.clone()
    }

}

#[derive(Clone, Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum InteractionResponseType {
    PONG = 1,
    ACK = 2,
    CHANNEL_MESSAGE = 3,
    CHANNEL_MESSAGE_WITH_SOURCE = 4,
    DEFFERED_CHANNEL_MESSAGE_WITH_SOURCE = 5,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InteractionApplicationCommandCallbackData {
    tts: Option<bool>,
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    allowed_mentions: Option<AllowedMentions>,
}

impl InteractionApplicationCommandCallbackData{
    pub fn new() -> Self{
        Self{
            tts: None,
            content: None,
            embeds: None,
            allowed_mentions: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
enum AllowedMentionTypes {
    ROLES,
    USERS,
    EVERYONE,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct AllowedMentions {
    parse: Vec<AllowedMentionTypes>,
    roles: Vec<Snowflake>,
    users: Vec<Snowflake>,
    replied_user: bool,
}
