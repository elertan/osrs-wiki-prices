use serde::{de, Deserialize, Deserializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(u32);

impl ItemId {
    pub fn new(id: u32) -> Self {
        ItemId(id)
    }

    pub fn id(&self) -> u32 {
        self.0
    }
}

struct ItemIdVisitor;

impl<'de> de::Visitor<'de> for ItemIdVisitor {
    type Value = ItemId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a u32 or a string representing a u32")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let id = u32::try_from(value).map_err(|_| E::custom("value out of range for u32"))?;
        Ok(ItemId::new(id))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let id = value.parse::<u32>().map_err(E::custom)?;
        Ok(ItemId::new(id))
    }
}

impl<'de> Deserialize<'de> for ItemId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ItemIdVisitor)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_id() {
        let id = ItemId::new(123);
        assert_eq!(id.id(), 123);

        let json = r#""123""#;
        let deserialized: ItemId = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.id(), 123);

        let json = r#"123"#;
        let deserialized: ItemId = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.id(), 123);
    }
}