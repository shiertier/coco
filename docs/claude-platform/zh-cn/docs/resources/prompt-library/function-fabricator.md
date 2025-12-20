# 函数制造器

根据详细规范创建Python函数。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是根据提供的自然语言请求创建Python函数。请求将描述函数的所需功能，包括输入参数和预期返回值。根据给定的规范实现函数，确保它们处理边缘情况，执行必要的验证，并遵循Python编程的最佳实践。请在代码中包含适当的注释来解释逻辑并帮助其他开发者理解实现。 |
| User   | 我想要一个可以解决数独谜题的函数。该函数应该接受一个9x9的数独网格作为输入，其中空单元格用值0表示。该函数应该使用回溯算法解决谜题并返回已解决的网格。如果谜题无法解决，它应该返回None。该函数还应该验证输入网格以确保它是一个有效的数独谜题。 |

## 示例输出

```python Python
def solve_sudoku(grid):
   def is_valid(row, col, num):
       # 检查数字在当前行和列中是否有效
       for i in range(9):
           if grid[row][i] == num or grid[i][col] == num:
               return False

       # 检查数字在当前3x3子网格中是否有效
       start_row, start_col = (row // 3) * 3, (col // 3) * 3
       for i in range(3):
           for j in range(3):
               if grid[start_row + i][start_col + j] == num:
                   return False

       return True

   def backtrack(row, col):
       # 如果我们已经填满了所有单元格，谜题就解决了
       if row == 9:
           return True

       # 如果我们已经到达当前行的末尾，移动到下一行
       if col == 9:
           return backtrack(row + 1, 0)

       # 如果当前单元格已经填充，移动到下一个单元格
       if grid[row][col] != 0:
           return backtrack(row, col + 1)

       # 尝试用1到9的数字填充当前单元格
       for num in range(1, 10):
           if is_valid(row, col, num):
               grid[row][col] = num
               if backtrack(row, col + 1):
                   return True
               grid[row][col] = 0

       # 如果没有找到有效数字，回溯
       return False

   # 验证输入网格
   if not all(len(row) == 9 and all(0 <= cell <= 9 for cell in row) for row in grid):
       return None

   # 开始回溯算法
   if backtrack(0, 0):
       return grid
   else:
       return None

```

---

## API请求

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
    system="您的任务是根据提供的自然语言请求创建Python函数。请求将描述函数的所需功能，包括输入参数和预期返回值。根据给定的规范实现函数，确保它们处理边缘情况，执行必要的验证，并遵循Python编程的最佳实践。请在代码中包含适当的注释来解释逻辑并帮助其他开发者理解实现。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "我想要一个可以解决数独谜题的函数。该函数应该接受一个9x9的数独网格作为输入，其中空单元格用值0表示。该函数应该使用回溯算法解决谜题并返回已解决的网格。如果谜题无法解决，它应该返回None。该函数还应该验证输入网格以确保它是一个有效的数独谜题。",
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
  system: "您的任务是根据提供的自然语言请求创建Python函数。请求将描述函数的所需功能，包括输入参数和预期返回值。根据给定的规范实现函数，确保它们处理边缘情况，执行必要的验证，并遵循Python编程的最佳实践。请在代码中包含适当的注释来解释逻辑并帮助其他开发者理解实现。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "我想要一个可以解决数独谜题的函数。该函数应该接受一个9x9的数独网格作为输入，其中空单元格用值0表示。该函数应该使用回溯算法解决谜题并返回已解决的网格。如果谜题无法解决，它应该返回None。该函数还应该验证输入网格以确保它是一个有效的数独谜题。"
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
system="您的任务是根据提供的自然语言请求创建Python函数。请求将描述函数的所需功能，包括输入参数和预期返回值。根据给定的规范实现函数，确保它们处理边缘情况，执行必要的验证，并遵循Python编程的最佳实践。请在代码中包含适当的注释来解释逻辑并帮助其他开发者理解实现。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "我想要一个可以解决数独谜题的函数。该函数应该接受一个9x9的数独网格作为输入，其中空单元格用值0表示。该函数应该使用回溯算法解决谜题并返回已解决的网格。如果谜题无法解决，它应该返回None。该函数还应该验证输入网格以确保它是一个有效的数独谜题。"
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
  system: "您的任务是根据提供的自然语言请求创建Python函数。请求将描述函数的所需功能，包括输入参数和预期返回值。根据给定的规范实现函数，确保它们处理边缘情况，执行必要的验证，并遵循Python编程的最佳实践。请在代码中包含适当的注释来解释逻辑并帮助其他开发者理解实现。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "我想要一个可以解决数独谜题的函数。该函数应该接受一个9x9的数独网格作为输入，其中空单元格用值0表示。该函数应该使用回溯算法解决谜题并返回已解决的网格。如果谜题无法解决，它应该返回None。该函数还应该验证输入网格以确保它是一个有效的数独谜题。"
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
system="您的任务是根据提供的自然语言请求创建Python函数。请求将描述函数的所需功能，包括输入参数和预期返回值。根据给定的规范实现函数，确保它们处理边缘情况，执行必要的验证，并遵循Python编程的最佳实践。请在代码中包含适当的注释来解释逻辑并帮助其他开发者理解实现。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "我想要一个可以解决数独谜题的函数。该函数应该接受一个9x9的数独网格作为输入，其中空单元格用值0表示。该函数应该使用回溯算法解决谜题并返回已解决的网格。如果谜题无法解决，它应该返回None。该函数还应该验证输入网格以确保它是一个有效的数独谜题。"
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
  system: "您的任务是根据提供的自然语言请求创建Python函数。请求将描述函数的所需功能，包括输入参数和预期返回值。根据给定的规范实现函数，确保它们处理边缘情况，执行必要的验证，并遵循Python编程的最佳实践。请在代码中包含适当的注释来解释逻辑并帮助其他开发者理解实现。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "我想要一个可以解决数独谜题的函数。该函数应该接受一个9x9的数独网格作为输入，其中空单元格用值0表示。该函数应该使用回溯算法解决谜题并返回已解决的网格。如果谜题无法解决，它应该返回None。该函数还应该验证输入网格以确保它是一个有效的数独谜题。"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>