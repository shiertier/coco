# Strumenti Personalizzati

Costruisci e integra strumenti personalizzati per estendere le funzionalità di Claude Agent SDK

---

Gli strumenti personalizzati ti permettono di estendere le capacità di Claude Code con le tue funzionalità attraverso server MCP in-process, consentendo a Claude di interagire con servizi esterni, API o eseguire operazioni specializzate.

## Creazione di Strumenti Personalizzati

Usa le funzioni helper `createSdkMcpServer` e `tool` per definire strumenti personalizzati type-safe:

<CodeGroup>

```typescript TypeScript
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

// Crea un server SDK MCP con strumenti personalizzati
const customServer = createSdkMcpServer({
  name: "my-custom-tools",
  version: "1.0.0",
  tools: [
    tool(
      "get_weather",
      "Ottieni la temperatura attuale per una località usando le coordinate",
      {
        latitude: z.number().describe("Coordinata di latitudine"),
        longitude: z.number().describe("Coordinata di longitudine")
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

# Definisci uno strumento personalizzato usando il decoratore @tool
@tool("get_weather", "Ottieni la temperatura attuale per una località usando le coordinate", {"latitude": float, "longitude": float})
async def get_weather(args: dict[str, Any]) -> dict[str, Any]:
    # Chiama l'API meteo
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

# Crea un server SDK MCP con lo strumento personalizzato
custom_server = create_sdk_mcp_server(
    name="my-custom-tools",
    version="1.0.0",
    tools=[get_weather]  # Passa la funzione decorata
)
```

</CodeGroup>

## Utilizzo di Strumenti Personalizzati

Passa il server personalizzato alla funzione `query` tramite l'opzione `mcpServers` come dizionario/oggetto.

<Note>
**Importante:** Gli strumenti MCP personalizzati richiedono la modalità di input streaming. Devi usare un generatore asincrono/iterabile per il parametro `prompt` - una semplice stringa non funzionerà con i server MCP.
</Note>

### Formato del Nome dello Strumento

Quando gli strumenti MCP vengono esposti a Claude, i loro nomi seguono un formato specifico:
- Pattern: `mcp__{server_name}__{tool_name}`
- Esempio: Uno strumento chiamato `get_weather` nel server `my-custom-tools` diventa `mcp__my-custom-tools__get_weather`

### Configurazione degli Strumenti Consentiti

Puoi controllare quali strumenti Claude può usare tramite l'opzione `allowedTools`:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Usa gli strumenti personalizzati nella tua query con input streaming
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "Che tempo fa a San Francisco?"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // Usa generatore asincrono per input streaming
  options: {
    mcpServers: {
      "my-custom-tools": customServer  // Passa come oggetto/dizionario, non array
    },
    // Opzionalmente specifica quali strumenti Claude può usare
    allowedTools: [
      "mcp__my-custom-tools__get_weather",  // Consenti lo strumento meteo
      // Aggiungi altri strumenti se necessario
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

# Usa gli strumenti personalizzati con Claude
options = ClaudeAgentOptions(
    mcp_servers={"my-custom-tools": custom_server},
    allowed_tools=[
        "mcp__my-custom-tools__get_weather",  # Consenti lo strumento meteo
        # Aggiungi altri strumenti se necessario
    ]
)

async def main():
    async with ClaudeSDKClient(options=options) as client:
        await client.query("Che tempo fa a San Francisco?")

        # Estrai e stampa la risposta
        async for msg in client.receive_response():
            print(msg)

asyncio.run(main())
```

</CodeGroup>

### Esempio con Strumenti Multipli

Quando il tuo server MCP ha strumenti multipli, puoi consentirli selettivamente:

<CodeGroup>

```typescript TypeScript
const multiToolServer = createSdkMcpServer({
  name: "utilities",
  version: "1.0.0",
  tools: [
    tool("calculate", "Esegui calcoli", { /* ... */ }, async (args) => { /* ... */ }),
    tool("translate", "Traduci testo", { /* ... */ }, async (args) => { /* ... */ }),
    tool("search_web", "Cerca nel web", { /* ... */ }, async (args) => { /* ... */ })
  ]
});

// Consenti solo strumenti specifici con input streaming
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "Calcola 5 + 3 e traduci 'ciao' in spagnolo"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // Usa generatore asincrono per input streaming
  options: {
    mcpServers: {
      utilities: multiToolServer
    },
    allowedTools: [
      "mcp__utilities__calculate",   // Consenti calcolatrice
      "mcp__utilities__translate",   // Consenti traduttore
      // "mcp__utilities__search_web" NON è consentito
    ]
  }
})) {
  // Elabora messaggi
}
```

```python Python
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, tool, create_sdk_mcp_server
from typing import Any
import asyncio

# Definisci strumenti multipli usando il decoratore @tool
@tool("calculate", "Esegui calcoli", {"expression": str})
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    result = eval(args["expression"])  # Usa eval sicuro in produzione
    return {"content": [{"type": "text", "text": f"Risultato: {result}"}]}

