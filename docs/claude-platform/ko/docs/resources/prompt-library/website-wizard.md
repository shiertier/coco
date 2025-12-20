# 웹사이트 마법사

사용자 사양에 따라 단일 페이지 웹사이트를 생성합니다.

---

> 이 프롬프트를 우리 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 당신의 임무는 주어진 사양에 따라 단일 페이지 웹사이트를 생성하는 것으로, JavaScript와 CSS가 임베드된 HTML 파일로 제공됩니다. 웹사이트는 드롭다운 메뉴, 동적 텍스트 및 콘텐츠, 클릭 가능한 버튼 등과 같은 다양한 매력적이고 상호작용적인 디자인 기능을 통합해야 합니다. 디자인이 시각적으로 매력적이고, 반응형이며, 사용자 친화적인지 확인하세요. HTML, CSS, JavaScript 코드는 잘 구조화되고, 효율적으로 조직되며, 가독성과 유지보수성을 위해 적절히 주석 처리되어야 합니다. |
| User   | "EduQuest"라는 온라인 학습 플랫폼을 위한 단일 페이지 웹사이트를 다음 기능과 섹션으로 생성하세요:<br/><br/>1. 과정 카테고리(수학, 과학, 언어, 예술) 링크와 검색 바가 있는 고정 내비게이션 바.<br/>2. 온라인으로 학습하는 학생들을 보여주는 비디오 배경, "자신만의 속도로 학습하세요," "새로운 열정을 발견하세요," "시야를 넓히세요"를 3초마다 회전하는 동적 태그라인, 그리고 과정 카탈로그로 이어지는 "시작하기" 버튼이 있는 히어로 섹션.<br/>3. 과정 이미지, 제목, 강사, 설명을 위한 플레이스홀더가 있는 과정 카드를 표시하는 추천 과정 섹션.<br/>4. 학습 스타일과 관심사를 결정하는 짧은 퀴즈와 퀴즈를 시작하는 버튼이 있는 상호작용적인 "학습 경로" 섹션.<br/>5. 만족한 학생들의 추천사를 특징으로 하는 "성공 사례" 섹션으로, 추천사 텍스트와 학생 이름을 위한 플레이스홀더 포함.<br/>6. 플랫폼의 블로그, FAQ, 개인정보 보호정책 링크와 연락처 양식 및 고객 지원 정보가 있는 모달 창을 여는 "문의하기" 버튼이 있는 푸터.<br/><br/>비디오 배경, 과정 카드, 추천사를 위한 필러 플레이스홀더 콘텐츠를 포함하세요. CSS 스타일을 `<head>` 섹션의 `<style>` 태그 내에 임베드하고 JavaScript 코드를 `<body>` 섹션 끝의 `<script>` 태그 내에 배치하세요.<br/><br/>JavaScript 코드는 히어로 섹션의 동적 태그라인을 처리하여 3초마다 다른 태그라인들을 회전시켜야 합니다. |

## 예시 출력

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>EduQuest - Online Learning Platform</title>
    <style>
        /* CSS Styles */
        body {
            font-family: Arial, sans-serif;
            margin: 0;
            padding: 0;
        }

        header {
            background-color: #333;
            color: #fff;
            padding: 20px;
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            z-index: 999;
        }

        nav ul {
            list-style: none;
            margin: 0;
            padding: 0;
            display: flex;
            justify-content: space-between;
        }

        nav ul li a {
            color: #fff;
            text-decoration: none;
            padding: 10px;
        }

        .hero {
            height: 100vh;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            text-align: center;
            position: relative;
            overflow: hidden;
        }

        .hero video {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            object-fit: cover;
            z-index: -1;
        }

        .hero h1 {
            font-size: 48px;
            margin-bottom: 20px;
        }

        .hero p {
            font-size: 24px;
            margin-bottom: 40px;
        }

        .hero button {
            padding: 10px 20px;
            font-size: 18px;
            background-color: #333;
            color: #fff;
            border: none;
            cursor: pointer;
        }

        .featured-courses, .success-stories {
            padding: 40px;
        }

        .course-card {
            background-color: #f5f5f5;
            padding: 20px;
            margin-bottom: 20px;
        }

        .testimonial {
            background-color: #f5f5f5;
            padding: 20px;
            margin-bottom: 20px;
        }

        footer {
            background-color: #333;
            color: #fff;
            padding: 20px;
            text-align: center;
        }

        footer a {
            color: #fff;
            text-decoration: none;
            margin: 0 10px;
        }

        footer button {
            padding: 10px 20px;
            font-size: 18px;
            background-color: #fff;
            color: #333;
            border: none;
            cursor: pointer;
            margin-top: 20px;
        }
    </style>
