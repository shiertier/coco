# 客户支持代理

本指南介绍如何利用 Claude 的高级对话能力实时处理客户询问，提供 24/7 支持，减少等待时间，并通过准确的回复和积极的互动来管理大量支持工作。

---

## 使用 Claude 之前的准备

### 决定是否使用 Claude 进行支持聊天

以下是一些关键指标，表明您应该采用 Claude 等大语言模型来自动化客户支持流程的部分工作：

  <section title="大量重复性查询">

    Claude 擅长高效处理大量类似问题，为人工代理腾出时间处理更复杂的问题。
  
</section>
  <section title="需要快速信息综合">

    Claude 可以快速检索、处理和整合来自庞大知识库的信息，而人工代理可能需要时间进行研究或查阅多个来源。
  
</section>
  <section title="24/7 可用性要求">

    Claude 可以提供全天候支持而不会疲劳，而为持续覆盖配备人工代理可能成本高昂且具有挑战性。
  
</section>
  <section title="高峰期快速扩展">

    Claude 可以处理查询量的突然增加，无需招聘和培训额外员工。
  
</section>
  <section title="一致的品牌声音">

    您可以指示 Claude 始终代表您品牌的语调和价值观，而人工代理的沟通风格可能会有所不同。
  
</section>

选择 Claude 而不是其他大语言模型的一些考虑因素：

- 您优先考虑自然、细致的对话：Claude 的复杂语言理解能力允许进行更自然、上下文感知的对话，感觉比与其他大语言模型的聊天更像人类。
- 您经常收到复杂和开放式的查询：Claude 可以处理广泛的主题和询问，而无需生成模板化回复或需要对用户表述的排列进行大量编程。
- 您需要可扩展的多语言支持：Claude 的多语言能力允许它用 200 多种语言进行对话，无需为每种支持的语言单独配置聊天机器人或进行大量翻译流程。

### 定义理想的聊天互动

概述理想的客户互动，以定义您期望客户如何以及何时与 Claude 互动。这个概述将帮助确定您解决方案的技术要求。

以下是汽车保险客户支持的示例聊天互动：

* **客户**：启动支持聊天体验
   * **Claude**：热情地问候客户并启动对话
* **客户**：询问关于其新电动汽车的保险
   * **Claude**：提供关于电动车覆盖的相关信息
* **客户**：提出与电动车保险独特需求相关的问题
   * **Claude**：用准确和信息丰富的答案回复，并提供信息来源的链接
* **客户**：提出与保险或汽车无关的题外话
   * **Claude**：澄清它不讨论无关主题，并将用户引导回汽车保险
* **客户**：表示对保险报价感兴趣
   * **Claude**：提出一系列问题以确定适当的报价，根据他们的回复进行调整
   * **Claude**：发送使用报价生成 API 工具的请求，以及从用户收集的必要信息
   * **Claude**：接收来自 API 工具使用的响应信息，将信息综合为自然回复，并向用户呈现提供的报价
* **客户**：提出后续问题
   * **Claude**：根据需要回答后续问题
   * **Claude**：指导客户进行保险流程的后续步骤并结束对话

<Tip>在您为自己的用例编写的真实示例中，您可能会发现将此互动中的实际措辞写出来很有用，这样您也可以了解 Claude 应该具有的理想语调、回复长度和详细程度。</Tip>

### 将互动分解为独特的任务

客户支持聊天是多个不同任务的集合，从问题回答到信息检索再到对请求采取行动，全部包装在单个客户互动中。在开始构建之前，将您的理想客户互动分解为您希望 Claude 能够执行的每项任务。这确保您可以为每项任务提示和评估 Claude，并让您很好地了解编写测试用例时需要考虑的互动范围。

<Tip>客户有时会发现将其可视化为可能的对话拐点的互动流程图很有帮助，具体取决于用户请求。</Tip>

以下是与上面的示例保险互动相关的关键任务：

1. 问候和一般指导
   - 热情地问候客户并启动对话
   - 提供关于公司和互动的一般信息

2. 产品信息
   - 提供关于电动车覆盖的信息
   <Note>这将要求 Claude 在其上下文中拥有必要的信息，并可能意味着需要 [RAG 集成](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/retrieval_augmented_generation/guide.ipynb)。</Note>
   - 回答与独特电动车保险需求相关的问题
   - 回答关于报价或保险详情的后续问题
   - 在适当时提供信息来源的链接

3. 对话管理
   - 保持主题（汽车保险）
   - 将题外话重定向回相关主题

