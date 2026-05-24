# Slack MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-slack.svg)](https://crates.io/crates/mcp-slack)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)
[![Registry Ready](https://img.shields.io/badge/ADK_Registry-Ready-green.svg)](https://www.zavora.ai)

Give your AI agents full Slack access — messages, threads, DMs, channels, canvases, reactions, and search. 16 tools over the Slack Web API with enterprise governance and audit-ready risk classification.

## Tools (16)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `list_channels` | List public and private channels | Read-only |
| `get_channel_history` | Get recent messages from a channel | Read-only |
| `get_thread` | Get thread replies for a message | Read-only |
| `search_messages` | Search messages across all channels | Read-only |
| `list_users` | List workspace users | Read-only |
| `get_user` | Get user profile info by ID | Read-only |
| `list_members` | List members of a channel | Read-only |
| `list_dms` | List direct message conversations | Read-only |
| `list_emoji` | List custom emoji in the workspace | Read-only |
| `send_message` | Send a message to a channel or thread | External write |
| `open_dm` | Open a DM conversation with users | External write |
| `add_reaction` | Add an emoji reaction to a message | External write |
| `set_channel_topic` | Set a channel's topic | External write |
| `upload_file` | Upload a text file to a channel | External write |
| `create_channel` | Create a new public or private channel | External write |
| `create_canvas` | Create a rich document and share in channel | External write |

## Comparison with Other Slack MCP Servers

| Feature | Slack Official | korotovsky/slack-mcp | **Ours** |
|---------|---------------|---------------------|----------|
| Thread replies (read) | ✅ | ✅ | ✅ |
| DMs / Group DMs | ✅ | ✅ | ✅ |
| Canvases (create) | ✅ | ❌ | ✅ |
| Create channel | ✅ | ✅ | ✅ |
| List channel members | ✅ | ✅ | ✅ |
| Custom emoji list | ✅ | ❌ | ✅ |
| Send messages + threads | ✅ | ✅ | ✅ |
| Reactions | ✅ | ✅ | ✅ |
| Search | ✅ | ✅ | ✅ |
| File upload | ✅ | ❌ | ✅ |
| File reading | ✅ | ❌ | ❌ |
| Canvas read/update | ✅ | ❌ | ❌ |
| Draft messages | ✅ | ❌ | ❌ |
| Bookmarks | ❌ | ✅ | ❌ |
| Pagination (cursor) | ✅ | ✅ | ❌ |
| OAuth/user tokens | ✅ | ✅ (cookie) | Bot token |
| **Registry governance** | ❌ | ❌ | ✅ |
| **Risk classification** | ❌ | ❌ | ✅ |
| **HealthCheck trait** | ❌ | ❌ | ✅ |
| **Rust / single binary** | ❌ (Node) | ❌ (Node) | ✅ |

## Installation

### Build from source

```bash
git clone https://github.com/zavora-ai/mcp-slack
cd mcp-slack
cargo build --release
```

### From crates.io

```bash
cargo install mcp-slack
```

## Configuration

### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `SLACK_BOT_TOKEN` | ✅ | Bot User OAuth Token (`xoxb-...`) |
| `SLACK_TOKEN` | Alternative | Alias for `SLACK_BOT_TOKEN` |
| `RUST_LOG` | ❌ | Log level (default: `info`) |

### Required Bot Token Scopes

```
channels:read        — list channels
channels:history     — read messages
channels:manage      — set topic
groups:read          — private channels
groups:history       — private channel messages
chat:write           — send messages
reactions:write      — add reactions
search:read          — search messages
users:read           — list/get users
files:write          — upload files
channels:join        — self-join channels (optional)
```

### Slack App Manifest

```yaml
display_information:
  name: MCP Slack Bot
  description: AI agent access to Slack via MCP protocol
features:
  bot_user:
    display_name: MCP Bot
    always_online: true
oauth_config:
  scopes:
    bot:
      - channels:read
      - channels:history
      - channels:manage
      - groups:read
      - groups:history
      - chat:write
      - reactions:write
      - search:read
      - users:read
      - files:write
settings:
  org_deploy_enabled: false
  socket_mode_enabled: false
  token_rotation_enabled: false
```

## Client Configuration

### Claude Desktop

```json
{
  "mcpServers": {
    "slack": {
      "command": "mcp-slack",
      "args": [],
      "env": {
        "SLACK_BOT_TOKEN": "xoxb-your-token"
      }
    }
  }
}
```

### Kiro

Add to `.kiro/settings/mcp.json`:

```json
{
  "mcpServers": {
    "slack": {
      "command": "mcp-slack",
      "args": [],
      "env": {
        "SLACK_BOT_TOKEN": "xoxb-your-token"
      }
    }
  }
}
```

### Cursor

Add to `.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "slack": {
      "command": "mcp-slack",
      "args": [],
      "env": {
        "SLACK_BOT_TOKEN": "xoxb-your-token"
      }
    }
  }
}
```

### Codex (OpenAI)

```json
{
  "mcpServers": {
    "slack": {
      "command": "mcp-slack"
    }
  }
}
```

### Windsurf

Add to `~/.codeium/windsurf/mcp_config.json`:

```json
{
  "mcpServers": {
    "slack": {
      "command": "mcp-slack",
      "args": [],
      "env": {
        "SLACK_BOT_TOKEN": "xoxb-your-token"
      }
    }
  }
}
```

## Quick Start

```bash
# Set token
export SLACK_BOT_TOKEN="xoxb-your-token"

# Verify token works
curl -s -H "Authorization: Bearer $SLACK_BOT_TOKEN" https://slack.com/api/auth.test

# Run server
mcp-slack
```

### Test with MCP Inspector

```bash
npx @modelcontextprotocol/inspector mcp-slack
```

## Usage Examples

### List channels
```
"List my Slack channels"
→ calls list_channels
```

### Read a thread
```
"What did the team say in the thread about the deployment?"
→ calls search_messages → get_thread
```

### Send a message
```
"Post in #engineering: the release is ready for QA"
→ calls send_message with channel + text
```

### Reply in a thread
```
"Reply to that message saying I'll review it"
→ calls send_message with thread_ts
```

### Open a DM
```
"Send a DM to James asking about the PR"
→ calls open_dm → send_message
```

### Create a channel
```
"Create a private channel called project-alpha"
→ calls create_channel with is_private=true
```

## MCP Server Manifest

```toml
server_id = "mcp_slack"
display_name = "Slack"
version = "1.1.0"
domain = "collaboration"
risk_level = "medium"
writes_allowed = "gated"
transports = ["stdio"]
credentials = ["vault://slack-bot-token"]
```

## Testing

```bash
# Build
cargo build

# Run tests
cargo test

# Integration test (requires SLACK_BOT_TOKEN)
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"list_channels","arguments":{"limit":5}}}' | cargo run
```

## Roadmap

- [ ] File reading (`files.info` + download)
- [ ] Canvas read/update (`canvases.sections.lookup`, `canvases.edit`)
- [ ] Cursor-based pagination for large workspaces
- [ ] Bookmarks (`bookmarks.list`, `bookmarks.add`)
- [ ] User status (`users.profile.set`)
- [ ] Draft messages (preview before send)
- [ ] OAuth user token support (for user-level access)

## Registry Compliance

This server implements the [ADK MCP SDK](https://crates.io/crates/adk-mcp-sdk) contract:

- **HealthCheck** — async health probe for registry monitoring
- **mcp-server.toml** — manifest declaring tools, risk classes, and credentials
- **Structured tracing** — `RUST_LOG` env-filter for observability

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START -->
| [<img src="https://github.com/jkmaina.png" width="80px;" alt=""/><br /><sub><b>James Karanja Maina</b></sub>](https://github.com/jkmaina) |
|:---:|
<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Apache-2.0 — see [LICENSE](LICENSE) for details.

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.

Built with ❤️ by [Zavora AI](https://zavora.ai)
