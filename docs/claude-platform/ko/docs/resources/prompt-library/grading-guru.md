# 채점 전문가

사용자 정의 기준과 표준에 따라 작성된 텍스트의 품질을 비교하고 평가합니다.

---

> 이 프롬프트를 우리 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

| 역할 | 내용 |
| ---- | --- |
| 사용자 | 주어진 기준에 따라 다음 두 텍스트를 평가하세요: <br/> <br/> 텍스트 1: <br/> 해가 산 뒤로 지고 있었고, 따뜻한 황금빛 광채가 계곡 전체에 드리워졌다. 새들이 저녁 노래를 지저귀고 부드러운 바람이 나뭇잎 사이로 바스락거렸다. 그것은 평화롭고 고요한 순간이었으며, 마음을 기쁨과 만족으로 가득 채우는 순간이었다. <br/><br/> 텍스트 2: <br/> 일몰이 예뻤다. 새들이 노래하고 바람이 불고 있었다. 그것이 나를 행복하게 만들었다. <br/><br/> 기준: <br/> 1. 묘사적 언어와 이미지 <br/> 2. 문장 구조와 다양성 <br/> 3. 감정적 임팩트와 참여도 <br/> 4. 문법과 구두점 |

## 예시 출력

제공된 기준에 따라 두 텍스트에 대한 평가는 다음과 같습니다:

텍스트 1:

1.  묘사적 언어와 이미지: 텍스트는 생생하고 감동적인 언어를 사용하여 설정의 그림을 그립니다. "따뜻한 황금빛 광채," "새들이 저녁 노래를 지저귀고," "부드러운 바람이 나뭇잎 사이로 바스락거렸다"와 같은 구문들이 강한 시각적, 청각적 이미지를 만들어내어 독자를 장면에 몰입시킵니다.
2.  문장 구조와 다양성: 텍스트는 단순문과 복합문을 모두 포함한 문장 구조의 좋은 조합을 보여줍니다. 이러한 다양성은 매끄러운 흐름을 유지하고 독자의 관심을 끌어줍니다.
3.  감정적 임팩트와 참여도: 텍스트는 평화, 기쁨, 만족감을 효과적으로 전달하여 독자가 그 순간의 감정적 경험을 공유하도록 초대합니다. 설정의 묘사와 긍정적인 언어의 사용이 강한 감정적 연결을 만들어냅니다.
4.  문법과 구두점: 텍스트는 문법적으로 올바르고 잘 구두점이 찍혀 있으며, 명백한 오류나 문제가 없어 보입니다.

텍스트 2:

1.  묘사적 언어와 이미지: 텍스트는 상세한 묘사적 언어와 이미지가 부족합니다. 일몰, 새, 바람을 언급하지만 독자가 장면을 시각화하는 데 도움이 되는 생생하거나 감동적인 세부사항을 제공하지 않습니다.
2.  문장 구조와 다양성: 텍스트는 구조의 변화가 거의 없는 짧고 단순한 문장들로 구성되어 있습니다. 이러한 다양성의 부족은 글쓰기를 단조롭고 덜 매력적으로 느끼게 할 수 있습니다.
3.  감정적 임팩트와 참여도: 텍스트는 "그것이 나를 행복하게 만들었다"라고 말함으로써 긍정적인 감정을 전달하지만, 묘사적 언어와 이미지의 부족으로 인해 강한 감정적 연결이나 참여도를 제공하지 못합니다.
4.  문법과 구두점: 텍스트는 문법적으로 올바르고 잘 구두점이 찍혀 있으며, 명백한 오류나 문제가 없어 보입니다.

