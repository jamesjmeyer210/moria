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