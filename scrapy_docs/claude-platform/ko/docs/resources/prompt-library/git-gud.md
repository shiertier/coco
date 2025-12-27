# Git gud

사용자가 설명한 버전 관리 작업을 기반으로 적절한 Git 명령어를 생성합니다.

---

> 이 프롬프트를 우리의 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

| 역할 | 내용 |
| ---- | --- |
| User | 로컬 파일에 일부 변경사항을 만들었고 이를 로컬 Git 저장소에 저장하고 싶습니다. 어떤 Git 명령어를 사용해야 하나요? |

## 예시 출력

로컬 파일에 만든 변경사항을 로컬 Git 저장소에 저장하려면 다음 Git 명령어를 사용해야 합니다:

1.  `git add .` 또는 `git add <file>`

    - 이 명령어는 만든 변경사항을 스테이징하여 커밋할 준비를 합니다. `.`는 현재 디렉토리와 하위 디렉토리의 모든 변경사항을 추가합니다. 특정 파일만 스테이징하려면 `.`를 파일 이름이나 경로로 바꾸세요.

2.  `git commit -m "커밋 메시지"`

    - 이 명령어는 스테이징된 변경사항을 로컬 Git 저장소에 커밋합니다. `-m` 플래그를 사용하면 만든 변경사항을 설명하는 커밋 메시지를 제공할 수 있습니다. `"커밋 메시지"`를 변경사항에 대한 간단하고 설명적인 요약으로 바꾸세요.

예를 들어, `example.txt`라는 파일에 변경사항을 만들었다면 다음과 같이 실행합니다:

```
git add example.txt
git commit -m "새로운 내용으로 example.txt 업데이트"

```

이 명령어들은 로컬 Git 저장소에 변경사항을 저장하여 제공된 메시지와 함께 새로운 커밋을 생성합니다. 나중에 이러한 변경사항을 원격 저장소에 푸시하거나 로컬 브랜치에서 계속 작업할 수 있습니다.

터미널이나 명령 프롬프트에서 Git 저장소 디렉토리 내에서 이 명령어들을 실행하는 것을 기억하세요.

---

## API 요청

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?",
                }
            ],
        }
    ],
)
print(message.content)


````
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>