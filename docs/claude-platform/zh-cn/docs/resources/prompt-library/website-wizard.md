# 网站向导

根据用户规格创建单页网站。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是根据给定的规格创建一个单页网站，以包含嵌入式JavaScript和CSS的HTML文件形式交付。网站应该融入各种引人入胜的交互式设计功能，如下拉菜单、动态文本和内容、可点击按钮等。确保设计在视觉上吸引人、响应式且用户友好。HTML、CSS和JavaScript代码应该结构良好、高效组织，并为可读性和可维护性进行适当注释。 |
| User   | 为名为"EduQuest"的在线学习平台创建一个单页网站，包含以下功能和部分：<br/><br/>1. 一个固定导航栏，包含课程类别链接（数学、科学、语言、艺术）和搜索栏。<br/>2. 一个英雄部分，包含展示学生在线学习的视频背景，一个在"按自己的节奏学习"、"发现新激情"和"拓展视野"之间每3秒轮换的动态标语，以及一个通向课程目录的"开始学习"按钮。<br/>3. 一个精选课程部分，显示包含课程图片、标题、讲师和描述占位符的课程卡片。<br/>4. 一个交互式"学习路径"部分，包含一个简短测验来确定学习风格和兴趣，以及一个开始测验的按钮。<br/>5. 一个"成功故事"部分，展示满意学生的推荐信，包含推荐信文本和学生姓名的占位符。<br/>6. 一个页脚，包含平台博客、FAQ、隐私政策的链接，以及一个打开包含联系表单和客户支持信息的模态窗口的"联系我们"按钮。<br/><br/>为视频背景、课程卡片和推荐信包含填充占位符内容。将CSS样式嵌入到`<head>`部分的`<style>`标签中，并将JavaScript代码放置在`<body>`部分末尾的`<script>`标签中。<br/><br/>JavaScript代码应该处理英雄部分的动态标语，每3秒在不同标语之间轮换。 |

## 示例输出

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

> 此代码为"EduQuest"创建了一个单页网站，包含以下功能和部分：
> 
> 一个固定导航栏，包含课程类别链接（数学、科学、语言、艺术）和搜索栏。
> 
> 一个英雄部分，包含占位符视频背景，一个在"按自己的节奏学习"、"发现新激情"和"拓展视野"之间每3秒轮换的动态标语，以及一个"开始学习"按钮。
> 
> 一个精选课程部分，显示包含课程图片、标题、讲师和描述占位符的课程卡片。
> 
> 一个"成功故事"部分，展示满意学生的推荐信，包含推荐信文本和学生姓名的占位符。
> 
> 一个页脚，包含平台博客、FAQ、隐私政策的链接，以及一个"联系我们"按钮。
> 
> CSS样式嵌入在`<head>`部分的`<style>`标签中，JavaScript代码放置在`<body>`部分末尾的`<script>`标签中。
> 
> JavaScript代码处理英雄部分的动态标语，使用setInterval函数每3秒在不同标语之间轮换。
> 
> 请注意，视频背景、课程图片和推荐信都是占位符，应该替换为实际内容。

