use std::collections::HashMap;
use crate::model::AuthObj;
use crate::url::url::{Url, UrlBuilder};
use std::error::Error;

enum UrlTableError {
    KeysValuesMismatch(usize, usize),
}

pub struct UrlTable {
    urls: Vec<Url>,
    static_table: HashMap<String,usize>,        // (method + path), url
    dynamic_table: Vec<(usize, usize, usize)>   // method, path, url
}

impl UrlTable {

    fn new() -> Self {
        UrlTable {
            urls: Vec::new(),
            static_table: HashMap::new(),
            dynamic_table: Vec::new(),
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