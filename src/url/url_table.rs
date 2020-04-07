use std::collections::HashMap;
use crate::model::AuthObj;
use crate::url::url::{Url, UrlBuilder};
use std::error::Error;
use std::hash::Hash;

enum UrlTableError {
    LengthsNotEqual,
    KeysValuesMismatch(usize, usize),
}

pub struct UrlTable {
    urls: Vec<Url>,
    url_builder: UrlBuilder,
    static_table: HashMap<String,usize>,        // (method + path), url
    dynamic_table: Vec<(usize, usize, usize)>   // method, path, url
}

impl UrlTable {

    fn new() -> Self {
        UrlTable {
            urls: Vec::new(),
            url_builder: UrlBuilder::new(),
            static_table: HashMap::new(),
            dynamic_table: Vec::new(),
        }
    }

    fn from_lists(methods: &Vec<&str>, paths: &Vec<&str>, groups: &Vec<&Vec<&str>>, origins: &Vec<&str>) -> Result<Self,UrlTableError> {
        if (methods.len() != paths.len()) && (paths.len() != groups.len()) && (groups.len() != origins.len()) {
            Err(UrlTableError::LengthsNotEqual)
        }
        else {
            let mut urls = Vec::with_capacity(methods.len());
            let mut url_builder = UrlBuilder::new();
            for i in 0..methods.len() {
                urls.push(url_builder.build(
                    methods.get(i).unwrap(),
                    paths.get(i).unwrap(),
                    groups.get(i).unwrap(),
                    origins.get(i).unwrap())
                )
            }
            let mut static_table = HashMap::new();
            let mut dynamic_table = Vec::new();
            let mut i: usize = 0;
            for url in urls.iter() {
                match url.fixed {
                    true => {
                        let key = format!("{}{}",
                                          url_builder.get_method(url.method).unwrap(),
                                          url_builder.get_path(url.path).unwrap());
                        static_table.insert(key, i);
                    },
                    false => dynamic_table.push((url.method, url.path, i)),
                }
                i += 1;
            }

            Ok(UrlTable {
                urls,
                url_builder,
                static_table,
                dynamic_table,
            })
        }
    }

    // fn init_static(&mut self, keys: &Vec<&str>, values: &Vec<&Url>) -> Result<(),UrlTableError> {
    //     if keys.len() != values.len() {
    //         Err(UrlTableError::KeysValuesMismatch(keys.len(),values.len()))
    //     }
    //     else {
    //         for i in 0..keys.len() {
    //             self.urls.push(values.get(i).unwrap().unwrap().clone());
    //             self.static_table.insert(
    //                 keys.get(i).unwrap().to_string(),
    //                 0, // TODO: complete this insert
    //             );
    //         }
    //         Ok(())
    //     }
    // }

    fn static_lookup(&self, method: &str, path: &str,) -> Option<&usize> {
        let key = format!("{}{}", method, path);
        self.static_table.get(&key)
    }

    pub fn lookup(&self, method: &str, path: &str) -> Option<Url> {

        let try_static = self.static_lookup(method, path);
        if try_static.is_some() {
            Some(Url::from_parts(true,0, 0, vec![0], 0))
        }
        else if false {
            // TODO: try dynamic_lookup
            None
        }
        else {
            None
        }
    }
    // TODO:
    // Implement dynamic lookup to complement static lookup

    // fn init_dynamic(&self, keys: &Vec<str>, values: &Vec<&AuthObj>) -> Result<(),Error> {
    //     Ok(())
    // }

    // fn dynamic_lookup(&self, method: &str, path: &str) -> Option<Url> {
    //     unimplemented!()
    // }
}

#[cfg(test)]
mod test {

}