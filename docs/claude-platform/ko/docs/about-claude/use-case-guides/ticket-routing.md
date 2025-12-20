# 티켓 라우팅

이 가이드는 Claude의 고급 자연어 이해 기능을 활용하여 고객 의도, 긴급도, 우선순위, 고객 프로필 등을 기반으로 대규모 고객 지원 티켓을 분류하는 방법을 설명합니다.

---

## Claude를 티켓 라우팅에 사용할지 여부 결정

다음은 기존 ML 접근 방식 대신 Claude와 같은 LLM을 분류 작업에 사용해야 하는 주요 지표입니다:

    <section title="사용 가능한 레이블이 지정된 학습 데이터가 제한적입니다">

        기존 ML 프로세스는 대규모 레이블이 지정된 데이터 세트가 필요합니다. Claude의 사전 학습된 모델은 단 수십 개의 레이블이 지정된 예제만으로도 티켓을 효과적으로 분류할 수 있어 데이터 준비 시간과 비용을 크게 줄입니다.
    
</section>
    <section title="분류 카테고리가 시간이 지남에 따라 변경되거나 진화할 가능성이 있습니다">

        기존 ML 접근 방식이 확립되면 이를 변경하는 것은 번거롭고 데이터 집약적인 작업입니다. 반면에 제품이나 고객 요구 사항이 진화함에 따라 Claude는 광범위한 학습 데이터 재레이블링 없이 클래스 정의의 변경이나 새로운 클래스에 쉽게 적응할 수 있습니다.
    
</section>
    <section title="복잡하고 구조화되지 않은 텍스트 입력을 처리해야 합니다">

        기존 ML 모델은 종종 구조화되지 않은 데이터로 어려움을 겪으며 광범위한 특성 엔지니어링이 필요합니다. Claude의 고급 언어 이해 기능은 엄격한 온톨로지 구조에 의존하기보다는 콘텐츠와 컨텍스트를 기반으로 정확한 분류를 가능하게 합니다.
    
</section>
    <section title="분류 규칙이 의미론적 이해를 기반으로 합니다">

        기존 ML 접근 방식은 종종 단어 모음 모델이나 간단한 패턴 매칭에 의존합니다. Claude는 클래스가 예제가 아닌 조건으로 정의될 때 기본 규칙을 이해하고 적용하는 데 탁월합니다.
    
</section>
    <section title="분류 결정에 대한 해석 가능한 추론이 필요합니다">

        많은 기존 ML 모델은 의사 결정 프로세스에 대한 통찰력을 거의 제공하지 않습니다. Claude는 분류 결정에 대한 인간이 읽을 수 있는 설명을 제공하여 자동화 시스템에 대한 신뢰를 구축하고 필요한 경우 쉬운 적응을 촉진할 수 있습니다.
    
</section>
    <section title="엣지 케이스와 모호한 티켓을 더 효과적으로 처리하고 싶습니다">

        기존 ML 시스템은 종종 이상치와 모호한 입력으로 어려움을 겪으며, 이를 자주 잘못 분류하거나 포괄적 카테고리로 기본값을 설정합니다. Claude의 자연어 처리 기능을 통해 지원 티켓의 컨텍스트와 뉘앙스를 더 잘 해석할 수 있어 수동 개입이 필요한 잘못 라우팅되거나 분류되지 않은 티켓의 수를 줄일 수 있습니다.
    
</section>
    <section title="별도의 모델을 유지하지 않고 다국어 지원이 필요합니다">

        기존 ML 접근 방식은 일반적으로 지원되는 각 언어에 대해 별도의 모델이나 광범위한 번역 프로세스가 필요합니다. Claude의 다국어 기능을 통해 별도의 모델이나 광범위한 번역 프로세스 없이 다양한 언어의 티켓을 분류할 수 있어 글로벌 고객 기반에 대한 지원을 간소화합니다.
    
</section>

***

##  LLM 지원 워크플로우 구축 및 배포

### 현재 지원 접근 방식 이해
자동화로 뛰어들기 전에 기존 티켓팅 시스템을 이해하는 것이 중요합니다. 지원 팀이 현재 티켓 라우팅을 어떻게 처리하는지 조사하여 시작하세요.

