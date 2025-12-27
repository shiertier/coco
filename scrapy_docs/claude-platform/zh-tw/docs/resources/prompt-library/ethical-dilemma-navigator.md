# 倫理困境導航器

幫助用戶思考複雜的倫理困境並提供不同的觀點。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 通過識別核心倫理原則、探索不同的倫理框架、考慮潛在後果、承認複雜性、鼓勵個人反思並提供額外資源，幫助用戶導航複雜的倫理困境。保持客觀、非評判性的語調，並強調批判性思維、同理心和負責任的決策制定。 |
| User   | 我是一名記者，發現了涉及高級政府官員腐敗的證據。發表這個故事可能會引起公眾憤怒並要求該官員辭職，但也可能會破壞政府穩定並造成經濟不確定性。該官員向我提供了一筆可觀的賄賂來保持故事的沉默。我應該接受賄賂、發表故事，還是找到另一個解決方案？ |

## 示例輸出

這個困境涉及真實性、公共利益、個人誠信和社會責任等原則。不同的倫理框架提供了見解：

- 功利主義：發表故事可能通過揭露腐敗和促進負責任的治理來帶來更大的整體福祉，儘管會有短期的不穩定。
- 義務論：接受賄賂違反了誠實和公正的倫理義務。無論後果如何，都可能有報告真相的道德義務。
- 美德倫理學：有美德的記者會優先考慮誠信、勇氣和正義，抵制從賄賂中獲得個人利益，並維護新聞界在民主社會中的作用。
- 關懷倫理學：考慮對關係和信任的影響，包括與公眾和新聞界內部的關係。

發表的短期後果包括公眾憤怒、政治不穩定和經濟不確定性。長期影響可能涉及改善治理、增加公眾對媒體的信任以及個人/職業誠信。

接受賄賂會損害新聞倫理並助長腐敗文化。這可能導致個人內疚、失去可信度，並促成進一步的不道德行為。

考慮你的價值觀、職業倫理和新聞業在社會中的作用。尋求倫理行為準則、法律顧問和可信同事的指導。

額外資源包括面臨類似困境的記者案例研究、媒體倫理指導原則，以及關於新聞界在追究權力責任方面作用的討論。

總之，這個困境需要權衡公共利益與潛在不穩定性，考慮職業誠信，並反思你決定的長期影響。努力找到一個維護倫理原則和新聞業在民主社會中重要作用的解決方案。

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
  max_tokens=2000,
  temperature=1,
  system="通過識別核心倫理原則、探索不同的倫理框架、考慮潛在後果、承認複雜性、鼓勵個人反思並提供額外資源，幫助用戶導航複雜的倫理困境。保持客觀、非評判性的語調，並強調批判性思維、同理心和負責任的決策制定。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "我是一名記者，發現了涉及高級政府官員腐敗的證據。發表這個故事可能會引起公眾憤怒並要求該官員辭職，但也可能會破壞政府穩定並造成經濟不確定性。該官員向我提供了一筆可觀的賄賂來保持故事的沉默。我應該接受賄賂、發表故事，還是找到另一個解決方案？"
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
  max_tokens: 2000,
  temperature: 1,
  system: "通過識別核心倫理原則、探索不同的倫理框架、考慮潛在後果、承認複雜性、鼓勵個人反思並提供額外資源，幫助用戶導航複雜的倫理困境。保持客觀、非評判性的語調，並強調批判性思維、同理心和負責任的決策制定。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "我是一名記者，發現了涉及高級政府官員腐敗的證據。發表這個故事可能會引起公眾憤怒並要求該官員辭職，但也可能會破壞政府穩定並造成經濟不確定性。該官員向我提供了一筆可觀的賄賂來保持故事的沉默。我應該接受賄賂、發表故事，還是找到另一個解決方案？"
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
    max_tokens=2000,
    temperature=1,
    system="通過識別核心倫理原則、探索不同的倫理框架、考慮潛在後果、承認複雜性、鼓勵個人反思並提供額外資源，幫助用戶導航複雜的倫理困境。保持客觀、非評判性的語調，並強調批判性思維、同理心和負責任的決策制定。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "我是一名記者，發現了涉及高級政府官員腐敗的證據。發表這個故事可能會引起公眾憤怒並要求該官員辭職，但也可能會破壞政府穩定並造成經濟不確定性。該官員向我提供了一筆可觀的賄賂來保持故事的沉默。我應該接受賄賂、發表故事，還是找到另一個解決方案？"
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
  temperature: 1,
  system: "通過識別核心倫理原則、探索不同的倫理框架、考慮潛在後果、承認複雜性、鼓勵個人反思並提供額外資源，幫助用戶導航複雜的倫理困境。保持客觀、非評判性的語調，並強調批判性思維、同理心和負責任的決策制定。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "我是一名記者，發現了涉及高級政府官員腐敗的證據。發表這個故事可能會引起公眾憤怒並要求該官員辭職，但也可能會破壞政府穩定並造成經濟不確定性。該官員向我提供了一筆可觀的賄賂來保持故事的沉默。我應該接受賄賂、發表故事，還是找到另一個解決方案？"
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
    temperature=1,
    system="通過識別核心倫理原則、探索不同的倫理框架、考慮潛在後果、承認複雜性、鼓勵個人反思並提供額外資源，幫助用戶導航複雜的倫理困境。保持客觀、非評判性的語調，並強調批判性思維、同理心和負責任的決策制定。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "我是一名記者，發現了涉及高級政府官員腐敗的證據。發表這個故事可能會引起公眾憤怒並要求該官員辭職，但也可能會破壞政府穩定並造成經濟不確定性。該官員向我提供了一筆可觀的賄賂來保持故事的沉默。我應該接受賄賂、發表故事，還是找到另一個解決方案？"
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
temperature: 1,
system: "通過識別核心倫理原則、探索不同的倫理框架、考慮潛在後果、承認複雜性、鼓勵個人反思並提供額外資源，幫助用戶導航複雜的倫理困境。保持客觀、非評判性的語調，並強調批判性思維、同理心和負責任的決策制定。",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "我是一名記者，發現了涉及高級政府官員腐敗的證據。發表這個故事可能會引起公眾憤怒並要求該官員辭職，但也可能會破壞政府穩定並造成經濟不確定性。該官員向我提供了一筆可觀的賄賂來保持故事的沉默。我應該接受賄賂、發表故事，還是找到另一個解決方案？"
}
]
}
]
});
console.log(msg);
```

</Tab>
</Tabs>