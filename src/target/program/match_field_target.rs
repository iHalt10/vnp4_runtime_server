use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

#[derive(Debug, Clone)]
pub struct MatchFieldTarget {
    pub header_name: String,
    pub field_name: String,
}

impl<'de> Deserialize<'de> for MatchFieldTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (header_name, field_name) = <(String, String)>::deserialize(deserializer)?;
        Ok(MatchFieldTarget { header_name, field_name })
    }
}

impl Serialize for MatchFieldTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (self.header_name.clone(), self.field_name.clone()).serialize(serializer)
    }
}
