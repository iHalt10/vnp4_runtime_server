use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub bitwidth: i32,
    pub signed: bool,
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (name, bitwidth, signed) = <(String, i32, bool)>::deserialize(deserializer)?;
        Ok(Field { name, bitwidth, signed })
    }
}

impl Serialize for Field {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (self.name.clone(), self.bitwidth, self.signed).serialize(serializer)
    }
}
