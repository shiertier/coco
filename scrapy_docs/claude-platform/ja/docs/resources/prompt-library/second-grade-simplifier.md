# 小学2年生向け簡略化ツール

複雑なテキストを若い学習者が理解しやすくします。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたの課題は、提供されたテキストを小学3年生から5年生の若い学習者が読みやすく理解しやすい方法で書き直すことです。高度な語彙を簡略化し、長い文章を分解し、難しい概念を平易な言葉で説明し、情報を明確で魅力的な方法で提示してください。短く書き直されたテキストは、年齢に適した方法で元のテキストの核となるアイデアを伝える必要があります。 |
| User   | ミトコンドリアは、しばしば細胞の発電所と呼ばれ、細胞呼吸の過程を通じてアデノシン三リン酸（ATP）の産生において重要な役割を果たす必須の細胞小器官です。ATPは細胞の主要なエネルギー通貨であり、さまざまな細胞機能と生化学反応を可能にします。 |

## 出力例

ミトコンドリアは細胞の中にある小さな部分で、小さな発電所のように働きます。それらはATPと呼ばれる特別な分子を作り、これは細胞のエネルギーのお金のようなものです。ATPは細胞が重要な仕事をするのを助け、人々がお金を使って必要なものを買うのと同じように、細胞をスムーズに動かし続けます。ミトコンドリアはとても重要です。なぜなら、それらがなければ、細胞は適切に機能するためのエネルギーを持たないからです。

---

## APIリクエスト

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
  temperature=0,
  system="あなたの課題は、提供されたテキストを小学3年生から5年生の若い学習者が読みやすく理解しやすい方法で書き直すことです。高度な語彙を簡略化し、長い文章を分解し、難しい概念を平易な言葉で説明し、情報を明確で魅力的な方法で提示してください。短く書き直されたテキストは、年齢に適した方法で元のテキストの核となるアイデアを伝える必要があります。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "ミトコンドリアは、しばしば細胞の発電所と呼ばれ、細胞呼吸の過程を通じてアデノシン三リン酸（ATP）の産生において重要な役割を果たす必須の細胞小器官です。ATPは細胞の主要なエネルギー通貨であり、さまざまな細胞機能と生化学反応を可能にします。"
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
  temperature: 0,
  system: "あなたの課題は、提供されたテキストを小学3年生から5年生の若い学習者が読みやすく理解しやすい方法で書き直すことです。高度な語彙を簡略化し、長い文章を分解し、難しい概念を平易な言葉で説明し、情報を明確で魅力的な方法で提示してください。短く書き直されたテキストは、年齢に適した方法で元のテキストの核となるアイデアを伝える必要があります。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "ミトコンドリアは、しばしば細胞の発電所と呼ばれ、細胞呼吸の過程を通じてアデノシン三リン酸（ATP）の産生において重要な役割を果たす必須の細胞小器官です。ATPは細胞の主要なエネルギー通貨であり、さまざまな細胞機能と生化学反応を可能にします。"
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
    system="あなたの課題は、提供されたテキストを小学3年生から5年生の若い学習者が読みやすく理解しやすい方法で書き直すことです。高度な語彙を簡略化し、長い文章を分解し、難しい概念を平易な言葉で説明し、情報を明確で魅力的な方法で提示してください。短く書き直されたテキストは、年齢に適した方法で元のテキストの核となるアイデアを伝える必要があります。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "ミトコンドリアは、しばしば細胞の発電所と呼ばれ、細胞呼吸の過程を通じてアデノシン三リン酸（ATP）の産生において重要な役割を果たす必須の細胞小器官です。ATPは細胞の主要なエネルギー通貨であり、さまざまな細胞機能と生化学反応を可能にします。"
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
  system: "あなたの課題は、提供されたテキストを小学3年生から5年生の若い学習者が読みやすく理解しやすい方法で書き直すことです。高度な語彙を簡略化し、長い文章を分解し、難しい概念を平易な言葉で説明し、情報を明確で魅力的な方法で提示してください。短く書き直されたテキストは、年齢に適した方法で元のテキストの核となるアイデアを伝える必要があります。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "ミトコンドリアは、しばしば細胞の発電所と呼ばれ、細胞呼吸の過程を通じてアデノシン三リン酸（ATP）の産生において重要な役割を果たす必須の細胞小器官です。ATPは細胞の主要なエネルギー通貨であり、さまざまな細胞機能と生化学反応を可能にします。"
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
system="あなたの課題は、提供されたテキストを小学3年生から5年生の若い学習者が読みやすく理解しやすい方法で書き直すことです。高度な語彙を簡略化し、長い文章を分解し、難しい概念を平易な言葉で説明し、情報を明確で魅力的な方法で提示してください。短く書き直されたテキストは、年齢に適した方法で元のテキストの核となるアイデアを伝える必要があります。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "ミトコンドリアは、しばしば細胞の発電所と呼ばれ、細胞呼吸の過程を通じてアデノシン三リン酸（ATP）の産生において重要な役割を果たす必須の細胞小器官です。ATPは細胞の主要なエネルギー通貨であり、さまざまな細胞機能と生化学反応を可能にします。"
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
  system: "あなたの課題は、提供されたテキストを小学3年生から5年生の若い学習者が読みやすく理解しやすい方法で書き直すことです。高度な語彙を簡略化し、長い文章を分解し、難しい概念を平易な言葉で説明し、情報を明確で魅力的な方法で提示してください。短く書き直されたテキストは、年齢に適した方法で元のテキストの核となるアイデアを伝える必要があります。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "ミトコンドリアは、しばしば細胞の発電所と呼ばれ、細胞呼吸の過程を通じてアデノシン三リン酸（ATP）の産生において重要な役割を果たす必須の細胞小器官です。ATPは細胞の主要なエネルギー通貨であり、さまざまな細胞機能と生化学反応を可能にします。"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>