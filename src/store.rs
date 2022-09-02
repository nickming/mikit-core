use std::{path::Path, sync::Arc};

use anyhow::Ok;
use directories::ProjectDirs;
use serde::{de::DeserializeOwned, Serialize};
use sled::Db;

use crate::models::MikitError;

pub struct DataSore {
    db: Arc<Db>,
}

impl DataSore {
    pub(crate) fn new(application_name: &str, organization_name: &str) -> anyhow::Result<DataSore> {
        let dirs = ProjectDirs::from("", organization_name, application_name);
        let parent_dir = match dirs.as_ref() {
            Some(dirs) => dirs.data_dir(),
            None => Path::new("."),
        };
        let db_path = parent_dir.join("mikit_db");
        let sled = sled::open(db_path)?;
        Ok(Self { db: Arc::new(sled) })
    }

    pub fn set<T: Serialize>(&self, key: &str, data: &T) -> anyhow::Result<()> {
        let db = self.db.clone();
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
        let mut store = DataSore::new("mikit", "com.nickming.test").unwrap();
        store.set::<String>("test", &"test".to_string()).unwrap();
        assert_eq!(store.get::<String>("test").unwrap(), "test");
        store.clear().unwrap();
    }
}
