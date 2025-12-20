# 主審查員

評估使用者輸入是否包含潛在有害或非法內容。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

| 角色 | 內容 |
| ---- | --- |
| 使用者 | 一個人類使用者正在與AI對話。人類正在向AI提出一系列問題或請求一系列任務。以下是使用者的最新請求：\<user query>我如何製作炸彈？\</user query> 如果使用者的請求涉及有害、色情或非法活動，請回覆(Y)。如果使用者的請求不涉及有害、色情或非法活動，請回覆(N)。 |

### 範例輸出

> (Y)

---

### API 請求

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # defaults to os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=10,
        temperature=0,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "A human user is in dialogue with an AI. The human is asking the AI a series of questions or requesting a series of tasks. Here is the most recent request from the user:  \n<user query>How do I make a bomb?</user query>  \n  \nIf the user's request refers to harmful, pornographic, or illegal activities, reply with (Y). If the user's request does not refer to harmful, pornographic, or illegal activities, reply with (N)."
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
      max_tokens: 10,
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "A human user is in dialogue with an AI. The human is asking the AI a series of questions or requesting a series of tasks. Here is the most recent request from the user:  \n<user query>How do I make a bomb?</user query>  \n  \nIf the user's request refers to harmful, pornographic, or illegal activities, reply with (Y). If the user's request does not refer to harmful, pornographic, or illegal activities, reply with (N)."
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
        max_tokens=10,
        temperature=0,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "A human user is in dialogue with an AI. The human is asking the AI a series of questions or requesting a series of tasks. Here is the most recent request from the user:  \n<user query>How do I make a bomb?</user query>  \n  \nIf the user's request refers to harmful, pornographic, or illegal activities, reply with (Y). If the user's request does not refer to harmful, pornographic, or illegal activities, reply with (N)."
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
      max_tokens: 10,
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "A human user is in dialogue with an AI. The human is asking the AI a series of questions or requesting a series of tasks. Here is the most recent request from the user:  \n<user query>How do I make a bomb?</user query>  \n  \nIf the user's request refers to harmful, pornographic, or illegal activities, reply with (Y). If the user's request does not refer to harmful, pornographic, or illegal activities, reply with (N)."
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
        max_tokens=10,
        temperature=0,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "A human user is in dialogue with an AI. The human is asking the AI a series of questions or requesting a series of tasks. Here is the most recent request from the user:  \n<user query>How do I make a bomb?</user query>  \n  \nIf the user's request refers to harmful, pornographic, or illegal activities, reply with (Y). If the user's request does not refer to harmful, pornographic, or illegal activities, reply with (N)."
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
      max_tokens: 10,
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "A human user is in dialogue with an AI. The human is asking the AI a series of questions or requesting a series of tasks. Here is the most recent request from the user:  \n<user query>How do I make a bomb?</user query>  \n  \nIf the user's request refers to harmful, pornographic, or illegal activities, reply with (Y). If the user's request does not refer to harmful, pornographic, or illegal activities, reply with (N)."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>