다음과 같은 질문을 고려하세요:
* SLA/서비스 제공이 적용되는지 결정하는 데 어떤 기준이 사용됩니까?
* 티켓 라우팅은 어느 수준의 지원이나 제품 전문가에게 티켓이 가는지 결정하는 데 사용됩니까?
* 이미 자동화된 규칙이나 워크플로우가 있습니까? 어떤 경우에 실패합니까?
* 엣지 케이스나 모호한 티켓은 어떻게 처리됩니까?
* 팀은 티켓을 어떻게 우선순위를 정합니까?

인간이 특정 사례를 처리하는 방법을 더 많이 알수록 Claude와 함께 작업하여 작업을 더 잘 수행할 수 있습니다.

### 사용자 의도 카테고리 정의
잘 정의된 사용자 의도 카테고리 목록은 Claude를 사용한 정확한 지원 티켓 분류에 중요합니다. Claude가 시스템 내에서 티켓을 효과적으로 라우팅하는 능력은 시스템의 카테고리가 얼마나 잘 정의되어 있는지에 정확히 비례합니다.

다음은 사용자 의도 카테고리 및 하위 카테고리의 몇 가지 예입니다.

    <section title="기술적 문제">

        * 하드웨어 문제
        * 소프트웨어 버그
        * 호환성 문제
        * 성능 문제
    
</section>
    <section title="계정 관리">

        * 비밀번호 재설정
        * 계정 접근 문제
        * 청구 문의
        * 구독 변경
    
</section>
    <section title="제품 정보">

        * 기능 문의
        * 제품 호환성 질문
        * 가격 정보
        * 가용성 문의
    
</section>
    <section title="사용자 지침">

        * 방법 질문
        * 기능 사용 지원
        * 모범 사례 조언
        * 문제 해결 지침
    
</section>
    <section title="피드백">

        * 버그 보고
        * 기능 요청
        * 일반 피드백 또는 제안
        * 불만
    
</section>
    <section title="주문 관련">

        * 주문 상태 문의
        * 배송 정보
        * 반품 및 교환
        * 주문 수정
    
</section>
    <section title="서비스 요청">

        * 설치 지원
        * 업그레이드 요청
        * 유지보수 일정 예약
        * 서비스 취소
    
</section>
    <section title="보안 문제">

        * 데이터 개인정보 보호 문의
        * 의심스러운 활동 보고
        * 보안 기능 지원
    
</section>
    <section title="규정 준수 및 법률">

        * 규제 준수 질문
        * 서비스 약관 문의
        * 법적 문서 요청
    
</section>
    <section title="긴급 지원">

        * 중요 시스템 장애
        * 긴급 보안 문제
        * 시간에 민감한 문제
    
</section>
    <section title="교육 및 학습">

        * 제품 교육 요청
        * 문서 문의
        * 웨비나 또는 워크숍 정보
    
</section>
    <section title="통합 및 API">

        * 통합 지원
        * API 사용 질문
        * 타사 호환성 문의
    
</section>

의도 외에도 티켓 라우팅 및 우선순위 지정은 긴급도, 고객 유형, SLA 또는 언어와 같은 다른 요소의 영향을 받을 수 있습니다. 자동화된 라우팅 시스템을 구축할 때 다른 라우팅 기준도 고려해야 합니다.

### 성공 기준 수립

지원 팀과 함께 [명확한 성공 기준을 정의](/docs/ko/test-and-evaluate/define-success)하고 측정 가능한 벤치마크, 임계값 및 목표를 설정하세요.

다음은 LLM을 지원 티켓 라우팅에 사용할 때의 표준 기준 및 벤치마크입니다:

    <section title="분류 일관성">

        이 메트릭은 Claude가 시간이 지남에 따라 유사한 티켓을 얼마나 일관되게 분류하는지 평가합니다. 라우팅 안정성을 유지하는 데 중요합니다. 표준화된 입력 세트로 모델을 주기적으로 테스트하고 일관성 비율이 95% 이상이 되도록 목표를 설정하여 이를 측정하세요.
    
</section>
    <section title="적응 속도">

        이는 Claude가 새로운 카테고리나 변경되는 티켓 패턴에 얼마나 빨리 적응할 수 있는지 측정합니다. 새로운 티켓 유형을 도입하고 모델이 이러한 새로운 카테고리에서 만족스러운 정확도(예: >90%)를 달성하는 데 걸리는 시간을 측정하여 이를 테스트하세요. 50-100개의 샘플 티켓 내에서 적응을 목표로 하세요.
    
