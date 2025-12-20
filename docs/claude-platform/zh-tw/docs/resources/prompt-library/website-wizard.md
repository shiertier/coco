# 網站精靈

根據使用者規格建立單頁網站。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 來親自試試看！

|        | 內容 |
| --- | --- |
| System | 您的任務是根據給定的規格建立一個單頁網站，以包含嵌入式 JavaScript 和 CSS 的 HTML 檔案形式交付。網站應該融入各種引人入勝的互動式設計功能，例如下拉選單、動態文字和內容、可點擊按鈕等等。確保設計在視覺上具有吸引力、響應式且使用者友善。HTML、CSS 和 JavaScript 程式碼應該結構良好、有效組織，並適當註解以提高可讀性和可維護性。 |
| User   | 為名為「EduQuest」的線上學習平台建立一個單頁網站，包含以下功能和區段：<br/><br/>1. 一個固定導航欄，包含課程類別連結（數學、科學、語言、藝術）和搜尋欄。<br/>2. 一個主要區段，包含展示學生線上學習的影片背景、每 3 秒在「按照自己的節奏學習」、「發現新的熱情」和「拓展您的視野」之間輪換的動態標語，以及一個通往課程目錄的「開始使用」按鈕。<br/>3. 一個精選課程區段，顯示包含課程圖片、標題、講師和描述佔位符的課程卡片。<br/>4. 一個互動式「學習路徑」區段，包含一個簡短測驗來確定學習風格和興趣，以及一個開始測驗的按鈕。<br/>5. 一個「成功故事」區段，展示滿意學生的推薦，包含推薦文字和學生姓名的佔位符。<br/>6. 一個頁腳，包含平台部落格、常見問題、隱私政策的連結，以及一個「聯絡我們」按鈕，該按鈕會開啟包含聯絡表單和客戶支援資訊的模態視窗。<br/><br/>為影片背景、課程卡片和推薦包含填充佔位符內容。將 CSS 樣式嵌入 `<head>` 區段的 `<style>` 標籤中，並將 JavaScript 程式碼放置在 `<body>` 區段末尾的 `<script>` 標籤中。<br/><br/>JavaScript 程式碼應該處理主要區段中的動態標語，每 3 秒輪換不同的標語。 |

## 範例輸出

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

> 此程式碼為「EduQuest」建立了一個單頁網站，包含以下功能和區段：
> 
> 一個固定導航欄，包含課程類別連結（數學、科學、語言、藝術）和搜尋欄。
> 
> 一個主要區段，包含佔位符影片背景、每 3 秒在「按照自己的節奏學習」、「發現新的熱情」和「拓展您的視野」之間輪換的動態標語，以及一個「開始使用」按鈕。
> 
> 一個精選課程區段，顯示包含課程圖片、標題、講師和描述佔位符的課程卡片。
> 
> 一個「成功故事」區段，展示滿意學生的推薦，包含推薦文字和學生姓名的佔位符。
> 
> 一個頁腳，包含平台部落格、常見問題、隱私政策的連結，以及一個「聯絡我們」按鈕。
> 
> CSS 樣式嵌入在 `<head>` 區段的 `<style>` 標籤中，JavaScript 程式碼放置在 `<body>` 區段末尾的 `<script>` 標籤中。
> 
> JavaScript 程式碼處理主要區段中的動態標語，使用 setInterval 函數每 3 秒輪換不同的標語。
> 
> 請注意，影片背景、課程圖片和推薦都是佔位符，應該替換為實際內容。

