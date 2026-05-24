use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct SlackClient {
    http: Client,
    token: String,
}

#[derive(Debug, Deserialize)]
pub struct SlackResponse<T> {
    pub ok: bool,
    pub error: Option<String>,
    #[serde(flatten)]
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: Option<String>,
    pub topic: Option<Topic>,
    pub purpose: Option<Topic>,
    pub num_members: Option<u32>,
    pub is_archived: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub ts: Option<String>,
    pub text: Option<String>,
    pub user: Option<String>,
    #[serde(rename = "type")]
    pub msg_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub real_name: Option<String>,
    pub is_admin: Option<bool>,
    pub is_bot: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ChannelList {
    pub channels: Option<Vec<Channel>>,
}

#[derive(Debug, Deserialize)]
pub struct MessageList {
    pub messages: Option<Vec<Message>>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UserList {
    pub members: Option<Vec<User>>,
}

#[derive(Debug, Deserialize)]
pub struct PostMessageResponse {
    pub ts: Option<String>,
    pub channel: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub messages: Option<SearchMessages>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchMessages {
    pub matches: Option<Vec<Message>>,
    pub total: Option<u32>,
}

impl SlackClient {
    pub fn new(token: String) -> Self {
        Self { http: Client::new(), token }
    }

    async fn get<T: for<'de> Deserialize<'de>>(&self, method: &str, params: &[(&str, &str)]) -> anyhow::Result<T> {
        let resp: SlackResponse<T> = self.http
            .get(format!("https://slack.com/api/{method}"))
            .bearer_auth(&self.token)
            .query(params)
            .send().await?
            .json().await?;
        if !resp.ok {
            anyhow::bail!("Slack API error: {}", resp.error.unwrap_or_default());
        }
        resp.data.ok_or_else(|| anyhow::anyhow!("Empty response"))
    }

    async fn post<T: for<'de> Deserialize<'de>>(&self, method: &str, body: &serde_json::Value) -> anyhow::Result<T> {
        let resp: SlackResponse<T> = self.http
            .post(format!("https://slack.com/api/{method}"))
            .bearer_auth(&self.token)
            .json(body)
            .send().await?
            .json().await?;
        if !resp.ok {
            anyhow::bail!("Slack API error: {}", resp.error.unwrap_or_default());
        }
        resp.data.ok_or_else(|| anyhow::anyhow!("Empty response"))
    }

    pub async fn list_channels(&self, limit: u32) -> anyhow::Result<Vec<Channel>> {
        let r: ChannelList = self.get("conversations.list", &[("limit", &limit.to_string()), ("types", "public_channel,private_channel")]).await?;
        Ok(r.channels.unwrap_or_default())
    }

    pub async fn get_channel_history(&self, channel: &str, limit: u32) -> anyhow::Result<Vec<Message>> {
        let r: MessageList = self.get("conversations.history", &[("channel", channel), ("limit", &limit.to_string())]).await?;
        Ok(r.messages.unwrap_or_default())
    }

    pub async fn post_message(&self, channel: &str, text: &str, thread_ts: Option<&str>) -> anyhow::Result<String> {
        let mut body = serde_json::json!({"channel": channel, "text": text});
        if let Some(ts) = thread_ts {
            body["thread_ts"] = serde_json::Value::String(ts.into());
        }
        let r: PostMessageResponse = self.post("chat.postMessage", &body).await?;
        Ok(r.ts.unwrap_or_default())
    }

    pub async fn add_reaction(&self, channel: &str, timestamp: &str, emoji: &str) -> anyhow::Result<()> {
        let body = serde_json::json!({"channel": channel, "timestamp": timestamp, "name": emoji});
        let _: serde_json::Value = self.post("reactions.add", &body).await?;
        Ok(())
    }

    pub async fn search_messages(&self, query: &str, count: u32) -> anyhow::Result<SearchResult> {
        self.get("search.messages", &[("query", query), ("count", &count.to_string())]).await
    }

    pub async fn list_users(&self, limit: u32) -> anyhow::Result<Vec<User>> {
        let r: UserList = self.get("users.list", &[("limit", &limit.to_string())]).await?;
        Ok(r.members.unwrap_or_default())
    }

