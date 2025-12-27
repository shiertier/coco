# 工單路由

本指南介紹如何利用 Claude 的先進自然語言理解能力，根據客戶意圖、緊急程度、優先級、客戶檔案等因素，大規模分類客戶支援工單。

---

## 確定是否應使用 Claude 進行工單路由

以下是一些關鍵指標，表明您應該使用 Claude 等 LLM，而不是傳統 ML 方法進行分類任務：

    <section title="您的標記訓練數據有限">

        傳統 ML 流程需要大量標記的數據集。Claude 的預訓練模型只需幾十個標記示例就能有效分類工單，大大減少了數據準備時間和成本。
    
</section>
    <section title="您的分類類別可能會隨著時間推移而改變或演變">

        一旦建立了傳統 ML 方法，改變它就是一項繁瑣且數據密集的工作。另一方面，隨著您的產品或客戶需求的演變，Claude 可以輕鬆適應類別定義的變化或新類別，無需大量重新標記訓練數據。
    
</section>
    <section title="您需要處理複雜的非結構化文本輸入">

        傳統 ML 模型通常難以處理非結構化數據，需要大量特徵工程。Claude 的先進語言理解能力允許基於內容和上下文進行準確分類，而不是依賴嚴格的本體結構。
    
</section>
    <section title="您的分類規則基於語義理解">

        傳統 ML 方法通常依賴於詞袋模型或簡單的模式匹配。Claude 擅長於理解和應用基礎規則，當類別由條件而非示例定義時。
    
</section>
    <section title="您需要為分類決策提供可解釋的推理">

        許多傳統 ML 模型對其決策過程提供的洞察很少。Claude 可以為其分類決策提供人類可讀的解釋，建立對自動化系統的信任，並在需要時便於輕鬆調整。
    
</section>
    <section title="您想更有效地處理邊界情況和模糊工單">

        傳統 ML 系統通常難以處理異常值和模糊輸入，經常誤分類或默認為通用類別。Claude 的自然語言處理能力使其能夠更好地解釋支援工單中的上下文和細微差別，可能減少需要人工干預的誤路由或未分類工單的數量。
    
</section>
    <section title="您需要多語言支援而無需維護單獨的模型">

        傳統 ML 方法通常需要為每種支援的語言使用單獨的模型或廣泛的翻譯流程。Claude 的多語言能力使其能夠以各種語言分類工單，無需單獨的模型或廣泛的翻譯流程，簡化了對全球客戶群的支援。
    
</section>

***

## 構建和部署您的 LLM 支援工作流

### 了解您當前的支援方法
在深入自動化之前，了解您現有的工單系統至關重要。首先調查您的支援團隊目前如何處理工單路由。

考慮以下問題：
* 使用什麼標準來確定應用哪個 SLA/服務方案？
* 工單路由是否用於確定工單發送到哪個支援層級或產品專家？
* 是否已有任何自動化規則或工作流程？它們在什麼情況下失敗？
* 邊界情況或模糊工單如何處理？
* 團隊如何優先處理工單？

您對人類如何處理某些情況的了解越多，您就越能夠與 Claude 合作完成任務。

### 定義用戶意圖類別
明確定義的用戶意圖類別列表對於使用 Claude 進行準確的支援工單分類至關重要。Claude 在您系統內有效路由工單的能力與您系統類別的定義程度成正比。

以下是一些用戶意圖類別和子類別的示例。

    <section title="技術問題">

        * 硬體問題
        * 軟體錯誤
        * 相容性問題
        * 效能問題
    
</section>
    <section title="帳戶管理">

        * 密碼重設
        * 帳戶訪問問題
        * 計費查詢
        * 訂閱變更
    
</section>
    <section title="產品信息">

        * 功能查詢
        * 產品相容性問題
        * 定價信息
        * 可用性查詢
    
</section>
    <section title="用戶指導">

        * 操作方法問題
        * 功能使用協助
        * 最佳實踐建議
        * 故障排除指導
    
</section>
    <section title="反饋">

        * 錯誤報告
        * 功能請求
        * 一般反饋或建議
        * 投訴
    
</section>
    <section title="訂單相關">

        * 訂單狀態查詢
        * 運輸信息
        * 退貨和換貨
        * 訂單修改
    
</section>
    <section title="服務請求">

        * 安裝協助
        * 升級請求
        * 維護安排
        * 服務取消
    
