use serde::{Deserialize, Serialize};

use serde_with::*;

use serde_repr::*;

use super::embed::*;
use super::user::*;
use super::Snowflake;

#[cfg(feature = "handler")]
use log::{error, info};
#[cfg(feature = "handler")]
use reqwest::{Client, StatusCode};

#[cfg(feature = "handler")]
#[derive(Clone)]
pub struct Context {
    client: Client,
    pub interaction: Interaction,
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

#[cfg(feature = "handler")]
impl Context {
    pub fn new(c: Client, i: Interaction) -> Self {
        Self {
            client: c,
            interaction: i,
        }
    }

    /// Respond to an Interaction
    pub fn respond(&self) -> InteractionResponseBuilder {
        InteractionResponseBuilder::default()
    }

    pub async fn edit_original(&self, new_content: &WebhookMessage) {
        let url = format!(
            "{}/webhooks/{:?}/{}/messages/@original",
            crate::BASE_URL,
            self.interaction.application_id.unwrap(),
            self.interaction.token.as_ref().unwrap().to_string()
        );
        println!("{}", url);
        println!("{:#?}", new_content);
        println!("Excluding any headers");
        let c = self.client.patch(&url).json(new_content).send().await;

        match c {
            Err(e) => {
                error!("Editing original message failed: {:?}", e)
            }
            Ok(r) => {
                if r.status() != StatusCode::OK {
                    error!(
                        "Editing original message failed: API Returned {:?}{:#?}",
                        r.status(),
                        r.text().await
                    );
                } else {
                    info!("Sucessfully edited original message: {:#?}", r.text().await)
                }
            }
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

#[cfg(feature = "handler")]
#[derive(Clone, Debug)]
pub struct InteractionResponseBuilder {
    pub r#type: InteractionResponseType,
    pub data: Option<InteractionApplicationCommandCallbackData>,
}

impl InteractionResponse {
    pub fn new(
        rtype: InteractionResponseType,
        data: Option<InteractionApplicationCommandCallbackData>,
    ) -> InteractionResponse {
        InteractionResponse {
            r#type: rtype,
            data: data,
        }
    }
}

#[cfg(feature = "handler")]
impl Default for InteractionResponseBuilder {
    /// This will default to responding with the `InteractionResponseType::CHANNEL_MESSAGE_WITH_SOURCE` response type and no data.
    /// Adding data yourself is expected.
    fn default() -> Self {
        Self {
            r#type: InteractionResponseType::CHANNEL_MESSAGE_WITH_SOURCE,
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

    pub fn respond_type(mut self, t: InteractionResponseType) -> Self {
        self.r#type = t;
        self
    }

    /// Fills the `InteractionResponse` with some `InteractionApplicationCommandCallbackData`
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
        return self.content(c);
    }

    /// Add an `Embed` to the response.
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
#[allow(non_camel_case_types)]
pub enum InteractionResponseType {
    PONG = 1,
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
    flags: Option<i32>,
}

impl InteractionApplicationCommandCallbackData {
    pub fn new() -> Self {
        Self {
            tts: None,
            content: None,
            embeds: None,
            allowed_mentions: None,
            flags: None,
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
#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WebhookMessage {
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
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