</head>
<body>
    <header>
        <nav>
            <ul>
                <li><a href="#">Math</a></li>
                <li><a href="#">Science</a></li>
                <li><a href="#">Languages</a></li>
                <li><a href="#">Arts</a></li>
                <li><input type="text" placeholder="Search"></li>
            </ul>
        </nav>
    </header>

    <section class="hero">
        <video src="placeholder-video.mp4" autoplay loop muted></video>
        <h1>Welcome to EduQuest</h1>
        <p id="tagline">Learn at your own pace</p>
        <button>Get Started</button>
    </section>

    <section class="featured-courses">
        <h2>Featured Courses</h2>
        <div class="course-card">
            <img src="placeholder-course-image.jpg" alt="Course Image">
            <h3>Course Title</h3>
            <p>Instructor: John Doe</p>
            <p>Course description goes here.</p>
        </div>
        <div class="course-card">
            <img src="placeholder-course-image.jpg" alt="Course Image">
            <h3>Course Title</h3>
            <p>Instructor: Jane Smith</p>
            <p>Course description goes here.</p>
        </div>
    </section>

    <section class="success-stories">
        <h2>Success Stories</h2>
        <div class="testimonial">
            <p>"EduQuest helped me learn a new skill and advance my career. Highly recommended!"</p>
            <p>- John Doe</p>
        </div>
        <div class="testimonial">
            <p>"I never thought I could learn a new language online, but EduQuest made it possible. Thank you!"</p>
            <p>- Jane Smith</p>
        </div>
    </section>

    <footer>
        <a href="#">Blog</a>
        <a href="#">FAQ</a>
        <a href="#">Privacy Policy</a>
        <button>Contact Us</button>
    </footer>

    <script>
        // JavaScript Code
        const taglines = [
            "Learn at your own pace",
            "Discover new passions",
            "Expand your horizons"
        ];

        let index = 0;
        const taglineElement = document.getElementById("tagline");

        function rotateTagline() {
            taglineElement.textContent = taglines[index];
            index = (index + 1) % taglines.length;
        }

        setInterval(rotateTagline, 3000);
    </script>