4. 报价生成
   - 提出适当的问题以确定报价资格
   - 根据客户回复调整问题
   - 将收集的信息提交到报价生成 API
   - 向客户呈现提供的报价

### 建立成功标准

与您的支持团队合作 [定义明确的成功标准](/docs/zh-CN/test-and-evaluate/define-success) 并编写 [详细的评估](/docs/zh-CN/test-and-evaluate/develop-tests) 以及可衡量的基准和目标。

以下是可用于评估 Claude 成功执行定义任务的标准和基准：

  <section title="查询理解准确性">

    此指标评估 Claude 在各种主题中准确理解客户询问的程度。通过审查对话样本并评估 Claude 是否正确理解了客户意图、关键后续步骤、成功解决方案的样子等来衡量。目标是达到 95% 或更高的理解准确性。
  
</section>
  <section title="回复相关性">

    这评估 Claude 的回复在多大程度上解决了客户的具体问题或问题。评估一组对话并评估每个回复的相关性（使用基于大语言模型的评分以实现规模化）。目标是达到 90% 或更高的相关性得分。
  
</section>
  <section title="回复准确性">

    根据提供给 Claude 的上下文中的信息，评估提供给用户的一般公司和产品信息的正确性。在此介绍性信息中目标是达到 100% 的准确性。
  
</section>
  <section title="引用提供相关性">

    跟踪提供的链接或来源的频率和相关性。目标是在 80% 的互动中提供相关来源，其中可能需要额外信息。
  
</section>
  <section title="主题坚持">

    衡量 Claude 在主题上的坚持程度，例如我们示例实现中的汽车保险主题。目标是 95% 的回复与汽车保险或客户的具体查询直接相关。
  
</section>
  <section title="内容生成有效性">

    衡量 Claude 在确定何时生成信息内容以及该内容的相关性方面的成功程度。例如，在我们的实现中，我们将确定 Claude 在多大程度上理解何时生成报价以及该报价的准确性。目标是达到 100% 的准确性，因为这对成功的客户互动至关重要。
  
</section>
  <section title="升级效率">

    这衡量 Claude 识别何时查询需要人工干预并适当升级的能力。跟踪正确升级的对话百分比与应该升级但未升级的对话。目标是达到 95% 或更高的升级准确性。
  
</section>

以下是可用于评估采用 Claude 进行支持的业务影响的标准和基准：

  <section title="情感维持">

    这评估 Claude 在整个对话中维持或改善客户情感的能力。使用情感分析工具衡量每次对话开始和结束时的情感。目标是在 90% 的互动中维持或改善情感。
  
</section>
  <section title="偏转率">

    聊天机器人成功处理的客户询问百分比，无需人工干预。通常目标是 70-80% 的偏转率，具体取决于询问的复杂性。
  
</section>
  <section title="客户满意度得分">

    衡量客户对其聊天机器人互动满意度的指标。通常通过互动后调查完成。目标是达到 5 分中的 4 分或更高的 CSAT 得分。
  
</section>
  <section title="平均处理时间">

    聊天机器人解决询问所需的平均时间。这因问题的复杂性而异，但通常目标是相比人工代理的平均处理时间更低。
  
</section>

## 如何将 Claude 实现为客户服务代理

### 选择合适的 Claude 模型

模型的选择取决于成本、准确性和响应时间之间的权衡。

对于客户支持聊天，Claude Sonnet 4.5 非常适合平衡智能、延迟和成本。但是，对于具有多个提示的对话流程（包括 RAG、工具使用和/或长上下文提示）的情况，Claude Haiku 4.5 可能更适合优化延迟。

### 构建强大的提示

使用 Claude 进行客户支持需要 Claude 有足够的指导和上下文来适当地响应，同时具有足够的灵活性来处理广泛的客户询问。

让我们从编写强大提示的元素开始，从系统提示开始：

```python
IDENTITY = """You are Eva, a friendly and knowledgeable AI assistant for Acme Insurance 
Company. Your role is to warmly welcome customers and provide information on 
Acme's insurance offerings, which include car insurance and electric car 
insurance. You can also help customers get quotes for their insurance needs."""
```

<Tip>虽然您可能想将所有信息放在系统提示中作为将指令与用户对话分开的方式，但 Claude 实际上在将大部分提示内容写在第一个 `User` 轮次中时效果最好（唯一的例外是角色提示）。在 [使用系统提示给 Claude 一个角色](/docs/zh-CN/build-with-claude/prompt-engineering/system-prompts) 中了解更多。</Tip>

