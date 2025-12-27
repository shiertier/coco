# 콘텐츠 조정

콘텐츠 조정은 디지털 애플리케이션에서 안전하고 존중받으며 생산적인 환경을 유지하는 데 중요한 측면입니다. 이 가이드에서는 Claude를 사용하여 디지털 애플리케이션 내에서 콘텐츠를 조정하는 방법에 대해 논의하겠습니다.

---

> Claude를 사용한 콘텐츠 조정 구현 예제를 보려면 [콘텐츠 조정 쿡북](https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fmoderation%5Ffilter.ipynb)을 방문하세요.

<Tip>이 가이드는 애플리케이션 내에서 사용자 생성 콘텐츠를 조정하는 데 중점을 둡니다. Claude와의 상호작용을 조정하는 지침을 찾고 있다면 [가드레일 가이드](/docs/ko/test-and-evaluate/strengthen-guardrails/reduce-hallucinations)를 참조하세요.</Tip>

## Claude로 구축하기 전에

### Claude를 콘텐츠 조정에 사용할지 결정하기

다음은 콘텐츠 조정을 위해 전통적인 ML이나 규칙 기반 접근법 대신 Claude와 같은 LLM을 사용해야 한다는 주요 지표들입니다:

<section title="비용 효율적이고 빠른 구현을 원하는 경우">
전통적인 ML 방법은 상당한 엔지니어링 리소스, ML 전문 지식, 인프라 비용이 필요합니다. 인간 조정 시스템은 더 높은 비용이 발생합니다. Claude를 사용하면 훨씬 적은 시간과 비용으로 정교한 조정 시스템을 구축하고 실행할 수 있습니다.
</section>
<section title="의미적 이해와 빠른 결정을 모두 원하는 경우">
bag-of-words 모델이나 단순한 패턴 매칭과 같은 전통적인 ML 접근법은 콘텐츠의 톤, 의도, 맥락을 이해하는 데 어려움을 겪는 경우가 많습니다. 인간 조정 시스템은 의미적 의미를 이해하는 데 뛰어나지만 콘텐츠를 검토하는 데 시간이 필요합니다. Claude는 의미적 이해와 빠른 조정 결정을 제공하는 능력을 결합하여 이러한 격차를 해소합니다.
</section>
<section title="일관된 정책 결정이 필요한 경우">
Claude는 고급 추론 능력을 활용하여 복잡한 조정 가이드라인을 균일하게 해석하고 적용할 수 있습니다. 이러한 일관성은 모든 콘텐츠의 공정한 처리를 보장하고, 사용자 신뢰를 훼손할 수 있는 일관성 없거나 편향된 조정 결정의 위험을 줄입니다.
</section>
<section title="조정 정책이 시간이 지남에 따라 변경되거나 발전할 가능성이 있는 경우">
전통적인 ML 접근법이 확립되면 이를 변경하는 것은 노동 집약적이고 데이터 집약적인 작업입니다. 반면에 제품이나 고객 요구가 발전함에 따라 Claude는 광범위한 훈련 데이터 재라벨링 없이도 조정 정책의 변경이나 추가에 쉽게 적응할 수 있습니다.
</section>
<section title="조정 결정에 대한 해석 가능한 추론이 필요한 경우">
조정 결정 뒤의 명확한 설명을 사용자나 규제 기관에 제공하고자 한다면, Claude는 상세하고 일관된 정당화를 생성할 수 있습니다. 이러한 투명성은 콘텐츠 조정 관행에서 신뢰를 구축하고 책임성을 보장하는 데 중요합니다.
</section>
<section title="별도의 모델을 유지하지 않고 다국어 지원이 필요한 경우">
전통적인 ML 접근법은 일반적으로 지원하는 각 언어에 대해 별도의 모델이나 광범위한 번역 프로세스가 필요합니다. 인간 조정은 지원하는 각 언어에 능통한 인력을 고용해야 합니다. Claude의 다국어 기능을 통해 별도의 모델이나 광범위한 번역 프로세스 없이도 다양한 언어로 티켓을 분류할 수 있어 글로벌 고객 기반을 위한 조정을 간소화합니다.
</section>
<section title="멀티모달 지원이 필요한 경우">
Claude의 멀티모달 기능을 통해 텍스트와 이미지 모두에서 콘텐츠를 분석하고 해석할 수 있습니다. 이는 다양한 미디어 유형을 함께 평가해야 하는 환경에서 포괄적인 콘텐츠 조정을 위한 다재다능한 도구가 됩니다.
</section>

