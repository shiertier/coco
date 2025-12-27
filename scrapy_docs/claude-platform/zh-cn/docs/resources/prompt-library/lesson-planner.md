# 课程规划师

制作任何学科的深入课程计划。

---

> 将此提示复制到我们的开发者[控制台](/login?selectAccount=true&returnTo=%2Fdashboard%3F)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是为给定学科创建一个全面、引人入胜且结构良好的课程计划。该课程计划应设计为60分钟的课堂会话，并应适合特定的年级水平或年龄组。首先陈述课程目标，这些目标应该清晰、可衡量，并与相关教育标准保持一致。接下来，提供课程的详细大纲，将其分解为引言、主要活动和结论。对于每个部分，描述您将使用的教学方法、学习活动和资源，以有效传达内容并吸引学生。最后，描述您将采用的评估方法来评价学生对课程目标的理解和掌握程度。课程计划应该组织良好、易于遵循，并促进主动学习和批判性思维。 |
| User   | 学科：光合作用简介 年级水平：7年级（12-13岁） |

## 示例输出

> 课程计划：光合作用简介
> 年级水平：7年级（12-13岁）
> 持续时间：60分钟
>
> 课程目标：
> 在课程结束时，学生将能够：
>
> 1. 定义光合作用并解释其在生态系统中的重要性。
> 2. 识别光合作用发生所需的关键组成部分。
> 3. 描述光合作用过程，包括光能、二氧化碳和水的作用。
> 4. 认识光合作用的产物及其对生物的意义。
>    课程大纲：
>
> 5. 引言（10分钟）
>    i. 通过展示植物的短视频片段或图像并讨论它们在环境中的作用来吸引学生。
>    ii. 介绍光合作用的概念及其在维持地球生命中的重要性。
>    iii. 陈述课程目标并提供课程结构概述。
> 6. 主要活动（40分钟）
>    i. 指导教学（15分钟）
>    a. 解释光合作用所需的关键组成部分：阳光、二氧化碳、水和叶绿素。
>    b. 描述光合作用过程，强调光能、二氧化碳和水的作用。
>    c. 讨论光合作用的产物（葡萄糖和氧气）及其对生物的意义。
>    ii. 小组活动（15分钟）
>    a. 将班级分成小组，为每组提供工作表或图形组织器。
>    b. 要求学生合作创建光合作用过程的视觉表示，标记关键组成部分和产物。
>    c. 鼓励学生在小组内讨论并解释他们对光合作用的理解。
>    iii. 班级讨论（10分钟）
>    a. 邀请每组与全班分享他们的视觉表示。
>    b. 促进班级讨论，鼓励学生提问、提供反馈并澄清任何误解。
> 7. 结论（10分钟）
>    i. 总结课程要点，强化光合作用的定义、组成部分、过程和产物。
>    ii. 进行简短测验或出门票来评估学生对课程目标的理解。
>    iii. 为学生提供家庭作业，进一步探索光合作用在不同生态系统中的重要性。
>    评估方法：
>
> 8. 形成性评估：观察学生在指导教学和小组活动期间的参与度和理解程度。
> 9. 总结性评估：评价学生的视觉表示和他们在简短测验或出门票上的表现。
> 10. 家庭作业：审查学生的家庭作业提交，以衡量他们应用课程中所学概念的能力。
>     资源：
>
> 11. 植物及其在环境中作用的视频片段或图像
> 12. 小组活动用的工作表或图形组织器
> 13. 用于评估的简短测验或出门票
> 14. 家庭作业说明

