# MCP en el SDK

Extiende Claude Code con herramientas personalizadas usando servidores del Protocolo de Contexto de Modelo

---

## Descripción general

Los servidores del Protocolo de Contexto de Modelo (MCP) extienden Claude Code con herramientas y capacidades personalizadas. Los MCP pueden ejecutarse como procesos externos, conectarse vía HTTP/SSE, o ejecutarse directamente dentro de tu aplicación SDK.

## Configuración

### Configuración básica

Configura servidores MCP en `.mcp.json` en la raíz de tu proyecto:

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

### Usando servidores MCP en el SDK

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Lista archivos en mi proyecto",
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
    prompt="Lista archivos en mi proyecto",
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

## Tipos de transporte

### Servidores stdio

Procesos externos que se comunican vía stdin/stdout:

<CodeGroup>

```typescript TypeScript
// configuración .mcp.json
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
# configuración .mcp.json
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

### Servidores HTTP/SSE

Servidores remotos con comunicación de red:

<CodeGroup>

```typescript TypeScript
// configuración de servidor SSE
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

// configuración de servidor HTTP
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
# configuración de servidor SSE
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

# configuración de servidor HTTP
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

### Servidores MCP del SDK

Servidores en proceso que se ejecutan dentro de tu aplicación. Para información detallada sobre crear herramientas personalizadas, consulta la [guía de Herramientas personalizadas](/docs/es/agent-sdk/custom-tools):

## Gestión de recursos

Los servidores MCP pueden exponer recursos que Claude puede listar y leer:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Listar recursos disponibles
for await (const message of query({
  prompt: "¿Qué recursos están disponibles desde el servidor de base de datos?",
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

# Listar recursos disponibles
async for message in query(
    prompt="¿Qué recursos están disponibles desde el servidor de base de datos?",
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

## Autenticación

### Variables de entorno

<CodeGroup>

```typescript TypeScript
// .mcp.json con variables de entorno
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

// Establecer variables de entorno
process.env.API_TOKEN = "your-token";
process.env.API_KEY = "your-key";
```

```python Python
# .mcp.json con variables de entorno
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

# Establecer variables de entorno
import os
os.environ["API_TOKEN"] = "your-token"
os.environ["API_KEY"] = "your-key"
```

</CodeGroup>

### Autenticación OAuth2

La autenticación OAuth2 MCP en el cliente no está actualmente soportada.

## Manejo de errores

Maneja las fallas de conexión MCP de manera elegante:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Procesar datos",
  options: {
    mcpServers: {
      "data-processor": dataServer
    }
  }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // Verificar estado del servidor MCP
    const failedServers = message.mcp_servers.filter(
      s => s.status !== "connected"
    );
    
    if (failedServers.length > 0) {
      console.warn("Falló la conexión:", failedServers);
    }
  }
  
  if (message.type === "result" && message.subtype === "error_during_execution") {
    console.error("Falló la ejecución");
  }
}
```

```python Python
from claude_agent_sdk import query

async for message in query(
    prompt="Procesar datos",
    options={
        "mcpServers": {
            "data-processor": data_server
        }
    }
):
    if message["type"] == "system" and message["subtype"] == "init":
        # Verificar estado del servidor MCP
        failed_servers = [
            s for s in message["mcp_servers"]
            if s["status"] != "connected"
        ]
        
        if failed_servers:
            print(f"Falló la conexión: {failed_servers}")
    
    if message["type"] == "result" and message["subtype"] == "error_during_execution":
        print("Falló la ejecución")
```

</CodeGroup>

## Recursos relacionados

- [Guía de herramientas personalizadas](/docs/es/agent-sdk/custom-tools) - Guía detallada sobre crear servidores MCP del SDK
- [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript)
- [Referencia del SDK de Python](/docs/es/agent-sdk/python)
- [Permisos del SDK](/docs/es/agent-sdk/permissions)
- [Flujos de trabajo comunes](https://code.claude.com/docs/common-workflows)