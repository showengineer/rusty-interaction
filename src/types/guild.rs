use serde::{Deserialize, Serialize};
use crate::handler::InteractionHandler;
use ::chrono::{DateTime, Utc};
use serde_with::*;

use super::Snowflake;
use super::HttpError;
use reqwest::{Client, StatusCode};

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// A guild (also known as a 'server') in Discord
pub struct Guild{
    /// The ID of the guild
    #[serde_as(as = "DisplayFromStr")]
    pub id: Snowflake,
    /// Name of this guild
    pub name: String,

    /// Icon hash
    pub icon: Option<String>,
    /// Icon hash, returned when in the template object
    pub icon_hash: Option<String>,

    /// Splash hash
    pub splash: Option<String>,
    /// discovery splash hash; only present for guilds with the `DISCOVERABLE` feature
    pub discovery_splash: Option<String>,
    /// id of owner
    #[serde_as(as = "DisplayFromStr")]
    pub owner_id: Snowflake,
    /// voice region id for the guild
    pub region: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// Id of afk channel
    pub afk_channel_id: Option<Snowflake>,
    /// AFK timeout in seconds
    pub afk_timeout: u32,
    /// true if widget is enabled
    pub widget_enabled: Option<bool>,
    /// The channel id that the widget will generate an invite to, or null if set to no invite
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub widget_channel_id: Option<Snowflake>,
    /// [Verfication level](https://discord.com/developers/docs/resources/guild#guild-object-verification-level) required for the guild
    pub verfication_level: Option<u8>,
    /// default [message notifications level](https://discord.com/developers/docs/resources/guild#guild-object-default-message-notification-level)
    pub default_message_notifications: Option<u8>,
    /// [Explicit content filter level](https://discord.com/developers/docs/resources/guild#guild-object-explicit-content-filter-level)
    pub explicit_content_filter: Option<u8>,

    /// Roles in this guild
    pub roles: Vec<Role>,

    /// The required [MFA level](https://discord.com/developers/docs/resources/guild#guild-object-mfa-level) in this guild
    pub mfa_level: u8,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    ///	application id of the guild creator if it is bot-created
    pub application_id: Option<Snowflake>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// the id of the channel where guild notices such as welcome messages and boost events are posted
    pub system_channel_id: Option<Snowflake>,
    /// system channel flags
    pub system_channel_flags: Option<u8>,
    /// the id of the channel where Community guilds can display rules and/or guidelines
    pub rules_channel_id: Option<Snowflake>,

    /// the vanity url code for the guild
    pub vanity_url_code: Option<String>,
    ///	the description for the guild, if the guild is discoverable
    pub description: Option<String>,

    /// premium tier (Server Boost level)
    pub premium_tier: u8,
    ///	the number of boosts this guild currently has
    pub premium_tier_subscription_count: Option<u8>,

    /// the preferred locale of a Community guild; used in server discovery and notices from Discord; defaults to "en-US"
    pub preffered_locale: Option<String>,
    /// the maximum amount of users in a video channel
    pub max_video_channel_users: Option<u32>,
    /// approximate number of members in this guild
    pub approximate_member_count: u32,
    /// approximate number of non-offline members in this guild
    pub approximate_presence_count: u32,
    /// true if this guild is [designated as NSFW](https://support.discord.com/hc/en-us/articles/1500005389362-NSFW-Server-Designation)
    pub nsfw: bool,
}

impl Into<Snowflake> for Guild{
    fn into(self) -> Snowflake{
        self.id
    }
}

#[serde_as]
#[skip_serializing_none]
/// A role is a way to group people in a Guild and assign certain permissions to them.
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Role{
    #[serde_as(as = "DisplayFromStr")]
    /// The id of the role
    pub id: Snowflake,

    /// The name of the role
    pub name: String,
    /// Role color
    pub color: u32,

    ///	If this role is pinned in the user listing
    pub hoist: bool,

    /// Position of the role
    pub position: u16,

    /// Permission bit set
    pub permissions: String,

    /// Whether this role is managed by an integration
    pub managed: bool,

    /// Whether this role is mentionable
    pub mentionable: bool,

    /// The tags this role has
    pub tags: Option<RoleTag>
}
#[serde_as]
#[skip_serializing_none]
/// Role tags
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct RoleTag{
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// the id of the bot this role belongs to
    pub bot_id: Option<Snowflake>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// the id of the integration this role belongs to
    pub integration_id: Option<Snowflake>,
    /// whether this is the guild's premium subscriber role. 
    pub premium_subscriber: Option<String>
}


