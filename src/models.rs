use serde::{
    de::{value, DeserializeOwned},
    Deserialize, Serialize,
};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiAccount {
    pub user_id: String,
    pub security_token: String,
    pub device_id: String,
    pub service_token: String,
    pub cookies: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountSignatureResponse {
    pub qs: String,
    #[serde(alias = "_sign")]
    pub sign: String,
    pub sid: String,
    pub callback: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountLoginResponse {
    pub code: u8,
    pub desc: String,
    pub nonce: String,
    pub location: String,
    #[serde(alias = "userId")]
    pub user_id: String,
    pub ssecurity: String,
}
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub enum CommandResponse<T> {
//     OK { code: String, result: T },
//     Error { code: String, message: String },
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandResponse<T> {
    pub code: usize,
    pub message: String,
    pub result: Option<T>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub did: String,
    pub token: String,
    #[serde(alias = "isOnline")]
    pub is_online: bool,
    pub model: String,
    pub localip: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FetchDevicePropertiesRequestParams {
    pub params: Vec<Device>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceProperties {
    pub did: String,
    pub siid: usize,
    pub piid: usize,
    pub value: Option<Value>,
    pub code: Option<usize>,
    #[serde(alias = "in")]
    pub action: Option<Value>,
}
