use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait Request {
    async fn send(&self) -> anyhow::Result<String>;
}

pub struct StandardRequest {
    method: reqwest::Method,
    url: String,
    headers: HashMap<String, String>,
    body: String,
}

#[async_trait]
impl Request for StandardRequest {
    async fn send(&self) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        println!("Send standard request to {}", self.url);
        let resp = client.get(&self.url).send().await?;
        println!("{resp:#?}");
        Ok("200".to_string())
    }
}

impl StandardRequest {
    pub fn new(
        method: reqwest::Method,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> Self {
        Self {
            method,
            url,
            headers,
            body,
        }
    }
}