最好将复杂提示分解为小节，并一次编写一部分。对于每项任务，您可能会通过遵循逐步流程来定义 Claude 完成任务所需的提示部分而获得更大的成功。对于此汽车保险客户支持示例，我们将逐步编写从"问候和一般指导"任务开始的提示的所有部分。这也使调试您的提示更容易，因为您可以更快地调整总体提示的各个部分。

我们将把所有这些部分放在一个名为 `config.py` 的文件中。

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

然后我们对汽车保险和电动车保险信息做同样的处理。

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

现在我们有了静态内容，让我们添加至少 4-5 个示例"好的"互动来指导 Claude 的回复。这些示例应该代表您的理想客户互动，并可以包括护栏、工具调用等。

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

您还需要包括任何重要的指令，概述 Claude 应如何与客户互动的做法和禁止事项。这可能来自品牌护栏或支持政策。

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

现在让我们将所有这些部分组合成一个字符串以用作我们的提示。

```python
TASK_SPECIFIC_INSTRUCTIONS = ' '.join([
   STATIC_GREETINGS_AND_GENERAL,
   STATIC_CAR_INSURANCE,
   STATIC_ELECTRIC_CAR_INSURANCE,
   EXAMPLES,
   ADDITIONAL_GUARDRAILS,
])
```

### 使用工具使用添加动态和代理功能

Claude 能够使用客户端工具使用功能来采取行动和动态检索信息。首先列出提示应该使用的任何外部工具或 API。

对于此示例，我们将从一个用于计算报价的工具开始。

<Tip>作为提醒，此工具不会执行实际计算，它只会向应用程序发出信号，表明应该使用具有指定参数的工具。</Tip>

示例保险报价计算器：

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

在测试生产设置中部署提示并 [运行评估](/docs/zh-CN/test-and-evaluate/develop-tests) 来了解您的提示效果如何是很困难的，所以让我们使用我们的提示、Anthropic SDK 和 streamlit 用户界面构建一个小应用程序。

在名为 `chatbot.py` 的文件中，首先设置 ChatBot 类，它将封装与 Anthropic SDK 的交互。

该类应该有两个主要方法：`generate_message` 和 `process_user_input`。

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

### 构建您的用户界面

使用主方法通过 Streamlit 测试部署此代码。此 `main()` 函数设置基于 Streamlit 的聊天界面。

我们将在名为 `app.py` 的文件中执行此操作

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

运行程序：

```
streamlit run app.py
```

### 评估您的提示

