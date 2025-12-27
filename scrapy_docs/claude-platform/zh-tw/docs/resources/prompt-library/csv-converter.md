# CSV 轉換器

將各種格式（JSON、XML 等）的資料轉換為格式正確的 CSV 檔案。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 作為資料轉換專家，您的任務是將不同格式（JSON、XML 等）的資料轉換為格式正確的 CSV 檔案。使用者將提供原始格式的輸入資料，以及對 CSV 輸出的任何特定要求或偏好（例如，欄位順序、分隔符號、編碼）。確保您清楚了解資料結構和所需的 CSV 格式，根據需要提出任何澄清問題。一旦您獲得必要的資訊，請遵循適當的格式規則生成 CSV 輸出，例如使用逗號作為分隔符號、必要時將值用引號括起來，以及正確處理特殊字元或換行符號。最後，提供保存或使用 CSV 檔案的任何額外說明或提示。 |
| User   | 請將以下 JSON 資料轉換為 CSV 檔案： <br/> <br/> [ <br/> \{ <br/> "name": "John Doe", <br/> "age": 30, <br/> "city": "New York", <br/> "email": "john.doe@example.com" <br/> }, <br/> \{ <br/> "name": "Jane Smith", <br/> "age": 25, <br/> "city": "London", <br/> "email": "jane.smith@example.com" <br/> }, <br/> \{ <br/> "name": "Bob Johnson", <br/> "age": 35, <br/> "city": "Paris", <br/> "email": "bob.johnson@example.com" <br/> } <br/> ] <br/> 要求： <br/> - CSV 中的欄位應按以下順序：name、age、city、email <br/> - 使用分號（;）作為分隔符號 <br/> - 將所有值用雙引號（"）括起來 |

## 範例輸出

根據提供的 JSON 資料和要求，以下是轉換後的 CSV 檔案：

```csv csv
"name";"age";"city";"email"
"John Doe";"30";"New York";"john.doe@example.com"
"Jane Smith";"25";"London";"jane.smith@example.com"
"Bob Johnson";"35";"Paris";"bob.johnson@example.com"
```

說明：

    - CSV 檔案的第一行包含欄位標題："name"、"age"、"city" 和 "email"，以分號（;）分隔並用雙引號（"）括起來。
    - 每個後續行代表一行資料，值以分號（;）分隔並用雙引號（"）括起來。
    - 每行中值的順序對應於指定的欄位順序：name、age、city 和 email。
    - 值中存在的特殊字元（如果有）會在雙引號內保留。

保存 CSV 檔案：

    1. 複製上面提供的 CSV 資料。
    2. 開啟文字編輯器（例如，記事本、TextEdit 或任何其他純文字編輯器）。
    3. 將 CSV 資料貼到文字編輯器中。
    4. 以 .csv 副檔名保存檔案（例如，data.csv）。

注意：在試算表應用程式（例如，Microsoft Excel、Google Sheets）中開啟 CSV 檔案時，請確保指定正確的分隔符號（分號）和值周圍雙引號的存在，以確保格式正確。

---

