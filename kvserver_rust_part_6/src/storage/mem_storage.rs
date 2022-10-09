use bytes::Bytes;
use dashmap::DashMap;

use crate::Storage;

#[derive(Clone, Debug, Default)]
pub struct MemStorage {
    map: DashMap<String, Bytes>,
}

impl MemStorage {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl Storage for MemStorage {
    fn get(&self, key: &str) -> Result<Option<Bytes>, Box<dyn std::error::Error>> {
        Ok(self.map.get(key).map(|v| v.value().clone()))
    }

    fn set(&self, key: &str, value: Bytes) -> Result<Option<Bytes>, Box<dyn std::error::Error>> {
        self.map.insert(key.to_string(), value.clone());
        Ok(Some(value))
    }
}
