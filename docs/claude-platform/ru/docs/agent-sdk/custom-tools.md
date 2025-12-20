# Пользовательские инструменты

Создавайте и интегрируйте пользовательские инструменты для расширения функциональности Claude Agent SDK

---

Пользовательские инструменты позволяют расширить возможности Claude Code с помощью собственной функциональности через внутрипроцессные MCP серверы, позволяя Claude взаимодействовать с внешними сервисами, API или выполнять специализированные операции.

## Создание пользовательских инструментов

Используйте вспомогательные функции `createSdkMcpServer` и `tool` для определения типобезопасных пользовательских инструментов:

<CodeGroup>

```typescript TypeScript
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

// Создание SDK MCP сервера с пользовательскими инструментами
const customServer = createSdkMcpServer({
  name: "my-custom-tools",
  version: "1.0.0",
  tools: [
    tool(
      "get_weather",
      "Получить текущую температуру для местоположения по координатам",
      {
        latitude: z.number().describe("Координата широты"),
        longitude: z.number().describe("Координата долготы")
      },
      async (args) => {
        const response = await fetch(`https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&current=temperature_2m&temperature_unit=fahrenheit`);
        const data = await response.json();

        return {
          content: [{
            type: "text",
            text: `Температура: ${data.current.temperature_2m}°F`
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

# Определение пользовательского инструмента с использованием декоратора @tool
@tool("get_weather", "Получить текущую температуру для местоположения по координатам", {"latitude": float, "longitude": float})
async def get_weather(args: dict[str, Any]) -> dict[str, Any]:
    # Вызов API погоды
    async with aiohttp.ClientSession() as session:
        async with session.get(
            f"https://api.open-meteo.com/v1/forecast?latitude={args['latitude']}&longitude={args['longitude']}&current=temperature_2m&temperature_unit=fahrenheit"
        ) as response:
            data = await response.json()

    return {
        "content": [{
            "type": "text",
            "text": f"Температура: {data['current']['temperature_2m']}°F"
        }]
    }

# Создание SDK MCP сервера с пользовательским инструментом
custom_server = create_sdk_mcp_server(
    name="my-custom-tools",
    version="1.0.0",
    tools=[get_weather]  # Передача декорированной функции
)
```

</CodeGroup>

## Использование пользовательских инструментов

Передайте пользовательский сервер в функцию `query` через опцию `mcpServers` в виде словаря/объекта.

<Note>
**Важно:** Пользовательские MCP инструменты требуют режима потокового ввода. Вы должны использовать асинхронный генератор/итерируемый объект для параметра `prompt` - простая строка не будет работать с MCP серверами.
</Note>

### Формат имени инструмента

Когда MCP инструменты предоставляются Claude, их имена следуют определенному формату:
- Шаблон: `mcp__{server_name}__{tool_name}`
- Пример: Инструмент с именем `get_weather` на сервере `my-custom-tools` становится `mcp__my-custom-tools__get_weather`

### Настройка разрешенных инструментов

