# 自訂工具

建立並整合自訂工具以擴展 Claude Agent SDK 功能

---

自訂工具允許您透過程序內 MCP 伺服器使用自己的功能來擴展 Claude Code 的能力，讓 Claude 能夠與外部服務、API 互動，或執行專門的操作。

## 建立自訂工具

使用 `createSdkMcpServer` 和 `tool` 輔助函數來定義型別安全的自訂工具：

<CodeGroup>

```typescript TypeScript
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

// 建立一個帶有自訂工具的 SDK MCP 伺服器
const customServer = createSdkMcpServer({
  name: "my-custom-tools",
  version: "1.0.0",
  tools: [
    tool(
      "get_weather",
      "使用座標取得某個位置的當前溫度",
      {
        latitude: z.number().describe("緯度座標"),
        longitude: z.number().describe("經度座標")
      },
      async (args) => {
        const response = await fetch(`https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&current=temperature_2m&temperature_unit=fahrenheit`);
        const data = await response.json();

        return {
          content: [{
            type: "text",
            text: `溫度：${data.current.temperature_2m}°F`
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

# 使用 @tool 裝飾器定義自訂工具
@tool("get_weather", "使用座標取得某個位置的當前溫度", {"latitude": float, "longitude": float})
async def get_weather(args: dict[str, Any]) -> dict[str, Any]:
    # 呼叫天氣 API
    async with aiohttp.ClientSession() as session:
        async with session.get(
            f"https://api.open-meteo.com/v1/forecast?latitude={args['latitude']}&longitude={args['longitude']}&current=temperature_2m&temperature_unit=fahrenheit"
        ) as response:
            data = await response.json()

    return {
        "content": [{
            "type": "text",
            "text": f"溫度：{data['current']['temperature_2m']}°F"
        }]
    }

# 使用自訂工具建立 SDK MCP 伺服器
custom_server = create_sdk_mcp_server(
    name="my-custom-tools",
    version="1.0.0",
    tools=[get_weather]  # 傳遞裝飾過的函數
)
```

</CodeGroup>

## 使用自訂工具

透過 `mcpServers` 選項將自訂伺服器作為字典/物件傳遞給 `query` 函數。

<Note>
**重要：** 自訂 MCP 工具需要串流輸入模式。您必須為 `prompt` 參數使用非同步產生器/可迭代物件 - 簡單的字串無法與 MCP 伺服器一起使用。
</Note>

### 工具名稱格式

當 MCP 工具暴露給 Claude 時，它們的名稱遵循特定格式：
- 模式：`mcp__{server_name}__{tool_name}`
- 範例：在伺服器 `my-custom-tools` 中名為 `get_weather` 的工具會變成 `mcp__my-custom-tools__get_weather`

### 設定允許的工具

您可以透過 `allowedTools` 選項控制 Claude 可以使用哪些工具：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 在查詢中使用自訂工具與串流輸入
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "舊金山的天氣如何？"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // 使用非同步產生器進行串流輸入
  options: {
    mcpServers: {
      "my-custom-tools": customServer  // 作為物件/字典傳遞，而非陣列
    },
    // 可選擇性地指定 Claude 可以使用哪些工具
    allowedTools: [
      "mcp__my-custom-tools__get_weather",  // 允許天氣工具
      // 根據需要新增其他工具
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

# 與 Claude 一起使用自訂工具
options = ClaudeAgentOptions(
    mcp_servers={"my-custom-tools": custom_server},
    allowed_tools=[
        "mcp__my-custom-tools__get_weather",  # 允許天氣工具
        # 根據需要新增其他工具
    ]
)

async def main():
    async with ClaudeSDKClient(options=options) as client:
        await client.query("舊金山的天氣如何？")

        # 提取並列印回應
        async for msg in client.receive_response():
            print(msg)

asyncio.run(main())
```

</CodeGroup>

### 多工具範例

當您的 MCP 伺服器有多個工具時，您可以選擇性地允許它們：

<CodeGroup>

```typescript TypeScript
const multiToolServer = createSdkMcpServer({
  name: "utilities",
  version: "1.0.0",
  tools: [
    tool("calculate", "執行計算", { /* ... */ }, async (args) => { /* ... */ }),
    tool("translate", "翻譯文字", { /* ... */ }, async (args) => { /* ... */ }),
    tool("search_web", "搜尋網路", { /* ... */ }, async (args) => { /* ... */ })
  ]
});

// 只允許特定工具與串流輸入
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "計算 5 + 3 並將 'hello' 翻譯成西班牙文"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // 使用非同步產生器進行串流輸入
  options: {
    mcpServers: {
      utilities: multiToolServer
    },
    allowedTools: [
      "mcp__utilities__calculate",   // 允許計算器
      "mcp__utilities__translate",   // 允許翻譯器
      // "mcp__utilities__search_web" 不被允許
    ]
  }
})) {
  // 處理訊息
}
```

```python Python
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, tool, create_sdk_mcp_server
from typing import Any
import asyncio

