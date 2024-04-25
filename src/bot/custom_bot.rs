use serde::Serialize;
use log::debug;

use crate::bot::LarkBot;
use crate::message::MessageTrait;

/// 自定义机器人
///
/// [使用指南](https://open.feishu.cn/document/client-docs/bot-v3/add-custom-bot)
pub struct CustomBot {
    /// webhook 地址
    webhook_url: String,
    /// 密钥
    secret: Option<String>,
    client: reqwest::Client,
}

impl CustomBot {
    pub fn new(webhook_url: String, secret: Option<String>) -> Self {
        CustomBot {
            webhook_url,
            secret,
            client: reqwest::Client::new(),
        }
    }
}

impl LarkBot for CustomBot {
    async fn send_raw_message(&self, body: impl Serialize + Send) {
        self.client
            .post(&self.webhook_url)
            .json(&body)
            .send()
            .await
            .unwrap();
    }

    async fn send_message(&self, message: impl MessageTrait + Send) {
        let body = serde_json::json!({
           "msg_type": message.message_type(),
            "content": message
        });

        debug!("{}", serde_json::to_string_pretty(&body).unwrap());

        self.client
            .post(&self.webhook_url)
            .json(&body)
            .send()
            .await
            .unwrap();
    }
}
