use failure::{Error, Fail};
use serde::{Deserialize, Serialize};

/// Offset represents the log offset in file.
pub type Offset = i32;

/// Single entry of recording log.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Record<V> {
    key: String,
    val: V,
}

impl<'d, V: 'd> Record<V>
    where
        V: Serialize + Deserialize<'d>,
{
    /// Create a new record.
    fn new(key: String, val: V) -> Self {
        Record { key, val }
    }

    /// Serialize to string.
    fn to_string(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|e| e.into())
    }

    /// Deserialize from string.
    fn from_str(raw: &'d str) -> Result<Record<V>, Error> {
        serde_json::from_str(raw).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::log::Record;

    #[test]
    fn test_record_serialization() {
        let record = Record::new(String::from("key0"), String::from("val0"));
        let result = record.to_string().expect("should serialize to string");
        assert_eq!(result, "{\"key\":\"key0\",\"val\":\"val0\"}")
    }

    #[test]
    fn test_record_deserialization() {
        let raw = "{\"key\": \"key0\",\"val\":12}";

        let record = Record::from_str(raw).expect("should deserialize from string");

        assert_eq!(record, Record::new("key0".to_string(), 12));
    }
}
