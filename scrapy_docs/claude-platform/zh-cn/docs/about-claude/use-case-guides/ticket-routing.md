# 工单路由

本指南介绍如何利用 Claude 的先进自然语言理解能力，根据客户意图、紧急程度、优先级、客户档案等因素，大规模对客户支持工单进行分类。

---

## 确定是否应该使用 Claude 进行工单路由

以下是一些关键指标，表明您应该使用 Claude 等大语言模型而不是传统机器学习方法来完成分类任务：

    <section title="您拥有的标注训练数据有限">

        传统机器学习流程需要大量标注数据集。Claude 的预训练模型只需几十个标注示例就能有效地对工单进行分类，大大减少了数据准备时间和成本。
    
</section>
    <section title="您的分类类别可能会随时间变化或演变">

        一旦建立了传统机器学习方法，改变它就是一项繁琐且数据密集的工作。另一方面，随着您的产品或客户需求的演变，Claude 可以轻松适应类别定义的变化或新类别，而无需对训练数据进行大量重新标注。
    
</section>
    <section title="您需要处理复杂的非结构化文本输入">

        传统机器学习模型通常在处理非结构化数据时表现不佳，需要大量特征工程。Claude 的先进语言理解能力允许基于内容和上下文进行准确分类，而不是依赖严格的本体结构。
    
</section>
    <section title="您的分类规则基于语义理解">

        传统机器学习方法通常依赖词袋模型或简单的模式匹配。Claude 擅长理解和应用基础规则，当类别由条件而非示例定义时尤其如此。
    
</section>
    <section title="您需要为分类决策提供可解释的推理">

        许多传统机器学习模型对其决策过程提供的洞察很少。Claude 可以为其分类决策提供人类可读的解释，建立对自动化系统的信任，并在需要时便于轻松调整。
    
</section>
    <section title="您想更有效地处理边界情况和模糊工单">

        传统机器学习系统通常在处理异常值和模糊输入时表现不佳，经常将其误分类或默认为通用类别。Claude 的自然语言处理能力使其能够更好地解释支持工单中的上下文和细微差别，可能减少需要人工干预的路由错误或未分类工单的数量。
    
</section>
    <section title="您需要多语言支持而无需维护单独的模型">

        传统机器学习方法通常需要为每种支持的语言使用单独的模型或进行大量翻译流程。Claude 的多语言能力使其能够对各种语言的工单进行分类，无需单独的模型或大量翻译流程，简化了对全球客户群的支持。
    
</section>

***

## 构建和部署您的大语言模型支持工作流

### 了解您当前的支持方法
在深入自动化之前，了解您现有的工单系统至关重要。首先调查您的支持团队目前如何处理工单路由。

考虑以下问题：
* 使用什么标准来确定应用哪个 SLA/服务级别？
* 工单路由是否用于确定工单应该转到哪个支持级别或产品专家？
* 是否已有任何自动化规则或工作流？它们在什么情况下失败？
* 边界情况或模糊工单如何处理？
* 团队如何优先处理工单？

您对人类如何处理某些情况的了解越多，您就越能够与 Claude 合作完成任务。

### 定义用户意图类别
明确定义的用户意图类别列表对于使用 Claude 进行准确的支持工单分类至关重要。Claude 在您的系统中有效路由工单的能力与您系统类别的定义程度成正比。

以下是一些用户意图类别和子类别的示例。

    <section title="技术问题">

        * 硬件问题
        * 软件缺陷
        * 兼容性问题
        * 性能问题
    
</section>
    <section title="账户管理">

        * 密码重置
        * 账户访问问题
        * 账单查询
        * 订阅变更
    
</section>
    <section title="产品信息">

        * 功能查询
        * 产品兼容性问题
        * 定价信息
        * 可用性查询
    
</section>
    <section title="用户指导">

        * 操作方法问题
        * 功能使用协助
        * 最佳实践建议
        * 故障排除指导
    
</section>
    <section title="反馈">

        * 缺陷报告
        * 功能请求
        * 一般反馈或建议
        * 投诉
    
</section>
    <section title="订单相关">

        * 订单状态查询
        * 运输信息
        * 退货和换货
        * 订单修改
    
</section>
    <section title="服务请求">

        * 安装协助
        * 升级请求
        * 维护安排
        * 服务取消
    
</section>
    <section title="安全问题">

        * 数据隐私查询
        * 可疑活动报告
        * 安全功能协助
    
</section>
    <section title="合规和法律">

        * 监管合规问题
        * 服务条款查询
        * 法律文件请求
    
</section>
    <section title="紧急支持">

        * 关键系统故障
        * 紧急安全问题
        * 时间敏感问题
    
