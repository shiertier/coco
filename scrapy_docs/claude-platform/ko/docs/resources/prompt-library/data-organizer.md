# 데이터 정리기

비구조화된 텍스트를 맞춤형 JSON 테이블로 변환합니다.

---

> 이 프롬프트를 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 당신의 임무는 제공된 비구조화된 텍스트를 받아서 JSON을 사용하여 잘 정리된 테이블 형식으로 변환하는 것입니다. 텍스트에서 언급된 주요 엔티티, 속성 또는 카테고리를 식별하고 이를 JSON 객체의 키로 사용하세요. 그런 다음 텍스트에서 관련 정보를 추출하여 JSON 객체의 해당 값을 채우세요. 데이터가 정확하게 표현되고 JSON 구조 내에서 적절하게 형식화되었는지 확인하세요. 결과 JSON 테이블은 원본 텍스트에 제시된 정보의 명확하고 구조화된 개요를 제공해야 합니다. |
| User   | Silvermist Hollow는 매력적인 마을로, 특별한 개인들의 그룹이 살고 있었습니다. 그 중에는 45세의 Yale에서 교육받은 신경외과 의사인 Dr. Liam Patel이 있었는데, 그는 지역 의료센터에서 수술 기법을 혁신했습니다. 28세의 Olivia Chen은 UC Berkeley 출신의 혁신적인 건축가로, 지속 가능하고 숨막히게 아름다운 디자인으로 마을의 풍경을 변화시켰습니다. 지역 극장은 72세의 Juilliard에서 훈련받은 음악가이자 작곡가인 Ethan Kovacs의 매혹적인 교향곡으로 빛났습니다. Isabella Torres는 지역에서 조달한 재료에 대한 열정을 가진 독학 요리사로, 농장에서 식탁까지의 레스토랑으로 요리 센세이션을 일으켜 음식 애호가들이 꼭 방문해야 할 목적지가 되었습니다. 각각 독특한 재능을 가진 이 놀라운 개인들은 Silvermist Hollow의 활기찬 삶의 태피스트리에 기여했습니다. |

### 예시 출력