### API请求

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # defaults to os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=4000,
        temperature=0.5,
        system="您的任务是为给定学科创建一个全面、引人入胜且结构良好的课程计划。该课程计划应设计为60分钟的课堂会话，并应适合特定的年级水平或年龄组。首先陈述课程目标，这些目标应该清晰、可衡量，并与相关教育标准保持一致。接下来，提供课程的详细大纲，将其分解为引言、主要活动和结论。对于每个部分，描述您将使用的教学方法、学习活动和资源，以有效传达内容并吸引学生。最后，描述您将采用的评估方法来评价学生对课程目标的理解和掌握程度。课程计划应该组织良好、易于遵循，并促进主动学习和批判性思维。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "学科：光合作用简介  \n年级水平：7年级（12-13岁）"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript TypeScript
    import Anthropic from "@anthropic-ai/sdk";
    
    const anthropic = new Anthropic({
      apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
    });
    
    const msg = await anthropic.messages.create({
      model: "claude-sonnet-4-5",
      max_tokens: 4000,
      temperature: 0.5,
      system: "您的任务是为给定学科创建一个全面、引人入胜且结构良好的课程计划。该课程计划应设计为60分钟的课堂会话，并应适合特定的年级水平或年龄组。首先陈述课程目标，这些目标应该清晰、可衡量，并与相关教育标准保持一致。接下来，提供课程的详细大纲，将其分解为引言、主要活动和结论。对于每个部分，描述您将使用的教学方法、学习活动和资源，以有效传达内容并吸引学生。最后，描述您将采用的评估方法来评价学生对课程目标的理解和掌握程度。课程计划应该组织良好、易于遵循，并促进主动学习和批判性思维。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "学科：光合作用简介  \n年级水平：7年级（12-13岁）"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
    
    ```python AWS Bedrock Python
    from anthropic import AnthropicBedrock
    
    # See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # for authentication options
    client = AnthropicBedrock()
    
    message = client.messages.create(
        model="anthropic.claude-sonnet-4-5-20250929-v1:0",
        max_tokens=4000,
        temperature=0.5,
        system="您的任务是为给定学科创建一个全面、引人入胜且结构良好的课程计划。该课程计划应设计为60分钟的课堂会话，并应适合特定的年级水平或年龄组。首先陈述课程目标，这些目标应该清晰、可衡量，并与相关教育标准保持一致。接下来，提供课程的详细大纲，将其分解为引言、主要活动和结论。对于每个部分，描述您将使用的教学方法、学习活动和资源，以有效传达内容并吸引学生。最后，描述您将采用的评估方法来评价学生对课程目标的理解和掌握程度。课程计划应该组织良好、易于遵循，并促进主动学习和批判性思维。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "学科：光合作用简介  \n年级水平：7年级（12-13岁）"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    // See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    // for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 4000,
      temperature: 0.5,
      system: "您的任务是为给定学科创建一个全面、引人入胜且结构良好的课程计划。该课程计划应设计为60分钟的课堂会话，并应适合特定的年级水平或年龄组。首先陈述课程目标，这些目标应该清晰、可衡量，并与相关教育标准保持一致。接下来，提供课程的详细大纲，将其分解为引言、主要活动和结论。对于每个部分，描述您将使用的教学方法、学习活动和资源，以有效传达内容并吸引学生。最后，描述您将采用的评估方法来评价学生对课程目标的理解和掌握程度。课程计划应该组织良好、易于遵循，并促进主动学习和批判性思维。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "学科：光合作用简介  \n年级水平：7年级（12-13岁）"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
    
    ```python Vertex AI Python
    from anthropic import AnthropicVertex
    
    client = AnthropicVertex()
    
    message = client.messages.create(
        model="claude-sonnet-4@20250514",
        max_tokens=4000,
        temperature=0.5,
        system="您的任务是为给定学科创建一个全面、引人入胜且结构良好的课程计划。该课程计划应设计为60分钟的课堂会话，并应适合特定的年级水平或年龄组。首先陈述课程目标，这些目标应该清晰、可衡量，并与相关教育标准保持一致。接下来，提供课程的详细大纲，将其分解为引言、主要活动和结论。对于每个部分，描述您将使用的教学方法、学习活动和资源，以有效传达内容并吸引学生。最后，描述您将采用的评估方法来评价学生对课程目标的理解和掌握程度。课程计划应该组织良好、易于遵循，并促进主动学习和批判性思维。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "学科：光合作用简介  \n年级水平：7年级（12-13岁）"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 4000,
      temperature: 0.5,
      system: "您的任务是为给定学科创建一个全面、引人入胜且结构良好的课程计划。该课程计划应设计为60分钟的课堂会话，并应适合特定的年级水平或年龄组。首先陈述课程目标，这些目标应该清晰、可衡量，并与相关教育标准保持一致。接下来，提供课程的详细大纲，将其分解为引言、主要活动和结论。对于每个部分，描述您将使用的教学方法、学习活动和资源，以有效传达内容并吸引学生。最后，描述您将采用的评估方法来评价学生对课程目标的理解和掌握程度。课程计划应该组织良好、易于遵循，并促进主动学习和批判性思维。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "学科：光合作用简介  \n年级水平：7年级（12-13岁）"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>