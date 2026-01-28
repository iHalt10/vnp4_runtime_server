use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ffi::CString;

pub fn serialize<S>(cstring: &CString, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    cstring.to_str().unwrap_or("").serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<CString, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    CString::new(s).map_err(serde::de::Error::custom)
}