</body>
</html>
```

> 이 코드는 다음 기능과 섹션을 가진 "EduQuest"를 위한 단일 페이지 웹사이트를 생성합니다:
> 
> 과정 카테고리(수학, 과학, 언어, 예술) 링크와 검색 바가 있는 고정 내비게이션 바.
> 
> 플레이스홀더 비디오 배경, "자신만의 속도로 학습하세요," "새로운 열정을 발견하세요," "시야를 넓히세요"를 3초마다 회전하는 동적 태그라인, 그리고 "시작하기" 버튼이 있는 히어로 섹션.
> 
> 과정 이미지, 제목, 강사, 설명을 위한 플레이스홀더가 있는 과정 카드를 표시하는 추천 과정 섹션.
> 
> 만족한 학생들의 추천사를 특징으로 하는 "성공 사례" 섹션으로, 추천사 텍스트와 학생 이름을 위한 플레이스홀더 포함.
> 
> 플랫폼의 블로그, FAQ, 개인정보 보호정책 링크와 "문의하기" 버튼이 있는 푸터.
> 
> CSS 스타일은 `<head>` 섹션의 `<style>` 태그 내에 임베드되고, JavaScript 코드는 `<body>` 섹션 끝의 `<script>` 태그 내에 배치됩니다.
> 
> JavaScript 코드는 setInterval 함수를 사용하여 3초마다 다른 태그라인들을 회전시키며 히어로 섹션의 동적 태그라인을 처리합니다.
> 
> 비디오 배경, 과정 이미지, 추천사는 플레이스홀더이며 실제 콘텐츠로 교체되어야 한다는 점을 참고하세요.

## API 요청

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic( # defaults to os.environ.get("ANTHROPIC_API_KEY")
api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=4000,
  temperature=0,
  system="당신의 임무는 주어진 사양에 따라 단일 페이지 웹사이트를 생성하는 것으로, JavaScript와 CSS가 임베드된 HTML 파일로 제공됩니다. 웹사이트는 드롭다운 메뉴, 동적 텍스트 및 콘텐츠, 클릭 가능한 버튼 등과 같은 다양한 매력적이고 상호작용적인 디자인 기능을 통합해야 합니다. 디자인이 시각적으로 매력적이고, 반응형이며, 사용자 친화적인지 확인하세요. HTML, CSS, JavaScript 코드는 잘 구조화되고, 효율적으로 조직되며, 가독성과 유지보수성을 위해 적절히 주석 처리되어야 합니다.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "\"EduQuest\"라는 온라인 학습 플랫폼을 위한 단일 페이지 웹사이트를 다음 기능과 섹션으로 생성하세요: \n \n1. 과정 카테고리(수학, 과학, 언어, 예술) 링크와 검색 바가 있는 고정 내비게이션 바. \n \n2. 온라인으로 학습하는 학생들을 보여주는 비디오 배경, \"자신만의 속도로 학습하세요,\" \"새로운 열정을 발견하세요,\" \"시야를 넓히세요\"를 3초마다 회전하는 동적 태그라인, 그리고 과정 카탈로그로 이어지는 \"시작하기\" 버튼이 있는 히어로 섹션. \n \n3. 과정 이미지, 제목, 강사, 설명을 위한 플레이스홀더가 있는 과정 카드를 표시하는 추천 과정 섹션. \n \n4. 학습 스타일과 관심사를 결정하는 짧은 퀴즈와 퀴즈를 시작하는 버튼이 있는 상호작용적인 \"학습 경로\" 섹션. \n \n5. 만족한 학생들의 추천사를 특징으로 하는 \"성공 사례\" 섹션으로, 추천사 텍스트와 학생 이름을 위한 플레이스홀더 포함. \n \n6. 플랫폼의 블로그, FAQ, 개인정보 보호정책 링크와 연락처 양식 및 고객 지원 정보가 있는 모달 창을 여는 \"문의하기\" 버튼이 있는 푸터. \n \n비디오 배경, 과정 카드, 추천사를 위한 필러 플레이스홀더 콘텐츠를 포함하세요. CSS 스타일을 `<head>` 섹션의 `<style>` 태그 내에 임베드하고 JavaScript 코드를 `<body>` 섹션 끝의 `<script>` 태그 내에 배치하세요. \n \nJavaScript 코드는 히어로 섹션의 동적 태그라인을 처리하여 3초마다 다른 태그라인들을 회전시켜야 합니다."
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
  max_tokens: 4000,
  temperature: 0,
  system: "당신의 임무는 주어진 사양에 따라 단일 페이지 웹사이트를 생성하는 것으로, JavaScript와 CSS가 임베드된 HTML 파일로 제공됩니다. 웹사이트는 드롭다운 메뉴, 동적 텍스트 및 콘텐츠, 클릭 가능한 버튼 등과 같은 다양한 매력적이고 상호작용적인 디자인 기능을 통합해야 합니다. 디자인이 시각적으로 매력적이고, 반응형이며, 사용자 친화적인지 확인하세요. HTML, CSS, JavaScript 코드는 잘 구조화되고, 효율적으로 조직되며, 가독성과 유지보수성을 위해 적절히 주석 처리되어야 합니다.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "\"EduQuest\"라는 온라인 학습 플랫폼을 위한 단일 페이지 웹사이트를 다음 기능과 섹션으로 생성하세요:  \n  \n1. 과정 카테고리(수학, 과학, 언어, 예술) 링크와 검색 바가 있는 고정 내비게이션 바.  \n  \n2. 온라인으로 학습하는 학생들을 보여주는 비디오 배경, \"자신만의 속도로 학습하세요,\" \"새로운 열정을 발견하세요,\" \"시야를 넓히세요\"를 3초마다 회전하는 동적 태그라인, 그리고 과정 카탈로그로 이어지는 \"시작하기\" 버튼이 있는 히어로 섹션.  \n  \n3. 과정 이미지, 제목, 강사, 설명을 위한 플레이스홀더가 있는 과정 카드를 표시하는 추천 과정 섹션.  \n  \n4. 학습 스타일과 관심사를 결정하는 짧은 퀴즈와 퀴즈를 시작하는 버튼이 있는 상호작용적인 \"학습 경로\" 섹션.  \n  \n5. 만족한 학생들의 추천사를 특징으로 하는 \"성공 사례\" 섹션으로, 추천사 텍스트와 학생 이름을 위한 플레이스홀더 포함.  \n  \n6. 플랫폼의 블로그, FAQ, 개인정보 보호정책 링크와 연락처 양식 및 고객 지원 정보가 있는 모달 창을 여는 \"문의하기\" 버튼이 있는 푸터.  \n  \n비디오 배경, 과정 카드, 추천사를 위한 필러 플레이스홀더 콘텐츠를 포함하세요. CSS 스타일을 `<head>` 섹션의 `<style>` 태그 내에 임베드하고 JavaScript 코드를 `<body>` 섹션 끝의 `<script>` 태그 내에 배치하세요.  \n  \nJavaScript 코드는 히어로 섹션의 동적 태그라인을 처리하여 3초마다 다른 태그라인들을 회전시켜야 합니다."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="AWS Bedrock Python">

```python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# for authentication options

