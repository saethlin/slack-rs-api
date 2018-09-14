/// Gets the access logs for the current team.
///
/// Wraps https://api.slack.com/methods/team.accessLogs
use std::collections::HashMap;

api_call!(access_logs, "team.accessLogs", AccessLogsRequest => AccessLogsResponse);

#[derive(Clone, Debug, Serialize, new)]
pub struct AccessLogsRequest {
    /// Number of items to return per page.
    #[new(default)]
    pub count: Option<u32>,
    /// Page number of results to return.
    #[new(default)]
    pub page: Option<u32>,
    /// End of time range of logs to include in results (inclusive).
    #[new(default)]
    pub before: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessLogsResponse {
    ok: bool,
    pub logins: Option<Vec<AccessLogsResponseLogin>>,
    pub paging: Option<::Paging>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessLogsResponseLogin {
    pub count: Option<i32>,
    pub country: Option<String>,
    pub date_first: Option<f32>,
    pub date_last: Option<f32>,
    pub ip: Option<String>,
    pub isp: Option<String>,
    pub region: Option<String>,
    pub user_agent: Option<String>,
    pub user_id: Option<String>,
    pub username: Option<String>,
}

/// Gets billable users information for the current team.
///
/// Wraps https://api.slack.com/methods/team.billableInfo

api_call!(
    billable_info,
    "team.billableInfo",
    BillableInfoRequest =>
    BillableInfoResponse
);

#[derive(Clone, Debug, Serialize, new)]
pub struct BillableInfoRequest {
    /// A user to retrieve the billable information for. Defaults to all users.
    #[new(default)]
    pub user: Option<::UserId>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BillableInfoResponse {
    ok: bool,
    pub billable_info: HashMap<String, bool>,
}

/// Gets information about the current team.
///
/// Wraps https://api.slack.com/methods/team.info

api_call!(info, "team.info", => InfoResponse);

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfoResponse {
    ok: bool,
    pub team: Option<::Team>,
}

/// Gets the integration logs for the current team.
///
/// Wraps https://api.slack.com/methods/team.integrationLogs

api_call!(
    integration_logs,
    "team.integrationLogs",
    IntegrationLogsRequest =>
    IntegrationLogsResponse
);

#[derive(Clone, Debug, Serialize, new)]
pub struct IntegrationLogsRequest<'a> {
    /// Filter logs to this service. Defaults to all logs.
    #[new(default)]
    pub service_id: Option<&'a str>,
    /// Filter logs to this Slack app. Defaults to all logs.
    #[new(default)]
    pub app_id: Option<::AppId>,
    /// Filter logs generated by this user’s actions. Defaults to all logs.
    #[new(default)]
    pub user: Option<::UserId>,
    /// Filter logs with this change type. Defaults to all logs.
    #[new(default)]
    pub change_type: Option<&'a str>,
    /// Number of items to return per page.
    #[new(default)]
    pub count: Option<u32>,
    /// Page number of results to return.
    #[new(default)]
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrationLogsResponse {
    ok: bool,
    pub logs: Option<Vec<IntegrationLogsResponseLog>>,
    pub paging: Option<::Paging>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrationLogsResponseLog {
    pub app_id: Option<String>,
    pub app_type: Option<String>,
    pub change_type: Option<String>,
    pub channel: Option<String>,
    pub date: Option<String>,
    pub reason: Option<String>,
    pub scope: Option<String>,
    pub service_id: Option<String>,
    pub service_type: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
}
