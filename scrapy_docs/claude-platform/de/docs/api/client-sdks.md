# Client SDKs

Wir stellen Client-Bibliotheken in einer Reihe von beliebten Sprachen zur Verfügung, die die Arbeit mit der Claude API erleichtern.

---

Diese Seite enthält kurze Installationsanweisungen und Links zu den Open-Source-GitHub-Repositories für die Client SDKs von Anthropic. Grundlegende Verwendungsanweisungen finden Sie in der [API-Referenz](/docs/de/api/overview). Detaillierte Verwendungsanweisungen finden Sie im GitHub-Repository jedes SDK.

<Note>
Zusätzliche Konfiguration ist erforderlich, um die Client SDKs von Anthropic über eine Partner-Plattform zu verwenden. Wenn Sie Amazon Bedrock verwenden, siehe [diese Anleitung](/docs/de/build-with-claude/claude-on-amazon-bedrock); wenn Sie Google Cloud Vertex AI verwenden, siehe [diese Anleitung](/docs/de/build-with-claude/claude-on-vertex-ai); wenn Sie Microsoft Foundry verwenden, siehe [diese Anleitung](/docs/de/build-with-claude/claude-in-microsoft-foundry).
</Note>

## Python

[Python-Bibliothek GitHub-Repository](https://github.com/anthropics/anthropic-sdk-python)

**Anforderungen:** Python 3.8+

**Installation:**

```bash
pip install anthropic
```

---

## TypeScript

[TypeScript-Bibliothek GitHub-Repository](https://github.com/anthropics/anthropic-sdk-typescript)

<Info>
Obwohl diese Bibliothek in TypeScript geschrieben ist, kann sie auch in JavaScript-Bibliotheken verwendet werden.
</Info>

**Installation:**

```bash
npm install @anthropic-ai/sdk
```

---

## Java

[Java-Bibliothek GitHub-Repository](https://github.com/anthropics/anthropic-sdk-java)

**Anforderungen:** Java 8 oder später

**Installation:**

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

[Go-Bibliothek GitHub-Repository](https://github.com/anthropics/anthropic-sdk-go)

**Anforderungen:** Go 1.22+

**Installation:**

```bash
go get -u 'github.com/anthropics/anthropic-sdk-go@v1.17.0'
```

---

## C#

[C#-Bibliothek GitHub-Repository](https://github.com/anthropics/anthropic-sdk-csharp)

<Info>
Das C# SDK befindet sich derzeit in der Beta-Phase.
</Info>

**Anforderungen:** .NET 8 oder später

**Installation:**

```bash
git clone git@github.com:anthropics/anthropic-sdk-csharp.git
dotnet add reference anthropic-sdk-csharp/src/Anthropic.Client
```

---

## Ruby

[Ruby-Bibliothek GitHub-Repository](https://github.com/anthropics/anthropic-sdk-ruby)

**Anforderungen:** Ruby 3.2.0 oder später

**Installation:**

Fügen Sie zu Ihrer Gemfile hinzu:
```ruby
gem "anthropic", "~> 1.13.0"
```

Führen Sie dann aus:
```bash
bundle install
```

---

## PHP

[PHP-Bibliothek GitHub-Repository](https://github.com/anthropics/anthropic-sdk-php)

<Info>
Das PHP SDK befindet sich derzeit in der Beta-Phase.
</Info>

**Anforderungen:** PHP 8.1.0 oder höher

**Installation:**

```bash
composer require "anthropic-ai/sdk 0.3.0"
```

---

## Beta-Namespace in Client SDKs

Jedes SDK hat einen `beta`-Namespace, der für den Zugriff auf neue Funktionen verfügbar ist, die Anthropic in Beta-Versionen veröffentlicht. Verwenden Sie dies in Verbindung mit [Beta-Headern](/docs/de/api/beta-headers), um auf diese Funktionen zuzugreifen. Beachten Sie das GitHub-Repository jedes SDK für spezifische Verwendungsbeispiele.