# 客戶支援代理

本指南介紹如何利用 Claude 的先進對話能力即時處理客戶查詢，提供 24/7 支援、減少等待時間，並以準確的回應和積極的互動管理高支援量。

---

## 使用 Claude 進行構建之前

### 決定是否使用 Claude 進行支援聊天

以下是一些關鍵指標，表明您應該使用 Claude 等 LLM 來自動化客戶支援流程的部分：

  <section title="大量重複查詢">

    Claude 擅長高效處理大量類似問題，為人工代理騰出時間處理更複雜的問題。
  
</section>
  <section title="需要快速資訊合成">

    Claude 可以快速檢索、處理和整合來自龐大知識庫的資訊，而人工代理可能需要時間進行研究或查詢多個來源。
  
</section>
  <section title="24/7 可用性要求">

    Claude 可以提供全天候支援而不會疲勞，而為持續覆蓋配備人工代理可能成本高昂且具有挑戰性。
  
</section>
  <section title="高峰期快速擴展">

    Claude 可以在查詢量突然增加時進行處理，無需招聘和培訓額外員工。
  
</section>
  <section title="一致的品牌聲音">

    您可以指示 Claude 始終代表您品牌的語調和價值觀，而人工代理可能在溝通風格上有所不同。
  
</section>

選擇 Claude 而不是其他 LLM 的一些考慮因素：

- 您優先考慮自然、細緻的對話：Claude 的複雜語言理解能力允許進行更自然、上下文感知的對話，感覺比與其他 LLM 的聊天更像人類。
- 您經常收到複雜和開放式查詢：Claude 可以處理各種主題和查詢，而無需生成罐裝回應或需要對用戶表述的排列進行廣泛編程。
- 您需要可擴展的多語言支援：Claude 的多語言能力允許它用 200 多種語言進行對話，無需為每種支援的語言單獨使用聊天機器人或進行廣泛的翻譯流程。

### 定義您的理想聊天互動

概述理想的客戶互動，以定義您期望客戶與 Claude 互動的方式和時間。此概述將幫助確定您解決方案的技術要求。

以下是汽車保險客戶支援的示例聊天互動：

* **客戶**：啟動支援聊天體驗
   * **Claude**：熱情地問候客戶並啟動對話
* **客戶**：詢問他們新電動車的保險
   * **Claude**：提供有關電動車覆蓋的相關資訊
* **客戶**：提出與電動車保險獨特需求相關的問題
   * **Claude**：以準確和資訊豐富的答案回應，並提供來源連結
* **客戶**：提出與保險或汽車無關的題外話題問題
   * **Claude**：澄清它不討論無關主題，並將用戶引導回汽車保險
* **客戶**：表示對保險報價感興趣
   * **Claude**：提出一系列問題以確定適當的報價，根據他們的回應進行調整
   * **Claude**：發送使用報價生成 API 工具的請求，以及從用戶收集的必要資訊
   * **Claude**：接收來自 API 工具使用的回應資訊，將資訊合成為自然回應，並向用戶呈現提供的報價
* **客戶**：提出後續問題
   * **Claude**：根據需要回答後續問題
   * **Claude**：指導客戶進行保險流程中的後續步驟並結束對話

<Tip>在您為自己的用例編寫的真實示例中，您可能會發現寫出此互動中的實際措辭很有用，這樣您也可以了解您希望 Claude 具有的理想語調、回應長度和詳細程度。</Tip>

### 將互動分解為獨特任務

客戶支援聊天是多個不同任務的集合，從問題解答到資訊檢索再到對請求採取行動，全部包含在單個客戶互動中。在開始構建之前，將您的理想客戶互動分解為您希望 Claude 能夠執行的每項任務。這確保您可以為每項任務提示和評估 Claude，並讓您很好地了解在編寫測試用例時需要考慮的互動範圍。

<Tip>客戶有時會發現將其視為可能對話轉折點的互動流程圖很有幫助，具體取決於用戶請求。</Tip>

以下是與上述示例保險互動相關的關鍵任務：