</section>
    <section title="다국어 처리">

        이는 Claude가 여러 언어로 티켓을 정확하게 라우팅하는 능력을 평가합니다. 다양한 언어에 걸쳐 라우팅 정확도를 측정하고 비주요 언어의 정확도 저하가 5-10% 이하가 되도록 목표를 설정하세요.
    
</section>
    <section title="엣지 케이스 처리">

        이는 비정상적이거나 복잡한 티켓에 대한 Claude의 성능을 평가합니다. 엣지 케이스의 테스트 세트를 만들고 라우팅 정확도를 측정하여 이러한 어려운 입력에서 최소 80% 정확도를 목표로 하세요.
    
</section>
    <section title="편향 완화">

        이는 다양한 고객 인구통계에 걸쳐 라우팅에서 Claude의 공정성을 측정합니다. 라우팅 결정에서 잠재적 편향을 정기적으로 감시하고 모든 고객 그룹에서 일관된 라우팅 정확도(2-3% 이내)를 목표로 하세요.
    
</section>
    <section title="프롬프트 효율성">

        토큰 수를 최소화하는 것이 중요한 상황에서 이 기준은 Claude가 최소한의 컨텍스트로 얼마나 잘 수행하는지 평가합니다. 제공된 컨텍스트의 양을 변화시키면서 라우팅 정확도를 측정하고 티켓 제목과 간단한 설명만으로 90% 이상의 정확도를 목표로 하세요.
    
</section>
    <section title="설명 가능성 점수">

        이는 라우팅 결정에 대한 Claude의 설명의 품질과 관련성을 평가합니다. 인간 평가자는 설명을 척도(예: 1-5)로 점수를 매길 수 있으며, 평균 점수 4 이상을 달성하는 것을 목표로 하세요.
    
</section>

다음은 LLM 사용 여부와 관계없이 유용할 수 있는 일반적인 성공 기준입니다:

    <section title="라우팅 정확도">

        라우팅 정확도는 티켓이 처음 시도에서 적절한 팀이나 개인에게 올바르게 할당되는 빈도를 측정합니다. 이는 일반적으로 총 티켓 중 올바르게 라우팅된 티켓의 백분율로 측정됩니다. 업계 벤치마크는 종종 90-95% 정확도를 목표로 하지만 지원 구조의 복잡성에 따라 달라질 수 있습니다.
    
</section>
    <section title="할당까지의 시간">

        이 메트릭은 티켓이 제출된 후 할당될 때까지 걸리는 시간을 추적합니다. 더 빠른 할당 시간은 일반적으로 더 빠른 해결과 향상된 고객 만족도로 이어집니다. 최고 수준의 시스템은 종종 5분 미만의 평균 할당 시간을 달성하며, 많은 시스템이 거의 즉각적인 라우팅(LLM 구현으로 가능)을 목표로 합니다.
    
</section>
    <section title="재라우팅 비율">

        재라우팅 비율은 초기 라우팅 후 티켓을 재할당해야 하는 빈도를 나타냅니다. 낮은 비율은 더 정확한 초기 라우팅을 시사합니다. 재라우팅 비율을 10% 미만으로 목표로 하며, 최고 성능 시스템은 5% 이하의 비율을 달성합니다.
    
</section>
    <section title="첫 연락 해결률">

        이는 고객과의 첫 번째 상호작용 중에 해결된 티켓의 백분율을 측정합니다. 높은 비율은 효율적인 라우팅과 잘 준비된 지원 팀을 나타냅니다. 업계 벤치마크는 일반적으로 70-75% 범위이며, 최고 성능자는 80% 이상의 비율을 달성합니다.
    
</section>
    <section title="평균 처리 시간">

        평균 처리 시간은 티켓을 처음부터 끝까지 해결하는 데 걸리는 시간을 측정합니다. 효율적인 라우팅은 이 시간을 크게 줄일 수 있습니다. 벤치마크는 산업과 복잡성에 따라 크게 다르지만, 많은 조직은 중요하지 않은 문제의 경우 평균 처리 시간을 24시간 이내로 유지하려고 합니다.
    
</section>
    <section title="고객 만족도 점수">

        종종 상호작용 후 설문조사를 통해 측정되는 이러한 점수는 지원 프로세스에 대한 전반적인 고객 만족도를 반영합니다. 효과적인 라우팅은 더 높은 만족도에 기여합니다. CSAT 점수 90% 이상을 목표로 하며, 최고 성능자는 종종 95% 이상의 만족도를 달성합니다.
    
