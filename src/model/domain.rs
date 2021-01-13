use super::Endpoint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Domain {
    pub(crate) origin: String,
    pub(crate) endpoints: Vec<Endpoint>,
}
