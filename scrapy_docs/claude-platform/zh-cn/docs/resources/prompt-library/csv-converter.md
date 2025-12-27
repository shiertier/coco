# CSV 转换器

将各种格式（JSON、XML 等）的数据转换为格式正确的 CSV 文件。

---

> 将此提示复制到我们的开发者 [Console](/dashboard) 中亲自试用！

|        | 内容 |
| --- | --- |
| System | 作为数据转换专家，您的任务是将不同格式（JSON、XML 等）的数据转换为格式正确的 CSV 文件。用户将提供原始格式的输入数据，以及对 CSV 输出的任何特定要求或偏好（例如，列顺序、分隔符、编码）。确保您清楚了解数据结构和所需的 CSV 格式，根据需要提出任何澄清问题。一旦您获得必要的信息，通过遵循适当的格式规则生成 CSV 输出，例如使用逗号作为分隔符、必要时将值用引号括起来，以及正确处理特殊字符或换行符。最后，提供保存或使用 CSV 文件的任何其他说明或提示。 |
| User   | 请将以下 JSON 数据转换为 CSV 文件： <br/> <br/> [ <br/> \{ <br/> "name": "John Doe", <br/> "age": 30, <br/> "city": "New York", <br/> "email": "john.doe@example.com" <br/> }, <br/> \{ <br/> "name": "Jane Smith", <br/> "age": 25, <br/> "city": "London", <br/> "email": "jane.smith@example.com" <br/> }, <br/> \{ <br/> "name": "Bob Johnson", <br/> "age": 35, <br/> "city": "Paris", <br/> "email": "bob.johnson@example.com" <br/> } <br/> ] <br/> 要求： <br/> - CSV 中的列应按以下顺序：name、age、city、email <br/> - 使用分号（;）作为分隔符 <br/> - 将所有值用双引号（"）括起来 |

## 示例输出

基于提供的 JSON 数据和要求，这是转换后的 CSV 文件：

```csv csv
"name";"age";"city";"email"
"John Doe";"30";"New York";"john.doe@example.com"
"Jane Smith";"25";"London";"jane.smith@example.com"
"Bob Johnson";"35";"Paris";"bob.johnson@example.com"
```

说明：

    - CSV 文件的第一行包含列标题："name"、"age"、"city" 和 "email"，用分号（;）分隔并用双引号（"）括起来。
    - 每个后续行代表一行数据，值用分号（;）分隔并用双引号（"）括起来。
    - 每行中值的顺序对应于指定的列顺序：name、age、city 和 email。
    - 如果值中存在特殊字符，它们会在双引号内保留。

保存 CSV 文件：

    1. 复制上面提供的 CSV 数据。
    2. 打开文本编辑器（例如，记事本、TextEdit 或任何其他纯文本编辑器）。
    3. 将 CSV 数据粘贴到文本编辑器中。
    4. 使用 .csv 扩展名保存文件（例如，data.csv）。

注意：在电子表格应用程序（例如，Microsoft Excel、Google Sheets）中打开 CSV 文件时，确保指定正确的分隔符（分号）和值周围双引号的存在，以确保正确的格式。

---

## API 请求

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    system="作为数据转换专家，您的任务是将不同格式（JSON、XML 等）的数据转换为格式正确的 CSV 文件。用户将提供原始格式的输入数据，以及对 CSV 输出的任何特定要求或偏好（例如，列顺序、分隔符、编码）。确保您清楚了解数据结构和所需的 CSV 格式，根据需要提出任何澄清问题。一旦您获得必要的信息，通过遵循适当的格式规则生成 CSV 输出，例如使用逗号作为分隔符、必要时将值用引号括起来，以及正确处理特殊字符或换行符。最后，提供保存或使用 CSV 文件的任何其他说明或提示。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": '请将以下 JSON 数据转换为 CSV 文件： \n \n[ \n { \n "name": "John Doe", \n "age": 30, \n "city": "New York", \n "email": "[email protected]" \n }, \n { \n "name": "Jane Smith", \n "age": 25, \n "city": "London", \n "email": "[email protected]" \n }, \n { \n "name": "Bob Johnson", \n "age": 35, \n "city": "Paris", \n "email": "[email protected]" \n } \n] \n \n要求： \n- CSV 中的列应按以下顺序：name、age、city、email \n- 使用分号（;）作为分隔符 \n- 将所有值用双引号（"）括起来',
                }
            ],
        }
    ],
)
print(message.content)


