//! Configurations for bitcask.

use failure::Fail;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::Path;

#[derive(Debug, Fail)]
enum ConfigError {
    #[fail(display = "file {} not found", name)]
    FileNotFound { name: String },

    #[fail(display = "malformed format while deserializing json")]
    MalformedFormat,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u32,
}

fn default_host() -> String {
    String::from("0.0.0.0")
}

fn default_port() -> u32 {
    7616
}

impl Config {
    /// Load config from json string.
    fn from_str(source: &str) -> Result<Config, failure::Error> {
        let config = serde_json::from_str(source)?;
        Ok(config)
    }

    /// Load config from file.
    pub fn file<P: AsRef<Path>>(p: P) -> Result<Config, failure::Error> {
        let text = std::fs::read_to_string(p)?;

        let config = serde_json::from_str(&text)?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loading() {
        let c0 = "{\"host\": \"127.0.0.1\", \"port\": 1222}";
        let c = Config::from_str(c0).expect("should success deserialize correct json format");
        assert_eq!(
            Config {
                host: String::from("127.0.0.1"),
                port: 1222,
            },
            c
        );

        // fallback using default
        let c1 = "{\"port\": 1222}";
        let c = Config::from_str(c1).expect("should success deserialize partial json content");
        assert_eq!(
            Config {
                host: default_host(),
                port: 1222,
            },
            c
        );

        // fallback from empty
        let c2 = "{}";
        let c = Config::from_str(c2).expect("should success deserialize empty json format");
        assert_eq!(
            Config {
                host: default_host(),
                port: default_port(),
            },
            c
        )
    }
}
