use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MikitError {
    #[error("network error")]
    Network(#[from] reqwest::Error),
    #[error("unknown data store error:{0}")]
    Unknown(String),
    #[error("json parse error")]
    JsonParse(#[from] serde_json::Error),
    #[error("db operation error")]
    Store(#[from] sled::Error),
    #[error("unlogin eror")]
    UnLogin,
}

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
    pub nonce: u128,
    pub location: String,
    #[serde(alias = "userId")]
    pub user_id: u64,
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
pub struct DeviceListResult {
    pub list: Vec<Device>,
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
pub struct DevicePropertiesRequestParams {
    pub params: Vec<DeviceProperties>,
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

impl DeviceProperties {
    pub fn new_get_properties(did: &str, siid: usize, piid: usize) -> Self {
        Self {
            did: did.to_string(),
            siid: siid,
            piid: piid,
            value: None,
            code: None,
            action: None,
        }
    }

    pub fn new_set_properties(did: &str, siid: usize, piid: usize, value: Value) -> Self {
        Self {
            did: did.to_string(),
            siid: siid,
            piid: piid,
            value: Some(value),
            code: None,
            action: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub model: String,
    pub props: Vec<ModelProperty>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelProperty {
    pub name: String,
    pub siid: String,
    pub piid: String,
    pub value: Value,
}
