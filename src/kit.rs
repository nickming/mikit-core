use std::sync::{atomic::AtomicBool, Arc, RwLock};

use anyhow::Ok;

use crate::{models::MiAccount, network::HttpClient, store::DataSore};

pub struct MiKit {
    http_client: Arc<HttpClient>,
    db: Arc<DataSore>,
    account: Arc<RwLock<Option<MiAccount>>>,
    is_logged: AtomicBool,
}

impl MiKit {
    pub fn new() -> anyhow::Result<Self> {
        let db = DataSore::new()?;
        let account = db.get::<MiAccount>("account").ok();
        let is_logged = AtomicBool::new(account.is_some());
        Ok(MiKit {
            http_client: Arc::new(HttpClient::default()),
            db: Arc::new(db),
            account: Arc::new(RwLock::new(account)),
            is_logged,
        })
    }

    pub async fn login(&self, username: &str, password: &str) -> anyhow::Result<()> {
        let client = self.http_client.clone();
        let account = client.login(username, password).await?;

        let db = self.db.clone();
        db.set("account", &account)?;

        let mut guard = self.account.write().unwrap();
        *guard = Some(account.clone());

        Ok(())
    }

    pub fn logout(&mut self) -> anyhow::Result<()> {
        let mut account = self.account.write().unwrap();
        *account = None;
        self.db.clear()
    }

    pub fn get_account(&self) -> Option<MiAccount> {
        let account = self.account.clone();
        let account = account.read().unwrap();
        account.as_ref().and_then(|value| Some(value.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::MiKit;

    #[tokio::test]
    async fn feature() {
        let mikit = MiKit::new().unwrap();
        mikit.login("xxx", "xxx").await.unwrap();
        let account = mikit.get_account().unwrap();
        println!("{:?}", &account);
    }
}
