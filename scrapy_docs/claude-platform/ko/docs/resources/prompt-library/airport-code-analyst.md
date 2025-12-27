# 공항 코드 분석기

텍스트에서 공항 코드를 찾고 추출합니다.

---

> 이 프롬프트를 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 제공된 텍스트를 분석하고 그 안에 언급된 공항 코드를 식별하는 것이 당신의 임무입니다. 이러한 공항 코드를 텍스트에 나타나는 순서대로 목록으로 제시하세요. 공항 코드가 발견되지 않으면 빈 목록을 반환하세요. |
| User   | 제 다음 여행은 시애틀에서 암스테르담으로 비행하는 것입니다. 암스테르담에서 며칠을 보낸 후 파리로 가서 로마로 가는 연결 항공편을 탈 예정입니다. |

### 예시 출력

> 다음은 텍스트에 언급된 공항 코드 목록으로, 나타나는 순서대로 정리했습니다:
>
> 1. SEA (시애틀)
> 2. AMS (암스테르담)
> 3. CDG (파리)
> 4. FCO (로마)

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
        system="제공된 텍스트를 분석하고 그 안에 언급된 공항 코드를 식별하는 것이 당신의 임무입니다. 이러한 공항 코드를 텍스트에 나타나는 순서대로 목록으로 제시하세요. 공항 코드가 발견되지 않으면 빈 목록을 반환하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "제 다음 여행은 시애틀에서 암스테르담으로 비행하는 것입니다. 암스테르담에서 며칠을 보낸 후 파리로 가서 로마로 가는 연결 항공편을 탈 예정입니다."
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
      system: "제공된 텍스트를 분석하고 그 안에 언급된 공항 코드를 식별하는 것이 당신의 임무입니다. 이러한 공항 코드를 텍스트에 나타나는 순서대로 목록으로 제시하세요. 공항 코드가 발견되지 않으면 빈 목록을 반환하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "제 다음 여행은 시애틀에서 암스테르담으로 비행하는 것입니다. 암스테르담에서 며칠을 보낸 후 파리로 가서 로마로 가는 연결 항공편을 탈 예정입니다."
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
        system="제공된 텍스트를 분석하고 그 안에 언급된 공항 코드를 식별하는 것이 당신의 임무입니다. 이러한 공항 코드를 텍스트에 나타나는 순서대로 목록으로 제시하세요. 공항 코드가 발견되지 않으면 빈 목록을 반환하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "제 다음 여행은 시애틀에서 암스테르담으로 비행하는 것입니다. 암스테르담에서 며칠을 보낸 후 파리로 가서 로마로 가는 연결 항공편을 탈 예정입니다."
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
      system: "제공된 텍스트를 분석하고 그 안에 언급된 공항 코드를 식별하는 것이 당신의 임무입니다. 이러한 공항 코드를 텍스트에 나타나는 순서대로 목록으로 제시하세요. 공항 코드가 발견되지 않으면 빈 목록을 반환하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "제 다음 여행은 시애틀에서 암스테르담으로 비행하는 것입니다. 암스테르담에서 며칠을 보낸 후 파리로 가서 로마로 가는 연결 항공편을 탈 예정입니다."
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
        system="제공된 텍스트를 분석하고 그 안에 언급된 공항 코드를 식별하는 것이 당신의 임무입니다. 이러한 공항 코드를 텍스트에 나타나는 순서대로 목록으로 제시하세요. 공항 코드가 발견되지 않으면 빈 목록을 반환하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "제 다음 여행은 시애틀에서 암스테르담으로 비행하는 것입니다. 암스테르담에서 며칠을 보낸 후 파리로 가서 로마로 가는 연결 항공편을 탈 예정입니다."
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
      system: "제공된 텍스트를 분석하고 그 안에 언급된 공항 코드를 식별하는 것이 당신의 임무입니다. 이러한 공항 코드를 텍스트에 나타나는 순서대로 목록으로 제시하세요. 공항 코드가 발견되지 않으면 빈 목록을 반환하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "제 다음 여행은 시애틀에서 암스테르담으로 비행하는 것입니다. 암스테르담에서 며칠을 보낸 후 파리로 가서 로마로 가는 연결 항공편을 탈 예정입니다."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>