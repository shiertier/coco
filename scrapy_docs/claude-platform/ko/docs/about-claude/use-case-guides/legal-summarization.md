# 법률 문서 요약

이 가이드는 Claude의 고급 자연어 처리 기능을 활용하여 법률 문서를 효율적으로 요약하고, 핵심 정보를 추출하며, 법률 연구를 가속화하는 방법을 설명합니다. Claude를 사용하면 계약서, 소송 준비 및 규제 업무 검토를 간소화하여 시간을 절약하고 법률 프로세스의 정확성을 보장할 수 있습니다.

---

> Claude를 사용한 법률 문서 요약 구현의 예시를 보려면 [요약 cookbook](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb)을 방문하세요.

## Claude로 구축하기 전에

### Claude를 법률 문서 요약에 사용할지 결정하기

Claude와 같은 LLM을 사용하여 법률 문서를 요약해야 하는 주요 지표는 다음과 같습니다:

<section title="많은 양의 문서를 효율적이고 저렴하게 검토하고 싶습니다">
대규모 문서 검토는 수동으로 수행할 때 시간이 많이 걸리고 비용이 많이 들 수 있습니다. Claude는 방대한 양의 법률 문서를 빠르게 처리하고 요약할 수 있으므로 문서 검토와 관련된 시간과 비용을 크게 줄일 수 있습니다. 이 기능은 실사, 계약 분석 또는 소송 발견과 같이 효율성이 중요한 작업에 특히 유용합니다.
</section>
<section title="주요 메타데이터의 자동 추출이 필요합니다">
Claude는 관련 당사자, 날짜, 계약 조건 또는 특정 조항과 같은 법률 문서의 중요한 메타데이터를 효율적으로 추출하고 분류할 수 있습니다. 이 자동 추출은 정보를 정리하여 대규모 문서 세트를 더 쉽게 검색, 분석 및 관리할 수 있도록 합니다. 계약 관리, 규정 준수 확인 또는 법률 정보의 검색 가능한 데이터베이스 생성에 특히 유용합니다.
</section>
<section title="명확하고 간결하며 표준화된 요약을 생성하고 싶습니다">
Claude는 미리 정해진 형식을 따르는 구조화된 요약을 생성할 수 있으므로 법률 전문가가 다양한 문서의 핵심 요점을 빠르게 파악할 수 있습니다. 이러한 표준화된 요약은 가독성을 개선하고, 문서 간 비교를 용이하게 하며, 특히 복잡한 법률 언어나 기술 용어를 다룰 때 전반적인 이해도를 향상시킬 수 있습니다.
</section>
<section title="요약에 대한 정확한 인용이 필요합니다">
법률 요약을 작성할 때 적절한 출처 표시와 인용은 신뢰성을 보장하고 법률 기준을 준수하기 위해 중요합니다. Claude는 참조된 모든 법률 요점에 대해 정확한 인용을 포함하도록 프롬프트할 수 있으므로 법률 전문가가 요약된 정보를 더 쉽게 검토하고 확인할 수 있습니다.
</section>
<section title="법률 연구 프로세스를 간소화하고 가속화하고 싶습니다">
Claude는 판례법, 법령 및 법률 해설의 대량을 빠르게 분석하여 법률 연구를 지원할 수 있습니다. 관련 판례를 식별하고, 핵심 법률 원칙을 추출하며, 복잡한 법률 주장을 요약할 수 있습니다. 이 기능은 연구 프로세스를 크게 가속화하여 법률 전문가가 더 높은 수준의 분석 및 전략 개발에 집중할 수 있도록 합니다.
</section>

### 요약에서 추출할 세부 정보 결정하기
어떤 문서든 단 하나의 올바른 요약은 없습니다. 명확한 지시가 없으면 Claude가 포함할 세부 정보를 결정하기 어려울 수 있습니다. 최적의 결과를 얻으려면 요약에 포함하고 싶은 구체적인 정보를 파악하세요.

예를 들어, 부분 임차 계약을 요약할 때 다음과 같은 핵심 요점을 추출하고 싶을 수 있습니다:

```python
details_to_extract = [
    'Parties involved (sublessor, sublessee, and original lessor)',
    'Property details (address, description, and permitted use)', 
    'Term and rent (start date, end date, monthly rent, and security deposit)',
    'Responsibilities (utilities, maintenance, and repairs)',
    'Consent and notices (landlord\'s consent, and notice requirements)',
    'Special provisions (furniture, parking, and subletting restrictions)'
]
```

### 성공 기준 수립하기

요약의 품질을 평가하는 것은 악명 높게 어려운 작업입니다. 다른 많은 자연어 처리 작업과 달리 요약 평가는 종종 명확한 객관적 지표가 부족합니다. 이 프로세스는 매우 주관적일 수 있으며, 다양한 독자들이 요약의 다양한 측면을 중요하게 생각합니다. Claude가 법률 문서 요약을 얼마나 잘 수행하는지 평가할 때 고려할 수 있는 기준은 다음과 같습니다.

<section title="사실적 정확성">
요약은 문서의 사실, 법률 개념 및 핵심 요점을 정확하게 나타내야 합니다.
</section>
<section title="법률적 정확성">
용어와 법령, 판례법 또는 규정에 대한 참조는 정확해야 하며 법률 기준에 부합해야 합니다.
</section>
<section title="간결성">
요약은 법률 문서를 중요한 요점으로 축약해야 하며 중요한 세부 정보를 잃지 않아야 합니다.
</section>
<section title="일관성">
여러 문서를 요약할 때 LLM은 각 요약에 대해 일관된 구조와 접근 방식을 유지해야 합니다.
</section>
<section title="가독성">
텍스트는 명확하고 이해하기 쉬워야 합니다. 대상 독자가 법률 전문가가 아닌 경우 요약에 대상 독자를 혼동시킬 수 있는 법률 용어가 포함되어서는 안 됩니다.
</section>
<section title="편향성 및 공정성">
요약은 법률 주장과 입장에 대한 편견 없고 공정한 묘사를 제시해야 합니다.
</section>

자세한 내용은 [성공 기준 수립](/docs/ko/test-and-evaluate/define-success)에 대한 가이드를 참조하세요.

---

## Claude를 사용하여 법률 문서를 요약하는 방법

### 올바른 Claude 모델 선택하기

모델 정확도는 법률 문서를 요약할 때 매우 중요합니다. Claude Sonnet 4.5는 높은 정확도가 필요한 이러한 사용 사례에 탁월한 선택입니다. 문서의 크기와 수량이 커서 비용이 문제가 되기 시작하면 Claude Haiku 4.5와 같은 더 작은 모델을 사용해 볼 수도 있습니다.

이러한 비용을 추정하는 데 도움이 되도록 Sonnet과 Haiku를 모두 사용하여 1,000개의 부분 임차 계약을 요약하는 비용 비교는 다음과 같습니다:

* **콘텐츠 크기**
    * 계약 수: 1,000
    * 계약당 문자: 300,000
    * 총 문자: 300M

* **예상 토큰**
    * 입력 토큰: 86M (1 토큰당 3.5자 가정)
    * 요약당 출력 토큰: 350
    * 총 출력 토큰: 350,000
 
* **Claude Sonnet 4.5 예상 비용**
    * 입력 토큰 비용: 86 MTok * \$3.00/MTok = \$258
    * 출력 토큰 비용: 0.35 MTok * \$15.00/MTok = \$5.25
    * 총 비용: \$258.00 + \$5.25 = \$263.25

* **Claude Haiku 3 예상 비용**
    * 입력 토큰 비용: 86 MTok * \$0.25/MTok = \$21.50
    * 출력 토큰 비용: 0.35 MTok * \$1.25/MTok = \$0.44
    * 총 비용: \$21.50 + \$0.44 = \$21.96

