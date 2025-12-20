# 사용자 정의 도구

Claude Agent SDK 기능을 확장하기 위한 사용자 정의 도구를 구축하고 통합하세요

---

사용자 정의 도구를 사용하면 인프로세스 MCP 서버를 통해 자체 기능으로 Claude Code의 기능을 확장할 수 있으며, Claude가 외부 서비스, API와 상호 작용하거나 특수한 작업을 수행할 수 있습니다.

## 사용자 정의 도구 생성

`createSdkMcpServer`와 `tool` 헬퍼 함수를 사용하여 타입 안전한 사용자 정의 도구를 정의하세요:

<CodeGroup>

```typescript TypeScript
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

// 사용자 정의 도구로 SDK MCP 서버 생성
const customServer = createSdkMcpServer({
  name: "my-custom-tools",
  version: "1.0.0",
  tools: [
    tool(
      "get_weather",
      "좌표를 사용하여 위치의 현재 온도 가져오기",
      {
        latitude: z.number().describe("위도 좌표"),
        longitude: z.number().describe("경도 좌표")
      },
      async (args) => {
        const response = await fetch(`https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&current=temperature_2m&temperature_unit=fahrenheit`);
        const data = await response.json();

        return {
          content: [{
            type: "text",
            text: `온도: ${data.current.temperature_2m}°F`
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

# @tool 데코레이터를 사용하여 사용자 정의 도구 정의
@tool("get_weather", "좌표를 사용하여 위치의 현재 온도 가져오기", {"latitude": float, "longitude": float})
async def get_weather(args: dict[str, Any]) -> dict[str, Any]:
    # 날씨 API 호출
    async with aiohttp.ClientSession() as session:
        async with session.get(
            f"https://api.open-meteo.com/v1/forecast?latitude={args['latitude']}&longitude={args['longitude']}&current=temperature_2m&temperature_unit=fahrenheit"
        ) as response:
            data = await response.json()

    return {
        "content": [{
            "type": "text",
            "text": f"온도: {data['current']['temperature_2m']}°F"
        }]
    }

# 사용자 정의 도구로 SDK MCP 서버 생성
custom_server = create_sdk_mcp_server(
    name="my-custom-tools",
    version="1.0.0",
    tools=[get_weather]  # 데코레이트된 함수 전달
)
```

</CodeGroup>

## 사용자 정의 도구 사용

`mcpServers` 옵션을 통해 사용자 정의 서버를 딕셔너리/객체로 `query` 함수에 전달하세요.

<Note>
**중요:** 사용자 정의 MCP 도구는 스트리밍 입력 모드가 필요합니다. `prompt` 매개변수에 비동기 제너레이터/이터러블을 사용해야 하며, 단순한 문자열은 MCP 서버와 함께 작동하지 않습니다.
</Note>

### 도구 이름 형식

MCP 도구가 Claude에 노출될 때, 이름은 특정 형식을 따릅니다:
- 패턴: `mcp__{server_name}__{tool_name}`
- 예시: 서버 `my-custom-tools`의 `get_weather`라는 도구는 `mcp__my-custom-tools__get_weather`가 됩니다

### 허용된 도구 구성