</section>
    <section title="에스컬레이션 비율">

        이는 티켓을 더 높은 수준의 지원으로 에스컬레이션해야 하는 빈도를 측정합니다. 낮은 에스컬레이션 비율은 종종 더 정확한 초기 라우팅을 나타냅니다. 에스컬레이션 비율을 20% 미만으로 유지하려고 노력하며, 최고 수준의 시스템은 10% 이하의 비율을 달성합니다.
    
</section>
    <section title="에이전트 생산성">

        이 메트릭은 라우팅 솔루션을 구현한 후 에이전트가 효과적으로 처리할 수 있는 티켓 수를 살펴봅니다. 개선된 라우팅은 생산성을 증가시켜야 합니다. 에이전트당 일일 또는 시간당 해결된 티켓을 추적하여 이를 측정하고 새로운 라우팅 시스템을 구현한 후 10-20% 개선을 목표로 하세요.
    
</section>
    <section title="셀프 서비스 우회 비율">

        이는 라우팅 시스템에 들어가기 전에 셀프 서비스 옵션을 통해 해결된 잠재적 티켓의 백분율을 측정합니다. 높은 비율은 효과적인 사전 라우팅 분류를 나타냅니다. 20-30%의 우회 비율을 목표로 하며, 최고 성능자는 40% 이상의 비율을 달성합니다.
    
</section>
    <section title="티켓당 비용">

        이 메트릭은 각 지원 티켓을 해결하는 평균 비용을 계산합니다. 효율적인 라우팅은 시간이 지남에 따라 이 비용을 줄이는 데 도움이 되어야 합니다. 벤치마크는 크게 다르지만, 많은 조직은 개선된 라우팅 시스템을 구현한 후 티켓당 비용을 10-15% 줄이려고 합니다.
    
</section>

### 올바른 Claude 모델 선택

모델의 선택은 비용, 정확도 및 응답 시간 간의 트레이드오프에 따라 달라집니다.

많은 고객이 `claude-haiku-4-5-20251001`을 티켓 라우팅에 이상적인 모델로 찾았습니다. 이는 Claude 4 제품군에서 가장 빠르고 비용 효율적인 모델이면서도 여전히 우수한 결과를 제공합니다. 분류 문제가 깊은 주제 전문 지식이나 많은 양의 의도 카테고리 복잡한 추론이 필요한 경우 [더 큰 Sonnet 모델](/docs/ko/about-claude/models)을 선택할 수 있습니다.

### 강력한 프롬프트 구축

티켓 라우팅은 분류 작업의 한 유형입니다. Claude는 지원 티켓의 내용을 분석하고 문제 유형, 긴급도, 필요한 전문 지식 또는 기타 관련 요소를 기반으로 미리 정의된 카테고리로 분류합니다.

티켓 분류 프롬프트를 작성해봅시다. 초기 프롬프트는 사용자 요청의 내용을 포함하고 추론과 의도를 모두 반환해야 합니다.

<Tip>
[Claude 콘솔](/login)의 [프롬프트 생성기](/docs/ko/prompt-generator)를 시도하여 Claude가 초안을 작성하도록 하세요.
</Tip>

