use serde::{Serialize, Deserialize};

//pub mod map;
pub mod url_map;
pub type UrlMap = url_map::UrlMap;

pub mod url_type;
pub type UrlType = url_type::UrlType;

pub mod static_map;
pub type StaticMap = static_map::StaticMap;

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