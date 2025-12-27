# Herramientas Personalizadas

Construye e integra herramientas personalizadas para extender la funcionalidad del SDK de Claude Agent

---

Las herramientas personalizadas te permiten extender las capacidades de Claude Code con tu propia funcionalidad a través de servidores MCP en proceso, permitiendo que Claude interactúe con servicios externos, APIs, o realice operaciones especializadas.

## Creando Herramientas Personalizadas

Usa las funciones auxiliares `createSdkMcpServer` y `tool` para definir herramientas personalizadas con seguridad de tipos:

<CodeGroup>

```typescript TypeScript
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

// Crear un servidor MCP del SDK con herramientas personalizadas
const customServer = createSdkMcpServer({
  name: "my-custom-tools",
  version: "1.0.0",
  tools: [
    tool(
      "get_weather",
      "Obtener la temperatura actual de una ubicación usando coordenadas",
      {
        latitude: z.number().describe("Coordenada de latitud"),
        longitude: z.number().describe("Coordenada de longitud")
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

# Definir una herramienta personalizada usando el decorador @tool
@tool("get_weather", "Obtener la temperatura actual de una ubicación usando coordenadas", {"latitude": float, "longitude": float})
async def get_weather(args: dict[str, Any]) -> dict[str, Any]:
    # Llamar a la API del clima
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

# Crear un servidor MCP del SDK con la herramienta personalizada
custom_server = create_sdk_mcp_server(
    name="my-custom-tools",
    version="1.0.0",
    tools=[get_weather]  # Pasar la función decorada
)
```

</CodeGroup>

## Usando Herramientas Personalizadas

Pasa el servidor personalizado a la función `query` a través de la opción `mcpServers` como un diccionario/objeto.

<Note>
**Importante:** Las herramientas MCP personalizadas requieren modo de entrada de streaming. Debes usar un generador asíncrono/iterable para el parámetro `prompt` - una cadena simple no funcionará con servidores MCP.
</Note>

### Formato del Nombre de la Herramienta

Cuando las herramientas MCP se exponen a Claude, sus nombres siguen un formato específico:
- Patrón: `mcp__{server_name}__{tool_name}`
- Ejemplo: Una herramienta llamada `get_weather` en el servidor `my-custom-tools` se convierte en `mcp__my-custom-tools__get_weather`

### Configurando Herramientas Permitidas

Puedes controlar qué herramientas puede usar Claude a través de la opción `allowedTools`:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Usar las herramientas personalizadas en tu consulta con entrada de streaming
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "¿Cuál es el clima en San Francisco?"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // Usar generador asíncrono para entrada de streaming
  options: {
    mcpServers: {
      "my-custom-tools": customServer  // Pasar como objeto/diccionario, no como array
    },
    // Opcionalmente especificar qué herramientas puede usar Claude
    allowedTools: [
      "mcp__my-custom-tools__get_weather",  // Permitir la herramienta del clima
      // Agregar otras herramientas según sea necesario
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

# Usar las herramientas personalizadas con Claude
options = ClaudeAgentOptions(
    mcp_servers={"my-custom-tools": custom_server},
    allowed_tools=[
        "mcp__my-custom-tools__get_weather",  # Permitir la herramienta del clima
        # Agregar otras herramientas según sea necesario
    ]
)

async def main():
    async with ClaudeSDKClient(options=options) as client:
        await client.query("¿Cuál es el clima en San Francisco?")

        # Extraer e imprimir respuesta
        async for msg in client.receive_response():
            print(msg)

asyncio.run(main())
```

</CodeGroup>

### Ejemplo de Múltiples Herramientas

Cuando tu servidor MCP tiene múltiples herramientas, puedes permitirlas selectivamente:

<CodeGroup>

```typescript TypeScript
const multiToolServer = createSdkMcpServer({
  name: "utilities",
  version: "1.0.0",
  tools: [
    tool("calculate", "Realizar cálculos", { /* ... */ }, async (args) => { /* ... */ }),
    tool("translate", "Traducir texto", { /* ... */ }, async (args) => { /* ... */ }),
    tool("search_web", "Buscar en la web", { /* ... */ }, async (args) => { /* ... */ })
  ]
});

// Permitir solo herramientas específicas con entrada de streaming
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "Calcula 5 + 3 y traduce 'hello' al español"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // Usar generador asíncrono para entrada de streaming
  options: {
    mcpServers: {
      utilities: multiToolServer
    },
    allowedTools: [
      "mcp__utilities__calculate",   // Permitir calculadora
      "mcp__utilities__translate",   // Permitir traductor
      // "mcp__utilities__search_web" NO está permitido
    ]
  }
})) {
  // Procesar mensajes
}
```

```python Python
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, tool, create_sdk_mcp_server
from typing import Any
import asyncio

