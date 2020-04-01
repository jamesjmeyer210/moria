use std::fs;
use crate::url::Domain;

macro_rules! load_json_file {
    ($path:expr) => (
        serde_json::from_str(
            fs::read_to_string($path).unwrap_or_else(|error|{
                panic!("Error reading in file {}\n{}", $path, error)
            })
            .as_str()
        ).unwrap_or_else(|error|{
            panic!("Error converting file {}\n{}", $path, error);
        })
    )
}

pub fn load_domains(path: &str) -> Vec<Domain> {
    load_json_file!(path)
}

pub mod service;

mod config;
pub type Config = config::Config;
pub type ConfigError = config::ConfigError;