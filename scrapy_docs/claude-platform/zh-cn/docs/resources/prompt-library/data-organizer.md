# 数据整理器

将非结构化文本转换为定制的JSON表格。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是将提供的非结构化文本转换为使用JSON的良好组织的表格格式。识别文本中提到的主要实体、属性或类别，并将它们用作JSON对象中的键。然后，从文本中提取相关信息并填充JSON对象中的相应值。确保数据在JSON结构中准确表示和正确格式化。生成的JSON表格应该提供原始文本中呈现信息的清晰、结构化概览。 |
| User   | Silvermist Hollow，一个迷人的村庄，是一群非凡个体的家园。其中有Liam Patel博士，一位45岁的耶鲁大学培养的神经外科医生，他在地区医疗中心革新了手术技术。Olivia Chen，28岁，是一位来自加州大学伯克利分校的创新建筑师，她用可持续和令人惊叹的设计改变了村庄的景观。当地剧院因Ethan Kovacs的迷人交响乐而增色，他是一位72岁的茱莉亚音乐学院培养的音乐家和作曲家。Isabella Torres，一位自学成才的厨师，对本地采购食材充满热情，她用农场到餐桌的餐厅创造了烹饪轰动，成为美食爱好者必访的目的地。这些杰出的个体，每个人都有自己独特的才能，为Silvermist Hollow充满活力的生活画卷做出了贡献。 |

### 示例输出

```json
[
  {
    "name": "Dr. Liam Patel",
    "age": 45,
    "profession": "Neurosurgeon",
    "education": "Yale",
    "accomplishments": "Revolutionized surgical techniques at the regional medical center"
  },
  {
    "name": "Olivia Chen",
    "age": 28,
    "profession": "Architect",
    "education": "UC Berkeley",
    "accomplishments": "Transformed the village's landscape with sustainable and breathtaking designs"
  },
  {
    "name": "Ethan Kovacs",
    "age": 72,
    "profession": "Musician and Composer",
    "education": "Juilliard",
    "accomplishments": "Graced the local theater with enchanting symphonies"
  },
  {
    "name": "Isabella Torres",
    "age": null,
    "profession": "Chef",
    "education": "Self-taught",
    "accomplishments": "Created a culinary sensation with her farm-to-table restaurant, which became a must-visit destination for food lovers"
  }
]
```

