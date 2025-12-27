# 서비스 계층

다양한 서비스 계층을 통해 애플리케이션의 필요에 따라 가용성, 성능 및 예측 가능한 비용의 균형을 맞출 수 있습니다.

---

저희는 세 가지 서비스 계층을 제공합니다:
- **Priority Tier:** 시간, 가용성 및 예측 가능한 가격이 중요한 프로덕션에 배포된 워크플로우에 최적
- **Standard:** 파일럿 및 일상적인 사용 사례 확장을 위한 기본 계층
- **Batch:** 비동기 워크플로우로서 대기하거나 정상 용량 외에서 실행되는 것이 유리한 경우에 최적

## Standard Tier

Standard 계층은 모든 API 요청의 기본 서비스 계층입니다. 이 계층의 요청은 다른 모든 요청과 함께 우선순위가 지정되며 최선의 노력 가용성을 준수합니다.

## Priority Tier

이 계층의 요청은 Anthropic에 대한 다른 모든 요청보다 우선순위가 지정됩니다. 이러한 우선순위 지정은 피크 시간대에도 ["server overloaded" 오류](/docs/ko/api/errors#http-errors)를 최소화하는 데 도움이 됩니다.

자세한 내용은 [Priority Tier 시작하기](#get-started-with-priority-tier)를 참조하세요.

## 요청이 계층에 할당되는 방식

요청을 처리할 때 Anthropic은 다음 시나리오에서 요청을 Priority Tier에 할당하기로 결정합니다:
- 조직이 충분한 Priority Tier 용량 **input** 토큰/분을 보유하고 있음
- 조직이 충분한 Priority Tier 용량 **output** 토큰/분을 보유하고 있음

Anthropic은 다음과 같이 Priority Tier 용량에 대한 사용량을 계산합니다:

**Input Tokens**
- 캐시 읽기는 캐시에서 읽은 토큰당 0.1 토큰
- 캐시 쓰기는 5분 TTL로 캐시에 쓴 토큰당 1.25 토큰
- 캐시 쓰기는 1시간 TTL로 캐시에 쓴 토큰당 2.00 토큰
- [long-context](/docs/ko/build-with-claude/context-windows) (>200k input 토큰) 요청의 경우, input 토큰은 토큰당 2 토큰
- 다른 모든 input 토큰은 토큰당 1 토큰

**Output Tokens**
- [long-context](/docs/ko/build-with-claude/context-windows) (>200k input 토큰) 요청의 경우, output 토큰은 토큰당 1.5 토큰
- 다른 모든 output 토큰은 토큰당 1 토큰

그 외의 경우 요청은 Standard 계층으로 진행됩니다.

<Note>
Priority Tier에 할당된 요청은 Priority Tier 용량과 일반 속도 제한 모두에서 가져옵니다.
요청을 처리하면 속도 제한을 초과하는 경우 요청이 거부됩니다.
</Note>

## 서비스 계층 사용

`service_tier` 매개변수를 설정하여 요청에 사용할 수 있는 서비스 계층을 제어할 수 있습니다:

```python
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello, Claude!"}],
    service_tier="auto"  # Automatically use Priority Tier when available, fallback to standard
)
```

`service_tier` 매개변수는 다음 값을 허용합니다:

- `"auto"` (기본값) - 사용 가능한 경우 Priority Tier 용량을 사용하고, 그렇지 않으면 다른 용량으로 폴백
- `"standard_only"` - Standard 계층 용량만 사용하며, Priority Tier 용량을 사용하지 않으려는 경우에 유용

응답 `usage` 객체에는 요청에 할당된 서비스 계층도 포함됩니다:

```json
{
  "usage": {
    "input_tokens": 410,
    "cache_creation_input_tokens": 0,
    "cache_read_input_tokens": 0,
    "output_tokens": 585,
    "service_tier": "priority"
  }
}
```
이를 통해 요청에 할당된 서비스 계층을 확인할 수 있습니다.

Priority Tier 약정이 있는 모델로 `service_tier="auto"`를 요청할 때 이러한 응답 헤더는 통찰력을 제공합니다:
```
anthropic-priority-input-tokens-limit: 10000
anthropic-priority-input-tokens-remaining: 9618
anthropic-priority-input-tokens-reset: 2025-01-12T23:11:59Z
anthropic-priority-output-tokens-limit: 10000
anthropic-priority-output-tokens-remaining: 6000
anthropic-priority-output-tokens-reset: 2025-01-12T23:12:21Z
```
이러한 헤더의 존재를 사용하여 요청이 한계를 초과했더라도 Priority Tier에 적격인지 감지할 수 있습니다.

## Priority Tier 시작하기

다음에 관심이 있는 경우 Priority Tier 용량에 약정하고 싶을 수 있습니다:
- **더 높은 가용성**: 우선순위가 지정된 계산 리소스로 99.5% 가동 시간 목표
- **비용 제어**: 예측 가능한 지출 및 더 긴 약정에 대한 할인
- **유연한 오버플로우**: 약정된 용량을 초과할 때 자동으로 Standard 계층으로 폴백

Priority Tier에 약정하려면 다음을 결정해야 합니다:
- Input 토큰/분의 수
- Output 토큰/분의 수
- 약정 기간 (1, 3, 6 또는 12개월)
- 특정 모델 버전

<Note>
구매하는 Input 토큰과 Output 토큰의 비율이 중요합니다. Priority Tier 용량을 실제 트래픽 패턴에 맞게 조정하면 구매한 토큰의 활용도를 최대화할 수 있습니다.
</Note>

### 지원되는 모델

Priority Tier는 다음에서 지원됩니다:

- Claude Opus 4.5
- Claude Sonnet 4.5
- Claude Haiku 4.5
- Claude Opus 4.1
- Claude Opus 4
- Claude Sonnet 4
- Claude Sonnet 3.7 ([deprecated](/docs/ko/about-claude/model-deprecations))
- Claude Haiku 3.5 ([deprecated](/docs/ko/about-claude/model-deprecations))

모델에 대한 자세한 내용은 [모델 개요 페이지](/docs/ko/about-claude/models/overview)를 확인하세요.

### Priority Tier에 액세스하는 방법

Priority Tier 사용을 시작하려면:

1. [영업팀에 문의](https://claude.com/contact-sales/priority-tier)하여 프로비저닝 완료
2. (선택 사항) API 요청을 업데이트하여 선택적으로 `service_tier` 매개변수를 `auto`로 설정
3. 응답 헤더 및 Claude Console을 통해 사용량 모니터링