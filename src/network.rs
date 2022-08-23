use std::sync::Arc;

use anyhow::anyhow;
use reqwest::{Client, Url};

use crate::models::AccountSignatureResponse;

static BASE_UA: &str = "APP/com.xiaomi.mihome APPV/6.0.103 iosPassportSDK/3.9.0 iOS/14.4 miHSTS";

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = reqwest::ClientBuilder::new()
            .user_agent(BASE_UA)
            .build()
            .unwrap();
        HttpClient { client }
    }

    pub async fn get_signature(&self) -> anyhow::Result<AccountSignatureResponse> {
        let url = Url::parse_with_params(
            "https://account.xiaomi.com/pass/serviceLogin",
            &[("sid", "xiaomiio"), ("_json", "true")],
        )?;
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        Ok(serde_json::from_str::<AccountSignatureResponse>(
            &body[11..],
        )?)
    }
}

#[cfg(test)]
mod test {
    use super::HttpClient;

    #[tokio::test]
    async fn test_sigature() {
        let client = HttpClient::new();
        let result = client.get_signature().await.unwrap();
        println!("{:?}", result)
    }
}
