# 클라이언트 SDK

Claude API를 더 쉽게 사용할 수 있도록 여러 인기 있는 언어로 제공되는 클라이언트 라이브러리를 제공합니다.

---

이 페이지에는 Anthropic의 클라이언트 SDK에 대한 간단한 설치 지침과 오픈 소스 GitHub 저장소 링크가 포함되어 있습니다. 기본 사용 지침은 [API 참조](/docs/ko/api/overview)를 참조하세요. 자세한 사용 지침은 각 SDK의 GitHub 저장소를 참조하세요.

<Note>
Anthropic의 클라이언트 SDK를 파트너 플랫폼을 통해 사용하려면 추가 구성이 필요합니다. Amazon Bedrock을 사용하는 경우 [이 가이드](/docs/ko/build-with-claude/claude-on-amazon-bedrock)를 참조하세요. Google Cloud Vertex AI를 사용하는 경우 [이 가이드](/docs/ko/build-with-claude/claude-on-vertex-ai)를 참조하세요. Microsoft Foundry를 사용하는 경우 [이 가이드](/docs/ko/build-with-claude/claude-in-microsoft-foundry)를 참조하세요.
</Note>

## Python

[Python 라이브러리 GitHub 저장소](https://github.com/anthropics/anthropic-sdk-python)

**요구사항:** Python 3.8+

**설치:**

```bash
pip install anthropic
```

---

## TypeScript

[TypeScript 라이브러리 GitHub 저장소](https://github.com/anthropics/anthropic-sdk-typescript)

<Info>
이 라이브러리는 TypeScript로 작성되었지만 JavaScript 라이브러리에서도 사용할 수 있습니다.
</Info>

**설치:**

```bash
npm install @anthropic-ai/sdk
```

---

## Java

[Java 라이브러리 GitHub 저장소](https://github.com/anthropics/anthropic-sdk-java)

**요구사항:** Java 8 이상

**설치:**

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

[Go 라이브러리 GitHub 저장소](https://github.com/anthropics/anthropic-sdk-go)

**요구사항:** Go 1.22+

**설치:**

```bash
go get -u 'github.com/anthropics/anthropic-sdk-go@v1.17.0'
```

---

## C#

[C# 라이브러리 GitHub 저장소](https://github.com/anthropics/anthropic-sdk-csharp)

<Info>
C# SDK는 현재 베타 버전입니다.
</Info>

**요구사항:** .NET 8 이상

**설치:**

```bash
git clone git@github.com:anthropics/anthropic-sdk-csharp.git
dotnet add reference anthropic-sdk-csharp/src/Anthropic.Client
```

---

## Ruby

[Ruby 라이브러리 GitHub 저장소](https://github.com/anthropics/anthropic-sdk-ruby)

**요구사항:** Ruby 3.2.0 이상

**설치:**

Gemfile에 추가:
```ruby
gem "anthropic", "~> 1.13.0"
```

그 다음 실행:
```bash
bundle install
```

---

## PHP

[PHP 라이브러리 GitHub 저장소](https://github.com/anthropics/anthropic-sdk-php)

<Info>
PHP SDK는 현재 베타 버전입니다.
</Info>

**요구사항:** PHP 8.1.0 이상

**설치:**

```bash
composer require "anthropic-ai/sdk 0.3.0"
```

---

## 클라이언트 SDK의 베타 네임스페이스

모든 SDK에는 Anthropic이 베타 버전으로 출시하는 새로운 기능에 접근하기 위해 사용할 수 있는 `beta` 네임스페이스가 있습니다. 이를 [베타 헤더](/docs/ko/api/beta-headers)와 함께 사용하여 이러한 기능에 접근하세요. 특정 사용 예제는 각 SDK의 GitHub 저장소를 참조하세요.