<Tip>실제 비용은 이러한 추정치와 다를 수 있습니다. 이러한 추정치는 [프롬프팅](#build-a-strong-prompt) 섹션에서 강조된 예시를 기반으로 합니다.</Tip>

### 문서를 Claude가 처리할 수 있는 형식으로 변환하기

문서 요약을 시작하기 전에 데이터를 준비해야 합니다. 여기에는 PDF에서 텍스트 추출, 텍스트 정리 및 Claude에서 처리할 준비가 되어 있는지 확인하는 작업이 포함됩니다.

다음은 샘플 pdf에서 이 프로세스의 데모입니다:

```python
from io import BytesIO
import re

import pypdf
import requests

def get_llm_text(pdf_file):
    reader = pypdf.PdfReader(pdf_file)
    text = "\n".join([page.extract_text() for page in reader.pages])

    # Remove extra whitespace
    text = re.sub(r'\s+', ' ', text) 

    # Remove page numbers
    text = re.sub(r'\n\s*\d+\s*\n', '\n', text) 

    return text


# Create the full URL from the GitHub repository
url = "https://raw.githubusercontent.com/anthropics/anthropic-cookbook/main/skills/summarization/data/Sample Sublease Agreement.pdf"
url = url.replace(" ", "%20")

# Download the PDF file into memory
response = requests.get(url)

# Load the PDF from memory
pdf_file = BytesIO(response.content)

document_text = get_llm_text(pdf_file) 
print(document_text[:50000]) 
```

이 예시에서는 먼저 [요약 cookbook](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/data/Sample%20Sublease%20Agreement.pdf)에서 사용되는 샘플 부분 임차 계약의 pdf를 다운로드합니다. 이 계약은 [sec.gov 웹사이트](https://www.sec.gov/Archives/edgar/data/1045425/000119312507044370/dex1032.htm)에서 공개적으로 사용 가능한 부분 임차 계약에서 출처를 얻었습니다.

pypdf 라이브러리를 사용하여 pdf의 내용을 추출하고 텍스트로 변환합니다. 그런 다음 텍스트 데이터는 추가 공백과 페이지 번호를 제거하여 정리됩니다.

### 강력한 프롬프트 구축하기

Claude는 다양한 요약 스타일에 적응할 수 있습니다. 프롬프트의 세부 정보를 변경하여 Claude가 더 길거나 짧게 표현하도록 안내하거나, 더 많거나 적은 기술 용어를 포함하거나, 현재 상황에 대한 더 높거나 낮은 수준의 요약을 제공할 수 있습니다.

다음은 부분 임차 계약을 분석할 때 생성된 요약이 일관된 구조를 따르도록 하는 프롬프트를 만드는 방법의 예시입니다:

```python
import anthropic

# Initialize the Anthropic client
client = anthropic.Anthropic()

def summarize_document(text, details_to_extract, model="claude-sonnet-4-5", max_tokens=1000):

    # Format the details to extract to be placed within the prompt's context
    details_to_extract_str = '\n'.join(details_to_extract)
    
    # Prompt the model to summarize the sublease agreement
    prompt = f"""Summarize the following sublease agreement. Focus on these key aspects:

    {details_to_extract_str}

    Provide the summary in bullet points nested within the XML header for each section. For example:

    <parties involved>
    - Sublessor: [Name]
    // Add more details as needed
    </parties involved>
    
    If any information is not explicitly stated in the document, note it as "Not specified". Do not preamble.

    Sublease agreement text:
    {text}
    """

    response = client.messages.create(
        model=model,
        max_tokens=max_tokens,
        system="You are a legal analyst specializing in real estate law, known for highly accurate and detailed summaries of sublease agreements.",
        messages=[
            {"role": "user", "content": prompt},
            {"role": "assistant", "content": "Here is the summary of the sublease agreement: <summary>"}
        ],
        stop_sequences=["</summary>"]
    )

    return response.content[0].text

sublease_summary = summarize_document(document_text, details_to_extract)
print(sublease_summary)
```

이 코드는 Claude를 사용하여 부분 임차 계약의 내용을 요약하는 `summarize_document` 함수를 구현합니다. 이 함수는 텍스트 문자열과 추출할 세부 정보 목록을 입력으로 받습니다. 이 예시에서는 이전 코드 스니펫에서 정의한 `document_text` 및 `details_to_extract` 변수를 사용하여 함수를 호출합니다.

함수 내에서 Claude에 대한 프롬프트가 생성되며, 여기에는 요약할 문서, 추출할 세부 정보 및 문서 요약에 대한 구체적인 지침이 포함됩니다. 프롬프트는 Claude에게 각 세부 정보의 요약을 XML 헤더 내에 중첩되어 응답하도록 지시합니다.

요약의 각 섹션을 태그 내에 출력하기로 결정했으므로 각 섹션을 후처리 단계로 쉽게 구문 분석할 수 있습니다. 이 접근 방식은 사용 사례에 맞게 조정할 수 있는 구조화된 요약을 가능하게 하므로 각 요약이 동일한 패턴을 따릅니다.

### 프롬프트 평가하기

프롬프팅은 프로덕션 준비가 되려면 테스트 및 최적화가 필요한 경우가 많습니다. 솔루션의 준비 상태를 결정하려면 정량적 및 정성적 방법을 결합하는 체계적인 프로세스를 사용하여 요약의 품질을 평가하세요. 정의된 성공 기준을 기반으로 [강력한 경험적 평가](/docs/ko/test-and-evaluate/develop-tests#building-evals-and-test-cases)를 만들면 프롬프트를 최적화할 수 있습니다. 경험적 평가에 포함할 수 있는 일부 지표는 다음과 같습니다:

<section title="ROUGE 점수">
이는 생성된 요약과 전문가가 작성한 참조 요약 간의 겹침을 측정합니다. 이 지표는 주로 재현율에 초점을 맞추며 콘텐츠 범위를 평가하는 데 유용합니다.
</section>
<section title="BLEU 점수">
원래 기계 번역을 위해 개발되었지만 이 지표는 요약 작업에 맞게 조정할 수 있습니다. BLEU 점수는 생성된 요약과 참조 요약 간의 n-gram 일치의 정확도를 측정합니다. 점수가 높을수록 생성된 요약에 참조 요약과 유사한 구문과 용어가 포함되어 있음을 나타냅니다.
</section>
<section title="문맥적 임베딩 유사성">
이 지표는 생성된 요약과 참조 요약의 벡터 표현(임베딩)을 만드는 것을 포함합니다. 그런 다음 이러한 임베딩 간의 유사성을 계산하며, 종종 코사인 유사성을 사용합니다. 유사성 점수가 높을수록 생성된 요약이 정확한 표현이 다르더라도 참조 요약의 의미론적 의미와 문맥을 포착함을 나타냅니다.
</section>
<section title="LLM 기반 채점">
이 방법은 Claude와 같은 LLM을 사용하여 채점 루브릭에 대해 생성된 요약의 품질을 평가하는 것을 포함합니다. 루브릭은 정확성, 완전성 및 일관성과 같은 주요 요소를 평가하여 특정 요구 사항에 맞게 조정할 수 있습니다. LLM 기반 채점 구현에 대한 지침은 이러한 [팁](/docs/ko/test-and-evaluate/develop-tests#tips-for-llm-based-grading)을 참조하세요.
</section>
<section title="인간 평가">
참조 요약을 만드는 것 외에도 법률 전문가는 생성된 요약의 품질을 평가할 수도 있습니다. 이것은 규모에 따라 비용이 많이 들고 시간이 많이 걸리지만, 프로덕션에 배포하기 전에 몇 가지 요약에 대한 건전성 검사로 종종 수행됩니다.
</section>

### 프롬프트 배포하기

프로덕션에 솔루션을 배포할 때 염두에 두어야 할 몇 가지 추가 고려 사항이 있습니다.

1. **책임 보장:** 요약의 오류로 인한 법적 영향을 이해하세요. 이는 조직이나 클라이언트에 대한 법적 책임으로 이어질 수 있습니다. 요약이 AI에 의해 생성되었으며 법률 전문가가 검토해야 함을 명확히 하는 면책 조항 또는 법적 공지를 제공하세요.

2. **다양한 문서 유형 처리:** 이 가이드에서는 PDF에서 텍스트를 추출하는 방법을 설명했습니다. 실제로 문서는 다양한 형식(PDF, Word 문서, 텍스트 파일 등)으로 제공될 수 있습니다. 데이터 추출 파이프라인이 받을 것으로 예상되는 모든 파일 형식을 변환할 수 있는지 확인하세요.

3. **Claude에 대한 API 호출 병렬화:** 많은 수의 토큰이 있는 긴 문서는 Claude가 요약을 생성하는 데 최대 1분이 걸릴 수 있습니다. 대규모 문서 컬렉션의 경우 Claude에 대한 API 호출을 병렬로 보내서 요약을 합리적인 시간 내에 완료할 수 있습니다. Anthropic의 [속도 제한](/docs/ko/api/rate-limits#rate-limits)을 참조하여 병렬로 수행할 수 있는 최대 API 호출 수를 결정하세요.

---

## 성능 개선하기

복잡한 시나리오에서는 표준 [프롬프트 엔지니어링 기법](/docs/ko/build-with-claude/prompt-engineering/overview)을 넘어 성능을 개선하기 위한 추가 전략을 고려하는 것이 도움이 될 수 있습니다. 다음은 몇 가지 고급 전략입니다:

### 메타 요약을 수행하여 긴 문서 요약하기

법률 문서 요약은 종종 긴 문서 또는 여러 관련 문서를 한 번에 처리하는 것을 포함하므로 Claude의 컨텍스트 윈도우를 초과할 수 있습니다. 이 사용 사례를 처리하기 위해 메타 요약이라는 청킹 방법을 사용할 수 있습니다. 이 기법은 문서를 더 작고 관리 가능한 청크로 분해한 다음 각 청크를 별도로 처리하는 것을 포함합니다. 그런 다음 각 청크의 요약을 결합하여 전체 문서의 메타 요약을 만들 수 있습니다.

다음은 메타 요약을 수행하는 방법의 예시입니다:

```python
import anthropic

# Initialize the Anthropic client
client = anthropic.Anthropic()

def chunk_text(text, chunk_size=20000):
    return [text[i:i+chunk_size] for i in range(0, len(text), chunk_size)]

def summarize_long_document(text, details_to_extract, model="claude-sonnet-4-5", max_tokens=1000):

    # Format the details to extract to be placed within the prompt's context
    details_to_extract_str = '\n'.join(details_to_extract)

    # Iterate over chunks and summarize each one
    chunk_summaries = [summarize_document(chunk, details_to_extract, model=model, max_tokens=max_tokens) for chunk in chunk_text(text)]
    
    final_summary_prompt = f"""
    
    You are looking at the chunked summaries of multiple documents that are all related. 
    Combine the following summaries of the document from different truthful sources into a coherent overall summary:

    <chunked_summaries>
    {"".join(chunk_summaries)}
    </chunked_summaries>

    Focus on these key aspects:
    {details_to_extract_str})

    Provide the summary in bullet points nested within the XML header for each section. For example:

    <parties involved>
    - Sublessor: [Name]
    // Add more details as needed
    </parties involved>
    
    If any information is not explicitly stated in the document, note it as "Not specified". Do not preamble.
    """

    response = client.messages.create(
        model=model,
        max_tokens=max_tokens,
        system="You are a legal expert that summarizes notes on one document.",
        messages=[
            {"role": "user",  "content": final_summary_prompt},
            {"role": "assistant", "content": "Here is the summary of the sublease agreement: <summary>"}

        ],
        stop_sequences=["</summary>"]
    )
    
    return response.content[0].text

long_summary = summarize_long_document(document_text, details_to_extract)
print(long_summary)
```

`summarize_long_document` 함수는 문서를 더 작은 청크로 분할하고 각 청크를 개별적으로 요약하여 이전의 `summarize_document` 함수를 기반으로 합니다.

코드는 원본 문서 내의 20,000자의 각 청크에 `summarize_document` 함수를 적용하여 이를 달성합니다. 그런 다음 개별 요약을 결합하고 이러한 청크 요약에서 최종 요약을 만듭니다.

`summarize_long_document` 함수는 전체 문서가 Claude의 컨텍스트 윈도우 내에 맞으므로 예시 pdf에 반드시 필요하지는 않습니다. 그러나 Claude의 컨텍스트 윈도우를 초과하는 문서나 여러 관련 문서를 함께 요약할 때 필수가 됩니다. 어쨌든 이 메타 요약 기법은 종종 이전의 단일 요약 접근 방식에서 놓친 최종 요약의 추가 중요한 세부 정보를 포착합니다.

### 요약 인덱싱된 문서를 사용하여 대규모 문서 컬렉션 탐색하기

LLM을 사용하여 문서 컬렉션을 검색하는 것은 일반적으로 검색 증강 생성(RAG)을 포함합니다. 그러나 대규모 문서를 포함하거나 정확한 정보 검색이 중요한 시나리오에서는 기본 RAG 접근 방식이 불충분할 수 있습니다. 요약 인덱싱된 문서는 전통적인 RAG 방법보다 적은 컨텍스트를 사용하여 검색을 위한 문서의 순위를 매기는 더 효율적인 방법을 제공하는 고급 RAG 접근 방식입니다. 이 접근 방식에서는 먼저 Claude를 사용하여 코퍼스의 각 문서에 대한 간결한 요약을 생성한 다음 Claude를 사용하여 요청되는 쿼리에 대한 각 요약의 관련성을 순위를 매깁니다. 이 접근 방식에 대한 자세한 내용과 코드 기반 예시는 [요약 cookbook](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb)의 요약 인덱싱된 문서 섹션을 확인하세요.

### Claude를 미세 조정하여 데이터 세트에서 학습하기

Claude의 요약 생성 능력을 개선하기 위한 또 다른 고급 기법은 미세 조정입니다. 미세 조정은 법률 요약 요구 사항에 특별히 맞춘 사용자 정의 데이터 세트에서 Claude를 훈련하는 것을 포함하므로 Claude가 사용 사례에 적응하도록 합니다. 다음은 미세 조정을 수행하는 방법에 대한 개요입니다:

1. **오류 식별:** Claude의 요약이 부족한 인스턴스를 수집하여 시작하세요. 여기에는 중요한 법률 세부 정보 누락, 문맥 오해 또는 부적절한 법률 용어 사용이 포함될 수 있습니다.

2. **데이터 세트 큐레이션:** 이러한 문제를 식별한 후 이러한 문제가 있는 예시의 데이터 세트를 컴파일하세요. 이 데이터 세트에는 원본 법률 문서와 수정된 요약이 포함되어야 하므로 Claude가 원하는 동작을 학습합니다.

3. **미세 조정 수행:** 미세 조정은 큐레이션된 데이터 세트에서 모델을 재훈련하여 가중치와 매개변수를 조정하는 것을 포함합니다. 이 재훈련은 Claude가 법률 도메인의 특정 요구 사항을 더 잘 이해하도록 도와 표준에 따라 문서를 요약하는 능력을 개선합니다.

4. **반복적 개선:** 미세 조정은 일회성 프로세스가 아닙니다. Claude가 계속 요약을 생성하면서 성능이 저조한 새로운 예시를 반복적으로 추가하여 기능을 더욱 개선할 수 있습니다. 시간이 지남에 따라 이 지속적인 피드백 루프는 법률 요약 작업에 매우 특화된 모델을 만들 것입니다.

<Tip>미세 조정은 현재 Amazon Bedrock을 통해서만 사용 가능합니다. 추가 세부 정보는 [AWS 출시 블로그](https://aws.amazon.com/blogs/machine-learning/fine-tune-anthropics-claude-3-haiku-in-amazon-bedrock-to-boost-model-accuracy-and-quality/)에서 확인할 수 있습니다.</Tip>

<CardGroup cols={2}> 
  <Card title="요약 cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb">
    Claude를 사용하여 계약을 요약하는 방법의 완전히 구현된 코드 기반 예시를 봅니다.
  </Card>
  <Card title="인용 cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb">
    정보의 정확성과 설명 가능성을 보장하는 방법에 대한 지침을 위해 인용 cookbook 레시피를 탐색합니다.
  </Card>
</CardGroup>