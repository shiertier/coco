# ウェブサイトウィザード

ユーザーの仕様に基づいてワンページウェブサイトを作成します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | コンテンツ |
| --- | --- |
| System | あなたのタスクは、指定された仕様に基づいてワンページウェブサイトを作成し、埋め込みJavaScriptとCSSを含むHTMLファイルとして提供することです。ウェブサイトには、ドロップダウンメニュー、動的テキストとコンテンツ、クリック可能なボタンなど、様々な魅力的でインタラクティブなデザイン機能を組み込む必要があります。デザインが視覚的に魅力的で、レスポンシブで、ユーザーフレンドリーであることを確認してください。HTML、CSS、JavaScriptコードは、読みやすさと保守性のために、よく構造化され、効率的に整理され、適切にコメントされている必要があります。 |
| User   | 以下の機能とセクションを持つ「EduQuest」というオンライン学習プラットフォームのワンページウェブサイトを作成してください：<br/><br/>1. コースカテゴリ（数学、科学、言語、芸術）へのリンクと検索バーを含む固定ナビゲーションバー。<br/>2. オンラインで学習する学生を紹介する動画背景、「自分のペースで学習」「新しい情熱を発見」「視野を広げる」の間で3秒ごとに回転する動的なタグライン、コースカタログにつながる「始める」ボタンを含むヒーローセクション。<br/>3. コース画像、タイトル、講師、説明のプレースホルダーを含むコースカードを表示する注目コースセクション。<br/>4. 学習スタイルと興味を判定する短いクイズと、クイズを開始するボタンを含むインタラクティブな「学習パス」セクション。<br/>5. 満足した学生からの証言を特集する「成功事例」セクション。証言テキストと学生名のプレースホルダーを含む。<br/>6. プラットフォームのブログ、FAQ、プライバシーポリシーへのリンクと、連絡フォームとカスタマーサポート情報を含むモーダルウィンドウを開く「お問い合わせ」ボタンを含むフッター。<br/><br/>動画背景、コースカード、証言にはフィラープレースホルダーコンテンツを含めてください。CSSスタイルは`<head>`セクション内の`<style>`タグに埋め込み、JavaScriptコードは`<body>`セクションの最後の`<script>`タグ内に配置してください。<br/><br/>JavaScriptコードは、ヒーローセクションの動的なタグラインを処理し、3秒ごとに異なるタグラインを回転させる必要があります。 |

## 出力例

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

> このコードは、以下の機能とセクションを持つ「EduQuest」のワンページウェブサイトを作成します：
> 
> コースカテゴリ（数学、科学、言語、芸術）へのリンクと検索バーを含む固定ナビゲーションバー。
> 
> プレースホルダー動画背景、「自分のペースで学習」「新しい情熱を発見」「視野を広げる」の間で3秒ごとに回転する動的なタグライン、「始める」ボタンを含むヒーローセクション。
> 
> コース画像、タイトル、講師、説明のプレースホルダーを含むコースカードを表示する注目コースセクション。
> 
> 満足した学生からの証言を特集する「成功事例」セクション。証言テキストと学生名のプレースホルダーを含む。
> 
> プラットフォームのブログ、FAQ、プライバシーポリシーへのリンクと「お問い合わせ」ボタンを含むフッター。
> 
> CSSスタイルは`<head>`セクション内の`<style>`タグに埋め込まれ、JavaScriptコードは`<body>`セクションの最後の`<script>`タグ内に配置されています。
> 
> JavaScriptコードは、setInterval関数を使用して3秒ごとに異なるタグラインを回転させ、ヒーローセクションの動的なタグラインを処理します。
> 
> 動画背景、コース画像、証言はプレースホルダーであり、実際のコンテンツに置き換える必要があることにご注意ください。

