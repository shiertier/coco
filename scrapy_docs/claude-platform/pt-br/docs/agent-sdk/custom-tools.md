# Ferramentas Personalizadas

Construa e integre ferramentas personalizadas para estender a funcionalidade do Claude Agent SDK

---

As ferramentas personalizadas permitem que você estenda as capacidades do Claude Code com sua própria funcionalidade através de servidores MCP em processo, permitindo que o Claude interaja com serviços externos, APIs ou execute operações especializadas.

## Criando Ferramentas Personalizadas

Use as funções auxiliares `createSdkMcpServer` e `tool` para definir ferramentas personalizadas com segurança de tipos:

<CodeGroup>

```typescript TypeScript
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

// Criar um servidor MCP SDK com ferramentas personalizadas
const customServer = createSdkMcpServer({
  name: "my-custom-tools",
  version: "1.0.0",
  tools: [
    tool(
      "get_weather",
      "Obter temperatura atual para uma localização usando coordenadas",
      {
        latitude: z.number().describe("Coordenada de latitude"),
        longitude: z.number().describe("Coordenada de longitude")
      },
      async (args) => {
        const response = await fetch(`https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&current=temperature_2m&temperature_unit=fahrenheit`);
        const data = await response.json();

        return {
          content: [{
            type: "text",
            text: `Temperatura: ${data.current.temperature_2m}°F`
          }]
        };
      }
    )
  ]
});
```

```python Python
from claude_agent_sdk import tool, create_sdk_mcp_server, ClaudeSDKClient, ClaudeAgentOptions
from typing import Any
import aiohttp

# Definir uma ferramenta personalizada usando o decorador @tool
@tool("get_weather", "Obter temperatura atual para uma localização usando coordenadas", {"latitude": float, "longitude": float})
async def get_weather(args: dict[str, Any]) -> dict[str, Any]:
    # Chamar API do clima
    async with aiohttp.ClientSession() as session:
        async with session.get(
            f"https://api.open-meteo.com/v1/forecast?latitude={args['latitude']}&longitude={args['longitude']}&current=temperature_2m&temperature_unit=fahrenheit"
        ) as response:
            data = await response.json()

    return {
        "content": [{
            "type": "text",
            "text": f"Temperatura: {data['current']['temperature_2m']}°F"
        }]
    }

# Criar um servidor MCP SDK com a ferramenta personalizada
custom_server = create_sdk_mcp_server(
    name="my-custom-tools",
    version="1.0.0",
    tools=[get_weather]  # Passar a função decorada
)
```

</CodeGroup>

## Usando Ferramentas Personalizadas

Passe o servidor personalizado para a função `query` através da opção `mcpServers` como um dicionário/objeto.

<Note>
**Importante:** Ferramentas MCP personalizadas requerem modo de entrada de streaming. Você deve usar um gerador assíncrono/iterável para o parâmetro `prompt` - uma string simples não funcionará com servidores MCP.
</Note>

### Formato do Nome da Ferramenta

Quando as ferramentas MCP são expostas ao Claude, seus nomes seguem um formato específico:
- Padrão: `mcp__{nome_do_servidor}__{nome_da_ferramenta}`
- Exemplo: Uma ferramenta chamada `get_weather` no servidor `my-custom-tools` torna-se `mcp__my-custom-tools__get_weather`

### Configurando Ferramentas Permitidas

Você pode controlar quais ferramentas o Claude pode usar através da opção `allowedTools`:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Usar as ferramentas personalizadas em sua consulta com entrada de streaming
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "Qual é o clima em São Francisco?"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // Usar gerador assíncrono para entrada de streaming
  options: {
    mcpServers: {
      "my-custom-tools": customServer  // Passar como objeto/dicionário, não array
    },
    // Opcionalmente especificar quais ferramentas o Claude pode usar
    allowedTools: [
      "mcp__my-custom-tools__get_weather",  // Permitir a ferramenta do clima
      // Adicionar outras ferramentas conforme necessário
    ],
    maxTurns: 3
  }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

