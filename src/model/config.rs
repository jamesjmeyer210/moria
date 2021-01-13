use serde::{Deserialize, Serialize};
use std::fs;
#[macro_use]
use super::util::load_json;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    InvalidPublicKey,
    InvalidPrivateKey,
    InvalidPort,
    InvalidMaxConnection,
    InvalidMaxRateOfConnection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "jwtKeyName")]
    pub jwt_key_name: String,

    #[serde(rename = "jwtSecret")]
    pub jwt_secret: String,

    pub ip: String,
    pub port: u16, // ports can only be 1-65535

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

impl Config {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let config: Config = load_json_file!(path);

        if config.port == 0 {
            return Err(ConfigError::InvalidPort);
        } else if config.max_connection == 0 {
            return Err(ConfigError::InvalidMaxConnection);
        } else if config.max_rate_of_connection == 0 {
            return Err(ConfigError::InvalidMaxRateOfConnection);
        } else if fs::metadata(&config.ssl_public_key).is_err() {
            return Err(ConfigError::InvalidPublicKey);
        } else if fs::metadata(&config.ssl_private_key).is_err() {
            return Err(ConfigError::InvalidPrivateKey);
        }

        return Ok(config);
    }

    pub fn default() -> Self {
        Config {
            jwt_key_name: "jwt-token".to_string(),
            jwt_secret: "secret".to_string(),
            ip: "127.0.0.1".to_string(),
            port: 443,
            ssl_public_key: "ssl/key.pem".to_string(),
            ssl_private_key: "ssl/cert.pem".to_string(),
            max_connection: 25000,
            max_rate_of_connection: 255,
            timeout: 1000,
            max_payload_size: 1000,
        }
    }

    pub fn from_jwt_key_value(jwt_key: &str, jwt_value: &str) -> Self {
        Config {
            jwt_key_name: jwt_key.to_string(),
            jwt_secret: jwt_value.to_string(),
            ip: "127.0.0.1".to_string(),
            port: 8000,
            ssl_public_key: "".to_string(),
            ssl_private_key: "".to_string(),
            max_connection: 0,
            max_rate_of_connection: 0,
            timeout: 0,
            max_payload_size: 0,
        }
    }
}