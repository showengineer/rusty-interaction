use serde::{Deserialize, Serialize, Deserializer, de};
#[macro_use]
use serde_with::*;
use ::chrono::{DateTime, TimeZone, Utc};
use std::time::SystemTime;
use serde_repr::*;

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

/// Discord's 'snowflake'. It's a 64bit integer that is mainly used for identifying objects.  
type Snowflake = i64;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Embed {
    title: String,
    r#type: String,
    description: String,
    url: String,
    timestamp: SystemTime,
    color: i32,
    footer: EmbedFooter,
    image: EmbedImage,
    thumbnail: EmbedThumbnail,
    video: EmbedVideo,
    provider: EmbedProvider,
    author: EmbedAuthor,
    fields: Vec<EmbedField>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EmbedThumbnail {
    url: String,
    proxy_url: String,
    height: i32,
    width: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EmbedVideo {
    url: String,
    proxy_url: String,
    height: i32,
    witdh: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EmbedImage {
    url: String,
    proxy_url: String,
    height: i32,
    width: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EmbedProvider {
    name: String,
    url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EmbedAuthor {
    name: String,
    url: String,
    icon_url: String,
    proxy_icon_url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EmbedFooter {
    text: String,
    icon_url: String,
    proxy_icon_url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct EmbedField {
    name: String,
    value: String,
    inline: bool,
}
#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    #[serde_as(as = "DisplayFromStr")]
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<i32>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub premium_type: Option<i8>,

    pub public_flags: Option<i32>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Member {
    user: User,
    nick: Option<String>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    roles: Vec<Snowflake>,
    #[serde_as(as = "DisplayFromStr")]
    joined_at: DateTime::<Utc>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    premium_since: Option<DateTime::<Utc>>,
    deaf: bool,
    mute: bool,
    pending: bool,
    permissions: String,
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

    #[serde_as(as = "DisplayFromStr")]
    pub id: Snowflake,
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InteractionApplicationCommandCallbackData {
    tts: bool,
    content: String,
    embeds: Vec<Embed>,
    allowed_mentions: Option<AllowedMentions>,
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
