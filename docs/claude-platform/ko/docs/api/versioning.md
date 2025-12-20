# 버전

API 요청을 할 때 `anthropic-version` 요청 헤더를 보내야 합니다. 예를 들어, `anthropic-version: 2023-06-01`입니다. [클라이언트 SDK](/docs/ko/api/client-sdks)를 사용하는 경우 이는 자동으로 처리됩니다.

---

주어진 API 버전에 대해 다음을 보존합니다:

* 기존 입력 매개변수
* 기존 출력 매개변수

하지만 다음을 수행할 수 있습니다:

* 추가적인 선택적 입력 추가
* 출력에 추가 값 추가
* 특정 오류 유형에 대한 조건 변경
* 열거형과 같은 출력 값에 새로운 변형 추가 (예: 스트리밍 이벤트 유형)

일반적으로 이 참조에서 문서화된 대로 API를 사용하는 경우 사용법이 중단되지 않습니다.

## 버전 기록

가능한 한 항상 최신 API 버전을 사용하는 것을 권장합니다. 이전 버전은 더 이상 사용되지 않는 것으로 간주되며 새 사용자에게는 사용할 수 없을 수 있습니다.

* `2023-06-01`  
   * [스트리밍](/docs/ko/api/streaming) 서버 전송 이벤트(SSE)의 새로운 형식:  
         * 완성은 증분식입니다. 예를 들어, `" Hello"`, `" my"`, `" name"`, `" is"`, `" Claude." ` 대신 `" Hello"`, `" Hello my"`, `" Hello my name"`, `" Hello my name is"`, `" Hello my name is Claude."`입니다.  
         * 모든 이벤트는 [데이터 전용 이벤트](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#data-only%5Fmessages)가 아닌 [명명된 이벤트](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#named%5Fevents)입니다.  
         * 불필요한 `data: [DONE]` 이벤트가 제거되었습니다.  
   * 응답에서 레거시 `exception` 및 `truncated` 값이 제거되었습니다.
* `2023-01-01`: 초기 릴리스.