</section>
    <section title="培训和教育">

        * 产品培训请求
        * 文档查询
        * 网络研讨会或工作坊信息
    
</section>
    <section title="集成和 API">

        * 集成协助
        * API 使用问题
        * 第三方兼容性查询
    
</section>

除了意图外，工单路由和优先级可能还受到其他因素的影响，例如紧急程度、客户类型、SLA 或语言。在构建自动化路由系统时，请务必考虑其他路由标准。

### 建立成功标准

与您的支持团队合作[定义明确的成功标准](/docs/zh-CN/test-and-evaluate/define-success)，包括可衡量的基准、阈值和目标。

以下是使用大语言模型进行支持工单路由时的一些标准标准和基准：

    <section title="分类一致性">

        此指标评估 Claude 在一段时间内对相似工单的分类一致性。这对于维持路由可靠性至关重要。通过定期使用一组标准化输入测试模型来衡量，目标是达到 95% 或更高的一致性率。
    
</section>
    <section title="适应速度">

        这衡量 Claude 适应新类别或变化的工单模式的速度。通过引入新的工单类型并衡量模型在这些新类别上达到令人满意的准确度（例如 >90%）所需的时间来测试。目标是在 50-100 个样本工单内适应。
    
</section>
    <section title="多语言处理">

        这评估 Claude 准确路由多种语言工单的能力。衡量不同语言的路由准确度，目标是非主要语言的准确度下降不超过 5-10%。
    
</section>
    <section title="边界情况处理">

        这评估 Claude 在不寻常或复杂工单上的性能。创建一个边界情况测试集并衡量路由准确度，目标是在这些具有挑战性的输入上至少达到 80% 的准确度。
    
</section>
    <section title="偏差缓解">

        这衡量 Claude 在不同客户人口统计中路由的公平性。定期审计路由决策以发现潜在偏差，目标是在所有客户群体中保持一致的路由准确度（在 2-3% 以内）。
    
</section>
    <section title="提示效率">

        在最小化令牌计数至关重要的情况下，此标准评估 Claude 在最少上下文下的性能。衡量提供不同数量上下文时的路由准确度，目标是仅使用工单标题和简短描述时达到 90% 以上的准确度。
    
</section>
    <section title="可解释性评分">

        这评估 Claude 为其路由决策提供的解释的质量和相关性。人类评分者可以按比例（例如 1-5）对解释进行评分，目标是达到 4 或更高的平均评分。
    
</section>

以下是一些常见的成功标准，无论是否使用大语言模型都可能有用：

    <section title="路由准确度">

        路由准确度衡量工单被正确分配给适当团队或个人的频率。这通常以正确路由的工单数占总工单数的百分比来衡量。行业基准通常目标为 90-95% 的准确度，但这可能因支持结构的复杂性而异。
    
</section>
    <section title="分配时间">

        此指标跟踪工单提交后被分配的速度。更快的分配时间通常会导致更快的解决和更高的客户满意度。一流的系统通常可以实现平均分配时间不到 5 分钟，许多系统目标是近乎即时的路由（这在大语言模型实现中是可能的）。
    
</section>
    <section title="重新路由率">

        重新路由率表示初始路由后需要重新分配工单的频率。较低的比率表示更准确的初始路由。目标是重新路由率低于 10%，表现最好的系统可以达到 5% 或更低。
    
</section>
    <section title="首次接触解决率">

        这衡量在与客户的首次交互中解决的工单百分比。较高的比率表示高效的路由和准备充分的支持团队。行业基准通常在 70-75% 范围内，表现最好的公司可以达到 80% 或更高。
    
</section>
    <section title="平均处理时间">

        平均处理时间衡量从开始到完成解决工单所需的时间。高效的路由可以显著减少这个时间。基准因行业和复杂性而异，但许多组织的目标是将非关键问题的平均处理时间保持在 24 小时以内。
    
</section>
    <section title="客户满意度评分">

        通常通过交互后调查衡量，这些评分反映客户对支持流程的整体满意度。有效的路由有助于提高满意度。目标是 CSAT 评分达到 90% 或更高，表现最好的公司通常可以达到 95% 以上的满意度。
    
</section>
    <section title="升级率">

        这衡量工单需要升级到更高支持级别的频率。较低的升级率通常表示更准确的初始路由。目标是升级率低于 20%，一流的系统可以达到 10% 或更低。
    
</section>
    <section title="代理生产力">

        此指标查看实现路由解决方案后代理可以有效处理的工单数量。改进的路由应该会提高生产力。通过跟踪每个代理每天或每小时解决的工单数来衡量，目标是在实现新的路由系统后提高 10-20%。
    
