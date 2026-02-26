use async_trait::async_trait;
use http::{Method, Request as HttpRequest, Uri};
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use std::collections::HashMap;
use tokio::net::TcpStream;

#[async_trait]
pub trait Request {
    async fn send(&self) -> anyhow::Result<String>;
}

pub struct StandardRequest {
    method: Method,
    url: String,
    headers: HashMap<String, String>,
    body: String,
}

#[async_trait]
impl Request for StandardRequest {
    async fn send(&self) -> anyhow::Result<String> {
        let url: hyper::Uri = self.url.parse()?;
        let host = url
            .host()
            .ok_or_else(|| anyhow::anyhow!("Host is missing"))?;
        let port = url
            .port_u16()
            .unwrap_or(if url.scheme_str() == Some("https") {
                443
            } else {
                80
            });
        let address = format!("{host}:{port}");
        let stream = TcpStream::connect(address).await?;

        let io = TokioIo::new(stream);

        let mut req_builder = HttpRequest::builder()
            .method(&self.method)
            .uri(&url)
            .header("Host", host)
            .header("User-Agent", "duct/0.1.0");

        for (key, val) in &self.headers {
            req_builder = req_builder.header(key, val);
        }

        let request = req_builder.body(Full::new(Bytes::from(self.body.clone())))?;

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

        tokio::spawn(async move {
            if let Err(err) = conn.await {
                eprintln!("Connection failed: {:?}", err);
            }
        });

        let response = sender.send_request(request).await?;

        let body_bytes = response.into_body().collect().await?.to_bytes();
        let body_str = String::from_utf8_lossy(&body_bytes).to_string();

        println!("Send standard request to {}", self.url);

        Ok(body_str)
    }
}

impl StandardRequest {
    pub fn new(
        method: http::Method,
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
