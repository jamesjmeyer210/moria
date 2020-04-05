use crate::url::sub_path::SubPath::Fixed;
use crate::url::sub_path::SubPath::Dynamic;

#[derive(Debug, PartialEq)]
pub enum SubPath {
    Fixed(String),
    Dynamic(String),
}

impl SubPath {
    pub fn len(&self) -> usize {
        self.to_string().len()
    }
}

impl ToString for SubPath {
    fn to_string(&self) -> String {
        match self {
            Fixed(s) => s.to_string(),
            Dynamic(s) => s.to_string(),
        }
    }
}