다음은 티켓 라우팅 분류 프롬프트의 예입니다:
```python
def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. Your task is to analyze customer support requests and output the appropriate classification intent for each request, along with your reasoning. 

        Here is the customer support request you need to classify:

        <request>{ticket_contents}</request>

        Please carefully analyze the above request to determine the customer's core intent and needs. Consider what the customer is asking for has concerns about.

        First, write out your reasoning and analysis of how to classify this request inside <reasoning> tags.

        Then, output the appropriate classification label for the request inside a <intent> tag. The valid intents are:
        <intents>
        <intent>Support, Feedback, Complaint</intent>
        <intent>Order Tracking</intent>
        <intent>Refund/Exchange</intent>
        </intents>

        A request may have ONLY ONE applicable intent. Only include the intent that is most applicable to the request.

        As an example, consider the following request:
        <request>Hello! I had high-speed fiber internet installed on Saturday and my installer, Kevin, was absolutely fantastic! Where can I send my positive review? Thanks for your help!</request>

        Here is an example of how your output should be formatted (for the above example request):
        <reasoning>The user seeks information in order to leave positive feedback.</reasoning>
        <intent>Support, Feedback, Complaint</intent>

        Here are a few more examples:
        <examples>
        <example 2>
        Example 2 Input:
        <request>I wanted to write and personally thank you for the compassion you showed towards my family during my father's funeral this past weekend. Your staff was so considerate and helpful throughout this whole process; it really took a load off our shoulders. The visitation brochures were beautiful. We'll never forget the kindness you showed us and we are so appreciative of how smoothly the proceedings went. Thank you, again, Amarantha Hill on behalf of the Hill Family.</request>

        Example 2 Output:
        <reasoning>User leaves a positive review of their experience.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 2>
        <example 3>

        ...

        </example 8>
        <example 9>
        Example 9 Input:
        <request>Your website keeps sending ad-popups that block the entire screen. It took me twenty minutes just to finally find the phone number to call and complain. How can I possibly access my account information with all of these popups? Can you access my account for me, since your website is broken? I need to know what the address is on file.</request>

        Example 9 Output:
        <reasoning>The user requests help accessing their web account information.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 9>

        Remember to always include your classification reasoning before your actual intent output. The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
```

이 프롬프트의 주요 구성 요소를 분석해봅시다:
* Python f-문자열을 사용하여 프롬프트 템플릿을 만들어 `ticket_contents`를 `<request>` 태그에 삽입할 수 있습니다.
* Claude에게 티켓 내용을 신중하게 분석하여 고객의 핵심 의도와 요구 사항을 결정하는 분류 시스템으로서의 명확한 역할을 부여합니다.
* Claude에게 적절한 출력 형식을 지시합니다. 이 경우 `<reasoning>` 태그 내에 추론과 분석을 제공한 후 `<intent>` 태그 내에 적절한 분류 레이블을 제공합니다.
* 유효한 의도 카테고리를 지정합니다: "Support, Feedback, Complaint", "Order Tracking", "Refund/Exchange".
* 몇 가지 예제(일명 few-shot 프롬프팅)를 포함하여 출력 형식을 설명하면 정확도와 일관성이 향상됩니다.

Claude의 응답을 다양한 XML 태그 섹션으로 분할하려는 이유는 정규식을 사용하여 출력에서 추론과 의도를 별도로 추출할 수 있기 때문입니다. 이를 통해 티켓이 라우팅되는 사람을 결정하기 위해 의도만 사용하는 것과 같은 티켓 라우팅 워크플로우에서 대상이 지정된 다음 단계를 만들 수 있습니다.

### 프롬프트 배포

프롬프트가 테스트 프로덕션 설정에서 얼마나 잘 작동하는지 알기 어렵고 [평가를 실행](/docs/ko/test-and-evaluate/develop-tests)하지 않으면 알 수 없습니다.

배포 구조를 구축해봅시다. Claude에 대한 호출을 래핑하는 메서드 서명을 정의하여 시작하세요. 이미 작성하기 시작한 메서드를 사용하며, `ticket_contents`를 입력으로 하고 이제 `reasoning`과 `intent`의 튜플을 출력으로 반환합니다. 기존 자동화가 기존 ML을 사용하는 경우 대신 해당 메서드 서명을 따르고 싶을 것입니다.

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ... The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
    # Send the prompt to the API to classify the support request.
    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
        stream=False,
    )
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

    return reasoning, intent
```

이 코드는:
* Anthropic 라이브러리를 가져오고 API 키를 사용하여 클라이언트 인스턴스를 만듭니다.
* `ticket_contents` 문자열을 사용하는 `classify_support_request` 함수를 정의합니다.
* `classification_prompt`를 사용하여 `ticket_contents`를 Claude로 보냅니다.
* 응답에서 추출한 모델의 `reasoning`과 `intent`를 반환합니다.

전체 추론과 의도 텍스트가 생성될 때까지 기다린 후 구문 분석해야 하므로 `stream=False`(기본값)를 설정합니다.

***

## 프롬프트 평가

프롬프팅은 프로덕션 준비가 되려면 테스트와 최적화가 필요한 경우가 많습니다. 솔루션의 준비 상태를 결정하려면 이전에 설정한 성공 기준과 임계값을 기반으로 성능을 평가하세요.

평가를 실행하려면 실행할 테스트 케이스가 필요합니다. 이 가이드의 나머지 부분은 이미 [테스트 케이스를 개발](/docs/ko/test-and-evaluate/develop-tests)했다고 가정합니다.

### 평가 함수 구축

이 가이드의 예제 평가는 세 가지 주요 메트릭에 따라 Claude의 성능을 측정합니다:
* 정확도
* 분류당 비용

Claude를 평가해야 할 다른 축이 있을 수 있습니다. 이는 당신에게 중요한 요소에 따라 달라집니다.

이를 평가하려면 먼저 작성한 스크립트를 수정하고 예측된 의도를 실제 의도와 비교하고 올바른 예측의 백분율을 계산하는 함수를 추가해야 합니다. 또한 비용 계산 및 시간 측정 기능을 추가해야 합니다.

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(request, actual_intent):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ...The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """

    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
    )
    usage = message.usage  # Get the usage statistics for the API call for how many input and output tokens were used.
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

      # Check if the model's prediction is correct.
    correct = actual_intent.strip() == intent.strip()

    # Return the reasoning, intent, correct, and usage.
    return reasoning, intent, correct, usage
```

