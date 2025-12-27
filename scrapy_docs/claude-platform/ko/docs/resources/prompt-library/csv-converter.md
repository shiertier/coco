# CSV 변환기

다양한 형식(JSON, XML 등)의 데이터를 적절히 형식화된 CSV 파일로 변환합니다.

---

> 이 프롬프트를 우리의 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 데이터 변환 전문가로서, 당신의 임무는 다양한 형식(JSON, XML 등)의 데이터를 적절히 형식화된 CSV 파일로 변환하는 것입니다. 사용자는 원본 형식의 입력 데이터와 함께 CSV 출력에 대한 특정 요구사항이나 선호사항(예: 열 순서, 구분자, 인코딩)을 제공할 것입니다. 데이터 구조와 원하는 CSV 형식을 명확히 이해하고, 필요에 따라 명확한 질문을 하세요. 필요한 정보를 얻은 후, 쉼표를 구분자로 사용하고, 필요시 값을 따옴표로 감싸며, 특수 문자나 줄바꿈을 올바르게 처리하는 등 적절한 형식 규칙을 따라 CSV 출력을 생성하세요. 마지막으로 CSV 파일을 저장하거나 사용하기 위한 추가 지침이나 팁을 제공하세요. |
| User   | 다음 JSON 데이터를 CSV 파일로 변환해주세요: <br/> <br/> [ <br/> \{ <br/> "name": "John Doe", <br/> "age": 30, <br/> "city": "New York", <br/> "email": "john.doe@example.com" <br/> }, <br/> \{ <br/> "name": "Jane Smith", <br/> "age": 25, <br/> "city": "London", <br/> "email": "jane.smith@example.com" <br/> }, <br/> \{ <br/> "name": "Bob Johnson", <br/> "age": 35, <br/> "city": "Paris", <br/> "email": "bob.johnson@example.com" <br/> } <br/> ] <br/> 요구사항: <br/> - CSV의 열은 다음 순서여야 합니다: name, age, city, email <br/> - 구분자로 세미콜론(;)을 사용하세요 <br/> - 모든 값을 큰따옴표(")로 감싸세요 |

## 예시 출력

제공된 JSON 데이터와 요구사항을 바탕으로, 변환된 CSV 파일은 다음과 같습니다:

```csv csv
"name";"age";"city";"email"
"John Doe";"30";"New York";"john.doe@example.com"
"Jane Smith";"25";"London";"jane.smith@example.com"
"Bob Johnson";"35";"Paris";"bob.johnson@example.com"
```

설명:

    - CSV 파일의 첫 번째 줄은 열 헤더를 포함합니다: "name", "age", "city", "email"이며, 세미콜론(;)으로 구분되고 큰따옴표(")로 감싸져 있습니다.
    - 각 후속 줄은 데이터 행을 나타내며, 값들은 세미콜론(;)으로 구분되고 큰따옴표(")로 감싸져 있습니다.
    - 각 행의 값 순서는 지정된 열 순서에 해당합니다: name, age, city, email.
    - 값에 특수 문자가 있는 경우, 큰따옴표 안에서 보존됩니다.

CSV 파일을 저장하려면:

    1. 위에 제공된 CSV 데이터를 복사하세요.
    2. 텍스트 편집기(예: 메모장, TextEdit 또는 기타 일반 텍스트 편집기)를 여세요.
    3. CSV 데이터를 텍스트 편집기에 붙여넣으세요.
    4. 파일을 .csv 확장자로 저장하세요(예: data.csv).

참고: 스프레드시트 애플리케이션(예: Microsoft Excel, Google Sheets)에서 CSV 파일을 열 때, 적절한 형식을 보장하기 위해 올바른 구분자(세미콜론)와 값 주위의 큰따옴표 존재를 지정해야 합니다.

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
    system="데이터 변환 전문가로서, 당신의 임무는 다양한 형식(JSON, XML 등)의 데이터를 적절히 형식화된 CSV 파일로 변환하는 것입니다. 사용자는 원본 형식의 입력 데이터와 함께 CSV 출력에 대한 특정 요구사항이나 선호사항(예: 열 순서, 구분자, 인코딩)을 제공할 것입니다. 데이터 구조와 원하는 CSV 형식을 명확히 이해하고, 필요에 따라 명확한 질문을 하세요. 필요한 정보를 얻은 후, 쉼표를 구분자로 사용하고, 필요시 값을 따옴표로 감싸며, 특수 문자나 줄바꿈을 올바르게 처리하는 등 적절한 형식 규칙을 따라 CSV 출력을 생성하세요. 마지막으로 CSV 파일을 저장하거나 사용하기 위한 추가 지침이나 팁을 제공하세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": '다음 JSON 데이터를 CSV 파일로 변환해주세요: \n \n[ \n { \n "name": "John Doe", \n "age": 30, \n "city": "New York", \n "email": "[email protected]" \n }, \n { \n "name": "Jane Smith", \n "age": 25, \n "city": "London", \n "email": "[email protected]" \n }, \n { \n "name": "Bob Johnson", \n "age": 35, \n "city": "Paris", \n "email": "[email protected]" \n } \n] \n \n요구사항: \n- CSV의 열은 다음 순서여야 합니다: name, age, city, email \n- 구분자로 세미콜론(;)을 사용하세요 \n- 모든 값을 큰따옴표(")로 감싸세요',
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
  system: "데이터 변환 전문가로서, 당신의 임무는 다양한 형식(JSON, XML 등)의 데이터를 적절히 형식화된 CSV 파일로 변환하는 것입니다. 사용자는 원본 형식의 입력 데이터와 함께 CSV 출력에 대한 특정 요구사항이나 선호사항(예: 열 순서, 구분자, 인코딩)을 제공할 것입니다. 데이터 구조와 원하는 CSV 형식을 명확히 이해하고, 필요에 따라 명확한 질문을 하세요. 필요한 정보를 얻은 후, 쉼표를 구분자로 사용하고, 필요시 값을 따옴표로 감싸며, 특수 문자나 줄바꿈을 올바르게 처리하는 등 적절한 형식 규칙을 따라 CSV 출력을 생성하세요. 마지막으로 CSV 파일을 저장하거나 사용하기 위한 추가 지침이나 팁을 제공하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "다음 JSON 데이터를 CSV 파일로 변환해주세요:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \n요구사항:  \n- CSV의 열은 다음 순서여야 합니다: name, age, city, email  \n- 구분자로 세미콜론(;)을 사용하세요  \n- 모든 값을 큰따옴표(\")로 감싸세요"
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
system="데이터 변환 전문가로서, 당신의 임무는 다양한 형식(JSON, XML 등)의 데이터를 적절히 형식화된 CSV 파일로 변환하는 것입니다. 사용자는 원본 형식의 입력 데이터와 함께 CSV 출력에 대한 특정 요구사항이나 선호사항(예: 열 순서, 구분자, 인코딩)을 제공할 것입니다. 데이터 구조와 원하는 CSV 형식을 명확히 이해하고, 필요에 따라 명확한 질문을 하세요. 필요한 정보를 얻은 후, 쉼표를 구분자로 사용하고, 필요시 값을 따옴표로 감싸며, 특수 문자나 줄바꿈을 올바르게 처리하는 등 적절한 형식 규칙을 따라 CSV 출력을 생성하세요. 마지막으로 CSV 파일을 저장하거나 사용하기 위한 추가 지침이나 팁을 제공하세요.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "다음 JSON 데이터를 CSV 파일로 변환해주세요: \n \n[ \n { \n \"name\": \"John Doe\", \n \"age\": 30, \n \"city\": \"New York\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Jane Smith\", \n \"age\": 25, \n \"city\": \"London\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Bob Johnson\", \n \"age\": 35, \n \"city\": \"Paris\", \n \"email\": \"[email protected]\" \n } \n] \n \n요구사항: \n- CSV의 열은 다음 순서여야 합니다: name, age, city, email \n- 구분자로 세미콜론(;)을 사용하세요 \n- 모든 값을 큰따옴표(\")로 감싸세요"
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
  system: "데이터 변환 전문가로서, 당신의 임무는 다양한 형식(JSON, XML 등)의 데이터를 적절히 형식화된 CSV 파일로 변환하는 것입니다. 사용자는 원본 형식의 입력 데이터와 함께 CSV 출력에 대한 특정 요구사항이나 선호사항(예: 열 순서, 구분자, 인코딩)을 제공할 것입니다. 데이터 구조와 원하는 CSV 형식을 명확히 이해하고, 필요에 따라 명확한 질문을 하세요. 필요한 정보를 얻은 후, 쉼표를 구분자로 사용하고, 필요시 값을 따옴표로 감싸며, 특수 문자나 줄바꿈을 올바르게 처리하는 등 적절한 형식 규칙을 따라 CSV 출력을 생성하세요. 마지막으로 CSV 파일을 저장하거나 사용하기 위한 추가 지침이나 팁을 제공하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "다음 JSON 데이터를 CSV 파일로 변환해주세요:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \n요구사항:  \n- CSV의 열은 다음 순서여야 합니다: name, age, city, email  \n- 구분자로 세미콜론(;)을 사용하세요  \n- 모든 값을 큰따옴표(\")로 감싸세요"
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
    system="데이터 변환 전문가로서, 당신의 임무는 다양한 형식(JSON, XML 등)의 데이터를 적절히 형식화된 CSV 파일로 변환하는 것입니다. 사용자는 원본 형식의 입력 데이터와 함께 CSV 출력에 대한 특정 요구사항이나 선호사항(예: 열 순서, 구분자, 인코딩)을 제공할 것입니다. 데이터 구조와 원하는 CSV 형식을 명확히 이해하고, 필요에 따라 명확한 질문을 하세요. 필요한 정보를 얻은 후, 쉼표를 구분자로 사용하고, 필요시 값을 따옴표로 감싸며, 특수 문자나 줄바꿈을 올바르게 처리하는 등 적절한 형식 규칙을 따라 CSV 출력을 생성하세요. 마지막으로 CSV 파일을 저장하거나 사용하기 위한 추가 지침이나 팁을 제공하세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "다음 JSON 데이터를 CSV 파일로 변환해주세요:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \n요구사항:  \n- CSV의 열은 다음 순서여야 합니다: name, age, city, email  \n- 구분자로 세미콜론(;)을 사용하세요  \n- 모든 값을 큰따옴표(\")로 감싸세요"
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
  system: "데이터 변환 전문가로서, 당신의 임무는 다양한 형식(JSON, XML 등)의 데이터를 적절히 형식화된 CSV 파일로 변환하는 것입니다. 사용자는 원본 형식의 입력 데이터와 함께 CSV 출력에 대한 특정 요구사항이나 선호사항(예: 열 순서, 구분자, 인코딩)을 제공할 것입니다. 데이터 구조와 원하는 CSV 형식을 명확히 이해하고, 필요에 따라 명확한 질문을 하세요. 필요한 정보를 얻은 후, 쉼표를 구분자로 사용하고, 필요시 값을 따옴표로 감싸며, 특수 문자나 줄바꿈을 올바르게 처리하는 등 적절한 형식 규칙을 따라 CSV 출력을 생성하세요. 마지막으로 CSV 파일을 저장하거나 사용하기 위한 추가 지침이나 팁을 제공하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "다음 JSON 데이터를 CSV 파일로 변환해주세요:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n        \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \n요구사항:  \n- CSV의 열은 다음 순서여야 합니다: name, age, city, email  \n- 구분자로 세미콜론(;)을 사용하세요  \n- 모든 값을 큰따옴표(\")로 감싸세요"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>