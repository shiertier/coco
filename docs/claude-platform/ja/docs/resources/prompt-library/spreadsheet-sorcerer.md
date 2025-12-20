# スプレッドシート魔術師

さまざまなタイプのデータを含むCSVスプレッドシートを生成します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、指定されたタイプのデータを含むCSVスプレッドシートを生成することです。スプレッドシートは整理されており、明確な列ヘッダーと各列に適切なデータタイプを持つ必要があります。データが現実的で多様性があり、一貫してフォーマットされていることを確認してください。ヘッダー行を除いて、最低10行のデータを含めてください。 |
| User   | 図書館の本に関する情報を含むスプレッドシートを作成してください。次の列を含めてください：Book ID、Title、Author、Genre、Publication Year、Available Copies。データがさまざまなジャンルと出版年をカバーし、利用可能なコピー数が本によって異なることを確認してください。                             |

### 出力例

> あなたの要件に基づいて、図書館の本に関する情報を含むCSVスプレッドシートは以下の通りです：
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

### APIリクエスト

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
        system="あなたのタスクは、指定されたタイプのデータを含むCSVスプレッドシートを生成することです。スプレッドシートは整理されており、明確な列ヘッダーと各列に適切なデータタイプを持つ必要があります。データが現実的で多様性があり、一貫してフォーマットされていることを確認してください。ヘッダー行を除いて、最低10行のデータを含めてください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "図書館の本に関する情報を含むスプレッドシートを作成してください。次の列を含めてください：Book ID、Title、Author、Genre、Publication Year、Available Copies。データがさまざまなジャンルと出版年をカバーし、利用可能なコピー数が本によって異なることを確認してください。"
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
      system: "あなたのタスクは、指定されたタイプのデータを含むCSVスプレッドシートを生成することです。スプレッドシートは整理されており、明確な列ヘッダーと各列に適切なデータタイプを持つ必要があります。データが現実的で多様性があり、一貫してフォーマットされていることを確認してください。ヘッダー行を除いて、最低10行のデータを含めてください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "図書館の本に関する情報を含むスプレッドシートを作成してください。次の列を含めてください：Book ID、Title、Author、Genre、Publication Year、Available Copies。データがさまざまなジャンルと出版年をカバーし、利用可能なコピー数が本によって異なることを確認してください。"
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
        system="あなたのタスクは、指定されたタイプのデータを含むCSVスプレッドシートを生成することです。スプレッドシートは整理されており、明確な列ヘッダーと各列に適切なデータタイプを持つ必要があります。データが現実的で多様性があり、一貫してフォーマットされていることを確認してください。ヘッダー行を除いて、最低10行のデータを含めてください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "図書館の本に関する情報を含むスプレッドシートを作成してください。次の列を含めてください：Book ID、Title、Author、Genre、Publication Year、Available Copies。データがさまざまなジャンルと出版年をカバーし、利用可能なコピー数が本によって異なることを確認してください。"
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
      system: "あなたのタスクは、指定されたタイプのデータを含むCSVスプレッドシートを生成することです。スプレッドシートは整理されており、明確な列ヘッダーと各列に適切なデータタイプを持つ必要があります。データが現実的で多様性があり、一貫してフォーマットされていることを確認してください。ヘッダー行を除いて、最低10行のデータを含めてください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "図書館の本に関する情報を含むスプレッドシートを作成してください。次の列を含めてください：Book ID、Title、Author、Genre、Publication Year、Available Copies。データがさまざまなジャンルと出版年をカバーし、利用可能なコピー数が本によって異なることを確認してください。"
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
        system="あなたのタスクは、指定されたタイプのデータを含むCSVスプレッドシートを生成することです。スプレッドシートは整理されており、明確な列ヘッダーと各列に適切なデータタイプを持つ必要があります。データが現実的で多様性があり、一貫してフォーマットされていることを確認してください。ヘッダー行を除いて、最低10行のデータを含めてください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "図書館の本に関する情報を含むスプレッドシートを作成してください。次の列を含めてください：Book ID、Title、Author、Genre、Publication Year、Available Copies。データがさまざまなジャンルと出版年をカバーし、利用可能なコピー数が本によって異なることを確認してください。"
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
      system: "あなたのタスクは、指定されたタイプのデータを含むCSVスプレッドシートを生成することです。スプレッドシートは整理されており、明確な列ヘッダーと各列に適切なデータタイプを持つ必要があります。データが現実的で多様性があり、一貫してフォーマットされていることを確認してください。ヘッダー行を除いて、最低10行のデータを含めてください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "図書館の本に関する情報を含むスプレッドシートを作成してください。次の列を含めてください：Book ID、Title、Author、Genre、Publication Year、Available Copies。データがさまざまなジャンルと出版年をカバーし、利用可能なコピー数が本によって異なることを確認してください。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>