1. 問候和一般指導
   - 熱情地問候客戶並啟動對話
   - 提供有關公司和互動的一般資訊

2. 產品資訊
   - 提供有關電動車覆蓋的資訊
   <Note>這將要求 Claude 在其上下文中擁有必要的資訊，並可能意味著需要 [RAG 整合](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/retrieval_augmented_generation/guide.ipynb)。</Note>
   - 回答與獨特電動車保險需求相關的問題
   - 回答有關報價或保險詳細資訊的後續問題
   - 在適當時提供來源連結

3. 對話管理
   - 保持主題（汽車保險）
   - 將題外問題重新引導回相關主題

4. 報價生成
   - 提出適當的問題以確定報價資格
   - 根據客戶回應調整問題
   - 將收集的資訊提交到報價生成 API
   - 向客戶呈現提供的報價

### 建立成功標準

與您的支援團隊合作 [定義明確的成功標準](/docs/zh-TW/test-and-evaluate/define-success) 並編寫 [詳細評估](/docs/zh-TW/test-and-evaluate/develop-tests) 以及可測量的基準和目標。

以下是可用於評估 Claude 成功執行定義任務的標準和基準：

  <section title="查詢理解準確度">

    此指標評估 Claude 在各種主題中準確理解客戶查詢的程度。通過審查一組對話樣本並評估 Claude 是否正確解釋了客戶意圖、關鍵後續步驟、成功解決方案的樣子等來測量。目標是達到 95% 或更高的理解準確度。
  
</section>
  <section title="回應相關性">

    這評估 Claude 的回應在多大程度上解決了客戶的具體問題或問題。評估一組對話並評估每個回應的相關性（使用基於 LLM 的分級以進行擴展）。目標相關性得分為 90% 或以上。
  
</section>
  <section title="回應準確度">

    根據提供給 Claude 上下文中的資訊，評估提供給用戶的一般公司和產品資訊的正確性。在此介紹性資訊中目標為 100% 準確度。
  
</section>
  <section title="引用提供相關性">

    追蹤提供的連結或來源的頻率和相關性。目標是在 80% 的互動中提供相關來源，其中額外資訊可能有益。
  
</section>
  <section title="主題遵守">

    測量 Claude 在主題上的表現如何，例如我們示例實現中的汽車保險主題。目標是 95% 的回應與汽車保險或客戶的具體查詢直接相關。
  
</section>
  <section title="內容生成有效性">

    測量 Claude 在確定何時生成資訊內容以及該內容的相關性方面的成功程度。例如，在我們的實現中，我們將確定 Claude 在多大程度上理解何時生成報價以及該報價的準確性。目標為 100% 準確度，因為這是成功客戶互動的重要資訊。
  
</section>
  <section title="升級效率">

    這測量 Claude 識別何時查詢需要人工干預並適當升級的能力。追蹤正確升級的對話百分比與應該升級但未升級的對話。目標升級準確度為 95% 或更高。
  
</section>

以下是可用於評估採用 Claude 進行支援的業務影響的標準和基準：

  <section title="情感維護">

    這評估 Claude 在整個對話中維護或改善客戶情感的能力。使用情感分析工具測量每個對話開始和結束時的情感。目標是在 90% 的互動中維護或改善情感。
  
</section>
  <section title="偏轉率">

    聊天機器人成功處理的客戶查詢百分比，無需人工干預。通常目標為 70-80% 的偏轉率，具體取決於查詢的複雜性。
  
</section>
  <section title="客戶滿意度得分">

    衡量客戶對其聊天機器人互動滿意度的指標。通常通過互動後調查進行。目標 CSAT 得分為 5 分中的 4 分或更高。
  
</section>
  <section title="平均處理時間">

    聊天機器人解決查詢所需的平均時間。這因問題的複雜性而異，但通常目標是與人工代理相比更低的 AHT。
  
</section>

## 如何將 Claude 實現為客戶服務代理

### 選擇正確的 Claude 模型

模型的選擇取決於成本、準確度和回應時間之間的權衡。

