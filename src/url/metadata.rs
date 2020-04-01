#[derive(Debug, PartialEq, Clone)]
pub struct MetaData {
    pub origin: String,
    pub groups: Vec<String>,
}

impl MetaData {
    pub fn from_strings(o: String, g: Vec<String>) -> Self {
        MetaData {
            origin: o,
            groups: g,
        }
    }

    fn from_static(o: &'static str, g: Vec<&'static str>) -> Self {
        let mut copy = Vec::with_capacity(g.len());
        for group in g {
            copy.push(group.to_string());
        }

        MetaData {
            origin: o.to_string(),
            groups: copy,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::url::metadata::MetaData;

    #[test]
    fn from_static_returns_metatdata() {
        let m = MetaData::from_static("localhost", vec!["locals","hosts"]);

        assert_eq!(MetaData {
            origin: "localhost".to_string(),
            groups: vec!["locals".to_string(),"hosts".to_string()],
        }, m);
    }

}