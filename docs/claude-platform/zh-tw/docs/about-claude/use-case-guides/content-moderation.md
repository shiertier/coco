# 內容審核

內容審核是維護數位應用程式中安全、尊重和高效環境的關鍵方面。在本指南中，我們將討論如何使用 Claude 來審核您數位應用程式中的內容。

---

> 請造訪我們的[內容審核食譜](https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fmoderation%5Ffilter.ipynb)，查看使用 Claude 的內容審核實作範例。

<Tip>本指南專注於審核您應用程式中的使用者生成內容。如果您正在尋找審核與 Claude 互動的指導，請參考我們的[防護欄指南](/docs/zh-TW/test-and-evaluate/strengthen-guardrails/reduce-hallucinations)。</Tip>

## 使用 Claude 建構之前

### 決定是否使用 Claude 進行內容審核

以下是一些關鍵指標，表明您應該使用像 Claude 這樣的 LLM，而不是傳統的 ML 或基於規則的內容審核方法：

<section title="您想要具成本效益且快速的實作">
傳統的 ML 方法需要大量的工程資源、ML 專業知識和基礎設施成本。人工審核系統甚至會產生更高的成本。使用 Claude，您可以在極短的時間內以極低的價格建立一個複雜的審核系統。
</section>
<section title="您希望同時具備語義理解和快速決策">
傳統的 ML 方法，如詞袋模型或簡單的模式匹配，通常難以理解內容的語調、意圖和上下文。雖然人工審核系統在理解語義意義方面表現出色，但它們需要時間來審查內容。Claude 通過結合語義理解與快速提供審核決策的能力來彌補這一差距。
</section>
<section title="您需要一致的政策決策">
通過利用其先進的推理能力，Claude 可以統一解釋和應用複雜的審核指導原則。這種一致性有助於確保對所有內容的公平對待，降低可能破壞使用者信任的不一致或有偏見的審核決策風險。
</section>
<section title="您的審核政策可能會隨時間改變或演進">
一旦建立了傳統的 ML 方法，改變它是一項費力且資料密集的工作。另一方面，隨著您的產品或客戶需求的演變，Claude 可以輕鬆適應審核政策的變更或新增，而無需大量重新標記訓練資料。
</section>
<section title="您需要為審核決策提供可解釋的推理">
如果您希望為使用者或監管機構提供審核決策背後的清晰解釋，Claude 可以生成詳細且連貫的理由。這種透明度對於建立信任和確保內容審核實踐的問責制非常重要。
</section>
<section title="您需要多語言支援而無需維護單獨的模型">
傳統的 ML 方法通常需要為每種支援的語言建立單獨的模型或進行大量的翻譯過程。人工審核需要雇用精通每種支援語言的工作人員。Claude 的多語言能力使其能夠對各種語言的工單進行分類，而無需單獨的模型或大量的翻譯過程，為全球客戶群簡化了審核流程。
</section>
<section title="您需要多模態支援">
Claude 的多模態能力使其能夠分析和解釋文字和圖像的內容。這使其成為在需要一起評估不同媒體類型的環境中進行全面內容審核的多功能工具。
</section>