```python Python
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions
import asyncio

# Usar as ferramentas personalizadas com o Claude
options = ClaudeAgentOptions(
    mcp_servers={"my-custom-tools": custom_server},
    allowed_tools=[
        "mcp__my-custom-tools__get_weather",  # Permitir a ferramenta do clima
        # Adicionar outras ferramentas conforme necessário
    ]
)

async def main():
    async with ClaudeSDKClient(options=options) as client:
        await client.query("Qual é o clima em São Francisco?")

        # Extrair e imprimir resposta
        async for msg in client.receive_response():
            print(msg)

asyncio.run(main())
```

</CodeGroup>

### Exemplo de Múltiplas Ferramentas

Quando seu servidor MCP tem múltiplas ferramentas, você pode permitir seletivamente:

<CodeGroup>

```typescript TypeScript
const multiToolServer = createSdkMcpServer({
  name: "utilities",
  version: "1.0.0",
  tools: [
    tool("calculate", "Realizar cálculos", { /* ... */ }, async (args) => { /* ... */ }),
    tool("translate", "Traduzir texto", { /* ... */ }, async (args) => { /* ... */ }),
    tool("search_web", "Pesquisar na web", { /* ... */ }, async (args) => { /* ... */ })
  ]
});

// Permitir apenas ferramentas específicas com entrada de streaming
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "Calcule 5 + 3 e traduza 'olá' para espanhol"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // Usar gerador assíncrono para entrada de streaming
  options: {
    mcpServers: {
      utilities: multiToolServer
    },
    allowedTools: [
      "mcp__utilities__calculate",   // Permitir calculadora
      "mcp__utilities__translate",   // Permitir tradutor
      // "mcp__utilities__search_web" NÃO é permitido
    ]
  }
})) {
  // Processar mensagens
}
```

```python Python
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, tool, create_sdk_mcp_server
from typing import Any
import asyncio

# Definir múltiplas ferramentas usando o decorador @tool
@tool("calculate", "Realizar cálculos", {"expression": str})
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    result = eval(args["expression"])  # Use eval seguro em produção
    return {"content": [{"type": "text", "text": f"Resultado: {result}"}]}

@tool("translate", "Traduzir texto", {"text": str, "target_lang": str})
async def translate(args: dict[str, Any]) -> dict[str, Any]:
    # Lógica de tradução aqui
    return {"content": [{"type": "text", "text": f"Traduzido: {args['text']}"}]}

@tool("search_web", "Pesquisar na web", {"query": str})
async def search_web(args: dict[str, Any]) -> dict[str, Any]:
    # Lógica de pesquisa aqui
    return {"content": [{"type": "text", "text": f"Resultados de pesquisa para: {args['query']}"}]}

multi_tool_server = create_sdk_mcp_server(
    name="utilities",
    version="1.0.0",
    tools=[calculate, translate, search_web]  # Passar funções decoradas
)

# Permitir apenas ferramentas específicas com entrada de streaming
async def message_generator():
    yield {
        "type": "user",
        "message": {
            "role": "user",
            "content": "Calcule 5 + 3 e traduza 'olá' para espanhol"
        }
    }

async for message in query(
    prompt=message_generator(),  # Usar gerador assíncrono para entrada de streaming
    options=ClaudeAgentOptions(
        mcp_servers={"utilities": multi_tool_server},
        allowed_tools=[
            "mcp__utilities__calculate",   # Permitir calculadora
            "mcp__utilities__translate",   # Permitir tradutor
            # "mcp__utilities__search_web" NÃO é permitido
        ]
    )
):
    if hasattr(message, 'result'):
        print(message.result)
```

</CodeGroup>

## Segurança de Tipos com Python

O decorador `@tool` suporta várias abordagens de definição de esquema para segurança de tipos:

<CodeGroup>

```typescript TypeScript
import { z } from "zod";

tool(
  "process_data",
  "Processar dados estruturados com segurança de tipos",
  {
    // Esquema Zod define tanto validação em tempo de execução quanto tipos TypeScript
    data: z.object({
      name: z.string(),
      age: z.number().min(0).max(150),
      email: z.string().email(),
      preferences: z.array(z.string()).optional()
    }),
    format: z.enum(["json", "csv", "xml"]).default("json")
  },
  async (args) => {
    // args é totalmente tipado baseado no esquema
    // TypeScript sabe: args.data.name é string, args.data.age é number, etc.
    console.log(`Processando dados de ${args.data.name} como ${args.format}`);
    
    // Sua lógica de processamento aqui
    return {
      content: [{
        type: "text",
        text: `Dados processados para ${args.data.name}`
      }]
    };
  }
)
```