</section>
    <section title="安全問題">

        * 數據隱私查詢
        * 可疑活動報告
        * 安全功能協助
    
</section>
    <section title="合規和法律">

        * 監管合規問題
        * 服務條款查詢
        * 法律文件請求
    
</section>
    <section title="緊急支援">

        * 關鍵系統故障
        * 緊急安全問題
        * 時間敏感問題
    
</section>
    <section title="培訓和教育">

        * 產品培訓請求
        * 文檔查詢
        * 網絡研討會或工作坊信息
    
</section>
    <section title="集成和 API">

        * 集成協助
        * API 使用問題
        * 第三方相容性查詢
    
</section>

除了意圖外，工單路由和優先級可能還受到其他因素的影響，例如緊急程度、客戶類型、SLA 或語言。在構建自動化路由系統時，請務必考慮其他路由標準。

### 建立成功標準

與您的支援團隊合作[定義明確的成功標準](/docs/zh-TW/test-and-evaluate/define-success)，包括可測量的基準、閾值和目標。

以下是使用 LLM 進行支援工單路由時的一些標準標準和基準：

    <section title="分類一致性">

        此指標評估 Claude 在一段時間內對相似工單的分類一致性。這對於維持路由可靠性至關重要。通過定期使用一組標準化輸入測試模型並目標達到 95% 或更高的一致性率來測量。
    
</section>
    <section title="適應速度">

        這測量 Claude 適應新類別或變化的工單模式的速度。通過引入新的工單類型並測量模型在這些新類別上達到令人滿意的準確度（例如 >90%）所需的時間來測試。目標是在 50-100 個樣本工單內適應。
    
</section>
    <section title="多語言處理">

        這評估 Claude 準確路由多種語言工單的能力。測量不同語言的路由準確度，目標是非主要語言的準確度下降不超過 5-10%。
    
</section>
    <section title="邊界情況處理">

        這評估 Claude 在不尋常或複雜工單上的效能。創建一組邊界情況測試集並測量路由準確度，目標是在這些具有挑戰性的輸入上至少達到 80% 的準確度。
    
</section>
    <section title="偏差緩解">

        這測量 Claude 在不同客戶人口統計中的路由公平性。定期審計路由決策以尋找潛在偏差，目標是在所有客戶群體中保持一致的路由準確度（在 2-3% 內）。
    
</section>
    <section title="提示效率">

        在最小化令牌計數至關重要的情況下，此標準評估 Claude 在最少上下文下的效能。測量提供不同數量上下文時的路由準確度，目標是僅使用工單標題和簡短描述時達到 90% 以上的準確度。
    
</section>
    <section title="可解釋性評分">

        這評估 Claude 對其路由決策解釋的質量和相關性。人類評分者可以按比例（例如 1-5）對解釋進行評分，目標是達到平均 4 分或更高。
    
</section>

以下是一些常見的成功標準，無論是否使用 LLM 都可能有用：

    <section title="路由準確度">

        路由準確度測量工單被正確分配給適當團隊或個人的頻率。這通常以正確路由的工單數量佔總工單數量的百分比來測量。行業基準通常目標為 90-95% 的準確度，但這可能因支援結構的複雜性而異。
    
</section>
    <section title="分配時間">

        此指標追蹤工單提交後被分配的速度。更快的分配時間通常導致更快的解決和改進的客戶滿意度。一流的系統通常實現平均分配時間少於 5 分鐘，許多系統目標為近乎即時的路由（這在 LLM 實現中是可能的）。
    
</section>
    <section title="重新路由率">

        重新路由率表示工單在初始路由後需要重新分配的頻率。較低的率表示更準確的初始路由。目標是重新路由率低於 10%，頂級系統達到 5% 或更低的率。
    
</section>
    <section title="首次聯繫解決率">

        這測量在與客戶的首次互動中解決的工單百分比。較高的率表示有效的路由和準備充分的支援團隊。行業基準通常在 70-75% 範圍內，頂級表現者達到 80% 或更高的率。
    
</section>
    <section title="平均處理時間">

        平均處理時間測量從開始到完成解決工單所需的時間。有效的路由可以顯著減少此時間。基準因行業和複雜性而異，但許多組織目標是將非關鍵問題的平均處理時間保持在 24 小時以下。
    
