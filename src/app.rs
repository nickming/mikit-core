use std::{
    any,
    sync::{Arc, RwLock},
};

use anyhow::Ok;
use sled::Db;

use crate::network::HttpClient;

type Lock<T> = Arc<RwLock<Option<T>>>;

struct App {
    http_client: Arc<HttpClient>,
    db: Lock<Db>,
}

impl App {
    fn new() -> Self {
        App {
            http_client: Arc::new(HttpClient::default()),
            db: Arc::new(RwLock::new(None)),
        }
    }

    fn initialize(&mut self) -> anyhow::Result<()> {
        let sled = sled::open("mikit_db")?;
        let mut guard = self.db.write().unwrap();
        *guard = Some(sled);
        Ok(())
    }

    fn login(&self, username: &str, password: &str) {}

    fn logout(&mut self) -> anyhow::Result<()> {
        let guard = self.db.write().unwrap();
        if let Some(db) = guard.as_ref() {
            db.clear()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn feature() {}
}