```python Python
from typing import Any

# Mapeamento de tipo simples - recomendado para a maioria dos casos
@tool(
    "process_data",
    "Processar dados estruturados com segurança de tipos",
    {
        "name": str,
        "age": int,
        "email": str,
        "preferences": list  # Parâmetros opcionais podem ser tratados na função
    }
)
async def process_data(args: dict[str, Any]) -> dict[str, Any]:
    # Acessar argumentos com dicas de tipo para suporte do IDE
    name = args["name"]
    age = args["age"]
    email = args["email"]
    preferences = args.get("preferences", [])
    
    print(f"Processando dados de {name} (idade: {age})")
    
    return {
        "content": [{
            "type": "text",
            "text": f"Dados processados para {name}"
        }]
    }

# Para esquemas mais complexos, você pode usar formato JSON Schema
@tool(
    "advanced_process",
    "Processar dados com validação avançada",
    {
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "integer", "minimum": 0, "maximum": 150},
            "email": {"type": "string", "format": "email"},
            "format": {"type": "string", "enum": ["json", "csv", "xml"], "default": "json"}
        },
        "required": ["name", "age", "email"]
    }
)
async def advanced_process(args: dict[str, Any]) -> dict[str, Any]:
    # Processar com validação de esquema avançada
    return {
        "content": [{
            "type": "text",
            "text": f"Processamento avançado para {args['name']}"
        }]
    }
```

</CodeGroup>

## Tratamento de Erros

Trate erros graciosamente para fornecer feedback significativo:

<CodeGroup>

```typescript TypeScript
tool(
  "fetch_data",
  "Buscar dados de uma API",
  {
    endpoint: z.string().url().describe("URL do endpoint da API")
  },
  async (args) => {
    try {
      const response = await fetch(args.endpoint);
      
      if (!response.ok) {
        return {
          content: [{
            type: "text",
            text: `Erro da API: ${response.status} ${response.statusText}`
          }]
        };
      }
      
      const data = await response.json();
      return {
        content: [{
          type: "text",
          text: JSON.stringify(data, null, 2)
        }]
      };
    } catch (error) {
      return {
        content: [{
          type: "text",
          text: `Falha ao buscar dados: ${error.message}`
        }]
      };
    }
  }
)
```

```python Python
import json
import aiohttp
from typing import Any

@tool(
    "fetch_data",
    "Buscar dados de uma API",
    {"endpoint": str}  # Esquema simples
)
async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
    try:
        async with aiohttp.ClientSession() as session:
            async with session.get(args["endpoint"]) as response:
                if response.status != 200:
                    return {
                        "content": [{
                            "type": "text",
                            "text": f"Erro da API: {response.status} {response.reason}"
                        }]
                    }
                
                data = await response.json()
                return {
                    "content": [{
                        "type": "text",
                        "text": json.dumps(data, indent=2)
                    }]
                }
    except Exception as e:
        return {
            "content": [{
                "type": "text",
                "text": f"Falha ao buscar dados: {str(e)}"
            }]
        }
```

</CodeGroup>

## Ferramentas de Exemplo

### Ferramenta de Consulta de Banco de Dados

<CodeGroup>

```typescript TypeScript
const databaseServer = createSdkMcpServer({
  name: "database-tools",
  version: "1.0.0",
  tools: [
    tool(
      "query_database",
      "Executar uma consulta de banco de dados",
      {
        query: z.string().describe("Consulta SQL para executar"),
        params: z.array(z.any()).optional().describe("Parâmetros da consulta")
      },
      async (args) => {
        const results = await db.query(args.query, args.params || []);
        return {
          content: [{
            type: "text",
            text: `Encontradas ${results.length} linhas:\n${JSON.stringify(results, null, 2)}`
          }]
        };
      }
    )
  ]
});
```