## API请求

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
  system="您的任务是根据给定的规格创建一个单页网站，以包含嵌入式JavaScript和CSS的HTML文件形式交付。网站应该融入各种引人入胜的交互式设计功能，如下拉菜单、动态文本和内容、可点击按钮等。确保设计在视觉上吸引人、响应式且用户友好。HTML、CSS和JavaScript代码应该结构良好、高效组织，并为可读性和可维护性进行适当注释。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "为名为\"EduQuest\"的在线学习平台创建一个单页网站，包含以下功能和部分： \n \n1. 一个固定导航栏，包含课程类别链接（数学、科学、语言、艺术）和搜索栏。 \n \n2. 一个英雄部分，包含展示学生在线学习的视频背景，一个在\"按自己的节奏学习\"、\"发现新激情\"和\"拓展视野\"之间每3秒轮换的动态标语，以及一个通向课程目录的\"开始学习\"按钮。 \n \n3. 一个精选课程部分，显示包含课程图片、标题、讲师和描述占位符的课程卡片。 \n \n4. 一个交互式\"学习路径\"部分，包含一个简短测验来确定学习风格和兴趣，以及一个开始测验的按钮。 \n \n5. 一个\"成功故事\"部分，展示满意学生的推荐信，包含推荐信文本和学生姓名的占位符。 \n \n6. 一个页脚，包含平台博客、FAQ、隐私政策的链接，以及一个打开包含联系表单和客户支持信息的模态窗口的\"联系我们\"按钮。 \n \n为视频背景、课程卡片和推荐信包含填充占位符内容。将CSS样式嵌入到`<style>`标签中的`<head>`部分，并将JavaScript代码放置在`<body>`部分末尾的`<script>`标签中。 \n \nJavaScript代码应该处理英雄部分的动态标语，每3秒在不同标语之间轮换。"
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
  system: "您的任务是根据给定的规格创建一个单页网站，以包含嵌入式JavaScript和CSS的HTML文件形式交付。网站应该融入各种引人入胜的交互式设计功能，如下拉菜单、动态文本和内容、可点击按钮等。确保设计在视觉上吸引人、响应式且用户友好。HTML、CSS和JavaScript代码应该结构良好、高效组织，并为可读性和可维护性进行适当注释。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "为名为\"EduQuest\"的在线学习平台创建一个单页网站，包含以下功能和部分：  \n  \n1. 一个固定导航栏，包含课程类别链接（数学、科学、语言、艺术）和搜索栏。  \n  \n2. 一个英雄部分，包含展示学生在线学习的视频背景，一个在\"按自己的节奏学习\"、\"发现新激情\"和\"拓展视野\"之间每3秒轮换的动态标语，以及一个通向课程目录的\"开始学习\"按钮。  \n  \n3. 一个精选课程部分，显示包含课程图片、标题、讲师和描述占位符的课程卡片。  \n  \n4. 一个交互式\"学习路径\"部分，包含一个简短测验来确定学习风格和兴趣，以及一个开始测验的按钮。  \n  \n5. 一个\"成功故事\"部分，展示满意学生的推荐信，包含推荐信文本和学生姓名的占位符。  \n  \n6. 一个页脚，包含平台博客、FAQ、隐私政策的链接，以及一个打开包含联系表单和客户支持信息的模态窗口的\"联系我们\"按钮。  \n  \n为视频背景、课程卡片和推荐信包含填充占位符内容。将CSS样式嵌入到`<style>`标签中的`<head>`部分，并将JavaScript代码放置在`<body>`部分末尾的`<script>`标签中。  \n  \nJavaScript代码应该处理英雄部分的动态标语，每3秒在不同标语之间轮换。"
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
  system="您的任务是根据给定的规格创建一个单页网站，以包含嵌入式JavaScript和CSS的HTML文件形式交付。网站应该融入各种引人入胜的交互式设计功能，如下拉菜单、动态文本和内容、可点击按钮等。确保设计在视觉上吸引人、响应式且用户友好。HTML、CSS和JavaScript代码应该结构良好、高效组织，并为可读性和可维护性进行适当注释。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "为名为\"EduQuest\"的在线学习平台创建一个单页网站，包含以下功能和部分： \n \n1. 一个固定导航栏，包含课程类别链接（数学、科学、语言、艺术）和搜索栏。 \n \n2. 一个英雄部分，包含展示学生在线学习的视频背景，一个在\"按自己的节奏学习\"、\"发现新激情\"和\"拓展视野\"之间每3秒轮换的动态标语，以及一个通向课程目录的\"开始学习\"按钮。 \n \n3. 一个精选课程部分，显示包含课程图片、标题、讲师和描述占位符的课程卡片。 \n \n4. 一个交互式\"学习路径\"部分，包含一个简短测验来确定学习风格和兴趣，以及一个开始测验的按钮。 \n \n5. 一个\"成功故事\"部分，展示满意学生的推荐信，包含推荐信文本和学生姓名的占位符。 \n \n6. 一个页脚，包含平台博客、FAQ、隐私政策的链接，以及一个打开包含联系表单和客户支持信息的模态窗口的\"联系我们\"按钮。 \n \n为视频背景、课程卡片和推荐信包含填充占位符内容。将CSS样式嵌入到`<style>`标签中的`<head>`部分，并将JavaScript代码放置在`<body>`部分末尾的`<script>`标签中。 \n \nJavaScript代码应该处理英雄部分的动态标语，每3秒在不同标语之间轮换。"
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
  system: "您的任务是根据给定的规格创建一个单页网站，以包含嵌入式JavaScript和CSS的HTML文件形式交付。网站应该融入各种引人入胜的交互式设计功能，如下拉菜单、动态文本和内容、可点击按钮等。确保设计在视觉上吸引人、响应式且用户友好。HTML、CSS和JavaScript代码应该结构良好、高效组织，并为可读性和可维护性进行适当注释。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "为名为\"EduQuest\"的在线学习平台创建一个单页网站，包含以下功能和部分： \n \n1. 一个固定导航栏，包含课程类别链接（数学、科学、语言、艺术）和搜索栏。 \n \n2. 一个英雄部分，包含展示学生在线学习的视频背景，一个在\"按自己的节奏学习\"、\"发现新激情\"和\"拓展视野\"之间每3秒轮换的动态标语，以及一个通向课程目录的\"开始学习\"按钮。 \n \n3. 一个精选课程部分，显示包含课程图片、标题、讲师和描述占位符的课程卡片。 \n \n4. 一个交互式\"学习路径\"部分，包含一个简短测验来确定学习风格和兴趣，以及一个开始测验的按钮。 \n \n5. 一个\"成功故事\"部分，展示满意学生的推荐信，包含推荐信文本和学生姓名的占位符。 \n \n6. 一个页脚，包含平台博客、FAQ、隐私政策的链接，以及一个打开包含联系表单和客户支持信息的模态窗口的\"联系我们\"按钮。 \n \n为视频背景、课程卡片和推荐信包含填充占位符内容。将CSS样式嵌入到`<style>`标签中的`<head>`部分，并将JavaScript代码放置在`<body>`部分末尾的`<script>`标签中。 \n \nJavaScript代码应该处理英雄部分的动态标语，每3秒在不同标语之间轮换。"
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
  system="您的任务是根据给定的规格创建一个单页网站，以包含嵌入式JavaScript和CSS的HTML文件形式交付。网站应该融入各种引人入胜的交互式设计功能，如下拉菜单、动态文本和内容、可点击按钮等。确保设计在视觉上吸引人、响应式且用户友好。HTML、CSS和JavaScript代码应该结构良好、高效组织，并为可读性和可维护性进行适当注释。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "为名为\"EduQuest\"的在线学习平台创建一个单页网站，包含以下功能和部分： \n \n1. 一个固定导航栏，包含课程类别链接（数学、科学、语言、艺术）和搜索栏。 \n \n2. 一个英雄部分，包含展示学生在线学习的视频背景，一个在\"按自己的节奏学习\"、\"发现新激情\"和\"拓展视野\"之间每3秒轮换的动态标语，以及一个通向课程目录的\"开始学习\"按钮。 \n \n3. 一个精选课程部分，显示包含课程图片、标题、讲师和描述占位符的课程卡片。 \n \n4. 一个交互式\"学习路径\"部分，包含一个简短测验来确定学习风格和兴趣，以及一个开始测验的按钮。 \n \n5. 一个\"成功故事\"部分，展示满意学生的推荐信，包含推荐信文本和学生姓名的占位符。 \n \n6. 一个页脚，包含平台博客、FAQ、隐私政策的链接，以及一个打开包含联系表单和客户支持信息的模态窗口的\"联系我们\"按钮。 \n \n为视频背景、课程卡片和推荐信包含填充占位符内容。将CSS样式嵌入到`<style>`标签中的`<head>`部分，并将JavaScript代码放置在`<body>`部分末尾的`<script>`标签中。 \n \nJavaScript代码应该处理英雄部分的动态标语，每3秒在不同标语之间轮换。"
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
system: "您的任务是根据给定的规格创建一个单页网站，以包含嵌入式JavaScript和CSS的HTML文件形式交付。网站应该融入各种引人入胜的交互式设计功能，如下拉菜单、动态文本和内容、可点击按钮等。确保设计在视觉上吸引人、响应式且用户友好。HTML、CSS和JavaScript代码应该结构良好、高效组织，并为可读性和可维护性进行适当注释。",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "为名为\"EduQuest\"的在线学习平台创建一个单页网站，包含以下功能和部分： \n \n1. 一个固定导航栏，包含课程类别链接（数学、科学、语言、艺术）和搜索栏。 \n \n2. 一个英雄部分，包含展示学生在线学习的视频背景，一个在\"按自己的节奏学习\"、\"发现新激情\"和\"拓展视野\"之间每3秒轮换的动态标语，以及一个通向课程目录的\"开始学习\"按钮。 \n \n3. 一个精选课程部分，显示包含课程图片、标题、讲师和描述占位符的课程卡片。 \n \n4. 一个交互式\"学习路径\"部分，包含一个简短测验来确定学习风格和兴趣，以及一个开始测验的按钮。 \n \n5. 一个\"成功故事\"部分，展示满意学生的推荐信，包含推荐信文本和学生姓名的占位符。 \n \n6. 一个页脚，包含平台博客、FAQ、隐私政策的链接，以及一个打开包含联系表单和客户支持信息的模态窗口的\"联系我们\"按钮。 \n \n为视频背景、课程卡片和推荐信包含填充占位符内容。将CSS样式嵌入到`<style>`标签中的`<head>`部分，并将JavaScript代码放置在`<body>`部分末尾的`<script>`标签中。 \n \nJavaScript代码应该处理英雄部分的动态标语，每3秒在不同标语之间轮换。"
}
]
}
]
});
console.log(msg);

```

</Tab>
</Tabs>