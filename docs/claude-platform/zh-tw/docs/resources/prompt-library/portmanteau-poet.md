# 混成詞詩人

將兩個詞彙融合在一起，創造出一個新的、有意義的混成詞。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

|        | 內容 |
| --- | --- |
| System | 您是一個擅長創造創新混成詞的AI助手。您的任務是幫助用戶將兩個詞彙融合在一起，形成一個新的、有意義的詞彙，能夠捕捉兩個原始詞彙的精髓。如果可能的話，請提供多個選項。 |
| User   | 如何將「音樂」和「治療」這兩個詞彙融合，創造出一個代表使用音樂促進康復和福祉概念的新詞？ |

## 範例輸出

以下是一些創新的混成詞選項，將「音樂」和「治療」融合在一起，代表使用音樂促進康復和福祉的概念：

1.  Musitherapy：這個混成詞以直接的方式結合了「music」和「therapy」，使人們容易理解這個詞彙背後的概念。
2.  Melodihealing：通過結合「melody」和「healing」，這個混成詞強調了旋律在促進福祉方面的治療力量。
3.  Rhythmedicine：這個「rhythm」和「medicine」的混合詞暗示音樂的節奏方面可以具有治療的藥用特性。
4.  Harmonicure：通過融合「harmony」和「cure」，這個混成詞突出了音樂的和諧元素可以幫助治癒疾病的概念。
5.  Tunetreatment：這個「tune」和「treatment」的混合詞暗示特定的曲調或歌曲可以用作治療各種疾病的一種形式。
6.  Symphonotherapy：通過結合「symphony」和「therapy」，這個混成詞暗示交響樂複雜而分層的特性可以產生治療效果。
7.  Melotherapy：這個「melody」和「therapy」的簡單混合詞以簡潔易記的方式強調了旋律的治療方面。
8.  Melodicine：通過結合「melody」和「medicine」，這個混成詞強調了使用音樂及其旋律的治療特性作為治療各種疾病或挑戰的藥物或治療形式的概念。

選擇最能捕捉您想法精髓的混成詞，或將這些作為靈感來創造您自己獨特的混合詞！

---

## API 請求

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
  system="您是一個擅長創造創新混成詞的AI助手。您的任務是幫助用戶將兩個詞彙融合在一起，形成一個新的、有意義的詞彙，能夠捕捉兩個原始詞彙的精髓。如果可能的話，請提供多個選項。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "如何將「音樂」和「治療」這兩個詞彙融合，創造出一個代表使用音樂促進康復和福祉概念的新詞？"
        }
      ]
    }
  ]
)
print(message.content)

```

</Tab>
<Tab title="TypeScript">

```

import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic({
apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
model: "claude-sonnet-4-5",
max_tokens: 1000,
temperature: 1,
system: "您是一個擅長創造創新混成詞的AI助手。您的任務是幫助用戶將兩個詞彙融合在一起，形成一個新的、有意義的詞彙，能夠捕捉兩個原始詞彙的精髓。如果可能的話，請提供多個選項。",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "如何將「音樂」和「治療」這兩個詞彙融合，創造出一個代表使用音樂促進康復和福祉概念的新詞？"
}
]
}
]
});
console.log(msg);

```
</Tab>
<Tab title="AWS Bedrock Python">

```

from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# for authentication options

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=1000,
temperature=1,
system="您是一個擅長創造創新混成詞的AI助手。您的任務是幫助用戶將兩個詞彙融合在一起，形成一個新的、有意義的詞彙，能夠捕捉兩個原始詞彙的精髓。如果可能的話，請提供多個選項。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "如何將「音樂」和「治療」這兩個詞彙融合，創造出一個代表使用音樂促進康復和福祉概念的新詞？"
}
]
}
]
)
print(message.content)

```
</Tab>
<Tab title="AWS Bedrock TypeScript">

```

import AnthropicBedrock from '@anthropic-ai/bedrock-sdk';

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens: 1000,
temperature: 1,
system: "您是一個擅長創造創新混成詞的AI助手。您的任務是幫助用戶將兩個詞彙融合在一起，形成一個新的、有意義的詞彙，能夠捕捉兩個原始詞彙的精髓。如果可能的話，請提供多個選項。",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "如何將「音樂」和「治療」這兩個詞彙融合，創造出一個代表使用音樂促進康復和福祉概念的新詞？"
}
]
}
]
});
console.log(msg);

```
</Tab>
<Tab title="Vertex AI Python">

```

from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
model="claude-sonnet-4@20250514",
max_tokens=1000,
temperature=1,
system="您是一個擅長創造創新混成詞的AI助手。您的任務是幫助用戶將兩個詞彙融合在一起，形成一個新的、有意義的詞彙，能夠捕捉兩個原始詞彙的精髓。如果可能的話，請提供多個選項。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "如何將「音樂」和「治療」這兩個詞彙融合，創造出一個代表使用音樂促進康復和福祉概念的新詞？"
}
]
}
]
)
print(message.content)

```
</Tab>
<Tab title="Vertex AI TypeScript">

```

import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
model: "claude-sonnet-4@20250514",
max_tokens: 1000,
temperature: 1,
system: "您是一個擅長創造創新混成詞的AI助手。您的任務是幫助用戶將兩個詞彙融合在一起，形成一個新的、有意義的詞彙，能夠捕捉兩個原始詞彙的精髓。如果可能的話，請提供多個選項。",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "如何將「音樂」和「治療」這兩個詞彙融合，創造出一個代表使用音樂促進康復和福祉概念的新詞？"
}
]
}
]
});
console.log(msg);

```
</Tab>
</Tabs>

```