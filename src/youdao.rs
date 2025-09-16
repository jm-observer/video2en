use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

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
    pub fn new(app_key: String, app_secret: String) -> Self {
        Self {
            app_key,
            app_secret,
            client: reqwest::Client::new(),
        }
    }

    pub async fn translate(&self, text: &str) -> Result<String> {
        // 生成随机盐值
        let salt = format!("{}", SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs());
        
        // 生成签名: MD5(appKey + q + salt + appSecret)
        let sign_string = format!("{}{}{}{}", self.app_key, text, salt, self.app_secret);
        let sign = format!("{:x}", md5::compute(sign_string.as_bytes()));

        let request_body = YoudaoTranslationRequest {
            q: text.to_string(),
            from: "en".to_string(),
            to: "zh-CHS".to_string(),
            appKey: self.app_key.clone(),
            salt,
            sign,
        };

        let response = self.client
            .post("https://openapi.youdao.com/api")
            .form(&request_body)
            .send()
            .await
            .context("Failed to send translation request")?;

        if !response.status().is_success() {
            return Err(anyhow!("Youdao API returned error: {}", response.status()));
        }

        let translation_response: YoudaoTranslationResponse = response
            .json()
            .await
            .context("Failed to parse translation response")?;

        if translation_response.error_code != "0" {
            return Err(anyhow!("Youdao API error: {}", translation_response.error_code));
        }

        if let Some(translation) = translation_response.translations.first() {
            Ok(translation.clone())
        } else {
            Err(anyhow!("No translation found in response"))
        }
    }

    pub async fn translate_batch(&self, texts: &[String]) -> Result<Vec<String>> {
        let mut results = Vec::new();
        
        for (i, text) in texts.iter().enumerate() {
            print!("\r🔄 翻译进度: {}/{}", i + 1, texts.len());
            std::io::Write::flush(&mut std::io::stdout()).ok();
            
            match self.translate(text).await {
                Ok(translation) => results.push(translation),
                Err(e) => {
                    println!("\n⚠️ 翻译失败: {} - {}", text, e);
                    results.push("翻译失败".to_string());
                }
            }
            
            // 添加小延迟避免API限制
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        println!("\n✅ 翻译完成!");
        Ok(results)
    }
}