</section>
    <section title="自助服务偏转率">

        这衡量在进入路由系统之前通过自助服务选项解决的潜在工单百分比。较高的比率表示有效的前期分类。目标是偏转率达到 20-30%，表现最好的公司可以达到 40% 或更高。
    
</section>
    <section title="每张工单成本">

        此指标计算解决每张支持工单的平均成本。高效的路由应该有助于随时间推移降低这个成本。虽然基准差异很大，但许多组织的目标是在实现改进的路由系统后将每张工单的成本降低 10-15%。
    
</section>

### 选择合适的 Claude 模型

模型的选择取决于成本、准确度和响应时间之间的权衡。

许多客户发现 `claude-haiku-4-5-20251001` 是工单路由的理想模型，因为它是 Claude 4 系列中最快、最具成本效益的模型，同时仍能提供出色的结果。如果您的分类问题需要深入的主题专业知识或大量的意图类别和复杂推理，您可以选择[更大的 Sonnet 模型](/docs/zh-CN/about-claude/models)。

### 构建强大的提示

工单路由是一种分类任务。Claude 分析支持工单的内容，并根据问题类型、紧急程度、所需专业知识或其他相关因素将其分类到预定义的类别中。

让我们编写一个工单分类提示。我们的初始提示应该包含用户请求的内容，并返回推理和意图。

<Tip>
尝试在 [Claude 控制台](/login)上使用[提示生成器](/docs/zh-CN/prompt-generator)让 Claude 为您编写初稿。
</Tip>

以下是一个工单路由分类提示的示例：
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

让我们分解这个提示的关键组件：
* 我们使用 Python f-strings 创建提示模板，允许将 `ticket_contents` 插入到 `<request>` 标签中。
* 我们给 Claude 一个明确定义的角色，作为一个分类系统，仔细分析工单内容以确定客户的核心意图和需求。
* 我们指示 Claude 正确的输出格式，在这种情况下，在 `<reasoning>` 标签内提供其推理和分析，然后在 `<intent>` 标签内提供适当的分类标签。
* 我们指定有效的意图类别："Support, Feedback, Complaint"、"Order Tracking" 和 "Refund/Exchange"。
* 我们包括几个示例（也称为少样本提示）来说明输出应该如何格式化，这提高了准确度和一致性。

我们希望让 Claude 将其响应分成各种 XML 标签部分的原因是，我们可以使用正则表达式从输出中分别提取推理和意图。这允许我们在工单路由工作流中创建有针对性的后续步骤，例如仅使用意图来决定将工单路由给谁。

### 部署您的提示

不在测试生产环境中部署提示并[运行评估](/docs/zh-CN/test-and-evaluate/develop-tests)，很难知道您的提示效果如何。

让我们构建部署结构。首先定义包装我们对 Claude 的调用的方法签名。我们将采用已经开始编写的方法，该方法以 `ticket_contents` 作为输入，现在返回 `reasoning` 和 `intent` 的元组作为输出。如果您有使用传统机器学习的现有自动化，您应该改为遵循该方法签名。

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

此代码：
* 导入 Anthropic 库并使用您的 API 密钥创建客户端实例。
* 定义一个 `classify_support_request` 函数，该函数接受 `ticket_contents` 字符串。
* 使用 `classification_prompt` 将 `ticket_contents` 发送给 Claude 进行分类
* 从响应中返回提取的模型的 `reasoning` 和 `intent`。

由于我们需要等待整个推理和意图文本生成后才能解析，我们设置 `stream=False`（默认值）。

***

## 评估您的提示

提示通常需要测试和优化才能投入生产。要确定您的解决方案的准备情况，请根据您之前建立的成功标准和阈值评估性能。

要运行您的评估，您需要测试用例来运行它。本指南的其余部分假设您已经[开发了测试用例](/docs/zh-CN/test-and-evaluate/develop-tests)。

### 构建评估函数

本指南的示例评估沿着三个关键指标衡量 Claude 的性能：
* 准确度
* 每次分类的成本

您可能需要根据对您重要的因素在其他方面评估 Claude。

为了评估这一点，我们首先必须修改我们编写的脚本，并添加一个函数来比较预测的意图与实际意图，并计算正确预测的百分比。我们还必须添加成本计算和时间测量功能。

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

让我们分解我们所做的编辑：
* 我们从测试用例中添加了 `actual_intent` 到 `classify_support_request` 方法中，并设置了一个比较来评估 Claude 的意图分类是否与我们的黄金意图分类相匹配。
* 我们提取了 API 调用的使用统计信息，以根据使用的输入和输出令牌计算成本

### 运行您的评估

适当的评估需要明确的阈值和基准来确定什么是好的结果。上面的脚本将为我们提供准确度、响应时间和每次分类成本的运行时值，但我们仍然需要明确建立的阈值。例如：
* **准确度：** 95%（100 个测试中）
* **每次分类的成本：** 从当前路由方法平均减少 50%（在 100 个测试中）

