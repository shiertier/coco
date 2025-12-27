# 函數製造器

根據詳細規格創建 Python 函數。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是根據提供的自然語言請求創建 Python 函數。請求將描述函數所需的功能，包括輸入參數和預期返回值。請根據給定的規格實現函數，確保它們處理邊緣情況、執行必要的驗證，並遵循 Python 程式設計的最佳實踐。請在程式碼中包含適當的註釋來解釋邏輯並協助其他開發者理解實現。 |
| User   | 我想要一個可以解決數獨謎題的函數。該函數應該接受一個 9x9 的數獨網格作為輸入，其中空格用值 0 表示。該函數應該使用回溯演算法解決謎題並返回已解決的網格。如果謎題無法解決，它應該返回 None。該函數還應該驗證輸入網格以確保它是一個有效的數獨謎題。 |

## 範例輸出

```python Python
def solve_sudoku(grid):
   def is_valid(row, col, num):
       # 檢查數字在當前行和列中是否有效
       for i in range(9):
           if grid[row][i] == num or grid[i][col] == num:
               return False

       # 檢查數字在當前 3x3 子網格中是否有效
       start_row, start_col = (row // 3) * 3, (col // 3) * 3
       for i in range(3):
           for j in range(3):
               if grid[start_row + i][start_col + j] == num:
                   return False

       return True

   def backtrack(row, col):
       # 如果我們已經填滿所有格子，謎題就解決了
       if row == 9:
           return True

       # 如果我們已經到達當前行的末尾，移動到下一行
       if col == 9:
           return backtrack(row + 1, 0)

       # 如果當前格子已經填滿，移動到下一個格子
       if grid[row][col] != 0:
           return backtrack(row, col + 1)

       # 嘗試用 1 到 9 的數字填充當前格子
       for num in range(1, 10):
           if is_valid(row, col, num):
               grid[row][col] = num
               if backtrack(row, col + 1):
                   return True
               grid[row][col] = 0

       # 如果找不到有效數字，回溯
       return False

   # 驗證輸入網格
   if not all(len(row) == 9 and all(0 <= cell <= 9 for cell in row) for row in grid):
       return None

   # 開始回溯演算法
   if backtrack(0, 0):
       return grid
   else:
       return None

```

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
    system="您的任務是根據提供的自然語言請求創建 Python 函數。請求將描述函數所需的功能，包括輸入參數和預期返回值。請根據給定的規格實現函數，確保它們處理邊緣情況、執行必要的驗證，並遵循 Python 程式設計的最佳實踐。請在程式碼中包含適當的註釋來解釋邏輯並協助其他開發者理解實現。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "我想要一個可以解決數獨謎題的函數。該函數應該接受一個 9x9 的數獨網格作為輸入，其中空格用值 0 表示。該函數應該使用回溯演算法解決謎題並返回已解決的網格。如果謎題無法解決，它應該返回 None。該函數還應該驗證輸入網格以確保它是一個有效的數獨謎題。",
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
  system: "您的任務是根據提供的自然語言請求創建 Python 函數。請求將描述函數所需的功能，包括輸入參數和預期返回值。請根據給定的規格實現函數，確保它們處理邊緣情況、執行必要的驗證，並遵循 Python 程式設計的最佳實踐。請在程式碼中包含適當的註釋來解釋邏輯並協助其他開發者理解實現。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "我想要一個可以解決數獨謎題的函數。該函數應該接受一個 9x9 的數獨網格作為輸入，其中空格用值 0 表示。該函數應該使用回溯演算法解決謎題並返回已解決的網格。如果謎題無法解決，它應該返回 None。該函數還應該驗證輸入網格以確保它是一個有效的數獨謎題。"
        }
      ]
    }
  ]
});
console.log(msg);

````

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
system="您的任務是根據提供的自然語言請求創建 Python 函數。請求將描述函數所需的功能，包括輸入參數和預期返回值。請根據給定的規格實現函數，確保它們處理邊緣情況、執行必要的驗證，並遵循 Python 程式設計的最佳實踐。請在程式碼中包含適當的註釋來解釋邏輯並協助其他開發者理解實現。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "我想要一個可以解決數獨謎題的函數。該函數應該接受一個 9x9 的數獨網格作為輸入，其中空格用值 0 表示。該函數應該使用回溯演算法解決謎題並返回已解決的網格。如果謎題無法解決，它應該返回 None。該函數還應該驗證輸入網格以確保它是一個有效的數獨謎題。"
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
  system: "您的任務是根據提供的自然語言請求創建 Python 函數。請求將描述函數所需的功能，包括輸入參數和預期返回值。請根據給定的規格實現函數，確保它們處理邊緣情況、執行必要的驗證，並遵循 Python 程式設計的最佳實踐。請在程式碼中包含適當的註釋來解釋邏輯並協助其他開發者理解實現。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "我想要一個可以解決數獨謎題的函數。該函數應該接受一個 9x9 的數獨網格作為輸入，其中空格用值 0 表示。該函數應該使用回溯演算法解決謎題並返回已解決的網格。如果謎題無法解決，它應該返回 None。該函數還應該驗證輸入網格以確保它是一個有效的數獨謎題。"
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
system="您的任務是根據提供的自然語言請求創建 Python 函數。請求將描述函數所需的功能，包括輸入參數和預期返回值。請根據給定的規格實現函數，確保它們處理邊緣情況、執行必要的驗證，並遵循 Python 程式設計的最佳實踐。請在程式碼中包含適當的註釋來解釋邏輯並協助其他開發者理解實現。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "我想要一個可以解決數獨謎題的函數。該函數應該接受一個 9x9 的數獨網格作為輸入，其中空格用值 0 表示。該函數應該使用回溯演算法解決謎題並返回已解決的網格。如果謎題無法解決，它應該返回 None。該函數還應該驗證輸入網格以確保它是一個有效的數獨謎題。"
}
]
}
]
)
print(message.content)

````
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
  system: "您的任務是根據提供的自然語言請求創建 Python 函數。請求將描述函數所需的功能，包括輸入參數和預期返回值。請根據給定的規格實現函數，確保它們處理邊緣情況、執行必要的驗證，並遵循 Python 程式設計的最佳實踐。請在程式碼中包含適當的註釋來解釋邏輯並協助其他開發者理解實現。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "我想要一個可以解決數獨謎題的函數。該函數應該接受一個 9x9 的數獨網格作為輸入，其中空格用值 0 表示。該函數應該使用回溯演算法解決謎題並返回已解決的網格。如果謎題無法解決，它應該返回 None。該函數還應該驗證輸入網格以確保它是一個有效的數獨謎題。"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>