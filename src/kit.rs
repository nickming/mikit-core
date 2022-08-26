use std::{
    any,
    sync::{
        atomic::{self, AtomicBool},
        Arc, RwLock,
    },
};

use anyhow::Ok;
use sled::Db;

use crate::{
    models::{MiAccount, MikitError},
    network::HttpClient,
};

struct MiKit {
    http_client: Arc<HttpClient>,
    db: Option<Arc<Db>>,
    account: Arc<RwLock<Option<MiAccount>>>,
    is_logged: AtomicBool,
}

impl MiKit {
    pub fn new() -> Self {
        MiKit {
            http_client: Arc::new(HttpClient::default()),
            db: None,
            account: Arc::new(RwLock::new(None)),
            is_logged: AtomicBool::new(false),
        }
    }

    pub fn initialize(&mut self) -> anyhow::Result<()> {
        let sled = sled::open("mikit_db")?;
        if let Some(vec) = sled.get("account")? {
            let account = rmp_serde::from_slice::<MiAccount>(&vec)?;
            let mut guard = self.account.write().unwrap();
            *guard = Some(account);
            self.is_logged.store(true, atomic::Ordering::Relaxed);
        }
        self.db = Some(Arc::new(sled));
        Ok(())
    }

    pub async fn login(&self, username: &str, password: &str) -> anyhow::Result<()> {
        let client = self.http_client.clone();
        let account = client.login(username, password).await?;
        let mut guard = self.account.write().unwrap();
        *guard = Some(account);
        Ok(())
    }

    pub fn logout(&mut self) -> anyhow::Result<()> {
        let db = self.db.as_ref().unwrap().clone();
        db.clear()?;
        Ok(())
    }

    async fn update_account(&self, account: MiAccount) -> anyhow::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn feature() {}
}
