# 客户端 SDK

我们提供多种流行语言的客户端库，使与 Claude API 的交互更加便捷。

---

本页面包含简要的安装说明和 Anthropic 客户端 SDK 的开源 GitHub 存储库链接。有关基本使用说明，请参阅 [API 参考](/docs/zh-CN/api/overview)。有关详细的使用说明，请参阅每个 SDK 的 GitHub 存储库。

<Note>
通过合作伙伴平台使用 Anthropic 的客户端 SDK 需要额外配置。如果您使用 Amazon Bedrock，请参阅[本指南](/docs/zh-CN/build-with-claude/claude-on-amazon-bedrock)；如果您使用 Google Cloud Vertex AI，请参阅[本指南](/docs/zh-CN/build-with-claude/claude-on-vertex-ai)；如果您使用 Microsoft Foundry，请参阅[本指南](/docs/zh-CN/build-with-claude/claude-in-microsoft-foundry)。
</Note>

## Python

[Python 库 GitHub 存储库](https://github.com/anthropics/anthropic-sdk-python)

**要求：** Python 3.8+

**安装：**

```bash
pip install anthropic
```

---

## TypeScript

[TypeScript 库 GitHub 存储库](https://github.com/anthropics/anthropic-sdk-typescript)

<Info>
虽然此库是用 TypeScript 编写的，但它也可以在 JavaScript 库中使用。
</Info>

**安装：**

```bash
npm install @anthropic-ai/sdk
```

---

## Java

[Java 库 GitHub 存储库](https://github.com/anthropics/anthropic-sdk-java)

**要求：** Java 8 或更高版本

**安装：**

Gradle:
```groovy
implementation("com.anthropic:anthropic-java:2.10.0")
```

Maven:
```xml
<dependency>
    <groupId>com.anthropic</groupId>
    <artifactId>anthropic-java</artifactId>
    <version>2.10.0</version>
</dependency>
```

---

## Go

[Go 库 GitHub 存储库](https://github.com/anthropics/anthropic-sdk-go)

**要求：** Go 1.22+

**安装：**

```bash
go get -u 'github.com/anthropics/anthropic-sdk-go@v1.17.0'
```

---

## C#

[C# 库 GitHub 存储库](https://github.com/anthropics/anthropic-sdk-csharp)

<Info>
C# SDK 目前处于测试版。
</Info>

**要求：** .NET 8 或更高版本

**安装：**

```bash
git clone git@github.com:anthropics/anthropic-sdk-csharp.git
dotnet add reference anthropic-sdk-csharp/src/Anthropic.Client
```

---

## Ruby

[Ruby 库 GitHub 存储库](https://github.com/anthropics/anthropic-sdk-ruby)

**要求：** Ruby 3.2.0 或更高版本

**安装：**

添加到您的 Gemfile：
```ruby
gem "anthropic", "~> 1.13.0"
```

然后运行：
```bash
bundle install
```

---

## PHP

[PHP 库 GitHub 存储库](https://github.com/anthropics/anthropic-sdk-php)

<Info>
PHP SDK 目前处于测试版。
</Info>

**要求：** PHP 8.1.0 或更高版本

**安装：**

```bash
composer require "anthropic-ai/sdk 0.3.0"
```

---

## 客户端 SDK 中的 Beta 命名空间

每个 SDK 都有一个 `beta` 命名空间，可用于访问 Anthropic 在测试版中发布的新功能。将其与 [beta 标头](/docs/zh-CN/api/beta-headers) 结合使用来访问这些功能。有关具体使用示例，请参阅每个 SDK 的 GitHub 存储库。