</section>
    <section title="客戶滿意度評分">

        通常通過互動後調查測量，這些評分反映客戶對支援流程的整體滿意度。有效的路由有助於提高滿意度。目標是 CSAT 評分達到 90% 或更高，頂級表現者通常達到 95% 以上的滿意度率。
    
</section>
    <section title="升級率">

        這測量工單需要升級到更高支援層級的頻率。較低的升級率通常表示更準確的初始路由。努力達到升級率低於 20%，一流系統達到 10% 或更低的率。
    
</section>
    <section title="代理生產力">

        此指標查看代理在實現路由解決方案後能夠有效處理的工單數量。改進的路由應增加生產力。通過追蹤每個代理每天或每小時解決的工單數量來測量，目標是在實現新路由系統後實現 10-20% 的改進。
    
</section>
    <section title="自助服務偏轉率">

        這測量通過自助服務選項在進入路由系統之前解決的潛在工單百分比。較高的率表示有效的預路由分類。目標是偏轉率達到 20-30%，頂級表現者達到 40% 或更高的率。
    
</section>
    <section title="每張工單成本">

        此指標計算解決每張支援工單的平均成本。有效的路由應有助於隨著時間推移降低此成本。雖然基準差異很大，但許多組織目標是在實現改進的路由系統後將每張工單成本降低 10-15%。
    
</section>

### 選擇正確的 Claude 模型

模型的選擇取決於成本、準確度和響應時間之間的權衡。

許多客戶發現 `claude-haiku-4-5-20251001` 是工單路由的理想模型，因為它是 Claude 4 系列中最快且最具成本效益的模型，同時仍能提供優異的結果。如果您的分類問題需要深入的主題專業知識或大量的意圖類別複雜推理，您可以選擇[更大的 Sonnet 模型](/docs/zh-TW/about-claude/models)。

### 構建強大的提示

工單路由是一種分類任務。Claude 分析支援工單的內容，並根據問題類型、緊急程度、所需專業知識或其他相關因素將其分類為預定義的類別。

讓我們編寫一個工單分類提示。我們的初始提示應包含用戶請求的內容，並返回推理和意圖。

<Tip>
在 [Claude 控制台](/login)上嘗試[提示生成器](/docs/zh-TW/prompt-generator)，讓 Claude 為您編寫初稿。
</Tip>

以下是一個工單路由分類提示的示例：
```python
def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. Your task is to analyze customer support requests and output the appropriate classification intent for each request, along with your reasoning. 

        Here is the customer support request you need to classify:

        <request>{ticket_contents}</request>

        Please carefully analyze the above request to determine the customer's core intent and needs. Consider what the customer is asking for has concerns about.

        First, write out your reasoning and analysis of how to classify this request inside <reasoning> tags.

        Then, output the appropriate classification label for the request inside a <intent> tag. The valid intents are:
        <intents>
        <intent>Support, Feedback, Complaint</intent>
        <intent>Order Tracking</intent>
        <intent>Refund/Exchange</intent>
        </intents>

        A request may have ONLY ONE applicable intent. Only include the intent that is most applicable to the request.

        As an example, consider the following request:
        <request>Hello! I had high-speed fiber internet installed on Saturday and my installer, Kevin, was absolutely fantastic! Where can I send my positive review? Thanks for your help!</request>

        Here is an example of how your output should be formatted (for the above example request):
        <reasoning>The user seeks information in order to leave positive feedback.</reasoning>
        <intent>Support, Feedback, Complaint</intent>

        Here are a few more examples:
        <examples>
        <example 2>
        Example 2 Input:
        <request>I wanted to write and personally thank you for the compassion you showed towards my family during my father's funeral this past weekend. Your staff was so considerate and helpful throughout this whole process; it really took a load off our shoulders. The visitation brochures were beautiful. We'll never forget the kindness you showed us and we are so appreciative of how smoothly the proceedings went. Thank you, again, Amarantha Hill on behalf of the Hill Family.</request>

        Example 2 Output:
        <reasoning>User leaves a positive review of their experience.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 2>
        <example 3>

        ...

        </example 8>
        <example 9>
        Example 9 Input:
        <request>Your website keeps sending ad-popups that block the entire screen. It took me twenty minutes just to finally find the phone number to call and complain. How can I possibly access my account information with all of these popups? Can you access my account for me, since your website is broken? I need to know what the address is on file.</request>

        Example 9 Output:
        <reasoning>The user requests help accessing their web account information.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 9>

        Remember to always include your classification reasoning before your actual intent output. The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
```