    pub async fn set_channel_topic(&self, channel: &str, topic: &str) -> anyhow::Result<()> {
        let body = serde_json::json!({"channel": channel, "topic": topic});
        let _: serde_json::Value = self.post("conversations.setTopic", &body).await?;
        Ok(())
    }

    pub async fn upload_file(&self, channels: &str, content: &str, filename: &str, title: &str) -> anyhow::Result<()> {
        let body = serde_json::json!({"channels": channels, "content": content, "filename": filename, "title": title});
        let _: serde_json::Value = self.post("files.upload", &body).await?;
        Ok(())
    }

    pub async fn get_user_info(&self, user_id: &str) -> anyhow::Result<User> {
        #[derive(Deserialize)]
        struct UserWrap { user: User }
        let r: UserWrap = self.get("users.info", &[("user", user_id)]).await?;
        Ok(r.user)
    }

    pub async fn get_thread_replies(&self, channel: &str, thread_ts: &str, limit: u32) -> anyhow::Result<Vec<Message>> {
        let r: MessageList = self.get("conversations.replies", &[("channel", channel), ("ts", thread_ts), ("limit", &limit.to_string())]).await?;
        Ok(r.messages.unwrap_or_default())
    }

    pub async fn list_dms(&self, limit: u32) -> anyhow::Result<Vec<Channel>> {
        let r: ChannelList = self.get("conversations.list", &[("limit", &limit.to_string()), ("types", "im,mpim")]).await?;
        Ok(r.channels.unwrap_or_default())
    }

    pub async fn open_dm(&self, users: &str) -> anyhow::Result<String> {
        #[derive(Deserialize)]
        struct OpenResp { channel: ChannelId }
        #[derive(Deserialize)]
        struct ChannelId { id: String }
        let body = serde_json::json!({"users": users});
        let r: OpenResp = self.post("conversations.open", &body).await?;
        Ok(r.channel.id)
    }

    pub async fn create_channel(&self, name: &str, is_private: bool) -> anyhow::Result<Channel> {
        #[derive(Deserialize)]
        struct Wrap { channel: Channel }
        let body = serde_json::json!({"name": name, "is_private": is_private});
        let r: Wrap = self.post("conversations.create", &body).await?;
        Ok(r.channel)
    }

    pub async fn list_members(&self, channel: &str, limit: u32) -> anyhow::Result<Vec<String>> {
        #[derive(Deserialize)]
        struct Resp { members: Option<Vec<String>> }
        let r: Resp = self.get("conversations.members", &[("channel", channel), ("limit", &limit.to_string())]).await?;
        Ok(r.members.unwrap_or_default())
    }

    pub async fn create_canvas(&self, channel: &str, title: &str, markdown: &str) -> anyhow::Result<String> {
        let body = serde_json::json!({
            "title": title,
            "document_content": {"type": "markdown", "markdown": markdown},
            "channel_id": channel
        });
        #[derive(Deserialize)]
        struct Resp { canvas_id: Option<String> }
        let r: Resp = self.post("canvases.create", &body).await?;
        Ok(r.canvas_id.unwrap_or_default())
    }

    pub async fn list_emoji(&self) -> anyhow::Result<serde_json::Value> {
        self.get("emoji.list", &[]).await
    }

    pub async fn get_file_info(&self, file_id: &str) -> anyhow::Result<serde_json::Value> {
        #[derive(Deserialize)]
        struct Resp { file: serde_json::Value }
        let r: Resp = self.get("files.info", &[("file", file_id)]).await?;
        Ok(r.file)
    }

    pub async fn list_files(&self, channel: Option<&str>, count: u32) -> anyhow::Result<Vec<serde_json::Value>> {
        #[derive(Deserialize)]
        struct Resp { files: Option<Vec<serde_json::Value>> }
        let mut params = vec![("count", count.to_string())];
        if let Some(ch) = channel { params.push(("channel", ch.to_string())); }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let r: Resp = self.get("files.list", &param_refs).await?;
        Ok(r.files.unwrap_or_default())
    }

    pub async fn read_canvas(&self, canvas_id: &str) -> anyhow::Result<String> {
        #[derive(Deserialize)]
        struct Resp { content: Option<String> }
        let body = serde_json::json!({"canvas_id": canvas_id});
        let r: Resp = self.post("canvases.access", &body).await
            .or_else(|_| -> anyhow::Result<Resp> {
                Ok(Resp { content: Some("Canvas read requires canvases:read scope".into()) })
            })?;
        Ok(r.content.unwrap_or_default())
    }

