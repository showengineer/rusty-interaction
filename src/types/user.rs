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


impl PartialEq for User{
    fn eq(&self, other: &Self) -> bool{
        self.id == other.id
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
/// Representing a Member in a guild.
pub struct Member {
    /// The user associated with this member
    pub user: User,
    /// The member's nickname, if any
    pub nick: Option<String>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    /// The member's assigned roles
    pub roles: Vec<Snowflake>,
    #[serde_as(as = "DisplayFromStr")]
    /// When this user joined
    pub joined_at: DateTime<Utc>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// When the member started boosting the server, if boosting
    pub premium_since: Option<DateTime<Utc>>,
    /// Is this member server deafened?
    pub deaf: bool,
    /// Is this member server muted (voice)?
    pub mute: bool,
    /// Pending status
    pub pending: bool,
    /// Permission overrides?
    pub permissions: Option<String>,
}

impl From<Member> for User {
    fn from(member: Member) -> User {
        member.user
    }
}

impl PartialEq for Member{
    fn eq(&self, other: &Self) -> bool{
        self.user.id == other.user.id
    }
}