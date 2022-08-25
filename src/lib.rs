use std::any;

use anyhow::Ok;
use network::HttpClient;

mod app;
mod models;
mod network;
mod utils;

struct App {
    http_client: HttpClient,
}

impl App {
    fn new() -> Self {
        App {
            http_client: HttpClient::default(),
        }
    }

    fn initialize(&self) -> anyhow::Result<()> {
        let sled = sled::open("mikit_db")?;
        Ok(())
    }

    fn login(&self, username: &str, password: &str) {}

    fn logout(&self) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn feature() {}
}