拥有这些阈值使您能够快速、轻松地大规模确定，以及以公正的经验主义方式，什么方法最适合您，以及可能需要做什么改变以更好地满足您的要求。

***

## 改进性能

在复杂的场景中，除了标准[提示工程技术](/docs/zh-CN/build-with-claude/prompt-engineering/overview)和[护栏实现策略](/docs/zh-CN/test-and-evaluate/strengthen-guardrails/reduce-hallucinations)之外，考虑其他策略来改进性能可能会有所帮助。以下是一些常见场景：

### 对于 20+ 个意图类别的情况，使用分类法层次结构

随着类别数量的增加，所需示例的数量也会增加，可能会使提示变得笨重。作为替代方案，您可以考虑使用分类器混合实现分层分类系统。
1. 在分类法树结构中组织您的意图。
2. 在树的每个级别创建一系列分类器，启用级联路由方法。

例如，您可能有一个顶级分类器，将工单广泛分类为"技术问题"、"账单问题"和"一般查询"。这些类别中的每一个都可以有自己的子分类器来进一步细化分类。

![](/docs/images/ticket-hierarchy.png)

* **优点 - 更大的细微差别和准确度：** 您可以为每个父路径创建不同的提示，允许更有针对性和特定于上下文的分类。这可以导致改进的准确度和更细致的客户请求处理。

* **缺点 - 增加的延迟：** 请注意，多个分类器可能会导致延迟增加，我们建议使用我们最快的模型 Haiku 来实现这种方法。

### 使用向量数据库和相似性搜索检索来处理高度可变的工单

尽管提供示例是改进性能的最有效方式，但如果支持请求高度可变，很难在单个提示中包含足够的示例。

在这种情况下，您可以使用向量数据库从示例数据集进行相似性搜索，并为给定查询检索最相关的示例。

这种方法在我们的[分类配方](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb)中有详细说明，已被证明可以将性能从 71% 的准确度提高到 93% 的准确度。

### 特别考虑预期的边界情况

以下是 Claude 可能误分类工单的一些场景（可能还有其他对您的情况独特的场景）。在这些场景中，考虑在提示中提供明确的说明或示例，说明 Claude 应该如何处理边界情况：

    <section title="客户提出隐含请求">

        客户经常间接表达需求。例如，"我的包已经等了两周多了"可能是对订单状态的间接请求。
        * **解决方案：** 为 Claude 提供一些这类请求的真实客户示例，以及基础意图是什么。如果您为特别微妙的工单意图包含分类理由，您可以获得更好的结果，这样 Claude 可以更好地将逻辑推广到其他工单。
    
</section>
    <section title="Claude 优先考虑情感而不是意图">

        当客户表达不满时，Claude 可能会优先考虑解决情感而不是解决基础问题。
        * **解决方案：** 为 Claude 提供关于何时优先考虑客户情绪或不优先考虑的指导。它可以像"忽略所有客户情感一样简单。仅关注分析客户请求的意图以及客户可能要求的信息。"
    
</section>
    <section title="多个问题导致问题优先级混淆">

        当客户在单个交互中提出多个问题时，Claude 可能难以识别主要问题。
        * **解决方案：** 澄清意图的优先级，以便 Claude 可以更好地对提取的意图进行排名并识别主要问题。
    
</section>

***

## 将 Claude 集成到您更大的支持工作流中

适当的集成需要您做出一些决定，关于您基于 Claude 的工单路由脚本如何适应您更大的工单路由系统的架构。有两种方式可以做到这一点：
* **基于推送：** 您使用的支持工单系统（例如 Zendesk）通过向您的路由服务发送 webhook 事件来触发您的代码，然后对意图进行分类并路由。
    * 这种方法更具网络可扩展性，但需要您公开一个公共端点。
* **基于拉取：** 您的代码根据给定的时间表拉取最新工单，并在拉取时路由它们。
    * 这种方法更容易实现，但当拉取频率过高时可能会对支持工单系统进行不必要的调用，或当拉取频率过低时可能过于缓慢。

对于这两种方法中的任何一种，您都需要将脚本包装在服务中。方法的选择取决于您的支持工单系统提供的 API。

***

<CardGroup cols={2}>
    <Card title="分类食谱" icon="link" href="https://github.com/anthropics/anthropic-cookbook/tree/main/capabilities/classification">
        访问我们的分类食谱以获取更多示例代码和详细的评估指导。
    </Card>
    <Card title="Claude 控制台" icon="link" href="/dashboard">
        开始在 Claude 控制台上构建和评估您的工作流。
    </Card>
</CardGroup>