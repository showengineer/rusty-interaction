use serde::{Deserialize, Serialize};
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

#[derive(Clone, Serialize, Deserialize, Debug)]
struct User {
    id: Snowflake,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: bool,
    system: bool,
    mfa_enabled: bool,
    locale: String,
    verified: bool,
    email: Option<String>,
    flags: i32,
    premium_type: i8,
    public_flags: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Member {
    user: User,
    nick: Option<String>,
    roles: Vec<Snowflake>,
    joined_at: SystemTime,
    premium_since: Option<SystemTime>,
    deaf: bool,
    mute: bool,
    pending: bool,
    permissions: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct ApplicationCommand {
    id: Snowflake,
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
enum ApplicationCommandOptionType {
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Interaction {
    pub id: Option<Snowflake>,
    pub r#type: InteractionType,
    pub data: Option<ApplicationCommandInteractionData>,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub member: Option<Member>,
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApplicationCommandInteractionData {
    id: Snowflake,
    name: String,
    options: Vec<ApplicationCommandInteractionDataOption>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
struct ApplicationCommandInteractionDataOption {
    name: String,
    value: ApplicationCommandOptionType,
    options: Vec<ApplicationCommandInteractionDataOption>,
}
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
