use serde::{Serialize, Deserialize};
use std::fs;
use super::*;
use crate::url::Domain;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "jwtKeyName")]
    pub jwt_key_name: String,
    #[serde(rename = "jwtSecret")]
    pub jwt_secret: String,
    pub ip: String,
    pub port: u16,  // ports can only be 1-65535
    #[serde(rename = "sslPublicKey")]
    pub ssl_public_key: String,
    #[serde(rename = "sslPrivateKey")]
    pub ssl_private_key: String,
    #[serde(rename = "maxConnection")]
    pub max_connection: usize, // cannot be 0
    #[serde(rename = "maxRateOfConnection")]
    pub max_rate_of_connection: usize, // cannot be 0
    pub timeout: usize,
    #[serde(rename = "maxPayloadSize")]
    pub max_payload_size: usize,
}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    InvalidPublicKey,
    InvalidPrivateKey,
    InvalidPort,
    InvalidMaxConnection,
    InvalidMaxRateOfConnection,
}

impl Config {

    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let config: Config = load_json_file!(path);

        if config.port == 0 {
            return Err(ConfigError::InvalidPort)
        }
        else if config.max_connection == 0 {
            return Err(ConfigError::InvalidMaxConnection)
        }
        else if config.max_rate_of_connection == 0 {
            return Err(ConfigError::InvalidMaxRateOfConnection)
        }
        else if fs::metadata(&config.ssl_public_key).is_err() {
            return Err(ConfigError::InvalidPublicKey)
        }
        else if fs::metadata(&config.ssl_private_key).is_err() {
            return Err(ConfigError::InvalidPrivateKey)
        }

        return Ok(config)
    }

}