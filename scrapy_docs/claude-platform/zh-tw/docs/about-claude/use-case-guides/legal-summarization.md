# 法律文件摘要

本指南介紹如何利用 Claude 的先進自然語言處理能力，有效地總結法律文件、提取關鍵資訊並加快法律研究。使用 Claude，您可以簡化合約審查、訴訟準備和監管工作的流程，節省時間並確保法律流程的準確性。

---

> 訪問我們的[摘要食譜](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb)，查看使用 Claude 的法律摘要實現範例。

## 使用 Claude 進行構建前

### 決定是否使用 Claude 進行法律文件摘要

以下是一些關鍵指標，表明您應該使用 Claude 等大型語言模型來總結法律文件：

<section title="您想要高效且經濟地審查大量文件">
大規模文件審查在手動進行時可能耗時且昂貴。Claude 可以快速處理和總結大量法律文件，大幅減少與文件審查相關的時間和成本。此功能對於盡職調查、合約分析或訴訟發現等效率至關重要的任務特別有價值。
</section>
<section title="您需要自動提取關鍵元數據">
Claude 可以有效地從法律文件中提取和分類重要的元數據，例如涉及方、日期、合約條款或特定條款。此自動提取可以幫助組織資訊，使其更容易搜尋、分析和管理大型文件集。這對於合約管理、合規檢查或建立可搜尋的法律資訊資料庫特別有用。
</section>
<section title="您想要生成清晰、簡潔且標準化的摘要">
Claude 可以生成遵循預定格式的結構化摘要，使法律專業人士能夠快速掌握各種文件的要點。這些標準化摘要可以提高可讀性、促進文件之間的比較，並增強整體理解，特別是在處理複雜的法律語言或技術術語時。
</section>
<section title="您需要為摘要提供精確的引用">
在建立法律摘要時，適當的歸因和引用對於確保可信度和遵守法律標準至關重要。Claude 可以被提示為所有參考的法律要點包含準確的引用，使法律專業人士更容易審查和驗證總結的資訊。
</section>
<section title="您想要簡化和加快法律研究流程">
Claude 可以通過快速分析大量案例法、法規和法律評論來協助法律研究。它可以識別相關先例、提取關鍵法律原則並總結複雜的法律論證。此功能可以大幅加快研究流程，使法律專業人士能夠專注於更高層次的分析和策略開發。
</section>

### 確定您希望摘要提取的詳細資訊
任何給定文件都沒有單一正確的摘要。沒有明確的指導，Claude 可能難以確定要包含哪些詳細資訊。為了獲得最佳結果，請確定您想要在摘要中包含的具體資訊。

例如，在總結轉租協議時，您可能希望提取以下關鍵要點：

```python
details_to_extract = [
    'Parties involved (sublessor, sublessee, and original lessor)',
    'Property details (address, description, and permitted use)', 
    'Term and rent (start date, end date, monthly rent, and security deposit)',
    'Responsibilities (utilities, maintenance, and repairs)',
    'Consent and notices (landlord\'s consent, and notice requirements)',
    'Special provisions (furniture, parking, and subletting restrictions)'
]
```

### 建立成功標準

評估摘要的品質是一項公認的具有挑戰性的任務。與許多其他自然語言處理任務不同，摘要的評估通常缺乏明確的客觀指標。此過程可能高度主觀，不同的讀者重視摘要的不同方面。以下是您在評估 Claude 執行法律摘要的效果時可能希望考慮的標準。

<section title="事實正確性">
摘要應準確代表文件中的事實、法律概念和要點。
</section>
<section title="法律精確性">
術語和對法規、案例法或法規的參考必須正確並符合法律標準。
</section>
<section title="簡潔性">
摘要應將法律文件濃縮為其基本要點，而不會遺失重要詳細資訊。
</section>
<section title="一致性">
如果總結多份文件，大型語言模型應對每份摘要保持一致的結構和方法。
</section>
<section title="可讀性">
文本應清晰易懂。如果受眾不是法律專家，摘要不應包含可能使受眾困惑的法律術語。
</section>
<section title="偏見和公平性">
摘要應呈現對法律論證和立場的無偏見和公平描述。
</section>

有關更多資訊，請參閱我們的[建立成功標準](/docs/zh-TW/test-and-evaluate/define-success)指南。

---

## 如何使用 Claude 總結法律文件

### 選擇正確的 Claude 模型

在總結法律文件時，模型準確性極其重要。Claude Sonnet 4.5 是需要高準確性的此類用例的絕佳選擇。如果您的文件大小和數量很大，以至於成本開始成為問題，您也可以嘗試使用較小的模型，如 Claude Haiku 4.5。

