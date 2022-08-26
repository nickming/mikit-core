use std::{
    any,
    sync::{
        atomic::{self, AtomicBool},
        Arc, RwLock,
    },
};

use anyhow::Ok;
use sled::Db;

use crate::network::HttpClient;

struct MiKit {
    http_client: Arc<HttpClient>,
    db: Option<Arc<Db>>,
    is_logged: AtomicBool,
}

impl MiKit {
    fn new() -> Self {
        MiKit {
            http_client: Arc::new(HttpClient::default()),
            db: None,
            is_logged: AtomicBool::new(false),
        }
    }

    fn initialize(&mut self) -> anyhow::Result<()> {
        let sled = sled::open("mikit_db")?;
        self.is_logged.store(true, atomic::Ordering::Relaxed);
        self.db = Some(Arc::new(sled));
        Ok(())
    }

    fn login(&self, username: &str, password: &str) {}

    fn logout(&mut self) -> anyhow::Result<()> {
        let db = self.db.as_ref().unwrap().clone();
        db.clear()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn feature() {}
}
