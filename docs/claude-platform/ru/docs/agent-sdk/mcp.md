# MCP в SDK

Расширьте Claude Code с помощью пользовательских инструментов, используя серверы Model Context Protocol

---

## Обзор

Серверы Model Context Protocol (MCP) расширяют Claude Code с помощью пользовательских инструментов и возможностей. MCP могут работать как внешние процессы, подключаться через HTTP/SSE или выполняться непосредственно в вашем SDK-приложении.

## Конфигурация

### Базовая конфигурация

Настройте MCP-серверы в `.mcp.json` в корне вашего проекта:

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

### Использование MCP-серверов в SDK

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

## Типы транспорта

### stdio-серверы

Внешние процессы, взаимодействующие через stdin/stdout:

<CodeGroup>

```typescript TypeScript
// Конфигурация .mcp.json
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
# Конфигурация .mcp.json
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

### HTTP/SSE-серверы

Удаленные серверы с сетевой связью:

<CodeGroup>

```typescript TypeScript
// Конфигурация SSE-сервера
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

// Конфигурация HTTP-сервера
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
# Конфигурация SSE-сервера
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

# Конфигурация HTTP-сервера
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

### SDK MCP-серверы

Внутрипроцессные серверы, работающие в вашем приложении. Для подробной информации о создании пользовательских инструментов см. [руководство по пользовательским инструментам](/docs/ru/agent-sdk/custom-tools):

## Управление ресурсами

MCP-серверы могут предоставлять ресурсы, которые Claude может перечислять и читать:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Перечислить доступные ресурсы
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

# Перечислить доступные ресурсы
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

## Аутентификация

### Переменные окружения

<CodeGroup>

```typescript TypeScript
// .mcp.json с переменными окружения
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

// Установить переменные окружения
process.env.API_TOKEN = "your-token";
process.env.API_KEY = "your-key";
```

```python Python
# .mcp.json с переменными окружения
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

# Установить переменные окружения
import os
os.environ["API_TOKEN"] = "your-token"
os.environ["API_KEY"] = "your-key"
```

</CodeGroup>

### Аутентификация OAuth2

Аутентификация OAuth2 MCP в клиенте в настоящее время не поддерживается.

## Обработка ошибок

Корректно обрабатывайте сбои подключения MCP:

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
    // Проверить статус MCP-сервера
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
        # Проверить статус MCP-сервера
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

## Связанные ресурсы

- [Руководство по пользовательским инструментам](/docs/ru/agent-sdk/custom-tools) - Подробное руководство по созданию SDK MCP-серверов
- [Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript)
- [Справочник Python SDK](/docs/ru/agent-sdk/python)
- [Разрешения SDK](/docs/ru/agent-sdk/permissions)
- [Общие рабочие процессы](https://code.claude.com/docs/common-workflows)