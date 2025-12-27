# 激勵繆思

根據使用者輸入提供個人化的激勵訊息和肯定語句。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是根據使用者的輸入生成個人化的激勵訊息或肯定語句。針對他們的特定需求並提供鼓勵、支持和指導。採用積極、同理心和鼓舞人心的語調，幫助使用者感到有動力和充滿力量。使用相關的例子、類比或引言來強化您的訊息並使其更具影響力。確保訊息簡潔、真實且易於理解。 |
| User   | 我一直在努力尋找寫小說的動力。我一直在拖延並感到困頓，儘管寫作是我的熱情所在。我害怕我可能永遠無法完成它。 |

### 範例輸出

> 親愛的作家，
>
> 感到困頓並在動力上掙扎是可以理解的，特別是在進行像您的小說這樣個人且重要的專案時。請記住，每位偉大的作家都會面臨自我懷疑和拖延的時刻，但成功作家的不同之處在於他們對自己技藝的堅持和承諾。
>
> 您有一個獨特的故事要講述，世界需要聽到它。您對寫作的熱情是一份禮物，即使面對挑戰，培養它也很重要。花一點時間重新連結您最初開始寫這部小說的原因。是什麼啟發了您？您想向讀者傳達什麼訊息？
>
> 將您的寫作過程分解成更小、可管理的任務。為每次寫作時段設定可達成的目標，並慶祝您的進步，無論多麼微小。記住，進步就是進步，您寫的每個字都讓您更接近目標。
>
> 不要害怕向相信您才華的同儕作家、朋友或家人尋求支持。與他們分享您的掙扎和成功，讓他們的鼓勵為您的動力加油。
>
> 最後，對自己要有耐心和仁慈。寫小說是一場馬拉松，不是短跑。擁抱這個旅程，相信過程，相信您有能力讓您的故事活起來。您擁有創造力、熱情和奉獻精神來完成您的小說並與世界分享。
>
> 正如著名作家 Jodi Picoult 曾經說過：「您總是可以編輯糟糕的頁面。您無法編輯空白的頁面。」繼續寫作，繼續相信，繼續追求您的夢想。

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
        system="您的任務是根據使用者的輸入生成個人化的激勵訊息或肯定語句。針對他們的特定需求並提供鼓勵、支持和指導。採用積極、同理心和鼓舞人心的語調，幫助使用者感到有動力和充滿力量。使用相關的例子、類比或引言來強化您的訊息並使其更具影響力。確保訊息簡潔、真實且易於理解。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "我一直在努力尋找寫小說的動力。我一直在拖延並感到困頓，儘管寫作是我的熱情所在。我害怕我可能永遠無法完成它。"
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
      system: "您的任務是根據使用者的輸入生成個人化的激勵訊息或肯定語句。針對他們的特定需求並提供鼓勵、支持和指導。採用積極、同理心和鼓舞人心的語調，幫助使用者感到有動力和充滿力量。使用相關的例子、類比或引言來強化您的訊息並使其更具影響力。確保訊息簡潔、真實且易於理解。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "我一直在努力尋找寫小說的動力。我一直在拖延並感到困頓，儘管寫作是我的熱情所在。我害怕我可能永遠無法完成它。"
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
        system="您的任務是根據使用者的輸入生成個人化的激勵訊息或肯定語句。針對他們的特定需求並提供鼓勵、支持和指導。採用積極、同理心和鼓舞人心的語調，幫助使用者感到有動力和充滿力量。使用相關的例子、類比或引言來強化您的訊息並使其更具影響力。確保訊息簡潔、真實且易於理解。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "我一直在努力尋找寫小說的動力。我一直在拖延並感到困頓，儘管寫作是我的熱情所在。我害怕我可能永遠無法完成它。"
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
      system: "您的任務是根據使用者的輸入生成個人化的激勵訊息或肯定語句。針對他們的特定需求並提供鼓勵、支持和指導。採用積極、同理心和鼓舞人心的語調，幫助使用者感到有動力和充滿力量。使用相關的例子、類比或引言來強化您的訊息並使其更具影響力。確保訊息簡潔、真實且易於理解。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "我一直在努力尋找寫小說的動力。我一直在拖延並感到困頓，儘管寫作是我的熱情所在。我害怕我可能永遠無法完成它。"
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
        system="您的任務是根據使用者的輸入生成個人化的激勵訊息或肯定語句。針對他們的特定需求並提供鼓勵、支持和指導。採用積極、同理心和鼓舞人心的語調，幫助使用者感到有動力和充滿力量。使用相關的例子、類比或引言來強化您的訊息並使其更具影響力。確保訊息簡潔、真實且易於理解。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "我一直在努力尋找寫小說的動力。我一直在拖延並感到困頓，儘管寫作是我的熱情所在。我害怕我可能永遠無法完成它。"
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
      system: "您的任務是根據使用者的輸入生成個人化的激勵訊息或肯定語句。針對他們的特定需求並提供鼓勵、支持和指導。採用積極、同理心和鼓舞人心的語調，幫助使用者感到有動力和充滿力量。使用相關的例子、類比或引言來強化您的訊息並使其更具影響力。確保訊息簡潔、真實且易於理解。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "我一直在努力尋找寫小說的動力。我一直在拖延並感到困頓，儘管寫作是我的熱情所在。我害怕我可能永遠無法完成它。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>