<Note>Anthropic 已訓練所有 Claude 模型誠實、有用且無害。這可能導致 Claude 審核被認為特別危險的內容（符合我們的[可接受使用政策](https://www.anthropic.com/legal/aup)），無論使用何種提示。例如，一個希望允許使用者發布明確性內容的成人網站可能會發現，即使他們在提示中指定不要審核明確性內容，Claude 仍會將明確內容標記為需要審核。我們建議在建立審核解決方案之前先審查我們的 AUP。</Note>

### 生成要審核的內容範例
在開發內容審核解決方案之前，首先創建應該被標記的內容和不應該被標記的內容的範例。確保包含邊緣案例和可能對內容審核系統有效處理困難的挑戰性場景。之後，審查您的範例以創建一個明確定義的審核類別清單。
例如，社交媒體平台生成的範例可能包括以下內容：

```python
allowed_user_comments = [
    'This movie was great, I really enjoyed it. The main actor really killed it!',
    'I hate Mondays.',
    'It is a great time to invest in gold!'
]

disallowed_user_comments = [
    'Delete this post now or you better hide. I am coming after you and your family.',
    'Stay away from the 5G cellphones!! They are using 5G to control you.',
    'Congratulations! You have won a $1,000 gift card. Click here to claim your prize!'
]

# Sample user comments to test the content moderation
user_comments = allowed_user_comments + disallowed_user_comments

# List of categories considered unsafe for content moderation
unsafe_categories = [
    'Child Exploitation',
    'Conspiracy Theories',
    'Hate',
    'Indiscriminate Weapons', 
    'Intellectual Property',
    'Non-Violent Crimes', 
    'Privacy',
    'Self-Harm',
    'Sex Crimes',
    'Sexual Content',
    'Specialized Advice',
    'Violent Crimes'
]
```

有效審核這些範例需要對語言的細緻理解。在評論 `This movie was great, I really enjoyed it. The main actor really killed it!` 中，內容審核系統需要識別出「killed it」是一個隱喻，而不是實際暴力的指示。相反，儘管沒有明確提及暴力，評論 `Delete this post now or you better hide. I am coming after you and your family.` 應該被內容審核系統標記。

`unsafe_categories` 清單可以根據您的特定需求進行自訂。例如，如果您希望防止未成年人在您的網站上創建內容，您可以將「未成年人發布」附加到清單中。

___

## 如何使用 Claude 審核內容

### 選擇正確的 Claude 模型
在選擇模型時，重要的是要考慮您資料的大小。如果成本是一個考慮因素，像 Claude Haiku 3 這樣的較小模型由於其成本效益而是一個絕佳選擇。以下是為每月接收十億篇貼文的社交媒體平台審核文字的成本估算：

* **內容大小**
    * 每月貼文數：10億
    * 每篇貼文字符數：100
    * 總字符數：1000億

* **估計代幣數**
    * 輸入代幣：286億（假設每3.5個字符1個代幣）
    * 被標記訊息的百分比：3%
    * 每個被標記訊息的輸出代幣：50
    * 總輸出代幣：15億

* **Claude Haiku 3 估計成本**
    * 輸入代幣成本：2,860 MTok * \$0.25/MTok = \$715
    * 輸出代幣成本：1,500 MTok * \$1.25/MTok = \$1,875
    * 月度成本：\$715 + \$1,875 = \$2,590

* **Claude Sonnet 4.5 估計成本**
    * 輸入代幣成本：2,860 MTok * \$3.00/MTok = \$8,580
    * 輸出代幣成本：1,500 MTok * \$15.00/MTok = \$22,500
    * 月度成本：\$8,580 + \$22,500 = \$31,080

<Tip>實際成本可能與這些估算不同。這些估算基於[批次處理](#consider-batch-processing)部分中強調的提示。通過從回應中移除 `explanation` 欄位，輸出代幣可以進一步減少。</Tip>  

### 建立強大的提示

為了使用 Claude 進行內容審核，Claude 必須理解您應用程式的審核要求。讓我們從編寫一個允許您定義審核需求的提示開始：

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def moderate_message(message, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Construct the prompt for Claude, including the message and unsafe categories
    assessment_prompt = f"""
    Determine whether the following message warrants moderation, 
    based on the unsafe categories outlined below.
    
    Message:
    <message>{message}</message>

    Unsafe Categories:
    <categories>
    {unsafe_category_str}
    </categories>

    Respond with ONLY a JSON object, using the format below:
    {{
    "violation": <Boolean field denoting whether the message should be moderated>,
    "categories": [Comma-separated list of violated categories],
    "explanation": [Optional. Only include if there is a violation.]
    }}"""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the violation status from the assessment
    contains_violation = assessment['violation']
    
    # If there's a violation, get the categories and explanation; otherwise, use empty defaults
    violated_categories = assessment.get('categories', []) if contains_violation else []
    explanation = assessment.get('explanation') if contains_violation else None
    
    return contains_violation, violated_categories, explanation

# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    violation, violated_categories, explanation = moderate_message(comment, unsafe_categories)
    
    if violation:
        print(f"Violated Categories: {', '.join(violated_categories)}")
        print(f"Explanation: {explanation}")
    else:
        print("No issues detected.")
```

在這個範例中，`moderate_message` 函數包含一個評估提示，其中包括不安全內容類別和我們希望評估的訊息。提示要求 Claude 根據我們定義的不安全類別評估訊息是否應該被審核。

然後解析模型的評估以確定是否存在違規。如果存在違規，Claude 還會返回違規類別清單，以及解釋訊息為何不安全的說明。

### 評估您的提示

內容審核是一個分類問題。因此，您可以使用我們[分類食譜](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/classification/guide.ipynb)中概述的相同技術來確定您內容審核系統的準確性。

另一個考慮因素是，您可以創建多個類別來表示各種風險級別，而不是將內容審核視為二元分類問題。創建多個風險級別允許您調整審核的積極性。例如，您可能希望自動阻止被認為是高風險的使用者查詢，而有許多中等風險查詢的使用者則被標記供人工審查。

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def assess_risk_level(message, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Construct the prompt for Claude, including the message, unsafe categories, and risk level definitions
    assessment_prompt = f"""
    Assess the risk level of the following message warrants moderation, 
    based on the unsafe categories listed below.
    
Message:
<message>{message}</message>

Unsafe Categories:
<categories>
{unsafe_category_str}
</categories>

Assign a risk level based on your confidence that the user's message should be moderated 
based on the defined unsafe categories:

0 - No risk
1 - Low risk
2 - Medium risk
3 - High risk

Respond with ONLY a JSON object, using the format below:
{{
  "risk_level": <Numerical field denoting the risk level>,
  "categories": [Comma-separated list of violated categories],
  "explanation": <Optional. Only include if risk level is greater than 0>
}}"""

    # Send the request to Claude for risk assessment
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the risk level, violated categories, and explanation from the assessment
    risk_level = assessment["risk_level"]
    violated_categories = assessment["categories"]
    explanation = assessment.get("explanation")
    
    return risk_level, violated_categories, explanation

# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    risk_level, violated_categories, explanation = assess_risk_level(comment, unsafe_categories)
    
    print(f"Risk Level: {risk_level}")
    if violated_categories:
        print(f"Violated Categories: {', '.join(violated_categories)}")
    if explanation:
        print(f"Explanation: {explanation}")
```

這段程式碼實作了一個 `assess_risk_level` 函數，使用 Claude 來評估訊息的風險級別。該函數接受一個訊息和一個不安全類別清單作為輸入。

在函數內部，為 Claude 生成一個提示，包括要評估的訊息、不安全類別，以及評估風險級別的具體指示。提示指示 Claude 回應一個 JSON 物件，包括風險級別、違規類別和可選的解釋。

這種方法通過分配風險級別實現靈活的內容審核。它可以無縫整合到更大的系統中，根據評估的風險級別自動過濾內容或標記評論供人工審查。例如，在執行此程式碼時，評論 `Delete this post now or you better hide. I am coming after you and your family.` 由於其危險威脅被識別為高風險。相反，評論 `Stay away from the 5G cellphones!! They are using 5G to control you.` 被歸類為中等風險。

### 部署您的提示

一旦您對解決方案的品質有信心，就是將其部署到生產環境的時候了。以下是在生產環境中使用內容審核時要遵循的一些最佳實踐：

1. **為使用者提供清晰的回饋：** 當使用者輸入被阻止或回應因內容審核而被標記時，提供資訊豐富且建設性的回饋，幫助使用者理解為什麼他們的訊息被標記，以及如何適當地重新表述。在上面的編碼範例中，這是通過 Claude 回應中的 `explanation` 標籤來完成的。

2. **分析被審核的內容：** 追蹤您的審核系統標記的內容類型，以識別趨勢和潛在的改進領域。

3. **持續評估和改進：** 使用精確度和召回率追蹤等指標定期評估您內容審核系統的性能。使用這些資料來迭代改進您的審核提示、關鍵字和評估標準。

___

## 提升性能

在複雜場景中，除了標準的[提示工程技術](/docs/zh-TW/build-with-claude/prompt-engineering/overview)之外，考慮額外的策略來提升性能可能會有所幫助。以下是一些進階策略：

### 定義主題並提供範例

除了在提示中列出不安全類別外，還可以通過提供與每個類別相關的定義和短語來進一步改進。

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

# Dictionary of categories considered unsafe for content moderation, with their definitions
unsafe_category_definitions = {
    'Child Exploitation': 'Content that depicts child nudity or that enables, encourages, excuses, or depicts the sexual abuse of children.',
    'Conspiracy Theories': """Content that promotes or endorses unfounded, false, or misleading theories about events, situations, or individuals, 
        which can incite harm or undermine public trust in institutions or facts.""",
    'Hate': """Content that is hateful toward people on the basis of their protected characteristics 
        (race, color, ethnicity, national origin, disability, religious affiliation, caste, sexual orientation, sex, gender identity and serious disease), 
        or content that perpetuates negative stereotypes.""",
    'Indiscriminate Weapons': 'Content that enables the creation of weapons of mass destruction.', 
    'Intellectual Property': 'Content that violates the intellectual property rights of third parties.',
    'Non-Violent Crimes': 'Content that enables, encourages, or excuses the commission of non-violent crimes.', 
    'Privacy': 'Content that contains sensitive, personal information about private individuals.',
    'Self-Harm': 'Content that enables, encourages, or excuses acts of intentional self-harm.',
    'Sex Crimes': 'Content that enables, encourages, or excuses the commission of sex crimes.',
    'Sexual Content': 'Content that contains erotic descriptions or explicit depictions of sex acts.',
    'Specialized Advice': """Content that contains financial, medical, or legal advice. 
        Financial advice includes guidance on investments, stocks, bonds, or any financial planning.""",
    'Violent Crimes': 'Content that enables, encourages, or excuses the commission of violent crimes.', 
}


def moderate_message_with_definitions(message, unsafe_category_definitions):
    # Format unsafe categories string, with each category and its definition on a new line
    unsafe_category_str = '\n'.join(f'{category}: {definition}' 
        for category, definition in unsafe_category_definitions.items()
    )
    
    # Construct the prompt for Claude, including the message and unsafe categories
    assessment_prompt = f"""Determine whether the following message warrants moderation, based on the unsafe categories outlined below.

Message:
<message>{message}</message>

Unsafe Categories and Their Definitions:
<categories>
{unsafe_category_str}
</categories>

It's important that you remember all unsafe categories and their definitions.

Respond with ONLY a JSON object, using the format below:
{{
  "violation": <Boolean field denoting whether the message should be moderated>,
  "categories": [Comma-separated list of violated categories],
  "explanation": [Optional. Only include if there is a violation.]
}}"""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the violation status from the assessment
    contains_violation = assessment['violation']
    
    # If there's a violation, get the categories and explanation; otherwise, use empty defaults
    violated_categories = assessment.get('categories', []) if contains_violation else []
    explanation = assessment.get('explanation') if contains_violation else None
    
    return contains_violation, violated_categories, explanation


# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    violation, violated_categories, explanation = moderate_message_with_definitions(comment, unsafe_category_definitions)
    
    if violation:
        print(f"Violated Categories: {', '.join(violated_categories)}")
        print(f"Explanation: {explanation}")
    else:
        print("No issues detected.")
```

`moderate_message_with_definitions` 函數通過允許每個不安全類別與詳細定義配對來擴展早期的 `moderate_message` 函數。這在程式碼中通過將原始函數中的 `unsafe_categories` 清單替換為 `unsafe_category_definitions` 字典來實現。這個字典將每個不安全類別映射到其對應的定義。類別名稱和其定義都包含在提示中。

值得注意的是，`Specialized Advice` 類別的定義現在指定了應該被禁止的金融建議類型。因此，之前通過 `moderate_message` 評估的評論 `It's a great time to invest in gold!` 現在觸發了違規。

### 考慮批次處理

在不需要即時審核的情況下，為了降低成本，考慮批次審核訊息。在提示的上下文中包含多個訊息，並要求 Claude 評估哪些訊息應該被審核。

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def batch_moderate_messages(messages, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Format messages string, with each message wrapped in XML-like tags and given an ID
    messages_str = '\n'.join([f'<message id={idx}>{msg}</message>' for idx, msg in enumerate(messages)])
    
    # Construct the prompt for Claude, including the messages and unsafe categories
    assessment_prompt = f"""Determine the messages to moderate, based on the unsafe categories outlined below.

Messages:
<messages>
{messages_str}
</messages>

Unsafe categories and their definitions:
<categories>
{unsafe_category_str}
</categories>

Respond with ONLY a JSON object, using the format below:
{{
  "violations": [
    {{
      "id": <message id>,
      "categories": [list of violated categories],
      "explanation": <Explanation of why there's a violation>
    }},
    ...
  ]
}}

Important Notes:
- Remember to analyze every message for a violation.
- Select any number of violations that reasonably apply."""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=2048,  # Increased max token count to handle batches
        temperature=0,    # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    return assessment


# Process the batch of comments and get the response
response_obj = batch_moderate_messages(user_comments, unsafe_categories)

# Print the results for each detected violation
for violation in response_obj['violations']:
    print(f"""Comment: {user_comments[violation['id']]}
Violated Categories: {', '.join(violation['categories'])}
Explanation: {violation['explanation']}
""")
```
在這個範例中，`batch_moderate_messages` 函數通過單次 Claude API 呼叫處理整批訊息的審核。
在函數內部，創建一個提示，包括要評估的訊息清單、定義的不安全內容類別及其描述。提示指示 Claude 返回一個 JSON 物件，列出所有包含違規的訊息。回應中的每個訊息都通過其 id 識別，該 id 對應於訊息在輸入清單中的位置。
請記住，為您的特定需求找到最佳批次大小可能需要一些實驗。雖然較大的批次大小可以降低成本，但它們也可能導致品質略有下降。此外，您可能需要增加 Claude API 呼叫中的 `max_tokens` 參數以容納更長的回應。有關您選擇的模型可以輸出的最大代幣數的詳細資訊，請參考[模型比較頁面](/docs/zh-TW/about-claude/models#model-comparison-table)。

<CardGroup cols={2}> 
  <Card title="內容審核食譜" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fmoderation%5Ffilter.ipynb">
    查看如何使用 Claude 進行內容審核的完整實作程式碼範例。
  </Card>
  <Card title="防護欄指南" icon="link" href="/docs/zh-TW/test-and-evaluate/strengthen-guardrails/reduce-hallucinations">
    探索我們的防護欄指南，了解審核與 Claude 互動的技術。
  </Card>
</CardGroup>