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

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetFileInput {
    /// File ID (e.g. F01234567)
    pub file_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListFilesInput {
    /// Channel ID (optional, filter by channel)
    #[serde(default)]
    pub channel: Option<String>,
    /// Max files (default 20)
    #[serde(default = "default_20")]
    pub count: u32,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReadCanvasInput {
    /// Canvas ID
    pub canvas_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateCanvasInput {
    /// Canvas ID
    pub canvas_id: String,
    /// New content in markdown (replaces entire canvas)
    pub markdown: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListBookmarksInput {
    /// Channel ID
    pub channel: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AddBookmarkInput {
    /// Channel ID
    pub channel: String,
    /// Bookmark title
    pub title: String,
    /// URL to bookmark
    pub link: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ScheduleMessageInput {
    /// Channel ID
    pub channel: String,
    /// Message text
    pub text: String,
    /// Unix timestamp for when to send
    pub post_at: u64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListScheduledInput {
    /// Channel ID (optional)
    #[serde(default)]
    pub channel: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DeleteScheduledInput {
    /// Channel ID
    pub channel: String,
    /// Scheduled message ID
    pub scheduled_message_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PaginatedChannelsInput {
    /// Max per page (default 100)
    #[serde(default = "default_100")]
    pub limit: u32,
    /// Cursor for next page (omit for first page)
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PaginatedHistoryInput {
    /// Channel ID
    pub channel: String,
    /// Max per page (default 20)
    #[serde(default = "default_20")]
    pub limit: u32,
    /// Cursor for next page (omit for first page)
    #[serde(default)]
    pub cursor: Option<String>,
}

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

    #[tool(description = "Get file metadata and download URL by file ID")]
    async fn get_file(&self, Parameters(i): Parameters<GetFileInput>) -> String {
        match self.client.get_file_info(&i.file_id).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List files shared in a channel or workspace")]
    async fn list_files(&self, Parameters(i): Parameters<ListFilesInput>) -> String {
        match self.client.list_files(i.channel.as_deref(), i.count).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Read a canvas (export as markdown)")]
    async fn read_canvas(&self, Parameters(i): Parameters<ReadCanvasInput>) -> String {
        match self.client.read_canvas(&i.canvas_id).await {
            Ok(v) => v,
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Update a canvas — replace content with new markdown")]
    async fn update_canvas(&self, Parameters(i): Parameters<UpdateCanvasInput>) -> String {
        match self.client.update_canvas(&i.canvas_id, &i.markdown).await {
            Ok(()) => "Canvas updated".into(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List bookmarks in a channel")]
    async fn list_bookmarks(&self, Parameters(i): Parameters<ListBookmarksInput>) -> String {
        match self.client.list_bookmarks(&i.channel).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Add a bookmark (link) to a channel")]
    async fn add_bookmark(&self, Parameters(i): Parameters<AddBookmarkInput>) -> String {
        match self.client.add_bookmark(&i.channel, &i.title, &i.link).await {
            Ok(()) => "Bookmark added".into(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Schedule a message for future delivery (draft/delayed send)")]
    async fn schedule_message(&self, Parameters(i): Parameters<ScheduleMessageInput>) -> String {
        match self.client.schedule_message(&i.channel, &i.text, i.post_at).await {
            Ok(id) => format!("Scheduled: {id}"),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List scheduled (draft) messages")]
    async fn list_scheduled_messages(&self, Parameters(i): Parameters<ListScheduledInput>) -> String {
        match self.client.list_scheduled_messages(i.channel.as_deref()).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Delete a scheduled message before it sends")]
    async fn delete_scheduled_message(&self, Parameters(i): Parameters<DeleteScheduledInput>) -> String {
        match self.client.delete_scheduled_message(&i.channel, &i.scheduled_message_id).await {
            Ok(()) => "Scheduled message deleted".into(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List channels with cursor pagination — returns channels + next_cursor for paging")]
    async fn list_channels_paginated(&self, Parameters(i): Parameters<PaginatedChannelsInput>) -> String {
        match self.client.list_channels_paginated(i.limit, i.cursor.as_deref()).await {
            Ok((channels, cursor)) => {
                let r = serde_json::json!({"channels": channels, "next_cursor": cursor});
                serde_json::to_string_pretty(&r).unwrap()
            }
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get channel history with cursor pagination — returns messages + next_cursor for paging")]
    async fn get_history_paginated(&self, Parameters(i): Parameters<PaginatedHistoryInput>) -> String {
        match self.client.get_history_paginated(&i.channel, i.limit, i.cursor.as_deref()).await {
            Ok((messages, cursor)) => {
                let r = serde_json::json!({"messages": messages, "next_cursor": cursor});
                serde_json::to_string_pretty(&r).unwrap()
            }
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