`allowedTools` 옵션을 통해 Claude가 사용할 수 있는 도구를 제어할 수 있습니다:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 스트리밍 입력으로 쿼리에서 사용자 정의 도구 사용
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "샌프란시스코의 날씨는 어떤가요?"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // 스트리밍 입력을 위한 비동기 제너레이터 사용
  options: {
    mcpServers: {
      "my-custom-tools": customServer  // 배열이 아닌 객체/딕셔너리로 전달
    },
    // 선택적으로 Claude가 사용할 수 있는 도구 지정
    allowedTools: [
      "mcp__my-custom-tools__get_weather",  // 날씨 도구 허용
      // 필요에 따라 다른 도구 추가
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

# Claude와 함께 사용자 정의 도구 사용
options = ClaudeAgentOptions(
    mcp_servers={"my-custom-tools": custom_server},
    allowed_tools=[
        "mcp__my-custom-tools__get_weather",  # 날씨 도구 허용
        # 필요에 따라 다른 도구 추가
    ]
)

async def main():
    async with ClaudeSDKClient(options=options) as client:
        await client.query("샌프란시스코의 날씨는 어떤가요?")

        # 응답 추출 및 출력
        async for msg in client.receive_response():
            print(msg)

asyncio.run(main())
```

</CodeGroup>

### 다중 도구 예시

MCP 서버에 여러 도구가 있을 때, 선택적으로 허용할 수 있습니다:

<CodeGroup>

```typescript TypeScript
const multiToolServer = createSdkMcpServer({
  name: "utilities",
  version: "1.0.0",
  tools: [
    tool("calculate", "계산 수행", { /* ... */ }, async (args) => { /* ... */ }),
    tool("translate", "텍스트 번역", { /* ... */ }, async (args) => { /* ... */ }),
    tool("search_web", "웹 검색", { /* ... */ }, async (args) => { /* ... */ })
  ]
});

// 스트리밍 입력으로 특정 도구만 허용
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "5 + 3을 계산하고 'hello'를 스페인어로 번역해주세요"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // 스트리밍 입력을 위한 비동기 제너레이터 사용
  options: {
    mcpServers: {
      utilities: multiToolServer
    },
    allowedTools: [
      "mcp__utilities__calculate",   // 계산기 허용
      "mcp__utilities__translate",   // 번역기 허용
      // "mcp__utilities__search_web"는 허용되지 않음
    ]
  }
})) {
  // 메시지 처리
}
```

```python Python
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, tool, create_sdk_mcp_server
from typing import Any
import asyncio

# @tool 데코레이터를 사용하여 여러 도구 정의
@tool("calculate", "계산 수행", {"expression": str})
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    result = eval(args["expression"])  # 프로덕션에서는 안전한 eval 사용
    return {"content": [{"type": "text", "text": f"결과: {result}"}]}

@tool("translate", "텍스트 번역", {"text": str, "target_lang": str})
async def translate(args: dict[str, Any]) -> dict[str, Any]:
    # 번역 로직
    return {"content": [{"type": "text", "text": f"번역됨: {args['text']}"}]}

@tool("search_web", "웹 검색", {"query": str})
async def search_web(args: dict[str, Any]) -> dict[str, Any]:
    # 검색 로직
    return {"content": [{"type": "text", "text": f"검색 결과: {args['query']}"}]}

multi_tool_server = create_sdk_mcp_server(
    name="utilities",
    version="1.0.0",
    tools=[calculate, translate, search_web]  # 데코레이트된 함수 전달
)

# 스트리밍 입력으로 특정 도구만 허용
async def message_generator():
    yield {
        "type": "user",
        "message": {
            "role": "user",
            "content": "5 + 3을 계산하고 'hello'를 스페인어로 번역해주세요"
        }
    }

async for message in query(
    prompt=message_generator(),  # 스트리밍 입력을 위한 비동기 제너레이터 사용
    options=ClaudeAgentOptions(
        mcp_servers={"utilities": multi_tool_server},
        allowed_tools=[
            "mcp__utilities__calculate",   # 계산기 허용
            "mcp__utilities__translate",   # 번역기 허용
            # "mcp__utilities__search_web"는 허용되지 않음
        ]
    )
):
    if hasattr(message, 'result'):
        print(message.result)
```

</CodeGroup>

## Python에서의 타입 안전성

`@tool` 데코레이터는 타입 안전성을 위한 다양한 스키마 정의 접근 방식을 지원합니다:

<CodeGroup>

```typescript TypeScript
import { z } from "zod";

