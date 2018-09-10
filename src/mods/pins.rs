/// Pins an item to a channel.
///
/// Wraps https://api.slack.com/methods/pins.add

api_call!(add, "pins.add", AddRequest => ());

#[derive(Clone, Debug, Serialize)]
pub struct AddRequest {
    /// Channel to pin the item in.
    pub channel: ::ChannelId,
    /// File to pin.
    pub file: Option<::FileId>,
    /// Timestamp of the message to pin.
    pub timestamp: Option<::Timestamp>,
}

/// Lists items pinned to a channel.
///
/// Wraps https://api.slack.com/methods/pins.list

api_call!(list, "pins.list", ListRequest, ListResponse);

#[derive(Clone, Debug, Serialize)]
pub struct ListRequest {
    /// Channel to get pinned items for.
    pub channel: ::ChannelId,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponse {
    pub items: Option<Vec<ListResponseItem>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum ListResponseItem {
    Message(ListResponseItemMessage),
    File(ListResponseItemFile),
    FileComment(ListResponseItemFileComment),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponseItemFile {
    pub created: Option<::Timestamp>,
    pub created_by: Option<::UserId>,
    pub file: ::File,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponseItemFileComment {
    pub comment: ::FileComment,
    pub created: Option<::Timestamp>,
    pub created_by: Option<::UserId>,
    pub file: ::File,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponseItemMessage {
    pub channel: ::ChannelId,
    pub created: Option<::Timestamp>,
    pub created_by: Option<::UserId>,
    pub message: ::Message,
}

/// Un-pins an item from a channel.
///
/// Wraps https://api.slack.com/methods/pins.remove

api_call!(remove, "pins.remove", RemoveRequest => ());

#[derive(Clone, Debug, Serialize)]
pub struct RemoveRequest {
    /// Channel where the item is pinned to.
    pub channel: ::ChannelId,
    /// File to un-pin.
    pub file: Option<::FileId>,
    /// Timestamp of the message to un-pin.
    pub timestamp: Option<::Timestamp>,
}
