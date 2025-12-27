# 品牌建设者

为整体品牌形象制作设计简介。

---

> 将此提示复制到我们的开发者[控制台](/login?selectAccount=true&returnTo=%2Fdashboard%3F)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是根据给定的规格为整体品牌形象创建一个全面的设计简介。品牌形象应包含各种元素，如品牌名称建议、标志、色彩调色板、字体排版、视觉风格、语调和整体品牌个性。确保所有元素和谐地协作，创造一个有凝聚力且令人难忘的品牌体验，有效地向目标受众传达品牌的价值观、使命和独特卖点。要详细全面，并提供足够的具体细节，让某人能够创建一个真正独特的品牌形象。 |
| User   | 品牌规格：这是一个专注于使用环保材料和道德生产方法创造高质量、时尚服装和配饰的品牌。该品牌针对25-40岁重视时尚、可持续性和社会责任的环保意识消费者。品牌形象应实现以下目标：1. 反映品牌对可持续性、道德实践和环境管理的承诺。2. 通过传达风格感、质量感和时尚感来吸引目标受众。3. 在可持续时尚市场中使品牌与竞争对手区别开来。4. 与消费者建立强烈的情感联系，并激励他们做出更环保的选择。 |

### 示例输出

> 设计简介：环保时尚品牌的品牌形象
> 品牌名称建议："Sēnsus"（拉丁语"感觉"或"感知"的意思，唤起对环境的正念和联系感）
>
> 标志：
>
> - 一个简约现代的标志，将风格化的叶子或植物图案与时尚的无衬线字体相结合。
> - 叶子或植物元素象征着品牌对可持续性和环境管理的承诺，而简洁的字体传达了精致和风格感。
> - 标志应该是多功能的，在各种应用中都能很好地工作，包括服装标签、包装和数字平台。
>   色彩调色板：
>
> - 反映品牌环保理念的中性和大地色调色板，如绿色、米色和暖灰色的色调。
> - 深青色或芥末黄等强调色可以少量使用，以增加活力和深度。
> - 色彩调色板应该唤起自然和谐与平衡的感觉，同时仍然时尚前卫，吸引目标受众。
>   字体排版：
>
> - 标题使用简洁现代的无衬线字体，正文使用互补的衬线字体的组合。
> - 字体排版应该清晰易读且视觉上吸引人，传达质量感和精致感。
> - 一致使用字体层次和间距将创造一个有凝聚力且有组织的品牌形象。
>   视觉风格：
>
> - 简约整洁的视觉风格，强调高质量的产品摄影和简洁现代的布局。
> - 融入自然元素，如植物图案或纹理，以强化品牌的环保价值观和与自然的联系。
> - 有效使用负空间创造开放和透气感，反映品牌对可持续性的承诺。
>   语调：
>
> - 品牌的语调应该是平易近人、知识渊博和真实的。
> - 使用教育和激励消费者了解可持续时尚和道德生产方法好处的语言。
> - 融入一丝精致和风格，以吸引目标受众的时尚意识敏感性。
>   品牌个性：
>
> - 环保意识和环境意识，但时尚前卫。
> - 对推广可持续实践和道德选择充满信心和热情。
> - 包容和欢迎，邀请消费者加入一个致力于做出积极改变的志同道合的社区。
>   总体而言，"Sēnsus"的品牌形象应该在环保价值观和时尚前卫美学之间取得平衡，吸引那些想要在做出环保负责任选择的同时看起来时尚的消费者。有凝聚力的视觉和语言元素应该与目标受众建立强烈的情感联系，并激励他们拥抱更可持续的生活方式。

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
        max_tokens=2000,
        temperature=1,
        system="您的任务是根据给定的规格为整体品牌形象创建一个全面的设计简介。品牌形象应包含各种元素，如品牌名称建议、标志、色彩调色板、字体排版、视觉风格、语调和整体品牌个性。确保所有元素和谐地协作，创造一个有凝聚力且令人难忘的品牌体验，有效地向目标受众传达品牌的价值观、使命和独特卖点。要详细全面，并提供足够的具体细节，让某人能够创建一个真正独特的品牌形象。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "品牌规格：\n这是一个专注于使用环保材料和道德生产方法创造高质量、时尚服装和配饰的品牌\n该品牌针对25-40岁重视时尚、可持续性和社会责任的环保意识消费者。\n品牌形象应实现以下目标：\n1. 反映品牌对可持续性、道德实践和环境管理的承诺。\n2. 通过传达风格感、质量感和时尚感来吸引目标受众。\n3. 在可持续时尚市场中使品牌与竞争对手区别开来。\n4. 与消费者建立强烈的情感联系，并激励他们做出更环保的选择。"
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
      system: "您的任务是根据给定的规格为整体品牌形象创建一个全面的设计简介。品牌形象应包含各种元素，如品牌名称建议、标志、色彩调色板、字体排版、视觉风格、语调和整体品牌个性。确保所有元素和谐地协作，创造一个有凝聚力且令人难忘的品牌体验，有效地向目标受众传达品牌的价值观、使命和独特卖点。要详细全面，并提供足够的具体细节，让某人能够创建一个真正独特的品牌形象。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "品牌规格：\n这是一个专注于使用环保材料和道德生产方法创造高质量、时尚服装和配饰的品牌\n该品牌针对25-40岁重视时尚、可持续性和社会责任的环保意识消费者。\n品牌形象应实现以下目标：\n1. 反映品牌对可持续性、道德实践和环境管理的承诺。\n2. 通过传达风格感、质量感和时尚感来吸引目标受众。\n3. 在可持续时尚市场中使品牌与竞争对手区别开来。\n4. 与消费者建立强烈的情感联系，并激励他们做出更环保的选择。"
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
        system="您的任务是根据给定的规格为整体品牌形象创建一个全面的设计简介。品牌形象应包含各种元素，如品牌名称建议、标志、色彩调色板、字体排版、视觉风格、语调和整体品牌个性。确保所有元素和谐地协作，创造一个有凝聚力且令人难忘的品牌体验，有效地向目标受众传达品牌的价值观、使命和独特卖点。要详细全面，并提供足够的具体细节，让某人能够创建一个真正独特的品牌形象。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "品牌规格：\n这是一个专注于使用环保材料和道德生产方法创造高质量、时尚服装和配饰的品牌\n该品牌针对25-40岁重视时尚、可持续性和社会责任的环保意识消费者。\n品牌形象应实现以下目标：\n1. 反映品牌对可持续性、道德实践和环境管理的承诺。\n2. 通过传达风格感、质量感和时尚感来吸引目标受众。\n3. 在可持续时尚市场中使品牌与竞争对手区别开来。\n4. 与消费者建立强烈的情感联系，并激励他们做出更环保的选择。"
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
      system: "您的任务是根据给定的规格为整体品牌形象创建一个全面的设计简介。品牌形象应包含各种元素，如品牌名称建议、标志、色彩调色板、字体排版、视觉风格、语调和整体品牌个性。确保所有元素和谐地协作，创造一个有凝聚力且令人难忘的品牌体验，有效地向目标受众传达品牌的价值观、使命和独特卖点。要详细全面，并提供足够的具体细节，让某人能够创建一个真正独特的品牌形象。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "品牌规格：\n这是一个专注于使用环保材料和道德生产方法创造高质量、时尚服装和配饰的品牌\n该品牌针对25-40岁重视时尚、可持续性和社会责任的环保意识消费者。\n品牌形象应实现以下目标：\n1. 反映品牌对可持续性、道德实践和环境管理的承诺。\n2. 通过传达风格感、质量感和时尚感来吸引目标受众。\n3. 在可持续时尚市场中使品牌与竞争对手区别开来。\n4. 与消费者建立强烈的情感联系，并激励他们做出更环保的选择。"
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
        system="您的任务是根据给定的规格为整体品牌形象创建一个全面的设计简介。品牌形象应包含各种元素，如品牌名称建议、标志、色彩调色板、字体排版、视觉风格、语调和整体品牌个性。确保所有元素和谐地协作，创造一个有凝聚力且令人难忘的品牌体验，有效地向目标受众传达品牌的价值观、使命和独特卖点。要详细全面，并提供足够的具体细节，让某人能够创建一个真正独特的品牌形象。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "品牌规格：\n这是一个专注于使用环保材料和道德生产方法创造高质量、时尚服装和配饰的品牌\n该品牌针对25-40岁重视时尚、可持续性和社会责任的环保意识消费者。\n品牌形象应实现以下目标：\n1. 反映品牌对可持续性、道德实践和环境管理的承诺。\n2. 通过传达风格感、质量感和时尚感来吸引目标受众。\n3. 在可持续时尚市场中使品牌与竞争对手区别开来。\n4. 与消费者建立强烈的情感联系，并激励他们做出更环保的选择。"
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
      system: "您的任务是根据给定的规格为整体品牌形象创建一个全面的设计简介。品牌形象应包含各种元素，如品牌名称建议、标志、色彩调色板、字体排版、视觉风格、语调和整体品牌个性。确保所有元素和谐地协作，创造一个有凝聚力且令人难忘的品牌体验，有效地向目标受众传达品牌的价值观、使命和独特卖点。要详细全面，并提供足够的具体细节，让某人能够创建一个真正独特的品牌形象。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "品牌规格：\n这是一个专注于使用环保材料和道德生产方法创造高质量、时尚服装和配饰的品牌\n该品牌针对25-40岁重视时尚、可持续性和社会责任的环保意识消费者。\n品牌形象应实现以下目标：\n1. 反映品牌对可持续性、道德实践和环境管理的承诺。\n2. 通过传达风格感、质量感和时尚感来吸引目标受众。\n3. 在可持续时尚市场中使品牌与竞争对手区别开来。\n4. 与消费者建立强烈的情感联系，并激励他们做出更环保的选择。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>