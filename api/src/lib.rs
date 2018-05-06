//! Get info on your team's Slack channels, create or archive channels, invite users, set the topic and purpose, and mark a channel as read.

extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_qs;

#[macro_use]
extern crate requests;

/// Checks API calling code.
///
/// Wraps https://api.slack.com/methods/api.test

api_call!(test, "api.test", TestRequest, TestResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct TestRequest<'a> {
    /// Error response to return
    pub error: Option<&'a str>,
    /// example property to return
    pub foo: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestResponse {
    ok: bool,
    pub args: std::collections::HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_test() {
        let client = requests::default_client().unwrap();
        let token = env::var("SLACK_API_TOKEN").unwrap();

        let req = TestRequest {
            error: None,
            foo: Some("bar"),
        };
        let response = test(&client, &token, &req).unwrap();
        assert_eq!(response.args["foo"], "bar".to_string())
    }
}