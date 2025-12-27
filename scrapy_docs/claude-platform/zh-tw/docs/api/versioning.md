# 版本

在發出 API 請求時，您必須發送 `anthropic-version` 請求標頭。例如，`anthropic-version: 2023-06-01`。如果您使用我們的[客戶端 SDK](/docs/zh-TW/api/client-sdks)，這將自動為您處理。

---

對於任何給定的 API 版本，我們將保留：

* 現有的輸入參數
* 現有的輸出參數

但是，我們可能會執行以下操作：

* 添加額外的可選輸入
* 向輸出添加額外的值
* 更改特定錯誤類型的條件
* 向類似枚舉的輸出值添加新變體（例如，串流事件類型）

一般來說，如果您按照此參考文件中記錄的方式使用 API，我們不會破壞您的使用。

## 版本歷史

我們始終建議盡可能使用最新的 API 版本。先前的版本被視為已棄用，可能對新用戶不可用。

* `2023-06-01`  
   * [串流](/docs/zh-TW/api/streaming)伺服器發送事件 (SSE) 的新格式：  
         * 完成是增量的。例如，`" Hello"`、`" my"`、`" name"`、`" is"`、`" Claude."` 而不是 `" Hello"`、`" Hello my"`、`" Hello my name"`、`" Hello my name is"`、`" Hello my name is Claude."`。  
         * 所有事件都是[命名事件](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#named%5Fevents)，而不是[僅數據事件](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#data-only%5Fmessages)。  
         * 移除了不必要的 `data: [DONE]` 事件。  
   * 移除了回應中的舊版 `exception` 和 `truncated` 值。
* `2023-01-01`：初始發布。