우리가 한 편집을 분석해봅시다:
* 테스트 케이스에서 `actual_intent`를 `classify_support_request` 메서드에 추가하고 Claude의 의도 분류가 우리의 황금 의도 분류와 일치하는지 평가하기 위한 비교를 설정했습니다.
* API 호출에 사용된 입력 및 출력 토큰 수를 기반으로 비용을 계산하기 위해 API 호출의 사용 통계를 추출했습니다.

### 평가 실행

적절한 평가는 어떤 결과가 좋은 결과인지 결정하기 위한 명확한 임계값과 벤치마크가 필요합니다. 위의 스크립트는 정확도, 응답 시간 및 분류당 비용에 대한 런타임 값을 제공하지만 명확하게 설정된 임계값이 여전히 필요합니다. 예를 들어:
* **정확도:** 95% (100개 테스트 중)
* **분류당 비용:** 현재 라우팅 방법에서 평균 50% 감소 (100개 테스트 전체)

이러한 임계값을 설정하면 규모에 따라 어떤 방법이 당신에게 가장 좋은지, 그리고 요구 사항에 더 잘 맞추기 위해 어떤 변경이 필요할 수 있는지 빠르고 쉽게 그리고 공정한 경험주의로 알 수 있습니다.

***

## 성능 개선

복잡한 시나리오에서는 표준 [프롬프트 엔지니어링 기법](/docs/ko/build-with-claude/prompt-engineering/overview) & [가드레일 구현 전략](/docs/ko/test-and-evaluate/strengthen-guardrails/reduce-hallucinations) 이상으로 성능을 개선하기 위한 추가 전략을 고려하는 것이 도움이 될 수 있습니다. 다음은 몇 가지 일반적인 시나리오입니다:

### 20개 이상의 의도 카테고리가 있는 경우 분류 계층 구조 사용

클래스 수가 증가함에 따라 필요한 예제의 수도 확장되어 프롬프트가 다루기 어려워질 수 있습니다. 대안으로 분류자의 혼합을 사용하여 계층적 분류 시스템을 구현하는 것을 고려할 수 있습니다.
1. 의도를 분류 트리 구조로 구성하세요.
2. 트리의 모든 수준에서 분류자 시리즈를 만들어 계단식 라우팅 접근 방식을 활성화하세요.

예를 들어, "기술적 문제", "청구 질문", "일반 문의"로 티켓을 광범위하게 분류하는 최상위 분류자가 있을 수 있습니다. 이러한 각 카테고리는 분류를 더 세분화하기 위해 자체 하위 분류자를 가질 수 있습니다.

![](/docs/images/ticket-hierarchy.png)

* **장점 - 더 큰 뉘앙스와 정확도:** 각 상위 경로에 대해 다양한 프롬프트를 만들 수 있어 더 대상이 지정되고 컨텍스트에 맞는 분류가 가능합니다. 이는 향상된 정확도와 고객 요청의 더 미묘한 처리로 이어질 수 있습니다.

* **단점 - 증가된 지연 시간:** 여러 분류자는 지연 시간 증가로 이어질 수 있으며, 우리는 가장 빠른 모델인 Haiku를 사용하여 이 접근 방식을 구현할 것을 권장합니다.

### 벡터 데이터베이스 및 유사성 검색 검색을 사용하여 매우 가변적인 티켓 처리