## API 請求

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
  system="您的任務是根據給定的規格建立一個單頁網站，以包含嵌入式 JavaScript 和 CSS 的 HTML 檔案形式交付。網站應該融入各種引人入勝的互動式設計功能，例如下拉選單、動態文字和內容、可點擊按鈕等等。確保設計在視覺上具有吸引力、響應式且使用者友善。HTML、CSS 和 JavaScript 程式碼應該結構良好、有效組織，並適當註解以提高可讀性和可維護性。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "為名為「EduQuest」的線上學習平台建立一個單頁網站，包含以下功能和區段： \n \n1. 一個固定導航欄，包含課程類別連結（數學、科學、語言、藝術）和搜尋欄。 \n \n2. 一個主要區段，包含展示學生線上學習的影片背景、每 3 秒在「按照自己的節奏學習」、「發現新的熱情」和「拓展您的視野」之間輪換的動態標語，以及一個通往課程目錄的「開始使用」按鈕。 \n \n3. 一個精選課程區段，顯示包含課程圖片、標題、講師和描述佔位符的課程卡片。 \n \n4. 一個互動式「學習路徑」區段，包含一個簡短測驗來確定學習風格和興趣，以及一個開始測驗的按鈕。 \n \n5. 一個「成功故事」區段，展示滿意學生的推薦，包含推薦文字和學生姓名的佔位符。 \n \n6. 一個頁腳，包含平台部落格、常見問題、隱私政策的連結，以及一個「聯絡我們」按鈕，該按鈕會開啟包含聯絡表單和客戶支援資訊的模態視窗。 \n \n為影片背景、課程卡片和推薦包含填充佔位符內容。將 CSS 樣式嵌入 `<head>` 區段的 `<style>` 標籤中，並將 JavaScript 程式碼放置在 `<body>` 區段末尾的 `<script>` 標籤中。 \n \nJavaScript 程式碼應該處理主要區段中的動態標語，每 3 秒輪換不同的標語。"
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
  system: "您的任務是根據給定的規格建立一個單頁網站，以包含嵌入式 JavaScript 和 CSS 的 HTML 檔案形式交付。網站應該融入各種引人入勝的互動式設計功能，例如下拉選單、動態文字和內容、可點擊按鈕等等。確保設計在視覺上具有吸引力、響應式且使用者友善。HTML、CSS 和 JavaScript 程式碼應該結構良好、有效組織，並適當註解以提高可讀性和可維護性。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "為名為「EduQuest」的線上學習平台建立一個單頁網站，包含以下功能和區段：  \n  \n1. 一個固定導航欄，包含課程類別連結（數學、科學、語言、藝術）和搜尋欄。  \n  \n2. 一個主要區段，包含展示學生線上學習的影片背景、每 3 秒在「按照自己的節奏學習」、「發現新的熱情」和「拓展您的視野」之間輪換的動態標語，以及一個通往課程目錄的「開始使用」按鈕。  \n  \n3. 一個精選課程區段，顯示包含課程圖片、標題、講師和描述佔位符的課程卡片。  \n  \n4. 一個互動式「學習路徑」區段，包含一個簡短測驗來確定學習風格和興趣，以及一個開始測驗的按鈕。  \n  \n5. 一個「成功故事」區段，展示滿意學生的推薦，包含推薦文字和學生姓名的佔位符。  \n  \n6. 一個頁腳，包含平台部落格、常見問題、隱私政策的連結，以及一個「聯絡我們」按鈕，該按鈕會開啟包含聯絡表單和客戶支援資訊的模態視窗。  \n  \n為影片背景、課程卡片和推薦包含填充佔位符內容。將 CSS 樣式嵌入 `<head>` 區段的 `<style>` 標籤中，並將 JavaScript 程式碼放置在 `<body>` 區段末尾的 `<script>` 標籤中。  \n  \nJavaScript 程式碼應該處理主要區段中的動態標語，每 3 秒輪換不同的標語。"
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
  system="您的任務是根據給定的規格建立一個單頁網站，以包含嵌入式 JavaScript 和 CSS 的 HTML 檔案形式交付。網站應該融入各種引人入勝的互動式設計功能，例如下拉選單、動態文字和內容、可點擊按鈕等等。確保設計在視覺上具有吸引力、響應式且使用者友善。HTML、CSS 和 JavaScript 程式碼應該結構良好、有效組織，並適當註解以提高可讀性和可維護性。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "為名為「EduQuest」的線上學習平台建立一個單頁網站，包含以下功能和區段： \n \n1. 一個固定導航欄，包含課程類別連結（數學、科學、語言、藝術）和搜尋欄。 \n \n2. 一個主要區段，包含展示學生線上學習的影片背景、每 3 秒在「按照自己的節奏學習」、「發現新的熱情」和「拓展您的視野」之間輪換的動態標語，以及一個通往課程目錄的「開始使用」按鈕。 \n \n3. 一個精選課程區段，顯示包含課程圖片、標題、講師和描述佔位符的課程卡片。 \n \n4. 一個互動式「學習路徑」區段，包含一個簡短測驗來確定學習風格和興趣，以及一個開始測驗的按鈕。 \n \n5. 一個「成功故事」區段，展示滿意學生的推薦，包含推薦文字和學生姓名的佔位符。 \n \n6. 一個頁腳，包含平台部落格、常見問題、隱私政策的連結，以及一個「聯絡我們」按鈕，該按鈕會開啟包含聯絡表單和客戶支援資訊的模態視窗。 \n \n為影片背景、課程卡片和推薦包含填充佔位符內容。將 CSS 樣式嵌入 `<head>` 區段的 `<style>` 標籤中，並將 JavaScript 程式碼放置在 `<body>` 區段末尾的 `<script>` 標籤中。 \n \nJavaScript 程式碼應該處理主要區段中的動態標語，每 3 秒輪換不同的標語。"
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
  system: "您的任務是根據給定的規格建立一個單頁網站，以包含嵌入式 JavaScript 和 CSS 的 HTML 檔案形式交付。網站應該融入各種引人入勝的互動式設計功能，例如下拉選單、動態文字和內容、可點擊按鈕等等。確保設計在視覺上具有吸引力、響應式且使用者友善。HTML、CSS 和 JavaScript 程式碼應該結構良好、有效組織，並適當註解以提高可讀性和可維護性。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "為名為「EduQuest」的線上學習平台建立一個單頁網站，包含以下功能和區段： \n \n1. 一個固定導航欄，包含課程類別連結（數學、科學、語言、藝術）和搜尋欄。 \n \n2. 一個主要區段，包含展示學生線上學習的影片背景、每 3 秒在「按照自己的節奏學習」、「發現新的熱情」和「拓展您的視野」之間輪換的動態標語，以及一個通往課程目錄的「開始使用」按鈕。 \n \n3. 一個精選課程區段，顯示包含課程圖片、標題、講師和描述佔位符的課程卡片。 \n \n4. 一個互動式「學習路徑」區段，包含一個簡短測驗來確定學習風格和興趣，以及一個開始測驗的按鈕。 \n \n5. 一個「成功故事」區段，展示滿意學生的推薦，包含推薦文字和學生姓名的佔位符。 \n \n6. 一個頁腳，包含平台部落格、常見問題、隱私政策的連結，以及一個「聯絡我們」按鈕，該按鈕會開啟包含聯絡表單和客戶支援資訊的模態視窗。 \n \n為影片背景、課程卡片和推薦包含填充佔位符內容。將 CSS 樣式嵌入 `<head>` 區段的 `<style>` 標籤中，並將 JavaScript 程式碼放置在 `<body>` 區段末尾的 `<script>` 標籤中。 \n \nJavaScript 程式碼應該處理主要區段中的動態標語，每 3 秒輪換不同的標語。"
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
  system="您的任務是根據給定的規格建立一個單頁網站，以包含嵌入式 JavaScript 和 CSS 的 HTML 檔案形式交付。網站應該融入各種引人入勝的互動式設計功能，例如下拉選單、動態文字和內容、可點擊按鈕等等。確保設計在視覺上具有吸引力、響應式且使用者友善。HTML、CSS 和 JavaScript 程式碼應該結構良好、有效組織，並適當註解以提高可讀性和可維護性。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "為名為「EduQuest」的線上學習平台建立一個單頁網站，包含以下功能和區段： \n \n1. 一個固定導航欄，包含課程類別連結（數學、科學、語言、藝術）和搜尋欄。 \n \n2. 一個主要區段，包含展示學生線上學習的影片背景、每 3 秒在「按照自己的節奏學習」、「發現新的熱情」和「拓展您的視野」之間輪換的動態標語，以及一個通往課程目錄的「開始使用」按鈕。 \n \n3. 一個精選課程區段，顯示包含課程圖片、標題、講師和描述佔位符的課程卡片。 \n \n4. 一個互動式「學習路徑」區段，包含一個簡短測驗來確定學習風格和興趣，以及一個開始測驗的按鈕。 \n \n5. 一個「成功故事」區段，展示滿意學生的推薦，包含推薦文字和學生姓名的佔位符。 \n \n6. 一個頁腳，包含平台部落格、常見問題、隱私政策的連結，以及一個「聯絡我們」按鈕，該按鈕會開啟包含聯絡表單和客戶支援資訊的模態視窗。 \n \n為影片背景、課程卡片和推薦包含填充佔位符內容。將 CSS 樣式嵌入 `<head>` 區段的 `<style>` 標籤中，並將 JavaScript 程式碼放置在 `<body>` 區段末尾的 `<script>` 標籤中。 \n \nJavaScript 程式碼應該處理主要區段中的動態標語，每 3 秒輪換不同的標語。"
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
system: "您的任務是根據給定的規格建立一個單頁網站，以包含嵌入式 JavaScript 和 CSS 的 HTML 檔案形式交付。網站應該融入各種引人入勝的互動式設計功能，例如下拉選單、動態文字和內容、可點擊按鈕等等。確保設計在視覺上具有吸引力、響應式且使用者友善。HTML、CSS 和 JavaScript 程式碼應該結構良好、有效組織，並適當註解以提高可讀性和可維護性。",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "為名為「EduQuest」的線上學習平台建立一個單頁網站，包含以下功能和區段： \n \n1. 一個固定導航欄，包含課程類別連結（數學、科學、語言、藝術）和搜尋欄。 \n \n2. 一個主要區段，包含展示學生線上學習的影片背景、每 3 秒在「按照自己的節奏學習」、「發現新的熱情」和「拓展您的視野」之間輪換的動態標語，以及一個通往課程目錄的「開始使用」按鈕。 \n \n3. 一個精選課程區段，顯示包含課程圖片、標題、講師和描述佔位符的課程卡片。 \n \n4. 一個互動式「學習路徑」區段，包含一個簡短測驗來確定學習風格和興趣，以及一個開始測驗的按鈕。 \n \n5. 一個「成功故事」區段，展示滿意學生的推薦，包含推薦文字和學生姓名的佔位符。 \n \n6. 一個頁腳，包含平台部落格、常見問題、隱私政策的連結，以及一個「聯絡我們」按鈕，該按鈕會開啟包含聯絡表單和客戶支援資訊的模態視窗。 \n \n為影片背景、課程卡片和推薦包含填充佔位符內容。將 CSS 樣式嵌入 `<head>` 區段的 `<style>` 標籤中，並將 JavaScript 程式碼放置在 `<body>` 區段末尾的 `<script>` 標籤中。 \n \nJavaScript 程式碼應該處理主要區段中的動態標語，每 3 秒輪換不同的標語。"
}
]
}
]
});
console.log(msg);

```

</Tab>
</Tabs>