tool(
  "process_data",
  "타입 안전성을 가진 구조화된 데이터 처리",
  {
    // Zod 스키마는 런타임 검증과 TypeScript 타입을 모두 정의
    data: z.object({
      name: z.string(),
      age: z.number().min(0).max(150),
      email: z.string().email(),
      preferences: z.array(z.string()).optional()
    }),
    format: z.enum(["json", "csv", "xml"]).default("json")
  },
  async (args) => {
    // args는 스키마를 기반으로 완전히 타입이 지정됨
    // TypeScript는 args.data.name이 string, args.data.age가 number 등임을 알고 있음
    console.log(`${args.data.name}의 데이터를 ${args.format}으로 처리 중`);
    
    // 처리 로직
    return {
      content: [{
        type: "text",
        text: `${args.data.name}의 데이터 처리됨`
      }]
    };
  }
)
```

```python Python
from typing import Any

# 간단한 타입 매핑 - 대부분의 경우에 권장
@tool(
    "process_data",
    "타입 안전성을 가진 구조화된 데이터 처리",
    {
        "name": str,
        "age": int,
        "email": str,
        "preferences": list  # 선택적 매개변수는 함수에서 처리 가능
    }
)
async def process_data(args: dict[str, Any]) -> dict[str, Any]:
    # IDE 지원을 위한 타입 힌트로 인수 접근
    name = args["name"]
    age = args["age"]
    email = args["email"]
    preferences = args.get("preferences", [])
    
    print(f"{name}의 데이터 처리 중 (나이: {age})")
    
    return {
        "content": [{
            "type": "text",
            "text": f"{name}의 데이터 처리됨"
        }]
    }

