use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "JwtKeyName")]
    pub jwt_key_name: String,
    #[serde(rename = "JwtSecret")]
    pub jwt_secret: String,
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