提示通常需要测试和优化才能做好生产准备。要确定解决方案的就绪情况，请使用结合定量和定性方法的系统流程来评估聊天机器人性能。基于您定义的成功标准创建 [强大的经验评估](/docs/zh-CN/test-and-evaluate/develop-tests#building-evals-and-test-cases) 将允许您优化您的提示。

<Tip>[Claude 控制台](/dashboard) 现在具有评估工具，允许您在各种场景下测试您的提示。</Tip>

### 改进性能

在复杂场景中，除了标准 [提示工程技术](/docs/zh-CN/build-with-claude/prompt-engineering/overview) 和 [护栏实现策略](/docs/zh-CN/test-and-evaluate/strengthen-guardrails/reduce-hallucinations) 之外，考虑其他策略来改进性能可能会有帮助。以下是一些常见场景：

#### 使用 RAG 减少长上下文延迟

处理大量静态和动态上下文时，在提示中包含所有信息可能导致高成本、响应时间变慢和达到上下文窗口限制。在这种情况下，实现检索增强生成 (RAG) 技术可以显著改进性能和效率。

通过使用 [Voyage 等嵌入模型](/docs/zh-CN/build-with-claude/embeddings) 将信息转换为向量表示，您可以创建更可扩展和响应更快的系统。这种方法允许根据当前查询动态检索相关信息，而不是在每个提示中包含所有可能的上下文。

为支持用例实现 RAG [RAG 配方](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb) 已被证明可以增加准确性、减少响应时间并降低具有广泛上下文要求的系统中的 API 成本。

#### 使用工具使用集成实时数据

处理需要实时信息的查询（如账户余额或政策详情）时，基于嵌入的 RAG 方法是不够的。相反，您可以利用工具使用来显著增强聊天机器人提供准确、实时响应的能力。例如，您可以使用工具使用来查找客户信息、检索订单详情和代表客户取消订单。

这种方法 [在我们的工具使用：客户服务代理配方中概述](https://github.com/anthropics/anthropic-cookbook/blob/main/tool_use/customer_service_agent.ipynb)，允许您无缝地将实时数据集成到 Claude 的响应中，并提供更个性化和高效的客户体验。

#### 加强输入和输出护栏

部署聊天机器人时，特别是在客户服务场景中，防止与滥用、超出范围的查询和不适当的回复相关的风险至关重要。虽然 Claude 对此类场景具有固有的弹性，但以下是加强聊天机器人护栏的其他步骤：

- [减少幻觉](/docs/zh-CN/test-and-evaluate/strengthen-guardrails/reduce-hallucinations)：实现事实检查机制和 [引用](https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb) 以将响应基于提供的信息。
- 交叉检查信息：验证代理的响应与您公司的政策和已知事实一致。
- 避免合同承诺：确保代理不会做出承诺或进入它无权做出的协议。
- [缓解越狱](/docs/zh-CN/test-and-evaluate/strengthen-guardrails/mitigate-jailbreaks)：使用无害性屏幕和输入验证等方法来防止用户利用模型漏洞生成不适当的内容。
- 避免提及竞争对手：实现竞争对手提及过滤器以维持品牌焦点，不提及任何竞争对手的产品或服务。
- [保持 Claude 的角色](/docs/zh-CN/test-and-evaluate/strengthen-guardrails/keep-claude-in-character)：防止 Claude 改变其风格或上下文，即使在长期、复杂的互动中。
- 删除个人身份信息 (PII)：除非明确要求和授权，否则从响应中删除任何 PII。

#### 使用流式传输减少感知响应时间

处理可能冗长的响应时，实现流式传输可以显著改进用户参与度和满意度。在这种情况下，用户逐步接收答案，而不是等待整个响应生成。

以下是如何实现流式传输的方法：
1. 使用 [Anthropic 流式传输 API](/docs/zh-CN/build-with-claude/streaming) 来支持流式传输响应。
2. 设置您的前端以处理传入的文本块。
3. 在每个块到达时显示它，模拟实时输入。
4. 实现一种机制来保存完整响应，允许用户在导航离开并返回时查看它。

在某些情况下，流式传输使得能够使用具有更高基础延迟的更高级模型，因为渐进式显示减轻了更长处理时间的影响。

#### 扩展您的聊天机器人

随着聊天机器人复杂性的增长，您的应用程序架构可以相应地发展。在向架构添加进一步层之前，请考虑以下不太详尽的选项：

- 确保您充分利用您的提示并通过提示工程进行优化。使用我们的 [提示工程指南](/docs/zh-CN/build-with-claude/prompt-engineering/overview) 来编写最有效的提示。
- 向提示添加其他 [工具](/docs/zh-CN/build-with-claude/tool-use)（可以包括 [提示链](/docs/zh-CN/build-with-claude/prompt-engineering/chain-prompts)），看看您是否可以实现所需的功能。

如果您的聊天机器人处理极其多样化的任务，您可能想考虑添加 [单独的意图分类器](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/classification/guide.ipynb) 来路由初始客户查询。对于现有应用程序，这将涉及创建一个决策树，该决策树将通过分类器路由客户查询，然后路由到专门的对话（具有自己的工具集和系统提示）。请注意，此方法需要额外的 Claude 调用，可能会增加延迟。

### 将 Claude 集成到您的支持工作流中

虽然我们的示例侧重于在 Streamlit 环境中可调用的 Python 函数，但部署 Claude 进行实时支持聊天机器人需要 API 服务。

以下是您可以如何处理这个问题：

1. 创建 API 包装器：围绕您的分类函数开发一个简单的 API 包装器。例如，您可以使用 Flask API 或 Fast API 将您的代码包装成 HTTP 服务。您的 HTTP 服务可以接受用户输入并完整返回助手响应。因此，您的服务可能具有以下特征：
   - 服务器发送事件 (SSE)：SSE 允许从服务器到客户端的响应实时流式传输。这对于在使用大语言模型时提供平滑、交互式体验至关重要。
   - 缓存：实现缓存可以显著改进响应时间并减少不必要的 API 调用。
   - 上下文保留：当用户导航离开并返回时维持上下文对于对话的连续性很重要。

2. 构建 Web 界面：为与 Claude 驱动的代理交互实现用户友好的 Web UI。

<CardGroup cols={2}>
  <Card title="检索增强生成 (RAG) 食谱" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/skills/retrieval_augmented_generation/guide.ipynb">
    访问我们的 RAG 食谱配方以获取更多示例代码和详细指导。
  </Card>
  <Card title="引用食谱" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb">
    探索我们的引用食谱配方以了解如何确保信息的准确性和可解释性。
  </Card>
</CardGroup>