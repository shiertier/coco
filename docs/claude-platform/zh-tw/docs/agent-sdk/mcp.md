# SDK 中的 MCP

使用模型上下文協議伺服器透過自訂工具擴展 Claude Code

---

## 概述

模型上下文協議 (MCP) 伺服器透過自訂工具和功能擴展 Claude Code。MCP 可以作為外部程序運行、透過 HTTP/SSE 連接，或直接在您的 SDK 應用程式中執行。

## 配置

### 基本配置

在專案根目錄的 `.mcp.json` 中配置 MCP 伺服器：

<CodeGroup>

```json TypeScript
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["@modelcontextprotocol/server-filesystem"],
      "env": {
        "ALLOWED_PATHS": "/Users/me/projects"
      }
    }
  }
}
```

```json Python
{
  "mcpServers": {
    "filesystem": {
      "command": "python",
      "args": ["-m", "mcp_server_filesystem"],
      "env": {
        "ALLOWED_PATHS": "/Users/me/projects"
      }
    }
  }
}
```

</CodeGroup>

### 在 SDK 中使用 MCP 伺服器

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "List files in my project",
  options: {
    mcpServers: {
      "filesystem": {
        command: "npx",
        args: ["@modelcontextprotocol/server-filesystem"],
        env: {
          ALLOWED_PATHS: "/Users/me/projects"
        }
      }
    },
    allowedTools: ["mcp__filesystem__list_files"]
  }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

```python Python
from claude_agent_sdk import query

async for message in query(
    prompt="List files in my project",
    options={
        "mcpServers": {
            "filesystem": {
                "command": "python",
                "args": ["-m", "mcp_server_filesystem"],
                "env": {
                    "ALLOWED_PATHS": "/Users/me/projects"
                }
            }
        },
        "allowedTools": ["mcp__filesystem__list_files"]
    }
):
    if message["type"] == "result" and message["subtype"] == "success":
        print(message["result"])
```

</CodeGroup>

## 傳輸類型

### stdio 伺服器

透過 stdin/stdout 通訊的外部程序：

<CodeGroup>

```typescript TypeScript
// .mcp.json configuration
{
  "mcpServers": {
    "my-tool": {
      "command": "node",
      "args": ["./my-mcp-server.js"],
      "env": {
        "DEBUG": "${DEBUG:-false}"
      }
    }
  }
}
```

```python Python
# .mcp.json configuration
{
  "mcpServers": {
    "my-tool": {
      "command": "python",
      "args": ["./my_mcp_server.py"],
      "env": {
        "DEBUG": "${DEBUG:-false}"
      }
    }
  }
}
```

</CodeGroup>

### HTTP/SSE 伺服器

具有網路通訊的遠端伺服器：

<CodeGroup>

```typescript TypeScript
// SSE server configuration
{
  "mcpServers": {
    "remote-api": {
      "type": "sse",
      "url": "https://api.example.com/mcp/sse",
      "headers": {
        "Authorization": "Bearer ${API_TOKEN}"
      }
    }
  }
}

// HTTP server configuration
{
  "mcpServers": {
    "http-service": {
      "type": "http",
      "url": "https://api.example.com/mcp",
      "headers": {
        "X-API-Key": "${API_KEY}"
      }
    }
  }
}
```

```python Python
# SSE server configuration
{
  "mcpServers": {
    "remote-api": {
      "type": "sse",
      "url": "https://api.example.com/mcp/sse",
      "headers": {
        "Authorization": "Bearer ${API_TOKEN}"
      }
    }
  }
}

# HTTP server configuration
{
  "mcpServers": {
    "http-service": {
      "type": "http",
      "url": "https://api.example.com/mcp",
      "headers": {
        "X-API-Key": "${API_KEY}"
      }
    }
  }
}
```

</CodeGroup>

### SDK MCP 伺服器

在您的應用程式中運行的程序內伺服器。有關建立自訂工具的詳細資訊，請參閱[自訂工具指南](/docs/zh-TW/agent-sdk/custom-tools)：

## 資源管理

MCP 伺服器可以公開 Claude 可以列出和讀取的資源：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// List available resources
for await (const message of query({
  prompt: "What resources are available from the database server?",
  options: {
    mcpServers: {
      "database": {
        command: "npx",
        args: ["@modelcontextprotocol/server-database"]
      }
    },
    allowedTools: ["mcp__list_resources", "mcp__read_resource"]
  }
})) {
  if (message.type === "result") console.log(message.result);
}
```

```python Python
from claude_agent_sdk import query

# List available resources
async for message in query(
    prompt="What resources are available from the database server?",
    options={
        "mcpServers": {
            "database": {
                "command": "python",
                "args": ["-m", "mcp_server_database"]
            }
        },
        "allowedTools": ["mcp__list_resources", "mcp__read_resource"]
    }
):
    if message["type"] == "result":
        print(message["result"])
```

</CodeGroup>

## 身份驗證

### 環境變數

<CodeGroup>

```typescript TypeScript
// .mcp.json with environment variables
{
  "mcpServers": {
    "secure-api": {
      "type": "sse",
      "url": "https://api.example.com/mcp",
      "headers": {
        "Authorization": "Bearer ${API_TOKEN}",
        "X-API-Key": "${API_KEY:-default-key}"
      }
    }
  }
}

// Set environment variables
process.env.API_TOKEN = "your-token";
process.env.API_KEY = "your-key";
```

```python Python
# .mcp.json with environment variables
{
  "mcpServers": {
    "secure-api": {
      "type": "sse",
      "url": "https://api.example.com/mcp",
      "headers": {
        "Authorization": "Bearer ${API_TOKEN}",
        "X-API-Key": "${API_KEY:-default-key}"
      }
    }
  }
}

# Set environment variables
import os
os.environ["API_TOKEN"] = "your-token"
os.environ["API_KEY"] = "your-key"
```

</CodeGroup>

### OAuth2 身份驗證

目前不支援客戶端內的 OAuth2 MCP 身份驗證。

## 錯誤處理

優雅地處理 MCP 連接失敗：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Process data",
  options: {
    mcpServers: {
      "data-processor": dataServer
    }
  }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // Check MCP server status
    const failedServers = message.mcp_servers.filter(
      s => s.status !== "connected"
    );
    
    if (failedServers.length > 0) {
      console.warn("Failed to connect:", failedServers);
    }
  }
  
  if (message.type === "result" && message.subtype === "error_during_execution") {
    console.error("Execution failed");
  }
}
```

```python Python
from claude_agent_sdk import query

async for message in query(
    prompt="Process data",
    options={
        "mcpServers": {
            "data-processor": data_server
        }
    }
):
    if message["type"] == "system" and message["subtype"] == "init":
        # Check MCP server status
        failed_servers = [
            s for s in message["mcp_servers"]
            if s["status"] != "connected"
        ]
        
        if failed_servers:
            print(f"Failed to connect: {failed_servers}")
    
    if message["type"] == "result" and message["subtype"] == "error_during_execution":
        print("Execution failed")
```

</CodeGroup>

## 相關資源

- [自訂工具指南](/docs/zh-TW/agent-sdk/custom-tools) - 建立 SDK MCP 伺服器的詳細指南
- [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript)
- [Python SDK 參考](/docs/zh-TW/agent-sdk/python)
- [SDK 權限](/docs/zh-TW/agent-sdk/permissions)
- [常見工作流程](https://code.claude.com/docs/common-workflows)