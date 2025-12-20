# 猜猜這個謎題

生成謎題並引導用戶找到解答。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試試看！

| 角色 | 內容 |
| ---- | --- |
| 用戶 | 生成一個巧妙的謎題，並提供逐步指南來幫助用戶找到正確的解答。謎題應該具有挑戰性，但可以通過邏輯思考和細心觀察來解決。在呈現每個謎題後，提供一系列提示或問題，逐步引導用戶找到答案。確保提示不會太明顯，但仍提供足夠的信息來引導用戶的思考過程。最後，揭示解答並簡要說明如何使用給定的提示來解決謎題。 |

## 範例輸出

我沒有生命，但我會成長；我沒有肺，但我需要空氣；我沒有嘴，但水會殺死我。我是什麼？

提示1：考慮物質的不同狀態以及它們如何變化。  
提示2：思考所提到的三個元素之間的關係：空氣、水和神秘物體。  
提示3：答案與日常生活中發生的常見現象有關。

解答：火。  
解釋：火沒有生命，但它會成長並在消耗燃料時蔓延。它沒有肺，但需要氧氣（空氣）來維持自己。雖然它沒有嘴，但水經常被用來撲滅火災，有效地「殺死」它們。

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
  temperature=0,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "生成一個巧妙的謎題，並提供逐步指南來幫助用戶找到正確的解答。謎題應該具有挑戰性，但可以通過邏輯思考和細心觀察來解決。在呈現每個謎題後，提供一系列提示或問題，逐步引導用戶找到答案。確保提示不會太明顯，但仍提供足夠的信息來引導用戶的思考過程。最後，揭示解答並簡要說明如何使用給定的提示來解決謎題。"
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "生成一個巧妙的謎題，並提供逐步指南來幫助用戶找到正確的解答。謎題應該具有挑戰性，但可以通過邏輯思考和細心觀察來解決。在呈現每個謎題後，提供一系列提示或問題，逐步引導用戶找到答案。確保提示不會太明顯，但仍提供足夠的信息來引導用戶的思考過程。最後，揭示解答並簡要說明如何使用給定的提示來解決謎題。"
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "生成一個巧妙的謎題，並提供逐步指南來幫助用戶找到正確的解答。謎題應該具有挑戰性，但可以通過邏輯思考和細心觀察來解決。在呈現每個謎題後，提供一系列提示或問題，逐步引導用戶找到答案。確保提示不會太明顯，但仍提供足夠的信息來引導用戶的思考過程。最後，揭示解答並簡要說明如何使用給定的提示來解決謎題。"
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "生成一個巧妙的謎題，並提供逐步指南來幫助用戶找到正確的解答。謎題應該具有挑戰性，但可以通過邏輯思考和細心觀察來解決。在呈現每個謎題後，提供一系列提示或問題，逐步引導用戶找到答案。確保提示不會太明顯，但仍提供足夠的信息來引導用戶的思考過程。最後，揭示解答並簡要說明如何使用給定的提示來解決謎題。"
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "生成一個巧妙的謎題，並提供逐步指南來幫助用戶找到正確的解答。謎題應該具有挑戰性，但可以通過邏輯思考和細心觀察來解決。在呈現每個謎題後，提供一系列提示或問題，逐步引導用戶找到答案。確保提示不會太明顯，但仍提供足夠的信息來引導用戶的思考過程。最後，揭示解答並簡要說明如何使用給定的提示來解決謎題。"
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "生成一個巧妙的謎題，並提供逐步指南來幫助用戶找到正確的解答。謎題應該具有挑戰性，但可以通過邏輯思考和細心觀察來解決。在呈現每個謎題後，提供一系列提示或問題，逐步引導用戶找到答案。確保提示不會太明顯，但仍提供足夠的信息來引導用戶的思考過程。最後，揭示解答並簡要說明如何使用給定的提示來解決謎題。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>