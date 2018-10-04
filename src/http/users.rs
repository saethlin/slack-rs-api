//! Get info on members of your Slack team.

use id::ChannelId;
use id::UserId;
use rtm::Cursor;
use rtm::{Paging, Team, User};
use timestamp::Timestamp;

/// Delete the user profile photo
///
/// Wraps https://api.slack.com/methods/users.deletePhoto

api_call!(delete_photo, "users.deletePhoto");

/// Gets user presence information.
///
/// Wraps https://api.slack.com/methods/users.getPresence

api_call!(
    get_presence,
    "users.getPresence",
    GetPresenceRequest =>
    GetPresenceResponse
);

#[derive(Clone, Debug, Serialize, new)]
pub struct GetPresenceRequest {
    /// User to get presence info on. Defaults to the authed user.
    pub user: UserId,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetPresenceResponse {
    ok: bool,
    pub presence: Option<String>,
}

/// Get a user's identity.
///
/// Wraps https://api.slack.com/methods/users.identity

api_call!(identity, "users.identity", => IdentityResponse);

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityResponse {
    ok: bool,
    pub team: Option<Team>,
    pub user: Option<User>,
}

/// Gets information about a user.
///
/// Wraps https://api.slack.com/methods/users.info

api_call!(info, "users.info", InfoRequest => InfoResponse);

#[derive(Clone, Debug, Serialize, new)]
pub struct InfoRequest {
    /// User to get info on
    pub user: UserId,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfoResponse {
    ok: bool,
    pub user: Option<User>,
}

/// Lists all users in a Slack team.
///
/// Wraps https://api.slack.com/methods/users.list

api_call!(list, "users.list", ListRequest => ListResponse);

/// At this time, providing no limit value will result in Slack
/// attempting to deliver you the entire result set.
/// If the collection is too large you may experience HTTP 500 errors.
/// Resolve this scenario by using pagination.
///
/// One day pagination will become required to use this method.
#[derive(Clone, Debug, Serialize, new)]
pub struct ListRequest {
    /// Whether to include presence data in the output
    #[new(default)]
    pub presence: Option<bool>,
    #[new(default)]
    pub cursor: Option<Cursor>,
    #[new(default)]
    pub limit: Option<usize>,
    #[new(default)]
    pub include_locale: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponse {
    ok: bool,
    pub members: Vec<User>,
    pub cache_ts: Option<Timestamp>,
    pub response_metadata: Option<Paging>,
    pub is_limited: Option<bool>,
}

/// Gets a users's preferences
///
/// Wraps https://api.slack.com/methods/users.prefs.get

api_call!(prefs_get, "users.prefs.get", => PrefsResponse);

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrefsResponse {
    ok: bool,
    pub prefs: UserPrefs,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserPrefs {
    muted_channels: Vec<ChannelId>,
}

/// Marks a user as active.
///
/// Wraps https://api.slack.com/methods/users.setActive

api_call!(set_active, "users.setActive");

/// Manually sets user presence.
///
/// Wraps https://api.slack.com/methods/users.setPresence

api_call!(set_presence, "users.setPresence", SetPresenceRequest =>);

#[derive(Clone, Debug, Serialize, new)]
pub struct SetPresenceRequest {
    /// Either auto or away
    pub presence: Presence,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename = "snake_case")]
pub enum Presence {
    Auto,
    Away,
}