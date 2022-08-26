use std::sync::Arc;

use anyhow::Ok;
use serde::{de::DeserializeOwned, Serialize};
use sled::Db;

use crate::models::MikitError;

pub struct DataSore {
    db: Arc<Db>,
}

impl DataSore {
    pub(crate) fn new() -> anyhow::Result<DataSore> {
        let sled = sled::open("mikit_db")?;
        Ok(Self { db: Arc::new(sled) })
    }

    pub fn set<T: Serialize>(&self, key: &str, data: &T) -> anyhow::Result<()> {
        let db = self.db.clone();
        // let json = serde_json::to_string_pretty(data)?;
        // db.insert(key, json.as_bytes())?;
        let mut serializer = rmp_serde::Serializer::new(Vec::new()).with_struct_map();
        data.serialize(&mut serializer)?;
        db.insert(key, serializer.into_inner())?;
        Ok(())
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> anyhow::Result<T> {
        let db = self.db.clone();
        let bytes = db
            .get(key)?
            .ok_or(MikitError::Unknown("none value!".to_string()))?;
        let value = rmp_serde::from_slice::<T>(&bytes)?;
        Ok(value)
    }

    pub fn clear(&self) -> anyhow::Result<()> {
        let db = self.db.clone();
        db.clear().map_err(|e| e.into())
    }
}

#[cfg(test)]
mod test {
    use super::DataSore;

    #[test]
    fn test() {
        let mut store = DataSore::new().unwrap();
        store.set::<String>("test", &"test".to_string()).unwrap();
        assert_eq!(store.get::<String>("test").unwrap(), "test");
        store.clear().unwrap();
    }
}
