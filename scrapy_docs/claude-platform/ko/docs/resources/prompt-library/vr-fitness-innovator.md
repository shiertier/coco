# VR 피트니스 혁신가

가상현실 피트니스 게임을 위한 창의적인 아이디어를 브레인스토밍하세요.

---

> 이 프롬프트를 우리의 개발자 [Console](/dashboard)에 복사해서 직접 시도해보세요!

| 역할 | 내용 |
| ---- | --- |
| 사용자 | 당신의 임무는 가상현실(VR) 피트니스 게임을 위한 혁신적이고 매력적인 아이디어 목록을 생성하는 것입니다. 다양한 게임 장르, 독특한 게임플레이 메커니즘, 몰입형 환경, 그리고 신체 운동과 움직임을 통합하는 창의적인 방법들을 고려하세요. 아이디어들은 광범위한 피트니스 애호가들과 게이머들에게 매력적이어야 하며, VR에서 운동하면서 활동적이고 재미있게 지낼 수 있도록 격려해야 합니다. 각 아이디어에 대해 게임 컨셉, 주요 기능, 그리고 피트니스를 어떻게 촉진하는지에 대한 간단한 설명을 제공하세요. |

## 예시 출력

1.  아이디어: VR 파쿠르 어드벤처  
    설명: 플레이어들은 역동적인 도시 환경을 탐색하며, 달리고, 점프하고, 오르고, 파쿠르 스턴트를 수행하여 장애물을 극복하고 목표에 도달합니다. 게임은 플레이어의 움직임에 적응하여 가상 도시를 탐험하면서 강렬한 전신 운동을 제공합니다.  
    주요 기능: 현실적인 파쿠르 메커니즘, 절차적으로 생성된 환경, 멀티플레이어 레이스, 그리고 도전과제.  
    피트니스 측면: 유산소 운동, 민첩성, 협응력, 그리고 하체 근력.
2.  아이디어: 리듬 복싱  
    설명: 인기 음악 트랙의 비트에 맞춘 고에너지 복싱 게임입니다. 플레이어들은 리듬에 맞춰 펀치하고, 피하고, 막아서 점수를 얻고 점점 더 도전적인 레벨을 통과해야 합니다. 게임은 다양한 복싱 스타일과 훈련 모드를 제공합니다.  
    주요 기능: 라이센스 음악, 커스터마이징 가능한 복서, 온라인 멀티플레이어, 그리고 다양한 복싱 기술.  
    피트니스 측면: 유산소 운동, 상체 근력, 반사신경, 그리고 지구력.
3.  아이디어: VR 피트니스 RPG  
    설명: 플레이어들이 자신만의 캐릭터를 만들고 판타지 세계를 구원하는 퀘스트를 시작하는 몰입형 롤플레잉 게임입니다. 게임은 전통적인 RPG 요소와 피트니스 도전과제를 결합하여, 플레이어들이 마법을 시전하고, 적을 물리치고, 캐릭터를 레벨업하기 위해 신체 운동을 수행하도록 요구합니다.  
    주요 기능: 캐릭터 커스터마이징, 스킬 트리, 서사적인 보스 전투, 그리고 근력, 유산소, 유연성 운동의 혼합.  
    피트니스 측면: 전신 운동, 근력 훈련, 유산소 운동, 그리고 유연성.

---

## API 요청

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic(
  # defaults to os.environ.get("ANTHROPIC_API_KEY")
  api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=1000,
  temperature=1,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "당신의 임무는 가상현실(VR) 피트니스 게임을 위한 혁신적이고 매력적인 아이디어 목록을 생성하는 것입니다. 다양한 게임 장르, 독특한 게임플레이 메커니즘, 몰입형 환경, 그리고 신체 운동과 움직임을 통합하는 창의적인 방법들을 고려하세요. 아이디어들은 광범위한 피트니스 애호가들과 게이머들에게 매력적이어야 하며, VR에서 운동하면서 활동적이고 재미있게 지낼 수 있도록 격려해야 합니다. 각 아이디어에 대해 게임 컨셉, 주요 기능, 그리고 피트니스를 어떻게 촉진하는지에 대한 간단한 설명을 제공하세요."
        }
      ]
    }
  ]
)
print(message.content)

