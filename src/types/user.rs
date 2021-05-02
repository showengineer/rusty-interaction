use serde::{Deserialize, Serialize};

use serde_with::*;
use ::chrono::{DateTime, Utc};

use super::{Snowflake};

// ======= STRUCTS =======

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