對於客戶支援聊天，Claude Sonnet 4.5 非常適合平衡智能、延遲和成本。但是，對於具有包括 RAG、工具使用和/或長上下文提示的多個提示的對話流的情況，Claude Haiku 4.5 可能更適合優化延遲。

### 構建強大的提示

使用 Claude 進行客戶支援需要 Claude 有足夠的方向和上下文以適當地回應，同時具有足夠的靈活性來處理各種客戶查詢。

讓我們從編寫強大提示的元素開始，從系統提示開始：

```python
IDENTITY = """You are Eva, a friendly and knowledgeable AI assistant for Acme Insurance 
Company. Your role is to warmly welcome customers and provide information on 
Acme's insurance offerings, which include car insurance and electric car 
insurance. You can also help customers get quotes for their insurance needs."""
```

<Tip>雖然您可能想將所有資訊放在系統提示中作為將指令與用戶對話分開的方式，但 Claude 實際上在將大部分提示內容寫在第一個 `User` 轉向中時效果最好（唯一的例外是角色提示）。在 [使用系統提示為 Claude 提供角色](/docs/zh-TW/build-with-claude/prompt-engineering/system-prompts) 中閱讀更多內容。</Tip>

最好將複雜提示分解為小節，一次編寫一部分。對於每項任務，您可能會通過遵循逐步流程來定義 Claude 執行任務所需的提示部分而獲得更大的成功。對於此汽車保險客戶支援示例，我們將逐段編寫提示的所有部分，從"問候和一般指導"任務開始。這也使調試提示更容易，因為您可以更快地調整整體提示的各個部分。

我們將把所有這些片段放在一個名為 `config.py` 的文件中。

```python
STATIC_GREETINGS_AND_GENERAL = """
<static_context>
Acme Auto Insurance: Your Trusted Companion on the Road

About:
At Acme Insurance, we understand that your vehicle is more than just a mode of transportation—it's your ticket to life's adventures. 
Since 1985, we've been crafting auto insurance policies that give drivers the confidence to explore, commute, and travel with peace of mind.
Whether you're navigating city streets or embarking on cross-country road trips, Acme is there to protect you and your vehicle. 
Our innovative auto insurance policies are designed to adapt to your unique needs, covering everything from fender benders to major collisions.
With Acme's award-winning customer service and swift claim resolution, you can focus on the joy of driving while we handle the rest. 
We're not just an insurance provider—we're your co-pilot in life's journeys.
Choose Acme Auto Insurance and experience the assurance that comes with superior coverage and genuine care. Because at Acme, we don't just 
insure your car—we fuel your adventures on the open road.

Note: We also offer specialized coverage for electric vehicles, ensuring that drivers of all car types can benefit from our protection.

Acme Insurance offers the following products:
- Car insurance
- Electric car insurance
- Two-wheeler insurance

Business hours: Monday-Friday, 9 AM - 5 PM EST
Customer service number: 1-800-123-4567
</static_context>
"""
```

然後我們對汽車保險和電動車保險資訊執行相同操作。

```python
STATIC_CAR_INSURANCE="""
<static_context>
Car Insurance Coverage:
Acme's car insurance policies typically cover:
1. Liability coverage: Pays for bodily injury and property damage you cause to others.
2. Collision coverage: Pays for damage to your car in an accident.
3. Comprehensive coverage: Pays for damage to your car from non-collision incidents.
4. Medical payments coverage: Pays for medical expenses after an accident.
5. Uninsured/underinsured motorist coverage: Protects you if you're hit by a driver with insufficient insurance.

Optional coverages include:
- Rental car reimbursement
- Roadside assistance
- New car replacement
</static_context>
"""

STATIC_ELECTRIC_CAR_INSURANCE="""
<static_context>
Electric Car Insurance:
Our specialized electric car insurance goes beyond traditional auto coverage, 
offering tailored protection for your high-tech investment. In addition to standard coverages, we offer:
- Safeguarding expensive battery systems
- Covering home charging equipment
- Adapting to the rapidly evolving world of electric vehicles

Whether you're driving a Rivian, Nissan Leaf, Chevrolet Bolt, 
or any other electric vehicle, our knowledgeable agents are ready to craft a policy 
that gives you the confidence to enjoy every electron-powered mile.
</static_context>
"""
```

