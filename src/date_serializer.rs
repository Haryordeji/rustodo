use chrono::NaiveDate;
use serde::{self, Deserialize, Serializer, Deserializer};

pub fn serialize<S>(
    date: &NaiveDate,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.format("%Y-%m-%d").to_string();
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
}