## APIリクエスト

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
  system="あなたのタスクは、指定された仕様に基づいてワンページウェブサイトを作成し、埋め込みJavaScriptとCSSを含むHTMLファイルとして提供することです。ウェブサイトには、ドロップダウンメニュー、動的テキストとコンテンツ、クリック可能なボタンなど、様々な魅力的でインタラクティブなデザイン機能を組み込む必要があります。デザインが視覚的に魅力的で、レスポンシブで、ユーザーフレンドリーであることを確認してください。HTML、CSS、JavaScriptコードは、読みやすさと保守性のために、よく構造化され、効率的に整理され、適切にコメントされている必要があります。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "以下の機能とセクションを持つ「EduQuest」というオンライン学習プラットフォームのワンページウェブサイトを作成してください： \n \n1. コースカテゴリ（数学、科学、言語、芸術）へのリンクと検索バーを含む固定ナビゲーションバー。 \n \n2. オンラインで学習する学生を紹介する動画背景、「自分のペースで学習」「新しい情熱を発見」「視野を広げる」の間で3秒ごとに回転する動的なタグライン、コースカタログにつながる「始める」ボタンを含むヒーローセクション。 \n \n3. コース画像、タイトル、講師、説明のプレースホルダーを含むコースカードを表示する注目コースセクション。 \n \n4. 学習スタイルと興味を判定する短いクイズと、クイズを開始するボタンを含むインタラクティブな「学習パス」セクション。 \n \n5. 満足した学生からの証言を特集する「成功事例」セクション。証言テキストと学生名のプレースホルダーを含む。 \n \n6. プラットフォームのブログ、FAQ、プライバシーポリシーへのリンクと、連絡フォームとカスタマーサポート情報を含むモーダルウィンドウを開く「お問い合わせ」ボタンを含むフッター。 \n \n動画背景、コースカード、証言にはフィラープレースホルダーコンテンツを含めてください。CSSスタイルは`<head>`セクション内の`<style>`タグに埋め込み、JavaScriptコードは`<body>`セクションの最後の`<script>`タグ内に配置してください。 \n \nJavaScriptコードは、ヒーローセクションの動的なタグラインを処理し、3秒ごとに異なるタグラインを回転させる必要があります。"
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
  system: "あなたのタスクは、指定された仕様に基づいてワンページウェブサイトを作成し、埋め込みJavaScriptとCSSを含むHTMLファイルとして提供することです。ウェブサイトには、ドロップダウンメニュー、動的テキストとコンテンツ、クリック可能なボタンなど、様々な魅力的でインタラクティブなデザイン機能を組み込む必要があります。デザインが視覚的に魅力的で、レスポンシブで、ユーザーフレンドリーであることを確認してください。HTML、CSS、JavaScriptコードは、読みやすさと保守性のために、よく構造化され、効率的に整理され、適切にコメントされている必要があります。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "以下の機能とセクションを持つ「EduQuest」というオンライン学習プラットフォームのワンページウェブサイトを作成してください：  \n  \n1. コースカテゴリ（数学、科学、言語、芸術）へのリンクと検索バーを含む固定ナビゲーションバー。  \n  \n2. オンラインで学習する学生を紹介する動画背景、「自分のペースで学習」「新しい情熱を発見」「視野を広げる」の間で3秒ごとに回転する動的なタグライン、コースカタログにつながる「始める」ボタンを含むヒーローセクション。  \n  \n3. コース画像、タイトル、講師、説明のプレースホルダーを含むコースカードを表示する注目コースセクション。  \n  \n4. 学習スタイルと興味を判定する短いクイズと、クイズを開始するボタンを含むインタラクティブな「学習パス」セクション。  \n  \n5. 満足した学生からの証言を特集する「成功事例」セクション。証言テキストと学生名のプレースホルダーを含む。  \n  \n6. プラットフォームのブログ、FAQ、プライバシーポリシーへのリンクと、連絡フォームとカスタマーサポート情報を含むモーダルウィンドウを開く「お問い合わせ」ボタンを含むフッター。  \n  \n動画背景、コースカード、証言にはフィラープレースホルダーコンテンツを含めてください。CSSスタイルは`<head>`セクション内の`<style>`タグに埋め込み、JavaScriptコードは`<body>`セクションの最後の`<script>`タグ内に配置してください。  \n  \nJavaScriptコードは、ヒーローセクションの動的なタグラインを処理し、3秒ごとに異なるタグラインを回転させる必要があります。"
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
  system="あなたのタスクは、指定された仕様に基づいてワンページウェブサイトを作成し、埋め込みJavaScriptとCSSを含むHTMLファイルとして提供することです。ウェブサイトには、ドロップダウンメニュー、動的テキストとコンテンツ、クリック可能なボタンなど、様々な魅力的でインタラクティブなデザイン機能を組み込む必要があります。デザインが視覚的に魅力的で、レスポンシブで、ユーザーフレンドリーであることを確認してください。HTML、CSS、JavaScriptコードは、読みやすさと保守性のために、よく構造化され、効率的に整理され、適切にコメントされている必要があります。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "以下の機能とセクションを持つ「EduQuest」というオンライン学習プラットフォームのワンページウェブサイトを作成してください： \n \n1. コースカテゴリ（数学、科学、言語、芸術）へのリンクと検索バーを含む固定ナビゲーションバー。 \n \n2. オンラインで学習する学生を紹介する動画背景、「自分のペースで学習」「新しい情熱を発見」「視野を広げる」の間で3秒ごとに回転する動的なタグライン、コースカタログにつながる「始める」ボタンを含むヒーローセクション。 \n \n3. コース画像、タイトル、講師、説明のプレースホルダーを含むコースカードを表示する注目コースセクション。 \n \n4. 学習スタイルと興味を判定する短いクイズと、クイズを開始するボタンを含むインタラクティブな「学習パス」セクション。 \n \n5. 満足した学生からの証言を特集する「成功事例」セクション。証言テキストと学生名のプレースホルダーを含む。 \n \n6. プラットフォームのブログ、FAQ、プライバシーポリシーへのリンクと、連絡フォームとカスタマーサポート情報を含むモーダルウィンドウを開く「お問い合わせ」ボタンを含むフッター。 \n \n動画背景、コースカード、証言にはフィラープレースホルダーコンテンツを含めてください。CSSスタイルは`<head>`セクション内の`<style>`タグに埋め込み、JavaScriptコードは`<body>`セクションの最後の`<script>`タグ内に配置してください。 \n \nJavaScriptコードは、ヒーローセクションの動的なタグラインを処理し、3秒ごとに異なるタグラインを回転させる必要があります。"
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
  system: "あなたのタスクは、指定された仕様に基づいてワンページウェブサイトを作成し、埋め込みJavaScriptとCSSを含むHTMLファイルとして提供することです。ウェブサイトには、ドロップダウンメニュー、動的テキストとコンテンツ、クリック可能なボタンなど、様々な魅力的でインタラクティブなデザイン機能を組み込む必要があります。デザインが視覚的に魅力的で、レスポンシブで、ユーザーフレンドリーであることを確認してください。HTML、CSS、JavaScriptコードは、読みやすさと保守性のために、よく構造化され、効率的に整理され、適切にコメントされている必要があります。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "以下の機能とセクションを持つ「EduQuest」というオンライン学習プラットフォームのワンページウェブサイトを作成してください： \n \n1. コースカテゴリ（数学、科学、言語、芸術）へのリンクと検索バーを含む固定ナビゲーションバー。 \n \n2. オンラインで学習する学生を紹介する動画背景、「自分のペースで学習」「新しい情熱を発見」「視野を広げる」の間で3秒ごとに回転する動的なタグライン、コースカタログにつながる「始める」ボタンを含むヒーローセクション。 \n \n3. コース画像、タイトル、講師、説明のプレースホルダーを含むコースカードを表示する注目コースセクション。 \n \n4. 学習スタイルと興味を判定する短いクイズと、クイズを開始するボタンを含むインタラクティブな「学習パス」セクション。 \n \n5. 満足した学生からの証言を特集する「成功事例」セクション。証言テキストと学生名のプレースホルダーを含む。 \n \n6. プラットフォームのブログ、FAQ、プライバシーポリシーへのリンクと、連絡フォームとカスタマーサポート情報を含むモーダルウィンドウを開く「お問い合わせ」ボタンを含むフッター。 \n \n動画背景、コースカード、証言にはフィラープレースホルダーコンテンツを含めてください。CSSスタイルは`<head>`セクション内の`<style>`タグに埋め込み、JavaScriptコードは`<body>`セクションの最後の`<script>`タグ内に配置してください。 \n \nJavaScriptコードは、ヒーローセクションの動的なタグラインを処理し、3秒ごとに異なるタグラインを回転させる必要があります。"
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
  system="あなたのタスクは、指定された仕様に基づいてワンページウェブサイトを作成し、埋め込みJavaScriptとCSSを含むHTMLファイルとして提供することです。ウェブサイトには、ドロップダウンメニュー、動的テキストとコンテンツ、クリック可能なボタンなど、様々な魅力的でインタラクティブなデザイン機能を組み込む必要があります。デザインが視覚的に魅力的で、レスポンシブで、ユーザーフレンドリーであることを確認してください。HTML、CSS、JavaScriptコードは、読みやすさと保守性のために、よく構造化され、効率的に整理され、適切にコメントされている必要があります。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "以下の機能とセクションを持つ「EduQuest」というオンライン学習プラットフォームのワンページウェブサイトを作成してください： \n \n1. コースカテゴリ（数学、科学、言語、芸術）へのリンクと検索バーを含む固定ナビゲーションバー。 \n \n2. オンラインで学習する学生を紹介する動画背景、「自分のペースで学習」「新しい情熱を発見」「視野を広げる」の間で3秒ごとに回転する動的なタグライン、コースカタログにつながる「始める」ボタンを含むヒーローセクション。 \n \n3. コース画像、タイトル、講師、説明のプレースホルダーを含むコースカードを表示する注目コースセクション。 \n \n4. 学習スタイルと興味を判定する短いクイズと、クイズを開始するボタンを含むインタラクティブな「学習パス」セクション。 \n \n5. 満足した学生からの証言を特集する「成功事例」セクション。証言テキストと学生名のプレースホルダーを含む。 \n \n6. プラットフォームのブログ、FAQ、プライバシーポリシーへのリンクと、連絡フォームとカスタマーサポート情報を含むモーダルウィンドウを開く「お問い合わせ」ボタンを含むフッター。 \n \n動画背景、コースカード、証言にはフィラープレースホルダーコンテンツを含めてください。CSSスタイルは`<head>`セクション内の`<style>`タグに埋め込み、JavaScriptコードは`<body>`セクションの最後の`<script>`タグ内に配置してください。 \n \nJavaScriptコードは、ヒーローセクションの動的なタグラインを処理し、3秒ごとに異なるタグラインを回転させる必要があります。"
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
system: "あなたのタスクは、指定された仕様に基づいてワンページウェブサイトを作成し、埋め込みJavaScriptとCSSを含むHTMLファイルとして提供することです。ウェブサイトには、ドロップダウンメニュー、動的テキストとコンテンツ、クリック可能なボタンなど、様々な魅力的でインタラクティブなデザイン機能を組み込む必要があります。デザインが視覚的に魅力的で、レスポンシブで、ユーザーフレンドリーであることを確認してください。HTML、CSS、JavaScriptコードは、読みやすさと保守性のために、よく構造化され、効率的に整理され、適切にコメントされている必要があります。",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "以下の機能とセクションを持つ「EduQuest」というオンライン学習プラットフォームのワンページウェブサイトを作成してください： \n \n1. コースカテゴリ（数学、科学、言語、芸術）へのリンクと検索バーを含む固定ナビゲーションバー。 \n \n2. オンラインで学習する学生を紹介する動画背景、「自分のペースで学習」「新しい情熱を発見」「視野を広げる」の間で3秒ごとに回転する動的なタグライン、コースカタログにつながる「始める」ボタンを含むヒーローセクション。 \n \n3. コース画像、タイトル、講師、説明のプレースホルダーを含むコースカードを表示する注目コースセクション。 \n \n4. 学習スタイルと興味を判定する短いクイズと、クイズを開始するボタンを含むインタラクティブな「学習パス」セクション。 \n \n5. 満足した学生からの証言を特集する「成功事例」セクション。証言テキストと学生名のプレースホルダーを含む。 \n \n6. プラットフォームのブログ、FAQ、プライバシーポリシーへのリンクと、連絡フォームとカスタマーサポート情報を含むモーダルウィンドウを開く「お問い合わせ」ボタンを含むフッター。 \n \n動画背景、コースカード、証言にはフィラープレースホルダーコンテンツを含めてください。CSSスタイルは`<head>`セクション内の`<style>`タグに埋め込み、JavaScriptコードは`<body>`セクションの最後の`<script>`タグ内に配置してください。 \n \nJavaScriptコードは、ヒーローセクションの動的なタグラインを処理し、3秒ごとに異なるタグラインを回転させる必要があります。"
}
]
}
]
});
console.log(msg);

```

</Tab>
</Tabs>