```python Python
from typing import Any
import json

@tool(
    "query_database",
    "Executar uma consulta de banco de dados",
    {"query": str, "params": list}  # Esquema simples com tipo list
)
async def query_database(args: dict[str, Any]) -> dict[str, Any]:
    results = await db.query(args["query"], args.get("params", []))
    return {
        "content": [{
            "type": "text",
            "text": f"Encontradas {len(results)} linhas:\n{json.dumps(results, indent=2)}"
        }]
    }

database_server = create_sdk_mcp_server(
    name="database-tools",
    version="1.0.0",
    tools=[query_database]  # Passar a função decorada
)
```

</CodeGroup>

### Ferramenta de Gateway de API

<CodeGroup>

```typescript TypeScript
const apiGatewayServer = createSdkMcpServer({
  name: "api-gateway",
  version: "1.0.0",
  tools: [
    tool(
      "api_request",
      "Fazer requisições de API autenticadas para serviços externos",
      {
        service: z.enum(["stripe", "github", "openai", "slack"]).describe("Serviço para chamar"),
        endpoint: z.string().describe("Caminho do endpoint da API"),
        method: z.enum(["GET", "POST", "PUT", "DELETE"]).describe("Método HTTP"),
        body: z.record(z.any()).optional().describe("Corpo da requisição"),
        query: z.record(z.string()).optional().describe("Parâmetros de consulta")
      },
      async (args) => {
        const config = {
          stripe: { baseUrl: "https://api.stripe.com/v1", key: process.env.STRIPE_KEY },
          github: { baseUrl: "https://api.github.com", key: process.env.GITHUB_TOKEN },
          openai: { baseUrl: "https://api.openai.com/v1", key: process.env.OPENAI_KEY },
          slack: { baseUrl: "https://slack.com/api", key: process.env.SLACK_TOKEN }
        };
        
        const { baseUrl, key } = config[args.service];
        const url = new URL(`${baseUrl}${args.endpoint}`);
        
        if (args.query) {
          Object.entries(args.query).forEach(([k, v]) => url.searchParams.set(k, v));
        }
        
        const response = await fetch(url, {
          method: args.method,
          headers: { Authorization: `Bearer ${key}`, "Content-Type": "application/json" },
          body: args.body ? JSON.stringify(args.body) : undefined
        });
        
        const data = await response.json();
        return {
          content: [{
            type: "text",
            text: JSON.stringify(data, null, 2)
          }]
        };
      }
    )
  ]
});
```

```python Python
import os
import json
import aiohttp
from typing import Any

# Para esquemas complexos com enums, use formato JSON Schema
@tool(
    "api_request",
    "Fazer requisições de API autenticadas para serviços externos",
    {
        "type": "object",
        "properties": {
            "service": {"type": "string", "enum": ["stripe", "github", "openai", "slack"]},
            "endpoint": {"type": "string"},
            "method": {"type": "string", "enum": ["GET", "POST", "PUT", "DELETE"]},
            "body": {"type": "object"},
            "query": {"type": "object"}
        },
        "required": ["service", "endpoint", "method"]
    }
)
async def api_request(args: dict[str, Any]) -> dict[str, Any]:
    config = {
        "stripe": {"base_url": "https://api.stripe.com/v1", "key": os.environ["STRIPE_KEY"]},
        "github": {"base_url": "https://api.github.com", "key": os.environ["GITHUB_TOKEN"]},
        "openai": {"base_url": "https://api.openai.com/v1", "key": os.environ["OPENAI_KEY"]},
        "slack": {"base_url": "https://slack.com/api", "key": os.environ["SLACK_TOKEN"]}
    }
    
    service_config = config[args["service"]]
    url = f"{service_config['base_url']}{args['endpoint']}"
    
    if args.get("query"):
        params = "&".join([f"{k}={v}" for k, v in args["query"].items()])
        url += f"?{params}"
    
    headers = {"Authorization": f"Bearer {service_config['key']}", "Content-Type": "application/json"}
    
    async with aiohttp.ClientSession() as session:
        async with session.request(
            args["method"], url, headers=headers, json=args.get("body")
        ) as response:
            data = await response.json()
            return {
                "content": [{
                    "type": "text",
                    "text": json.dumps(data, indent=2)
                }]
            }

api_gateway_server = create_sdk_mcp_server(
    name="api-gateway",
    version="1.0.0",
    tools=[api_request]  # Passar a função decorada
)
```

