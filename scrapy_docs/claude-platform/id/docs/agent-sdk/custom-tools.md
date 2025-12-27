# Alat Kustom

Bangun dan integrasikan alat kustom untuk memperluas fungsionalitas Claude Agent SDK

---

Alat kustom memungkinkan Anda memperluas kemampuan Claude Code dengan fungsionalitas Anda sendiri melalui server MCP dalam proses, memungkinkan Claude berinteraksi dengan layanan eksternal, API, atau melakukan operasi khusus.

## Membuat Alat Kustom

Gunakan fungsi helper `createSdkMcpServer` dan `tool` untuk mendefinisikan alat kustom yang type-safe:

<CodeGroup>

```typescript TypeScript
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

// Buat server SDK MCP dengan alat kustom
const customServer = createSdkMcpServer({
  name: "my-custom-tools",
  version: "1.0.0",
  tools: [
    tool(
      "get_weather",
      "Dapatkan suhu saat ini untuk suatu lokasi menggunakan koordinat",
      {
        latitude: z.number().describe("Koordinat lintang"),
        longitude: z.number().describe("Koordinat bujur")
      },
      async (args) => {
        const response = await fetch(`https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&current=temperature_2m&temperature_unit=fahrenheit`);
        const data = await response.json();

        return {
          content: [{
            type: "text",
            text: `Suhu: ${data.current.temperature_2m}°F`
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

# Definisikan alat kustom menggunakan decorator @tool
@tool("get_weather", "Dapatkan suhu saat ini untuk suatu lokasi menggunakan koordinat", {"latitude": float, "longitude": float})
async def get_weather(args: dict[str, Any]) -> dict[str, Any]:
    # Panggil API cuaca
    async with aiohttp.ClientSession() as session:
        async with session.get(
            f"https://api.open-meteo.com/v1/forecast?latitude={args['latitude']}&longitude={args['longitude']}&current=temperature_2m&temperature_unit=fahrenheit"
        ) as response:
            data = await response.json()

    return {
        "content": [{
            "type": "text",
            "text": f"Suhu: {data['current']['temperature_2m']}°F"
        }]
    }

# Buat server SDK MCP dengan alat kustom
custom_server = create_sdk_mcp_server(
    name="my-custom-tools",
    version="1.0.0",
    tools=[get_weather]  # Berikan fungsi yang telah didekorasi
)
```

</CodeGroup>

## Menggunakan Alat Kustom

Berikan server kustom ke fungsi `query` melalui opsi `mcpServers` sebagai dictionary/object.

<Note>
**Penting:** Alat MCP kustom memerlukan mode input streaming. Anda harus menggunakan async generator/iterable untuk parameter `prompt` - string sederhana tidak akan bekerja dengan server MCP.
</Note>

### Format Nama Alat

Ketika alat MCP diekspos ke Claude, nama mereka mengikuti format tertentu:
- Pola: `mcp__{server_name}__{tool_name}`
- Contoh: Alat bernama `get_weather` di server `my-custom-tools` menjadi `mcp__my-custom-tools__get_weather`

### Mengkonfigurasi Alat yang Diizinkan