# 더 복잡한 스키마의 경우 JSON Schema 형식 사용 가능
@tool(
    "advanced_process",
    "고급 검증을 통한 데이터 처리",
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
    # 고급 스키마 검증으로 처리
    return {
        "content": [{
            "type": "text",
            "text": f"{args['name']}의 고급 처리"
        }]
    }
```

</CodeGroup>

## 오류 처리

의미 있는 피드백을 제공하기 위해 오류를 우아하게 처리하세요:

<CodeGroup>

```typescript TypeScript
tool(
  "fetch_data",
  "API에서 데이터 가져오기",
  {
    endpoint: z.string().url().describe("API 엔드포인트 URL")
  },
  async (args) => {
    try {
      const response = await fetch(args.endpoint);
      
      if (!response.ok) {
        return {
          content: [{
            type: "text",
            text: `API 오류: ${response.status} ${response.statusText}`
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
          text: `데이터 가져오기 실패: ${error.message}`
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
    "API에서 데이터 가져오기",
    {"endpoint": str}  # 간단한 스키마
)
async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
    try:
        async with aiohttp.ClientSession() as session:
            async with session.get(args["endpoint"]) as response:
                if response.status != 200:
                    return {
                        "content": [{
                            "type": "text",
                            "text": f"API 오류: {response.status} {response.reason}"
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
                "text": f"데이터 가져오기 실패: {str(e)}"
            }]
        }
```

</CodeGroup>

## 예시 도구

### 데이터베이스 쿼리 도구

<CodeGroup>

```typescript TypeScript
const databaseServer = createSdkMcpServer({
  name: "database-tools",
  version: "1.0.0",
  tools: [
    tool(
      "query_database",
      "데이터베이스 쿼리 실행",
      {
        query: z.string().describe("실행할 SQL 쿼리"),
        params: z.array(z.any()).optional().describe("쿼리 매개변수")
      },
      async (args) => {
        const results = await db.query(args.query, args.params || []);
        return {
          content: [{
            type: "text",
            text: `${results.length}개 행 발견:\n${JSON.stringify(results, null, 2)}`
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
    "데이터베이스 쿼리 실행",
    {"query": str, "params": list}  # list 타입을 가진 간단한 스키마
)
async def query_database(args: dict[str, Any]) -> dict[str, Any]:
    results = await db.query(args["query"], args.get("params", []))
    return {
        "content": [{
            "type": "text",
            "text": f"{len(results)}개 행 발견:\n{json.dumps(results, indent=2)}"
        }]
    }

database_server = create_sdk_mcp_server(
    name="database-tools",
    version="1.0.0",
    tools=[query_database]  # 데코레이트된 함수 전달
)
```

</CodeGroup>

### API 게이트웨이 도구

<CodeGroup>

```typescript TypeScript
const apiGatewayServer = createSdkMcpServer({
  name: "api-gateway",
  version: "1.0.0",
  tools: [
    tool(
      "api_request",
      "외부 서비스에 인증된 API 요청 만들기",
      {
        service: z.enum(["stripe", "github", "openai", "slack"]).describe("호출할 서비스"),
        endpoint: z.string().describe("API 엔드포인트 경로"),
        method: z.enum(["GET", "POST", "PUT", "DELETE"]).describe("HTTP 메서드"),
        body: z.record(z.any()).optional().describe("요청 본문"),
        query: z.record(z.string()).optional().describe("쿼리 매개변수")
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

# 열거형이 있는 복잡한 스키마의 경우 JSON Schema 형식 사용
@tool(
    "api_request",
    "외부 서비스에 인증된 API 요청 만들기",
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
    tools=[api_request]  # 데코레이트된 함수 전달
)
```

</CodeGroup>

### 계산기 도구

<CodeGroup>

```typescript TypeScript
const calculatorServer = createSdkMcpServer({
  name: "calculator",
  version: "1.0.0",
  tools: [
    tool(
      "calculate",
      "수학적 계산 수행",
      {
        expression: z.string().describe("평가할 수학 표현식"),
        precision: z.number().optional().default(2).describe("소수점 정밀도")
      },
      async (args) => {
        try {
          // 프로덕션에서는 안전한 수학 평가 라이브러리 사용
          const result = eval(args.expression); // 예시 전용!
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
              text: `오류: 잘못된 표현식 - ${error.message}`
            }]
          };
        }
      }
    ),
    tool(
      "compound_interest",
      "투자의 복리 이자 계산",
      {
        principal: z.number().positive().describe("초기 투자 금액"),
        rate: z.number().describe("연간 이자율 (소수로, 예: 5%의 경우 0.05)"),
        time: z.number().positive().describe("투자 기간(년)"),
        n: z.number().positive().default(12).describe("연간 복리 빈도")
      },
      async (args) => {
        const amount = args.principal * Math.pow(1 + args.rate / args.n, args.n * args.time);
        const interest = amount - args.principal;
        
        return {
          content: [{
            type: "text",
            text: `투자 분석:\n` +
                  `원금: $${args.principal.toFixed(2)}\n` +
                  `이율: ${(args.rate * 100).toFixed(2)}%\n` +
                  `기간: ${args.time}년\n` +
                  `복리: 연 ${args.n}회\n\n` +
                  `최종 금액: $${amount.toFixed(2)}\n` +
                  `이자 수익: $${interest.toFixed(2)}\n` +
                  `수익률: ${((interest / args.principal) * 100).toFixed(2)}%`
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
    "수학적 계산 수행",
    {"expression": str, "precision": int}  # 간단한 스키마
)
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    try:
        # 프로덕션에서는 안전한 수학 평가 라이브러리 사용
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
                "text": f"오류: 잘못된 표현식 - {str(e)}"
            }]
        }

@tool(
    "compound_interest",
    "투자의 복리 이자 계산",
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
            "text": f"""투자 분석:
원금: ${principal:.2f}
이율: {rate * 100:.2f}%
기간: {time}년
복리: 연 {n}회

최종 금액: ${amount:.2f}
이자 수익: ${interest:.2f}
수익률: {(interest / principal) * 100:.2f}%"""
        }]
    }

calculator_server = create_sdk_mcp_server(
    name="calculator",
    version="1.0.0",
    tools=[calculate, compound_interest]  # 데코레이트된 함수 전달
)
```

</CodeGroup>

## 관련 문서

- [TypeScript SDK 참조](/docs/ko/agent-sdk/typescript)
- [Python SDK 참조](/docs/ko/agent-sdk/python)
- [MCP 문서](https://modelcontextprotocol.io)
- [SDK 개요](/docs/ko/agent-sdk/overview)