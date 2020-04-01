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

mod metadata;
pub type MetaData = metadata::MetaData;