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

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Debug)]
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

    #[test]
    #[should_panic]
    fn load_config_fails_on_invalid_path() {
        let _config = Config::from_file("/dev/null/config.json");
    }

    #[test]
    #[should_panic]
    fn load_config_fails_on_invalid_json(){
        let _config = Config::from_file("test/test.json");
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