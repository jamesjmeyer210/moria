use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Endpoint {
    pub(crate) path: String,
    pub(crate) method: String,
    pub(crate) groups: Vec<String>,
}