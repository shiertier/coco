# SDK에서의 MCP

Model Context Protocol 서버를 사용하여 사용자 정의 도구로 Claude Code를 확장하세요

---

## 개요

Model Context Protocol (MCP) 서버는 사용자 정의 도구와 기능으로 Claude Code를 확장합니다. MCP는 외부 프로세스로 실행되거나, HTTP/SSE를 통해 연결되거나, SDK 애플리케이션 내에서 직접 실행될 수 있습니다.

## 구성

### 기본 구성

프로젝트 루트의 `.mcp.json`에서 MCP 서버를 구성하세요:

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

### SDK에서 MCP 서버 사용하기

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "내 프로젝트의 파일들을 나열해줘",
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
    prompt="내 프로젝트의 파일들을 나열해줘",
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

## 전송 유형

### stdio 서버

stdin/stdout을 통해 통신하는 외부 프로세스:

<CodeGroup>

```typescript TypeScript
// .mcp.json 구성
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
# .mcp.json 구성
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

### HTTP/SSE 서버

네트워크 통신을 사용하는 원격 서버:

<CodeGroup>

```typescript TypeScript
// SSE 서버 구성
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

// HTTP 서버 구성
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
# SSE 서버 구성
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

# HTTP 서버 구성
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

### SDK MCP 서버

애플리케이션 내에서 실행되는 인프로세스 서버. 사용자 정의 도구 생성에 대한 자세한 정보는 [사용자 정의 도구 가이드](/docs/ko/agent-sdk/custom-tools)를 참조하세요:

## 리소스 관리

MCP 서버는 Claude가 나열하고 읽을 수 있는 리소스를 노출할 수 있습니다:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 사용 가능한 리소스 나열
for await (const message of query({
  prompt: "데이터베이스 서버에서 사용 가능한 리소스는 무엇인가요?",
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

# 사용 가능한 리소스 나열
async for message in query(
    prompt="데이터베이스 서버에서 사용 가능한 리소스는 무엇인가요?",
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

## 인증

### 환경 변수

<CodeGroup>

```typescript TypeScript
// 환경 변수가 포함된 .mcp.json
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

// 환경 변수 설정
process.env.API_TOKEN = "your-token";
process.env.API_KEY = "your-key";
```

```python Python
# 환경 변수가 포함된 .mcp.json
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

# 환경 변수 설정
import os
os.environ["API_TOKEN"] = "your-token"
os.environ["API_KEY"] = "your-key"
```

</CodeGroup>

### OAuth2 인증

클라이언트 내 OAuth2 MCP 인증은 현재 지원되지 않습니다.

## 오류 처리

MCP 연결 실패를 우아하게 처리하세요:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "데이터 처리",
  options: {
    mcpServers: {
      "data-processor": dataServer
    }
  }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // MCP 서버 상태 확인
    const failedServers = message.mcp_servers.filter(
      s => s.status !== "connected"
    );
    
    if (failedServers.length > 0) {
      console.warn("연결 실패:", failedServers);
    }
  }
  
  if (message.type === "result" && message.subtype === "error_during_execution") {
    console.error("실행 실패");
  }
}
```

```python Python
from claude_agent_sdk import query

async for message in query(
    prompt="데이터 처리",
    options={
        "mcpServers": {
            "data-processor": data_server
        }
    }
):
    if message["type"] == "system" and message["subtype"] == "init":
        # MCP 서버 상태 확인
        failed_servers = [
            s for s in message["mcp_servers"]
            if s["status"] != "connected"
        ]
        
        if failed_servers:
            print(f"연결 실패: {failed_servers}")
    
    if message["type"] == "result" and message["subtype"] == "error_during_execution":
        print("실행 실패")
```

</CodeGroup>

## 관련 리소스

- [사용자 정의 도구 가이드](/docs/ko/agent-sdk/custom-tools) - SDK MCP 서버 생성에 대한 자세한 가이드
- [TypeScript SDK 참조](/docs/ko/agent-sdk/typescript)
- [Python SDK 참조](/docs/ko/agent-sdk/python)
- [SDK 권한](/docs/ko/agent-sdk/permissions)
- [일반적인 워크플로우](https://code.claude.com/docs/common-workflows)