現在我們有了靜態內容，讓我們添加至少 4-5 個示例"好"互動來指導 Claude 的回應。這些示例應該代表您的理想客戶互動，並可以包括護欄、工具調用等。

```python
EXAMPLES="""
Here are a few examples of how you can interact with customers:

<example 1>
H: Hi, do you offer commercial insurance for small businesses?

A: Ah! Unfortunately, we don't offer commercial insurance at this time. 
However, we do provide a range of personal insurance products including car insurance and electric car insurance. 
Would you like to know more about these offerings?
</example 1>

<example 2>
H: Hi, what are your business hours?

A: Our business hours are Monday to Friday, 9 AM to 5 PM EST. 
If you have any questions or need assistance, feel free to reach out during those times. 
How else can I assist you today?
</example 2>

<example 3>
H: I'm shopping for car insurance. What does Acme offer?

A: Great choice! Acme's car insurance policies typically include:
- Liability coverage: Protects you if you're at fault in an accident
- Collision coverage: Pays for damage to your car in an accident
- Comprehensive coverage: Covers non-collision incidents like theft or vandalism
- Medical payments coverage: Pays for medical expenses after an accident
- Uninsured/underinsured motorist coverage: Protects you from drivers with insufficient insurance

We also offer optional coverages like rental car reimbursement, roadside assistance, and new car replacement. 
Would you like more details on any of these coverages, or do you have specific questions about insuring your vehicle?
</example 3>

<example 4>
H: I'd like to hear about your electric car insurance.

A: Certainly! Our electric car insurance is specially designed to meet the unique needs of electric vehicle owners. 
In addition to all the standard coverages we offer for regular cars, our electric car policies include:

- Battery coverage: Protection against damage or degradation of your vehicle's expensive battery system
- Tech package: Covers repairs or replacement of advanced electronics and onboard computer systems
- Charging station coverage: Protects both home and portable charging equipment
- Cybersecurity protection: Coverage against hacking or digital threats
- Specialized roadside assistance: Includes flatbed towing to protect the vehicle's undercarriage

We cover a wide range of electric vehicles, including popular models from Tesla, Rivian, Nissan, Chevrolet, and more. 
Is there a specific electric vehicle you're looking to insure, or would you like more details on any of these coverages?
</example 4>

<example 5>
H: I'd like to get a quote for my car insurance.

A: Certainly! I'd be happy to help you get a quote for your car insurance. 
To provide you with an accurate quote, I'll need to collect some information about your vehicle and the primary driver. 
Let's start with the basics:

1. What is the make and model of your vehicle?
2. What year was it manufactured?
3. Approximately how many miles have you driven?
4. What is the age of the primary driver?

Once you provide this information, I'll use our quoting tool to generate a personalized insurance quote for you.
</example 5>
"""
```

您還需要包括任何重要的指令，概述 Claude 應如何與客戶互動的做法和禁止做法。
這可能源自品牌護欄或支援政策。

```python
ADDITIONAL_GUARDRAILS = """Please adhere to the following guardrails:
1. Only provide information about insurance types listed in our offerings.
2. If asked about an insurance type we don't offer, politely state 
that we don't provide that service.
3. Do not speculate about future product offerings or company plans.
4. Don't make promises or enter into agreements it's not authorized to make.
You only provide information and guidance.
5. Do not mention any competitor's products or services.
"""
```

現在讓我們將所有這些部分組合成一個字符串以用作我們的提示。

```python
TASK_SPECIFIC_INSTRUCTIONS = ' '.join([
   STATIC_GREETINGS_AND_GENERAL,
   STATIC_CAR_INSURANCE,
   STATIC_ELECTRIC_CAR_INSURANCE,
   EXAMPLES,
   ADDITIONAL_GUARDRAILS,
])
```

### 使用工具使用添加動態和代理功能

Claude 能夠使用客戶端工具使用功能動態採取行動和檢索資訊。首先列出提示應使用的任何外部工具或 API。

對於此示例，我們將從一個用於計算報價的工具開始。

