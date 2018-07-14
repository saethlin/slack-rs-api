use serde::de::{self, Deserialize, Deserializer, Error, Visitor};
use std::fmt;

// NOTE: This falls apart after a few decades
#[derive(Clone, Copy, Debug)]
pub struct Timestamp {
    seconds: u32,
    milliseconds: Option<[u8; 3]>,
}

impl Into<::chrono::DateTime<::chrono::Utc>> for Timestamp {
    fn into(self) -> ::chrono::DateTime<::chrono::Utc> {
        let naive = ::chrono::naive::NaiveDateTime::from_timestamp(
            self.seconds as i64,
            u8s_to_u32(&self.milliseconds.unwrap_or([0, 0, 0])),
        );
        ::chrono::DateTime::from_utc(naive, ::chrono::Utc)
    }
}

struct TimestampVisitor;
use serde::de::MapAccess;
impl<'de> Visitor<'de> for TimestampVisitor {
    type Value = Timestamp;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Unix-style timestamp, as a u32 or string")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut message = String::new();
        while let Some((l, r)) = access.next_entry()? {
            let l: String = l;
            let r: String = r;
            message += &format!("{}, {}\n", l, r);
        }
        Err(Error::custom(message))
    }

    fn visit_str<E>(self, value: &str) -> Result<Timestamp, E>
    where
        E: de::Error,
    {
        if value.len() <= 17 {
            // Split at the decimal point
            let dot_location = value
                .find('.')
                .ok_or_else(|| Error::custom("Got a string without a ."))?;
            let (seconds_str, millis_str) = value.split_at(dot_location);
            Ok(Timestamp {
                seconds: seconds_str
                    .parse()
                    .map_err(|_| Error::custom(format!("Cannot parse {} as a number", seconds_str)))?,
                milliseconds: Some(u32_to_u8s(
                    millis_str[1..]
                        .parse()
                        .map_err(|_| Error::custom(format!("Cannot parse {} as a number", millis_str)))?,
                )),
            })
        } else {
            Err(E::custom(format!(
                "Timestamps must be string or number with 16 decimal places, got {}",
                value
            )))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Timestamp, E>
    where
        E: de::Error,
    {
        Ok(Timestamp {
            seconds: value as u32,
            milliseconds: None,
        })
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Timestamp, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TimestampVisitor)
    }
}

fn u32_to_u8s(x: u32) -> [u8; 3] {
    let b1: u8 = ((x >> 16) & 0xff) as u8;
    let b2: u8 = ((x >> 8) & 0xff) as u8;
    let b3: u8 = (x & 0xff) as u8;
    return [b1, b2, b3];
}

fn u8s_to_u32(x: &[u8; 3]) -> u32 {
    (x[0] as u32) << 16 | (x[1] as u32) << 8 | (x[2] as u32)
}

impl ::std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self.milliseconds {
            Some(millis) => write!(f, "\"{}.{:06}\"", self.seconds, u8s_to_u32(&millis)),
            None => write!(f, "{}", self.seconds),
        }
    }
}