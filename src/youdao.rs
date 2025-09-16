use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::str::FromStr;
use url::Url;
use md5::{Md5, Digest};
use hex;

use crate::youdao_translate::WordAllInfo;

#[derive(Debug, Deserialize)]
pub struct YoudaoTranslationResponse {
    #[serde(rename = "translation")]
    pub translations: Vec<String>,
    #[serde(rename = "query")]
    pub query: String,
    #[serde(rename = "errorCode")]
    pub error_code: String,
}

#[derive(Debug, Serialize)]
pub struct YoudaoTranslationRequest {
    pub q: String,
    pub from: String,
    pub to: String,
    pub appKey: String,
    pub salt: String,
    pub sign: String,
}

pub struct YoudaoTranslator {
    app_key: String,
    app_secret: String,
    client: reqwest::Client,
}

impl YoudaoTranslator {
    pub fn new() -> Self {
        let app_key = "Mk6hqtUp33DGGtoS63tTJbMUYjRrG1Lu".to_string();
        let app_secret = "your_app_secrewebdictt_here".to_string();
        Self {
            app_key,
            app_secret,
            client: reqwest::Client::new(),
        }
    }

    pub async fn translate(&self， text: &String) -> Result<WordAllInfo> {
        let client = reqwest::Client::builder().no_proxy().build()?;
        let w = "Mk6hqtUp33DGGtoS63tTJbMUYjRrG1Lu";
        let v = "webdict";
        let param_client = "web";
        let le = "en";
        let keyfrom = "webdict";
        let r = format!("{}{}", text, v);
        let time = (r.len() % 10).to_string();
    
        let mut hasher = Md5::new();
        hasher.update(r.as_bytes());
        let result = hasher.finalize();
        let o = hex::encode(result);
    
        let n = format!("{}{}{}{}{}", param_client, text, time, w, o);
        let mut hasher = Md5::new();
        hasher.update(n.as_bytes());
        let result = hasher.finalize();
        let f = hex::encode(result);
    
        let params = [
            ("q", text.as_str()),
            ("le", le),
            ("t", time.as_str()),
            ("client", param_client),
            ("sign", f.as_str()),
            ("keyfrom", keyfrom),
        ];
        client
            .post(Url::from_str(
                "https://dict.youdao.com/jsonapi_s?doctype=json&jsonversion=4",
            )?)
            .form(&params)
            .send()
            .await?
            .json()
            .await
            .map_err(|x| anyhow!("{} json fail: {}", text, x.to_string()))
    }

}
