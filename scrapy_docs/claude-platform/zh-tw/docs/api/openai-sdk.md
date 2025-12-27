# OpenAI SDK 相容性

Anthropic 提供相容性層，讓您能夠使用 OpenAI SDK 來測試 Claude API。只需進行少量程式碼變更，您就可以快速評估 Anthropic 模型功能。

---

<Note>
此相容性層主要用於測試和比較模型功能，對於大多數使用案例而言，不被視為長期或生產就緒的解決方案。雖然我們確實打算保持其完全功能性並不進行破壞性變更，但我們的優先事項是 [Claude API](/docs/zh-TW/api/overview) 的可靠性和有效性。

有關已知相容性限制的更多資訊，請參閱 [重要的 OpenAI 相容性限制](#important-openai-compatibility-limitations)。

如果您在使用 OpenAI SDK 相容性功能時遇到任何問題，請在 [此處](https://forms.gle/oQV4McQNiuuNbz9n8) 告訴我們。
</Note>

<Tip>
為了獲得最佳體驗並存取 Claude API 完整功能集（[PDF 處理](/docs/zh-TW/build-with-claude/pdf-support)、[引用](/docs/zh-TW/build-with-claude/citations)、[延伸思考](/docs/zh-TW/build-with-claude/extended-thinking) 和 [提示快取](/docs/zh-TW/build-with-claude/prompt-caching)），我們建議使用原生 [Claude API](/docs/zh-TW/api/overview)。
</Tip>

## 開始使用 OpenAI SDK

若要使用 OpenAI SDK 相容性功能，您需要：

1. 使用官方 OpenAI SDK  
2. 變更以下內容  
   * 更新您的基礎 URL 以指向 Claude API  
   * 將您的 API 金鑰替換為 [Claude API 金鑰](/settings/keys)  
   * 更新您的模型名稱以使用 [Claude 模型](/docs/zh-TW/about-claude/models/overview)  
3. 查看下方文件以了解支援的功能

### 快速入門範例

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

## 重要的 OpenAI 相容性限制

#### API 行為

以下是與使用 OpenAI 的最實質性差異：

* 函數呼叫的 `strict` 參數被忽略，這表示工具使用 JSON 不保證遵循提供的結構描述。為了保證結構描述一致性，請使用原生 [Claude API 與結構化輸出](/docs/zh-TW/build-with-claude/structured-outputs)。
* 不支援音訊輸入；它將被簡單地忽略並從輸入中移除  
* 不支援提示快取，但在 [Anthropic SDK](/docs/zh-TW/api/client-sdks) 中支援  
* 系統/開發者訊息被提升並連接到對話的開始，因為 Anthropic 只支援單一初始系統訊息。

大多數不支援的欄位會被無聲地忽略，而不是產生錯誤。這些都在下方記錄。

#### 輸出品質考量

如果您已對提示進行了大量調整，它可能已針對 OpenAI 進行了很好的調整。考慮使用我們 [Claude 控制台中的提示改進工具](/dashboard) 作為良好的起點。

#### 系統 / 開發者訊息提升

OpenAI SDK 的大多數輸入清楚地直接對應到 Anthropic 的 API 參數，但一個明顯的差異是系統 / 開發者提示的處理。這兩個提示可以透過 OpenAI 在聊天對話中的任何地方放置。由於 Anthropic 只支援初始系統訊息，我們會取得所有系統/開發者訊息並將它們連接在一起，中間用單一換行符 (`\n`) 分隔。然後將此完整字串作為單一系統訊息提供到訊息的開始。

#### 延伸思考支援

您可以透過新增 `thinking` 參數來啟用 [延伸思考](/docs/zh-TW/build-with-claude/extended-thinking) 功能。雖然這將改善 Claude 對複雜任務的推理，但 OpenAI SDK 不會傳回 Claude 的詳細思考過程。如需完整的延伸思考功能，包括存取 Claude 的逐步推理輸出，請使用原生 Claude API。

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

速率限制遵循 Anthropic 的 [`/v1/messages` 端點的標準限制](/docs/zh-TW/api/rate-limits)。

## 詳細的 OpenAI 相容 API 支援
### 請求欄位
#### 簡單欄位
| 欄位 | 支援狀態 |
|--------|----------------|
| `model` | 使用 Claude 模型名稱 |
| `max_tokens` | 完全支援 |
| `max_completion_tokens` | 完全支援 |
| `stream` | 完全支援 |
| `stream_options` | 完全支援 |
| `top_p` | 完全支援 |
| `parallel_tool_calls` | 完全支援 |
| `stop` | 所有非空白停止序列都有效 |
| `temperature` | 介於 0 到 1（含）之間。大於 1 的值會上限為 1。 |
| `n` | 必須恰好為 1 |
| `logprobs` | 忽略 |
| `metadata` | 忽略 |
| `response_format` | 忽略。如需 JSON 輸出，請使用原生 Claude API 的 [結構化輸出](/docs/zh-TW/build-with-claude/structured-outputs) |
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

#### `tools` / `functions` 欄位
<section title="顯示欄位">

<Tabs>
<Tab title="工具">
`tools[n].function` 欄位
| 欄位        | 支援狀態         |
|--------------|-----------------|
| `name`       | 完全支援 |
| `description`| 完全支援 |
| `parameters` | 完全支援 |
| `strict`     | 忽略。使用原生 Claude API 的 [結構化輸出](/docs/zh-TW/build-with-claude/structured-outputs) 進行嚴格結構描述驗證 |
</Tab>
<Tab title="函數">

`functions[n]` 欄位
<Info>
OpenAI 已棄用 `functions` 欄位，建議改用 `tools`。
</Info>
| 欄位        | 支援狀態         |
|--------------|-----------------|
| `name`       | 完全支援 |
| `description`| 完全支援 |
| `parameters` | 完全支援 |
| `strict`     | 忽略。使用原生 Claude API 的 [結構化輸出](/docs/zh-TW/build-with-claude/structured-outputs) 進行嚴格結構描述驗證 |
</Tab>
</Tabs>

</section>

#### `messages` 陣列欄位
<section title="顯示欄位">

<Tabs>
<Tab title="開發者角色">
`messages[n].role == "developer"` 的欄位
<Info>
開發者訊息被提升到對話開始作為初始系統訊息的一部分
</Info>
| 欄位 | 支援狀態 |
|-------|---------|
| `content` | 完全支援，但已提升 |
| `name` | 忽略 |

</Tab>
<Tab title="系統角色">
`messages[n].role == "system"` 的欄位

<Info>
系統訊息被提升到對話開始作為初始系統訊息的一部分
</Info>
| 欄位 | 支援狀態 |
|-------|---------|
| `content` | 完全支援，但已提升 |
| `name` | 忽略 |

</Tab>
<Tab title="使用者角色">
`messages[n].role == "user"` 的欄位

| 欄位 | 變體 | 子欄位 | 支援狀態 |
|-------|---------|-----------|----------------|
| `content` | `string` | | 完全支援 |
| | `array`, `type == "text"` | | 完全支援 |
| | `array`, `type == "image_url"` | `url` | 完全支援 |
| | | `detail` | 忽略 |
| | `array`, `type == "input_audio"` | | 忽略 |
| | `array`, `type == "file"` | | 忽略 |
| `name` | | | 忽略 |

</Tab>

<Tab title="助手角色">
`messages[n].role == "assistant"` 的欄位
| 欄位 | 變體 | 支援狀態 |
|-------|---------|----------------|
| `content` | `string` | 完全支援 |
| | `array`, `type == "text"` | 完全支援 |
| | `array`, `type == "refusal"` | 忽略 |
| `tool_calls` | | 完全支援 |
| `function_call` | | 完全支援 |
| `audio` | | 忽略 |
| `refusal` | | 忽略 |

</Tab>

<Tab title="工具角色">
`messages[n].role == "tool"` 的欄位
| 欄位 | 變體 | 支援狀態 |
|-------|---------|----------------|
| `content` | `string` | 完全支援 |
| | `array`, `type == "text"` | 完全支援 |
| `tool_call_id` | | 完全支援 |
| `tool_choice` | | 完全支援 |
| `name` | | 忽略 |
</Tab>

<Tab title="函數角色">
`messages[n].role == "function"` 的欄位
| 欄位 | 變體 | 支援狀態 |
|-------|---------|----------------|
| `content` | `string` | 完全支援 |
| | `array`, `type == "text"` | 完全支援 |
| `tool_choice` | | 完全支援 |
| `name` | | 忽略 |
</Tab>
</Tabs>

</section>

### 回應欄位

| 欄位 | 支援狀態 |
|---------------------------|----------------|
| `id` | 完全支援 |
| `choices[]` | 長度始終為 1 |
| `choices[].finish_reason` | 完全支援 |
| `choices[].index` | 完全支援 |
| `choices[].message.role` | 完全支援 |
| `choices[].message.content` | 完全支援 |
| `choices[].message.tool_calls` | 完全支援 |
| `object` | 完全支援 |
| `created` | 完全支援 |
| `model` | 完全支援 |
| `finish_reason` | 完全支援 |
| `content` | 完全支援 |
| `usage.completion_tokens` | 完全支援 |
| `usage.prompt_tokens` | 完全支援 |
| `usage.total_tokens` | 完全支援 |
| `usage.completion_tokens_details` | 始終為空 |
| `usage.prompt_tokens_details` | 始終為空 |
| `choices[].message.refusal` | 始終為空 |
| `choices[].message.audio` | 始終為空 |
| `logprobs` | 始終為空 |
| `service_tier` | 始終為空 |
| `system_fingerprint` | 始終為空 |

### 錯誤訊息相容性

相容性層與 OpenAI API 保持一致的錯誤格式。但是，詳細的錯誤訊息將不等同。我們建議僅使用錯誤訊息進行記錄和除錯。

### 標頭相容性

雖然 OpenAI SDK 會自動管理標頭，但以下是 Claude API 支援的完整標頭清單，供需要直接使用它們的開發人員參考。

| 標頭 | 支援狀態 |
|---------|----------------|
| `x-ratelimit-limit-requests` | 完全支援 |
| `x-ratelimit-limit-tokens` | 完全支援 |
| `x-ratelimit-remaining-requests` | 完全支援 |
| `x-ratelimit-remaining-tokens` | 完全支援 |
| `x-ratelimit-reset-requests` | 完全支援 |
| `x-ratelimit-reset-tokens` | 完全支援 |
| `retry-after` | 完全支援 |
| `request-id` | 完全支援 |
| `openai-version` | 始終為 `2020-10-01` |
| `authorization` | 完全支援 |
| `openai-processing-ms` | 始終為空 |