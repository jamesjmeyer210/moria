use serde::{Serialize, Deserialize};
use crate::url::Endpoint;

#[derive(Serialize, Deserialize)]
pub struct Domain {
    pub origin: String,
    pub endpoints: Vec<Endpoint>,
}

impl Domain {
    fn from_static(o: &'static str, e: Vec<Endpoint>) -> Self {
        Domain {
            origin: o.to_string(),
            endpoints: e,
        }
    }
}