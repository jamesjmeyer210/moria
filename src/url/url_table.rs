use std::collections::HashMap;
use crate::model::AuthObj;
use crate::url::url::{Url, UrlBuilder};
use std::error::Error;

enum UrlTableError {
    KeysValuesMismatch(usize, usize),
}

pub struct UrlTable {
    static_table: HashMap<String,AuthObj>,
    // TODO: Add dynamic table
}

impl UrlTable {
    fn init_static(&mut self, keys: &Vec<&str>, values: &Vec<&AuthObj>) -> Result<(),UrlTableError> {
        if keys.len() != values.len() {
            Err(UrlTableError::KeysValuesMismatch(keys.len(),values.len()))
        }
        else {
            self.static_table = HashMap::with_capacity(keys.len());
            for i in 0..keys.len() {
                self.static_table.insert(
                    keys.get(i).unwrap().to_string(),
                    values.get(i).unwrap().to_owned().clone(),
                );
            }
            Ok(())
        }
    }

    fn static_lookup(&self, method: &str, path: &str,) -> Option<&AuthObj> {
        let key = format!("{}{}", method, path);
        self.static_table.get(&key)
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