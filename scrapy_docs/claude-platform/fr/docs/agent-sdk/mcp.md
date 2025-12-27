# MCP dans le SDK

Étendez Claude Code avec des outils personnalisés en utilisant les serveurs Model Context Protocol

---

## Vue d'ensemble

Les serveurs Model Context Protocol (MCP) étendent Claude Code avec des outils et des capacités personnalisés. Les MCP peuvent s'exécuter en tant que processus externes, se connecter via HTTP/SSE, ou s'exécuter directement dans votre application SDK.

## Configuration

### Configuration de base

Configurez les serveurs MCP dans `.mcp.json` à la racine de votre projet :

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

### Utilisation des serveurs MCP dans le SDK

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Lister les fichiers dans mon projet",
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
    prompt="Lister les fichiers dans mon projet",
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

## Types de transport

### Serveurs stdio

Processus externes communiquant via stdin/stdout :

<CodeGroup>

```typescript TypeScript
// Configuration .mcp.json
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
# Configuration .mcp.json
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

### Serveurs HTTP/SSE

Serveurs distants avec communication réseau :

<CodeGroup>

```typescript TypeScript
// Configuration serveur SSE
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

// Configuration serveur HTTP
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
# Configuration serveur SSE
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

# Configuration serveur HTTP
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

### Serveurs MCP SDK

Serveurs en processus s'exécutant dans votre application. Pour des informations détaillées sur la création d'outils personnalisés, consultez le [guide des outils personnalisés](/docs/fr/agent-sdk/custom-tools) :

## Gestion des ressources

Les serveurs MCP peuvent exposer des ressources que Claude peut lister et lire :

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Lister les ressources disponibles
for await (const message of query({
  prompt: "Quelles ressources sont disponibles depuis le serveur de base de données ?",
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

# Lister les ressources disponibles
async for message in query(
    prompt="Quelles ressources sont disponibles depuis le serveur de base de données ?",
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

## Authentification

### Variables d'environnement

<CodeGroup>

```typescript TypeScript
// .mcp.json avec variables d'environnement
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

// Définir les variables d'environnement
process.env.API_TOKEN = "your-token";
process.env.API_KEY = "your-key";
```

```python Python
# .mcp.json avec variables d'environnement
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

# Définir les variables d'environnement
import os
os.environ["API_TOKEN"] = "your-token"
os.environ["API_KEY"] = "your-key"
```

</CodeGroup>

### Authentification OAuth2

L'authentification OAuth2 MCP côté client n'est actuellement pas prise en charge.

## Gestion des erreurs

Gérez les échecs de connexion MCP avec élégance :

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Traiter les données",
  options: {
    mcpServers: {
      "data-processor": dataServer
    }
  }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // Vérifier le statut du serveur MCP
    const failedServers = message.mcp_servers.filter(
      s => s.status !== "connected"
    );
    
    if (failedServers.length > 0) {
      console.warn("Échec de connexion :", failedServers);
    }
  }
  
  if (message.type === "result" && message.subtype === "error_during_execution") {
    console.error("Échec de l'exécution");
  }
}
```

```python Python
from claude_agent_sdk import query

async for message in query(
    prompt="Traiter les données",
    options={
        "mcpServers": {
            "data-processor": data_server
        }
    }
):
    if message["type"] == "system" and message["subtype"] == "init":
        # Vérifier le statut du serveur MCP
        failed_servers = [
            s for s in message["mcp_servers"]
            if s["status"] != "connected"
        ]
        
        if failed_servers:
            print(f"Échec de connexion : {failed_servers}")
    
    if message["type"] == "result" and message["subtype"] == "error_during_execution":
        print("Échec de l'exécution")
```

</CodeGroup>

## Ressources connexes

- [Guide des outils personnalisés](/docs/fr/agent-sdk/custom-tools) - Guide détaillé sur la création de serveurs MCP SDK
- [Référence SDK TypeScript](/docs/fr/agent-sdk/typescript)
- [Référence SDK Python](/docs/fr/agent-sdk/python)
- [Permissions SDK](/docs/fr/agent-sdk/permissions)
- [Flux de travail courants](https://code.claude.com/docs/common-workflows)