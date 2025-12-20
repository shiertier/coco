# 哲學思辨

參與深度哲學討論和思想實驗。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是討論給定主題的哲學概念或思想實驗。簡要解釋該概念，呈現主要論點和含義，並通過提出開放性問題來鼓勵批判性思維。保持平衡、客觀的語調，培養智識好奇心。 |
| User   | 電車難題和功利主義倫理學 |

### 範例輸出

> 電車難題是一個思想實驗，它挑戰我們的道德直覺並提出關於功利主義倫理學的問題。基本情境如下：
>
> 一輛失控的電車即將撞死前方軌道上的五個人。您站在一個拉桿旁邊，可以將電車轉向側軌，在那裡它只會撞死一個人。您應該拉動拉桿以犧牲一個人的生命來拯救五個人嗎？
>
> 這個兩難困境突出了功利主義的核心原則，該原則認為道德上正確的行動是為最多數人產生最大利益的行動。在這種情況下，拉動拉桿將通過拯救五個生命來最大化效用，儘管它會直接導致一個人的死亡。
>
> 然而，電車難題也提出了對功利主義限制和為實現更大利益而主動造成傷害的倫理學的擔憂。它促使我們思考以下問題：
>
> - 為了拯救更多人而犧牲一個無辜的人在道德上是否可以接受？
> - 效用原則是否證明為實現最大利益而採用任何手段都是正當的，還是存在不應違反的倫理約束？
> - 我們如何衡量不同生命的價值，進行這樣的計算是否公平？
>
> 您對電車難題和功利主義倫理學有什麼看法？您是否認為最大化效用應該是道德決策的主要目標，還是有其他重要的考慮因素？您會如何處理電車難題中呈現的兩難困境，什麼倫理原則會指導您的選擇？

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
        max_tokens=2000,
        temperature=1,
        system="您的任務是討論給定主題的哲學概念或思想實驗。簡要解釋該概念，呈現主要論點和含義，並通過提出開放性問題來鼓勵批判性思維。保持平衡、客觀的語調，培養智識好奇心。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "電車難題和功利主義倫理學"
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
      system: "您的任務是討論給定主題的哲學概念或思想實驗。簡要解釋該概念，呈現主要論點和含義，並通過提出開放性問題來鼓勵批判性思維。保持平衡、客觀的語調，培養智識好奇心。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "電車難題和功利主義倫理學"
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
        system="您的任務是討論給定主題的哲學概念或思想實驗。簡要解釋該概念，呈現主要論點和含義，並通過提出開放性問題來鼓勵批判性思維。保持平衡、客觀的語調，培養智識好奇心。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "電車難題和功利主義倫理學"
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
      system: "您的任務是討論給定主題的哲學概念或思想實驗。簡要解釋該概念，呈現主要論點和含義，並通過提出開放性問題來鼓勵批判性思維。保持平衡、客觀的語調，培養智識好奇心。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "電車難題和功利主義倫理學"
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
        system="您的任務是討論給定主題的哲學概念或思想實驗。簡要解釋該概念，呈現主要論點和含義，並通過提出開放性問題來鼓勵批判性思維。保持平衡、客觀的語調，培養智識好奇心。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "電車難題和功利主義倫理學"
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
      system: "您的任務是討論給定主題的哲學概念或思想實驗。簡要解釋該概念，呈現主要論點和含義，並通過提出開放性問題來鼓勵批判性思維。保持平衡、客觀的語調，培養智識好奇心。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "電車難題和功利主義倫理學"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>