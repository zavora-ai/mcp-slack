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
}