비교 분석:  
 평가에 따르면, 텍스트 1은 묘사적 언어, 이미지, 문장 구조, 다양성, 감정적 임팩트 측면에서 텍스트 2보다 훨씬 뛰어난 성능을 보입니다. 텍스트 1은 독자에게 생생하고 몰입적인 경험을 만들어주며 주어진 기준에 따라 더 높은 품질의 글쓰기를 보여줍니다. 강한 묘사적 언어, 다양한 문장 구조, 효과적인 감정적 참여를 통해서 말입니다. 텍스트 2는 문법적으로는 올바르지만 텍스트 1에 있는 묘사적 풍부함과 감정적 임팩트가 부족합니다.

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
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "주어진 기준에 따라 다음 두 텍스트를 평가하세요: \n \n텍스트 1: \n해가 산 뒤로 지고 있었고, 따뜻한 황금빛 광채가 계곡 전체에 드리워졌다. 새들이 저녁 노래를 지저귀고 부드러운 바람이 나뭇잎 사이로 바스락거렸다. 그것은 평화롭고 고요한 순간이었으며, 마음을 기쁨과 만족으로 가득 채우는 순간이었다. \n \n텍스트 2: \n일몰이 예뻤다. 새들이 노래하고 바람이 불고 있었다. 그것이 나를 행복하게 만들었다. \n \n기준: \n1. 묘사적 언어와 이미지 \n2. 문장 구조와 다양성 \n3. 감정적 임팩트와 참여도 \n4. 문법과 구두점",
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
          "text": "주어진 기준에 따라 다음 두 텍스트를 평가하세요:  \n  \n텍스트 1:  \n해가 산 뒤로 지고 있었고, 따뜻한 황금빛 광채가 계곡 전체에 드리워졌다. 새들이 저녁 노래를 지저귀고 부드러운 바람이 나뭇잎 사이로 바스락거렸다. 그것은 평화롭고 고요한 순간이었으며, 마음을 기쁨과 만족으로 가득 채우는 순간이었다.  \n  \n텍스트 2:  \n일몰이 예뻤다. 새들이 노래하고 바람이 불고 있었다. 그것이 나를 행복하게 만들었다.  \n  \n기준:  \n1. 묘사적 언어와 이미지  \n2. 문장 구조와 다양성  \n3. 감정적 임팩트와 참여도  \n4. 문법과 구두점"
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
                    "text": "주어진 기준에 따라 다음 두 텍스트를 평가하세요:  \n  \n텍스트 1:  \n해가 산 뒤로 지고 있었고, 따뜻한 황금빛 광채가 계곡 전체에 드리워졌다. 새들이 저녁 노래를 지저귀고 부드러운 바람이 나뭇잎 사이로 바스락거렸다. 그것은 평화롭고 고요한 순간이었으며, 마음을 기쁨과 만족으로 가득 채우는 순간이었다.  \n  \n텍스트 2:  \n일몰이 예뻤다. 새들이 노래하고 바람이 불고 있었다. 그것이 나를 행복하게 만들었다.  \n  \n기준:  \n1. 묘사적 언어와 이미지  \n2. 문장 구조와 다양성  \n3. 감정적 임팩트와 참여도  \n4. 문법과 구두점"
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
          "text": "주어진 기준에 따라 다음 두 텍스트를 평가하세요:  \n  \n텍스트 1:  \n해가 산 뒤로 지고 있었고, 따뜻한 황금빛 광채가 계곡 전체에 드리워졌다. 새들이 저녁 노래를 지저귀고 부드러운 바람이 나뭇잎 사이로 바스락거렸다. 그것은 평화롭고 고요한 순간이었으며, 마음을 기쁨과 만족으로 가득 채우는 순간이었다.  \n  \n텍스트 2:  \n일몰이 예뻤다. 새들이 노래하고 바람이 불고 있었다. 그것이 나를 행복하게 만들었다.  \n  \n기준:  \n1. 묘사적 언어와 이미지  \n2. 문장 구조와 다양성  \n3. 감정적 임팩트와 참여도  \n4. 문법과 구두점"
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
                    "text": "주어진 기준에 따라 다음 두 텍스트를 평가하세요:  \n  \n텍스트 1:  \n해가 산 뒤로 지고 있었고, 따뜻한 황금빛 광채가 계곡 전체에 드리워졌다. 새들이 저녁 노래를 지저귀고 부드러운 바람이 나뭇잎 사이로 바스락거렸다. 그것은 평화롭고 고요한 순간이었으며, 마음을 기쁨과 만족으로 가득 채우는 순간이었다.  \n  \n텍스트 2:  \n일몰이 예뻤다. 새들이 노래하고 바람이 불고 있었다. 그것이 나를 행복하게 만들었다.  \n  \n기준:  \n1. 묘사적 언어와 이미지  \n2. 문장 구조와 다양성  \n3. 감정적 임팩트와 참여도  \n4. 문법과 구두점"
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
          "text": "주어진 기준에 따라 다음 두 텍스트를 평가하세요:  \n  \n텍스트 1:  \n해가 산 뒤로 지고 있었고, 따뜻한 황금빛 광채가 계곡 전체에 드리워졌다. 새들이 저녁 노래를 지저귀고 부드러운 바람이 나뭇잎 사이로 바스락거렸다. 그것은 평화롭고 고요한 순간이었으며, 마음을 기쁨과 만족으로 가득 채우는 순간이었다.  \n  \n텍스트 2:  \n일몰이 예뻤다. 새들이 노래하고 바람이 불고 있었다. 그것이 나를 행복하게 만들었다.  \n  \n기준:  \n1. 묘사적 언어와 이미지  \n2. 문장 구조와 다양성  \n3. 감정적 임팩트와 참여도  \n4. 문법과 구두점"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>