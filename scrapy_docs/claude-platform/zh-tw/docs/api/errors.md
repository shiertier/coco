# 錯誤

API 錯誤代碼和處理指南

---

## HTTP 錯誤

我們的 API 遵循可預測的 HTTP 錯誤代碼格式：

* 400 - `invalid_request_error`：您的請求格式或內容有問題。我們也可能對下面未列出的其他 4XX 狀態代碼使用此錯誤類型。
* 401 - `authentication_error`：您的 API 金鑰有問題。
* 403 - `permission_error`：您的 API 金鑰沒有使用指定資源的權限。
* 404 - `not_found_error`：找不到請求的資源。
* 413 - `request_too_large`：請求超過允許的最大位元組數。標準 API 端點的最大請求大小為 32 MB。
* 429 - `rate_limit_error`：您的帳戶達到了速率限制。
* 500 - `api_error`：Anthropic 系統內部發生意外錯誤。
* 529 - `overloaded_error`：API 暫時過載。

  <Warning>
  當 API 在所有使用者中遇到高流量時，可能會出現 529 錯誤。
  
  在極少數情況下，如果您的組織使用量急劇增加，您可能會因為 API 上的加速限制而看到 429 錯誤。為了避免達到加速限制，請逐漸增加您的流量並保持一致的使用模式。
  </Warning>

當透過 SSE 接收[串流](/docs/zh-TW/build-with-claude/streaming)回應時，可能在返回 200 回應後發生錯誤，在這種情況下錯誤處理不會遵循這些標準機制。

## 請求大小限制

API 強制執行請求大小限制以確保最佳效能：

| 端點類型 | 最大請求大小 |
|:---|:---|
| Messages API | 32 MB |
| Token Counting API | 32 MB |
| [Batch API](/docs/zh-TW/build-with-claude/batch-processing) | 256 MB |
| [Files API](/docs/zh-TW/build-with-claude/files) | 500 MB |

如果您超過這些限制，您將收到 413 `request_too_large` 錯誤。此錯誤在請求到達我們的 API 伺服器之前由 Cloudflare 返回。

## 錯誤形狀

錯誤總是以 JSON 格式返回，具有頂層 `error` 物件，該物件總是包含 `type` 和 `message` 值。回應還包含 `request_id` 欄位以便於追蹤和除錯。例如：

```json JSON
{
  "type": "error",
  "error": {
    "type": "not_found_error",
    "message": "The requested resource could not be found."
  },
  "request_id": "req_011CSHoEeqs5C35K2UUqR7Fy"
}
```

根據我們的[版本控制](/docs/zh-TW/api/versioning)政策，我們可能會擴展這些物件內的值，並且 `type` 值可能會隨時間增長。

## 請求 ID

每個 API 回應都包含一個唯一的 `request-id` 標頭。此標頭包含諸如 `req_018EeWyXxfu5pfWkrYcMdjWG` 的值。當就特定請求聯繫支援時，請包含此 ID 以幫助我們快速解決您的問題。

我們的官方 SDK 在頂層回應物件上提供此值作為屬性，包含 `request-id` 標頭的值：

<CodeGroup>
  ```python Python
  import anthropic

  client = anthropic.Anthropic()

  message = client.messages.create(
      model="claude-sonnet-4-5",
      max_tokens=1024,
      messages=[
          {"role": "user", "content": "Hello, Claude"}
      ]
  )
  print(f"Request ID: {message._request_id}")
  ```

  ```typescript TypeScript
  import Anthropic from '@anthropic-ai/sdk';

  const client = new Anthropic();

  const message = await client.messages.create({
    model: 'claude-sonnet-4-5',
    max_tokens: 1024,
    messages: [
      {"role": "user", "content": "Hello, Claude"}
    ]
  });
  console.log('Request ID:', message._request_id);
  ```
</CodeGroup>

## 長請求

<Warning>
 我們強烈建議對長時間運行的請求使用[串流 Messages API](/docs/zh-TW/build-with-claude/streaming)或[Message Batches API](/docs/zh-TW/api/creating-message-batches)，特別是那些超過 10 分鐘的請求。
</Warning>

我們不建議在不使用我們的[串流 Messages API](/docs/zh-TW/build-with-claude/streaming)或[Message Batches API](/docs/zh-TW/api/creating-message-batches)的情況下設置大的 `max_tokens` 值：

- 某些網路可能會在可變時間段後斷開閒置連接，這可能導致請求失敗或超時而不從 Anthropic 接收回應。
- 網路的可靠性不同；我們的[Message Batches API](/docs/zh-TW/api/creating-message-batches)可以透過允許您輪詢結果而不是需要不間斷的網路連接來幫助您管理網路問題的風險。

如果您正在構建直接 API 整合，您應該知道設置 [TCP socket keep-alive](https://tldp.org/HOWTO/TCP-Keepalive-HOWTO/programming.html) 可以減少某些網路上閒置連接超時的影響。

我們的 [SDK](/docs/zh-TW/api/client-sdks) 將驗證您的非串流 Messages API 請求預期不會超過 10 分鐘超時，並且還會為 TCP keep-alive 設置 socket 選項。