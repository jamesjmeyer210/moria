use crate::url::path::Path;
use std::str::FromStr;
use crate::util::UniqueVec;

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
    methods: UniqueVec<String>,
    paths: UniqueVec<String>,
    origins: UniqueVec<String>,
    groups: UniqueVec<String>,
}
// TODO: build using the unique vec
impl UrlBuilder {
    pub fn new() -> Self {
        UrlBuilder {
            methods: UniqueVec::new(),
            paths: UniqueVec::new(),
            origins: UniqueVec::new(),
            groups: UniqueVec::new(),
        }
    }

    pub fn path_is_fixed(path: &str) -> bool {
        let sub_paths: Vec<&str> = path.split("/").collect();
        for sub_path in sub_paths.iter() {
            match *sub_path {
                "{}" => return false,
                _ => ()
            }
        }
        true
    }

    pub fn build(&mut self, method: &str, path: &str, groups: Vec<&str>, origin: &str) -> Url {

        let mut g = Vec::with_capacity(groups.len());
        for group in groups.iter() {
            g.push(self.groups.push(group.to_string()));
        }

        Url {
            fixed: UrlBuilder::path_is_fixed(path),
            method: self.methods.push(method.to_string()),
            path: self.paths.push(path.to_string()),
            groups: g,
            origin: self.origins.push(origin.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::url::url::UrlBuilder;

    #[test]
    fn path_is_fixed_returns_true_when_fixed() {
        let path = "/api/status";
        assert_eq!(true, UrlBuilder::path_is_fixed(path));
    }

    #[test]
    fn path_is_fixed_returns_false_when_dynamic(){
        let path = "/api/user/{}";
        assert_eq!(false, UrlBuilder::path_is_fixed(path));
    }

    #[test]
    fn build_creates_accessible_urls(){
        let mut ub = UrlBuilder::new();
        let a = ub.build("POST", "/api/add-user", vec!["users","admins"], "website.com");

        assert_eq!(0, a.method);
        assert_eq!(0, a.path);
        assert_eq!(vec![0, 1], a.groups);
        assert_eq!(0, a.origin);

        let b = ub.build("GET", "/api/user", vec!["users","admins","vendors"], "website.com");
        assert_eq!(1, b.method);
        assert_eq!(1, b.path);
        assert_eq!(vec![0, 1, 2], b.groups);
        assert_eq!(0, b.origin);
    }
}