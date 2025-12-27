# OpenAI SDK 호환성

Anthropic은 OpenAI SDK를 사용하여 Claude API를 테스트할 수 있도록 하는 호환성 계층을 제공합니다. 몇 가지 코드 변경만으로 Anthropic 모델 기능을 빠르게 평가할 수 있습니다.

---

<Note>
이 호환성 계층은 주로 모델 기능을 테스트하고 비교하기 위한 것이며, 대부분의 사용 사례에서 장기적이거나 프로덕션 준비가 된 솔루션으로 간주되지 않습니다. 완전히 기능하도록 유지하고 주요 변경 사항을 하지 않을 의도가 있지만, 우리의 우선순위는 [Claude API](/docs/ko/api/overview)의 안정성과 효과입니다.

알려진 호환성 제한 사항에 대한 자세한 정보는 [중요한 OpenAI 호환성 제한 사항](#important-openai-compatibility-limitations)을 참조하세요.

OpenAI SDK 호환성 기능에 문제가 발생하면 [여기](https://forms.gle/oQV4McQNiuuNbz9n8)에서 알려주세요.
</Note>

<Tip>
최고의 경험과 Claude API 전체 기능 세트([PDF 처리](/docs/ko/build-with-claude/pdf-support), [인용](/docs/ko/build-with-claude/citations), [확장 사고](/docs/ko/build-with-claude/extended-thinking), [프롬프트 캐싱](/docs/ko/build-with-claude/prompt-caching))에 접근하려면 네이티브 [Claude API](/docs/ko/api/overview)를 사용하는 것을 권장합니다.
</Tip>

## OpenAI SDK 시작하기

OpenAI SDK 호환성 기능을 사용하려면 다음을 수행해야 합니다:

1. 공식 OpenAI SDK 사용  
2. 다음 항목 변경  
   * 기본 URL을 Claude API를 가리키도록 업데이트  
   * API 키를 [Claude API 키](/settings/keys)로 교체  
   * 모델 이름을 [Claude 모델](/docs/ko/about-claude/models/overview)을 사용하도록 업데이트  
3. 지원되는 기능에 대해 아래 설명서를 검토하세요

### 빠른 시작 예제

<CodeGroup>
    ```python Python
    from openai import OpenAI

    client = OpenAI(
        api_key="ANTHROPIC_API_KEY",  # Your Claude API key
        base_url="https://api.anthropic.com/v1/"  # the Claude API endpoint
    )

    response = client.chat.completions.create(
        model="claude-sonnet-4-5", # Anthropic model name
        messages=[
            {"role": "system", "content": "You are a helpful assistant."},
            {"role": "user", "content": "Who are you?"}
        ],
    )

    print(response.choices[0].message.content)
    ```
    
    ```typescript TypeScript
    import OpenAI from 'openai';

    const openai = new OpenAI({
        apiKey: "ANTHROPIC_API_KEY",   // Your Claude API key
        baseURL: "https://api.anthropic.com/v1/",  // Claude API endpoint
    });

    const response = await openai.chat.completions.create({
        messages: [
            { role: "user", content: "Who are you?" }
        ],
        model: "claude-sonnet-4-5", // Claude model name
    });

    console.log(response.choices[0].message.content);
    ```
</CodeGroup>

## 중요한 OpenAI 호환성 제한 사항

#### API 동작

OpenAI 사용과의 가장 실질적인 차이점은 다음과 같습니다:

* 함수 호출을 위한 `strict` 매개변수는 무시되므로, 도구 사용 JSON이 제공된 스키마를 따르도록 보장되지 않습니다. 보장된 스키마 준수를 위해 네이티브 [Claude API와 구조화된 출력](/docs/ko/build-with-claude/structured-outputs)을 사용하세요.
* 오디오 입력은 지원되지 않으며, 단순히 무시되고 입력에서 제거됩니다  
* 프롬프트 캐싱은 지원되지 않지만, [Anthropic SDK](/docs/ko/api/client-sdks)에서는 지원됩니다  
* 시스템/개발자 메시지는 호이스팅되고 대화의 시작 부분에 연결됩니다. Anthropic은 단일 초기 시스템 메시지만 지원하기 때문입니다.

지원되지 않는 대부분의 필드는 오류를 생성하지 않고 자동으로 무시됩니다. 이들은 모두 아래에 문서화되어 있습니다.

#### 출력 품질 고려 사항

프롬프트를 많이 조정했다면, OpenAI에 특별히 잘 조정되었을 가능성이 높습니다. [Claude Console의 프롬프트 개선 도구](/dashboard)를 좋은 시작점으로 사용하는 것을 고려하세요.

#### 시스템 / 개발자 메시지 호이스팅

OpenAI SDK에 대한 대부분의 입력은 Anthropic의 API 매개변수에 직접 매핑되지만, 한 가지 뚜렷한 차이점은 시스템 / 개발자 프롬프트의 처리입니다. 이 두 프롬프트는 OpenAI를 통해 채팅 대화 전체에 배치될 수 있습니다. Anthropic은 초기 시스템 메시지만 지원하므로, 모든 시스템/개발자 메시지를 가져와 그 사이에 단일 줄바꿈(`\n`)으로 연결합니다. 이 전체 문자열은 메시지의 시작 부분에 단일 시스템 메시지로 제공됩니다.

#### 확장 사고 지원

`thinking` 매개변수를 추가하여 [확장 사고](/docs/ko/build-with-claude/extended-thinking) 기능을 활성화할 수 있습니다. 이것은 복잡한 작업에 대해 Claude의 추론을 개선하지만, OpenAI SDK는 Claude의 상세한 사고 과정을 반환하지 않습니다. Claude의 단계별 추론 출력에 대한 접근을 포함한 전체 확장 사고 기능을 사용하려면 네이티브 Claude API를 사용하세요.

<CodeGroup>
    ```python Python
    response = client.chat.completions.create(
        model="claude-sonnet-4-5",
        messages=...,
        extra_body={
            "thinking": { "type": "enabled", "budget_tokens": 2000 }
        }
    )
    ```
    
    ```typescript TypeScript
    const response = await openai.chat.completions.create({
        messages: [
            { role: "user", content: "Who are you?" }
        ],
        model: "claude-sonnet-4-5",
        // @ts-expect-error
        thinking: { type: "enabled", budget_tokens: 2000 }
    });

    ```
</CodeGroup>

## 속도 제한

속도 제한은 `/v1/messages` 엔드포인트에 대한 Anthropic의 [표준 제한](/docs/ko/api/rate-limits)을 따릅니다.

## 상세한 OpenAI 호환 API 지원
### 요청 필드
#### 단순 필드
| 필드 | 지원 상태 |
|--------|----------------|
| `model` | Claude 모델 이름 사용 |
| `max_tokens` | 완전히 지원됨 |
| `max_completion_tokens` | 완전히 지원됨 |
| `stream` | 완전히 지원됨 |
| `stream_options` | 완전히 지원됨 |
| `top_p` | 완전히 지원됨 |
| `parallel_tool_calls` | 완전히 지원됨 |
| `stop` | 모든 공백이 아닌 중지 시퀀스 작동 |
| `temperature` | 0과 1 사이(포함). 1보다 큰 값은 1로 제한됨 |
| `n` | 정확히 1이어야 함 |
| `logprobs` | 무시됨 |
| `metadata` | 무시됨 |
| `response_format` | 무시됨. JSON 출력의 경우 네이티브 Claude API와 함께 [구조화된 출력](/docs/ko/build-with-claude/structured-outputs)을 사용하세요 |
| `prediction` | 무시됨 |
| `presence_penalty` | 무시됨 |
| `frequency_penalty` | 무시됨 |
| `seed` | 무시됨 |
| `service_tier` | 무시됨 |
| `audio` | 무시됨 |
| `logit_bias` | 무시됨 |
| `store` | 무시됨 |
| `user` | 무시됨 |
| `modalities` | 무시됨 |
| `top_logprobs` | 무시됨 |
| `reasoning_effort` | 무시됨 |

#### `tools` / `functions` 필드
<section title="필드 표시">

<Tabs>
<Tab title="Tools">
`tools[n].function` 필드
| 필드        | 지원 상태         |
|--------------|-----------------|
| `name`       | 완전히 지원됨 |
| `description`| 완전히 지원됨 |
| `parameters` | 완전히 지원됨 |
| `strict`     | 무시됨. 엄격한 스키마 검증을 위해 네이티브 Claude API와 함께 [구조화된 출력](/docs/ko/build-with-claude/structured-outputs)을 사용하세요 |
</Tab>
<Tab title="Functions">

`functions[n]` 필드
<Info>
OpenAI는 `functions` 필드를 더 이상 사용하지 않으며 대신 `tools`를 사용할 것을 제안합니다.
</Info>
| 필드        | 지원 상태         |
|--------------|-----------------|
| `name`       | 완전히 지원됨 |
| `description`| 완전히 지원됨 |
| `parameters` | 완전히 지원됨 |
| `strict`     | 무시됨. 엄격한 스키마 검증을 위해 네이티브 Claude API와 함께 [구조화된 출력](/docs/ko/build-with-claude/structured-outputs)을 사용하세요 |
</Tab>
</Tabs>

</section>

#### `messages` 배열 필드
<section title="필드 표시">

<Tabs>
<Tab title="Developer role">
`messages[n].role == "developer"`에 대한 필드
<Info>
개발자 메시지는 초기 시스템 메시지의 일부로 대화의 시작 부분으로 호이스팅됩니다
</Info>
| 필드 | 지원 상태 |
|-------|---------|
| `content` | 완전히 지원됨, 하지만 호이스팅됨 |
| `name` | 무시됨 |

</Tab>
<Tab title="System role">
`messages[n].role == "system"`에 대한 필드

<Info>
시스템 메시지는 초기 시스템 메시지의 일부로 대화의 시작 부분으로 호이스팅됩니다
</Info>
| 필드 | 지원 상태 |
|-------|---------|
| `content` | 완전히 지원됨, 하지만 호이스팅됨 |
| `name` | 무시됨 |

</Tab>
<Tab title="User role">
`messages[n].role == "user"`에 대한 필드

| 필드 | 변형 | 하위 필드 | 지원 상태 |
|-------|---------|-----------|----------------|
| `content` | `string` | | 완전히 지원됨 |
| | `array`, `type == "text"` | | 완전히 지원됨 |
| | `array`, `type == "image_url"` | `url` | 완전히 지원됨 |
| | | `detail` | 무시됨 |
| | `array`, `type == "input_audio"` | | 무시됨 |
| | `array`, `type == "file"` | | 무시됨 |
| `name` | | | 무시됨 |

</Tab>

<Tab title="Assistant role">
`messages[n].role == "assistant"`에 대한 필드
| 필드 | 변형 | 지원 상태 |
|-------|---------|----------------|
| `content` | `string` | 완전히 지원됨 |
| | `array`, `type == "text"` | 완전히 지원됨 |
| | `array`, `type == "refusal"` | 무시됨 |
| `tool_calls` | | 완전히 지원됨 |
| `function_call` | | 완전히 지원됨 |
| `audio` | | 무시됨 |
| `refusal` | | 무시됨 |

</Tab>

<Tab title="Tool role">
`messages[n].role == "tool"`에 대한 필드
| 필드 | 변형 | 지원 상태 |
|-------|---------|----------------|
| `content` | `string` | 완전히 지원됨 |
| | `array`, `type == "text"` | 완전히 지원됨 |
| `tool_call_id` | | 완전히 지원됨 |
| `tool_choice` | | 완전히 지원됨 |
| `name` | | 무시됨 |
</Tab>

<Tab title="Function role">
`messages[n].role == "function"`에 대한 필드
| 필드 | 변형 | 지원 상태 |
|-------|---------|----------------|
| `content` | `string` | 완전히 지원됨 |
| | `array`, `type == "text"` | 완전히 지원됨 |
| `tool_choice` | | 완전히 지원됨 |
| `name` | | 무시됨 |
</Tab>
</Tabs>

</section>

### 응답 필드

| 필드 | 지원 상태 |
|---------------------------|----------------|
| `id` | 완전히 지원됨 |
| `choices[]` | 항상 길이가 1 |
| `choices[].finish_reason` | 완전히 지원됨 |
| `choices[].index` | 완전히 지원됨 |
| `choices[].message.role` | 완전히 지원됨 |
| `choices[].message.content` | 완전히 지원됨 |
| `choices[].message.tool_calls` | 완전히 지원됨 |
| `object` | 완전히 지원됨 |
| `created` | 완전히 지원됨 |
| `model` | 완전히 지원됨 |
| `finish_reason` | 완전히 지원됨 |
| `content` | 완전히 지원됨 |
| `usage.completion_tokens` | 완전히 지원됨 |
| `usage.prompt_tokens` | 완전히 지원됨 |
| `usage.total_tokens` | 완전히 지원됨 |
| `usage.completion_tokens_details` | 항상 비어있음 |
| `usage.prompt_tokens_details` | 항상 비어있음 |
| `choices[].message.refusal` | 항상 비어있음 |
| `choices[].message.audio` | 항상 비어있음 |
| `logprobs` | 항상 비어있음 |
| `service_tier` | 항상 비어있음 |
| `system_fingerprint` | 항상 비어있음 |

### 오류 메시지 호환성

호환성 계층은 OpenAI API와 일관된 오류 형식을 유지합니다. 그러나 상세한 오류 메시지는 동등하지 않을 것입니다. 로깅 및 디버깅에만 오류 메시지를 사용하는 것을 권장합니다.

### 헤더 호환성

OpenAI SDK가 자동으로 헤더를 관리하지만, 다음은 Claude API에서 지원하는 헤더의 전체 목록입니다. 헤더를 직접 작업해야 하는 개발자를 위한 것입니다.

| 헤더 | 지원 상태 |
|---------|----------------|
| `x-ratelimit-limit-requests` | 완전히 지원됨 |
| `x-ratelimit-limit-tokens` | 완전히 지원됨 |
| `x-ratelimit-remaining-requests` | 완전히 지원됨 |
| `x-ratelimit-remaining-tokens` | 완전히 지원됨 |
| `x-ratelimit-reset-requests` | 완전히 지원됨 |
| `x-ratelimit-reset-tokens` | 완전히 지원됨 |
| `retry-after` | 완전히 지원됨 |
| `request-id` | 완전히 지원됨 |
| `openai-version` | 항상 `2020-10-01` |
| `authorization` | 완전히 지원됨 |
| `openai-processing-ms` | 항상 비어있음 |