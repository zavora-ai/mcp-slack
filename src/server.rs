use adk_mcp_sdk::{HealthCheck, HealthStatus};
use crate::client::SlackClient;
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListChannelsInput {
    /// Max channels to return (default 100)
    #[serde(default = "default_100")]
    pub limit: u32,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetHistoryInput {
    /// Channel ID
    pub channel: String,
    /// Max messages (default 20)
    #[serde(default = "default_20")]
    pub limit: u32,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SendMessageInput {
    /// Channel ID or name
    pub channel: String,
    /// Message text (supports Slack mrkdwn)
    pub text: String,
    /// Thread timestamp to reply in a thread
    #[serde(default)]
    pub thread_ts: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AddReactionInput {
    /// Channel ID
    pub channel: String,
    /// Message timestamp
    pub timestamp: String,
    /// Emoji name without colons (e.g. "thumbsup")
    pub emoji: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchInput {
    /// Search query
    pub query: String,
    /// Max results (default 20)
    #[serde(default = "default_20")]
    pub count: u32,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListUsersInput {
    /// Max users (default 100)
    #[serde(default = "default_100")]
    pub limit: u32,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SetTopicInput {
    /// Channel ID
    pub channel: String,
    /// New topic text
    pub topic: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UploadFileInput {
    /// Channel ID to share file in
    pub channel: String,
    /// File content (text)
    pub content: String,
    /// Filename
    pub filename: String,
    /// File title
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetUserInput {
    /// User ID (e.g. U01234567)
    pub user_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetThreadInput {
    /// Channel ID
    pub channel: String,
    /// Parent message timestamp
    pub thread_ts: String,
    /// Max replies (default 20)
    #[serde(default = "default_20")]
    pub limit: u32,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct OpenDmInput {
    /// Comma-separated user IDs to open DM with
    pub users: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateChannelInput {
    /// Channel name (lowercase, no spaces, use hyphens)
    pub name: String,
    /// Create as private channel (default false)
    #[serde(default)]
    pub is_private: Option<bool>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListMembersInput {
    /// Channel ID
    pub channel: String,
    /// Max members (default 100)
    #[serde(default = "default_100")]
    pub limit: u32,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateCanvasInput {
    /// Channel ID to share canvas in
    pub channel: String,
    /// Canvas title
    pub title: String,
    /// Canvas content in markdown
    pub markdown: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EmptyInput {}

fn default_20() -> u32 { 20 }
fn default_100() -> u32 { 100 }

#[derive(Clone)]
pub struct SlackServer {
    pub client: Arc<SlackClient>,
}

#[tool_router(server_handler)]
impl SlackServer {
    #[tool(description = "List Slack channels (public and private) the bot has access to")]
    async fn list_channels(&self, Parameters(i): Parameters<ListChannelsInput>) -> String {
        match self.client.list_channels(i.limit).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get recent messages from a channel")]
    async fn get_channel_history(&self, Parameters(i): Parameters<GetHistoryInput>) -> String {
        match self.client.get_channel_history(&i.channel, i.limit).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Send a message to a Slack channel or thread")]
    async fn send_message(&self, Parameters(i): Parameters<SendMessageInput>) -> String {
        match self.client.post_message(&i.channel, &i.text, i.thread_ts.as_deref()).await {
            Ok(ts) => format!("Message sent (ts: {ts})"),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Add an emoji reaction to a message")]
    async fn add_reaction(&self, Parameters(i): Parameters<AddReactionInput>) -> String {
        match self.client.add_reaction(&i.channel, &i.timestamp, &i.emoji).await {
            Ok(()) => format!("Reaction :{}: added", i.emoji),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Search messages across all channels")]
    async fn search_messages(&self, Parameters(i): Parameters<SearchInput>) -> String {
        match self.client.search_messages(&i.query, i.count).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List workspace users")]
    async fn list_users(&self, Parameters(i): Parameters<ListUsersInput>) -> String {
        match self.client.list_users(i.limit).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Set a channel's topic")]
    async fn set_channel_topic(&self, Parameters(i): Parameters<SetTopicInput>) -> String {
        match self.client.set_channel_topic(&i.channel, &i.topic).await {
            Ok(()) => "Topic updated".into(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Upload a text file to a channel")]
    async fn upload_file(&self, Parameters(i): Parameters<UploadFileInput>) -> String {
        let title = i.title.as_deref().unwrap_or(&i.filename);
        match self.client.upload_file(&i.channel, &i.content, &i.filename, title).await {
            Ok(()) => "File uploaded".into(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get user profile info by user ID")]
    async fn get_user(&self, Parameters(i): Parameters<GetUserInput>) -> String {
        match self.client.get_user_info(&i.user_id).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get thread replies for a message")]
    async fn get_thread(&self, Parameters(i): Parameters<GetThreadInput>) -> String {
        match self.client.get_thread_replies(&i.channel, &i.thread_ts, i.limit).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List direct message conversations")]
    async fn list_dms(&self, Parameters(i): Parameters<ListChannelsInput>) -> String {
        match self.client.list_dms(i.limit).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Open or get a DM conversation with one or more users (comma-separated user IDs)")]
    async fn open_dm(&self, Parameters(i): Parameters<OpenDmInput>) -> String {
        match self.client.open_dm(&i.users).await {
            Ok(id) => format!("DM channel opened: {id}"),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Create a new channel")]
    async fn create_channel(&self, Parameters(i): Parameters<CreateChannelInput>) -> String {
        match self.client.create_channel(&i.name, i.is_private.unwrap_or(false)).await {
            Ok(c) => serde_json::to_string_pretty(&c).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List members of a channel (returns user IDs)")]
    async fn list_members(&self, Parameters(i): Parameters<ListMembersInput>) -> String {
        match self.client.list_members(&i.channel, i.limit).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Create a canvas (rich document) and share it in a channel")]
    async fn create_canvas(&self, Parameters(i): Parameters<CreateCanvasInput>) -> String {
        match self.client.create_canvas(&i.channel, &i.title, &i.markdown).await {
            Ok(id) => format!("Canvas created: {id}"),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List custom emoji in the workspace")]
    async fn list_emoji(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        match self.client.list_emoji().await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }
}

#[async_trait::async_trait]
impl HealthCheck for SlackServer {
    async fn check_health(&self) -> HealthStatus {
        HealthStatus {
            healthy: true,
            message: Some("operational".into()),
            latency_ms: Some(1),
        }
    }
}
