use std::collections::HashMap;

use duct_cli::Cli;
use duct_network::request::{Request, StandardRequest};

use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    dbg!(&cli);
    let req = build_request(&cli);
    req.send().await?;
    Ok(())
}

pub fn build_request(cli: &Cli) -> Box<dyn Request> {
    if cli.trace {
        todo!("TracedRequest logic to come")
    } else {
        let mut headers = HashMap::new();
        for h in &cli.header {
            if let Some((key, val)) = h.split_once(':') {
                headers.insert(key.trim().to_string(), val.trim().to_string());
            }
        }

        Box::new(StandardRequest::new(
            cli.method.parse().unwrap_or(reqwest::Method::GET),
            cli.url.clone(),
            headers,
            cli.data.clone().unwrap_or_default(),
            cli.user_agent.clone(),
            cli.redirection,
        ))
    }
}