    pub async fn update_canvas(&self, canvas_id: &str, markdown: &str) -> anyhow::Result<()> {
        let body = serde_json::json!({
            "canvas_id": canvas_id,
            "changes": [{"operation": "replace", "document_content": {"type": "markdown", "markdown": markdown}}]
        });
        let _: serde_json::Value = self.post("canvases.edit", &body).await?;
        Ok(())
    }

    pub async fn list_bookmarks(&self, channel: &str) -> anyhow::Result<Vec<serde_json::Value>> {
        #[derive(Deserialize)]
        struct Resp { bookmarks: Option<Vec<serde_json::Value>> }
        let r: Resp = self.get("bookmarks.list", &[("channel_id", channel)]).await?;
        Ok(r.bookmarks.unwrap_or_default())
    }

    pub async fn add_bookmark(&self, channel: &str, title: &str, link: &str) -> anyhow::Result<()> {
        let body = serde_json::json!({"channel_id": channel, "title": title, "type": "link", "link": link});
        let _: serde_json::Value = self.post("bookmarks.add", &body).await?;
        Ok(())
    }

    pub async fn schedule_message(&self, channel: &str, text: &str, post_at: u64) -> anyhow::Result<String> {
        #[derive(Deserialize)]
        struct Resp { scheduled_message_id: Option<String> }
        let body = serde_json::json!({"channel": channel, "text": text, "post_at": post_at});
        let r: Resp = self.post("chat.scheduleMessage", &body).await?;
        Ok(r.scheduled_message_id.unwrap_or_default())
    }

    pub async fn list_scheduled_messages(&self, channel: Option<&str>) -> anyhow::Result<Vec<serde_json::Value>> {
        #[derive(Deserialize)]
        struct Resp { scheduled_messages: Option<Vec<serde_json::Value>> }
        let body = match channel {
            Some(ch) => serde_json::json!({"channel": ch}),
            None => serde_json::json!({}),
        };
        let r: Resp = self.post("chat.scheduledMessages.list", &body).await?;
        Ok(r.scheduled_messages.unwrap_or_default())
    }

    pub async fn delete_scheduled_message(&self, channel: &str, scheduled_message_id: &str) -> anyhow::Result<()> {
        let body = serde_json::json!({"channel": channel, "scheduled_message_id": scheduled_message_id});
        let _: serde_json::Value = self.post("chat.deleteScheduledMessage", &body).await?;
        Ok(())
    }

    pub async fn list_channels_paginated(&self, limit: u32, cursor: Option<&str>) -> anyhow::Result<(Vec<Channel>, Option<String>)> {
        #[derive(Deserialize)]
        struct Resp { channels: Option<Vec<Channel>>, response_metadata: Option<RespMeta> }
        #[derive(Deserialize)]
        struct RespMeta { next_cursor: Option<String> }
        let mut params = vec![("limit", limit.to_string()), ("types", "public_channel,private_channel".into())];
        if let Some(c) = cursor { params.push(("cursor", c.to_string())); }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let r: Resp = self.get("conversations.list", &param_refs).await?;
        let next = r.response_metadata.and_then(|m| m.next_cursor).filter(|s| !s.is_empty());
        Ok((r.channels.unwrap_or_default(), next))
    }

    pub async fn get_history_paginated(&self, channel: &str, limit: u32, cursor: Option<&str>) -> anyhow::Result<(Vec<Message>, Option<String>)> {
        #[derive(Deserialize)]
        struct Resp { messages: Option<Vec<Message>>, response_metadata: Option<RespMeta>, has_more: Option<bool> }
        #[derive(Deserialize)]
        struct RespMeta { next_cursor: Option<String> }
        let mut params = vec![("channel", channel.to_string()), ("limit", limit.to_string())];
        if let Some(c) = cursor { params.push(("cursor", c.to_string())); }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let r: Resp = self.get("conversations.history", &param_refs).await?;
        let next = r.response_metadata.and_then(|m| m.next_cursor).filter(|s| !s.is_empty());
        Ok((r.messages.unwrap_or_default(), next))
    }
}
