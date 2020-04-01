use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Endpoint {
    pub path: String,
    pub method: String,
    pub groups: Vec<String>,
}

impl Endpoint {
    fn from_static(p: &'static str, m: &'static str, g: Vec<&'static str>) -> Endpoint {
        if g.len() == 0 {
            Endpoint {
                path: p.to_string(),
                method: m.to_string(),
                groups: Vec::new(),
            }
        } else {
            let mut groups: Vec<String> = Vec::with_capacity(g.len());
            for group in g {
                groups.push(group.to_string());
            }

            Endpoint {
                path: p.to_string(),
                method: m.to_string(),
                groups,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_static_returns_endpoint(){
        let e = Endpoint::from_static("/path", "GET", vec!["testers"]);

        assert_eq!(Endpoint {
            path: "/path".to_string(),
            method: "GET".to_string(),
            groups: vec!["testers".to_string()],
        }, e);
    }
}