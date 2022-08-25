use core::num;
use std::collections::HashMap;
use std::f32::consts::E;

use anyhow::{anyhow, Ok};
use base64::encode;
use log::{log, trace};
use reqwest::header::HeaderMap;
use reqwest::{header, Client, Response, Url};

use crate::models::{AccountLoginResponse, AccountSignatureResponse, MiAccount, MikitError};
use crate::utils::{encode_to_base64, encrypt_with_md5, encrypt_with_sha1, get_random_string};

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

    pub async fn login(&self, username: &str, password: &str) -> anyhow::Result<MiAccount> {
        let signature = self.fetch_signature().await?;
        let login_resp = self
            .fetch_login_response(username, password, &signature)
            .await?;
        self.fetch_auth_device_info(&login_resp).await
    }

    async fn fetch_signature(&self) -> anyhow::Result<AccountSignatureResponse> {
        let url = Url::parse_with_params(
            "https://account.xiaomi.com/pass/serviceLogin",
            &[("sid", "xiaomiio"), ("_json", "true")],
        )?;
        let response = self.client.get(url).send().await?;
        let json = self.parse_json_from_response(response).await?;
        Ok(serde_json::from_str(&json)?)
    }

    async fn fetch_login_response(
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

    async fn fetch_auth_device_info(
        &self,
        login_resp: &AccountLoginResponse,
    ) -> anyhow::Result<MiAccount> {
        let nonce = format!("nonce={}&{}", login_resp.nonce, login_resp.ssecurity);
        let url = format!(
            "{}&clientSign={}",
            login_resp.location,
            encode(encode_to_base64(&encrypt_with_sha1(&nonce)))
        );
        println!("url:{}", url);
        let response = self.client.get(url).send().await?;
        let cookies = self.parse_cookies(response.headers());
        if cookies.is_empty() {
            return Err(MikitError::NetworkError(
                "can not find cookies in auth device api".to_string(),
            )
            .into());
        }
        Ok(MiAccount {
            user_id: login_resp.user_id.to_string(),
            security_token: login_resp.ssecurity.to_string(),
            device_id: get_random_string(16),
            service_token: cookies
                .get("serviceToken")
                .unwrap_or(&"".to_string())
                .to_string(),
            cookies: cookies,
        })
    }

    async fn parse_json_from_response(&self, response: Response) -> anyhow::Result<String> {
        let body = response.text().await?;
        trace!("network response text:{}", &body);
        Ok(String::from(&body[11..]))
    }

    fn parse_cookies(&self, header_map: &HeaderMap) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        println!("{:?}", header_map);
        if let Some(value) = header_map.get("Set-Cookie") {
            if value.is_empty() {
                trace!("can not parse headers cause empty cookies");
                return result;
            }
            String::from_utf8(value.as_bytes().to_vec())
                .unwrap_or("".to_string())
                .trim()
                .split(";")
                .filter(|x| x.contains("="))
                .map(|x| x.split("=").collect())
                .filter(|x: &Vec<&str>| x.len() == 2)
                .for_each(|x| {
                    result.insert(x[0].to_string(), x[1].to_string());
                });
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::HttpClient;

    #[tokio::test]
    async fn test_login() {
        let client = HttpClient::new();
        let login_response = client.login("xxx", "xxx").await.unwrap();
        println!("{:?}", login_response)
    }
}