Вы можете контролировать, какие инструменты может использовать Claude, через опцию `allowedTools`:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Использование пользовательских инструментов в запросе с потоковым вводом
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "Какая погода в Сан-Франциско?"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // Использование асинхронного генератора для потокового ввода
  options: {
    mcpServers: {
      "my-custom-tools": customServer  // Передача как объект/словарь, не массив
    },
    // Опционально указать, какие инструменты может использовать Claude
    allowedTools: [
      "mcp__my-custom-tools__get_weather",  // Разрешить инструмент погоды
      // Добавить другие инструменты по необходимости
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

# Использование пользовательских инструментов с Claude
options = ClaudeAgentOptions(
    mcp_servers={"my-custom-tools": custom_server},
    allowed_tools=[
        "mcp__my-custom-tools__get_weather",  # Разрешить инструмент погоды
        # Добавить другие инструменты по необходимости
    ]
)

async def main():
    async with ClaudeSDKClient(options=options) as client:
        await client.query("Какая погода в Сан-Франциско?")

        # Извлечение и вывод ответа
        async for msg in client.receive_response():
            print(msg)

asyncio.run(main())
```

</CodeGroup>

### Пример с несколькими инструментами

Когда ваш MCP сервер имеет несколько инструментов, вы можете выборочно разрешить их:

<CodeGroup>

```typescript TypeScript
const multiToolServer = createSdkMcpServer({
  name: "utilities",
  version: "1.0.0",
  tools: [
    tool("calculate", "Выполнить вычисления", { /* ... */ }, async (args) => { /* ... */ }),
    tool("translate", "Перевести текст", { /* ... */ }, async (args) => { /* ... */ }),
    tool("search_web", "Поиск в интернете", { /* ... */ }, async (args) => { /* ... */ })
  ]
});

// Разрешить только определенные инструменты с потоковым вводом
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "Вычисли 5 + 3 и переведи 'hello' на испанский"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // Использование асинхронного генератора для потокового ввода
  options: {
    mcpServers: {
      utilities: multiToolServer
    },
    allowedTools: [
      "mcp__utilities__calculate",   // Разрешить калькулятор
      "mcp__utilities__translate",   // Разрешить переводчик
      // "mcp__utilities__search_web" НЕ разрешен
    ]
  }
})) {
  // Обработка сообщений
}
```

```python Python
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, tool, create_sdk_mcp_server
from typing import Any
import asyncio

# Определение нескольких инструментов с использованием декоратора @tool
@tool("calculate", "Выполнить вычисления", {"expression": str})
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    result = eval(args["expression"])  # Используйте безопасный eval в продакшене
    return {"content": [{"type": "text", "text": f"Результат: {result}"}]}

@tool("translate", "Перевести текст", {"text": str, "target_lang": str})
async def translate(args: dict[str, Any]) -> dict[str, Any]:
    # Логика перевода здесь
    return {"content": [{"type": "text", "text": f"Переведено: {args['text']}"}]}

@tool("search_web", "Поиск в интернете", {"query": str})
async def search_web(args: dict[str, Any]) -> dict[str, Any]:
    # Логика поиска здесь
    return {"content": [{"type": "text", "text": f"Результаты поиска для: {args['query']}"}]}

multi_tool_server = create_sdk_mcp_server(
    name="utilities",
    version="1.0.0",
    tools=[calculate, translate, search_web]  # Передача декорированных функций
)

# Разрешить только определенные инструменты с потоковым вводом
async def message_generator():
    yield {
        "type": "user",
        "message": {
            "role": "user",
            "content": "Вычисли 5 + 3 и переведи 'hello' на испанский"
        }
    }

async for message in query(
    prompt=message_generator(),  # Использование асинхронного генератора для потокового ввода
    options=ClaudeAgentOptions(
        mcp_servers={"utilities": multi_tool_server},
        allowed_tools=[
            "mcp__utilities__calculate",   # Разрешить калькулятор
            "mcp__utilities__translate",   # Разрешить переводчик
            # "mcp__utilities__search_web" НЕ разрешен
        ]
    )
):
    if hasattr(message, 'result'):
        print(message.result)
```

</CodeGroup>

## Типобезопасность с Python

Декоратор `@tool` поддерживает различные подходы к определению схем для типобезопасности:

<CodeGroup>

```typescript TypeScript
import { z } from "zod";

tool(
  "process_data",
  "Обработать структурированные данные с типобезопасностью",
  {
    // Схема Zod определяет как валидацию времени выполнения, так и типы TypeScript
    data: z.object({
      name: z.string(),
      age: z.number().min(0).max(150),
      email: z.string().email(),
      preferences: z.array(z.string()).optional()
    }),
    format: z.enum(["json", "csv", "xml"]).default("json")
  },
  async (args) => {
    // args полностью типизирован на основе схемы
    // TypeScript знает: args.data.name это string, args.data.age это number, и т.д.
    console.log(`Обработка данных ${args.data.name} как ${args.format}`);
    
    // Ваша логика обработки здесь
    return {
      content: [{
        type: "text",
        text: `Обработаны данные для ${args.data.name}`
      }]
    };
  }
)
```

```python Python
from typing import Any

