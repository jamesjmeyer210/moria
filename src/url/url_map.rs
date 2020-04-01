use regex::Regex;

use super::MetaData;
use crate::util::either::Either;
use crate::app;
use crate::util::{UniqueVec, UrlRegex};
use std::borrow::Borrow;
use crate::url::url_type::UrlType;
use std::str::FromStr;
use std::str::Split;
use crate::url::Domain;
use crate::url::convert::ConversionError;

#[derive(Debug, PartialEq)]
struct MetaDataRef {
    method: usize,
    origin: usize,
    groups: Vec<usize>,
}

type PathRef = Vec<(usize,usize)>;

struct UrlRef {
    path: PathRef,
    metadata: usize,
}

pub struct UrlMap {
    // These items to the next comment are built from init_origins_groups_methods_metadata(...)
    groups: Vec<String>,
    origins: Vec<String>,
    methods: Vec<String>,
    metadata: Vec<MetaDataRef>,
    // The below items are built via init_map_url(...)
    map: Vec<Vec<Either<String,UrlRegex>>>,
    urls: Vec<UrlRef>,
}

impl UrlMap {

    fn init_origins_groups_methods_metadata(domains: &Vec<Domain>)
        -> (Vec<String>, // origins
            Vec<String>, // groups
            Vec<String>, // methods
            Vec<MetaDataRef>) // metadata
    {
        let mut origins = UniqueVec::with_capacity(domains.len());
        let mut groups = UniqueVec::new();
        let mut methods = UniqueVec::new();
        let mut metadata = Vec::new();

        for domain in domains.iter() {
            // add only the unique origins to the variable `o`
            let o = origins.push(domain.origin.clone());

            for endpoint in domain.endpoints.iter() {
                // add only the unique groups and store their indexes to the sequence, `g`
                let mut g = Vec::with_capacity(endpoint.groups.len());
                for group in endpoint.groups.iter() {
                    g.push( groups.push(group.clone()));
                }
                // add only a method if it is unique
                let m = methods.push(endpoint.method.clone());
                metadata.push(MetaDataRef { method: m, origin: o, groups: g, });
            }
        }
        // TODO: trim these vecs before returning them to ensure optimal compression
        (origins.to_vec(), groups.to_vec(), methods.to_vec(), metadata)
    }

    fn add_path(map: &mut Vec<UniqueVec<Either<String,UrlRegex>>>
                , path: &str
                , static_path: &Regex
                , dynamic_path: &Regex) -> Result<PathRef, ConversionError>
    {
        let sub_paths: Vec<&str> = path.split("/").collect();
        let mut path_ref: PathRef = Vec::with_capacity(sub_paths.len());

        for i in 0..sub_paths.len() {
            if map.len() - 1 < i {
                map.push(UniqueVec::new());
            }

            // TODO: consolidate these two match blocks into a single block
            match static_path.captures(sub_paths.get(i).unwrap()) {
                Some(static_sub_path) => {
                    // TODO: looks like this code belongs in UrlRegex
                    let static_sub_path = static_sub_path.get(0)
                        .unwrap()
                        .as_str();

                    let j = map.get_mut(i)
                        .unwrap()
                        .push(Either::This(static_sub_path.to_string()));
                    path_ref.push((i, j));
                },
                None => match dynamic_path.captures(sub_paths.get(i).unwrap()) {
                    Some(dynamic_sub_path) => {
                        // TODO: looks like this code belongs in the UrlRegex
                        let dynamic_sub_path = dynamic_sub_path.get(0)
                            .unwrap()
                            .as_str();
                        let expr = UrlType::from_str(dynamic_sub_path)
                            .unwrap()
                            .get_regex_str();
                        let expr = Regex::from_str(expr).unwrap();

                        let j = map.get_mut(i)
                            .unwrap()
                            .push(Either::That(UrlRegex{ expr, }));
                        path_ref.push((i, j));
                    },
                    None => return Err(ConversionError::UnknownType),
                }
            }
        }

        Ok(path_ref)
    }

    fn init_map_url(domains: &Vec<Domain>) -> (Vec<Vec<Either<String,UrlRegex>>>, Vec<UrlRef>) {
        // TODO: instantiate the map and the urls within
        let mut map: Vec<UniqueVec<Either<String,UrlRegex>>> = Vec::new();
        let static_sub_path = Regex::new(r"[a-zA-Z0-9]").unwrap();
        let dynamic_sub_path = Regex::new(r"(\{string\}|\{integer\}|\{bool\}|\{real\})").unwrap();

        let mut count: (usize, usize) = (0, 0);

        for domain in domains.iter() {
            for endpoint in domain.endpoints.iter() {
                let mut i: usize = 0;
                for sub_path in endpoint.path.clone().split("/") {
                    // If we have iterated to a point that has not yet been reached, we'll add a
                    // new UniqueVec to our map
                    if map.len() - 1 < i {
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

                    i += 1;
                    count.1 += 1;
                }
                count.0 += 1;
            }
        }

        // TODO: instantiate these values
        let mut paths: Vec<Vec<Either<String,UrlRegex>>> = Vec::with_capacity(count.0);
        for i in 0..count.0 {
            paths.push(Vec::new());
        }

        let mut urls: Vec<UrlRef> = Vec::with_capacity(count.1);

        (paths, urls)
    }

    // TODO: break apart this function into sub-function and test them individually
    fn from_file(path: &str) -> Self {
        let domains = app::load_domains(path);

        let meta = UrlMap::init_origins_groups_methods_metadata(&domains);
        // TODO: instantiate the map and the urls within
        let map_and_url = UrlMap::init_map_url(&domains);

        UrlMap {
            groups: meta.0,
            origins: meta.1,
            methods: meta.2,
            metadata: meta.3,
            map: map_and_url.0,
            urls: map_and_url.1,
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
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::url::Endpoint;

    #[test]
    fn init_meta_returns_tuple_when_passed_proper_collection() {
        let domains: Vec<Domain> = vec![
            Domain::from_static("first-origin", vec![
                Endpoint::from_static("/foo", "GET", vec!["foos"]),
                Endpoint::from_static("/bar", "POST", vec!["bars"]),
            ]),
            Domain::from_static("second-origin", vec![
                Endpoint::from_static("/another-foo", "GET", vec!["foos"]),
                Endpoint::from_static("/another-bar", "POST", vec!["bars"]),
            ]),
        ];

        let meta = UrlMap::init_origins_groups_methods_metadata(&domains);

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

    // #[test]
    // fn add_path_returns_path_ref_of_ones_when_uninitialized() {
    //     let map: Vec<UniqueVec<Either<String,UrlRegex>>> = Vec::new();
    // }
}