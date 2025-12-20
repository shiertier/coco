# 직유 전문가

기본적인 설명으로부터 직유를 생성합니다.

---

> 이 프롬프트를 우리의 개발자 [Console](/dashboard)에 복사해서 직접 시도해보세요!

| Role | Content |
| ---- | --- |
| User | 기쁘고 전염성 있는 사람의 웃음을 묘사하는 직유를 만드는 것을 도와주세요? |

## 예시 출력

1.  그의 웃음은 갓 열린 샴페인 병처럼 거품이 올라왔고, 활기차고 생명력이 넘쳤다.
2.  그들의 웃음은 쌀쌀한 저녁의 아늑한 벽난로처럼 따뜻하고 매력적이어서, 사람들을 끌어들여 즐거움을 함께 나누게 했다.

---

## API 요청

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic(
  # defaults to os.environ.get("ANTHROPIC_API_KEY")
  api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=1000,
  temperature=1,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Help me create some similes to describe a person's laughter that is joyful and contagious?"
        }
      ]
    }
  ]
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
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Help me create some similes to describe a person's laughter that is joyful and contagious?"
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
    temperature=1,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Help me create some similes to describe a person's laughter that is joyful and contagious?"
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
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Help me create some similes to describe a person's laughter that is joyful and contagious?"
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
    temperature=1,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Help me create some similes to describe a person's laughter that is joyful and contagious?"
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
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Help me create some similes to describe a person's laughter that is joyful and contagious?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>