# Definir múltiples herramientas usando el decorador @tool
@tool("calculate", "Realizar cálculos", {"expression": str})
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    result = eval(args["expression"])  # Usar eval seguro en producción
    return {"content": [{"type": "text", "text": f"Resultado: {result}"}]}

@tool("translate", "Traducir texto", {"text": str, "target_lang": str})
async def translate(args: dict[str, Any]) -> dict[str, Any]:
    # Lógica de traducción aquí
    return {"content": [{"type": "text", "text": f"Traducido: {args['text']}"}]}

@tool("search_web", "Buscar en la web", {"query": str})
async def search_web(args: dict[str, Any]) -> dict[str, Any]:
    # Lógica de búsqueda aquí
    return {"content": [{"type": "text", "text": f"Resultados de búsqueda para: {args['query']}"}]}

multi_tool_server = create_sdk_mcp_server(
    name="utilities",
    version="1.0.0",
    tools=[calculate, translate, search_web]  # Pasar funciones decoradas
)

# Permitir solo herramientas específicas con entrada de streaming
async def message_generator():
    yield {
        "type": "user",
        "message": {
            "role": "user",
            "content": "Calcula 5 + 3 y traduce 'hello' al español"
        }
    }

async for message in query(
    prompt=message_generator(),  # Usar generador asíncrono para entrada de streaming
    options=ClaudeAgentOptions(
        mcp_servers={"utilities": multi_tool_server},
        allowed_tools=[
            "mcp__utilities__calculate",   # Permitir calculadora
            "mcp__utilities__translate",   # Permitir traductor
            # "mcp__utilities__search_web" NO está permitido
        ]
    )
):
    if hasattr(message, 'result'):
        print(message.result)
```

</CodeGroup>

## Seguridad de Tipos con Python

El decorador `@tool` soporta varios enfoques de definición de esquemas para seguridad de tipos:

<CodeGroup>

```typescript TypeScript
import { z } from "zod";

tool(
  "process_data",
  "Procesar datos estructurados con seguridad de tipos",
  {
    // El esquema Zod define tanto la validación en tiempo de ejecución como los tipos de TypeScript
    data: z.object({
      name: z.string(),
      age: z.number().min(0).max(150),
      email: z.string().email(),
      preferences: z.array(z.string()).optional()
    }),
    format: z.enum(["json", "csv", "xml"]).default("json")
  },
  async (args) => {
    // args está completamente tipado basado en el esquema
    // TypeScript sabe: args.data.name es string, args.data.age es number, etc.
    console.log(`Procesando los datos de ${args.data.name} como ${args.format}`);
    
    // Tu lógica de procesamiento aquí
    return {
      content: [{
        type: "text",
        text: `Datos procesados para ${args.data.name}`
      }]
    };
  }
)
```

```python Python
from typing import Any

# Mapeo de tipos simple - recomendado para la mayoría de casos
@tool(
    "process_data",
    "Procesar datos estructurados con seguridad de tipos",
    {
        "name": str,
        "age": int,
        "email": str,
        "preferences": list  # Los parámetros opcionales se pueden manejar en la función
    }
)
async def process_data(args: dict[str, Any]) -> dict[str, Any]:
    # Acceder argumentos con sugerencias de tipo para soporte del IDE
    name = args["name"]
    age = args["age"]
    email = args["email"]
    preferences = args.get("preferences", [])
    
    print(f"Procesando los datos de {name} (edad: {age})")
    
    return {
        "content": [{
            "type": "text",
            "text": f"Datos procesados para {name}"
        }]
    }