```

</Tab>
<Tab title="TypeScript">

```typescript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic({
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "作为数据转换专家，您的任务是将不同格式（JSON、XML 等）的数据转换为格式正确的 CSV 文件。用户将提供原始格式的输入数据，以及对 CSV 输出的任何特定要求或偏好（例如，列顺序、分隔符、编码）。确保您清楚了解数据结构和所需的 CSV 格式，根据需要提出任何澄清问题。一旦您获得必要的信息，通过遵循适当的格式规则生成 CSV 输出，例如使用逗号作为分隔符、必要时将值用引号括起来，以及正确处理特殊字符或换行符。最后，提供保存或使用 CSV 文件的任何其他说明或提示。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "请将以下 JSON 数据转换为 CSV 文件：  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \n要求：  \n- CSV 中的列应按以下顺序：name、age、city、email  \n- 使用分号（;）作为分隔符  \n- 将所有值用双引号（\"）括起来"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="AWS Bedrock Python">
```python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# for authentication options

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=1000,
temperature=0,
system="作为数据转换专家，您的任务是将不同格式（JSON、XML 等）的数据转换为格式正确的 CSV 文件。用户将提供原始格式的输入数据，以及对 CSV 输出的任何特定要求或偏好（例如，列顺序、分隔符、编码）。确保您清楚了解数据结构和所需的 CSV 格式，根据需要提出任何澄清问题。一旦您获得必要的信息，通过遵循适当的格式规则生成 CSV 输出，例如使用逗号作为分隔符、必要时将值用引号括起来，以及正确处理特殊字符或换行符。最后，提供保存或使用 CSV 文件的任何其他说明或提示。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "请将以下 JSON 数据转换为 CSV 文件： \n \n[ \n { \n \"name\": \"John Doe\", \n \"age\": 30, \n \"city\": \"New York\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Jane Smith\", \n \"age\": 25, \n \"city\": \"London\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Bob Johnson\", \n \"age\": 35, \n \"city\": \"Paris\", \n \"email\": \"[email protected]\" \n } \n] \n \n要求： \n- CSV 中的列应按以下顺序：name、age、city、email \n- 使用分号（;）作为分隔符 \n- 将所有值用双引号（\"）括起来"
}
]
}
]
)
print(message.content)

```
</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "作为数据转换专家，您的任务是将不同格式（JSON、XML 等）的数据转换为格式正确的 CSV 文件。用户将提供原始格式的输入数据，以及对 CSV 输出的任何特定要求或偏好（例如，列顺序、分隔符、编码）。确保您清楚了解数据结构和所需的 CSV 格式，根据需要提出任何澄清问题。一旦您获得必要的信息，通过遵循适当的格式规则生成 CSV 输出，例如使用逗号作为分隔符、必要时将值用引号括起来，以及正确处理特殊字符或换行符。最后，提供保存或使用 CSV 文件的任何其他说明或提示。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "请将以下 JSON 数据转换为 CSV 文件：  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \n要求：  \n- CSV 中的列应按以下顺序：name、age、city、email  \n- 使用分号（;）作为分隔符  \n- 将所有值用双引号（\"）括起来"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=1000,
    temperature=0,
    system="作为数据转换专家，您的任务是将不同格式（JSON、XML 等）的数据转换为格式正确的 CSV 文件。用户将提供原始格式的输入数据，以及对 CSV 输出的任何特定要求或偏好（例如，列顺序、分隔符、编码）。确保您清楚了解数据结构和所需的 CSV 格式，根据需要提出任何澄清问题。一旦您获得必要的信息，通过遵循适当的格式规则生成 CSV 输出，例如使用逗号作为分隔符、必要时将值用引号括起来，以及正确处理特殊字符或换行符。最后，提供保存或使用 CSV 文件的任何其他说明或提示。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "请将以下 JSON 数据转换为 CSV 文件：  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \n要求：  \n- CSV 中的列应按以下顺序：name、age、city、email  \n- 使用分号（;）作为分隔符  \n- 将所有值用双引号（\"）括起来"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 0,
  system: "作为数据转换专家，您的任务是将不同格式（JSON、XML 等）的数据转换为格式正确的 CSV 文件。用户将提供原始格式的输入数据，以及对 CSV 输出的任何特定要求或偏好（例如，列顺序、分隔符、编码）。确保您清楚了解数据结构和所需的 CSV 格式，根据需要提出任何澄清问题。一旦您获得必要的信息，通过遵循适当的格式规则生成 CSV 输出，例如使用逗号作为分隔符、必要时将值用引号括起来，以及正确处理特殊字符或换行符。最后，提供保存或使用 CSV 文件的任何其他说明或提示。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "请将以下 JSON 数据转换为 CSV 文件：  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \n要求：  \n- CSV 中的列应按以下顺序：name、age、city、email  \n- 使用分号（;）作为分隔符  \n- 将所有值用双引号（\"）括起来"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>