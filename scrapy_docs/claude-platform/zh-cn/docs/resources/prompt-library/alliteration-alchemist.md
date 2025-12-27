# 头韵炼金术师

为任何给定主题生成头韵短语和句子。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 你的任务是为给定主题创建头韵短语和句子。确保头韵不仅听起来悦耳，还能传达相关信息或唤起与主题相关的适当情感。 |
| User   | 海洋 |

### 示例输出

> 1.  轻柔的波浪冲刷着海岸。
> 2.  咸咸的海浪飞溅在沙滩上。
> 3.  汹涌的洋流雕刻着海岸悬崖。
> 4.  深邃、黑暗的深处隐藏着危险。
> 5.  潺潺的小溪为海滩带来生机。
> 6.  强大、雄伟的哺乳动物在水中迁徙。
> 7.  雷鸣般的潮汐将船只抛来抛去。
> 8.  闪闪发光的浅滩庇护着神秘的海洋生物。
> 9.  波光粼粼的倒影揭示了水面下的世界。
> 10. 旋转、汹涌的海洋塑造着海岸线。
> 11. 泡沫翻滚的浮冰自由流动。
> 12. 优雅的海鸥在闪闪发光的海洋上空滑翔。
> 13. 顽皮的海豚在纯净的水中跳跃。
> 14. 咸咸的微风吹过无边的蓝色。
> 15. 有节奏的波纹向岩石海岸滚动。
> 16. 珊瑚洞穴隐藏着多彩的水生群落。
> 17. 暴风雨的天空和汹涌的海洋显示着自然的力量。
> 18. 翻滚的潮汐改变着宁静的海湾。
> 19. 旋转、呼啸的风掀起波浪。
> 20. 宁静、蓝宝石般的海水延伸到地平线。

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
        max_tokens=1000,
        temperature=1,
        system="你的任务是为给定主题创建头韵短语和句子。确保头韵不仅听起来悦耳，还能传达相关信息或唤起与主题相关的适当情感。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "海洋"
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
      system: "你的任务是为给定主题创建头韵短语和句子。确保头韵不仅听起来悦耳，还能传达相关信息或唤起与主题相关的适当情感。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "海洋"
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
        system="你的任务是为给定主题创建头韵短语和句子。确保头韵不仅听起来悦耳，还能传达相关信息或唤起与主题相关的适当情感。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "海洋"
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
      system: "你的任务是为给定主题创建头韵短语和句子。确保头韵不仅听起来悦耳，还能传达相关信息或唤起与主题相关的适当情感。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "海洋"
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
        system="你的任务是为给定主题创建头韵短语和句子。确保头韵不仅听起来悦耳，还能传达相关信息或唤起与主题相关的适当情感。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "海洋"
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
      system: "你的任务是为给定主题创建头韵短语和句子。确保头韵不仅听起来悦耳，还能传达相关信息或唤起与主题相关的适当情感。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "海洋"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>