<Tip>作為提醒，此工具不會執行實際計算，它只會向應用程式發出信號，表示應使用具有指定的任何參數的工具。</Tip>

示例保險報價計算器：

```python
TOOLS = [{
  "name": "get_quote",
  "description": "Calculate the insurance quote based on user input. Returned value is per month premium.",
  "input_schema": {
    "type": "object",
    "properties": {
      "make": {"type": "string", "description": "The make of the vehicle."},
      "model": {"type": "string", "description": "The model of the vehicle."},
      "year": {"type": "integer", "description": "The year the vehicle was manufactured."},
      "mileage": {"type": "integer", "description": "The mileage on the vehicle."},
      "driver_age": {"type": "integer", "description": "The age of the primary driver."}
    },
    "required": ["make", "model", "year", "mileage", "driver_age"]
  }
}]

def get_quote(make, model, year, mileage, driver_age):
    """Returns the premium per month in USD"""
    # You can call an http endpoint or a database to get the quote.
    # Here, we simulate a delay of 1 seconds and return a fixed quote of 100.
    time.sleep(1)
    return 100
```

### 部署您的提示

在測試生產設置中部署提示並 [運行評估](/docs/zh-TW/test-and-evaluate/develop-tests) 是很難知道您的提示效果如何，所以讓我們使用我們的提示、Anthropic SDK 和 streamlit 構建一個小應用程式以獲得用戶界面。

在名為 `chatbot.py` 的文件中，首先設置 ChatBot 類，該類將封裝與 Anthropic SDK 的互動。

該類應有兩個主要方法：`generate_message` 和 `process_user_input`。

```python
from anthropic import Anthropic
from config import IDENTITY, TOOLS, MODEL, get_quote
from dotenv import load_dotenv

load_dotenv()

class ChatBot:
   def __init__(self, session_state):
       self.anthropic = Anthropic()
       self.session_state = session_state

   def generate_message(
       self,
       messages,
       max_tokens,
   ):
       try:
           response = self.anthropic.messages.create(
               model=MODEL,
               system=IDENTITY,
               max_tokens=max_tokens,
               messages=messages,
               tools=TOOLS,
           )
           return response
       except Exception as e:
           return {"error": str(e)}

   def process_user_input(self, user_input):
       self.session_state.messages.append({"role": "user", "content": user_input})

       response_message = self.generate_message(
           messages=self.session_state.messages,
           max_tokens=2048,
       )

       if "error" in response_message:
           return f"An error occurred: {response_message['error']}"

       if response_message.content[-1].type == "tool_use":
           tool_use = response_message.content[-1]
           func_name = tool_use.name
           func_params = tool_use.input
           tool_use_id = tool_use.id

           result = self.handle_tool_use(func_name, func_params)
           self.session_state.messages.append(
               {"role": "assistant", "content": response_message.content}
           )
           self.session_state.messages.append({
               "role": "user",
               "content": [{
                   "type": "tool_result",
                   "tool_use_id": tool_use_id,
                   "content": f"{result}",
               }],
           })

           follow_up_response = self.generate_message(
               messages=self.session_state.messages,
               max_tokens=2048,
           )

           if "error" in follow_up_response:
               return f"An error occurred: {follow_up_response['error']}"

           response_text = follow_up_response.content[0].text
           self.session_state.messages.append(
               {"role": "assistant", "content": response_text}
           )
           return response_text
      
       elif response_message.content[0].type == "text":
           response_text = response_message.content[0].text
           self.session_state.messages.append(
               {"role": "assistant", "content": response_text}
           )
           return response_text
      
       else:
           raise Exception("An error occurred: Unexpected response type")

   def handle_tool_use(self, func_name, func_params):
       if func_name == "get_quote":
           premium = get_quote(**func_params)
           return f"Quote generated: ${premium:.2f} per month"
      
       raise Exception("An unexpected tool was used")
```

### 構建您的用戶界面

使用主方法測試使用 Streamlit 部署此代碼。此 `main()` 函數設置基於 Streamlit 的聊天界面。