# 使用 @tool 裝飾器定義多個工具
@tool("calculate", "執行計算", {"expression": str})
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    result = eval(args["expression"])  # 在生產環境中使用安全的 eval
    return {"content": [{"type": "text", "text": f"結果：{result}"}]}

@tool("translate", "翻譯文字", {"text": str, "target_lang": str})
async def translate(args: dict[str, Any]) -> dict[str, Any]:
    # 翻譯邏輯在此
    return {"content": [{"type": "text", "text": f"翻譯：{args['text']}"}]}

@tool("search_web", "搜尋網路", {"query": str})
async def search_web(args: dict[str, Any]) -> dict[str, Any]:
    # 搜尋邏輯在此
    return {"content": [{"type": "text", "text": f"搜尋結果：{args['query']}"}]}

multi_tool_server = create_sdk_mcp_server(
    name="utilities",
    version="1.0.0",
    tools=[calculate, translate, search_web]  # 傳遞裝飾過的函數
)

# 只允許特定工具與串流輸入
async def message_generator():
    yield {
        "type": "user",
        "message": {
            "role": "user",
            "content": "計算 5 + 3 並將 'hello' 翻譯成西班牙文"
        }
    }

async for message in query(
    prompt=message_generator(),  # 使用非同步產生器進行串流輸入
    options=ClaudeAgentOptions(
        mcp_servers={"utilities": multi_tool_server},
        allowed_tools=[
            "mcp__utilities__calculate",   # 允許計算器
            "mcp__utilities__translate",   # 允許翻譯器
            # "mcp__utilities__search_web" 不被允許
        ]
    )
):
    if hasattr(message, 'result'):
        print(message.result)
```

</CodeGroup>

## Python 的型別安全

`@tool` 裝飾器支援各種模式定義方法以實現型別安全：

<CodeGroup>

```typescript TypeScript
import { z } from "zod";

tool(
  "process_data",
  "使用型別安全處理結構化資料",
  {
    // Zod 模式定義執行時驗證和 TypeScript 型別
    data: z.object({
      name: z.string(),
      age: z.number().min(0).max(150),
      email: z.string().email(),
      preferences: z.array(z.string()).optional()
    }),
    format: z.enum(["json", "csv", "xml"]).default("json")
  },
  async (args) => {
    // args 根據模式完全型別化
    // TypeScript 知道：args.data.name 是字串，args.data.age 是數字等
    console.log(`正在將 ${args.data.name} 的資料處理為 ${args.format}`);
    
    // 您的處理邏輯在此
    return {
      content: [{
        type: "text",
        text: `已處理 ${args.data.name} 的資料`
      }]
    };
  }
)
```

```python Python
from typing import Any

# 簡單型別對應 - 建議用於大多數情況
@tool(
    "process_data",
    "使用型別安全處理結構化資料",
    {
        "name": str,
        "age": int,
        "email": str,
        "preferences": list  # 可選參數可以在函數中處理
    }
)
async def process_data(args: dict[str, Any]) -> dict[str, Any]:
    # 使用型別提示存取參數以獲得 IDE 支援
    name = args["name"]
    age = args["age"]
    email = args["email"]
    preferences = args.get("preferences", [])
    
    print(f"正在處理 {name} 的資料（年齡：{age}）")
    
    return {
        "content": [{
            "type": "text",
            "text": f"已處理 {name} 的資料"
        }]
    }