@tool("translate", "Traduci testo", {"text": str, "target_lang": str})
async def translate(args: dict[str, Any]) -> dict[str, Any]:
    # Logica di traduzione qui
    return {"content": [{"type": "text", "text": f"Tradotto: {args['text']}"}]}

@tool("search_web", "Cerca nel web", {"query": str})
async def search_web(args: dict[str, Any]) -> dict[str, Any]:
    # Logica di ricerca qui
    return {"content": [{"type": "text", "text": f"Risultati di ricerca per: {args['query']}"}]}

multi_tool_server = create_sdk_mcp_server(
    name="utilities",
    version="1.0.0",
    tools=[calculate, translate, search_web]  # Passa funzioni decorate
)

# Consenti solo strumenti specifici con input streaming
async def message_generator():
    yield {
        "type": "user",
        "message": {
            "role": "user",
            "content": "Calcola 5 + 3 e traduci 'ciao' in spagnolo"
        }
    }

async for message in query(
    prompt=message_generator(),  # Usa generatore asincrono per input streaming
    options=ClaudeAgentOptions(
        mcp_servers={"utilities": multi_tool_server},
        allowed_tools=[
            "mcp__utilities__calculate",   # Consenti calcolatrice
            "mcp__utilities__translate",   # Consenti traduttore
            # "mcp__utilities__search_web" NON è consentito
        ]
    )
):
    if hasattr(message, 'result'):
        print(message.result)
```

</CodeGroup>

## Type Safety con Python

Il decoratore `@tool` supporta vari approcci di definizione dello schema per la type safety:

<CodeGroup>

```typescript TypeScript
import { z } from "zod";

tool(
  "process_data",
  "Elabora dati strutturati con type safety",
  {
    // Lo schema Zod definisce sia la validazione runtime che i tipi TypeScript
    data: z.object({
      name: z.string(),
      age: z.number().min(0).max(150),
      email: z.string().email(),
      preferences: z.array(z.string()).optional()
    }),
    format: z.enum(["json", "csv", "xml"]).default("json")
  },
  async (args) => {
    // args è completamente tipizzato basato sullo schema
    // TypeScript sa: args.data.name è string, args.data.age è number, ecc.
    console.log(`Elaborando i dati di ${args.data.name} come ${args.format}`);
    
    // La tua logica di elaborazione qui
    return {
      content: [{
        type: "text",
        text: `Dati elaborati per ${args.data.name}`
      }]
    };
  }
)
```

```python Python
from typing import Any

# Mappatura di tipo semplice - raccomandato per la maggior parte dei casi
@tool(
    "process_data",
    "Elabora dati strutturati con type safety",
    {
        "name": str,
        "age": int,
        "email": str,
        "preferences": list  # I parametri opzionali possono essere gestiti nella funzione
    }
)
async def process_data(args: dict[str, Any]) -> dict[str, Any]:
    # Accedi agli argomenti con suggerimenti di tipo per il supporto IDE
    name = args["name"]
    age = args["age"]
    email = args["email"]
    preferences = args.get("preferences", [])
    
    print(f"Elaborando i dati di {name} (età: {age})")
    
    return {
        "content": [{
            "type": "text",
            "text": f"Dati elaborati per {name}"
        }]
    }

