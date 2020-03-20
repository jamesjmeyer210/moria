use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};
use crate::model::{AuthObj};

macro_rules! load_json_file {
    ($path:expr) => (
        serde_json::from_str(
            fs::read_to_string($path).unwrap_or_else(|error|{
                panic!("Error reading in file {}\n{}", $path, error)
            })
            .as_str()
        ).unwrap_or_else(|error|{
            panic!("Error converting file {}\n{}", $path, error);
        })
    )
}

#[derive(Serialize, Deserialize)]
struct Endpoint {
    path: String,
    method: String,
    groups: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Domain {
    origin: String,
    endpoints: Vec<Endpoint>,
}

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

pub fn load_endpoints(path: &str) -> HashMap<String,AuthObj> {
    let domains: Vec<Domain> = load_json_file!(path);

    let mut map: HashMap<String,AuthObj> = HashMap::new();

    for domain in domains {
        for endpoint in domain.endpoints {
            let key = format!("{} {}", endpoint.method, &endpoint.path);
            map.insert(key, AuthObj::new(
                domain.origin.clone(),
                endpoint.groups,
            ));
        }
    }

    return map;
}

#[cfg(test)]
mod test {

    use super::*;

    fn default_config() -> Config {
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

    macro_rules! before_config {
        ($var:ident, $val:expr, $path:expr) => (
            {
                let mut invalid_config = default_config();
                invalid_config.$var = $val;

                let json = serde_json::to_string(&invalid_config).unwrap();
                fs::write($path, json).unwrap();
            }
        )
    }

    macro_rules! after_config {
        ($path:expr) => (
            {
                fs::remove_file($path).unwrap();
            }
        )
    }

    #[test]
    #[should_panic]
    fn from_file_fails_on_invalid_path() {
        let _config = Config::from_file("/dev/null/config.json");
    }

    #[test]
    #[should_panic]
    fn from_file_fails_on_invalid_json(){
        let _config = Config::from_file("test/test.json");
    }

    #[test]
    fn from_file_returns_invalid_port_when_port_is_zero(){
        before_config!(port, 0, "/tmp/invalid_port.json");
        let error = Config::from_file("/tmp/invalid_port.json").unwrap_err();

        assert_eq!(ConfigError::InvalidPort, error);

        after_config!("/tmp/invalid_port.json")
    }

    #[test]
    fn from_file_returns_invalid_private_ssl_key_when_file_not_found(){
        before_config!(
            ssl_public_key,
            "test/ssl/key.pem".to_string(),
            "/tmp/invalid_private_ssl_key.json"
        );
        let error = Config::from_file("/tmp/invalid_private_ssl_key.json").unwrap_err();

        assert_eq!(ConfigError::InvalidPublicKey, error);

        after_config!("/tmp/invalid_private_ssl_key.json");
    }

    #[test]
    fn from_file_returns_invalid_public_ssl_key_when_file_not_found(){
        before_config!(
            ssl_private_key,
            "test/ssl/cert.pem".to_string(),
            "/tmp/invalid_public_ssl_key.json"
        );
        let error = Config::from_file("/tmp/invalid_public_ssl_key.json").unwrap_err();

        assert_eq!(ConfigError::InvalidPrivateKey, error);

        after_config!("/tmp/invalid_public_ssl_key.json");
    }

    #[test]
    fn from_file_returns_invalid_max_connection_when_value_is_zero(){
        before_config!(
            max_connection,
            0,
            "/tmp/invalid_max_connection.json"
        );
        let error = Config::from_file("/tmp/invalid_max_connection.json").unwrap_err();

        assert_eq!(ConfigError::InvalidMaxConnection, error);

        after_config!("/tmp/invalid_max_connection.json");
    }

    #[test]
    fn from_file_returns_invalid_max_rate_of_connection_when_value_is_zero(){
        before_config!(
            max_rate_of_connection,
            0,
            "/tmp/invalid_max_rate_of_connection.json"
        );
        let error = Config::from_file("/tmp/invalid_max_rate_of_connection.json").unwrap_err();

        assert_eq!(ConfigError::InvalidMaxRateOfConnection, error);

        after_config!("/tmp/invalid_max_rate_of_connection.json");
    }

    #[test]
    #[should_panic]
    fn load_endpoints_fails_on_invalid_path(){
        let _config = Config::from_file("/dev/null/endpoints.json");
    }

    #[test]
    #[should_panic]
    fn load_endpoints_fails_on_invalid_json(){
        let _config = Config::from_file("test/test.json");
    }

}