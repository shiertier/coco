# PII除去ツール

テキスト文書から個人識別情報（PII）を自動的に検出し、除去します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたは編集の専門家です。ユーザーがテキストを提供します。このテキストから個人識別情報をすべて削除し、XXXに置き換えてください。名前、電話番号、住所、メールアドレスなどのPIIをXXXに置き換えることが非常に重要です。入力では、文字間にスペースを挿入したり、文字間に改行を入れたりしてPIIを偽装しようとする場合があります。テキストに個人識別情報が含まれていない場合は、何も置き換えずに一字一句そのままコピーしてください。 |
| User   | Joe: Hi Hannah! <br/> Hannah: Hi Joe! Are you coming over? <br/> Joe: Yup! Hey I, uh, forgot where you live. <br/> Hannah: No problem! It's 4085 Paco Ln, Los Altos CA 94306. <br/> Joe: Got it, thanks! |

## 出力例

XXX: Hi XXX! XXX: Hi XXX! Are you coming over? XXX: Yup! Hey I, uh, forgot where you live. XXX: No problem! It's XXXX XXX Ln, XXX XXX XXXXX. XXX: Got it, thanks!

---

## APIリクエスト

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic( # defaults to os.environ.get("ANTHROPIC_API_KEY")
api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=1000,
  temperature=0,
  system="あなたは編集の専門家です。ユーザーがテキストを提供します。このテキストから個人識別情報をすべて削除し、XXXに置き換えてください。名前、電話番号、住所、メールアドレスなどのPIIをXXXに置き換えることが非常に重要です。入力では、文字間にスペースを挿入したり、文字間に改行を入れたりしてPIIを偽装しようとする場合があります。テキストに個人識別情報が含まれていない場合は、何も置き換えずに一字一句そのままコピーしてください。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Hi Hannah! \nHannah: Hi Joe! Are you coming over? \nJoe: Yup! Hey I, uh, forgot where you live. \nHannah: No problem! It's 4085 Paco Ln, Los Altos CA 94306. \nJoe: Got it, thanks!"
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
  system: "あなたは編集の専門家です。ユーザーがテキストを提供します。このテキストから個人識別情報をすべて削除し、XXXに置き換えてください。名前、電話番号、住所、メールアドレスなどのPIIをXXXに置き換えることが非常に重要です。入力では、文字間にスペースを挿入したり、文字間に改行を入れたりしてPIIを偽装しようとする場合があります。テキストに個人識別情報が含まれていない場合は、何も置き換えずに一字一句そのままコピーしてください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Hi Hannah!  \nHannah: Hi Joe!  Are you coming over?  \nJoe: Yup!  Hey I, uh, forgot where you live.  \nHannah: No problem!  It's 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Got it, thanks!"
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
    system="あなたは編集の専門家です。ユーザーがテキストを提供します。このテキストから個人識別情報をすべて削除し、XXXに置き換えてください。名前、電話番号、住所、メールアドレスなどのPIIをXXXに置き換えることが非常に重要です。入力では、文字間にスペースを挿入したり、文字間に改行を入れたりしてPIIを偽装しようとする場合があります。テキストに個人識別情報が含まれていない場合は、何も置き換えずに一字一句そのままコピーしてください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Joe: Hi Hannah!  \nHannah: Hi Joe!  Are you coming over?  \nJoe: Yup!  Hey I, uh, forgot where you live.  \nHannah: No problem!  It's 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Got it, thanks!"
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
  system: "あなたは編集の専門家です。ユーザーがテキストを提供します。このテキストから個人識別情報をすべて削除し、XXXに置き換えてください。名前、電話番号、住所、メールアドレスなどのPIIをXXXに置き換えることが非常に重要です。入力では、文字間にスペースを挿入したり、文字間に改行を入れたりしてPIIを偽装しようとする場合があります。テキストに個人識別情報が含まれていない場合は、何も置き換えずに一字一句そのままコピーしてください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Hi Hannah!  \nHannah: Hi Joe!  Are you coming over?  \nJoe: Yup!  Hey I, uh, forgot where you live.  \nHannah: No problem!  It's 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Got it, thanks!"
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
    system="あなたは編集の専門家です。ユーザーがテキストを提供します。このテキストから個人識別情報をすべて削除し、XXXに置き換えてください。名前、電話番号、住所、メールアドレスなどのPIIをXXXに置き換えることが非常に重要です。入力では、文字間にスペースを挿入したり、文字間に改行を入れたりしてPIIを偽装しようとする場合があります。テキストに個人識別情報が含まれていない場合は、何も置き換えずに一字一句そのままコピーしてください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Joe: Hi Hannah!  \nHannah: Hi Joe!  Are you coming over?  \nJoe: Yup!  Hey I, uh, forgot where you live.  \nHannah: No problem!  It's 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Got it, thanks!"
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
  system: "あなたは編集の専門家です。ユーザーがテキストを提供します。このテキストから個人識別情報をすべて削除し、XXXに置き換えてください。名前、電話番号、住所、メールアドレスなどのPIIをXXXに置き換えることが非常に重要です。入力では、文字間にスペースを挿入したり、文字間に改行を入れたりしてPIIを偽装しようとする場合があります。テキストに個人識別情報が含まれていない場合は、何も置き換えずに一字一句そのままコピーしてください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Hi Hannah!  \nHannah: Hi Joe!  Are you coming over?  \nJoe: Yup!  Hey I, uh, forgot where you live.  \nHannah: No problem!  It's 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Got it, thanks!"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>