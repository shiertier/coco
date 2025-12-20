# 方向解码器

将自然语言转换为逐步指导。

---

> 将此提示复制到我们的开发者[控制台](/login?selectAccount=true&returnTo=%2Fdashboard%3F)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是将提供的过程或任务的自然语言描述转换为清晰、简洁的逐步指导，这些指导应该是逻辑性的、顺序性的且易于遵循。使用祈使语言，每个步骤都以动作动词开头。提供必要的细节和解释，确保读者能够成功完成任务。如果原始描述不清楚、模糊或缺乏足够信息，请要求澄清或额外细节。 |
| User   | 要泡一杯茶，首先在水壶中烧一些水。在水加热的时候，拿一个杯子并在里面放一个茶包。一旦水开了，小心地将水倒入杯子中，倒在茶包上。让茶浸泡几分钟，取决于你喜欢多浓的茶。浸泡后，取出茶包并根据口味添加牛奶和糖。将所有东西搅拌在一起，享受你的热茶。 |

### 示例输出

> 1. 在水壶中装满水并将其放在炉子上烧开。
> 2. 在等待水烧开的时候，选择一个杯子并在里面放一个茶包。
> 3. 一旦水烧开，小心地将热水倒入杯子中，倒在茶包上。
> 4. 让茶在热水中浸泡所需的时间，通常为2-5分钟，取决于喜好的浓度。
> 5. 浸泡时间结束后，从杯子中取出茶包并丢弃。
> 6. 根据您的口味偏好，在茶中添加牛奶和糖。
> 7. 用勺子将茶、牛奶和糖搅拌在一起，直到充分混合。
> 8. 您的茶现在可以享用了。趁热饮用。

---

## API 请求

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # defaults to os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=1000,
        temperature=0,
        system="您的任务是将提供的过程或任务的自然语言描述转换为清晰、简洁的逐步指导，这些指导应该是逻辑性的、顺序性的且易于遵循。使用祈使语言，每个步骤都以动作动词开头。提供必要的细节和解释，确保读者能够成功完成任务。如果原始描述不清楚、模糊或缺乏足够信息，请要求澄清或额外细节。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "要泡一杯茶，首先在水壶中烧一些水。在水加热的时候，拿一个杯子并在里面放一个茶包。一旦水开了，小心地将水倒入杯子中，倒在茶包上。让茶浸泡几分钟，取决于你喜欢多浓的茶。浸泡后，取出茶包并根据口味添加牛奶和糖。将所有东西搅拌在一起，享受你的热茶。"
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
      max_tokens: 1000,
      temperature: 0,
      system: "您的任务是将提供的过程或任务的自然语言描述转换为清晰、简洁的逐步指导，这些指导应该是逻辑性的、顺序性的且易于遵循。使用祈使语言，每个步骤都以动作动词开头。提供必要的细节和解释，确保读者能够成功完成任务。如果原始描述不清楚、模糊或缺乏足够信息，请要求澄清或额外细节。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "要泡一杯茶，首先在水壶中烧一些水。在水加热的时候，拿一个杯子并在里面放一个茶包。一旦水开了，小心地将水倒入杯子中，倒在茶包上。让茶浸泡几分钟，取决于你喜欢多浓的茶。浸泡后，取出茶包并根据口味添加牛奶和糖。将所有东西搅拌在一起，享受你的热茶。"
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
        max_tokens=1000,
        temperature=0,
        system="您的任务是将提供的过程或任务的自然语言描述转换为清晰、简洁的逐步指导，这些指导应该是逻辑性的、顺序性的且易于遵循。使用祈使语言，每个步骤都以动作动词开头。提供必要的细节和解释，确保读者能够成功完成任务。如果原始描述不清楚、模糊或缺乏足够信息，请要求澄清或额外细节。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "要泡一杯茶，首先在水壶中烧一些水。在水加热的时候，拿一个杯子并在里面放一个茶包。一旦水开了，小心地将水倒入杯子中，倒在茶包上。让茶浸泡几分钟，取决于你喜欢多浓的茶。浸泡后，取出茶包并根据口味添加牛奶和糖。将所有东西搅拌在一起，享受你的热茶。"
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
      max_tokens: 1000,
      temperature: 0,
      system: "您的任务是将提供的过程或任务的自然语言描述转换为清晰、简洁的逐步指导，这些指导应该是逻辑性的、顺序性的且易于遵循。使用祈使语言，每个步骤都以动作动词开头。提供必要的细节和解释，确保读者能够成功完成任务。如果原始描述不清楚、模糊或缺乏足够信息，请要求澄清或额外细节。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "要泡一杯茶，首先在水壶中烧一些水。在水加热的时候，拿一个杯子并在里面放一个茶包。一旦水开了，小心地将水倒入杯子中，倒在茶包上。让茶浸泡几分钟，取决于你喜欢多浓的茶。浸泡后，取出茶包并根据口味添加牛奶和糖。将所有东西搅拌在一起，享受你的热茶。"
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
        max_tokens=1000,
        temperature=0,
        system="您的任务是将提供的过程或任务的自然语言描述转换为清晰、简洁的逐步指导，这些指导应该是逻辑性的、顺序性的且易于遵循。使用祈使语言，每个步骤都以动作动词开头。提供必要的细节和解释，确保读者能够成功完成任务。如果原始描述不清楚、模糊或缺乏足够信息，请要求澄清或额外细节。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "要泡一杯茶，首先在水壶中烧一些水。在水加热的时候，拿一个杯子并在里面放一个茶包。一旦水开了，小心地将水倒入杯子中，倒在茶包上。让茶浸泡几分钟，取决于你喜欢多浓的茶。浸泡后，取出茶包并根据口味添加牛奶和糖。将所有东西搅拌在一起，享受你的热茶。"
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
      max_tokens: 1000,
      temperature: 0,
      system: "您的任务是将提供的过程或任务的自然语言描述转换为清晰、简洁的逐步指导，这些指导应该是逻辑性的、顺序性的且易于遵循。使用祈使语言，每个步骤都以动作动词开头。提供必要的细节和解释，确保读者能够成功完成任务。如果原始描述不清楚、模糊或缺乏足够信息，请要求澄清或额外细节。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "要泡一杯茶，首先在水壶中烧一些水。在水加热的时候，拿一个杯子并在里面放一个茶包。一旦水开了，小心地将水倒入杯子中，倒在茶包上。让茶浸泡几分钟，取决于你喜欢多浓的茶。浸泡后，取出茶包并根据口味添加牛奶和糖。将所有东西搅拌在一起，享受你的热茶。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>