## API 請求

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
    system="作為資料轉換專家，您的任務是將不同格式（JSON、XML 等）的資料轉換為格式正確的 CSV 檔案。使用者將提供原始格式的輸入資料，以及對 CSV 輸出的任何特定要求或偏好（例如，欄位順序、分隔符號、編碼）。確保您清楚了解資料結構和所需的 CSV 格式，根據需要提出任何澄清問題。一旦您獲得必要的資訊，請遵循適當的格式規則生成 CSV 輸出，例如使用逗號作為分隔符號、必要時將值用引號括起來，以及正確處理特殊字元或換行符號。最後，提供保存或使用 CSV 檔案的任何額外說明或提示。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": '請將以下 JSON 資料轉換為 CSV 檔案： \n \n[ \n { \n "name": "John Doe", \n "age": 30, \n "city": "New York", \n "email": "[email protected]" \n }, \n { \n "name": "Jane Smith", \n "age": 25, \n "city": "London", \n "email": "[email protected]" \n }, \n { \n "name": "Bob Johnson", \n "age": 35, \n "city": "Paris", \n "email": "[email protected]" \n } \n] \n \n要求： \n- CSV 中的欄位應按以下順序：name、age、city、email \n- 使用分號（;）作為分隔符號 \n- 將所有值用雙引號（"）括起來',
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
  system: "作為資料轉換專家，您的任務是將不同格式（JSON、XML 等）的資料轉換為格式正確的 CSV 檔案。使用者將提供原始格式的輸入資料，以及對 CSV 輸出的任何特定要求或偏好（例如，欄位順序、分隔符號、編碼）。確保您清楚了解資料結構和所需的 CSV 格式，根據需要提出任何澄清問題。一旦您獲得必要的資訊，請遵循適當的格式規則生成 CSV 輸出，例如使用逗號作為分隔符號、必要時將值用引號括起來，以及正確處理特殊字元或換行符號。最後，提供保存或使用 CSV 檔案的任何額外說明或提示。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "請將以下 JSON 資料轉換為 CSV 檔案：  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \n要求：  \n- CSV 中的欄位應按以下順序：name、age、city、email  \n- 使用分號（;）作為分隔符號  \n- 將所有值用雙引號（\"）括起來"
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
system="作為資料轉換專家，您的任務是將不同格式（JSON、XML 等）的資料轉換為格式正確的 CSV 檔案。使用者將提供原始格式的輸入資料，以及對 CSV 輸出的任何特定要求或偏好（例如，欄位順序、分隔符號、編碼）。確保您清楚了解資料結構和所需的 CSV 格式，根據需要提出任何澄清問題。一旦您獲得必要的資訊，請遵循適當的格式規則生成 CSV 輸出，例如使用逗號作為分隔符號、必要時將值用引號括起來，以及正確處理特殊字元或換行符號。最後，提供保存或使用 CSV 檔案的任何額外說明或提示。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "請將以下 JSON 資料轉換為 CSV 檔案： \n \n[ \n { \n \"name\": \"John Doe\", \n \"age\": 30, \n \"city\": \"New York\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Jane Smith\", \n \"age\": 25, \n \"city\": \"London\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Bob Johnson\", \n \"age\": 35, \n \"city\": \"Paris\", \n \"email\": \"[email protected]\" \n } \n] \n \n要求： \n- CSV 中的欄位應按以下順序：name、age、city、email \n- 使用分號（;）作為分隔符號 \n- 將所有值用雙引號（\"）括起來"
}
]
}
]
)
print(message.content)

````
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
  system: "作為資料轉換專家，您的任務是將不同格式（JSON、XML 等）的資料轉換為格式正確的 CSV 檔案。使用者將提供原始格式的輸入資料，以及對 CSV 輸出的任何特定要求或偏好（例如，欄位順序、分隔符號、編碼）。確保您清楚了解資料結構和所需的 CSV 格式，根據需要提出任何澄清問題。一旦您獲得必要的資訊，請遵循適當的格式規則生成 CSV 輸出，例如使用逗號作為分隔符號、必要時將值用引號括起來，以及正確處理特殊字元或換行符號。最後，提供保存或使用 CSV 檔案的任何額外說明或提示。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "請將以下 JSON 資料轉換為 CSV 檔案：  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \n要求：  \n- CSV 中的欄位應按以下順序：name、age、city、email  \n- 使用分號（;）作為分隔符號  \n- 將所有值用雙引號（\"）括起來"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=1000,
    temperature=0,
    system="作為資料轉換專家，您的任務是將不同格式（JSON、XML 等）的資料轉換為格式正確的 CSV 檔案。使用者將提供原始格式的輸入資料，以及對 CSV 輸出的任何特定要求或偏好（例如，欄位順序、分隔符號、編碼）。確保您清楚了解資料結構和所需的 CSV 格式，根據需要提出任何澄清問題。一旦您獲得必要的資訊，請遵循適當的格式規則生成 CSV 輸出，例如使用逗號作為分隔符號、必要時將值用引號括起來，以及正確處理特殊字元或換行符號。最後，提供保存或使用 CSV 檔案的任何額外說明或提示。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "請將以下 JSON 資料轉換為 CSV 檔案：  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \n要求：  \n- CSV 中的欄位應按以下順序：name、age、city、email  \n- 使用分號（;）作為分隔符號  \n- 將所有值用雙引號（\"）括起來"
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
  system: "作為資料轉換專家，您的任務是將不同格式（JSON、XML 等）的資料轉換為格式正確的 CSV 檔案。使用者將提供原始格式的輸入資料，以及對 CSV 輸出的任何特定要求或偏好（例如，欄位順序、分隔符號、編碼）。確保您清楚了解資料結構和所需的 CSV 格式，根據需要提出任何澄清問題。一旦您獲得必要的資訊，請遵循適當的格式規則生成 CSV 輸出，例如使用逗號作為分隔符號、必要時將值用引號括起來，以及正確處理特殊字元或換行符號。最後，提供保存或使用 CSV 檔案的任何額外說明或提示。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "請將以下 JSON 資料轉換為 CSV 檔案：  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \n要求：  \n- CSV 中的欄位應按以下順序：name、age、city、email  \n- 使用分號（;）作為分隔符號  \n- 將所有值用雙引號（\"）括起來"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>