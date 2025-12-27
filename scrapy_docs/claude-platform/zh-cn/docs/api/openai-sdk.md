# OpenAI SDK 兼容性

Anthropic 提供了一个兼容性层，使您能够使用 OpenAI SDK 来测试 Claude API。只需进行少量代码更改，您就可以快速评估 Anthropic 模型的功能。

---

<Note>
此兼容性层主要用于测试和比较模型功能，对于大多数用例而言，不被视为长期或生产就绪的解决方案。虽然我们确实打算保持其完全功能并不做出破坏性更改，但我们的优先事项是 [Claude API](/docs/zh-CN/api/overview) 的可靠性和有效性。

有关已知兼容性限制的更多信息，请参阅 [重要的 OpenAI 兼容性限制](#important-openai-compatibility-limitations)。

如果您在使用 OpenAI SDK 兼容性功能时遇到任何问题，请在 [此处](https://forms.gle/oQV4McQNiuuNbz9n8) 告诉我们。
</Note>

<Tip>
为了获得最佳体验并访问 Claude API 的完整功能集（[PDF 处理](/docs/zh-CN/build-with-claude/pdf-support)、[引用](/docs/zh-CN/build-with-claude/citations)、[扩展思考](/docs/zh-CN/build-with-claude/extended-thinking) 和 [提示缓存](/docs/zh-CN/build-with-claude/prompt-caching)），我们建议使用原生 [Claude API](/docs/zh-CN/api/overview)。
</Tip>

## 开始使用 OpenAI SDK

要使用 OpenAI SDK 兼容性功能，您需要：

1. 使用官方 OpenAI SDK  
2. 进行以下更改  
   * 更新您的基础 URL 以指向 Claude API  
   * 将您的 API 密钥替换为 [Claude API 密钥](/settings/keys)  
   * 更新您的模型名称以使用 [Claude 模型](/docs/zh-CN/about-claude/models/overview)  
3. 查看下面的文档以了解支持哪些功能

### 快速开始示例

<CodeGroup>
    ```python Python
    from openai import OpenAI

    client = OpenAI(
        api_key="ANTHROPIC_API_KEY",  # Your Claude API key
        base_url="https://api.anthropic.com/v1/"  # the Claude API endpoint
    )

    response = client.chat.completions.create(
        model="claude-sonnet-4-5", # Anthropic model name
        messages=[
            {"role": "system", "content": "You are a helpful assistant."},
            {"role": "user", "content": "Who are you?"}
        ],
    )

    print(response.choices[0].message.content)
    ```
    
    ```typescript TypeScript
    import OpenAI from 'openai';

    const openai = new OpenAI({
        apiKey: "ANTHROPIC_API_KEY",   // Your Claude API key
        baseURL: "https://api.anthropic.com/v1/",  // Claude API endpoint
    });

    const response = await openai.chat.completions.create({
        messages: [
            { role: "user", content: "Who are you?" }
        ],
        model: "claude-sonnet-4-5", // Claude model name
    });

    console.log(response.choices[0].message.content);
    ```
</CodeGroup>

## 重要的 OpenAI 兼容性限制

#### API 行为

以下是与使用 OpenAI 相比最实质性的差异：

* 函数调用的 `strict` 参数被忽略，这意味着工具使用 JSON 不保证遵循提供的架构。为了保证架构一致性，请使用原生 [Claude API 与结构化输出](/docs/zh-CN/build-with-claude/structured-outputs)。
* 不支持音频输入；它将被简单地忽略并从输入中删除  
* 不支持提示缓存，但在 [Anthropic SDK](/docs/zh-CN/api/client-sdks) 中支持  
* 系统/开发者消息被提升并连接到对话的开头，因为 Anthropic 仅支持单个初始系统消息。

大多数不支持的字段被静默忽略而不是产生错误。这些都在下面有文档记录。

#### 输出质量考虑

如果您对提示进行了大量调整，它可能已针对 OpenAI 进行了很好的调整。考虑使用我们的 [Claude 控制台中的提示改进工具](/dashboard) 作为一个很好的起点。

#### 系统/开发者消息提升

OpenAI SDK 的大多数输入清楚地直接映射到 Anthropic 的 API 参数，但一个明显的区别是系统/开发者提示的处理。这两个提示可以通过 OpenAI 在聊天对话中放置。由于 Anthropic 仅支持初始系统消息，我们将所有系统/开发者消息连接在一起，中间用单个换行符 (`\n`) 分隔。然后将此完整字符串作为单个系统消息在消息的开头提供。

#### 扩展思考支持

您可以通过添加 `thinking` 参数来启用 [扩展思考](/docs/zh-CN/build-with-claude/extended-thinking) 功能。虽然这将改进 Claude 对复杂任务的推理，但 OpenAI SDK 不会返回 Claude 的详细思考过程。为了获得完整的扩展思考功能，包括访问 Claude 的逐步推理输出，请使用原生 Claude API。

<CodeGroup>
    ```python Python
    response = client.chat.completions.create(
        model="claude-sonnet-4-5",
        messages=...,
        extra_body={
            "thinking": { "type": "enabled", "budget_tokens": 2000 }
        }
    )
    ```
    
    ```typescript TypeScript
    const response = await openai.chat.completions.create({
        messages: [
            { role: "user", content: "Who are you?" }
        ],
        model: "claude-sonnet-4-5",
        // @ts-expect-error
        thinking: { type: "enabled", budget_tokens: 2000 }
    });

    ```
</CodeGroup>

## 速率限制

速率限制遵循 Anthropic 的 [`/v1/messages` 端点的 [标准限制](/docs/zh-CN/api/rate-limits)。

## 详细的 OpenAI 兼容 API 支持
### 请求字段
#### 简单字段
| 字段 | 支持状态 |
|--------|----------------|
| `model` | 使用 Claude 模型名称 |
| `max_tokens` | 完全支持 |
| `max_completion_tokens` | 完全支持 |
| `stream` | 完全支持 |
| `stream_options` | 完全支持 |
| `top_p` | 完全支持 |
| `parallel_tool_calls` | 完全支持 |
| `stop` | 所有非空白停止序列都有效 |
| `temperature` | 在 0 到 1 之间（含）。大于 1 的值被限制在 1。 |
| `n` | 必须恰好为 1 |
| `logprobs` | 忽略 |
| `metadata` | 忽略 |
| `response_format` | 忽略。对于 JSON 输出，使用原生 Claude API 的 [结构化输出](/docs/zh-CN/build-with-claude/structured-outputs) |
| `prediction` | 忽略 |
| `presence_penalty` | 忽略 |
| `frequency_penalty` | 忽略 |
| `seed` | 忽略 |
| `service_tier` | 忽略 |
| `audio` | 忽略 |
| `logit_bias` | 忽略 |
| `store` | 忽略 |
| `user` | 忽略 |
| `modalities` | 忽略 |
| `top_logprobs` | 忽略 |
| `reasoning_effort` | 忽略 |

#### `tools` / `functions` 字段
<section title="显示字段">

<Tabs>
<Tab title="Tools">
`tools[n].function` 字段
| 字段        | 支持状态         |
|--------------|-----------------|
| `name`       | 完全支持 |
| `description`| 完全支持 |
| `parameters` | 完全支持 |
| `strict`     | 忽略。使用原生 Claude API 的 [结构化输出](/docs/zh-CN/build-with-claude/structured-outputs) 进行严格的架构验证 |
</Tab>
<Tab title="Functions">

`functions[n]` 字段
<Info>
OpenAI 已弃用 `functions` 字段，建议改用 `tools`。
</Info>
| 字段        | 支持状态         |
|--------------|-----------------|
| `name`       | 完全支持 |
| `description`| 完全支持 |
| `parameters` | 完全支持 |
| `strict`     | 忽略。使用原生 Claude API 的 [结构化输出](/docs/zh-CN/build-with-claude/structured-outputs) 进行严格的架构验证 |
</Tab>
</Tabs>

</section>

#### `messages` 数组字段
<section title="显示字段">

<Tabs>
<Tab title="Developer role">
`messages[n].role == "developer"` 的字段
<Info>
开发者消息被提升到对话的开头作为初始系统消息的一部分
</Info>
| 字段 | 支持状态 |
|-------|---------|
| `content` | 完全支持，但已提升 |
| `name` | 忽略 |

</Tab>
<Tab title="System role">
`messages[n].role == "system"` 的字段

<Info>
系统消息被提升到对话的开头作为初始系统消息的一部分
</Info>
| 字段 | 支持状态 |
|-------|---------|
| `content` | 完全支持，但已提升 |
| `name` | 忽略 |

</Tab>
<Tab title="User role">
`messages[n].role == "user"` 的字段

| 字段 | 变体 | 子字段 | 支持状态 |
|-------|---------|-----------|----------------|
| `content` | `string` | | 完全支持 |
| | `array`, `type == "text"` | | 完全支持 |
| | `array`, `type == "image_url"` | `url` | 完全支持 |
| | | `detail` | 忽略 |
| | `array`, `type == "input_audio"` | | 忽略 |
| | `array`, `type == "file"` | | 忽略 |
| `name` | | | 忽略 |

</Tab>

<Tab title="Assistant role">
`messages[n].role == "assistant"` 的字段
| 字段 | 变体 | 支持状态 |
|-------|---------|----------------|
| `content` | `string` | 完全支持 |
| | `array`, `type == "text"` | 完全支持 |
| | `array`, `type == "refusal"` | 忽略 |
| `tool_calls` | | 完全支持 |
| `function_call` | | 完全支持 |
| `audio` | | 忽略 |
| `refusal` | | 忽略 |

</Tab>

<Tab title="Tool role">
`messages[n].role == "tool"` 的字段
| 字段 | 变体 | 支持状态 |
|-------|---------|----------------|
| `content` | `string` | 完全支持 |
| | `array`, `type == "text"` | 完全支持 |
| `tool_call_id` | | 完全支持 |
| `tool_choice` | | 完全支持 |
| `name` | | 忽略 |
</Tab>

<Tab title="Function role">
`messages[n].role == "function"` 的字段
| 字段 | 变体 | 支持状态 |
|-------|---------|----------------|
| `content` | `string` | 完全支持 |
| | `array`, `type == "text"` | 完全支持 |
| `tool_choice` | | 完全支持 |
| `name` | | 忽略 |
</Tab>
</Tabs>

</section>

### 响应字段

| 字段 | 支持状态 |
|---------------------------|----------------|
| `id` | 完全支持 |
| `choices[]` | 长度始终为 1 |
| `choices[].finish_reason` | 完全支持 |
| `choices[].index` | 完全支持 |
| `choices[].message.role` | 完全支持 |
| `choices[].message.content` | 完全支持 |
| `choices[].message.tool_calls` | 完全支持 |
| `object` | 完全支持 |
| `created` | 完全支持 |
| `model` | 完全支持 |
| `finish_reason` | 完全支持 |
| `content` | 完全支持 |
| `usage.completion_tokens` | 完全支持 |
| `usage.prompt_tokens` | 完全支持 |
| `usage.total_tokens` | 完全支持 |
| `usage.completion_tokens_details` | 始终为空 |
| `usage.prompt_tokens_details` | 始终为空 |
| `choices[].message.refusal` | 始终为空 |
| `choices[].message.audio` | 始终为空 |
| `logprobs` | 始终为空 |
| `service_tier` | 始终为空 |
| `system_fingerprint` | 始终为空 |

### 错误消息兼容性

兼容性层与 OpenAI API 保持一致的错误格式。但是，详细的错误消息将不等同。我们建议仅将错误消息用于日志记录和调试。

### 标头兼容性

虽然 OpenAI SDK 自动管理标头，但以下是 Claude API 支持的完整标头列表，供需要直接使用它们的开发人员参考。

| 标头 | 支持状态 |
|---------|----------------|
| `x-ratelimit-limit-requests` | 完全支持 |
| `x-ratelimit-limit-tokens` | 完全支持 |
| `x-ratelimit-remaining-requests` | 完全支持 |
| `x-ratelimit-remaining-tokens` | 完全支持 |
| `x-ratelimit-reset-requests` | 完全支持 |
| `x-ratelimit-reset-tokens` | 完全支持 |
| `retry-after` | 完全支持 |
| `request-id` | 完全支持 |
| `openai-version` | 始终为 `2020-10-01` |
| `authorization` | 完全支持 |
| `openai-processing-ms` | 始终为空 |