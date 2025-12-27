# 版本

在发出 API 请求时，您必须发送一个 `anthropic-version` 请求头。例如，`anthropic-version: 2023-06-01`。如果您使用我们的[客户端 SDK](/docs/zh-CN/api/client-sdks)，这将自动为您处理。

---

对于任何给定的 API 版本，我们将保留：

* 现有的输入参数
* 现有的输出参数

但是，我们可能会执行以下操作：

* 添加额外的可选输入
* 向输出添加额外的值
* 更改特定错误类型的条件
* 向类似枚举的输出值添加新变体（例如，流式事件类型）

通常，如果您按照本参考文档中记录的方式使用 API，我们不会破坏您的使用。

## 版本历史

我们始终建议尽可能使用最新的 API 版本。以前的版本被视为已弃用，可能对新用户不可用。

* `2023-06-01`  
   * [流式传输](/docs/zh-CN/api/streaming)服务器发送事件 (SSE) 的新格式：  
         * 完成是增量的。例如，`" Hello"`、`" my"`、`" name"`、`" is"`、`" Claude."` 而不是 `" Hello"`、`" Hello my"`、`" Hello my name"`、`" Hello my name is"`、`" Hello my name is Claude."`。  
         * 所有事件都是[命名事件](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#named%5Fevents)，而不是[仅数据事件](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#data-only%5Fmessages)。  
         * 删除了不必要的 `data: [DONE]` 事件。  
   * 删除了响应中的遗留 `exception` 和 `truncated` 值。
* `2023-01-01`：初始发布。