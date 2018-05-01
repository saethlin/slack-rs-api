// Copyright 2015-2016 the slack-rs authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Low-level, direct interface for the [Slack Web
//! API](https://api.slack.com/methods).
extern crate reqwest;
#[macro_use]
extern crate derive_error;

pub mod requests;

#[macro_use]
macro_rules! api_call {
    ($name:ident, $strname:expr, $reqty:ty, Result<$okty:ty, $errty:tt>) => {
        pub fn $name (
            client: &::reqwest::Client,
            token: &str,
            request: &$reqty,
        ) -> Result<$okty, $errty>
        {
            #[derive(Deserialize)]
            struct Temp {
                error: $errty,
            }

            let url = ::get_slack_url_for_method($strname) + "?token=" + token;
            let bytes = ::requests::send_structured(client, &url, &request).map_err($errty::Client)?;
            match serde_json::from_str::<$okty>(&bytes) {
                Ok(v) => Ok(v),
                Err(_) => match serde_json::from_str::<Temp>(&bytes) {
                    Ok(temp) => Err(temp.error),
                    Err(e) => Err($errty::MalformedResponse(e)),
                }
            }
        }
    }
}

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_qs;

mod mods;
pub use mods::*;

mod types;
pub use types::*;

pub use requests::default_client;

fn get_slack_url_for_method(method: &str) -> String {
    format!("https://slack.com/api/{}", method)
}

fn optional_struct_or_empty_array<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: serde::Deserialize<'de> + Default,
          D: serde::Deserializer<'de>
{
    use std::marker::PhantomData;
    use serde::de;

    struct StructOrEmptyArray<T>(PhantomData<T>);

    impl<'de, T> de::Visitor<'de> for StructOrEmptyArray<T>
        where T: de::Deserialize<'de> + Default
    {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct or empty array")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Option<T>, A::Error>
            where A: de::SeqAccess<'de>
        {
            match seq.next_element::<T>()? {
                Some(_) => Err(de::Error::custom("non-empty array is not valid")),
                None => Ok(Some(T::default())),
            }
        }

        fn visit_unit<E>(self) -> Result<Option<T>, E>
            where E: de::Error
        {
            Ok(None)
        }

        fn visit_map<M>(self, access: M) -> Result<Option<T>, M::Error>
            where M: de::MapAccess<'de>
        {
            T::deserialize(de::value::MapAccessDeserializer::new(access)).map(Some)
        }
    }

    deserializer.deserialize_any(StructOrEmptyArray(PhantomData))
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::UserProfile;

    #[test]
    fn test_user_profile_fields_empty_array_deserialize() {
        let user_profile: UserProfile = serde_json::from_str(r#"{"fields": []}"#).unwrap();
        assert_eq!(0, user_profile.fields.unwrap().len());
    }

    #[test]
    fn test_user_profile_fields_empty_map_deserialize() {
        let user_profile: UserProfile = serde_json::from_str(r#"{"fields": {}}"#).unwrap();
        assert_eq!(0, user_profile.fields.unwrap().len());
    }

    #[test]
    fn test_user_profile_fields_nonempty_map_deserialize() {
        let user_profile: UserProfile = serde_json::from_str(r#"{"fields": {"some_field": {"alt": "foo", "label": "bar"}}}"#).unwrap();
        assert_eq!(1, user_profile.fields.unwrap().len());
    }

    #[test]
    fn test_user_profile_fields_null_deserialize() {
        let user_profile: UserProfile = serde_json::from_str(r#"{"fields": null}"#).unwrap();
        assert!(user_profile.fields.is_none());
    }

    #[test]
    fn test_user_profile_fields_undefined_deserialize() {
        let user_profile: UserProfile = serde_json::from_str(r#"{}"#).unwrap();
        assert!(user_profile.fields.is_none());
    }
}
