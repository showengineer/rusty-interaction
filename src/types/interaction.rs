use serde::{Deserialize, Serialize};

use serde_with::*;

use serde_repr::*;

use super::application::*;
use super::embed::*;
use super::HttpError;
use super::user::*;
use super::Snowflake;

// ===== USEFUL MACROS =====
macro_rules! expect_successful_api_response {
    ($response:ident, $succret:expr) => {
        match $response {
            Err(e) => {
                Err(
                    HttpError{
                        code: 0,
                        message: format!("{:#?}", e),
                    }
                )
            }
            Ok(r) => {
                if !r.status().is_success() {
                    Err(
                        HttpError{
                            code: 0,
                            message: format!("{:#?}", r.text().await),
                        }
                    )
                } else {
                    $succret
                }
            }
        }
    };
}

macro_rules! expect_specific_api_response {
    ($response:ident, $expres:expr, $succret:expr) => {
        match $response {
            Err(e) => {
                Err(
                    HttpError{
                        code: 0,
                        message: format!("{:#?}", e),
                    }
                )
            }
            Ok(r) => {
                if r.status() != $expres {
                    Err(
                        HttpError{
                            code: 0,
                            message: format!("{:#?}", r.text().await),
                        }
                    )
                } else {
                    $succret
                }
            }
        }
    };
}





#[cfg(feature = "handler")]
use log::error;
#[cfg(feature = "handler")]
use reqwest::{Client, StatusCode};

#[cfg(feature = "handler")]
#[derive(Clone)]
///
pub struct Context {
    client: Client,

    /// The [`Interaction`] sent by Discord.
    pub interaction: Interaction,
}

#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
/// The base Interaction structure. When Interactions are received, this structure is wrapped inside a [`Context`]
/// and can be used to get information about the Interaction.
pub struct Interaction {
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// The application id of your applicaton
    pub application_id: Option<Snowflake>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// Unique id identifying the interaction
    pub id: Option<Snowflake>,
    /// The type of interaction
    pub r#type: InteractionType,
    /// Interaction data, if applicable
    pub data: Option<ApplicationCommandInteractionData>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// The ID of the guild where the Interaction took place (None if in DM)
    pub guild_id: Option<Snowflake>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// The channel ID where the Interaction took place
    pub channel_id: Option<Snowflake>,
    /// The [`Member`] who invoked the command (None if in DM, use [`User`] instead)
    pub member: Option<Member>,
    /// The [`User`] who invoked the command (None if in guild, use [`Member`] instead)
    pub user: Option<User>,
    /// Unique token used for editing messages and managing follow-up messages
    pub token: Option<String>,
    /// Read-only. Always `1`
    pub version: Option<i8>,
}

#[cfg(feature = "handler")]
impl Context {
    /// Creates a new [`Context`]
    pub fn new(c: Client, i: Interaction) -> Self {
        Self {
            client: c,
            interaction: i,
        }
    }

    /// Respond to an Interaction
    ///
    /// This returns an [`InteractionResponseBuilder`] which you can use to build an [`InteractionResponse`]
    ///
    /// # Example
    /// ```ignore
    /// let response = ctx.respond()
    ///                   .content("Example message")
    ///                   .tts(true)
    ///                   .finish();
    /// ```
    pub fn respond(&self) -> InteractionResponseBuilder {
        InteractionResponseBuilder::default()
    }

    /// Edit the original interaction response
    ///
    /// This takes an [`WebhookMessage`]. You can convert an [`InteractionResponse`] using [`WebhookMessage::from`].
    pub async fn edit_original(&self, new_content: &WebhookMessage) -> Result<(), HttpError>{
        let url = format!(
            "{}/webhooks/{:?}/{}/messages/@original",
            crate::BASE_URL,
            self.interaction.application_id.unwrap(),
            self.interaction.token.as_ref().unwrap().to_string()
        );
        let c = self.client.patch(&url).json(new_content).send().await;

        expect_successful_api_response!(c, Ok(()))
    }

