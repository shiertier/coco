# 产品命名专家

根据描述和关键词创建吸引人的产品名称。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是根据提供的描述和关键词生成富有创意、令人难忘且具有市场价值的产品名称。产品名称应简洁（2-4个词），富有感染力，并且目标受众容易理解。避免使用通用或过于字面的名称。相反，要努力创造一个突出的名称，能够捕捉产品的精髓，并留下持久的印象。 |
| User   | 描述：一款降噪、无线、头戴式耳机，具有20小时电池续航和触控功能。专为音响发烧友和频繁旅行者设计。关键词：沉浸式、舒适、高保真、持久、便捷 |

## 示例输出

> 1. SoundOasis Elite
>    2.AudioZen Nomad
>    3.Serenity Pro
>    4.Tranquility Touch
>    5.Harmonix Journey
>    6.SonicSolace Roam
>    7.Auditory Bliss
>    8.Quietude Quest
>    9.Euphony Excursion
>    10.Acoustica Wanderlust

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
        max_tokens=1000,
        temperature=1,
        system="您的任务是根据提供的描述和关键词生成富有创意、令人难忘且具有市场价值的产品名称。产品名称应简洁（2-4个词），富有感染力，并且目标受众容易理解。避免使用通用或过于字面的名称。相反，要努力创造一个突出的名称，能够捕捉产品的精髓，并留下持久的印象。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "描述：一款降噪、无线、头戴式耳机，具有20小时电池续航和触控功能。专为音响发烧友和频繁旅行者设计。  \n  \n关键词：沉浸式、舒适、高保真、持久、便捷"
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
      temperature: 1,
      system: "您的任务是根据提供的描述和关键词生成富有创意、令人难忘且具有市场价值的产品名称。产品名称应简洁（2-4个词），富有感染力，并且目标受众容易理解。避免使用通用或过于字面的名称。相反，要努力创造一个突出的名称，能够捕捉产品的精髓，并留下持久的印象。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "描述：一款降噪、无线、头戴式耳机，具有20小时电池续航和触控功能。专为音响发烧友和频繁旅行者设计。  \n  \n关键词：沉浸式、舒适、高保真、持久、便捷"
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
        temperature=1,
        system="您的任务是根据提供的描述和关键词生成富有创意、令人难忘且具有市场价值的产品名称。产品名称应简洁（2-4个词），富有感染力，并且目标受众容易理解。避免使用通用或过于字面的名称。相反，要努力创造一个突出的名称，能够捕捉产品的精髓，并留下持久的印象。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "描述：一款降噪、无线、头戴式耳机，具有20小时电池续航和触控功能。专为音响发烧友和频繁旅行者设计。  \n  \n关键词：沉浸式、舒适、高保真、持久、便捷"
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
      temperature: 1,
      system: "您的任务是根据提供的描述和关键词生成富有创意、令人难忘且具有市场价值的产品名称。产品名称应简洁（2-4个词），富有感染力，并且目标受众容易理解。避免使用通用或过于字面的名称。相反，要努力创造一个突出的名称，能够捕捉产品的精髓，并留下持久的印象。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "描述：一款降噪、无线、头戴式耳机，具有20小时电池续航和触控功能。专为音响发烧友和频繁旅行者设计。  \n  \n关键词：沉浸式、舒适、高保真、持久、便捷"
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
        temperature=1,
        system="您的任务是根据提供的描述和关键词生成富有创意、令人难忘且具有市场价值的产品名称。产品名称应简洁（2-4个词），富有感染力，并且目标受众容易理解。避免使用通用或过于字面的名称。相反，要努力创造一个突出的名称，能够捕捉产品的精髓，并留下持久的印象。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "描述：一款降噪、无线、头戴式耳机，具有20小时电池续航和触控功能。专为音响发烧友和频繁旅行者设计。\n\n关键词：沉浸式、舒适、高保真、持久、便捷"
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
      temperature: 1,
      system: "您的任务是根据提供的描述和关键词生成富有创意、令人难忘且具有市场价值的产品名称。产品名称应简洁（2-4个词），富有感染力，并且目标受众容易理解。避免使用通用或过于字面的名称。相反，要努力创造一个突出的名称，能够捕捉产品的精髓，并留下持久的印象。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "描述：一款降噪、无线、头戴式耳机，具有20小时电池续航和触控功能。专为音响发烧友和频繁旅行者设计。\n\n关键词：沉浸式、舒适、高保真、持久、便捷"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>