Anda dapat mengontrol alat mana yang dapat digunakan Claude melalui opsi `allowedTools`:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Gunakan alat kustom dalam query Anda dengan input streaming
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "Bagaimana cuaca di San Francisco?"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // Gunakan async generator untuk input streaming
  options: {
    mcpServers: {
      "my-custom-tools": customServer  // Berikan sebagai object/dictionary, bukan array
    },
    // Secara opsional tentukan alat mana yang dapat digunakan Claude
    allowedTools: [
      "mcp__my-custom-tools__get_weather",  // Izinkan alat cuaca
      // Tambahkan alat lain sesuai kebutuhan
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

# Gunakan alat kustom dengan Claude
options = ClaudeAgentOptions(
    mcp_servers={"my-custom-tools": custom_server},
    allowed_tools=[
        "mcp__my-custom-tools__get_weather",  # Izinkan alat cuaca
        # Tambahkan alat lain sesuai kebutuhan
    ]
)

async def main():
    async with ClaudeSDKClient(options=options) as client:
        await client.query("Bagaimana cuaca di San Francisco?")

        # Ekstrak dan cetak respons
        async for msg in client.receive_response():
            print(msg)

asyncio.run(main())
```

</CodeGroup>

### Contoh Beberapa Alat

Ketika server MCP Anda memiliki beberapa alat, Anda dapat secara selektif mengizinkannya:

<CodeGroup>

```typescript TypeScript
const multiToolServer = createSdkMcpServer({
  name: "utilities",
  version: "1.0.0",
  tools: [
    tool("calculate", "Lakukan perhitungan", { /* ... */ }, async (args) => { /* ... */ }),
    tool("translate", "Terjemahkan teks", { /* ... */ }, async (args) => { /* ... */ }),
    tool("search_web", "Cari di web", { /* ... */ }, async (args) => { /* ... */ })
  ]
});

// Izinkan hanya alat tertentu dengan input streaming
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "Hitung 5 + 3 dan terjemahkan 'hello' ke bahasa Spanyol"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // Gunakan async generator untuk input streaming
  options: {
    mcpServers: {
      utilities: multiToolServer
    },
    allowedTools: [
      "mcp__utilities__calculate",   // Izinkan kalkulator
      "mcp__utilities__translate",   // Izinkan penerjemah
      // "mcp__utilities__search_web" TIDAK diizinkan
    ]
  }
})) {
  // Proses pesan
}
```

```python Python
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, tool, create_sdk_mcp_server
from typing import Any
import asyncio

# Definisikan beberapa alat menggunakan decorator @tool
@tool("calculate", "Lakukan perhitungan", {"expression": str})
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    result = eval(args["expression"])  # Gunakan eval yang aman di produksi
    return {"content": [{"type": "text", "text": f"Hasil: {result}"}]}

@tool("translate", "Terjemahkan teks", {"text": str, "target_lang": str})
async def translate(args: dict[str, Any]) -> dict[str, Any]:
    # Logika terjemahan di sini
    return {"content": [{"type": "text", "text": f"Diterjemahkan: {args['text']}"}]}

@tool("search_web", "Cari di web", {"query": str})
async def search_web(args: dict[str, Any]) -> dict[str, Any]:
    # Logika pencarian di sini
    return {"content": [{"type": "text", "text": f"Hasil pencarian untuk: {args['query']}"}]}

multi_tool_server = create_sdk_mcp_server(
    name="utilities",
    version="1.0.0",
    tools=[calculate, translate, search_web]  # Berikan fungsi yang telah didekorasi
)

# Izinkan hanya alat tertentu dengan input streaming
async def message_generator():
    yield {
        "type": "user",
        "message": {
            "role": "user",
            "content": "Hitung 5 + 3 dan terjemahkan 'hello' ke bahasa Spanyol"
        }
    }

async for message in query(
    prompt=message_generator(),  # Gunakan async generator untuk input streaming
    options=ClaudeAgentOptions(
        mcp_servers={"utilities": multi_tool_server},
        allowed_tools=[
            "mcp__utilities__calculate",   # Izinkan kalkulator
            "mcp__utilities__translate",   # Izinkan penerjemah
            # "mcp__utilities__search_web" TIDAK diizinkan
        ]
    )
):
    if hasattr(message, 'result'):
        print(message.result)
```

</CodeGroup>

## Keamanan Tipe dengan Python

Decorator `@tool` mendukung berbagai pendekatan definisi skema untuk keamanan tipe:

<CodeGroup>

```typescript TypeScript
import { z } from "zod";

