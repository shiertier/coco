# 电子表格魔法师

生成包含各种类型数据的CSV电子表格。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是生成包含指定类型数据的CSV电子表格。电子表格应该组织良好，具有清晰的列标题和每列适当的数据类型。确保数据真实、多样化且格式一致。包含至少10行数据，不计算标题行。 |
| User   | 创建一个包含图书馆书籍信息的电子表格。包含以下列：图书ID、标题、作者、类型、出版年份和可用副本数。确保数据涵盖各种类型和出版年份，并且可用副本数在不同书籍之间有所变化。                             |

### 示例输出

> 这是一个包含图书馆书籍信息的CSV电子表格，基于您的要求：
>
> Book ID,Title,Author,Genre,Publication Year,Available Copies  
> 1,To Kill a Mockingbird,Harper Lee,Classic,1960,5  
> 2,1984,George Orwell,Dystopian,1949,3  
> 3,Pride and Prejudice,Jane Austen,Romance,1813,7  
> 4,The Catcher in the Rye,J.D. Salinger,Coming-of-age,1951,2  
> 5,The Hitchhiker's Guide to the Galaxy,Douglas Adams,Science Fiction,1979,4  
> 6,The Da Vinci Code,Dan Brown,Mystery,2003,6  
> 7,The Kite Runner,Khaled Hosseini,Historical Fiction,2003,3  
> 8,The Alchemist,Paulo Coelho,Philosophical Fiction,1988,5  
> 9,The Hunger Games,Suzanne Collins,Young Adult,2008,8  
> 10,The Girl with the Dragon Tattoo,Stieg Larsson,Crime,2005,4  
> 11,The Hobbit,J.R.R. Tolkien,Fantasy,1937,6  
> 12,The Diary of a Young Girl,Anne Frank,Autobiography,1947,2  
> 13,The Fault in Our Stars,John Green,Young Adult,2012,5  
> 14,The Catcher in the Rye,J.D. Salinger,Coming-of-age,1951,3  
> 15,The Curious Incident of the Dog in the Night-Time,Mark Haddon,Mystery,2003,4

---

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
        temperature=0,
        system="您的任务是生成包含指定类型数据的CSV电子表格。电子表格应该组织良好，具有清晰的列标题和每列适当的数据类型。确保数据真实、多样化且格式一致。包含至少10行数据，不计算标题行。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "创建一个包含图书馆书籍信息的电子表格。包含以下列：图书ID、标题、作者、类型、出版年份和可用副本数。确保数据涵盖各种类型和出版年份，并且可用副本数在不同书籍之间有所变化。"
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
      system: "您的任务是生成包含指定类型数据的CSV电子表格。电子表格应该组织良好，具有清晰的列标题和每列适当的数据类型。确保数据真实、多样化且格式一致。包含至少10行数据，不计算标题行。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "创建一个包含图书馆书籍信息的电子表格。包含以下列：图书ID、标题、作者、类型、出版年份和可用副本数。确保数据涵盖各种类型和出版年份，并且可用副本数在不同书籍之间有所变化。"
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
        system="您的任务是生成包含指定类型数据的CSV电子表格。电子表格应该组织良好，具有清晰的列标题和每列适当的数据类型。确保数据真实、多样化且格式一致。包含至少10行数据，不计算标题行。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "创建一个包含图书馆书籍信息的电子表格。包含以下列：图书ID、标题、作者、类型、出版年份和可用副本数。确保数据涵盖各种类型和出版年份，并且可用副本数在不同书籍之间有所变化。"
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
      system: "您的任务是生成包含指定类型数据的CSV电子表格。电子表格应该组织良好，具有清晰的列标题和每列适当的数据类型。确保数据真实、多样化且格式一致。包含至少10行数据，不计算标题行。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "创建一个包含图书馆书籍信息的电子表格。包含以下列：图书ID、标题、作者、类型、出版年份和可用副本数。确保数据涵盖各种类型和出版年份，并且可用副本数在不同书籍之间有所变化。"
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
        system="您的任务是生成包含指定类型数据的CSV电子表格。电子表格应该组织良好，具有清晰的列标题和每列适当的数据类型。确保数据真实、多样化且格式一致。包含至少10行数据，不计算标题行。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "创建一个包含图书馆书籍信息的电子表格。包含以下列：图书ID、标题、作者、类型、出版年份和可用副本数。确保数据涵盖各种类型和出版年份，并且可用副本数在不同书籍之间有所变化。"
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
      system: "您的任务是生成包含指定类型数据的CSV电子表格。电子表格应该组织良好，具有清晰的列标题和每列适当的数据类型。确保数据真实、多样化且格式一致。包含至少10行数据，不计算标题行。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "创建一个包含图书馆书籍信息的电子表格。包含以下列：图书ID、标题、作者、类型、出版年份和可用副本数。确保数据涵盖各种类型和出版年份，并且可用副本数在不同书籍之间有所变化。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>