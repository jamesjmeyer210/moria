use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JwtPayload {
    pub(crate) exp: i64,
    pub(crate) groups: Vec<String>,
}
