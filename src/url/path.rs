//use regex::Regex;
use crate::url::sub_path::SubPath;
use std::str::FromStr;
use std::string::ToString;
use actix_web::dev::ResourcePath;

pub struct Path {
    dynamic: bool,
    sub_paths: Vec<SubPath>
}

impl Path {
    fn init_sub_paths(path: &str) -> Vec<SubPath> {
        let sp: Vec<&str> = path.split('/').collect();
        let mut sub_paths = Vec::with_capacity(sp.len());
        for sub_path in sp {
            // TODO: add dynamic paths
            sub_paths.push(SubPath::Fixed(sub_path.to_string()));
        }
        sub_paths
    }
}

impl FromStr for Path {
    type Err = ();

    fn from_str(p: &str) -> Result<Self, Self::Err> {
        Ok(Path {
            dynamic: false,
            sub_paths: Path::init_sub_paths(p),
        })
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        let mut sb = Vec::with_capacity(
            self.sub_paths.len() * (
                (self.sub_paths.first().unwrap().len() +
                    self.sub_paths.last().unwrap().len()) / 2)
        );
        for s in self.sub_paths.iter() {
            sb.push("/".to_string());
            sb.push(s.to_string());
        }
        "".to_string()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn from_str_creates_path_with_sub_paths() {
        let path = Path::from_str("/api/status").unwrap();

        assert_eq!(&vec![
            SubPath::Fixed("".to_string()),
            SubPath::Fixed("api".to_string()),
            SubPath::Fixed("status".to_string())], &path.sub_paths);
    }
}