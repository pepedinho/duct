use async_trait::async_trait;
use duct_logger::log;
use reqwest::{
    Body,
    header::{HeaderMap, HeaderName, HeaderValue},
    redirect::Policy,
};
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
    redirection: Option<u8>,
    user_agent: String,
}

fn redirection_policy(redir: Option<u8>) -> Policy {
    if let Some(redir) = redir {
        Policy::limited(redir as usize)
    } else {
        Policy::none()
    }
}

fn hasmap_to_header_map(
    map: &HashMap<String, String>,
) -> anyhow::Result<reqwest::header::HeaderMap> {
    map.into_iter()
        .try_fold(HeaderMap::new(), |mut headers, (k, v)| {
            let name = HeaderName::from_bytes(k.as_bytes())?;
            let value = HeaderValue::from_str(&v)?;
            headers.insert(name, value);
            Ok(headers)
        })
}

#[async_trait]
impl Request for StandardRequest {
    async fn send(&self) -> anyhow::Result<String> {
        let client = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .redirect(redirection_policy(self.redirection))
            .build()?;
        println!("Send standard request to {}", self.url);
        let headers = hasmap_to_header_map(&self.headers)?;
        let resp = client
            .request(self.method.clone(), &self.url)
            .headers(headers)
            .body(self.body.clone())
            .send()
            .await?;
        let status = &resp.status();
        log!(duct_logger::Level::Info, "[{status}] - {}", resp.url());
        let body = resp.text().await?;
        if !body.is_empty() {
            println!("{body}");
        }
        Ok("200".to_string())
    }
}

impl StandardRequest {
    pub fn new(
        method: reqwest::Method,
        url: String,
        headers: HashMap<String, String>,
        body: String,
        user_agent: Option<String>,
        redirection: Option<u8>,
    ) -> Self {
        Self {
            method,
            url,
            headers,
            body,
            user_agent: user_agent.unwrap_or("duct/0.1.0".to_string()),
            redirection,
        }
    }
}
