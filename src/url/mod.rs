use serde::{Serialize, Deserialize};

pub mod url_type;
pub mod static_map;

#[derive(Clone)]
pub struct MetaData {
    pub origin: String,
    pub groups: Vec<String>,
}

impl MetaData {
    pub fn new(o: String, g: Vec<String>) -> Self {
        MetaData {
            origin: o,
            groups: g,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Endpoint {
    pub path: String,
    pub method: String,
    pub groups: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Domain {
    pub origin: String,
    pub endpoints: Vec<Endpoint>,
}