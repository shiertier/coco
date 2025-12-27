# 베타 헤더

Claude API에서 베타 헤더 사용에 대한 문서

---

베타 헤더를 사용하면 실험적 기능과 새로운 모델 기능이 표준 API의 일부가 되기 전에 미리 액세스할 수 있습니다.

이러한 기능은 변경될 수 있으며 향후 릴리스에서 수정되거나 제거될 수 있습니다.

<Info>
베타 헤더는 종종 [클라이언트 SDK의 베타 네임스페이스](/docs/ko/api/client-sdks#beta-namespace-in-client-sdks)와 함께 사용됩니다
</Info>

## 베타 헤더 사용 방법

베타 기능에 액세스하려면 API 요청에 `anthropic-beta` 헤더를 포함하세요:

```http
POST /v1/messages
Content-Type: application/json
X-API-Key: YOUR_API_KEY
anthropic-beta: BETA_FEATURE_NAME
```

SDK를 사용할 때는 요청 옵션에서 베타 헤더를 지정할 수 있습니다:

<CodeGroup>

```python Python
from anthropic import Anthropic

client = Anthropic()

response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[
        {"role": "user", "content": "Hello, Claude"}
    ],
    betas=["beta-feature-name"]
)
```

```typescript TypeScript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

const msg = await anthropic.beta.messages.create({
  model: 'claude-sonnet-4-5',
  max_tokens: 1024,
  messages: [
    { role: 'user', content: 'Hello, Claude' }
  ],
  betas: ['beta-feature-name']
});
```

```bash cURL
curl https://api.anthropic.com/v1/messages \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "anthropic-beta: beta-feature-name" \
  -H "content-type: application/json" \
  -d '{
    "model": "claude-sonnet-4-5",
    "max_tokens": 1024,
    "messages": [
      {"role": "user", "content": "Hello, Claude"}
    ]
  }'
```

</CodeGroup>

<Warning>
베타 기능은 실험적이며 다음과 같을 수 있습니다:
- 예고 없이 중단적 변경 사항이 있을 수 있음
- 더 이상 사용되지 않거나 제거될 수 있음
- 다른 속도 제한 또는 가격 책정이 있을 수 있음
- 모든 지역에서 사용할 수 없을 수 있음
</Warning>

### 여러 베타 기능

단일 요청에서 여러 베타 기능을 사용하려면 헤더에 모든 기능 이름을 쉼표로 구분하여 포함하세요:

```http
anthropic-beta: feature1,feature2,feature3
```

### 버전 명명 규칙

베타 기능 이름은 일반적으로 `feature-name-YYYY-MM-DD` 패턴을 따르며, 여기서 날짜는 베타 버전이 릴리스된 시점을 나타냅니다. 항상 문서화된 정확한 베타 기능 이름을 사용하세요.

## 오류 처리

유효하지 않거나 사용할 수 없는 베타 헤더를 사용하면 오류 응답을 받게 됩니다:

```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "Unsupported beta header: invalid-beta-name"
  }
}
```

## 도움 받기

베타 기능에 대한 질문이 있는 경우:

1. 특정 기능에 대한 문서를 확인하세요
2. 업데이트에 대한 [API 변경 로그](/docs/ko/api/versioning)를 검토하세요
3. 프로덕션 사용에 대한 지원을 받으려면 지원팀에 문의하세요

베타 기능은 "있는 그대로" 제공되며 안정적인 API 기능과 동일한 SLA 보장이 없을 수 있음을 기억하세요.