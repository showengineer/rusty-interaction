use serde::{Deserialize, Serialize};

// async_trait::async_trait;

/// Module containing the embed structures
pub mod embed;

/// Module containing all structs for defining application commands
pub mod application;
/// Module containing structures for interactions
pub mod interaction;

/// Module containing structures for members/users.
pub mod user;

/// Module containing structures for using [Message Components](https://discord.com/developers/docs/interactions/message-components#what-are-components)
pub mod components;

/// Module containing structures for guilds
pub mod guild;
//use interaction::{InteractionResponse, Interaction};

/// Discord's 'snowflake'. It's a 64bit unsigned integer that is mainly used for identifying anything Discord.  
pub type Snowflake = u64;


#[doc(hidden)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HttpError {
    pub code: u16,
    pub message: String,
}
#[doc(hidden)]
impl HttpError {
    pub fn new(code: u16, message: String) -> HttpError {
        HttpError { code, message }
    }
}
#[doc(hidden)]
#[derive(Clone, Serialize, Deserialize)]
pub struct MessageError {
    pub message: String,
}
#[doc(hidden)]
impl MessageError {
    pub fn new(message: String) -> MessageError {
        MessageError { message }
    }
}
#[doc(hidden)]
impl From<HttpError> for MessageError {
    fn from(HttpError { message, .. }: HttpError) -> MessageError {
        MessageError { message }
    }
}