```

</Tab>
<Tab title="TypeScript">

```typescript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic({
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "당신의 임무는 가상현실(VR) 피트니스 게임을 위한 혁신적이고 매력적인 아이디어 목록을 생성하는 것입니다. 다양한 게임 장르, 독특한 게임플레이 메커니즘, 몰입형 환경, 그리고 신체 운동과 움직임을 통합하는 창의적인 방법들을 고려하세요. 아이디어들은 광범위한 피트니스 애호가들과 게이머들에게 매력적이어야 하며, VR에서 운동하면서 활동적이고 재미있게 지낼 수 있도록 격려해야 합니다. 각 아이디어에 대해 게임 컨셉, 주요 기능, 그리고 피트니스를 어떻게 촉진하는지에 대한 간단한 설명을 제공하세요."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">

```python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=1000,
    temperature=1,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "당신의 임무는 가상현실(VR) 피트니스 게임을 위한 혁신적이고 매력적인 아이디어 목록을 생성하는 것입니다. 다양한 게임 장르, 독특한 게임플레이 메커니즘, 몰입형 환경, 그리고 신체 운동과 움직임을 통합하는 창의적인 방법들을 고려하세요. 아이디어들은 광범위한 피트니스 애호가들과 게이머들에게 매력적이어야 하며, VR에서 운동하면서 활동적이고 재미있게 지낼 수 있도록 격려해야 합니다. 각 아이디어에 대해 게임 컨셉, 주요 기능, 그리고 피트니스를 어떻게 촉진하는지에 대한 간단한 설명을 제공하세요."
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "당신의 임무는 가상현실(VR) 피트니스 게임을 위한 혁신적이고 매력적인 아이디어 목록을 생성하는 것입니다. 다양한 게임 장르, 독특한 게임플레이 메커니즘, 몰입형 환경, 그리고 신체 운동과 움직임을 통합하는 창의적인 방법들을 고려하세요. 아이디어들은 광범위한 피트니스 애호가들과 게이머들에게 매력적이어야 하며, VR에서 운동하면서 활동적이고 재미있게 지낼 수 있도록 격려해야 합니다. 각 아이디어에 대해 게임 컨셉, 주요 기능, 그리고 피트니스를 어떻게 촉진하는지에 대한 간단한 설명을 제공하세요."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=1000,
    temperature=1,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "당신의 임무는 가상현실(VR) 피트니스 게임을 위한 혁신적이고 매력적인 아이디어 목록을 생성하는 것입니다. 다양한 게임 장르, 독특한 게임플레이 메커니즘, 몰입형 환경, 그리고 신체 운동과 움직임을 통합하는 창의적인 방법들을 고려하세요. 아이디어들은 광범위한 피트니스 애호가들과 게이머들에게 매력적이어야 하며, VR에서 운동하면서 활동적이고 재미있게 지낼 수 있도록 격려해야 합니다. 각 아이디어에 대해 게임 컨셉, 주요 기능, 그리고 피트니스를 어떻게 촉진하는지에 대한 간단한 설명을 제공하세요."
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "당신의 임무는 가상현실(VR) 피트니스 게임을 위한 혁신적이고 매력적인 아이디어 목록을 생성하는 것입니다. 다양한 게임 장르, 독특한 게임플레이 메커니즘, 몰입형 환경, 그리고 신체 운동과 움직임을 통합하는 창의적인 방법들을 고려하세요. 아이디어들은 광범위한 피트니스 애호가들과 게이머들에게 매력적이어야 하며, VR에서 운동하면서 활동적이고 재미있게 지낼 수 있도록 격려해야 합니다. 각 아이디어에 대해 게임 컨셉, 주요 기능, 그리고 피트니스를 어떻게 촉진하는지에 대한 간단한 설명을 제공하세요."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>