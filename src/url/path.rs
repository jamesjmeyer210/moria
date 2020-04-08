use std::str::FromStr;
use std::string::ToString;
use crate::util::UniqueVec;

#[derive(PartialEq)]
enum SubPath {
    Fixed(String),
    Dynamic,
}

impl FromStr for SubPath {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "{}" => Ok(SubPath::Dynamic),
            _ => Ok(SubPath::Fixed(s.to_string())),
        }
    }
}

struct PathRef {
    fixed: bool,
    sub_paths: Vec<usize>
}

struct PathBuilder {
    paths: Vec<UniqueVec<SubPath>>
}

impl PathBuilder {
    fn new() -> Self {
        PathBuilder {
            paths: Vec::new(),
        }
    }

    fn build(&mut self, path: &str) -> PathRef {
        let sub_paths: Vec<&str> = path.split("/").collect();
        while sub_paths.len() < self.paths.len() {
            self.paths.push(UniqueVec::new());
        }

        let mut path_ref = PathRef {
            fixed: true,
            sub_paths: Vec::with_capacity(sub_paths.len())
        };

        for i in 0..sub_paths.len() {
            let sub_path = SubPath::from_str(sub_paths.get(i).unwrap()).unwrap();
            match sub_path {
                SubPath::Dynamic => path_ref.fixed = false,
                _ => ()
            }

            let j = self.paths.get_mut(i).unwrap().push(sub_path);
            path_ref.sub_paths.push(j);
        }

        path_ref
    }
}

#[cfg(test)]
mod test {

}