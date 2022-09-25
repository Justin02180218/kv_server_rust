use std::path::Path;

use rocksdb::DB;

use crate::Storage;

#[derive(Debug)]
pub struct RocksDbStorage(DB);

impl RocksDbStorage {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(DB::open_default(path).unwrap())
    }
}

impl Storage for RocksDbStorage {
    fn get(&self, key: &str) -> Result<Option<bytes::Bytes>, Box<dyn std::error::Error>> {
        let v = self.0.get(key)?.unwrap();
        Ok(Some(v.into()))
    }

    fn set(
        &self,
        key: &str,
        value: bytes::Bytes,
    ) -> Result<Option<bytes::Bytes>, Box<dyn std::error::Error>> {
        self.0.put(key, value.clone()).unwrap();
        Ok(Some(value))
    }
}
