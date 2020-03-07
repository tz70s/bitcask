use bytes::Bytes;
use failure::Error;
use serde::{Deserialize, Serialize};

/// Offset represents the log offset in file.
pub type Offset = u64;

/// Metadata storing offset and size of log record.
#[derive(Debug)]
pub struct Meta {
    pub offset: Offset,
    pub size: usize,
}

pub struct Repr {
    pub size: usize,
    pub bs: Bytes,
}

impl Repr {
    pub fn new(bs: Bytes) -> Repr {
        let size = bs.len();
        Repr { size, bs }
    }

    // Construct a Repr from given record.
    pub fn encode_from(record: Record) -> Result<Repr, Error> {
        let bs = record.to_bytes()?;
        Ok(Repr { size: bs.len(), bs })
    }
}

/// Single entry of recording log.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Record {
    pub key: String,
    pub val: String,
}

impl Record {
    /// Create a new record.
    pub fn new(key: String, val: String) -> Self {
        Record { key, val }
    }

    pub fn to_bytes(&self) -> Result<Bytes, Error> {
        let vec = serde_json::to_vec(self)?;
        let bytes = Bytes::from(vec);
        Ok(bytes)
    }

    pub fn from_repr(repr: Repr) -> Result<Record, Error> {
        serde_json::from_slice(&repr.bs[..]).map_err(|e| e.into())
    }

    /// Serialize to string.
    pub fn to_string(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|e| e.into())
    }

    /// Deserialize from string.
    pub fn from_str(raw: &str) -> Result<Record, Error> {
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
        let raw = "{\"key\": \"key0\",\"val\":\"12\"}";

        let record = Record::from_str(raw).expect("should deserialize from string");

        assert_eq!(record, Record::new("key0".to_string(), "12".to_string()));
    }
}
