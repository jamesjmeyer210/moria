use serde::{Deserialize, Serialize};
use super::Endpoint;

#[derive(Serialize, Deserialize)]
pub struct Domain {
    pub(crate) origin: String,
    pub(crate) endpoints: Vec<Endpoint>,
}