use serde::{Deserialize, Serialize};

// async_trait::async_trait;

pub mod embed;

pub mod application;
pub mod interaction;
pub mod user;

//use interaction::{InteractionResponse, Interaction};

/// Discord's 'snowflake'. It's a 64bit unsigned integer that is mainly used for identifying anything Discord.  
type Snowflake = u64;

/// HTTP Error return struct
#[derive(Clone, Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

impl Error {
    pub fn new(code: i32, message: String) -> Error {
        Error { code, message }
    }
}
/// Lame Message Error structure
#[derive(Clone, Serialize, Deserialize)]
pub struct MessageError {
    pub message: String,
}

impl MessageError {
    pub fn new(message: String) -> MessageError {
        MessageError { message }
    }
}

impl From<Error> for MessageError {
    fn from(Error { message, .. }: Error) -> MessageError {
        MessageError { message }
    }
}
