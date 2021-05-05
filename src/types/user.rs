use serde::{Deserialize, Serialize};

use ::chrono::{DateTime, Utc};
use serde_with::*;

use super::Snowflake;

// ======= STRUCTS =======

#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
/// A Discord user
pub struct User {
    #[serde_as(as = "DisplayFromStr")]
    /// User id
    pub id: Snowflake,
    /// The username
    pub username: String,
    /// The discriminator. (Ex. `#1337`)
    pub discriminator: String,
    /// URL to their avatar/profile picture
    pub avatar: Option<String>,
    /// Is it a bot?
    pub bot: Option<bool>,
    /// Is it a system user?
    pub system: Option<bool>,
    /// Do they have 2FA enabled?
    pub mfa_enabled: Option<bool>,
    /// User set locale
    pub locale: Option<String>,
    /// Email verified?
    pub verified: Option<bool>,
    /// User's email address
    pub email: Option<String>,
    /// Flags set on user
    pub flags: Option<i32>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// Type of nitro subscription
    pub premium_type: Option<i8>,

    /// Public flags for user
    pub public_flags: Option<i32>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
/// Representing a Member in a guild.
pub struct Member {
    user: User,
    nick: Option<String>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    roles: Vec<Snowflake>,
    #[serde_as(as = "DisplayFromStr")]
    joined_at: DateTime<Utc>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    premium_since: Option<DateTime<Utc>>,
    deaf: bool,
    mute: bool,
    pending: bool,
    permissions: String,
}
