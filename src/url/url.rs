use crate::url::path::Path;
use std::str::FromStr;

pub struct Url {
    fixed: bool,
    method: usize,
    path: usize,
    groups: Vec<usize>,
    origin: usize,
}

impl Url {
    pub fn from_parts(f: bool, m: usize, p: usize, g: Vec<usize>, o: usize) -> Self {
        Url {
            fixed: f,
            method: m,
            path: p,
            groups: g,
            origin: o,
        }
    }
}
// The UrlBuilder is a singleton that holds on to each unique origin and group
pub struct UrlBuilder {
    methods: Vec<String>,   // only unique methods
    paths: Vec<String>,     // only unique paths
    origins: Vec<String>,   // only unique origins
    groups: Vec<String>,    // only unique groups
}
// TODO: build using the unique vec
impl UrlBuilder {
    pub fn new() -> Self {
        UrlBuilder {
            methods: Vec::new(),
            paths: Vec::new(),
            origins: Vec::new(),
            groups: Vec::new(),
        }
    }

    pub fn build(&mut self, method: &str, path: &str, groups: Vec<&str>, origin: &str) -> Url {
        self.methods.push(method.to_string());
        self.paths.push(path.to_string());
        self.origins.push(origin.to_string());
        for group in groups {
            self.groups.push(group.to_string());
        }

        let mut groups: Vec<String> = Vec::with_capacity(self.groups.len());
        for group in self.groups.iter() {
            groups.push(group.to_string())
        }

        Url {
            fixed: false,
            method: 0,
            path: 0,
            groups: vec![0],
            origin: 0,
        }
    }
}