client = AnthropicBedrock()

message = client.messages.create(
  model="anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens=4000,
  temperature=0,
  system="당신의 임무는 주어진 사양에 따라 단일 페이지 웹사이트를 생성하는 것으로, JavaScript와 CSS가 임베드된 HTML 파일로 제공됩니다. 웹사이트는 드롭다운 메뉴, 동적 텍스트 및 콘텐츠, 클릭 가능한 버튼 등과 같은 다양한 매력적이고 상호작용적인 디자인 기능을 통합해야 합니다. 디자인이 시각적으로 매력적이고, 반응형이며, 사용자 친화적인지 확인하세요. HTML, CSS, JavaScript 코드는 잘 구조화되고, 효율적으로 조직되며, 가독성과 유지보수성을 위해 적절히 주석 처리되어야 합니다.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "\"EduQuest\"라는 온라인  학습 플랫폼을 위한 단일 페이지 웹사이트를 다음 기능과 섹션으로 생성하세요: \n \n1. 과정 카테고리(수학, 과학, 언어, 예술) 링크와 검색 바가 있는 고정 내비게이션 바. \n \n2. 온라인으로 학습하는 학생들을 보여주는 비디오 배경, \"자신만의 속도로 학습하세요,\" \"새로운 열정을 발견하세요,\" \"시야를 넓히세요\"를 3초마다 회전하는 동적 태그라인, 그리고 과정 카탈로그로 이어지는 \"시작하기\" 버튼이 있는 히어로 섹션. \n \n3. 과정 이미지, 제목, 강사, 설명을 위한 플레이스홀더가 있는 과정 카드를 표시하는 추천 과정 섹션. \n \n4. 학습 스타일과 관심사를 결정하는 짧은 퀴즈와 퀴즈를 시작하는 버튼이 있는 상호작용적인 \"학습 경로\" 섹션. \n \n5. 만족한 학생들의 추천사를 특징으로 하는 \"성공 사례\" 섹션으로, 추천사 텍스트와 학생 이름을 위한 플레이스홀더 포함. \n \n6. 플랫폼의 블로그, FAQ, 개인정보 보호정책 링크와 연락처 양식 및 고객 지원 정보가 있는 모달 창을 여는 \"문의하기\" 버튼이 있는 푸터. \n \n비디오 배경, 과정 카드, 추천사를 위한 필러 플레이스홀더 콘텐츠를 포함하세요. CSS 스타일을 `<head>` 섹션의 `<style>` 태그 내에 임베드하고 JavaScript 코드를 `<body>` 섹션 끝의 `<script>` 태그 내에 배치하세요. \n \nJavaScript 코드는 히어로 섹션의 동적 태그라인을 처리하여 3초마다 다른 태그라인들을 회전시켜야 합니다."
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

import AnthropicBedrock from '@anthropic-ai/bedrock-sdk';

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 4000,
  temperature: 0,
  system: "당신의 임무는 주어진 사양에 따라 단일 페이지 웹사이트를 생성하는 것으로, JavaScript와 CSS가 임베드된 HTML 파일로 제공됩니다. 웹사이트는 드롭다운 메뉴, 동적 텍스트 및 콘텐츠, 클릭 가능한 버튼 등과 같은 다양한 매력적이고 상호작용적인 디자인 기능을 통합해야 합니다. 디자인이 시각적으로 매력적이고, 반응형이며, 사용자 친화적인지 확인하세요. HTML, CSS, JavaScript 코드는 잘 구조화되고, 효율적으로 조직되며, 가독성과 유지보수성을 위해 적절히 주석 처리되어야 합니다.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "\"EduQuest\"라는 온라인 학습 플랫폼을 위한 단일 페이지 웹사이트를 다음 기능과 섹션으로 생성하세요: \n \n1. 과정 카테고리(수학, 과학, 언어, 예술) 링크와 검색 바가 있는 고정 내비게이션 바. \n \n2. 온라인으로 학습하는 학생들을 보여주는 비디오 배경, \"자신만의 속도로 학습하세요,\" \"새로운 열정을 발견하세요,\" \"시야를 넓히세요\"를 3초마다 회전하는 동적 태그라인, 그리고 과정 카탈로그로 이어지는 \"시작하기\" 버튼이 있는 히어로 섹션. \n \n3. 과정 이미지, 제목, 강사, 설명을 위한 플레이스홀더가 있는 과정 카드를 표시하는 추천 과정 섹션. \n \n4. 학습 스타일과 관심사를 결정하는 짧은 퀴즈와 퀴즈를 시작하는 버튼이 있는 상호작용적인 \"학습 경로\" 섹션. \n \n5. 만족한 학생들의 추천사를 특징으로 하는 \"성공 사례\" 섹션으로, 추천사 텍스트와 학생 이름을 위한 플레이스홀더 포함. \n \n6. 플랫폼의 블로그, FAQ, 개인정보 보호정책 링크와 연락처 양식 및 고객 지원 정보가 있는 모달 창을 여는 \"문의하기\" 버튼이 있는 푸터. \n \n비디오 배경, 과정 카드, 추천사를 위한 필러 플레이스홀더 콘텐츠를 포함하세요. CSS 스타일을 `<head>` 섹션의 `<style>` 태그 내에 임베드하고 JavaScript 코드를 `<body>` 섹션 끝의 `<script>` 태그 내에 배치하세요. \n \nJavaScript 코드는 히어로 섹션의 동적 태그라인을 처리하여 3초마다 다른 태그라인들을 회전시켜야 합니다."
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
  max_tokens=4000,
  temperature=0,
  system="당신의 임무는 주어진 사양에 따라 단일 페이지 웹사이트를 생성하는 것으로, JavaScript와 CSS가 임베드된 HTML 파일로 제공됩니다. 웹사이트는 드롭다운 메뉴, 동적 텍스트 및 콘텐츠, 클릭 가능한 버튼 등과 같은 다양한 매력적이고 상호작용적인 디자인 기능을 통합해야 합니다. 디자인이 시각적으로 매력적이고, 반응형이며, 사용자 친화적인지 확인하세요. HTML, CSS, JavaScript 코드는 잘 구조화되고, 효율적으로 조직되며, 가독성과 유지보수성을 위해 적절히 주석 처리되어야 합니다.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "\"EduQuest\"라는 온라인 학습 플랫폼을 위한 단일 페이지 웹사이트를 다음 기능과 섹션으로 생성하세요: \n \n1. 과정 카테고리(수학, 과학, 언어, 예술) 링크와 검색 바가 있는 고정 내비게이션 바. \n \n2. 온라인으로 학습하는 학생들을 보여주는 비디오 배경, \"자신만의 속도로 학습하세요,\" \"새로운 열정을 발견하세요,\" \"시야를 넓히세요\"를 3초마다 회전하는 동적 태그라인, 그리고 과정 카탈로그로 이어지는 \"시작하기\" 버튼이 있는 히어로 섹션. \n \n3. 과정 이미지, 제목, 강사, 설명을 위한 플레이스홀더가 있는 과정 카드를 표시하는 추천 과정 섹션. \n \n4. 학습 스타일과 관심사를 결정하는 짧은 퀴즈와 퀴즈를 시작하는 버튼이 있는 상호작용적인 \"학습 경로\" 섹션. \n \n5. 만족한 학생들의 추천사를 특징으로 하는 \"성공 사례\" 섹션으로, 추천사 텍스트와 학생 이름을 위한 플레이스홀더 포함. \n \n6. 플랫폼의 블로그, FAQ, 개인정보 보호정책 링크와 연락처 양식 및 고객 지원 정보가 있는 모달 창을 여는 \"문의하기\" 버튼이 있는 푸터. \n \n비디오 배경, 과정 카드, 추천사를 위한 필러 플레이스홀더 콘텐츠를 포함하세요. CSS 스타일을 `<head>` 섹션의 `<style>` 태그 내에 임베드하고 JavaScript 코드를 `<body>` 섹션 끝의 `<script>` 태그 내에 배치하세요. \n \nJavaScript 코드는 히어로 섹션의 동적 태그라인을 처리하여 3초마다 다른 태그라인들을 회전시켜야 합니다."
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
max_tokens: 4000,
temperature: 0,
system: "당신의 임무는 주어진 사양에 따라 단일 페이지 웹사이트를 생성하는 것으로, JavaScript와 CSS가 임베드된 HTML 파일로 제공됩니다. 웹사이트는 드롭다운 메뉴, 동적 텍스트 및 콘텐츠, 클릭 가능한 버튼 등과 같은 다양한 매력적이고 상호작용적인 디자인 기능을 통합해야 합니다. 디자인이 시각적으로 매력적이고, 반응형이며, 사용자 친화적인지 확인하세요. HTML, CSS, JavaScript 코드는 잘 구조화되고, 효율적으로 조직되며, 가독성과 유지보수성을 위해 적절히 주석 처리되어야 합니다.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "\"EduQuest\"라는 온라인 학습 플랫폼을 위한 단일 페이지 웹사이트를 다음 기능과 섹션으로 생성하세요: \n \n1. 과정 카테고리(수학, 과학, 언어, 예술) 링크와 검색 바가 있는 고정 내비게이션 바. \n \n2. 온라인으로 학습하는 학생들을 보여주는 비디오 배경, \"자신만의 속도로 학습하세요,\" \"새로운 열정을 발견하세요,\" \"시야를 넓히세요\"를 3초마다 회전하는 동적 태그라인, 그리고 과정 카탈로그로 이어지는 \"시작하기\" 버튼이 있는 히어로 섹션. \n \n3. 과정 이미지, 제목, 강사, 설명을 위한 플레이스홀더가 있는 과정 카드를 표시하는 추천 과정 섹션. \n \n4. 학습 스타일과 관심사를 결정하는 짧은 퀴즈와 퀴즈를 시작하는 버튼이 있는 상호작용적인 \"학습 경로\" 섹션. \n \n5. 만족한 학생들의 추천사를 특징으로 하는 \"성공 사례\" 섹션으로, 추천사 텍스트와 학생 이름을 위한 플레이스홀더 포함. \n \n6. 플랫폼의 블로그, FAQ, 개인정보 보호정책 링크와 연락처 양식 및 고객 지원 정보가 있는 모달 창을 여는 \"문의하기\" 버튼이 있는 푸터. \n \n비디오 배경, 과정 카드, 추천사를 위한 필러 플레이스홀더 콘텐츠를 포함하세요. CSS 스타일을 `<head>` 섹션의 `<style>` 태그 내에 임베드하고 JavaScript 코드를 `<body>` 섹션 끝의 `<script>` 태그 내에 배치하세요. \n \nJavaScript 코드는 히어로 섹션의 동적 태그라인을 처리하여 3초마다 다른 태그라인들을 회전시켜야 합니다."
}
]
}
]
});
console.log(msg);

```

</Tab>
</Tabs>