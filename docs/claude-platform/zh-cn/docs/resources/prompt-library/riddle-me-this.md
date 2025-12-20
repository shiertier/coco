# 猜猜这个谜语

生成谜语并引导用户找到解决方案。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自尝试！

| 角色 | 内容 |
| ---- | --- |
| 用户 | 生成一个巧妙的谜语，并提供逐步指导来帮助用户得出正确的解决方案。谜语应该具有挑战性，但通过逻辑思维和对细节的关注是可以解决的。在呈现每个谜语后，提供一组提示或问题，逐步引导用户找到答案。确保提示不要太明显，但仍要提供足够的信息来引导用户的思维过程。最后，揭示解决方案并简要解释如何使用给定的提示来解决谜语。 |

## 示例输出

我没有生命，但我会成长；我没有肺，但我需要空气；我没有嘴，但水会杀死我。我是什么？

提示1：考虑物质的不同状态以及它们如何变化。  
提示2：思考所提到的三个元素之间的关系：空气、水和神秘物体。  
提示3：答案与日常生活中发生的常见现象有关。

解决方案：火。  
解释：火没有生命，但它会成长并在消耗燃料时蔓延。它没有肺，但需要氧气（空气）来维持自己。虽然它没有嘴，但水经常被用来扑灭火，有效地"杀死"它们。

---

## API请求

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
          "text": "生成一个巧妙的谜语，并提供逐步指导来帮助用户得出正确的解决方案。谜语应该具有挑战性，但通过逻辑思维和对细节的关注是可以解决的。在呈现每个谜语后，提供一组提示或问题，逐步引导用户找到答案。确保提示不要太明显，但仍要提供足够的信息来引导用户的思维过程。最后，揭示解决方案并简要解释如何使用给定的提示来解决谜语。"
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
          "text": "生成一个巧妙的谜语，并提供逐步指导来帮助用户得出正确的解决方案。谜语应该具有挑战性，但通过逻辑思维和对细节的关注是可以解决的。在呈现每个谜语后，提供一组提示或问题，逐步引导用户找到答案。确保提示不要太明显，但仍要提供足够的信息来引导用户的思维过程。最后，揭示解决方案并简要解释如何使用给定的提示来解决谜语。"
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
                    "text": "生成一个巧妙的谜语，并提供逐步指导来帮助用户得出正确的解决方案。谜语应该具有挑战性，但通过逻辑思维和对细节的关注是可以解决的。在呈现每个谜语后，提供一组提示或问题，逐步引导用户找到答案。确保提示不要太明显，但仍要提供足够的信息来引导用户的思维过程。最后，揭示解决方案并简要解释如何使用给定的提示来解决谜语。"
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
          "text": "生成一个巧妙的谜语，并提供逐步指导来帮助用户得出正确的解决方案。谜语应该具有挑战性，但通过逻辑思维和对细节的关注是可以解决的。在呈现每个谜语后，提供一组提示或问题，逐步引导用户找到答案。确保提示不要太明显，但仍要提供足够的信息来引导用户的思维过程。最后，揭示解决方案并简要解释如何使用给定的提示来解决谜语。"
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
                    "text": "生成一个巧妙的谜语，并提供逐步指导来帮助用户得出正确的解决方案。谜语应该具有挑战性，但通过逻辑思维和对细节的关注是可以解决的。在呈现每个谜语后，提供一组提示或问题，逐步引导用户找到答案。确保提示不要太明显，但仍要提供足够的信息来引导用户的思维过程。最后，揭示解决方案并简要解释如何使用给定的提示来解决谜语。"
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
          "text": "生成一个巧妙的谜语，并提供逐步指导来帮助用户得出正确的解决方案。谜语应该具有挑战性，但通过逻辑思维和对细节的关注是可以解决的。在呈现每个谜语后，提供一组提示或问题，逐步引导用户找到答案。确保提示不要太明显，但仍要提供足够的信息来引导用户的思维过程。最后，揭示解决方案并简要解释如何使用给定的提示来解决谜语。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>