我們將在名為 `app.py` 的文件中執行此操作

```python
import streamlit as st
from chatbot import ChatBot
from config import TASK_SPECIFIC_INSTRUCTIONS

def main():
   st.title("Chat with Eva, Acme Insurance Company's Assistant🤖")

   if "messages" not in st.session_state:
       st.session_state.messages = [
           {'role': "user", "content": TASK_SPECIFIC_INSTRUCTIONS},
           {'role': "assistant", "content": "Understood"},
       ]

   chatbot = ChatBot(st.session_state)

   # Display user and assistant messages skipping the first two
   for message in st.session_state.messages[2:]:
       # ignore tool use blocks
       if isinstance(message["content"], str):
           with st.chat_message(message["role"]):
               st.markdown(message["content"])

   if user_msg := st.chat_input("Type your message here..."):
       st.chat_message("user").markdown(user_msg)

       with st.chat_message("assistant"):
           with st.spinner("Eva is thinking..."):
               response_placeholder = st.empty()
               full_response = chatbot.process_user_input(user_msg)
               response_placeholder.markdown(full_response)

if __name__ == "__main__":
   main()
```

使用以下命令運行程序：

```
streamlit run app.py
```

### 評估您的提示

提示通常需要測試和優化才能準備好投入生產。要確定解決方案的就緒情況，請使用結合定量和定性方法的系統流程評估聊天機器人性能。根據您定義的成功標準創建 [強大的經驗評估](/docs/zh-TW/test-and-evaluate/develop-tests#building-evals-and-test-cases) 將允許您優化提示。

<Tip>[Claude 控制台](/dashboard) 現在具有評估工具，允許您在各種場景下測試提示。</Tip>

### 改進性能

在複雜場景中，除了標準 [提示工程技術](/docs/zh-TW/build-with-claude/prompt-engineering/overview) 和 [護欄實現策略](/docs/zh-TW/test-and-evaluate/strengthen-guardrails/reduce-hallucinations) 之外，考慮其他策略來改進性能可能會有所幫助。以下是一些常見場景：

#### 使用 RAG 減少長上下文延遲

在處理大量靜態和動態上下文時，在提示中包含所有資訊可能導致高成本、更慢的回應時間和達到上下文窗口限制。在這種情況下，實現檢索增強生成 (RAG) 技術可以顯著改進性能和效率。

通過使用 [Voyage 等嵌入模型](/docs/zh-TW/build-with-claude/embeddings) 將資訊轉換為向量表示，您可以創建更可擴展和響應式的系統。此方法允許根據當前查詢動態檢索相關資訊，而不是在每個提示中包含所有可能的上下文。

為支援用例實現 RAG [RAG 配方](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb) 已被證明可以提高準確度、減少回應時間，並在具有廣泛上下文要求的系統中降低 API 成本。

#### 使用工具使用整合實時數據

在處理需要實時資訊的查詢時，例如帳戶餘額或政策詳細資訊，基於嵌入的 RAG 方法是不夠的。相反，您可以利用工具使用來顯著增強聊天機器人提供準確、實時回應的能力。例如，您可以使用工具使用來查找客戶資訊、檢索訂單詳細資訊，以及代表客戶取消訂單。

此方法 [在我們的工具使用：客戶服務代理配方中概述](https://github.com/anthropics/anthropic-cookbook/blob/main/tool_use/customer_service_agent.ipynb)，允許您無縫地將實時數據整合到 Claude 的回應中，並提供更個性化和高效的客戶體驗。

#### 加強輸入和輸出護欄

在部署聊天機器人時，特別是在客戶服務場景中，防止與濫用、超出範圍查詢和不適當回應相關的風險至關重要。雖然 Claude 對此類場景具有內在的彈性，但以下是加強聊天機器人護欄的其他步驟：

- [減少幻覺](/docs/zh-TW/test-and-evaluate/strengthen-guardrails/reduce-hallucinations)：實現事實檢查機制和 [引用](https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb) 以將回應基於提供的資訊。
- 交叉檢查資訊：驗證代理的回應是否符合您公司的政策和已知事實。
- 避免合同承諾：確保代理不會做出承諾或進入它無權進行的協議。
- [減輕越獄](/docs/zh-TW/test-and-evaluate/strengthen-guardrails/mitigate-jailbreaks)：使用無害性篩選和輸入驗證等方法來防止用戶利用模型漏洞，旨在生成不適當的內容。
- 避免提及競爭對手：實現競爭對手提及過濾器以維護品牌焦點，不提及任何競爭對手的產品或服務。
- [保持 Claude 的角色](/docs/zh-TW/test-and-evaluate/strengthen-guardrails/keep-claude-in-character)：防止 Claude 改變其風格上下文，即使在長期、複雜的互動中。
- 移除個人可識別資訊 (PII)：除非明確要求和授權，否則從回應中移除任何 PII。

#### 使用流式傳輸減少感知回應時間

在處理可能冗長的回應時，實現流式傳輸可以顯著改進用戶參與度和滿意度。在這種情況下，用戶逐步接收答案，而不是等待整個回應生成。

以下是如何實現流式傳輸的方法：
1. 使用 [Anthropic 流式傳輸 API](/docs/zh-TW/build-with-claude/streaming) 來支援流式傳輸回應。
2. 設置您的前端以處理傳入的文本塊。
3. 在每個塊到達時顯示它，模擬實時打字。
4. 實現一種機制來保存完整回應，允許用戶在導航離開並返回時查看它。

在某些情況下，流式傳輸使得能夠使用具有更高基本延遲的更高級模型，因為漸進式顯示減輕了更長處理時間的影響。

#### 擴展您的聊天機器人

隨著聊天機器人複雜性的增長，您的應用程式架構可以相應地發展。在向架構添加進一步層之前，請考慮以下不太詳盡的選項：

- 確保您充分利用提示並通過提示工程進行優化。使用我們的 [提示工程指南](/docs/zh-TW/build-with-claude/prompt-engineering/overview) 來編寫最有效的提示。
- 向提示添加其他 [工具](/docs/zh-TW/build-with-claude/tool-use)（可以包括 [提示鏈](/docs/zh-TW/build-with-claude/prompt-engineering/chain-prompts)），看看您是否可以實現所需的功能。

如果您的聊天機器人處理非常多樣化的任務，您可能想考慮添加 [單獨的意圖分類器](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/classification/guide.ipynb) 來路由初始客戶查詢。對於現有應用程式，這將涉及創建一個決策樹，該決策樹將通過分類器路由客戶查詢，然後路由到專門的對話（具有自己的工具和系統提示集）。請注意，此方法需要額外的 Claude 調用，可能會增加延遲。

### 將 Claude 整合到您的支援工作流中

雖然我們的示例側重於在 Streamlit 環境中可調用的 Python 函數，但部署 Claude 用於實時支援聊天機器人需要 API 服務。

以下是您可以如何處理此問題：

1. 創建 API 包裝器：圍繞您的分類函數開發一個簡單的 API 包裝器。例如，您可以使用 Flask API 或 Fast API 將代碼包裝到 HTTP 服務中。您的 HTTP 服務可以接受用戶輸入並完整返回助手回應。因此，您的服務可能具有以下特徵：
   - 服務器發送事件 (SSE)：SSE 允許從服務器到客戶端的實時回應流式傳輸。這對於在使用 LLM 時提供平滑、互動式體驗至關重要。
   - 緩存：實現緩存可以顯著改進回應時間並減少不必要的 API 調用。
   - 上下文保留：當用戶導航離開並返回時維護上下文對於對話的連續性很重要。

2. 構建 Web 界面：為與 Claude 驅動的代理互動實現用戶友好的 Web UI。

<CardGroup cols={2}>
  <Card title="檢索增強生成 (RAG) 食譜" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/skills/retrieval_augmented_generation/guide.ipynb">
    訪問我們的 RAG 食譜配方以獲取更多示例代碼和詳細指導。
  </Card>
  <Card title="引用食譜" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb">
    探索我們的引用食譜配方以了解如何確保資訊的準確性和可解釋性。
  </Card>
</CardGroup>