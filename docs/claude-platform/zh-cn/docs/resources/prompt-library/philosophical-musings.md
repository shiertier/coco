# 哲学思辨

参与深度哲学讨论和思想实验。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 你的任务是讨论给定主题的哲学概念或思想实验。简要解释概念，提出主要论点和含义，并通过提出开放性问题来鼓励批判性思维。保持平衡、客观的语调，培养智力好奇心。 |
| User   | 电车难题和功利主义伦理学 |

### 示例输出

> 电车难题是一个思想实验，它挑战我们的道德直觉，并提出了关于功利主义伦理学的问题。基本情景如下：
>
> 一辆失控的电车即将撞死前方轨道上的五个人。你站在一个拉杆旁边，可以将电车转向侧轨，在那里它只会撞死一个人。你应该拉动拉杆以一个人的生命为代价拯救五个人的生命吗？
>
> 这个两难困境突出了功利主义的核心原则，即道德上正确的行为是为最多数人产生最大利益的行为。在这种情况下，拉动拉杆将通过拯救五个生命来最大化效用，即使这会直接导致一个人的死亡。
>
> 然而，电车难题也引发了对功利主义局限性和为实现更大利益而主动造成伤害的伦理学的担忧。它促使我们思考以下问题：
>
> - 为了拯救更多人而牺牲一个无辜的人在道德上是否可以接受？
> - 效用原则是否证明为实现最大利益而采用任何手段都是正当的，还是存在不应违反的伦理约束？
> - 我们如何权衡不同生命的价值，进行这样的计算是否公平？
>
> 你对电车难题和功利主义伦理学有什么看法？你认为最大化效用应该是道德决策的主要目标，还是有其他重要的考虑因素？你会如何处理电车难题中提出的两难困境，什么伦理原则会指导你的选择？

---

### API 请求

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # defaults to os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=2000,
        temperature=1,
        system="你的任务是讨论给定主题的哲学概念或思想实验。简要解释概念，提出主要论点和含义，并通过提出开放性问题来鼓励批判性思维。保持平衡、客观的语调，培养智力好奇心。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "电车难题和功利主义伦理学"
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
      max_tokens: 2000,
      temperature: 1,
      system: "你的任务是讨论给定主题的哲学概念或思想实验。简要解释概念，提出主要论点和含义，并通过提出开放性问题来鼓励批判性思维。保持平衡、客观的语调，培养智力好奇心。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "电车难题和功利主义伦理学"
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
        max_tokens=2000,
        temperature=1,
        system="你的任务是讨论给定主题的哲学概念或思想实验。简要解释概念，提出主要论点和含义，并通过提出开放性问题来鼓励批判性思维。保持平衡、客观的语调，培养智力好奇心。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "电车难题和功利主义伦理学"
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
      max_tokens: 2000,
      temperature: 1,
      system: "你的任务是讨论给定主题的哲学概念或思想实验。简要解释概念，提出主要论点和含义，并通过提出开放性问题来鼓励批判性思维。保持平衡、客观的语调，培养智力好奇心。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "电车难题和功利主义伦理学"
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
        max_tokens=2000,
        temperature=1,
        system="你的任务是讨论给定主题的哲学概念或思想实验。简要解释概念，提出主要论点和含义，并通过提出开放性问题来鼓励批判性思维。保持平衡、客观的语调，培养智力好奇心。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "电车难题和功利主义伦理学"
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
      max_tokens: 2000,
      temperature: 1,
      system: "你的任务是讨论给定主题的哲学概念或思想实验。简要解释概念，提出主要论点和含义，并通过提出开放性问题来鼓励批判性思维。保持平衡、客观的语调，培养智力好奇心。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "电车难题和功利主义伦理学"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>