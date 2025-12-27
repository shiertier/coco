# 트윗 톤 감지기

트윗 뒤에 숨겨진 톤과 감정을 감지합니다.

---

> 이 프롬프트를 개발자 [Console](/dashboard)에 복사해서 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 당신의 작업은 제공된 트윗을 분석하고 작성자가 표현한 주요 톤과 감정을 식별하는 것입니다. 톤은 다음 중 하나로 분류되어야 합니다: 긍정적, 부정적, 중립적, 유머러스, 비꼬는, 열정적, 화난, 또는 정보적. 감정은 긍정적, 부정적, 또는 중립적으로 분류되어야 합니다. 당신의 분류에 대한 간단한 설명을 제공하고, 당신의 결정에 영향을 준 핵심 단어, 구문, 이모티콘, 또는 기타 요소들을 강조하세요. |
| User   | 와, 이 위기에 대한 회사의 대응에 정말 감명받았어요. 🙄 그들은 정말 우선순위를 제대로 알고 있네요. #sarcasm #fail |

### 예시 출력

> 톤: 비꼬는
> 감정: 부정적

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
        temperature=0,
        system="당신의 작업은 제공된 트윗을 분석하고 작성자가 표현한 주요 톤과 감정을 식별하는 것입니다. 톤은 다음 중 하나로 분류되어야 합니다: 긍정적, 부정적, 중립적, 유머러스, 비꼬는, 열정적, 화난, 또는 정보적. 감정은 긍정적, 부정적, 또는 중립적으로 분류되어야 합니다. 당신의 분류에 대한 간단한 설명을 제공하고, 당신의 결정에 영향을 준 핵심 단어, 구문, 이모티콘, 또는 기타 요소들을 강조하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "와, 이 위기에 대한 회사의 대응에 정말 감명받았어요. 🙄 그들은 정말 우선순위를 제대로 알고 있네요. #sarcasm #fail"
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
      system: "당신의 작업은 제공된 트윗을 분석하고 작성자가 표현한 주요 톤과 감정을 식별하는 것입니다. 톤은 다음 중 하나로 분류되어야 합니다: 긍정적, 부정적, 중립적, 유머러스, 비꼬는, 열정적, 화난, 또는 정보적. 감정은 긍정적, 부정적, 또는 중립적으로 분류되어야 합니다. 당신의 분류에 대한 간단한 설명을 제공하고, 당신의 결정에 영향을 준 핵심 단어, 구문, 이모티콘, 또는 기타 요소들을 강조하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "와, 이 위기에 대한 회사의 대응에 정말 감명받았어요. 🙄 그들은 정말 우선순위를 제대로 알고 있네요. #sarcasm #fail"
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
        system="당신의 작업은 제공된 트윗을 분석하고 작성자가 표현한 주요 톤과 감정을 식별하는 것입니다. 톤은 다음 중 하나로 분류되어야 합니다: 긍정적, 부정적, 중립적, 유머러스, 비꼬는, 열정적, 화난, 또는 정보적. 감정은 긍정적, 부정적, 또는 중립적으로 분류되어야 합니다. 당신의 분류에 대한 간단한 설명을 제공하고, 당신의 결정에 영향을 준 핵심 단어, 구문, 이모티콘, 또는 기타 요소들을 강조하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "와, 이 위기에 대한 회사의 대응에 정말 감명받았어요. 🙄 그들은 정말 우선순위를 제대로 알고 있네요. #sarcasm #fail"
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
      system: "당신의 작업은 제공된 트윗을 분석하고 작성자가 표현한 주요 톤과 감정을 식별하는 것입니다. 톤은 다음 중 하나로 분류되어야 합니다: 긍정적, 부정적, 중립적, 유머러스, 비꼬는, 열정적, 화난, 또는 정보적. 감정은 긍정적, 부정적, 또는 중립적으로 분류되어야 합니다. 당신의 분류에 대한 간단한 설명을 제공하고, 당신의 결정에 영향을 준 핵심 단어, 구문, 이모티콘, 또는 기타 요소들을 강조하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "와, 이 위기에 대한 회사의 대응에 정말 감명받았어요. 🙄 그들은 정말 우선순위를 제대로 알고 있네요. #sarcasm #fail"
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
        system="당신의 작업은 제공된 트윗을 분석하고 작성자가 표현한 주요 톤과 감정을 식별하는 것입니다. 톤은 다음 중 하나로 분류되어야 합니다: 긍정적, 부정적, 중립적, 유머러스, 비꼬는, 열정적, 화난, 또는 정보적. 감정은 긍정적, 부정적, 또는 중립적으로 분류되어야 합니다. 당신의 분류에 대한 간단한 설명을 제공하고, 당신의 결정에 영향을 준 핵심 단어, 구문, 이모티콘, 또는 기타 요소들을 강조하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "와, 이 위기에 대한 회사의 대응에 정말 감명받았어요. 🙄 그들은 정말 우선순위를 제대로 알고 있네요. #sarcasm #fail"
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
      system: "당신의 작업은 제공된 트윗을 분석하고 작성자가 표현한 주요 톤과 감정을 식별하는 것입니다. 톤은 다음 중 하나로 분류되어야 합니다: 긍정적, 부정적, 중립적, 유머러스, 비꼬는, 열정적, 화난, 또는 정보적. 감정은 긍정적, 부정적, 또는 중립적으로 분류되어야 합니다. 당신의 분류에 대한 간단한 설명을 제공하고, 당신의 결정에 영향을 준 핵심 단어, 구문, 이모티콘, 또는 기타 요소들을 강조하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "와, 이 위기에 대한 회사의 대응에 정말 감명받았어요. 🙄 그들은 정말 우선순위를 제대로 알고 있네요. #sarcasm #fail"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>