# 對於更複雜的模式，您可以使用 JSON Schema 格式
@tool(
    "advanced_process",
    "使用進階驗證處理資料",
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
    # 使用進階模式驗證進行處理
    return {
        "content": [{
            "type": "text",
            "text": f"對 {args['name']} 進行進階處理"
        }]
    }
```

</CodeGroup>

## 錯誤處理

優雅地處理錯誤以提供有意義的回饋：

<CodeGroup>

```typescript TypeScript
tool(
  "fetch_data",
  "從 API 取得資料",
  {
    endpoint: z.string().url().describe("API 端點 URL")
  },
  async (args) => {
    try {
      const response = await fetch(args.endpoint);
      
      if (!response.ok) {
        return {
          content: [{
            type: "text",
            text: `API 錯誤：${response.status} ${response.statusText}`
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
          text: `取得資料失敗：${error.message}`
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
    "從 API 取得資料",
    {"endpoint": str}  # 簡單模式
)
async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
    try:
        async with aiohttp.ClientSession() as session:
            async with session.get(args["endpoint"]) as response:
                if response.status != 200:
                    return {
                        "content": [{
                            "type": "text",
                            "text": f"API 錯誤：{response.status} {response.reason}"
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
                "text": f"取得資料失敗：{str(e)}"
            }]
        }
```

</CodeGroup>

## 範例工具

### 資料庫查詢工具

<CodeGroup>

```typescript TypeScript
const databaseServer = createSdkMcpServer({
  name: "database-tools",
  version: "1.0.0",
  tools: [
    tool(
      "query_database",
      "執行資料庫查詢",
      {
        query: z.string().describe("要執行的 SQL 查詢"),
        params: z.array(z.any()).optional().describe("查詢參數")
      },
      async (args) => {
        const results = await db.query(args.query, args.params || []);
        return {
          content: [{
            type: "text",
            text: `找到 ${results.length} 行：\n${JSON.stringify(results, null, 2)}`
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
    "執行資料庫查詢",
    {"query": str, "params": list}  # 帶有 list 型別的簡單模式
)
async def query_database(args: dict[str, Any]) -> dict[str, Any]:
    results = await db.query(args["query"], args.get("params", []))
    return {
        "content": [{
            "type": "text",
            "text": f"找到 {len(results)} 行：\n{json.dumps(results, indent=2)}"
        }]
    }

database_server = create_sdk_mcp_server(
    name="database-tools",
    version="1.0.0",
    tools=[query_database]  # 傳遞裝飾過的函數
)
```

</CodeGroup>

### API 閘道工具

<CodeGroup>

```typescript TypeScript
const apiGatewayServer = createSdkMcpServer({
  name: "api-gateway",
  version: "1.0.0",
  tools: [
    tool(
      "api_request",
      "對外部服務進行已驗證的 API 請求",
      {
        service: z.enum(["stripe", "github", "openai", "slack"]).describe("要呼叫的服務"),
        endpoint: z.string().describe("API 端點路徑"),
        method: z.enum(["GET", "POST", "PUT", "DELETE"]).describe("HTTP 方法"),
        body: z.record(z.any()).optional().describe("請求主體"),
        query: z.record(z.string()).optional().describe("查詢參數")
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

# 對於帶有列舉的複雜模式，使用 JSON Schema 格式
@tool(
    "api_request",
    "對外部服務進行已驗證的 API 請求",
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
    tools=[api_request]  # 傳遞裝飾過的函數
)
```

</CodeGroup>

### 計算器工具

<CodeGroup>

```typescript TypeScript
const calculatorServer = createSdkMcpServer({
  name: "calculator",
  version: "1.0.0",
  tools: [
    tool(
      "calculate",
      "執行數學計算",
      {
        expression: z.string().describe("要評估的數學表達式"),
        precision: z.number().optional().default(2).describe("小數精度")
      },
      async (args) => {
        try {
          // 在生產環境中使用安全的數學評估庫
          const result = eval(args.expression); // 僅供範例！
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
              text: `錯誤：無效表達式 - ${error.message}`
            }]
          };
        }
      }
    ),
    tool(
      "compound_interest",
      "計算投資的複利",
      {
        principal: z.number().positive().describe("初始投資金額"),
        rate: z.number().describe("年利率（以小數表示，例如 0.05 表示 5%）"),
        time: z.number().positive().describe("投資期間（年）"),
        n: z.number().positive().default(12).describe("每年複利頻率")
      },
      async (args) => {
        const amount = args.principal * Math.pow(1 + args.rate / args.n, args.n * args.time);
        const interest = amount - args.principal;
        
        return {
          content: [{
            type: "text",
            text: `投資分析：\n` +
                  `本金：$${args.principal.toFixed(2)}\n` +
                  `利率：${(args.rate * 100).toFixed(2)}%\n` +
                  `時間：${args.time} 年\n` +
                  `複利：每年 ${args.n} 次\n\n` +
                  `最終金額：$${amount.toFixed(2)}\n` +
                  `賺取利息：$${interest.toFixed(2)}\n` +
                  `回報：${((interest / args.principal) * 100).toFixed(2)}%`
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
    "執行數學計算",
    {"expression": str, "precision": int}  # 簡單模式
)
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    try:
        # 在生產環境中使用安全的數學評估庫
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
                "text": f"錯誤：無效表達式 - {str(e)}"
            }]
        }

@tool(
    "compound_interest",
    "計算投資的複利",
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
            "text": f"""投資分析：
本金：${principal:.2f}
利率：{rate * 100:.2f}%
時間：{time} 年
複利：每年 {n} 次

最終金額：${amount:.2f}
賺取利息：${interest:.2f}
回報：{(interest / principal) * 100:.2f}%"""
        }]
    }

calculator_server = create_sdk_mcp_server(
    name="calculator",
    version="1.0.0",
    tools=[calculate, compound_interest]  # 傳遞裝飾過的函數
)
```

</CodeGroup>

## 相關文件

- [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript)
- [Python SDK 參考](/docs/zh-TW/agent-sdk/python)
- [MCP 文件](https://modelcontextprotocol.io)
- [SDK 概述](/docs/zh-TW/agent-sdk/overview)