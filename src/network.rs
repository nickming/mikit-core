use std::any;
use std::collections::HashMap;

use anyhow::{anyhow, Ok};
use log::{log, trace};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, Client, Response, Url};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::to_string;

use crate::models::{
    AccountLoginResponse, AccountSignatureResponse, Device, DeviceProperties,
    FetchDevicePropertiesRequestParams, MiAccount, MikitError,
};
use crate::utils::{
    encode_to_base64, encrypt_with_md5, encrypt_with_sha1, generate_command_signature,
    generate_nonce, generate_signed_nonce, get_random_string,
};

static BASE_UA: &str = "APP/com.xiaomi.mihome APPV/6.0.103 iosPassportSDK/3.9.0 iOS/14.4 miHSTS";
static COMMAND_API: &str = "https://api.io.mi.com/app";
static SIGNATURE_API: &str = "https://account.xiaomi.com/pass/serviceLogin";
static LOGIN_API: &str = "https://account.xiaomi.com/pass/serviceLoginAuth2";

pub struct HttpClient {
    client: Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        let client = reqwest::ClientBuilder::new()
            .user_agent(BASE_UA)
            .build()
            .unwrap();
        Self { client }
    }
}

impl HttpClient {
    pub async fn login(&self, username: &str, password: &str) -> anyhow::Result<MiAccount> {
        let signature = self.fetch_signature().await?;
        let login_resp = self
            .fetch_login_response(username, password, &signature)
            .await?;
        self.fetch_auth_device_info(&login_resp).await
    }

    pub async fn execute_command<T: DeserializeOwned>(
        &self,
        command: CommandReqeust,
        account: &MiAccount,
    ) -> anyhow::Result<T> {
        let uri = command.get_uri();
        let data = command.get_data()?;
        let response_text = self
            .execute_command_uri_and_data(&uri, &data, account)
            .await?;
        Ok(self.parser_json::<T>(&response_text)?)
    }

    async fn fetch_signature(&self) -> anyhow::Result<AccountSignatureResponse> {
        let url = Url::parse_with_params(SIGNATURE_API, &[("sid", "xiaomiio"), ("_json", "true")])?;
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
        let response = self.client.post(LOGIN_API).form(&params).send().await?;
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
            urlencoding::encode(encode_to_base64(&encrypt_with_sha1(&nonce)).as_str())
        );
        let response = self.client.get(url).send().await?;
        let cookies = self.parse_cookies(response.headers());
        if cookies.is_empty() {
            return Err(
                MikitError::Network("can not find cookies in auth device api".to_string()).into(),
            );
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

    async fn execute_command_uri_and_data(
        &self,
        uri: &str,
        data: &str,
        account: &MiAccount,
    ) -> anyhow::Result<String> {
        let nonce = generate_nonce();
        let signed_nonce = generate_signed_nonce(&account.security_token, &nonce);
        let signature = generate_command_signature(uri, &signed_nonce, &nonce, data);
        let url = format!("{}{}", COMMAND_API, uri);
        let cookie = format!(
            "PassportDeviceId={};userId={};serviceToken={};",
            account.device_id.as_str(),
            account.user_id.as_str(),
            account.service_token
        );
        let mut headers = HeaderMap::new();
        headers.insert("Cookie", HeaderValue::from_str(&cookie)?);
        headers.insert(
            "x-xiaomi-protocal-flag-cli",
            HeaderValue::from_str("PROTOCAL-HTTP2")?,
        );
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("_nonce", &nonce);
        params.insert("data", data);
        params.insert("signature", &signature);
        Ok(self
            .client
            .post(url)
            .form(&params)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?)
    }

    fn parse_cookies(&self, header_map: &HeaderMap) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        let all_cookie_value = header_map.get_all("set-cookie");
        for value in all_cookie_value {
            if value.is_empty() {
                trace!("can not parse headers cause empty cookies");
                continue;
            }
            String::from_utf8(value.as_bytes().to_vec())
                .unwrap_or("".to_string())
                .trim()
                .split(";")
                .filter(|x| x.contains("="))
                .map(|x| x.split_once("=").unwrap_or((&"", &"")))
                .for_each(|x| {
                    result.insert(x.0.to_string(), x.1.to_string());
                });
        }
        result
    }

    fn parser_json<T: DeserializeOwned>(&self, json: &str) -> anyhow::Result<T> {
        serde_json::from_str(json).map_err(|e| e.into())
    }
}

pub enum CommandReqeust {
    DeviceList,
    DeviceProperties(FetchDevicePropertiesRequestParams),
}

impl CommandReqeust {
    fn get_data(&self) -> anyhow::Result<String> {
        match self {
            CommandReqeust::DeviceList => Ok(r#"{
                    "getVirtualModel":false,
                    "getHuamiDevices":0
                }"#
            .to_string()),
            CommandReqeust::DeviceProperties(params) => {
                serde_json::to_string_pretty(params).map_err(|e| e.into())
            }
        }
    }

    fn get_uri(&self) -> String {
        match self {
            CommandReqeust::DeviceList => "/home/device_list".to_string(),
            CommandReqeust::DeviceProperties(_) => "/miotspec/prop/get".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        models::{CommandResponse, Device, DeviceListResult},
        network::CommandReqeust,
    };

    use super::HttpClient;

    #[tokio::test]
    async fn test_network() {
        let client = HttpClient::default();
        let account = client.login("xxx", "xxx").await.unwrap();
        println!("{:?}", &account);
        let response = client
            .execute_command::<CommandResponse<DeviceListResult>>(
                CommandReqeust::DeviceList,
                &account,
            )
            .await
            .unwrap();
        println!("{:?}", &response);
    }
}
