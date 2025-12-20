# Клиентские SDK

Мы предоставляем клиентские библиотеки на ряде популярных языков программирования, которые облегчают работу с API Claude.

---

На этой странице содержатся краткие инструкции по установке и ссылки на открытые репозитории GitHub для Client SDKs Anthropic. Для получения основных инструкций по использованию см. [справочник API](/docs/ru/api/overview). Для получения подробных инструкций по использованию обратитесь к репозиторию GitHub каждого SDK.

<Note>
Для использования Client SDKs Anthropic через платформу партнера требуется дополнительная конфигурация. Если вы используете Amazon Bedrock, см. [это руководство](/docs/ru/build-with-claude/claude-on-amazon-bedrock); если вы используете Google Cloud Vertex AI, см. [это руководство](/docs/ru/build-with-claude/claude-on-vertex-ai); если вы используете Microsoft Foundry, см. [это руководство](/docs/ru/build-with-claude/claude-in-microsoft-foundry).
</Note>

## Python

[Репозиторий библиотеки Python на GitHub](https://github.com/anthropics/anthropic-sdk-python)

**Требования:** Python 3.8+

**Установка:**

```bash
pip install anthropic
```

---

## TypeScript

[Репозиторий библиотеки TypeScript на GitHub](https://github.com/anthropics/anthropic-sdk-typescript)

<Info>
Хотя эта библиотека написана на TypeScript, она также может использоваться в библиотеках JavaScript.
</Info>

**Установка:**

```bash
npm install @anthropic-ai/sdk
```

---

## Java

[Репозиторий библиотеки Java на GitHub](https://github.com/anthropics/anthropic-sdk-java)

**Требования:** Java 8 или более поздняя версия

**Установка:**

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

[Репозиторий библиотеки Go на GitHub](https://github.com/anthropics/anthropic-sdk-go)

**Требования:** Go 1.22+

**Установка:**

```bash
go get -u 'github.com/anthropics/anthropic-sdk-go@v1.17.0'
```

---

## C#

[Репозиторий библиотеки C# на GitHub](https://github.com/anthropics/anthropic-sdk-csharp)

<Info>
SDK для C# в настоящее время находится в бета-версии.
</Info>

**Требования:** .NET 8 или более поздняя версия

**Установка:**

```bash
git clone git@github.com:anthropics/anthropic-sdk-csharp.git
dotnet add reference anthropic-sdk-csharp/src/Anthropic.Client
```

---

## Ruby

[Репозиторий библиотеки Ruby на GitHub](https://github.com/anthropics/anthropic-sdk-ruby)

**Требования:** Ruby 3.2.0 или более поздняя версия

**Установка:**

Добавьте в ваш Gemfile:
```ruby
gem "anthropic", "~> 1.13.0"
```

Затем выполните:
```bash
bundle install
```

---

## PHP

[Репозиторий библиотеки PHP на GitHub](https://github.com/anthropics/anthropic-sdk-php)

<Info>
SDK для PHP в настоящее время находится в бета-версии.
</Info>

**Требования:** PHP 8.1.0 или выше

**Установка:**

```bash
composer require "anthropic-ai/sdk 0.3.0"
```

---

## Бета-пространство имён в клиентских SDK

Каждый SDK имеет пространство имён `beta`, которое доступно для доступа к новым функциям, которые Anthropic выпускает в бета-версиях. Используйте это в сочетании с [бета-заголовками](/docs/ru/api/beta-headers) для доступа к этим функциям. Обратитесь к репозиторию GitHub каждого SDK для получения конкретных примеров использования.