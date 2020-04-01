use serde::{Serialize, Deserialize};

//pub mod map;
mod url_map;
pub type UrlMap = url_map::UrlMap;

mod url_type;
pub type UrlType = url_type::UrlType;

mod static_map;
pub type StaticMap = static_map::StaticMap;

mod endpoint;
pub type Endpoint = endpoint::Endpoint;

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