---

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
        system="您的任务是将提供的非结构化文本转换为使用JSON的良好组织的表格格式。识别文本中提到的主要实体、属性或类别，并将它们用作JSON对象中的键。然后，从文本中提取相关信息并填充JSON对象中的相应值。确保数据在JSON结构中准确表示和正确格式化。生成的JSON表格应该提供原始文本中呈现信息的清晰、结构化概览。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow，一个迷人的村庄，是一群非凡个体的家园。其中有Liam Patel博士，一位45岁的耶鲁大学培养的神经外科医生，他在地区医疗中心革新了手术技术。Olivia Chen，28岁，是一位来自加州大学伯克利分校的创新建筑师，她用可持续和令人惊叹的设计改变了村庄的景观。当地剧院因Ethan Kovacs的迷人交响乐而增色，他是一位72岁的茱莉亚音乐学院培养的音乐家和作曲家。Isabella Torres，一位自学成才的厨师，对本地采购食材充满热情，她用农场到餐桌的餐厅创造了烹饪轰动，成为美食爱好者必访的目的地。这些杰出的个体，每个人都有自己独特的才能，为Silvermist Hollow充满活力的生活画卷做出了贡献。"
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
      system: "您的任务是将提供的非结构化文本转换为使用JSON的良好组织的表格格式。识别文本中提到的主要实体、属性或类别，并将它们用作JSON对象中的键。然后，从文本中提取相关信息并填充JSON对象中的相应值。确保数据在JSON结构中准确表示和正确格式化。生成的JSON表格应该提供原始文本中呈现信息的清晰、结构化概览。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow，一个迷人的村庄，是一群非凡个体的家园。其中有Liam Patel博士，一位45岁的耶鲁大学培养的神经外科医生，他在地区医疗中心革新了手术技术。Olivia Chen，28岁，是一位来自加州大学伯克利分校的创新建筑师，她用可持续和令人惊叹的设计改变了村庄的景观。当地剧院因Ethan Kovacs的迷人交响乐而增色，他是一位72岁的茱莉亚音乐学院培养的音乐家和作曲家。Isabella Torres，一位自学成才的厨师，对本地采购食材充满热情，她用农场到餐桌的餐厅创造了烹饪轰动，成为美食爱好者必访的目的地。这些杰出的个体，每个人都有自己独特的才能，为Silvermist Hollow充满活力的生活画卷做出了贡献。"
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
        system="您的任务是将提供的非结构化文本转换为使用JSON的良好组织的表格格式。识别文本中提到的主要实体、属性或类别，并将它们用作JSON对象中的键。然后，从文本中提取相关信息并填充JSON对象中的相应值。确保数据在JSON结构中准确表示和正确格式化。生成的JSON表格应该提供原始文本中呈现信息的清晰、结构化概览。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow，一个迷人的村庄，是一群非凡个体的家园。其中有Liam Patel博士，一位45岁的耶鲁大学培养的神经外科医生，他在地区医疗中心革新了手术技术。Olivia Chen，28岁，是一位来自加州大学伯克利分校的创新建筑师，她用可持续和令人惊叹的设计改变了村庄的景观。当地剧院因Ethan Kovacs的迷人交响乐而增色，他是一位72岁的茱莉亚音乐学院培养的音乐家和作曲家。Isabella Torres，一位自学成才的厨师，对本地采购食材充满热情，她用农场到餐桌的餐厅创造了烹饪轰动，成为美食爱好者必访的目的地。这些杰出的个体，每个人都有自己独特的才能，为Silvermist Hollow充满活力的生活画卷做出了贡献。"
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
      system: "您的任务是将提供的非结构化文本转换为使用JSON的良好组织的表格格式。识别文本中提到的主要实体、属性或类别，并将它们用作JSON对象中的键。然后，从文本中提取相关信息并填充JSON对象中的相应值。确保数据在JSON结构中准确表示和正确格式化。生成的JSON表格应该提供原始文本中呈现信息的清晰、结构化概览。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow，一个迷人的村庄，是一群非凡个体的家园。其中有Liam Patel博士，一位45岁的耶鲁大学培养的神经外科医生，他在地区医疗中心革新了手术技术。Olivia Chen，28岁，是一位来自加州大学伯克利分校的创新建筑师，她用可持续和令人惊叹的设计改变了村庄的景观。当地剧院因Ethan Kovacs的迷人交响乐而增色，他是一位72岁的茱莉亚音乐学院培养的音乐家和作曲家。Isabella Torres，一位自学成才的厨师，对本地采购食材充满热情，她用农场到餐桌的餐厅创造了烹饪轰动，成为美食爱好者必访的目的地。这些杰出的个体，每个人都有自己独特的才能，为Silvermist Hollow充满活力的生活画卷做出了贡献。"
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
        system="您的任务是将提供的非结构化文本转换为使用JSON的良好组织的表格格式。识别文本中提到的主要实体、属性或类别，并将它们用作JSON对象中的键。然后，从文本中提取相关信息并填充JSON对象中的相应值。确保数据在JSON结构中准确表示和正确格式化。生成的JSON表格应该提供原始文本中呈现信息的清晰、结构化概览。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow，一个迷人的村庄，是一群非凡个体的家园。其中有Liam Patel博士，一位45岁的耶鲁大学培养的神经外科医生，他在地区医疗中心革新了手术技术。Olivia Chen，28岁，是一位来自加州大学伯克利分校的创新建筑师，她用可持续和令人惊叹的设计改变了村庄的景观。当地剧院因Ethan Kovacs的迷人交响乐而增色，他是一位72岁的茱莉亚音乐学院培养的音乐家和作曲家。Isabella Torres，一位自学成才的厨师，对本地采购食材充满热情，她用农场到餐桌的餐厅创造了烹饪轰动，成为美食爱好者必访的目的地。这些杰出的个体，每个人都有自己独特的才能，为Silvermist Hollow充满活力的生活画卷做出了贡献。"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI Type
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 0,
      system: "您的任务是将提供的非结构化文本转换为使用JSON的良好组织的表格格式。识别文本中提到的主要实体、属性或类别，并将它们用作JSON对象中的键。然后，从文本中提取相关信息并填充JSON对象中的相应值。确保数据在JSON结构中准确表示和正确格式化。生成的JSON表格应该提供原始文本中呈现信息的清晰、结构化概览。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow，一个迷人的村庄，是一群非凡个体的家园。其中有Liam Patel博士，一位45岁的耶鲁大学培养的神经外科医生，他在地区医疗中心革新了手术技术。Olivia Chen，28岁，是一位来自加州大学伯克利分校的创新建筑师，她用可持续和令人惊叹的设计改变了村庄的景观。当地剧院因Ethan Kovacs的迷人交响乐而增色，他是一位72岁的茱莉亚音乐学院培养的音乐家和作曲家。Isabella Torres，一位自学成才的厨师，对本地采购食材充满热情，她用农场到餐桌的餐厅创造了烹饪轰动，成为美食爱好者必访的目的地。这些杰出的个体，每个人都有自己独特的才能，为Silvermist Hollow充满活力的生活画卷做出了贡献。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>