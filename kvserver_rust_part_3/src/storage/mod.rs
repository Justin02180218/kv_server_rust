pub mod mem_storage;

use std::error::Error;

use bytes::Bytes;

pub trait Storage {
    fn get(&self, key: &str) -> Result<Option<Bytes>, Box<dyn Error>>;
    fn set(&self, key: &str, value: Bytes) -> Result<Option<Bytes>, Box<dyn Error>>;
}