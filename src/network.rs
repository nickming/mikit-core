use std::collections::HashMap;

use reqwest::{Client, Response, Url};

use crate::models::{AccountLoginResponse, AccountSignatureResponse};
use crate::utils::encrypt_with_md5;

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
        let json = self.parse_json_from_response(response).await?;
        Ok(serde_json::from_str(&json)?)
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
        signature: &AccountSignatureResponse,
    ) -> anyhow::Result<AccountLoginResponse> {
        let hash = encrypt_with_md5(password).to_uppercase();
        let mut params = HashMap::new();
        params.insert("qs", signature.qs.as_str());
        params.insert("sid", signature.sid.as_str());
        params.insert("_sign", signature.sign.as_str());
        params.insert("callback", signature.callback.as_str());
        params.insert("_json", "true");
        params.insert("user", username);
        params.insert("hash", &hash);
        let response = self
            .client
            .post("https://account.xiaomi.com/pass/serviceLoginAuth2")
            .form(&params)
            .send()
            .await?;
        let json = self.parse_json_from_response(response).await?;
        Ok(serde_json::from_str(&json)?)
    }

    async fn parse_json_from_response(&self, response: Response) -> anyhow::Result<String> {
        let body = response.text().await?;
        println!("body:{}", &body);
        Ok(String::from(&body[11..]))
    }
}

#[cfg(test)]
mod test {
    use super::HttpClient;

    #[tokio::test]
    async fn test_login() {
        let client = HttpClient::new();
        let signature = client.get_signature().await.unwrap();
        let login_response = client.login("xxx", "xxx", &signature).await.unwrap();
        println!("{:?}", login_response)
    }
}
