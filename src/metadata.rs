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