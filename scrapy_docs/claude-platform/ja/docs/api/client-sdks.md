# クライアント SDK

Claude API を使いやすくするために、多くの人気のあるプログラミング言語でクライアント ライブラリを提供しています。

---

このページには、Anthropic のクライアント SDK の簡単なインストール手順と、オープンソース GitHub リポジトリへのリンクが含まれています。基本的な使用方法については、[API リファレンス](/docs/ja/api/overview)を参照してください。詳細な使用方法については、各 SDK の GitHub リポジトリを参照してください。

<Note>
Anthropic のクライアント SDK をパートナー プラットフォーム経由で使用するには、追加の設定が必要です。Amazon Bedrock を使用している場合は、[このガイド](/docs/ja/build-with-claude/claude-on-amazon-bedrock)を参照してください。Google Cloud Vertex AI を使用している場合は、[このガイド](/docs/ja/build-with-claude/claude-on-vertex-ai)を参照してください。Microsoft Foundry を使用している場合は、[このガイド](/docs/ja/build-with-claude/claude-in-microsoft-foundry)を参照してください。
</Note>

## Python

[Python ライブラリ GitHub リポジトリ](https://github.com/anthropics/anthropic-sdk-python)

**要件:** Python 3.8 以上

**インストール:**

```bash
pip install anthropic
```

---

## TypeScript

[TypeScript ライブラリ GitHub リポジトリ](https://github.com/anthropics/anthropic-sdk-typescript)

<Info>
このライブラリは TypeScript で書かれていますが、JavaScript ライブラリでも使用できます。
</Info>

**インストール:**

```bash
npm install @anthropic-ai/sdk
```

---

## Java

[Java ライブラリ GitHub リポジトリ](https://github.com/anthropics/anthropic-sdk-java)

**要件:** Java 8 以降

**インストール:**

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

[Go ライブラリ GitHub リポジトリ](https://github.com/anthropics/anthropic-sdk-go)

**要件:** Go 1.22 以上

**インストール:**

```bash
go get -u 'github.com/anthropics/anthropic-sdk-go@v1.17.0'
```

---

## C#

[C# ライブラリ GitHub リポジトリ](https://github.com/anthropics/anthropic-sdk-csharp)

<Info>
C# SDK は現在ベータ版です。
</Info>

**要件:** .NET 8 以降

**インストール:**

```bash
git clone git@github.com:anthropics/anthropic-sdk-csharp.git
dotnet add reference anthropic-sdk-csharp/src/Anthropic.Client
```

---

## Ruby

[Ruby ライブラリ GitHub リポジトリ](https://github.com/anthropics/anthropic-sdk-ruby)

**要件:** Ruby 3.2.0 以降

**インストール:**

Gemfile に追加:
```ruby
gem "anthropic", "~> 1.13.0"
```

その後、実行:
```bash
bundle install
```

---

## PHP

[PHP ライブラリ GitHub リポジトリ](https://github.com/anthropics/anthropic-sdk-php)

<Info>
PHP SDK は現在ベータ版です。
</Info>

**要件:** PHP 8.1.0 以上

**インストール:**

```bash
composer require "anthropic-ai/sdk 0.3.0"
```

---

## クライアント SDK のベータ名前空間

すべての SDK には、Anthropic がベータ版でリリースする新機能にアクセスするための `beta` 名前空間があります。これを [ベータ ヘッダー](/docs/ja/api/beta-headers)と組み合わせて使用して、これらの機能にアクセスしてください。具体的な使用例については、各 SDK の GitHub リポジトリを参照してください。