為了幫助估計這些成本，以下是使用 Sonnet 和 Haiku 總結 1,000 份轉租協議的成本比較：

* **內容大小**
    * 協議數量：1,000
    * 每份協議的字元數：300,000
    * 總字元數：300M

* **估計的代幣**
    * 輸入代幣：86M（假設每 3.5 個字元 1 個代幣）
    * 每份摘要的輸出代幣：350
    * 總輸出代幣：350,000
 
* **Claude Sonnet 4.5 估計成本**
    * 輸入代幣成本：86 MTok * \$3.00/MTok = \$258
    * 輸出代幣成本：0.35 MTok * \$15.00/MTok = \$5.25
    * 總成本：\$258.00 + \$5.25 = \$263.25

* **Claude Haiku 3 估計成本**
    * 輸入代幣成本：86 MTok * \$0.25/MTok = \$21.50
    * 輸出代幣成本：0.35 MTok * \$1.25/MTok = \$0.44
    * 總成本：\$21.50 + \$0.44 = \$21.96

<Tip>實際成本可能與這些估計不同。這些估計基於[提示](#build-a-strong-prompt)部分中突出顯示的範例。</Tip>

### 將文件轉換為 Claude 可以處理的格式

在開始總結文件之前，您需要準備您的數據。這涉及從 PDF 中提取文本、清理文本並確保其準備好由 Claude 處理。

以下是此流程在示例 pdf 上的演示：

```python
from io import BytesIO
import re

import pypdf
import requests

def get_llm_text(pdf_file):
    reader = pypdf.PdfReader(pdf_file)
    text = "\n".join([page.extract_text() for page in reader.pages])

    # Remove extra whitespace
    text = re.sub(r'\s+', ' ', text) 

    # Remove page numbers
    text = re.sub(r'\n\s*\d+\s*\n', '\n', text) 

    return text


# Create the full URL from the GitHub repository
url = "https://raw.githubusercontent.com/anthropics/anthropic-cookbook/main/skills/summarization/data/Sample Sublease Agreement.pdf"
url = url.replace(" ", "%20")

# Download the PDF file into memory
response = requests.get(url)

# Load the PDF from memory
pdf_file = BytesIO(response.content)

document_text = get_llm_text(pdf_file) 
print(document_text[:50000]) 
```

在此範例中，我們首先下載[摘要食譜](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/data/Sample%20Sublease%20Agreement.pdf)中使用的示例轉租協議的 pdf。此協議來自[sec.gov 網站](https://www.sec.gov/Archives/edgar/data/1045425/000119312507044370/dex1032.htm)上的公開轉租協議。

我們使用 pypdf 庫提取 pdf 的內容並將其轉換為文本。然後通過移除額外的空白和頁碼來清理文本數據。

### 構建強大的提示

Claude 可以適應各種摘要風格。您可以更改提示的詳細資訊，以指導 Claude 更詳細或更簡潔、包含更多或更少的技術術語，或提供更高或更低層次的上下文摘要。

以下是如何建立提示的範例，以確保在分析轉租協議時生成的摘要遵循一致的結構：

```python
import anthropic

# Initialize the Anthropic client
client = anthropic.Anthropic()

def summarize_document(text, details_to_extract, model="claude-sonnet-4-5", max_tokens=1000):

    # Format the details to extract to be placed within the prompt's context
    details_to_extract_str = '\n'.join(details_to_extract)
    
    # Prompt the model to summarize the sublease agreement
    prompt = f"""Summarize the following sublease agreement. Focus on these key aspects:

    {details_to_extract_str}

    Provide the summary in bullet points nested within the XML header for each section. For example:

    <parties involved>
    - Sublessor: [Name]
    // Add more details as needed
    </parties involved>
    
    If any information is not explicitly stated in the document, note it as "Not specified". Do not preamble.

    Sublease agreement text:
    {text}
    """

    response = client.messages.create(
        model=model,
        max_tokens=max_tokens,
        system="You are a legal analyst specializing in real estate law, known for highly accurate and detailed summaries of sublease agreements.",
        messages=[
            {"role": "user", "content": prompt},
            {"role": "assistant", "content": "Here is the summary of the sublease agreement: <summary>"}
        ],
        stop_sequences=["</summary>"]
    )

    return response.content[0].text

sublease_summary = summarize_document(document_text, details_to_extract)
print(sublease_summary)
```

此代碼實現了一個 `summarize_document` 函數，該函數使用 Claude 總結轉租協議的內容。該函數接受文本字符串和要提取的詳細資訊列表作為輸入。在此範例中，我們使用之前代碼片段中定義的 `document_text` 和 `details_to_extract` 變數呼叫該函數。

在函數中，為 Claude 生成提示，包括要總結的文件、要提取的詳細資訊以及總結文件的具體說明。提示指示 Claude 在 XML 標頭中以嵌套方式回應每個詳細資訊提取的摘要。

因為我們決定在標籤中輸出摘要的每個部分，所以每個部分可以輕鬆地作為後處理步驟進行解析。此方法可實現結構化摘要，可針對您的用例進行調整，以便每份摘要遵循相同的模式。

### 評估您的提示

提示通常需要測試和優化才能準備好用於生產。為了確定解決方案的準備情況，請使用結合定量和定性方法的系統流程評估摘要的品質。根據您定義的成功標準建立[強大的經驗評估](/docs/zh-TW/build-with-claude/develop-tests#building-evals-and-test-cases)將允許您優化提示。以下是您可能希望在經驗評估中包含的一些指標：

<section title="ROUGE 分數">
這測量生成的摘要與專家建立的參考摘要之間的重疊。此指標主要關注召回率，對於評估內容涵蓋範圍很有用。
</section>
<section title="BLEU 分數">
雖然最初是為機器翻譯開發的，但此指標可適應摘要任務。BLEU 分數測量生成的摘要與參考摘要之間的 n-gram 匹配的精確度。較高的分數表示生成的摘要包含與參考摘要相似的短語和術語。
</section>
<section title="上下文嵌入相似性">
此指標涉及建立生成的摘要和參考摘要的向量表示（嵌入）。然後計算這些嵌入之間的相似性，通常使用餘弦相似性。較高的相似性分數表示生成的摘要捕捉了參考摘要的語義含義和上下文，即使確切的措辭不同。
</section>
<section title="基於大型語言模型的評分">
此方法涉及使用 Claude 等大型語言模型根據評分標準評估生成的摘要品質。標準可以根據您的具體需求進行定制，評估準確性、完整性和連貫性等關鍵因素。有關實現基於大型語言模型的評分的指導，請查看這些[提示](/docs/zh-TW/build-with-claude/develop-tests#tips-for-llm-based-grading)。
</section>
<section title="人工評估">
除了建立參考摘要外，法律專家還可以評估生成的摘要的品質。雖然這在規模上既昂貴又耗時，但在部署到生產之前通常對少數摘要進行此操作作為健全性檢查。
</section>

### 部署您的提示

以下是部署解決方案到生產時要牢記的一些其他考慮事項。

1. **確保沒有責任**：了解摘要中錯誤的法律含義，這可能導致您的組織或客戶的法律責任。提供免責聲明或法律通知，澄清摘要由人工智能生成，應由法律專業人士審查。

2. **處理多種文件類型**：在本指南中，我們討論了如何從 PDF 中提取文本。在現實中，文件可能採用多種格式（PDF、Word 文件、文本文件等）。確保您的數據提取管道可以轉換您期望接收的所有文件格式。

3. **並行化對 Claude 的 API 呼叫**：具有大量代幣的長文件可能需要長達一分鐘的時間才能讓 Claude 生成摘要。對於大型文件集合，您可能希望並行向 Claude 發送 API 呼叫，以便摘要可以在合理的時間範圍內完成。請參閱 Anthropic 的[速率限制](/docs/zh-TW/api/rate-limits#rate-limits)以確定可以並行執行的最大 API 呼叫數。

---

## 改進效能

在複雜的情況下，除了標準[提示工程技術](/docs/zh-TW/build-with-claude/prompt-engineering/overview)外，考慮其他策略來改進效能可能會有所幫助。以下是一些進階策略：

### 執行元摘要以總結長文件

法律摘要通常涉及處理長文件或一次處理許多相關文件，使得您超過 Claude 的上下文視窗。您可以使用稱為元摘要的分塊方法來處理此用例。此技術涉及將文件分解為較小的、可管理的塊，然後分別處理每個塊。然後，您可以組合每個塊的摘要以建立整個文件的元摘要。

以下是如何執行元摘要的範例：

```python
import anthropic

# Initialize the Anthropic client
client = anthropic.Anthropic()

def chunk_text(text, chunk_size=20000):
    return [text[i:i+chunk_size] for i in range(0, len(text), chunk_size)]

def summarize_long_document(text, details_to_extract, model="claude-sonnet-4-5", max_tokens=1000):

    # Format the details to extract to be placed within the prompt's context
    details_to_extract_str = '\n'.join(details_to_extract)

    # Iterate over chunks and summarize each one
    chunk_summaries = [summarize_document(chunk, details_to_extract, model=model, max_tokens=max_tokens) for chunk in chunk_text(text)]
    
    final_summary_prompt = f"""
    
    You are looking at the chunked summaries of multiple documents that are all related. 
    Combine the following summaries of the document from different truthful sources into a coherent overall summary:

    <chunked_summaries>
    {"".join(chunk_summaries)}
    </chunked_summaries>

    Focus on these key aspects:
    {details_to_extract_str})

    Provide the summary in bullet points nested within the XML header for each section. For example:

    <parties involved>
    - Sublessor: [Name]
    // Add more details as needed
    </parties involved>
    
    If any information is not explicitly stated in the document, note it as "Not specified". Do not preamble.
    """

    response = client.messages.create(
        model=model,
        max_tokens=max_tokens,
        system="You are a legal expert that summarizes notes on one document.",
        messages=[
            {"role": "user",  "content": final_summary_prompt},
            {"role": "assistant", "content": "Here is the summary of the sublease agreement: <summary>"}

        ],
        stop_sequences=["</summary>"]
    )
    
    return response.content[0].text

long_summary = summarize_long_document(document_text, details_to_extract)
print(long_summary)
```

`summarize_long_document` 函數通過將文件分割成較小的塊並分別總結每個塊來建立在早期 `summarize_document` 函數的基礎上。

代碼通過將 `summarize_document` 函數應用於原始文件中的每個 20,000 字元的塊來實現此目的。然後組合各個摘要，並從這些塊摘要建立最終摘要。

請注意，`summarize_long_document` 函數對於我們的範例 pdf 並非嚴格必要的，因為整個文件適合 Claude 的上下文視窗。但是，對於超過 Claude 上下文視窗的文件或在總結多個相關文件時，它變得必不可少。無論如何，此元摘要技術通常在最終摘要中捕捉到早期單摘要方法中遺漏的其他重要詳細資訊。

### 使用摘要索引文件探索大型文件集合

使用大型語言模型搜尋文件集合通常涉及檢索增強生成 (RAG)。但是，在涉及大型文件或當精確資訊檢索至關重要時，基本 RAG 方法可能不夠。摘要索引文件是一種進階 RAG 方法，提供了一種更有效的方式來對檢索文件進行排名，使用的上下文比傳統 RAG 方法少。在此方法中，您首先使用 Claude 為語料庫中的每個文件生成簡潔摘要，然後使用 Claude 對每份摘要與所提出的查詢的相關性進行排名。有關此方法的進一步詳細資訊，包括基於代碼的範例，請查看[摘要食譜](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb)中的摘要索引文件部分。

### 微調 Claude 以從您的數據集中學習

改進 Claude 生成摘要能力的另一種進階技術是微調。微調涉及在與您的法律摘要需求特別一致的自訂數據集上訓練 Claude，確保 Claude 適應您的用例。以下是如何執行微調的概述：

1. **識別錯誤**：首先收集 Claude 摘要不足的實例 - 這可能包括遺漏關鍵法律詳細資訊、誤解上下文或使用不適當的法律術語。

2. **策劃數據集**：一旦您識別了這些問題，請編譯這些有問題範例的數據集。此數據集應包括原始法律文件以及您更正的摘要，確保 Claude 學習所需的行為。

3. **執行微調**：微調涉及在您的策劃數據集上重新訓練模型以調整其權重和參數。此重新訓練幫助 Claude 更好地理解您的法律領域的具體要求，改進其根據您的標準總結文件的能力。

4. **迭代改進**：微調不是一次性流程。當 Claude 繼續生成摘要時，您可以迭代地添加它表現不佳的新範例，進一步完善其功能。隨著時間的推移，此持續反饋循環將產生一個高度專門用於您的法律摘要任務的模型。

<Tip>微調目前僅通過 Amazon Bedrock 提供。其他詳細資訊可在 [AWS 啟動部落格](https://aws.amazon.com/blogs/machine-learning/fine-tune-anthropics-claude-3-haiku-in-amazon-bedrock-to-boost-model-accuracy-and-quality/)中取得。</Tip>

<CardGroup cols={2}> 
  <Card title="摘要食譜" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb">
    查看如何使用 Claude 總結合約的完整實現代碼範例。
  </Card>
  <Card title="引用食譜" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb">
    探索我們的引用食譜配方，以獲得有關如何確保資訊準確性和可解釋性的指導。
  </Card>
</CardGroup>