讓我們分解此提示的關鍵組件：
* 我們使用 Python f 字符串創建提示模板，允許 `ticket_contents` 插入到 `<request>` 標籤中。
* 我們給 Claude 一個明確定義的角色，作為一個分類系統，仔細分析工單內容以確定客戶的核心意圖和需求。
* 我們指示 Claude 適當的輸出格式，在這種情況下，在 `<reasoning>` 標籤內提供其推理和分析，然後在 `<intent>` 標籤內提供適當的分類標籤。
* 我們指定有效的意圖類別："Support, Feedback, Complaint"、"Order Tracking" 和 "Refund/Exchange"。
* 我們包含幾個示例（也稱為少量提示）來說明輸出應如何格式化，這提高了準確度和一致性。

我們希望 Claude 將其響應分成各種 XML 標籤部分的原因是，我們可以使用正則表達式從輸出中分別提取推理和意圖。這允許我們在工單路由工作流中創建有針對性的後續步驟，例如僅使用意圖來決定將工單路由給誰。

### 部署您的提示

在不在測試生產環境中部署提示並[運行評估](/docs/zh-TW/test-and-evaluate/develop-tests)的情況下，很難知道您的提示效果如何。

讓我們構建部署結構。首先定義方法簽名以包裝我們對 Claude 的調用。我們將採用已經開始編寫的方法，該方法以 `ticket_contents` 作為輸入，現在返回 `reasoning` 和 `intent` 的元組作為輸出。如果您有使用傳統 ML 的現有自動化，您應該改為遵循該方法簽名。

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ... The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
    # Send the prompt to the API to classify the support request.
    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
        stream=False,
    )
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

    return reasoning, intent
```

此代碼：
* 導入 Anthropic 庫並使用您的 API 密鑰創建客戶端實例。
* 定義一個 `classify_support_request` 函數，該函數接受 `ticket_contents` 字符串。
* 使用 `classification_prompt` 將 `ticket_contents` 發送給 Claude 進行分類
* 從響應中返回提取的模型的 `reasoning` 和 `intent`。

由於我們需要等待整個推理和意圖文本生成後才能解析，我們設置 `stream=False`（默認值）。

***

## 評估您的提示

提示通常需要測試和優化才能投入生產。要確定您的解決方案的準備情況，請根據您之前建立的成功標準和閾值評估效能。

要運行您的評估，您需要測試用例來運行它。本指南的其餘部分假設您已經[開發了測試用例](/docs/zh-TW/test-and-evaluate/develop-tests)。

### 構建評估函數

本指南的示例評估沿著三個關鍵指標測量 Claude 的效能：
* 準確度
* 每次分類的成本

根據對您重要的因素，您可能需要在其他軸上評估 Claude。

為了評估這一點，我們首先必須修改我們編寫的腳本並添加一個函數來比較預測的意圖與實際意圖，並計算正確預測的百分比。我們還必須添加成本計算和時間測量功能。

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(request, actual_intent):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ...The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """

    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
    )
    usage = message.usage  # Get the usage statistics for the API call for how many input and output tokens were used.
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

      # Check if the model's prediction is correct.
    correct = actual_intent.strip() == intent.strip()

    # Return the reasoning, intent, correct, and usage.
    return reasoning, intent, correct, usage
```

讓我們分解我們所做的編輯：
* 我們將測試用例中的 `actual_intent` 添加到 `classify_support_request` 方法中，並設置比較以評估 Claude 的意圖分類是否與我們的黃金意圖分類相匹配。
* 我們提取了 API 調用的使用統計信息，以根據使用的輸入和輸出令牌計算成本

### 運行您的評估

適當的評估需要明確的閾值和基準來確定什麼是好的結果。上面的腳本將為我們提供準確度、響應時間和每次分類成本的運行時值，但我們仍然需要明確建立的閾值。例如：
* **準確度：** 95%（100 次測試中）
* **每次分類的成本：** 從當前路由方法平均減少 50%（100 次測試中）

有了這些閾值，您可以快速輕鬆地大規模判斷，並以公正的經驗主義判斷，什麼方法最適合您，以及可能需要做什麼改變以更好地適應您的要求。

***

## 改進效能

