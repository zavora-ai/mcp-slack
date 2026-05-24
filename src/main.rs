mod client;
mod server;

use client::SlackClient;
use rmcp::{ServiceExt, transport::stdio};
use server::SlackServer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive("info".parse()?)).init();
    let token = std::env::var("SLACK_BOT_TOKEN")
        .or_else(|_| std::env::var("SLACK_TOKEN"))
        .expect("SLACK_BOT_TOKEN or SLACK_TOKEN required");
    let client = Arc::new(SlackClient::new(token));
    let server = SlackServer { client };
    tracing::info!("mcp-slack starting on stdio");
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