tool(
  "process_data",
  "Proses data terstruktur dengan keamanan tipe",
  {
    // Skema Zod mendefinisikan validasi runtime dan tipe TypeScript
    data: z.object({
      name: z.string(),
      age: z.number().min(0).max(150),
      email: z.string().email(),
      preferences: z.array(z.string()).optional()
    }),
    format: z.enum(["json", "csv", "xml"]).default("json")
  },
  async (args) => {
    // args sepenuhnya diketik berdasarkan skema
    // TypeScript tahu: args.data.name adalah string, args.data.age adalah number, dll.
    console.log(`Memproses data ${args.data.name} sebagai ${args.format}`);
    
    // Logika pemrosesan Anda di sini
    return {
      content: [{
        type: "text",
        text: `Data diproses untuk ${args.data.name}`
      }]
    };
  }
)
```

```python Python
from typing import Any

# Pemetaan tipe sederhana - direkomendasikan untuk sebagian besar kasus
@tool(
    "process_data",
    "Proses data terstruktur dengan keamanan tipe",
    {
        "name": str,
        "age": int,
        "email": str,
        "preferences": list  # Parameter opsional dapat ditangani dalam fungsi
    }
)
async def process_data(args: dict[str, Any]) -> dict[str, Any]:
    # Akses argumen dengan petunjuk tipe untuk dukungan IDE
    name = args["name"]
    age = args["age"]
    email = args["email"]
    preferences = args.get("preferences", [])
    
    print(f"Memproses data {name} (umur: {age})")
    
    return {
        "content": [{
            "type": "text",
            "text": f"Data diproses untuk {name}"
        }]
    }

