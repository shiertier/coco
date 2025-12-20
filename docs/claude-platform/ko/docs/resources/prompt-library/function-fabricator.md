# 함수 제작기

상세한 사양을 기반으로 Python 함수를 생성합니다.

---

> 이 프롬프트를 우리의 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 당신의 작업은 제공된 자연어 요청을 기반으로 Python 함수를 생성하는 것입니다. 요청은 입력 매개변수와 예상 반환 값을 포함하여 함수의 원하는 기능을 설명할 것입니다. 주어진 사양에 따라 함수를 구현하되, 엣지 케이스를 처리하고, 필요한 검증을 수행하며, Python 프로그래밍의 모범 사례를 따르도록 하세요. 로직을 설명하고 다른 개발자들이 구현을 이해할 수 있도록 코드에 적절한 주석을 포함해 주세요. |
| User   | 스도쿠 퍼즐을 해결할 수 있는 함수를 원합니다. 이 함수는 9x9 스도쿠 격자를 입력으로 받아야 하며, 빈 셀은 값 0으로 표현됩니다. 함수는 백트래킹 알고리즘을 사용하여 퍼즐을 해결하고 해결된 격자를 반환해야 합니다. 퍼즐이 해결 불가능한 경우 None을 반환해야 합니다. 함수는 또한 입력 격자가 유효한 스도쿠 퍼즐인지 확인하기 위해 검증해야 합니다. |

## 예시 출력

```python Python
def solve_sudoku(grid):
   def is_valid(row, col, num):
       # 현재 행과 열에서 숫자가 유효한지 확인
       for i in range(9):
           if grid[row][i] == num or grid[i][col] == num:
               return False

       # 현재 3x3 서브그리드에서 숫자가 유효한지 확인
       start_row, start_col = (row // 3) * 3, (col // 3) * 3
       for i in range(3):
           for j in range(3):
               if grid[start_row + i][start_col + j] == num:
                   return False

       return True

   def backtrack(row, col):
       # 모든 셀을 채웠다면 퍼즐이 해결됨
       if row == 9:
           return True

       # 현재 행의 끝에 도달했다면 다음 행으로 이동
       if col == 9:
           return backtrack(row + 1, 0)

       # 현재 셀이 이미 채워져 있다면 다음 셀로 이동
       if grid[row][col] != 0:
           return backtrack(row, col + 1)

       # 현재 셀을 1부터 9까지의 숫자로 채우기 시도
       for num in range(1, 10):
           if is_valid(row, col, num):
               grid[row][col] = num
               if backtrack(row, col + 1):
                   return True
               grid[row][col] = 0

       # 유효한 숫자를 찾지 못했다면 백트래킹
       return False

   # 입력 격자 검증
   if not all(len(row) == 9 and all(0 <= cell <= 9 for cell in row) for row in grid):
       return None

   # 백트래킹 알고리즘 시작
   if backtrack(0, 0):
       return grid
   else:
       return None

```

---

