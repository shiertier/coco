# MCP nell'SDK

Estendi Claude Code con strumenti personalizzati utilizzando i server del Model Context Protocol

---

## Panoramica

I server del Model Context Protocol (MCP) estendono Claude Code con strumenti e capacità personalizzati. Gli MCP possono essere eseguiti come processi esterni, connettersi tramite HTTP/SSE, o essere eseguiti direttamente all'interno della tua applicazione SDK.

## Configurazione

### Configurazione Base

Configura i server MCP in `.mcp.json` nella radice del tuo progetto:

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

### Utilizzo dei Server MCP nell'SDK

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Elenca i file nel mio progetto",
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
    prompt="Elenca i file nel mio progetto",
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

## Tipi di Trasporto

### Server stdio

Processi esterni che comunicano tramite stdin/stdout:

<CodeGroup>

```typescript TypeScript
// configurazione .mcp.json
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
# configurazione .mcp.json
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

### Server HTTP/SSE

Server remoti con comunicazione di rete:

<CodeGroup>

```typescript TypeScript
// configurazione server SSE
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

// configurazione server HTTP
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
# configurazione server SSE
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

# configurazione server HTTP
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

### Server MCP SDK

Server in-process che vengono eseguiti all'interno della tua applicazione. Per informazioni dettagliate sulla creazione di strumenti personalizzati, consulta la [guida agli Strumenti Personalizzati](/docs/it/agent-sdk/custom-tools):

## Gestione delle Risorse

I server MCP possono esporre risorse che Claude può elencare e leggere:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Elenca le risorse disponibili
for await (const message of query({
  prompt: "Quali risorse sono disponibili dal server del database?",
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

# Elenca le risorse disponibili
async for message in query(
    prompt="Quali risorse sono disponibili dal server del database?",
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

## Autenticazione

### Variabili d'Ambiente

<CodeGroup>

```typescript TypeScript
// .mcp.json con variabili d'ambiente
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

// Imposta le variabili d'ambiente
process.env.API_TOKEN = "your-token";
process.env.API_KEY = "your-key";
```

```python Python
# .mcp.json con variabili d'ambiente
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

# Imposta le variabili d'ambiente
import os
os.environ["API_TOKEN"] = "your-token"
os.environ["API_KEY"] = "your-key"
```

</CodeGroup>

### Autenticazione OAuth2

L'autenticazione MCP OAuth2 in-client non è attualmente supportata.

## Gestione degli Errori

Gestisci i fallimenti di connessione MCP con grazia:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Elabora dati",
  options: {
    mcpServers: {
      "data-processor": dataServer
    }
  }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // Controlla lo stato del server MCP
    const failedServers = message.mcp_servers.filter(
      s => s.status !== "connected"
    );
    
    if (failedServers.length > 0) {
      console.warn("Connessione fallita:", failedServers);
    }
  }
  
  if (message.type === "result" && message.subtype === "error_during_execution") {
    console.error("Esecuzione fallita");
  }
}
```

```python Python
from claude_agent_sdk import query

async for message in query(
    prompt="Elabora dati",
    options={
        "mcpServers": {
            "data-processor": data_server
        }
    }
):
    if message["type"] == "system" and message["subtype"] == "init":
        # Controlla lo stato del server MCP
        failed_servers = [
            s for s in message["mcp_servers"]
            if s["status"] != "connected"
        ]
        
        if failed_servers:
            print(f"Connessione fallita: {failed_servers}")
    
    if message["type"] == "result" and message["subtype"] == "error_during_execution":
        print("Esecuzione fallita")
```

</CodeGroup>

## Risorse Correlate

- [Guida agli Strumenti Personalizzati](/docs/it/agent-sdk/custom-tools) - Guida dettagliata sulla creazione di server MCP SDK
- [Riferimento SDK TypeScript](/docs/it/agent-sdk/typescript)
- [Riferimento SDK Python](/docs/it/agent-sdk/python)
- [Permessi SDK](/docs/it/agent-sdk/permissions)
- [Flussi di Lavoro Comuni](https://code.claude.com/docs/common-workflows)