</CodeGroup>

### Ferramenta de Calculadora

<CodeGroup>

```typescript TypeScript
const calculatorServer = createSdkMcpServer({
  name: "calculator",
  version: "1.0.0",
  tools: [
    tool(
      "calculate",
      "Realizar cálculos matemáticos",
      {
        expression: z.string().describe("Expressão matemática para avaliar"),
        precision: z.number().optional().default(2).describe("Precisão decimal")
      },
      async (args) => {
        try {
          // Use uma biblioteca de avaliação matemática segura em produção
          const result = eval(args.expression); // Apenas exemplo!
          const formatted = Number(result).toFixed(args.precision);
          
          return {
            content: [{
              type: "text",
              text: `${args.expression} = ${formatted}`
            }]
          };
        } catch (error) {
          return {
            content: [{
              type: "text",
              text: `Erro: Expressão inválida - ${error.message}`
            }]
          };
        }
      }
    ),
    tool(
      "compound_interest",
      "Calcular juros compostos para um investimento",
      {
        principal: z.number().positive().describe("Valor do investimento inicial"),
        rate: z.number().describe("Taxa de juros anual (como decimal, ex: 0.05 para 5%)"),
        time: z.number().positive().describe("Período de investimento em anos"),
        n: z.number().positive().default(12).describe("Frequência de capitalização por ano")
      },
      async (args) => {
        const amount = args.principal * Math.pow(1 + args.rate / args.n, args.n * args.time);
        const interest = amount - args.principal;
        
        return {
          content: [{
            type: "text",
            text: `Análise de Investimento:\n` +
                  `Principal: R$${args.principal.toFixed(2)}\n` +
                  `Taxa: ${(args.rate * 100).toFixed(2)}%\n` +
                  `Tempo: ${args.time} anos\n` +
                  `Capitalização: ${args.n} vezes por ano\n\n` +
                  `Valor Final: R$${amount.toFixed(2)}\n` +
                  `Juros Ganhos: R$${interest.toFixed(2)}\n` +
                  `Retorno: ${((interest / args.principal) * 100).toFixed(2)}%`
          }]
        };
      }
    )
  ]
});
```

```python Python
import math
from typing import Any

@tool(
    "calculate",
    "Realizar cálculos matemáticos",
    {"expression": str, "precision": int}  # Esquema simples
)
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    try:
        # Use uma biblioteca de avaliação matemática segura em produção
        result = eval(args["expression"], {"__builtins__": {}})
        precision = args.get("precision", 2)
        formatted = round(result, precision)
        
        return {
            "content": [{
                "type": "text",
                "text": f"{args['expression']} = {formatted}"
            }]
        }
    except Exception as e:
        return {
            "content": [{
                "type": "text",
                "text": f"Erro: Expressão inválida - {str(e)}"
            }]
        }

@tool(
    "compound_interest",
    "Calcular juros compostos para um investimento",
    {"principal": float, "rate": float, "time": float, "n": int}
)
async def compound_interest(args: dict[str, Any]) -> dict[str, Any]:
    principal = args["principal"]
    rate = args["rate"]
    time = args["time"]
    n = args.get("n", 12)
    
    amount = principal * (1 + rate / n) ** (n * time)
    interest = amount - principal
    
    return {
        "content": [{
            "type": "text",
            "text": f"""Análise de Investimento:
Principal: R${principal:.2f}
Taxa: {rate * 100:.2f}%
Tempo: {time} anos
Capitalização: {n} vezes por ano

Valor Final: R${amount:.2f}
Juros Ganhos: R${interest:.2f}
Retorno: {(interest / principal) * 100:.2f}%"""
        }]
    }

calculator_server = create_sdk_mcp_server(
    name="calculator",
    version="1.0.0",
    tools=[calculate, compound_interest]  # Passar funções decoradas
)
```

</CodeGroup>

## Documentação Relacionada

- [Referência do SDK TypeScript](/docs/pt-BR/agent-sdk/typescript)
- [Referência do SDK Python](/docs/pt-BR/agent-sdk/python)
- [Documentação MCP](https://modelcontextprotocol.io)
- [Visão Geral do SDK](/docs/pt-BR/agent-sdk/overview)