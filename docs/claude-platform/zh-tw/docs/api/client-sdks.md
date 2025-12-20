# 客戶端 SDK

我們提供多種流行語言的客戶端程式庫，使您更容易使用 Claude API。

---

本頁面包含簡要的安裝說明和 Anthropic 客戶端 SDK 的開源 GitHub 儲存庫連結。如需基本使用說明，請參閱 [API 參考](/docs/zh-TW/api/overview)。如需詳細的使用說明，請參閱各個 SDK 的 GitHub 儲存庫。

<Note>
透過合作夥伴平台使用 Anthropic 的客戶端 SDK 需要額外的配置。如果您使用 Amazon Bedrock，請參閱 [本指南](/docs/zh-TW/build-with-claude/claude-on-amazon-bedrock)；如果您使用 Google Cloud Vertex AI，請參閱 [本指南](/docs/zh-TW/build-with-claude/claude-on-vertex-ai)；如果您使用 Microsoft Foundry，請參閱 [本指南](/docs/zh-TW/build-with-claude/claude-in-microsoft-foundry)。
</Note>

## Python

[Python 程式庫 GitHub 儲存庫](https://github.com/anthropics/anthropic-sdk-python)

**需求：** Python 3.8+

**安裝：**

```bash
pip install anthropic
```

---

## TypeScript

[TypeScript 程式庫 GitHub 儲存庫](https://github.com/anthropics/anthropic-sdk-typescript)

<Info>
雖然此程式庫是用 TypeScript 編寫的，但它也可以在 JavaScript 程式庫中使用。
</Info>

**安裝：**

```bash
npm install @anthropic-ai/sdk
```

---

## Java

[Java 程式庫 GitHub 儲存庫](https://github.com/anthropics/anthropic-sdk-java)

**需求：** Java 8 或更新版本

**安裝：**

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

[Go 程式庫 GitHub 儲存庫](https://github.com/anthropics/anthropic-sdk-go)

**需求：** Go 1.22+

**安裝：**

```bash
go get -u 'github.com/anthropics/anthropic-sdk-go@v1.17.0'
```

---

## C#

[C# 程式庫 GitHub 儲存庫](https://github.com/anthropics/anthropic-sdk-csharp)

<Info>
C# SDK 目前處於測試版。
</Info>

**需求：** .NET 8 或更新版本

**安裝：**

```bash
git clone git@github.com:anthropics/anthropic-sdk-csharp.git
dotnet add reference anthropic-sdk-csharp/src/Anthropic.Client
```

---

## Ruby

[Ruby 程式庫 GitHub 儲存庫](https://github.com/anthropics/anthropic-sdk-ruby)

**需求：** Ruby 3.2.0 或更新版本

**安裝：**

新增至您的 Gemfile：
```ruby
gem "anthropic", "~> 1.13.0"
```

然後執行：
```bash
bundle install
```

---

## PHP

[PHP 程式庫 GitHub 儲存庫](https://github.com/anthropics/anthropic-sdk-php)

<Info>
PHP SDK 目前處於測試版。
</Info>

**需求：** PHP 8.1.0 或更新版本

**安裝：**

```bash
composer require "anthropic-ai/sdk 0.3.0"
```

---

## 客戶端 SDK 中的 Beta 命名空間

每個 SDK 都有一個 `beta` 命名空間，可用於存取 Anthropic 在測試版中發佈的新功能。將此與 [beta 標頭](/docs/zh-TW/api/beta-headers) 結合使用以存取這些功能。請參閱各個 SDK 的 GitHub 儲存庫以取得特定的使用範例。