    /// Delete the original interaction response
    pub async fn delete_original(&self) -> Result<(), HttpError>{
        let url = format!(
            "{}/webhooks/{:?}/{}/messages/@original",
            crate::BASE_URL,
            self.interaction.application_id.unwrap(),
            self.interaction.token.as_ref().unwrap().to_string()
        );
        let c = self.client.delete(&url).send().await;

        expect_specific_api_response!(c, StatusCode::NO_CONTENT, Ok(()))
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// Represents the type of interaction that comes in.
pub enum InteractionType {
    /// Discord requested a ping
    Ping = 1,
    /// A slash command
    ApplicationCommand = 2,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Struct repesenting an Interaction response
///
/// This is used to respond to incoming interactions.
pub struct InteractionResponse {
    /// Type of response
    pub r#type: InteractionResponseType,

    /// Optional data field
    pub data: Option<InteractionApplicationCommandCallbackData>,
}

#[cfg(feature = "handler")]
#[derive(Clone, Debug)]
/// Builder for making a [`InteractionResponse`]

pub struct InteractionResponseBuilder {
    #[doc(hidden)]
    pub r#type: InteractionResponseType,
    #[doc(hidden)]
    pub data: Option<InteractionApplicationCommandCallbackData>,
}

impl InteractionResponse {
    /// Creates a new InteractionResponse
    pub fn new(
        rtype: InteractionResponseType,
        data: Option<InteractionApplicationCommandCallbackData>,
    ) -> InteractionResponse {
        InteractionResponse {
            r#type: rtype,
            data,
        }
    }
}

#[cfg(feature = "handler")]
impl Default for InteractionResponseBuilder {
    /// This will default to responding with the `InteractionResponseType::CHANNEL_MESSAGE_WITH_SOURCE` response type and no data.
    /// Adding data yourself is expected.
    fn default() -> Self {
        Self {
            r#type: InteractionResponseType::ChannelMessageWithSource,
            data: None,
        }
    }
}

#[cfg(feature = "handler")]
impl InteractionResponseBuilder {
    fn ret(self) -> InteractionResponse {
        InteractionResponse {
            r#type: self.r#type,
            data: self.data,
        }
    }

    /// Sets the [`InteractionResponseType`]
    pub fn respond_type(mut self, t: InteractionResponseType) -> Self {
        self.r#type = t;
        self
    }

    /// Fills the [`InteractionResponse`] with some [`InteractionApplicationCommandCallbackData`]
    /// This returns an `InteractionResponse` and consumes itself.
    pub fn data(mut self, d: InteractionApplicationCommandCallbackData) -> InteractionResponse {
        self.data = Some(d);
        self.ret()
    }

    /// Sets the Text-To-Speech value of this `InteractionResponse`.
    pub fn tts(mut self, enable: bool) -> Self {
        // Does data exist?
        if self.data.is_none() {
            let mut d = InteractionApplicationCommandCallbackData::new();
            d.tts = Some(enable);
            self.data = Some(d);
        } else {
            self.data.as_mut().unwrap().tts = Some(enable);
        }
        self
    }

    /// This sets the `content` for an `InteractionResponse`
    pub fn content(mut self, c: &str) -> Self {
        match self.data.as_mut() {
            None => {
                let mut d = InteractionApplicationCommandCallbackData::new();
                d.content = Some(String::from(c));
                self.data = Some(d);
            }
            Some(mut d) => {
                d.content = Some(String::from(c));
            }
        }
        self
    }

    /// Sets the `content` for an `InteractionResponse`. Alias for `content()`
    pub fn message(self, c: &str) -> Self {
        self.content(c)
    }

    /// Add an [`Embed`] to the response.
    /// You can add up to 10 embeds.
    pub fn add_embed(mut self, e: Embed) -> Self {
        match self.data.as_mut() {
            None => {
                let mut d = InteractionApplicationCommandCallbackData::new();
                d.embeds = Some(vec![e]);
                self.data = Some(d);
            }
            Some(mut d) => {
                if d.embeds.is_none() {
                    d.embeds = Some(vec![e]);
                } else {
                    let v = d.embeds.as_mut().unwrap();
                    // Check if this will exceed the embed limit
                    if v.len() <= 9 {
                        v.push(e);
                    } else {
                        // Log an error for now.
                        error!("Tried to add embed while embed limit (max. 10 embeds) was already reached. Ignoring")
                    }
                }
            }
        }
        self
    }

    /// Returns an `InteractionResponse`, consuming itself.
    /// You can't use the builder anymore after you called this function.
    pub fn finish(self) -> InteractionResponse {
        self.ret()
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]

/// Representing the type of response to an [`Interaction`]
pub enum InteractionResponseType {
    /// ACK a PING
    Pong = 1,
    /// Respond to an [`Interaction`] with a message
    ChannelMessageWithSource = 4,
    /// ACK an interaction and edit a response later, the user sees a loading state
    DefferedChannelMessageWithSource = 5,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Representing the data used to respond to an [`Interaction`]
pub struct InteractionApplicationCommandCallbackData {
    tts: Option<bool>,
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    allowed_mentions: Option<AllowedMentions>,
    flags: Option<i32>,
}

impl InteractionApplicationCommandCallbackData {
    /// Creates a new [`InteractionApplicationCommandCallbackData`]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for InteractionApplicationCommandCallbackData {
    fn default() -> Self {
        Self {
            tts: None,
            content: None,
            embeds: None,
            allowed_mentions: None,
            flags: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Representing the allowed mention type
pub enum AllowedMentionTypes {
    /// Role mentions
    Roles,
    /// User mentions
    Users,
    /// @everyone mentions
    Everyone,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Representing the AllowedMentions data model
pub struct AllowedMentions {
    parse: Vec<AllowedMentionTypes>,
    roles: Vec<Snowflake>,
    users: Vec<Snowflake>,
    replied_user: bool,
}
#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
/// Representing a webhook message
pub struct WebhookMessage {
    /// The message contents
    pub content: Option<String>,
    /// Embeds in the message (max 10)
    pub embeds: Option<Vec<Embed>>,
    /// Used for files.
    pub payload_json: Option<String>,
    allowed_mentions: Option<AllowedMentions>,
}

impl From<InteractionResponse> for WebhookMessage {
    fn from(o: InteractionResponse) -> WebhookMessage {
        let data = o.data.unwrap();

        WebhookMessage {
            content: data.content,
            embeds: data.embeds,
            payload_json: None,
            allowed_mentions: None,
        }
    }
}
