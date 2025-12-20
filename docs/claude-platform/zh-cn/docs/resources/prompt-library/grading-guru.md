# 评分大师

基于用户定义的标准和要求比较和评估书面文本的质量。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

| 角色 | 内容 |
| ---- | --- |
| 用户 | 根据给定标准评估以下两个文本：<br/> <br/> 文本1：<br/> 太阳正在山后落下，在山谷中投下温暖的金色光芒。鸟儿唱着它们的晚歌，轻柔的微风在叶子间沙沙作响。这是一个宁静祥和的时刻，让心中充满了喜悦和满足。<br/><br/> 文本2：<br/> 日落很美。鸟儿在歌唱，风在吹。这让我很开心。<br/><br/> 标准：<br/> 1. 描述性语言和意象 <br/> 2. 句子结构和多样性 <br/> 3. 情感冲击力和参与度 <br/> 4. 语法和标点符号 |

## 示例输出

基于提供的标准，以下是我对两个文本的评估：

文本1：

1. 描述性语言和意象：文本使用生动而富有感染力的语言来描绘场景。像"温暖的金色光芒"、"鸟儿唱着它们的晚歌"和"轻柔的微风在叶子间沙沙作响"这样的短语创造了强烈的视觉和听觉意象，让读者沉浸在场景中。
2. 句子结构和多样性：文本展示了良好的句子结构组合，既有简单句也有复合句。这种多样性有助于保持流畅的节奏并保持读者的参与度。
3. 情感冲击力和参与度：文本有效地传达了宁静、喜悦和满足的感觉，邀请读者分享这一时刻的情感体验。对场景的描述和积极语言的使用创造了强烈的情感联系。
4. 语法和标点符号：文本在语法上似乎是正确的，标点符号使用得当，没有明显的错误或问题。

文本2：

1. 描述性语言和意象：文本缺乏详细的描述性语言和意象。虽然它提到了日落、鸟儿和风，但没有提供任何生动或富有感染力的细节来帮助读者想象场景。
2. 句子结构和多样性：文本由简短的简单句组成，结构变化很少。这种缺乏多样性会使写作感觉单调，参与度较低。
3. 情感冲击力和参与度：文本通过说"这让我很开心"传达了积极的情感，但由于缺乏描述性语言和意象，它没有提供强烈的情感联系或参与度。
4. 语法和标点符号：文本在语法上似乎是正确的，标点符号使用得当，没有明显的错误或问题。

比较分析：
基于评估，文本1在描述性语言、意象、句子结构、多样性和情感冲击力方面明显优于文本2。文本1为读者创造了生动而沉浸式的体验，并展示了基于给定标准的更高质量的写作，具有强烈的描述性语言、多样的句子结构和有效的情感参与。文本2虽然语法正确，但缺乏文本1中存在的描述性丰富度和情感冲击力。

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