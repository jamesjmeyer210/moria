use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "jwtKeyName")]
    pub jwt_key_name: String,
    #[serde(rename = "jwtSecret")]
    pub jwt_secret: String,
    #[serde(rename = "maxConnection")]
    pub max_connection: usize,
    #[serde(rename = "maxRateOfConnection")]
    pub max_rate_of_connection: usize,
    pub timeout: usize,
    #[serde(rename = "maxPayloadSize")]
    pub max_payload_size: usize,
}

#[derive(Clone)]
pub struct AuthObj {
    pub origin: String,
    pub groups: Vec<String>,
}

impl AuthObj {
    pub fn new(o: String, g: Vec<String>) -> Self {
        AuthObj {
            origin: o,
            groups: g,
        }
    }
}