在複雜的場景中，除了標準[提示工程技術](/docs/zh-TW/build-with-claude/prompt-engineering/overview)和[護欄實現策略](/docs/zh-TW/test-and-evaluate/strengthen-guardrails/reduce-hallucinations)之外，考慮其他策略來改進效能可能會有幫助。以下是一些常見場景：

### 對於 20+ 個意圖類別的情況，使用分類法層次結構

隨著類別數量的增加，所需示例的數量也會增加，可能使提示變得笨重。作為替代方案，您可以考慮使用分類器混合實現分層分類系統。
1. 在分類法樹結構中組織您的意圖。
2. 在樹的每個級別創建一系列分類器，啟用級聯路由方法。

例如，您可能有一個頂級分類器，將工單廣泛分類為"技術問題"、"計費問題"和"一般查詢"。這些類別中的每一個都可以有自己的子分類器來進一步細化分類。

![](/docs/images/ticket-hierarchy.png)

* **優點 - 更大的細微差別和準確度：** 您可以為每個父路徑創建不同的提示，允許更有針對性和上下文特定的分類。這可以導致改進的準確度和對客戶請求的更細緻的處理。

* **缺點 - 增加的延遲：** 請注意，多個分類器可能導致延遲增加，我們建議使用我們最快的模型 Haiku 實現此方法。

### 使用向量數據庫和相似性搜索檢索來處理高度可變的工單

儘管提供示例是改進效能的最有效方式，但如果支援請求高度可變，很難在單個提示中包含足夠的示例。

在這種情況下，您可以使用向量數據庫從示例數據集進行相似性搜索，並為給定查詢檢索最相關的示例。

這種方法在我們的[分類配方](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb)中詳細概述，已被證明可以將效能從 71% 準確度提高到 93% 準確度。

### 特別考慮預期的邊界情況

以下是 Claude 可能誤分類工單的一些場景（可能還有其他對您的情況獨特的場景）。在這些場景中，考慮在提示中提供明確的指示或示例，說明 Claude 應如何處理邊界情況：

    <section title="客戶提出隱含請求">

        客戶通常間接表達需求。例如，"我已經等待我的包裹超過兩週了"可能是對訂單狀態的間接請求。
        * **解決方案：** 為 Claude 提供一些這類請求的真實客戶示例，以及基礎意圖是什麼。如果您包含特別細緻的工單意圖的分類理由，您可以獲得更好的結果，以便 Claude 可以更好地將邏輯推廣到其他工單。
    
</section>
    <section title="Claude 優先考慮情感而非意圖">

        當客戶表達不滿時，Claude 可能會優先考慮解決情感而不是解決基礎問題。
        * **解決方案：** 為 Claude 提供關於何時優先考慮客戶情感或不優先考慮的方向。它可以像"忽略所有客戶情感一樣簡單。僅專注於分析客戶請求的意圖以及客戶可能要求的信息。"
    
</section>
    <section title="多個問題導致問題優先級混亂">

        當客戶在單個互動中提出多個問題時，Claude 可能難以識別主要問題。
        * **解決方案：** 澄清意圖的優先級，以便 Claude 可以更好地對提取的意圖進行排名並識別主要問題。
    
</section>

***

## 將 Claude 集成到您更大的支援工作流中

適當的集成需要您做出一些決策，關於您基於 Claude 的工單路由腳本如何適應您更大的工單路由系統的架構。有兩種方式可以做到這一點：
* **基於推送：** 您使用的支援工單系統（例如 Zendesk）通過向您的路由服務發送 webhook 事件來觸發您的代碼，然後分類意圖並路由它。
    * 這種方法更具網絡可擴展性，但需要您公開一個公共端點。
* **基於拉取：** 您的代碼根據給定的時間表拉取最新工單並在拉取時路由它們。
    * 這種方法更容易實現，但當拉取頻率過高時可能會對支援工單系統進行不必要的調用，或當拉取頻率過低時可能過於緩慢。

對於這兩種方法中的任何一種，您都需要將腳本包裝在服務中。方法的選擇取決於您的支援工單系統提供的 API。

***

<CardGroup cols={2}>
    <Card title="分類食譜" icon="link" href="https://github.com/anthropics/anthropic-cookbook/tree/main/capabilities/classification">
        訪問我們的分類食譜以獲取更多示例代碼和詳細的評估指導。
    </Card>
    <Card title="Claude 控制台" icon="link" href="/dashboard">
        開始在 Claude 控制台上構建和評估您的工作流。
    </Card>
</CardGroup>