# Per schemi più complessi, puoi usare il formato JSON Schema
@tool(
    "advanced_process",
    "Elabora dati con validazione avanzata",
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
    # Elabora con validazione schema avanzata
    return {
        "content": [{
            "type": "text",
            "text": f"Elaborazione avanzata per {args['name']}"
        }]
    }
```

</CodeGroup>

## Gestione degli Errori

Gestisci gli errori con grazia per fornire feedback significativo:

<CodeGroup>

```typescript TypeScript
tool(
  "fetch_data",
  "Recupera dati da un'API",
  {
    endpoint: z.string().url().describe("URL endpoint API")
  },
  async (args) => {
    try {
      const response = await fetch(args.endpoint);
      
      if (!response.ok) {
        return {
          content: [{
            type: "text",
            text: `Errore API: ${response.status} ${response.statusText}`
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
          text: `Impossibile recuperare dati: ${error.message}`
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
    "Recupera dati da un'API",
    {"endpoint": str}  # Schema semplice
)
async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
    try:
        async with aiohttp.ClientSession() as session:
            async with session.get(args["endpoint"]) as response:
                if response.status != 200:
                    return {
                        "content": [{
                            "type": "text",
                            "text": f"Errore API: {response.status} {response.reason}"
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
                "text": f"Impossibile recuperare dati: {str(e)}"
            }]
        }
```

</CodeGroup>

## Strumenti di Esempio

### Strumento di Query Database

<CodeGroup>

```typescript TypeScript
const databaseServer = createSdkMcpServer({
  name: "database-tools",
  version: "1.0.0",
  tools: [
    tool(
      "query_database",
      "Esegui una query database",
      {
        query: z.string().describe("Query SQL da eseguire"),
        params: z.array(z.any()).optional().describe("Parametri query")
      },
      async (args) => {
        const results = await db.query(args.query, args.params || []);
        return {
          content: [{
            type: "text",
            text: `Trovate ${results.length} righe:\n${JSON.stringify(results, null, 2)}`
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
    "Esegui una query database",
    {"query": str, "params": list}  # Schema semplice con tipo list
)
async def query_database(args: dict[str, Any]) -> dict[str, Any]:
    results = await db.query(args["query"], args.get("params", []))
    return {
        "content": [{
            "type": "text",
            "text": f"Trovate {len(results)} righe:\n{json.dumps(results, indent=2)}"
        }]
    }

database_server = create_sdk_mcp_server(
    name="database-tools",
    version="1.0.0",
    tools=[query_database]  # Passa la funzione decorata
)
```

</CodeGroup>

### Strumento API Gateway

<CodeGroup>

```typescript TypeScript
const apiGatewayServer = createSdkMcpServer({
  name: "api-gateway",
  version: "1.0.0",
  tools: [
    tool(
      "api_request",
      "Effettua richieste API autenticate a servizi esterni",
      {
        service: z.enum(["stripe", "github", "openai", "slack"]).describe("Servizio da chiamare"),
        endpoint: z.string().describe("Percorso endpoint API"),
        method: z.enum(["GET", "POST", "PUT", "DELETE"]).describe("Metodo HTTP"),
        body: z.record(z.any()).optional().describe("Corpo della richiesta"),
        query: z.record(z.string()).optional().describe("Parametri query")
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

# Per schemi complessi con enum, usa il formato JSON Schema
@tool(
    "api_request",
    "Effettua richieste API autenticate a servizi esterni",
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
    tools=[api_request]  # Passa la funzione decorata
)
```

</CodeGroup>

### Strumento Calcolatrice

<CodeGroup>

```typescript TypeScript
const calculatorServer = createSdkMcpServer({
  name: "calculator",
  version: "1.0.0",
  tools: [
    tool(
      "calculate",
      "Esegui calcoli matematici",
      {
        expression: z.string().describe("Espressione matematica da valutare"),
        precision: z.number().optional().default(2).describe("Precisione decimale")
      },
      async (args) => {
        try {
          // Usa una libreria di valutazione matematica sicura in produzione
          const result = eval(args.expression); // Solo esempio!
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
              text: `Errore: Espressione non valida - ${error.message}`
            }]
          };
        }
      }
    ),
    tool(
      "compound_interest",
      "Calcola l'interesse composto per un investimento",
      {
        principal: z.number().positive().describe("Importo investimento iniziale"),
        rate: z.number().describe("Tasso di interesse annuale (come decimale, es. 0.05 per 5%)"),
        time: z.number().positive().describe("Periodo di investimento in anni"),
        n: z.number().positive().default(12).describe("Frequenza di capitalizzazione per anno")
      },
      async (args) => {
        const amount = args.principal * Math.pow(1 + args.rate / args.n, args.n * args.time);
        const interest = amount - args.principal;
        
        return {
          content: [{
            type: "text",
            text: `Analisi Investimento:\n` +
                  `Capitale: $${args.principal.toFixed(2)}\n` +
                  `Tasso: ${(args.rate * 100).toFixed(2)}%\n` +
                  `Tempo: ${args.time} anni\n` +
                  `Capitalizzazione: ${args.n} volte per anno\n\n` +
                  `Importo Finale: $${amount.toFixed(2)}\n` +
                  `Interesse Guadagnato: $${interest.toFixed(2)}\n` +
                  `Rendimento: ${((interest / args.principal) * 100).toFixed(2)}%`
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
    "Esegui calcoli matematici",
    {"expression": str, "precision": int}  # Schema semplice
)
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    try:
        # Usa una libreria di valutazione matematica sicura in produzione
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
                "text": f"Errore: Espressione non valida - {str(e)}"
            }]
        }

@tool(
    "compound_interest",
    "Calcola l'interesse composto per un investimento",
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
            "text": f"""Analisi Investimento:
Capitale: ${principal:.2f}
Tasso: {rate * 100:.2f}%
Tempo: {time} anni
Capitalizzazione: {n} volte per anno

Importo Finale: ${amount:.2f}
Interesse Guadagnato: ${interest:.2f}
Rendimento: {(interest / principal) * 100:.2f}%"""
        }]
    }

calculator_server = create_sdk_mcp_server(
    name="calculator",
    version="1.0.0",
    tools=[calculate, compound_interest]  # Passa funzioni decorate
)
```

</CodeGroup>

## Documentazione Correlata

- [Riferimento SDK TypeScript](/docs/it/agent-sdk/typescript)
- [Riferimento SDK Python](/docs/it/agent-sdk/python)
- [Documentazione MCP](https://modelcontextprotocol.io)
- [Panoramica SDK](/docs/it/agent-sdk/overview)