```json
[
  {
    "name": "Dr. Liam Patel",
    "age": 45,
    "profession": "신경외과 의사",
    "education": "Yale",
    "accomplishments": "지역 의료센터에서 수술 기법을 혁신함"
  },
  {
    "name": "Olivia Chen",
    "age": 28,
    "profession": "건축가",
    "education": "UC Berkeley",
    "accomplishments": "지속 가능하고 숨막히게 아름다운 디자인으로 마을의 풍경을 변화시킴"
  },
  {
    "name": "Ethan Kovacs",
    "age": 72,
    "profession": "음악가 및 작곡가",
    "education": "Juilliard",
    "accomplishments": "매혹적인 교향곡으로 지역 극장을 빛냄"
  },
  {
    "name": "Isabella Torres",
    "age": null,
    "profession": "요리사",
    "education": "독학",
    "accomplishments": "농장에서 식탁까지의 레스토랑으로 요리 센세이션을 일으켜 음식 애호가들이 꼭 방문해야 할 목적지가 됨"
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
        system="당신의 임무는 제공된 비구조화된 텍스트를 받아서 JSON을 사용하여 잘 정리된 테이블 형식으로 변환하는 것입니다. 텍스트에서 언급된 주요 엔티티, 속성 또는 카테고리를 식별하고 이를 JSON 객체의 키로 사용하세요. 그런 다음 텍스트에서 관련 정보를 추출하여 JSON 객체의 해당 값을 채우세요. 데이터가 정확하게 표현되고 JSON 구조 내에서 적절하게 형식화되었는지 확인하세요. 결과 JSON 테이블은 원본 텍스트에 제시된 정보의 명확하고 구조화된 개요를 제공해야 합니다.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow는 매력적인 마을로, 특별한 개인들의 그룹이 살고 있었습니다. 그 중에는 45세의 Yale에서 교육받은 신경외과 의사인 Dr. Liam Patel이 있었는데, 그는 지역 의료센터에서 수술 기법을 혁신했습니다. 28세의 Olivia Chen은 UC Berkeley 출신의 혁신적인 건축가로, 지속 가능하고 숨막히게 아름다운 디자인으로 마을의 풍경을 변화시켰습니다. 지역 극장은 72세의 Juilliard에서 훈련받은 음악가이자 작곡가인 Ethan Kovacs의 매혹적인 교향곡으로 빛났습니다. Isabella Torres는 지역에서 조달한 재료에 대한 열정을 가진 독학 요리사로, 농장에서 식탁까지의 레스토랑으로 요리 센세이션을 일으켜 음식 애호가들이 꼭 방문해야 할 목적지가 되었습니다. 각각 독특한 재능을 가진 이 놀라운 개인들은 Silvermist Hollow의 활기찬 삶의 태피스트리에 기여했습니다."
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
      system: "당신의 임무는 제공된 비구조화된 텍스트를 받아서 JSON을 사용하여 잘 정리된 테이블 형식으로 변환하는 것입니다. 텍스트에서 언급된 주요 엔티티, 속성 또는 카테고리를 식별하고 이를 JSON 객체의 키로 사용하세요. 그런 다음 텍스트에서 관련 정보를 추출하여 JSON 객체의 해당 값을 채우세요. 데이터가 정확하게 표현되고 JSON 구조 내에서 적절하게 형식화되었는지 확인하세요. 결과 JSON 테이블은 원본 텍스트에 제시된 정보의 명확하고 구조화된 개요를 제공해야 합니다.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow는 매력적인 마을로, 특별한 개인들의 그룹이 살고 있었습니다. 그 중에는 45세의 Yale에서 교육받은 신경외과 의사인 Dr. Liam Patel이 있었는데, 그는 지역 의료센터에서 수술 기법을 혁신했습니다. 28세의 Olivia Chen은 UC Berkeley 출신의 혁신적인 건축가로, 지속 가능하고 숨막히게 아름다운 디자인으로 마을의 풍경을 변화시켰습니다. 지역 극장은 72세의 Juilliard에서 훈련받은 음악가이자 작곡가인 Ethan Kovacs의 매혹적인 교향곡으로 빛났습니다. Isabella Torres는 지역에서 조달한 재료에 대한 열정을 가진 독학 요리사로, 농장에서 식탁까지의 레스토랑으로 요리 센세이션을 일으켜 음식 애호가들이 꼭 방문해야 할 목적지가 되었습니다. 각각 독특한 재능을 가진 이 놀라운 개인들은 Silvermist Hollow의 활기찬 삶의 태피스트리에 기여했습니다."
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
        system="당신의 임무는 제공된 비구조화된 텍스트를 받아서 JSON을 사용하여 잘 정리된 테이블 형식으로 변환하는 것입니다. 텍스트에서 언급된 주요 엔티티, 속성 또는 카테고리를 식별하고 이를 JSON 객체의 키로 사용하세요. 그런 다음 텍스트에서 관련 정보를 추출하여 JSON 객체의 해당 값을 채우세요. 데이터가 정확하게 표현되고 JSON 구조 내에서 적절하게 형식화되었는지 확인하세요. 결과 JSON 테이블은 원본 텍스트에 제시된 정보의 명확하고 구조화된 개요를 제공해야 합니다.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow는 매력적인 마을로, 특별한 개인들의 그룹이 살고 있었습니다. 그 중에는 45세의 Yale에서 교육받은 신경외과 의사인 Dr. Liam Patel이 있었는데, 그는 지역 의료센터에서 수술 기법을 혁신했습니다. 28세의 Olivia Chen은 UC Berkeley 출신의 혁신적인 건축가로, 지속 가능하고 숨막히게 아름다운 디자인으로 마을의 풍경을 변화시켰습니다. 지역 극장은 72세의 Juilliard에서 훈련받은 음악가이자 작곡가인 Ethan Kovacs의 매혹적인 교향곡으로 빛났습니다. Isabella Torres는 지역에서 조달한 재료에 대한 열정을 가진 독학 요리사로, 농장에서 식탁까지의 레스토랑으로 요리 센세이션을 일으켜 음식 애호가들이 꼭 방문해야 할 목적지가 되었습니다. 각각 독특한 재능을 가진 이 놀라운 개인들은 Silvermist Hollow의 활기찬 삶의 태피스트리에 기여했습니다."
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
      system: "당신의 임무는 제공된 비구조화된 텍스트를 받아서 JSON을 사용하여 잘 정리된 테이블 형식으로 변환하는 것입니다. 텍스트에서 언급된 주요 엔티티, 속성 또는 카테고리를 식별하고 이를 JSON 객체의 키로 사용하세요. 그런 다음 텍스트에서 관련 정보를 추출하여 JSON 객체의 해당 값을 채우세요. 데이터가 정확하게 표현되고 JSON 구조 내에서 적절하게 형식화되었는지 확인하세요. 결과 JSON 테이블은 원본 텍스트에 제시된 정보의 명확하고 구조화된 개요를 제공해야 합니다.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow는 매력적인 마을로, 특별한 개인들의 그룹이 살고 있었습니다. 그 중에는 45세의 Yale에서 교육받은 신경외과 의사인 Dr. Liam Patel이 있었는데, 그는 지역 의료센터에서 수술 기법을 혁신했습니다. 28세의 Olivia Chen은 UC Berkeley 출신의 혁신적인 건축가로, 지속 가능하고 숨막히게 아름다운 디자인으로 마을의 풍경을 변화시켰습니다. 지역 극장은 72세의 Juilliard에서 훈련받은 음악가이자 작곡가인 Ethan Kovacs의 매혹적인 교향곡으로 빛났습니다. Isabella Torres는 지역에서 조달한 재료에 대한 열정을 가진 독학 요리사로, 농장에서 식탁까지의 레스토랑으로 요리 센세이션을 일으켜 음식 애호가들이 꼭 방문해야 할 목적지가 되었습니다. 각각 독특한 재능을 가진 이 놀라운 개인들은 Silvermist Hollow의 활기찬 삶의 태피스트리에 기여했습니다."
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
        system="당신의 임무는 제공된 비구조화된 텍스트를 받아서 JSON을 사용하여 잘 정리된 테이블 형식으로 변환하는 것입니다. 텍스트에서 언급된 주요 엔티티, 속성 또는 카테고리를 식별하고 이를 JSON 객체의 키로 사용하세요. 그런 다음 텍스트에서 관련 정보를 추출하여 JSON 객체의 해당 값을 채우세요. 데이터가 정확하게 표현되고 JSON 구조 내에서 적절하게 형식화되었는지 확인하세요. 결과 JSON 테이블은 원본 텍스트에 제시된 정보의 명확하고 구조화된 개요를 제공해야 합니다.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow는 매력적인 마을로, 특별한 개인들의 그룹이 살고 있었습니다. 그 중에는 45세의 Yale에서 교육받은 신경외과 의사인 Dr. Liam Patel이 있었는데, 그는 지역 의료센터에서 수술 기법을 혁신했습니다. 28세의 Olivia Chen은 UC Berkeley 출신의 혁신적인 건축가로, 지속 가능하고 숨막히게 아름다운 디자인으로 마을의 풍경을 변화시켰습니다. 지역 극장은 72세의 Juilliard에서 훈련받은 음악가이자 작곡가인 Ethan Kovacs의 매혹적인 교향곡으로 빛났습니다. Isabella Torres는 지역에서 조달한 재료에 대한 열정을 가진 독학 요리사로, 농장에서 식탁까지의 레스토랑으로 요리 센세이션을 일으켜 음식 애호가들이 꼭 방문해야 할 목적지가 되었습니다. 각각 독특한 재능을 가진 이 놀라운 개인들은 Silvermist Hollow의 활기찬 삶의 태피스트리에 기여했습니다."
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
      system: "당신의 임무는 제공된 비구조화된 텍스트를 받아서 JSON을 사용하여 잘 정리된 테이블 형식으로 변환하는 것입니다. 텍스트에서 언급된 주요 엔티티, 속성 또는 카테고리를 식별하고 이를 JSON 객체의 키로 사용하세요. 그런 다음 텍스트에서 관련 정보를 추출하여 JSON 객체의 해당 값을 채우세요. 데이터가 정확하게 표현되고 JSON 구조 내에서 적절하게 형식화되었는지 확인하세요. 결과 JSON 테이블은 원본 텍스트에 제시된 정보의 명확하고 구조화된 개요를 제공해야 합니다.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow는 매력적인 마을로, 특별한 개인들의 그룹이 살고 있었습니다. 그 중에는 45세의 Yale에서 교육받은 신경외과 의사인 Dr. Liam Patel이 있었는데, 그는 지역 의료센터에서 수술 기법을 혁신했습니다. 28세의 Olivia Chen은 UC Berkeley 출신의 혁신적인 건축가로, 지속 가능하고 숨막히게 아름다운 디자인으로 마을의 풍경을 변화시켰습니다. 지역 극장은 72세의 Juilliard에서 훈련받은 음악가이자 작곡가인 Ethan Kovacs의 매혹적인 교향곡으로 빛났습니다. Isabella Torres는 지역에서 조달한 재료에 대한 열정을 가진 독학 요리사로, 농장에서 식탁까지의 레스토랑으로 요리 센세이션을 일으켜 음식 애호가들이 꼭 방문해야 할 목적지가 되었습니다. 각각 독특한 재능을 가진 이 놀라운 개인들은 Silvermist Hollow의 활기찬 삶의 태피스트리에 기여했습니다."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>