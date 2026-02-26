use async_trait::async_trait;
use reqwest::redirect::Policy;
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
    redirection: bool,
    user_agent: String,
}

fn redirection_policy(redir: bool) -> Policy {
    if redir {
        Policy::limited(10)
    } else {
        Policy::none()
    }
}

#[async_trait]
impl Request for StandardRequest {
    async fn send(&self) -> anyhow::Result<String> {
        let client = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .redirect(redirection_policy(self.redirection))
            .build()?;
        println!("Send standard request to {}", self.url);
        // let resp = client.get(&self.url).send().await?;
        let resp = client
            .request(self.method.clone(), &self.url)
            .send()
            .await?;
        let status = &resp.status();
        println!("[{status}] - {}", resp.url());
        let body = resp.text().await?;
        if !body.is_empty() {
            println!("{body}");
        }
        // println!("{resp:#?}");
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
        redirection: bool,
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
