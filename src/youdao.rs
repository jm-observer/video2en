use anyhow::{anyhow, Result};
use std::str::FromStr;
use url::Url;

use crate::youdao_translate::WordAllInfo;


pub struct YoudaoTranslator;

impl YoudaoTranslator {


    pub async fn translate(&self, text: &String) -> Result<WordAllInfo> {
        let client = reqwest::Client::builder().no_proxy().build()?;
        let w = "Mk6hqtUp33DGGtoS63tTJbMUYjRrG1Lu";
        let v = "webdict";
        let param_client = "web";
        let le = "en";
        let keyfrom = "webdict";
        let r = format!("{}{}", text, v);
        let time = (r.len() % 10).to_string();
    
        let o = format!("{:x}", md5::compute(r.as_bytes()));
    
        let n = format!("{}{}{}{}{}", param_client, text, time, w, o);
        let f = format!("{:x}", md5::compute(n.as_bytes()));
    
        let params = [
            ("q", text.as_str()),
            ("le", le),
            ("t", time.as_str()),
            ("client", param_client),
            ("sign", f.as_str()),
            ("keyfrom", keyfrom),
        ];
        let response = client
            .post(Url::from_str(
                "https://dict.youdao.com/jsonapi_s?doctype=json&jsonversion=4",
            )?)
            .form(&params)
            .send()
            .await?;
            
        let response_text = response.text().await?;
        
        serde_json::from_str::<WordAllInfo>(&response_text)
            .map_err(|x| anyhow!("{} json fail: {}", text, x.to_string()))
    }

}
