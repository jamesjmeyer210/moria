use crate::url::path::Path;
use std::str::FromStr;

pub struct Url<'a> {
    method: &'a str,
    path: &'a str,
    groups: Vec<&'a str>,
    origin: &'a str,
}
// The UrlBuilder is a singleton that holds on to each unique origin and group
pub struct UrlBuilder {
    methods: Vec<String>,   // only unique methods
    paths: Vec<String>,     // only unique paths
    origins: Vec<String>,   // only unique origins
    groups: Vec<String>,    // only unique groups
}

impl UrlBuilder {
    fn new() -> Self {
        UrlBuilder {
            methods: Vec::new(),
            paths: Vec::new(),
            origins: Vec::new(),
            groups: Vec::new(),
        }
    }

    fn build(&mut self, method: &str, path: &str, groups: Vec<&str>, origin: &str) -> Url {
        self.methods.push(method.to_string());
        self.paths.push(path.to_string());
        self.origins.push(origin.to_string());
        for group in groups {
            self.groups.push(group.to_string());
        }

        let mut groups: Vec<&str> = Vec::with_capacity(self.groups.len());
        for group in self.groups.iter() {
            groups.push(group)
        }

        Url {
            method: self.methods.last().unwrap(),
            path: self.paths.last().unwrap(),
            groups,
            origin: self.origins.last().unwrap(),
        }
    }
}