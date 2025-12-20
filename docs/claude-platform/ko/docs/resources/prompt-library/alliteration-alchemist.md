# 두운 연금술사

주어진 주제에 대한 두운 구문과 문장을 생성합니다.

---

> 이 프롬프트를 우리 개발자 [Console](/dashboard)에 복사해서 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 주어진 주제에 대한 두운 구문과 문장을 만드는 것이 당신의 임무입니다. 두운이 듣기 좋을 뿐만 아니라 주제와 관련된 정보를 전달하거나 적절한 감정을 불러일으키도록 하세요. |
| User   | 바다 |

### 예시 출력

> 1.  속삭이는 파도가 해안을 씻어낸다.
> 2.  짠 바다 물보라가 모래 위에 튄다.
> 3.  부서지는 해류가 해안 절벽을 깎아낸다.
> 4.  깊고 어두운 심연이 숨겨진 위험을 감춘다.
> 5.  거품이는 개울이 해변에 생명을 가져다준다.
> 6.  강력하고 장엄한 포유동물들이 바다를 통해 이동한다.
> 7.  천둥 같은 조수가 배들을 이리저리 던진다.
> 8.  반짝이는 얕은 바다가 비밀스러운 바다 생물들을 보호한다.
> 9.  잔물결 반사가 수면 아래 세계를 드러낸다.
> 10. 소용돌이치고 솟구치는 바다가 해안선을 만든다.
> 11. 거품이 일고 끓어오르는 빙하가 자유롭게 흐른다.
> 12. 우아한 갈매기들이 반짝이는 바다 위를 활공한다.
> 13. 장난기 많은 돌고래들이 깨끗한 바다를 뛰어다닌다.
> 14. 짠 바람이 끝없는 푸른 바다를 가로질러 분다.
> 15. 리듬감 있는 잔물결이 바위 해안을 향해 굴러간다.
> 16. 산호 동굴이 다채로운 수중 공동체를 숨긴다.
> 17. 폭풍우 하늘과 부풀어 오르는 바다가 자연의 힘을 알린다.
> 18. 굴러가는 조수가 고요한 만을 변화시킨다.
> 19. 소용돌이치고 쌩쌩거리는 바람이 파도를 일으킨다.
> 20. 고요하고 사파이어 같은 바다가 지평선까지 뻗어있다.

---

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
        system="주어진 주제에 대한 두운 구문과 문장을 만드는 것이 당신의 임무입니다. 두운이 듣기 좋을 뿐만 아니라 주제와 관련된 정보를 전달하거나 적절한 감정을 불러일으키도록 하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "바다"
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
      system: "주어진 주제에 대한 두운 구문과 문장을 만드는 것이 당신의 임무입니다. 두운이 듣기 좋을 뿐만 아니라 주제와 관련된 정보를 전달하거나 적절한 감정을 불러일으키도록 하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "바다"
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
        system="주어진 주제에 대한 두운 구문과 문장을 만드는 것이 당신의 임무입니다. 두운이 듣기 좋을 뿐만 아니라 주제와 관련된 정보를 전달하거나 적절한 감정을 불러일으키도록 하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "바다"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    # See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 1000,
      temperature: 1,
      system: "주어진 주제에 대한 두운 구문과 문장을 만드는 것이 당신의 임무입니다. 두운이 듣기 좋을 뿐만 아니라 주제와 관련된 정보를 전달하거나 적절한 감정을 불러일으키도록 하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "바다"
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
        system="주어진 주제에 대한 두운 구문과 문장을 만드는 것이 당신의 임무입니다. 두운이 듣기 좋을 뿐만 아니라 주제와 관련된 정보를 전달하거나 적절한 감정을 불러일으키도록 하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "바다"
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
      system: "주어진 주제에 대한 두운 구문과 문장을 만드는 것이 당신의 임무입니다. 두운이 듣기 좋을 뿐만 아니라 주제와 관련된 정보를 전달하거나 적절한 감정을 불러일으키도록 하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "바다"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>