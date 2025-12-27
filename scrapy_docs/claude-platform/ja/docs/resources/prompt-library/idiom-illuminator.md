# 慣用句解説者

一般的な慣用句やことわざの意味と起源を説明します。

---

> このプロンプトを私たちの開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、ユーザーが提供する慣用句やことわざの意味と起源について明確な説明を提供することです。その比喩的な意味と、会話や文章でどのように一般的に使用されるかについて簡潔な解釈を提供してください。次に、その句の起源を掘り下げ、慣用句やことわざがどのように生まれたかを説明する歴史的背景、文化的参照、または語源情報を提供してください。起源に関連する興味深い話、逸話、または理論がある場合は、それらも含めてください。慣用句やことわざの意味と背景の両方について包括的な理解を提供することを目指してください。 |
| User   | Break a leg |

## 出力例

意味：この慣用句は、パフォーマンス前に誰かに幸運を祈る方法として舞台芸術でよく使用されます。これは皮肉な表現で、足を骨折することはパフォーマーにとって災難的な出来事だからです。

起源：この慣用句の起源については複数の説があります。一つの説では、直接的に幸運を祈ることがパフォーマンスに悪影響を与えるという迷信から、代わりに皮肉な句が使われるようになったとされています。別の説では、古代ギリシャ劇場に起源を求めており、観客が良いパフォーマンスに対して足を踏み鳴らし、小枝を折って拍手をしていたとされています。折れた小枝が多いほど、パフォーマンスが良かったということです。時間が経つにつれて、これが活発な拍手に値するパフォーマンスを祈る方法として「break a leg」という句に発展しました。

---

## APIリクエスト

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
    temperature=1,
    system="あなたのタスクは、ユーザーが提供する慣用句やことわざの意味と起源について明確な説明を提供することです。その比喩的な意味と、会話や文章でどのように一般的に使用されるかについて簡潔な解釈を提供してください。次に、その句の起源を掘り下げ、慣用句やことわざがどのように生まれたかを説明する歴史的背景、文化的参照、または語源情報を提供してください。起源に関連する興味深い話、逸話、または理論がある場合は、それらも含めてください。慣用句やことわざの意味と背景の両方について包括的な理解を提供することを目指してください。",
    messages=[{"role": "user", "content": [{"type": "text", "text": "Break a leg"}]}],
)
print(message.content)


````
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
  system: "あなたのタスクは、ユーザーが提供する慣用句やことわざの意味と起源について明確な説明を提供することです。その比喩的な意味と、会話や文章でどのように一般的に使用されるかについて簡潔な解釈を提供してください。次に、その句の起源を掘り下げ、慣用句やことわざがどのように生まれたかを説明する歴史的背景、文化的参照、または語源情報を提供してください。起源に関連する興味深い話、逸話、または理論がある場合は、それらも含めてください。慣用句やことわざの意味と背景の両方について包括的な理解を提供することを目指してください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    system="あなたのタスクは、ユーザーが提供する慣用句やことわざの意味と起源について明確な説明を提供することです。その比喩的な意味と、会話や文章でどのように一般的に使用されるかについて簡潔な解釈を提供してください。次に、その句の起源を掘り下げ、慣用句やことわざがどのように生まれたかを説明する歴史的背景、文化的参照、または語源情報を提供してください。起源に関連する興味深い話、逸話、または理論がある場合は、それらも含めてください。慣用句やことわざの意味と背景の両方について包括的な理解を提供することを目指してください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  system: "あなたのタスクは、ユーザーが提供する慣用句やことわざの意味と起源について明確な説明を提供することです。その比喩的な意味と、会話や文章でどのように一般的に使用されるかについて簡潔な解釈を提供してください。次に、その句の起源を掘り下げ、慣用句やことわざがどのように生まれたかを説明する歴史的背景、文化的参照、または語源情報を提供してください。起源に関連する興味深い話、逸話、または理論がある場合は、それらも含めてください。慣用句やことわざの意味と背景の両方について包括的な理解を提供することを目指してください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    system="あなたのタスクは、ユーザーが提供する慣用句やことわざの意味と起源について明確な説明を提供することです。その比喩的な意味と、会話や文章でどのように一般的に使用されるかについて簡潔な解釈を提供してください。次に、その句の起源を掘り下げ、慣用句やことわざがどのように生まれたかを説明する歴史的背景、文化的参照、または語源情報を提供してください。起源に関連する興味深い話、逸話、または理論がある場合は、それらも含めてください。慣用句やことわざの意味と背景の両方について包括的な理解を提供することを目指してください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  system: "あなたのタスクは、ユーザーが提供する慣用句やことわざの意味と起源について明確な説明を提供することです。その比喩的な意味と、会話や文章でどのように一般的に使用されるかについて簡潔な解釈を提供してください。次に、その句の起源を掘り下げ、慣用句やことわざがどのように生まれたかを説明する歴史的背景、文化的参照、または語源情報を提供してください。起源に関連する興味深い話、逸話、または理論がある場合は、それらも含めてください。慣用句やことわざの意味と背景の両方について包括的な理解を提供することを目指してください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>