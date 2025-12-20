# 제품명 작성 전문가

설명과 키워드로부터 매력적인 제품명을 만들어보세요.

---

> 이 프롬프트를 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 제공된 설명과 키워드를 바탕으로 창의적이고 기억에 남으며 마케팅 가능한 제품명을 생성하는 것이 당신의 임무입니다. 제품명은 간결하고(2-4단어), 연상시키며, 대상 고객이 쉽게 이해할 수 있어야 합니다. 일반적이거나 지나치게 직설적인 이름은 피하세요. 대신 눈에 띄고, 제품의 본질을 포착하며, 지속적인 인상을 남기는 이름을 만드는 것을 목표로 하세요. |
| User   | 설명: 20시간 배터리 수명과 터치 컨트롤을 갖춘 노이즈 캔슬링, 무선, 오버이어 헤드폰. 오디오파일과 잦은 여행자를 위해 설계됨. 키워드: 몰입감, 편안함, 고음질, 오래 지속, 편리함 |

## 예시 출력

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

### API 요청

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
        system="제공된 설명과 키워드를 바탕으로 창의적이고 기억에 남으며 마케팅 가능한 제품명을 생성하는 것이 당신의 임무입니다. 제품명은 간결하고(2-4단어), 연상시키며, 대상 고객이 쉽게 이해할 수 있어야 합니다. 일반적이거나 지나치게 직설적인 이름은 피하세요. 대신 눈에 띄고, 제품의 본질을 포착하며, 지속적인 인상을 남기는 이름을 만드는 것을 목표로 하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "설명: 20시간 배터리 수명과 터치 컨트롤을 갖춘 노이즈 캔슬링, 무선, 오버이어 헤드폰. 오디오파일과 잦은 여행자를 위해 설계됨.  \n  \n키워드: 몰입감, 편안함, 고음질, 오래 지속, 편리함"
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
      system: "제공된 설명과 키워드를 바탕으로 창의적이고 기억에 남으며 마케팅 가능한 제품명을 생성하는 것이 당신의 임무입니다. 제품명은 간결하고(2-4단어), 연상시키며, 대상 고객이 쉽게 이해할 수 있어야 합니다. 일반적이거나 지나치게 직설적인 이름은 피하세요. 대신 눈에 띄고, 제품의 본질을 포착하며, 지속적인 인상을 남기는 이름을 만드는 것을 목표로 하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "설명: 20시간 배터리 수명과 터치 컨트롤을 갖춘 노이즈 캔슬링, 무선, 오버이어 헤드폰. 오디오파일과 잦은 여행자를 위해 설계됨.  \n  \n키워드: 몰입감, 편안함, 고음질, 오래 지속, 편리함"
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
        system="제공된 설명과 키워드를 바탕으로 창의적이고 기억에 남으며 마케팅 가능한 제품명을 생성하는 것이 당신의 임무입니다. 제품명은 간결하고(2-4단어), 연상시키며, 대상 고객이 쉽게 이해할 수 있어야 합니다. 일반적이거나 지나치게 직설적인 이름은 피하세요. 대신 눈에 띄고, 제품의 본질을 포착하며, 지속적인 인상을 남기는 이름을 만드는 것을 목표로 하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "설명: 20시간 배터리 수명과 터치 컨트롤을 갖춘 노이즈 캔슬링, 무선, 오버이어 헤드폰. 오디오파일과 잦은 여행자를 위해 설계됨.  \n  \n키워드: 몰입감, 편안함, 고음질, 오래 지속, 편리함"
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
      system: "제공된 설명과 키워드를 바탕으로 창의적이고 기억에 남으며 마케팅 가능한 제품명을 생성하는 것이 당신의 임무입니다. 제품명은 간결하고(2-4단어), 연상시키며, 대상 고객이 쉽게 이해할 수 있어야 합니다. 일반적이거나 지나치게 직설적인 이름은 피하세요. 대신 눈에 띄고, 제품의 본질을 포착하며, 지속적인 인상을 남기는 이름을 만드는 것을 목표로 하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "설명: 20시간 배터리 수명과 터치 컨트롤을 갖춘 노이즈 캔슬링, 무선, 오버이어 헤드폰. 오디오파일과 잦은 여행자를 위해 설계됨.  \n  \n키워드: 몰입감, 편안함, 고음질, 오래 지속, 편리함"
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
        system="제공된 설명과 키워드를 바탕으로 창의적이고 기억에 남으며 마케팅 가능한 제품명을 생성하는 것이 당신의 임무입니다. 제품명은 간결하고(2-4단어), 연상시키며, 대상 고객이 쉽게 이해할 수 있어야 합니다. 일반적이거나 지나치게 직설적인 이름은 피하세요. 대신 눈에 띄고, 제품의 본질을 포착하며, 지속적인 인상을 남기는 이름을 만드는 것을 목표로 하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "설명: 20시간 배터리 수명과 터치 컨트롤을 갖춘 노이즈 캔슬링, 무선, 오버이어 헤드폰. 오디오파일과 잦은 여행자를 위해 설계됨.\n\n키워드: 몰입감, 편안함, 고음질, 오래 지속, 편리함"
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
      system: "제공된 설명과 키워드를 바탕으로 창의적이고 기억에 남으며 마케팅 가능한 제품명을 생성하는 것이 당신의 임무입니다. 제품명은 간결하고(2-4단어), 연상시키며, 대상 고객이 쉽게 이해할 수 있어야 합니다. 일반적이거나 지나치게 직설적인 이름은 피하세요. 대신 눈에 띄고, 제품의 본질을 포착하며, 지속적인 인상을 남기는 이름을 만드는 것을 목표로 하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "설명: 20시간 배터리 수명과 터치 컨트롤을 갖춘 노이즈 캔슬링, 무선, 오버이어 헤드폰. 오디오파일과 잦은 여행자를 위해 설계됨.\n\n키워드: 몰입감, 편안함, 고음질, 오래 지속, 편리함"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>