# Para esquemas más complejos, puedes usar el formato JSON Schema
@tool(
    "advanced_process",
    "Procesar datos con validación avanzada",
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
    # Procesar con validación de esquema avanzada
    return {
        "content": [{
            "type": "text",
            "text": f"Procesamiento avanzado para {args['name']}"
        }]
    }
```

</CodeGroup>

## Manejo de Errores

Maneja errores de manera elegante para proporcionar retroalimentación significativa:

<CodeGroup>

```typescript TypeScript
tool(
  "fetch_data",
  "Obtener datos de una API",
  {
    endpoint: z.string().url().describe("URL del endpoint de la API")
  },
  async (args) => {
    try {
      const response = await fetch(args.endpoint);
      
      if (!response.ok) {
        return {
          content: [{
            type: "text",
            text: `Error de API: ${response.status} ${response.statusText}`
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
          text: `Falló al obtener datos: ${error.message}`
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
    "Obtener datos de una API",
    {"endpoint": str}  # Esquema simple
)
async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
    try:
        async with aiohttp.ClientSession() as session:
            async with session.get(args["endpoint"]) as response:
                if response.status != 200:
                    return {
                        "content": [{
                            "type": "text",
                            "text": f"Error de API: {response.status} {response.reason}"
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
                "text": f"Falló al obtener datos: {str(e)}"
            }]
        }
```

</CodeGroup>

## Herramientas de Ejemplo

### Herramienta de Consulta de Base de Datos

<CodeGroup>

```typescript TypeScript
const databaseServer = createSdkMcpServer({
  name: "database-tools",
  version: "1.0.0",
  tools: [
    tool(
      "query_database",
      "Ejecutar una consulta de base de datos",
      {
        query: z.string().describe("Consulta SQL a ejecutar"),
        params: z.array(z.any()).optional().describe("Parámetros de la consulta")
      },
      async (args) => {
        const results = await db.query(args.query, args.params || []);
        return {
          content: [{
            type: "text",
            text: `Se encontraron ${results.length} filas:\n${JSON.stringify(results, null, 2)}`
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
    "Ejecutar una consulta de base de datos",
    {"query": str, "params": list}  # Esquema simple con tipo list
)
async def query_database(args: dict[str, Any]) -> dict[str, Any]:
    results = await db.query(args["query"], args.get("params", []))
    return {
        "content": [{
            "type": "text",
            "text": f"Se encontraron {len(results)} filas:\n{json.dumps(results, indent=2)}"
        }]
    }

database_server = create_sdk_mcp_server(
    name="database-tools",
    version="1.0.0",
    tools=[query_database]  # Pasar la función decorada
)
```

</CodeGroup>

### Herramienta de Gateway de API

<CodeGroup>

```typescript TypeScript
const apiGatewayServer = createSdkMcpServer({
  name: "api-gateway",
  version: "1.0.0",
  tools: [
    tool(
      "api_request",
      "Hacer solicitudes API autenticadas a servicios externos",
      {
        service: z.enum(["stripe", "github", "openai", "slack"]).describe("Servicio a llamar"),
        endpoint: z.string().describe("Ruta del endpoint de la API"),
        method: z.enum(["GET", "POST", "PUT", "DELETE"]).describe("Método HTTP"),
        body: z.record(z.any()).optional().describe("Cuerpo de la solicitud"),
        query: z.record(z.string()).optional().describe("Parámetros de consulta")
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

# Para esquemas complejos con enums, usar formato JSON Schema
@tool(
    "api_request",
    "Hacer solicitudes API autenticadas a servicios externos",
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
    tools=[api_request]  # Pasar la función decorada
)
```

</CodeGroup>

### Herramienta de Calculadora

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
        expression: z.string().describe("Expresión matemática a evaluar"),
        precision: z.number().optional().default(2).describe("Precisión decimal")
      },
      async (args) => {
        try {
          // Usar una librería de evaluación matemática segura en producción
          const result = eval(args.expression); // ¡Solo ejemplo!
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
              text: `Error: Expresión inválida - ${error.message}`
            }]
          };
        }
      }
    ),
    tool(
      "compound_interest",
      "Calcular interés compuesto para una inversión",
      {
        principal: z.number().positive().describe("Monto de inversión inicial"),
        rate: z.number().describe("Tasa de interés anual (como decimal, ej. 0.05 para 5%)"),
        time: z.number().positive().describe("Período de inversión en años"),
        n: z.number().positive().default(12).describe("Frecuencia de capitalización por año")
      },
      async (args) => {
        const amount = args.principal * Math.pow(1 + args.rate / args.n, args.n * args.time);
        const interest = amount - args.principal;
        
        return {
          content: [{
            type: "text",
            text: `Análisis de Inversión:\n` +
                  `Principal: $${args.principal.toFixed(2)}\n` +
                  `Tasa: ${(args.rate * 100).toFixed(2)}%\n` +
                  `Tiempo: ${args.time} años\n` +
                  `Capitalización: ${args.n} veces por año\n\n` +
                  `Monto Final: $${amount.toFixed(2)}\n` +
                  `Interés Ganado: $${interest.toFixed(2)}\n` +
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
    {"expression": str, "precision": int}  # Esquema simple
)
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    try:
        # Usar una librería de evaluación matemática segura en producción
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
                "text": f"Error: Expresión inválida - {str(e)}"
            }]
        }

@tool(
    "compound_interest",
    "Calcular interés compuesto para una inversión",
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
            "text": f"""Análisis de Inversión:
Principal: ${principal:.2f}
Tasa: {rate * 100:.2f}%
Tiempo: {time} años
Capitalización: {n} veces por año

Monto Final: ${amount:.2f}
Interés Ganado: ${interest:.2f}
Retorno: {(interest / principal) * 100:.2f}%"""
        }]
    }

calculator_server = create_sdk_mcp_server(
    name="calculator",
    version="1.0.0",
    tools=[calculate, compound_interest]  # Pasar funciones decoradas
)
```

</CodeGroup>

## Documentación Relacionada

- [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript)
- [Referencia del SDK de Python](/docs/es/agent-sdk/python)
- [Documentación de MCP](https://modelcontextprotocol.io)
- [Resumen del SDK](/docs/es/agent-sdk/overview)