# Простое сопоставление типов - рекомендуется для большинства случаев
@tool(
    "process_data",
    "Обработать структурированные данные с типобезопасностью",
    {
        "name": str,
        "age": int,
        "email": str,
        "preferences": list  # Опциональные параметры могут быть обработаны в функции
    }
)
async def process_data(args: dict[str, Any]) -> dict[str, Any]:
    # Доступ к аргументам с подсказками типов для поддержки IDE
    name = args["name"]
    age = args["age"]
    email = args["email"]
    preferences = args.get("preferences", [])
    
    print(f"Обработка данных {name} (возраст: {age})")
    
    return {
        "content": [{
            "type": "text",
            "text": f"Обработаны данные для {name}"
        }]
    }

# Для более сложных схем можно использовать формат JSON Schema
@tool(
    "advanced_process",
    "Обработать данные с расширенной валидацией",
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
    # Обработка с расширенной валидацией схемы
    return {
        "content": [{
            "type": "text",
            "text": f"Расширенная обработка для {args['name']}"
        }]
    }
```

</CodeGroup>

## Обработка ошибок

Обрабатывайте ошибки корректно, чтобы предоставить осмысленную обратную связь:

<CodeGroup>

```typescript TypeScript
tool(
  "fetch_data",
  "Получить данные из API",
  {
    endpoint: z.string().url().describe("URL конечной точки API")
  },
  async (args) => {
    try {
      const response = await fetch(args.endpoint);
      
      if (!response.ok) {
        return {
          content: [{
            type: "text",
            text: `Ошибка API: ${response.status} ${response.statusText}`
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
          text: `Не удалось получить данные: ${error.message}`
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
    "Получить данные из API",
    {"endpoint": str}  # Простая схема
)
async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
    try:
        async with aiohttp.ClientSession() as session:
            async with session.get(args["endpoint"]) as response:
                if response.status != 200:
                    return {
                        "content": [{
                            "type": "text",
                            "text": f"Ошибка API: {response.status} {response.reason}"
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
                "text": f"Не удалось получить данные: {str(e)}"
            }]
        }
```

</CodeGroup>

## Примеры инструментов

### Инструмент запроса к базе данных

<CodeGroup>

```typescript TypeScript
const databaseServer = createSdkMcpServer({
  name: "database-tools",
  version: "1.0.0",
  tools: [
    tool(
      "query_database",
      "Выполнить запрос к базе данных",
      {
        query: z.string().describe("SQL запрос для выполнения"),
        params: z.array(z.any()).optional().describe("Параметры запроса")
      },
      async (args) => {
        const results = await db.query(args.query, args.params || []);
        return {
          content: [{
            type: "text",
            text: `Найдено ${results.length} строк:\n${JSON.stringify(results, null, 2)}`
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
    "Выполнить запрос к базе данных",
    {"query": str, "params": list}  # Простая схема с типом list
)
async def query_database(args: dict[str, Any]) -> dict[str, Any]:
    results = await db.query(args["query"], args.get("params", []))
    return {
        "content": [{
            "type": "text",
            "text": f"Найдено {len(results)} строк:\n{json.dumps(results, indent=2)}"
        }]
    }

database_server = create_sdk_mcp_server(
    name="database-tools",
    version="1.0.0",
    tools=[query_database]  # Передача декорированной функции
)
```

</CodeGroup>

### Инструмент API шлюза

<CodeGroup>

```typescript TypeScript
const apiGatewayServer = createSdkMcpServer({
  name: "api-gateway",
  version: "1.0.0",
  tools: [
    tool(
      "api_request",
      "Выполнить аутентифицированные API запросы к внешним сервисам",
      {
        service: z.enum(["stripe", "github", "openai", "slack"]).describe("Сервис для вызова"),
        endpoint: z.string().describe("Путь конечной точки API"),
        method: z.enum(["GET", "POST", "PUT", "DELETE"]).describe("HTTP метод"),
        body: z.record(z.any()).optional().describe("Тело запроса"),
        query: z.record(z.string()).optional().describe("Параметры запроса")
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

# Для сложных схем с перечислениями используйте формат JSON Schema
@tool(
    "api_request",
    "Выполнить аутентифицированные API запросы к внешним сервисам",
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
    tools=[api_request]  # Передача декорированной функции
)
```

</CodeGroup>

### Инструмент калькулятора

<CodeGroup>

```typescript TypeScript
const calculatorServer = createSdkMcpServer({
  name: "calculator",
  version: "1.0.0",
  tools: [
    tool(
      "calculate",
      "Выполнить математические вычисления",
      {
        expression: z.string().describe("Математическое выражение для вычисления"),
        precision: z.number().optional().default(2).describe("Точность десятичных знаков")
      },
      async (args) => {
        try {
          // Используйте безопасную библиотеку математических вычислений в продакшене
          const result = eval(args.expression); // Только для примера!
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
              text: `Ошибка: Недопустимое выражение - ${error.message}`
            }]
          };
        }
      }
    ),
    tool(
      "compound_interest",
      "Вычислить сложные проценты для инвестиции",
      {
        principal: z.number().positive().describe("Начальная сумма инвестиции"),
        rate: z.number().describe("Годовая процентная ставка (как десятичная дробь, например, 0.05 для 5%)"),
        time: z.number().positive().describe("Период инвестиции в годах"),
        n: z.number().positive().default(12).describe("Частота начисления процентов в год")
      },
      async (args) => {
        const amount = args.principal * Math.pow(1 + args.rate / args.n, args.n * args.time);
        const interest = amount - args.principal;
        
        return {
          content: [{
            type: "text",
            text: `Анализ инвестиций:\n` +
                  `Основная сумма: $${args.principal.toFixed(2)}\n` +
                  `Ставка: ${(args.rate * 100).toFixed(2)}%\n` +
                  `Время: ${args.time} лет\n` +
                  `Начисление: ${args.n} раз в год\n\n` +
                  `Итоговая сумма: $${amount.toFixed(2)}\n` +
                  `Заработанные проценты: $${interest.toFixed(2)}\n` +
                  `Доходность: ${((interest / args.principal) * 100).toFixed(2)}%`
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
    "Выполнить математические вычисления",
    {"expression": str, "precision": int}  # Простая схема
)
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    try:
        # Используйте безопасную библиотеку математических вычислений в продакшене
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
                "text": f"Ошибка: Недопустимое выражение - {str(e)}"
            }]
        }

@tool(
    "compound_interest",
    "Вычислить сложные проценты для инвестиции",
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
            "text": f"""Анализ инвестиций:
Основная сумма: ${principal:.2f}
Ставка: {rate * 100:.2f}%
Время: {time} лет
Начисление: {n} раз в год

Итоговая сумма: ${amount:.2f}
Заработанные проценты: ${interest:.2f}
Доходность: {(interest / principal) * 100:.2f}%"""
        }]
    }

calculator_server = create_sdk_mcp_server(
    name="calculator",
    version="1.0.0",
    tools=[calculate, compound_interest]  # Передача декорированных функций
)
```

</CodeGroup>

## Связанная документация

- [Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript)
- [Справочник Python SDK](/docs/ru/agent-sdk/python)
- [Документация MCP](https://modelcontextprotocol.io)
- [Обзор SDK](/docs/ru/agent-sdk/overview)