# Untuk skema yang lebih kompleks, Anda dapat menggunakan format JSON Schema
@tool(
    "advanced_process",
    "Proses data dengan validasi lanjutan",
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
    # Proses dengan validasi skema lanjutan
    return {
        "content": [{
            "type": "text",
            "text": f"Pemrosesan lanjutan untuk {args['name']}"
        }]
    }
```

</CodeGroup>

## Penanganan Error

Tangani error dengan baik untuk memberikan umpan balik yang bermakna:

<CodeGroup>

```typescript TypeScript
tool(
  "fetch_data",
  "Ambil data dari API",
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
            text: `Error API: ${response.status} ${response.statusText}`
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
          text: `Gagal mengambil data: ${error.message}`
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
    "Ambil data dari API",
    {"endpoint": str}  # Skema sederhana
)
async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
    try:
        async with aiohttp.ClientSession() as session:
            async with session.get(args["endpoint"]) as response:
                if response.status != 200:
                    return {
                        "content": [{
                            "type": "text",
                            "text": f"Error API: {response.status} {response.reason}"
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
                "text": f"Gagal mengambil data: {str(e)}"
            }]
        }
```

</CodeGroup>

## Contoh Alat

### Alat Query Database

<CodeGroup>

```typescript TypeScript
const databaseServer = createSdkMcpServer({
  name: "database-tools",
  version: "1.0.0",
  tools: [
    tool(
      "query_database",
      "Jalankan query database",
      {
        query: z.string().describe("Query SQL untuk dijalankan"),
        params: z.array(z.any()).optional().describe("Parameter query")
      },
      async (args) => {
        const results = await db.query(args.query, args.params || []);
        return {
          content: [{
            type: "text",
            text: `Ditemukan ${results.length} baris:\n${JSON.stringify(results, null, 2)}`
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
    "Jalankan query database",
    {"query": str, "params": list}  # Skema sederhana dengan tipe list
)
async def query_database(args: dict[str, Any]) -> dict[str, Any]:
    results = await db.query(args["query"], args.get("params", []))
    return {
        "content": [{
            "type": "text",
            "text": f"Ditemukan {len(results)} baris:\n{json.dumps(results, indent=2)}"
        }]
    }

database_server = create_sdk_mcp_server(
    name="database-tools",
    version="1.0.0",
    tools=[query_database]  # Berikan fungsi yang telah didekorasi
)
```

</CodeGroup>

### Alat API Gateway

<CodeGroup>

```typescript TypeScript
const apiGatewayServer = createSdkMcpServer({
  name: "api-gateway",
  version: "1.0.0",
  tools: [
    tool(
      "api_request",
      "Buat permintaan API yang diautentikasi ke layanan eksternal",
      {
        service: z.enum(["stripe", "github", "openai", "slack"]).describe("Layanan yang akan dipanggil"),
        endpoint: z.string().describe("Path endpoint API"),
        method: z.enum(["GET", "POST", "PUT", "DELETE"]).describe("Metode HTTP"),
        body: z.record(z.any()).optional().describe("Body permintaan"),
        query: z.record(z.string()).optional().describe("Parameter query")
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

# Untuk skema kompleks dengan enum, gunakan format JSON Schema
@tool(
    "api_request",
    "Buat permintaan API yang diautentikasi ke layanan eksternal",
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
    tools=[api_request]  # Berikan fungsi yang telah didekorasi
)
```

</CodeGroup>

### Alat Kalkulator

<CodeGroup>

```typescript TypeScript
const calculatorServer = createSdkMcpServer({
  name: "calculator",
  version: "1.0.0",
  tools: [
    tool(
      "calculate",
      "Lakukan perhitungan matematika",
      {
        expression: z.string().describe("Ekspresi matematika untuk dievaluasi"),
        precision: z.number().optional().default(2).describe("Presisi desimal")
      },
      async (args) => {
        try {
          // Gunakan library evaluasi matematika yang aman di produksi
          const result = eval(args.expression); // Hanya contoh!
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
              text: `Error: Ekspresi tidak valid - ${error.message}`
            }]
          };
        }
      }
    ),
    tool(
      "compound_interest",
      "Hitung bunga majemuk untuk investasi",
      {
        principal: z.number().positive().describe("Jumlah investasi awal"),
        rate: z.number().describe("Tingkat bunga tahunan (sebagai desimal, mis. 0.05 untuk 5%)"),
        time: z.number().positive().describe("Periode investasi dalam tahun"),
        n: z.number().positive().default(12).describe("Frekuensi penggabungan per tahun")
      },
      async (args) => {
        const amount = args.principal * Math.pow(1 + args.rate / args.n, args.n * args.time);
        const interest = amount - args.principal;
        
        return {
          content: [{
            type: "text",
            text: `Analisis Investasi:\n` +
                  `Pokok: $${args.principal.toFixed(2)}\n` +
                  `Tingkat: ${(args.rate * 100).toFixed(2)}%\n` +
                  `Waktu: ${args.time} tahun\n` +
                  `Penggabungan: ${args.n} kali per tahun\n\n` +
                  `Jumlah Akhir: $${amount.toFixed(2)}\n` +
                  `Bunga yang Diperoleh: $${interest.toFixed(2)}\n` +
                  `Pengembalian: ${((interest / args.principal) * 100).toFixed(2)}%`
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
    "Lakukan perhitungan matematika",
    {"expression": str, "precision": int}  # Skema sederhana
)
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    try:
        # Gunakan library evaluasi matematika yang aman di produksi
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
                "text": f"Error: Ekspresi tidak valid - {str(e)}"
            }]
        }

@tool(
    "compound_interest",
    "Hitung bunga majemuk untuk investasi",
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
            "text": f"""Analisis Investasi:
Pokok: ${principal:.2f}
Tingkat: {rate * 100:.2f}%
Waktu: {time} tahun
Penggabungan: {n} kali per tahun

Jumlah Akhir: ${amount:.2f}
Bunga yang Diperoleh: ${interest:.2f}
Pengembalian: {(interest / principal) * 100:.2f}%"""
        }]
    }

calculator_server = create_sdk_mcp_server(
    name="calculator",
    version="1.0.0",
    tools=[calculate, compound_interest]  # Berikan fungsi yang telah didekorasi
)
```

</CodeGroup>

## Dokumentasi Terkait

- [Referensi TypeScript SDK](/docs/id/agent-sdk/typescript)
- [Referensi Python SDK](/docs/id/agent-sdk/python)
- [Dokumentasi MCP](https://modelcontextprotocol.io)
- [Ikhtisar SDK](/docs/id/agent-sdk/overview)