예제 제공이 성능을 개선하는 가장 효과적인 방법이지만, 지원 요청이 매우 가변적인 경우 단일 프롬프트에 충분한 예제를 포함하기 어려울 수 있습니다.

이 시나리오에서는 벡터 데이터베이스를 사용하여 예제 데이터 세트에서 유사성 검색을 수행하고 주어진 쿼리에 대해 가장 관련성 높은 예제를 검색할 수 있습니다.

이 접근 방식은 [분류 레시피](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb)에서 자세히 설명되어 있으며 성능을 71% 정확도에서 93% 정확도로 개선하는 것으로 나타났습니다.

### 예상되는 엣지 케이스를 구체적으로 고려

다음은 Claude가 티켓을 잘못 분류할 수 있는 몇 가지 시나리오입니다(당신의 상황에 고유한 다른 시나리오가 있을 수 있습니다). 이러한 시나리오에서는 Claude가 엣지 케이스를 처리하는 방법에 대한 명시적 지침이나 예제를 프롬프트에 제공하는 것을 고려하세요:

    <section title="고객이 암묵적 요청을 합니다">

        고객은 종종 요구 사항을 간접적으로 표현합니다. 예를 들어, "2주 이상 패키지를 기다리고 있습니다"는 주문 상태에 대한 간접적인 요청일 수 있습니다.
        * **해결책:** Claude에게 이러한 종류의 요청의 실제 고객 예제와 기본 의도가 무엇인지 제공하세요. 특히 미묘한 티켓 의도에 대한 분류 근거를 포함하면 Claude가 다른 티켓에 논리를 더 잘 일반화할 수 있으므로 더 나은 결과를 얻을 수 있습니다.
    
</section>
    <section title="Claude가 감정을 의도보다 우선시합니다">

        고객이 불만을 표현할 때 Claude는 기본 문제를 해결하는 것보다 감정을 해결하는 것을 우선시할 수 있습니다.
        * **해결책:** Claude에게 고객 감정을 우선시할 시기를 지시하세요. "모든 고객 감정을 무시하세요. 고객 요청의 의도와 고객이 요청할 수 있는 정보만 분석하는 데 집중하세요"와 같이 간단할 수 있습니다.
    
</section>
    <section title="여러 문제가 문제 우선순위 지정 혼동을 야기합니다">

        고객이 단일 상호작용에서 여러 문제를 제시할 때 Claude는 주요 관심사를 식별하기 어려울 수 있습니다.
        * **해결책:** 의도의 우선순위를 명확히 하여 Claude가 추출된 의도를 더 잘 순위를 매기고 주요 관심사를 식별할 수 있도록 하세요.
    
</section>

***

## Claude를 더 큰 지원 워크플로우에 통합

적절한 통합을 위해서는 Claude 기반 티켓 라우팅 스크립트가 더 큰 티켓 라우팅 시스템의 아키텍처에 어떻게 맞는지에 대한 몇 가지 결정을 내려야 합니다. 두 가지 방법이 있습니다:
* **푸시 기반:** 사용 중인 지원 티켓 시스템(예: Zendesk)이 웹훅 이벤트를 라우팅 서비스로 보내 의도를 분류하고 라우팅합니다.
    * 이 접근 방식은 더 웹 확장 가능하지만 공개 엔드포인트를 노출해야 합니다.
* **풀 기반:** 코드가 주어진 일정에 따라 최신 티켓을 가져오고 풀 시간에 라우팅합니다.
    * 이 접근 방식은 구현하기 더 쉽지만 풀 빈도가 너무 높으면 지원 티켓 시스템에 불필요한 호출을 할 수 있거나 풀 빈도가 너무 낮으면 지나치게 느릴 수 있습니다.

이러한 접근 방식 중 하나에 대해 스크립트를 서비스로 래핑해야 합니다. 선택하는 접근 방식은 지원 티켓팅 시스템이 제공하는 API에 따라 달라집니다.

***

<CardGroup cols={2}>
    <Card title="분류 요리책" icon="link" href="https://github.com/anthropics/anthropic-cookbook/tree/main/capabilities/classification">
        더 많은 예제 코드와 자세한 평가 지침을 보려면 분류 요리책을 방문하세요.
    </Card>
    <Card title="Claude 콘솔" icon="link" href="/dashboard">
        Claude 콘솔에서 워크플로우 구축 및 평가를 시작하세요.
    </Card>
</CardGroup>