<Note>Anthropic은 모든 Claude 모델을 정직하고 도움이 되며 무해하도록 훈련했습니다. 이로 인해 사용된 프롬프트에 관계없이 Claude가 특히 위험하다고 간주되는 콘텐츠([허용 가능한 사용 정책](https://www.anthropic.com/legal/aup)에 따라)를 조정할 수 있습니다. 예를 들어, 사용자가 노골적인 성적 콘텐츠를 게시할 수 있도록 허용하려는 성인 웹사이트는 프롬프트에서 노골적인 성적 콘텐츠를 조정하지 않도록 지정하더라도 Claude가 여전히 노골적인 콘텐츠를 조정이 필요한 것으로 플래그를 지정할 수 있습니다. 조정 솔루션을 구축하기 전에 AUP를 미리 검토하는 것을 권장합니다.</Note>

### 조정할 콘텐츠 예제 생성
콘텐츠 조정 솔루션을 개발하기 전에 먼저 플래그를 지정해야 하는 콘텐츠와 플래그를 지정하지 않아야 하는 콘텐츠의 예제를 만드세요. 콘텐츠 조정 시스템이 효과적으로 처리하기 어려울 수 있는 엣지 케이스와 도전적인 시나리오를 포함해야 합니다. 그 후 예제를 검토하여 잘 정의된 조정 카테고리 목록을 만드세요.
예를 들어, 소셜 미디어 플랫폼에서 생성된 예제는 다음을 포함할 수 있습니다:

```python
allowed_user_comments = [
    'This movie was great, I really enjoyed it. The main actor really killed it!',
    'I hate Mondays.',
    'It is a great time to invest in gold!'
]

disallowed_user_comments = [
    'Delete this post now or you better hide. I am coming after you and your family.',
    'Stay away from the 5G cellphones!! They are using 5G to control you.',
    'Congratulations! You have won a $1,000 gift card. Click here to claim your prize!'
]

# Sample user comments to test the content moderation
user_comments = allowed_user_comments + disallowed_user_comments

# List of categories considered unsafe for content moderation
unsafe_categories = [
    'Child Exploitation',
    'Conspiracy Theories',
    'Hate',
    'Indiscriminate Weapons', 
    'Intellectual Property',
    'Non-Violent Crimes', 
    'Privacy',
    'Self-Harm',
    'Sex Crimes',
    'Sexual Content',
    'Specialized Advice',
    'Violent Crimes'
]
```

이러한 예제를 효과적으로 조정하려면 언어에 대한 미묘한 이해가 필요합니다. `This movie was great, I really enjoyed it. The main actor really killed it!`라는 댓글에서 콘텐츠 조정 시스템은 "killed it"이 은유이지 실제 폭력의 표시가 아님을 인식해야 합니다. 반대로 명시적인 폭력 언급이 없음에도 불구하고 `Delete this post now or you better hide. I am coming after you and your family.`라는 댓글은 콘텐츠 조정 시스템에 의해 플래그가 지정되어야 합니다.

`unsafe_categories` 목록은 특정 요구 사항에 맞게 사용자 정의할 수 있습니다. 예를 들어, 미성년자가 웹사이트에서 콘텐츠를 생성하는 것을 방지하려면 목록에 "Underage Posting"을 추가할 수 있습니다.

___

## Claude를 사용하여 콘텐츠를 조정하는 방법

### 적절한 Claude 모델 선택
모델을 선택할 때 데이터의 크기를 고려하는 것이 중요합니다. 비용이 우려사항이라면 Claude Haiku 3와 같은 더 작은 모델이 비용 효율성으로 인해 훌륭한 선택입니다. 다음은 월 10억 개의 게시물을 받는 소셜 미디어 플랫폼의 텍스트 조정 비용 추정입니다:

* **콘텐츠 크기**
    * 월 게시물 수: 10억 개
    * 게시물당 문자 수: 100개
    * 총 문자 수: 1000억 개

* **예상 토큰**
    * 입력 토큰: 286억 개 (3.5문자당 1토큰 가정)
    * 플래그된 메시지 비율: 3%
    * 플래그된 메시지당 출력 토큰: 50개
    * 총 출력 토큰: 15억 개

* **Claude Haiku 3 예상 비용**
    * 입력 토큰 비용: 2,860 MTok * \$0.25/MTok = \$715
    * 출력 토큰 비용: 1,500 MTok * \$1.25/MTok = \$1,875
    * 월 비용: \$715 + \$1,875 = \$2,590

* **Claude Sonnet 4.5 예상 비용**
    * 입력 토큰 비용: 2,860 MTok * \$3.00/MTok = \$8,580
    * 출력 토큰 비용: 1,500 MTok * \$15.00/MTok = \$22,500
    * 월 비용: \$8,580 + \$22,500 = \$31,080

<Tip>실제 비용은 이러한 추정치와 다를 수 있습니다. 이러한 추정치는 [배치 처리](#consider-batch-processing) 섹션에서 강조된 프롬프트를 기반으로 합니다. 응답에서 `explanation` 필드를 제거하여 출력 토큰을 더욱 줄일 수 있습니다.</Tip>  

### 강력한 프롬프트 구축

Claude를 콘텐츠 조정에 사용하려면 Claude가 애플리케이션의 조정 요구 사항을 이해해야 합니다. 조정 요구 사항을 정의할 수 있는 프롬프트를 작성하는 것부터 시작해보겠습니다:

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def moderate_message(message, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Construct the prompt for Claude, including the message and unsafe categories
    assessment_prompt = f"""
    Determine whether the following message warrants moderation, 
    based on the unsafe categories outlined below.
    
    Message:
    <message>{message}</message>

    Unsafe Categories:
    <categories>
    {unsafe_category_str}
    </categories>

    Respond with ONLY a JSON object, using the format below:
    {{
    "violation": <Boolean field denoting whether the message should be moderated>,
    "categories": [Comma-separated list of violated categories],
    "explanation": [Optional. Only include if there is a violation.]
    }}"""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the violation status from the assessment
    contains_violation = assessment['violation']
    
    # If there's a violation, get the categories and explanation; otherwise, use empty defaults
    violated_categories = assessment.get('categories', []) if contains_violation else []
    explanation = assessment.get('explanation') if contains_violation else None
    
    return contains_violation, violated_categories, explanation

# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    violation, violated_categories, explanation = moderate_message(comment, unsafe_categories)
    
    if violation:
        print(f"Violated Categories: {', '.join(violated_categories)}")
        print(f"Explanation: {explanation}")
    else:
        print("No issues detected.")
```

이 예제에서 `moderate_message` 함수는 안전하지 않은 콘텐츠 카테고리와 평가하려는 메시지를 포함하는 평가 프롬프트를 포함합니다. 프롬프트는 Claude에게 정의한 안전하지 않은 카테고리를 기반으로 메시지를 조정해야 하는지 평가하도록 요청합니다.

그런 다음 모델의 평가를 파싱하여 위반이 있는지 확인합니다. 위반이 있는 경우 Claude는 위반된 카테고리 목록과 메시지가 안전하지 않은 이유에 대한 설명도 반환합니다.

### 프롬프트 평가

콘텐츠 조정은 분류 문제입니다. 따라서 [분류 쿡북](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/classification/guide.ipynb)에서 설명한 동일한 기법을 사용하여 콘텐츠 조정 시스템의 정확도를 결정할 수 있습니다.

추가로 고려할 사항은 콘텐츠 조정을 이진 분류 문제로 처리하는 대신 다양한 위험 수준을 나타내는 여러 카테고리를 만들 수 있다는 것입니다. 여러 위험 수준을 만들면 조정의 공격성을 조정할 수 있습니다. 예를 들어, 고위험으로 간주되는 사용자 쿼리는 자동으로 차단하고, 중간 위험 쿼리가 많은 사용자는 인간 검토를 위해 플래그를 지정할 수 있습니다.

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def assess_risk_level(message, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Construct the prompt for Claude, including the message, unsafe categories, and risk level definitions
    assessment_prompt = f"""
    Assess the risk level of the following message warrants moderation, 
    based on the unsafe categories listed below.
    
Message:
<message>{message}</message>

Unsafe Categories:
<categories>
{unsafe_category_str}
</categories>

Assign a risk level based on your confidence that the user's message should be moderated 
based on the defined unsafe categories:

0 - No risk
1 - Low risk
2 - Medium risk
3 - High risk

Respond with ONLY a JSON object, using the format below:
{{
  "risk_level": <Numerical field denoting the risk level>,
  "categories": [Comma-separated list of violated categories],
  "explanation": <Optional. Only include if risk level is greater than 0>
}}"""

    # Send the request to Claude for risk assessment
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the risk level, violated categories, and explanation from the assessment
    risk_level = assessment["risk_level"]
    violated_categories = assessment["categories"]
    explanation = assessment.get("explanation")
    
    return risk_level, violated_categories, explanation

# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    risk_level, violated_categories, explanation = assess_risk_level(comment, unsafe_categories)
    
    print(f"Risk Level: {risk_level}")
    if violated_categories:
        print(f"Violated Categories: {', '.join(violated_categories)}")
    if explanation:
        print(f"Explanation: {explanation}")
```

이 코드는 Claude를 사용하여 메시지의 위험 수준을 평가하는 `assess_risk_level` 함수를 구현합니다. 이 함수는 메시지와 안전하지 않은 카테고리 목록을 입력으로 받습니다.

함수 내에서 평가할 메시지, 안전하지 않은 카테고리, 위험 수준 평가를 위한 구체적인 지침을 포함하여 Claude를 위한 프롬프트가 생성됩니다. 프롬프트는 Claude에게 위험 수준, 위반된 카테고리, 선택적 설명을 포함하는 JSON 객체로 응답하도록 지시합니다.

이 접근법은 위험 수준을 할당하여 유연한 콘텐츠 조정을 가능하게 합니다. 평가된 위험 수준에 따라 콘텐츠 필터링을 자동화하거나 인간 검토를 위해 댓글을 플래그하는 더 큰 시스템에 원활하게 통합될 수 있습니다. 예를 들어, 이 코드를 실행할 때 `Delete this post now or you better hide. I am coming after you and your family.`라는 댓글은 위험한 위협으로 인해 고위험으로 식별됩니다. 반대로 `Stay away from the 5G cellphones!! They are using 5G to control you.`라는 댓글은 중간 위험으로 분류됩니다.

### 프롬프트 배포

솔루션의 품질에 확신이 들면 프로덕션에 배포할 때입니다. 프로덕션에서 콘텐츠 조정을 사용할 때 따라야 할 몇 가지 모범 사례는 다음과 같습니다:

1. **사용자에게 명확한 피드백 제공:** 콘텐츠 조정으로 인해 사용자 입력이 차단되거나 응답에 플래그가 지정될 때, 사용자가 메시지에 플래그가 지정된 이유와 적절하게 다시 표현하는 방법을 이해할 수 있도록 유익하고 건설적인 피드백을 제공하세요. 위의 코딩 예제에서는 Claude 응답의 `explanation` 태그를 통해 이를 수행합니다.

2. **조정된 콘텐츠 분석:** 조정 시스템에 의해 플래그가 지정되는 콘텐츠 유형을 추적하여 트렌드와 개선 가능한 잠재적 영역을 식별하세요.

3. **지속적인 평가 및 개선:** 정밀도 및 재현율 추적과 같은 메트릭을 사용하여 콘텐츠 조정 시스템의 성능을 정기적으로 평가하세요. 이 데이터를 사용하여 조정 프롬프트, 키워드 및 평가 기준을 반복적으로 개선하세요.

___

## 성능 향상

복잡한 시나리오에서는 표준 [프롬프트 엔지니어링 기법](/docs/ko/build-with-claude/prompt-engineering/overview)을 넘어서는 추가 전략을 고려하여 성능을 향상시키는 것이 도움이 될 수 있습니다. 다음은 몇 가지 고급 전략입니다:

### 주제 정의 및 예제 제공

프롬프트에 안전하지 않은 카테고리를 나열하는 것 외에도 각 카테고리와 관련된 정의와 문구를 제공하여 추가적인 개선을 할 수 있습니다.

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

# Dictionary of categories considered unsafe for content moderation, with their definitions
unsafe_category_definitions = {
    'Child Exploitation': 'Content that depicts child nudity or that enables, encourages, excuses, or depicts the sexual abuse of children.',
    'Conspiracy Theories': """Content that promotes or endorses unfounded, false, or misleading theories about events, situations, or individuals, 
        which can incite harm or undermine public trust in institutions or facts.""",
    'Hate': """Content that is hateful toward people on the basis of their protected characteristics 
        (race, color, ethnicity, national origin, disability, religious affiliation, caste, sexual orientation, sex, gender identity and serious disease), 
        or content that perpetuates negative stereotypes.""",
    'Indiscriminate Weapons': 'Content that enables the creation of weapons of mass destruction.', 
    'Intellectual Property': 'Content that violates the intellectual property rights of third parties.',
    'Non-Violent Crimes': 'Content that enables, encourages, or excuses the commission of non-violent crimes.', 
    'Privacy': 'Content that contains sensitive, personal information about private individuals.',
    'Self-Harm': 'Content that enables, encourages, or excuses acts of intentional self-harm.',
    'Sex Crimes': 'Content that enables, encourages, or excuses the commission of sex crimes.',
    'Sexual Content': 'Content that contains erotic descriptions or explicit depictions of sex acts.',
    'Specialized Advice': """Content that contains financial, medical, or legal advice. 
        Financial advice includes guidance on investments, stocks, bonds, or any financial planning.""",
    'Violent Crimes': 'Content that enables, encourages, or excuses the commission of violent crimes.', 
}


def moderate_message_with_definitions(message, unsafe_category_definitions):
    # Format unsafe categories string, with each category and its definition on a new line
    unsafe_category_str = '\n'.join(f'{category}: {definition}' 
        for category, definition in unsafe_category_definitions.items()
    )
    
    # Construct the prompt for Claude, including the message and unsafe categories
    assessment_prompt = f"""Determine whether the following message warrants moderation, based on the unsafe categories outlined below.

Message:
<message>{message}</message>

Unsafe Categories and Their Definitions:
<categories>
{unsafe_category_str}
</categories>

It's important that you remember all unsafe categories and their definitions.

Respond with ONLY a JSON object, using the format below:
{{
  "violation": <Boolean field denoting whether the message should be moderated>,
  "categories": [Comma-separated list of violated categories],
  "explanation": [Optional. Only include if there is a violation.]
}}"""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the violation status from the assessment
    contains_violation = assessment['violation']
    
    # If there's a violation, get the categories and explanation; otherwise, use empty defaults
    violated_categories = assessment.get('categories', []) if contains_violation else []
    explanation = assessment.get('explanation') if contains_violation else None
    
    return contains_violation, violated_categories, explanation


# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    violation, violated_categories, explanation = moderate_message_with_definitions(comment, unsafe_category_definitions)
    
    if violation:
        print(f"Violated Categories: {', '.join(violated_categories)}")
        print(f"Explanation: {explanation}")
    else:
        print("No issues detected.")
```

`moderate_message_with_definitions` 함수는 각 안전하지 않은 카테고리를 상세한 정의와 쌍을 이룰 수 있도록 하여 이전의 `moderate_message` 함수를 확장합니다. 이는 원래 함수의 `unsafe_categories` 목록을 `unsafe_category_definitions` 딕셔너리로 교체하여 코드에서 발생합니다. 이 딕셔너리는 각 안전하지 않은 카테고리를 해당 정의에 매핑합니다. 카테고리 이름과 정의 모두 프롬프트에 포함됩니다.

특히 `Specialized Advice` 카테고리의 정의는 이제 금지되어야 하는 금융 조언의 유형을 명시합니다. 결과적으로 이전에 `moderate_message` 평가를 통과했던 `It's a great time to invest in gold!`라는 댓글이 이제 위반을 트리거합니다.

### 배치 처리 고려

실시간 조정이 필요하지 않은 상황에서 비용을 줄이려면 메시지를 배치로 조정하는 것을 고려하세요. 프롬프트의 컨텍스트 내에 여러 메시지를 포함하고 Claude에게 어떤 메시지를 조정해야 하는지 평가하도록 요청하세요.

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def batch_moderate_messages(messages, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Format messages string, with each message wrapped in XML-like tags and given an ID
    messages_str = '\n'.join([f'<message id={idx}>{msg}</message>' for idx, msg in enumerate(messages)])
    
    # Construct the prompt for Claude, including the messages and unsafe categories
    assessment_prompt = f"""Determine the messages to moderate, based on the unsafe categories outlined below.

Messages:
<messages>
{messages_str}
</messages>

Unsafe categories and their definitions:
<categories>
{unsafe_category_str}
</categories>

Respond with ONLY a JSON object, using the format below:
{{
  "violations": [
    {{
      "id": <message id>,
      "categories": [list of violated categories],
      "explanation": <Explanation of why there's a violation>
    }},
    ...
  ]
}}

Important Notes:
- Remember to analyze every message for a violation.
- Select any number of violations that reasonably apply."""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=2048,  # Increased max token count to handle batches
        temperature=0,    # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    return assessment


# Process the batch of comments and get the response
response_obj = batch_moderate_messages(user_comments, unsafe_categories)

# Print the results for each detected violation
for violation in response_obj['violations']:
    print(f"""Comment: {user_comments[violation['id']]}
Violated Categories: {', '.join(violation['categories'])}
Explanation: {violation['explanation']}
""")
```
이 예제에서 `batch_moderate_messages` 함수는 단일 Claude API 호출로 전체 메시지 배치의 조정을 처리합니다.
함수 내에서 평가할 메시지 목록, 정의된 안전하지 않은 콘텐츠 카테고리 및 그 설명을 포함하는 프롬프트가 생성됩니다. 프롬프트는 Claude에게 위반을 포함하는 모든 메시지를 나열하는 JSON 객체를 반환하도록 지시합니다. 응답의 각 메시지는 입력 목록에서 메시지의 위치에 해당하는 id로 식별됩니다.
특정 요구 사항에 대한 최적의 배치 크기를 찾는 것은 약간의 실험이 필요할 수 있다는 점을 염두에 두세요. 더 큰 배치 크기는 비용을 낮출 수 있지만 품질이 약간 감소할 수도 있습니다. 또한 더 긴 응답을 수용하기 위해 Claude API 호출에서 `max_tokens` 매개변수를 늘려야 할 수도 있습니다. 선택한 모델이 출력할 수 있는 최대 토큰 수에 대한 자세한 내용은 [모델 비교 페이지](/docs/ko/about-claude/models#model-comparison-table)를 참조하세요.

<CardGroup cols={2}> 
  <Card title="콘텐츠 조정 쿡북" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fmoderation%5Ffilter.ipynb">
    Claude를 콘텐츠 조정에 사용하는 방법에 대한 완전히 구현된 코드 기반 예제를 확인하세요.
  </Card>
  <Card title="가드레일 가이드" icon="link" href="/docs/ko/test-and-evaluate/strengthen-guardrails/reduce-hallucinations">
    Claude와의 상호작용을 조정하는 기법에 대한 가드레일 가이드를 탐색하세요.
  </Card>
</CardGroup>