## API 요청

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
    system="당신의 작업은 제공된 자연어 요청을 기반으로 Python 함수를 생성하는 것입니다. 요청은 입력 매개변수와 예상 반환 값을 포함하여 함수의 원하는 기능을 설명할 것입니다. 주어진 사양에 따라 함수를 구현하되, 엣지 케이스를 처리하고, 필요한 검증을 수행하며, Python 프로그래밍의 모범 사례를 따르도록 하세요. 로직을 설명하고 다른 개발자들이 구현을 이해할 수 있도록 코드에 적절한 주석을 포함해 주세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "스도쿠 퍼즐을 해결할 수 있는 함수를 원합니다. 이 함수는 9x9 스도쿠 격자를 입력으로 받아야 하며, 빈 셀은 값 0으로 표현됩니다. 함수는 백트래킹 알고리즘을 사용하여 퍼즐을 해결하고 해결된 격자를 반환해야 합니다. 퍼즐이 해결 불가능한 경우 None을 반환해야 합니다. 함수는 또한 입력 격자가 유효한 스도쿠 퍼즐인지 확인하기 위해 검증해야 합니다.",
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
  system: "당신의 작업은 제공된 자연어 요청을 기반으로 Python 함수를 생성하는 것입니다. 요청은 입력 매개변수와 예상 반환 값을 포함하여 함수의 원하는 기능을 설명할 것입니다. 주어진 사양에 따라 함수를 구현하되, 엣지 케이스를 처리하고, 필요한 검증을 수행하며, Python 프로그래밍의 모범 사례를 따르도록 하세요. 로직을 설명하고 다른 개발자들이 구현을 이해할 수 있도록 코드에 적절한 주석을 포함해 주세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "스도쿠 퍼즐을 해결할 수 있는 함수를 원합니다. 이 함수는 9x9 스도쿠 격자를 입력으로 받아야 하며, 빈 셀은 값 0으로 표현됩니다. 함수는 백트래킹 알고리즘을 사용하여 퍼즐을 해결하고 해결된 격자를 반환해야 합니다. 퍼즐이 해결 불가능한 경우 None을 반환해야 합니다. 함수는 또한 입력 격자가 유효한 스도쿠 퍼즐인지 확인하기 위해 검증해야 합니다."
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
system="당신의 작업은 제공된 자연어 요청을 기반으로 Python 함수를 생성하는 것입니다. 요청은 입력 매개변수와 예상 반환 값을 포함하여 함수의 원하는 기능을 설명할 것입니다. 주어진 사양에 따라 함수를 구현하되, 엣지 케이스를 처리하고, 필요한 검증을 수행하며, Python 프로그래밍의 모범 사례를 따르도록 하세요. 로직을 설명하고 다른 개발자들이 구현을 이해할 수 있도록 코드에 적절한 주석을 포함해 주세요.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "스도쿠 퍼즐을 해결할 수 있는 함수를 원합니다. 이 함수는 9x9 스도쿠 격자를 입력으로 받아야 하며, 빈 셀은 값 0으로 표현됩니다. 함수는 백트래킹 알고리즘을 사용하여 퍼즐을 해결하고 해결된 격자를 반환해야 합니다. 퍼즐이 해결 불가능한 경우 None을 반환해야 합니다. 함수는 또한 입력 격자가 유효한 스도쿠 퍼즐인지 확인하기 위해 검증해야 합니다."
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
  system: "당신의 작업은 제공된 자연어 요청을 기반으로 Python 함수를 생성하는 것입니다. 요청은 입력 매개변수와 예상 반환 값을 포함하여 함수의 원하는 기능을 설명할 것입니다. 주어진 사양에 따라 함수를 구현하되, 엣지 케이스를 처리하고, 필요한 검증을 수행하며, Python 프로그래밍의 모범 사례를 따르도록 하세요. 로직을 설명하고 다른 개발자들이 구현을 이해할 수 있도록 코드에 적절한 주석을 포함해 주세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "스도쿠 퍼즐을 해결할 수 있는 함수를 원합니다. 이 함수는 9x9 스도쿠 격자를 입력으로 받아야 하며, 빈 셀은 값 0으로 표현됩니다. 함수는 백트래킹 알고리즘을 사용하여 퍼즐을 해결하고 해결된 격자를 반환해야 합니다. 퍼즐이 해결 불가능한 경우 None을 반환해야 합니다. 함수는 또한 입력 격자가 유효한 스도쿠 퍼즐인지 확인하기 위해 검증해야 합니다."
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
system="당신의 작업은 제공된 자연어 요청을 기반으로 Python 함수를 생성하는 것입니다. 요청은 입력 매개변수와 예상 반환 값을 포함하여 함수의 원하는 기능을 설명할 것입니다. 주어진 사양에 따라 함수를 구현하되, 엣지 케이스를 처리하고, 필요한 검증을 수행하며, Python 프로그래밍의 모범 사례를 따르도록 하세요. 로직을 설명하고 다른 개발자들이 구현을 이해할 수 있도록 코드에 적절한 주석을 포함해 주세요.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "스도쿠 퍼즐을 해결할 수 있는 함수를 원합니다. 이 함수는 9x9 스도쿠 격자를 입력으로 받아야 하며, 빈 셀은 값 0으로 표현됩니다. 함수는 백트래킹 알고리즘을 사용하여 퍼즐을 해결하고 해결된 격자를 반환해야 합니다. 퍼즐이 해결 불가능한 경우 None을 반환해야 합니다. 함수는 또한 입력 격자가 유효한 스도쿠 퍼즐인지 확인하기 위해 검증해야 합니다."
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
  system: "당신의 작업은 제공된 자연어 요청을 기반으로 Python 함수를 생성하는 것입니다. 요청은 입력 매개변수와 예상 반환 값을 포함하여 함수의 원하는 기능을 설명할 것입니다. 주어진 사양에 따라 함수를 구현하되, 엣지 케이스를 처리하고, 필요한 검증을 수행하며, Python 프로그래밍의 모범 사례를 따르도록 하세요. 로직을 설명하고 다른 개발자들이 구현을 이해할 수 있도록 코드에 적절한 주석을 포함해 주세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "스도쿠 퍼즐을 해결할 수 있는 함수를 원합니다. 이 함수는 9x9 스도쿠 격자를 입력으로 받아야 하며, 빈 셀은 값 0으로 표현됩니다. 함수는 백트래킹 알고리즘을 사용하여 퍼즐을 해결하고 해결된 격자를 반환해야 합니다. 퍼즐이 해결 불가능한 경우 None을 반환해야 합니다. 함수는 또한 입력 격자가 유효한 스도쿠 퍼즐인지 확인하기 위해 검증해야 합니다."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>