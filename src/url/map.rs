use regex::Regex;

use super::MetaData;
use crate::util::either::Either;
use crate::app;
use crate::util::{UniqueVec, UrlRegex};
use std::borrow::Borrow;
use crate::url::url_type::UrlType;
use std::str::FromStr;
use crate::url::Domain;

#[derive(Debug, PartialEq)]
struct MetaDataRef {
    method: usize,
    origin: usize,
    groups: Vec<usize>,
}

struct UrlRef {
    path: Vec<usize>,
    metadata: usize,
}

pub struct UrlMap {
    groups: Vec<String>,
    origins: Vec<String>,
    methods: Vec<String>,
    map: Vec<Vec<Either<String,UrlRegex>>>,
    metadata: Vec<MetaDataRef>,
    urls: Vec<UrlRef>,
}

impl UrlMap {

    fn init_meta(domains: &Vec<Domain>) -> (Vec<String>, Vec<String>, Vec<String>, Vec<MetaDataRef>) {
        let mut origins = UniqueVec::with_capacity(domains.len());
        let mut groups = UniqueVec::new();
        let mut methods = UniqueVec::new();
        let mut metadata = Vec::new();

        for domain in domains.iter() {
            // add only the unique origins
            let o = origins.push(domain.origin.clone());

            for endpoint in domain.endpoints.iter() {
                // add only the unique groups and store their indexes
                let mut g = Vec::with_capacity(endpoint.groups.len());
                for group in endpoint.groups.iter() {
                    g.push( groups.push(group.clone()));
                }
                // add only a method if it is unique
                let m = methods.push(endpoint.method.clone());

                metadata.push(MetaDataRef {
                    method: m,
                    origin: o,
                    groups: g,
                });
            }
        }
        // TODO: trim these vecs before returning them to ensure optimal compression
        (origins.to_vec(), groups.to_vec(), methods.to_vec(), metadata)
    }

    // TODO: break apart this function into sub-function and test them individually
    fn from_file(path: &str) -> Self {
        let domains = app::load_domains(path);

        let meta = UrlMap::init_meta(&domains);

        // TODO: instantiate the map and the urls within
        let mut map: Vec<UniqueVec<Either<String,UrlRegex>>> = Vec::new();
        let static_sub_path = Regex::new(r"[a-zA-Z0-9]").unwrap();
        let dynamic_sub_path = Regex::new(r"(\{string\}|\{integer\}|\{bool\}|\{real\})").unwrap();

        for domain in domains.iter() {
            for endpoint in domain.endpoints.iter() {
                let mut i: usize = 0;
                for sub_path in endpoint.path.clone().split("/") {
                    // If we have iterated to a point that has not yet been reached, we'll add a
                    // new UniqueVec to our map
                    if map.len() < i {
                        map.push(UniqueVec::new());
                    }

                    if static_sub_path.captures(sub_path).is_some() {
                        map.get_mut(i).unwrap().push(Either::This(sub_path.to_string()));
                    }
                    else if dynamic_sub_path.captures(sub_path).is_some() {
                        map.get_mut(i).unwrap().push(Either::That(UrlRegex {
                            expr: Regex::from_str(
                                UrlType::from_str(sub_path).unwrap().get_regex_str()
                            ).unwrap()
                        }));
                    }
                    else {
                        // TODO: get rid of this panic here, by passing an error back up the call stack
                        panic!("Illegal url sub-path: {}", sub_path);
                    }
                }
            }
        }

        UrlMap {
            groups: meta.0,
            origins: meta.1,
            methods: meta.2,
            map: Vec::new(), // TODO: convert the map to the proper type and store it here
            metadata: meta.3,
            urls: Vec::new(), // TODO: store the urls here
        }
    }

    // TODO: this algorithm needs to be changed to do the static pass first and the dynamic pass second
    fn find_in_vec(vec: &Vec<Either<String,Regex>>, target: &str) -> Option<usize> {
        let mut i: usize = 0;
        for either in vec {
            match either {
                // Either we can have a string of our url piece, like "/api" or "/users"
                Either::This(a) => {
                    if a == target {
                        i = i + 1;
                        return Some(i)
                    }
                },
                // Or we have a regex because it is an optional type
                Either::That(b) => {
                    if b.captures(target).is_some() {
                        i = i + 1;
                        return Some(i)
                    }
                },
                _ => ()
            }
        }
        None
    }

    // TODO: re-implement this method with the finalized structures if needed
    // fn get(&self, target: (usize,usize)) -> Either<String,Regex> {
    //     let index_of: Either<String,Regex> = match self.map.get(target.0) {
    //         Either::This(a) => Either::This(a),
    //         _ => Either::None,
    //     };
    //     Either::None
    // }

    // TODO: re-implement this method with the finalized structures if needed
    // fn find(&self, taget: Vec<&str>) -> Vec<(usize,usize)> {
    //     let mut x =  0;
    //     let mut at: Vec<(usize,usize)> = Vec::new();
    //
    //     for iter in self.map.iter() {
    //         match UrlMap::find_in_vec(i, taget.get(i).unwrap()) {
    //             Some(y) => at.push((x,y)),
    //             _ => (),
    //         }
    //     }
    //     at
    // }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::url::Endpoint;

    #[test]
    fn init_meta_returns_tuple_when_passed_proper_collection() {
        let domains = vec![
            Domain {
                origin: "first-origin".to_string(),
                endpoints: vec![
                    Endpoint {
                        path: "/foo".to_string(),
                        method: "GET".to_string(),
                        groups: vec!["foos".to_string()]
                    },
                    Endpoint {
                        path: "/bar".to_string(),
                        method: "POST".to_string(),
                        groups: vec!["bars".to_string()]
                    }
                ]
            },
            Domain {
                origin: "second-origin".to_string(),
                endpoints: vec![
                    Endpoint {
                        path: "/another-foo".to_string(),
                        method: "GET".to_string(),
                        groups: vec!["foos".to_string()]
                    },
                    Endpoint {
                        path: "/another-bar".to_string(),
                        method: "POST".to_string(),
                        groups: vec!["bars".to_string()]
                    }
                ]
            }
        ];

        let meta = UrlMap::init_meta(&domains);

        let expected_orgins = vec![
            "first-origin".to_string(),
            "second-origin".to_string()
        ];
        assert_eq!(expected_orgins, meta.0);

        let expected_groups = vec![
            "foos".to_string(),
            "bars".to_string(),
        ];
        assert_eq!(expected_groups, meta.1);

        let expected_methods = vec![
            "GET".to_string(),
            "POST".to_string(),
        ];
        assert_eq!(expected_methods, meta.2);

        let expected_metadata = vec![
            MetaDataRef { method: 0, origin: 0, groups: vec![0] },
            MetaDataRef { method: 1, origin: 0, groups: vec![1] },
            MetaDataRef { method: 0, origin: 1, groups: vec![0] },
            MetaDataRef { method: 1, origin: 1, groups: vec![1] },
        ];
        assert_eq!(expected_metadata, meta.3);
    }
}