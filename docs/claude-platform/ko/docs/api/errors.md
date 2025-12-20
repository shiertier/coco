# 오류

API 오류 코드, 요청 크기 제한, 오류 형태 및 긴 요청 처리에 대한 가이드

---

## HTTP 오류

저희 API는 예측 가능한 HTTP 오류 코드 형식을 따릅니다:

* 400 - `invalid_request_error`: 요청의 형식이나 내용에 문제가 있습니다. 아래에 나열되지 않은 다른 4XX 상태 코드에 대해서도 이 오류 유형을 사용할 수 있습니다.
* 401 - `authentication_error`: API 키에 문제가 있습니다.
* 403 - `permission_error`: API 키가 지정된 리소스를 사용할 권한이 없습니다.
* 404 - `not_found_error`: 요청된 리소스를 찾을 수 없습니다.
* 413 - `request_too_large`: 요청이 허용되는 최대 바이트 수를 초과합니다. 표준 API 엔드포인트의 최대 요청 크기는 32 MB입니다.
* 429 - `rate_limit_error`: 계정이 속도 제한에 도달했습니다.
* 500 - `api_error`: Anthropic 시스템 내부에서 예상치 못한 오류가 발생했습니다.
* 529 - `overloaded_error`: API가 일시적으로 과부하 상태입니다.

  <Warning>
  529 오류는 API가 모든 사용자에게 높은 트래픽을 경험할 때 발생할 수 있습니다.
  
  드문 경우지만, 조직의 사용량이 급격히 증가하면 API의 가속 제한으로 인해 429 오류가 발생할 수 있습니다. 가속 제한에 도달하지 않으려면 트래픽을 점진적으로 늘리고 일관된 사용 패턴을 유지하세요.
  </Warning>

SSE를 통해 [스트리밍](/docs/ko/build-with-claude/streaming) 응답을 받을 때, 200 응답을 반환한 후에 오류가 발생할 수 있으며, 이 경우 오류 처리는 이러한 표준 메커니즘을 따르지 않습니다.

## 요청 크기 제한

API는 최적의 성능을 보장하기 위해 요청 크기 제한을 적용합니다:

| 엔드포인트 유형 | 최대 요청 크기 |
|:---|:---|
| Messages API | 32 MB |
| Token Counting API | 32 MB |
| [Batch API](/docs/ko/build-with-claude/batch-processing) | 256 MB |
| [Files API](/docs/ko/build-with-claude/files) | 500 MB |

이러한 제한을 초과하면 413 `request_too_large` 오류를 받게 됩니다. 이 오류는 요청이 저희 API 서버에 도달하기 전에 Cloudflare에서 반환됩니다.

## 오류 형태

오류는 항상 JSON으로 반환되며, 항상 `type`과 `message` 값을 포함하는 최상위 `error` 객체가 있습니다. 응답에는 더 쉬운 추적과 디버깅을 위한 `request_id` 필드도 포함됩니다. 예를 들어:

```json JSON
{
  "type": "error",
  "error": {
    "type": "not_found_error",
    "message": "The requested resource could not be found."
  },
  "request_id": "req_011CSHoEeqs5C35K2UUqR7Fy"
}
```

저희 [버전 관리](/docs/ko/api/versioning) 정책에 따라, 이러한 객체 내의 값을 확장할 수 있으며, `type` 값이 시간이 지남에 따라 증가할 가능성이 있습니다.

## 요청 ID

모든 API 응답에는 고유한 `request-id` 헤더가 포함됩니다. 이 헤더에는 `req_018EeWyXxfu5pfWkrYcMdjWG`와 같은 값이 포함됩니다. 특정 요청에 대해 지원팀에 문의할 때, 문제를 신속하게 해결할 수 있도록 이 ID를 포함해 주세요.

저희 공식 SDK는 `request-id` 헤더의 값을 포함하는 최상위 응답 객체의 속성으로 이 값을 제공합니다:

<CodeGroup>
  ```python Python
  import anthropic

  client = anthropic.Anthropic()

  message = client.messages.create(
      model="claude-sonnet-4-5",
      max_tokens=1024,
      messages=[
          {"role": "user", "content": "Hello, Claude"}
      ]
  )
  print(f"Request ID: {message._request_id}")
  ```

  ```typescript TypeScript
  import Anthropic from '@anthropic-ai/sdk';

  const client = new Anthropic();

  const message = await client.messages.create({
    model: 'claude-sonnet-4-5',
    max_tokens: 1024,
    messages: [
      {"role": "user", "content": "Hello, Claude"}
    ]
  });
  console.log('Request ID:', message._request_id);
  ```
</CodeGroup>

## 긴 요청

<Warning>
 특히 10분 이상 걸리는 긴 실행 요청에 대해서는 [스트리밍 Messages API](/docs/ko/build-with-claude/streaming) 또는 [Message Batches API](/docs/ko/api/creating-message-batches) 사용을 강력히 권장합니다.
</Warning>

저희 [스트리밍 Messages API](/docs/ko/build-with-claude/streaming) 또는 [Message Batches API](/docs/ko/api/creating-message-batches)를 사용하지 않고 큰 `max_tokens` 값을 설정하는 것은 권장하지 않습니다:

- 일부 네트워크는 가변적인 시간 후에 유휴 연결을 끊을 수 있으며, 이로 인해 Anthropic으로부터 응답을 받지 못하고 요청이 실패하거나 시간 초과될 수 있습니다.
- 네트워크의 신뢰성은 다양합니다. 저희 [Message Batches API](/docs/ko/api/creating-message-batches)는 중단되지 않는 네트워크 연결을 요구하는 대신 결과를 폴링할 수 있게 하여 네트워크 문제의 위험을 관리하는 데 도움이 될 수 있습니다.

직접 API 통합을 구축하는 경우, [TCP 소켓 keep-alive](https://tldp.org/HOWTO/TCP-Keepalive-HOWTO/programming.html) 설정이 일부 네트워크에서 유휴 연결 시간 초과의 영향을 줄일 수 있다는 점을 알아야 합니다.

저희 [SDK](/docs/ko/api/client-sdks)는 비스트리밍 Messages API 요청이 10분 시간 초과를 초과할 것으로 예상되지 않는지 검증하고 TCP keep-alive를 위한 소켓 옵션도 설정합니다.