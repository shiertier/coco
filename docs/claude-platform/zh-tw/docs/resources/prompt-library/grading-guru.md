# 評分專家

根據使用者定義的標準和準則比較和評估書面文本的品質。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

| 角色 | 內容 |
| ---- | --- |
| 使用者 | 根據給定標準評估以下兩個文本： <br/> <br/> 文本 1： <br/> 太陽正在山後落下，在山谷中投下溫暖的金色光芒。鳥兒唱著它們的晚歌，溫和的微風在樹葉間沙沙作響。這是一個平靜祥和的時刻，讓心中充滿了喜悅和滿足。 <br/><br/> 文本 2： <br/> 日落很美。鳥兒在歌唱，風在吹。這讓我很開心。 <br/><br/> 標準： <br/> 1. 描述性語言和意象 <br/> 2. 句子結構和變化 <br/> 3. 情感影響和參與度 <br/> 4. 語法和標點符號 |

## 範例輸出

根據提供的標準，以下是我對兩個文本的評估：

文本 1：

1.  描述性語言和意象：文本使用生動且富有感染力的語言來描繪場景。像「溫暖的金色光芒」、「鳥兒唱著它們的晚歌」和「溫和的微風在樹葉間沙沙作響」這樣的短語創造了強烈的視覺和聽覺意象，讓讀者沉浸在場景中。
2.  句子結構和變化：文本展示了良好的句子結構組合，包含簡單句和複合句。這種變化有助於保持流暢的節奏並保持讀者的參與度。
3.  情感影響和參與度：文本有效地傳達了平靜、喜悅和滿足的感覺，邀請讀者分享這一時刻的情感體驗。對場景的描述和積極語言的使用創造了強烈的情感連結。
4.  語法和標點符號：文本在語法上似乎是正確的，標點符號使用得當，沒有明顯的錯誤或問題。

文本 2：

1.  描述性語言和意象：文本缺乏詳細的描述性語言和意象。雖然它提到了日落、鳥兒和風，但沒有提供任何生動或富有感染力的細節來幫助讀者視覺化場景。
2.  句子結構和變化：文本由短而簡單的句子組成，結構變化很少。這種缺乏變化可能使寫作感覺單調且較不吸引人。
3.  情感影響和參與度：文本通過說「這讓我很開心」傳達了積極的情感，但由於缺乏描述性語言和意象，它沒有提供強烈的情感連結或參與度。
4.  語法和標點符號：文本在語法上似乎是正確的，標點符號使用得當，沒有明顯的錯誤或問題。

比較分析：  
 根據評估，文本 1 在描述性語言、意象、句子結構、變化和情感影響方面明顯優於文本 2。文本 1 為讀者創造了生動且身臨其境的體驗，並根據給定標準展示了更高品質的寫作，具有強烈的描述性語言、多樣的句子結構和有效的情感參與。文本 2 雖然語法正確，但缺乏文本 1 中存在的描述豐富性和情感影響。

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
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Evaluate the following two texts based on the given criteria: \n \nText 1: \nThe sun was setting behind the mountains, casting a warm golden glow across the valley. The birds chirped their evening songs as the gentle breeze rustled through the leaves. It was a peaceful and serene moment, one that filled the heart with joy and contentment. \n \nText 2: \nThe sunset was pretty. The birds were singing and the wind was blowing. It made me happy. \n \nCriteria: \n1. Descriptive language and imagery \n2. Sentence structure and variety \n3. Emotional impact and engagement \n4. Grammar and punctuation",
                }
            ],
        }
    ],
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
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Evaluate the following two texts based on the given criteria:  \n  \nText 1:  \nThe sun was setting behind the mountains, casting a warm golden glow across the valley. The birds chirped their evening songs as the gentle breeze rustled through the leaves. It was a peaceful and serene moment, one that filled the heart with joy and contentment.  \n  \nText 2:  \nThe sunset was pretty. The birds were singing and the wind was blowing. It made me happy.  \n  \nCriteria:  \n1. Descriptive language and imagery  \n2. Sentence structure and variety  \n3. Emotional impact and engagement  \n4. Grammar and punctuation"
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
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Evaluate the following two texts based on the given criteria:  \n  \nText 1:  \nThe sun was setting behind the mountains, casting a warm golden glow across the valley. The birds chirped their evening songs as the gentle breeze rustled through the leaves. It was a peaceful and serene moment, one that filled the heart with joy and contentment.  \n  \nText 2:  \nThe sunset was pretty. The birds were singing and the wind was blowing. It made me happy.  \n  \nCriteria:  \n1. Descriptive language and imagery  \n2. Sentence structure and variety  \n3. Emotional impact and engagement  \n4. Grammar and punctuation"
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
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Evaluate the following two texts based on the given criteria:  \n  \nText 1:  \nThe sun was setting behind the mountains, casting a warm golden glow across the valley. The birds chirped their evening songs as the gentle breeze rustled through the leaves. It was a peaceful and serene moment, one that filled the heart with joy and contentment.  \n  \nText 2:  \nThe sunset was pretty. The birds were singing and the wind was blowing. It made me happy.  \n  \nCriteria:  \n1. Descriptive language and imagery  \n2. Sentence structure and variety  \n3. Emotional impact and engagement  \n4. Grammar and punctuation"
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
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Evaluate the following two texts based on the given criteria:  \n  \nText 1:  \nThe sun was setting behind the mountains, casting a warm golden glow across the valley. The birds chirped their evening songs as the gentle breeze rustled through the leaves. It was a peaceful and serene moment, one that filled the heart with joy and contentment.  \n  \nText 2:  \nThe sunset was pretty. The birds were singing and the wind was blowing. It made me happy.  \n  \nCriteria:  \n1. Descriptive language and imagery  \n2. Sentence structure and variety  \n3. Emotional impact and engagement  \n4. Grammar and punctuation"
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
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Evaluate the following two texts based on the given criteria:  \n  \nText 1:  \nThe sun was setting behind the mountains, casting a warm golden glow across the valley. The birds chirped their evening songs as the gentle breeze rustled through the leaves. It was a peaceful and serene moment, one that filled the heart with joy and contentment.  \n  \nText 2:  \nThe sunset was pretty. The birds were singing and the wind was blowing. It made me happy.  \n  \nCriteria:  \n1. Descriptive language and imagery  \n2. Sentence structure and variety  \n3. Emotional impact and engagement  \n4. Grammar and punctuation"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>