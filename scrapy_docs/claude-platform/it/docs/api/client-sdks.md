# Client SDK

Forniamo librerie client in diversi linguaggi popolari che semplificano il lavoro con l'API Claude.

---

Questa pagina include brevi istruzioni di installazione e link ai repository GitHub open-source degli SDK Client di Anthropic. Per le istruzioni di utilizzo di base, consulta il [riferimento API](/docs/it/api/overview). Per istruzioni di utilizzo dettagliate, fai riferimento al repository GitHub di ogni SDK.

<Note>
È necessaria una configurazione aggiuntiva per utilizzare gli SDK Client di Anthropic attraverso una piattaforma partner. Se stai utilizzando Amazon Bedrock, consulta [questa guida](/docs/it/build-with-claude/claude-on-amazon-bedrock); se stai utilizzando Google Cloud Vertex AI, consulta [questa guida](/docs/it/build-with-claude/claude-on-vertex-ai); se stai utilizzando Microsoft Foundry, consulta [questa guida](/docs/it/build-with-claude/claude-in-microsoft-foundry).
</Note>

## Python

[Repository GitHub della libreria Python](https://github.com/anthropics/anthropic-sdk-python)

**Requisiti:** Python 3.8+

**Installazione:**

```bash
pip install anthropic
```

---

## TypeScript

[Repository GitHub della libreria TypeScript](https://github.com/anthropics/anthropic-sdk-typescript)

<Info>
Sebbene questa libreria sia in TypeScript, può essere utilizzata anche in librerie JavaScript.
</Info>

**Installazione:**

```bash
npm install @anthropic-ai/sdk
```

---

## Java

[Repository GitHub della libreria Java](https://github.com/anthropics/anthropic-sdk-java)

**Requisiti:** Java 8 o versioni successive

**Installazione:**

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

[Repository GitHub della libreria Go](https://github.com/anthropics/anthropic-sdk-go)

**Requisiti:** Go 1.22+

**Installazione:**

```bash
go get -u 'github.com/anthropics/anthropic-sdk-go@v1.17.0'
```

---

## C#

[Repository GitHub della libreria C#](https://github.com/anthropics/anthropic-sdk-csharp)

<Info>
L'SDK C# è attualmente in beta.
</Info>

**Requisiti:** .NET 8 o versioni successive

**Installazione:**

```bash
git clone git@github.com:anthropics/anthropic-sdk-csharp.git
dotnet add reference anthropic-sdk-csharp/src/Anthropic.Client
```

---

## Ruby

[Repository GitHub della libreria Ruby](https://github.com/anthropics/anthropic-sdk-ruby)

**Requisiti:** Ruby 3.2.0 o versioni successive

**Installazione:**

Aggiungi al tuo Gemfile:
```ruby
gem "anthropic", "~> 1.13.0"
```

Quindi esegui:
```bash
bundle install
```

---

## PHP

[Repository GitHub della libreria PHP](https://github.com/anthropics/anthropic-sdk-php)

<Info>
L'SDK PHP è attualmente in beta.
</Info>

**Requisiti:** PHP 8.1.0 o versioni successive

**Installazione:**

```bash
composer require "anthropic-ai/sdk 0.3.0"
```

---

## Namespace beta negli SDK client

Ogni SDK ha uno spazio dei nomi `beta` disponibile per accedere alle nuove funzionalità che Anthropic rilascia nelle versioni beta. Utilizza questo in combinazione con [intestazioni beta](/docs/it/api/beta-headers) per accedere a queste funzionalità. Fai riferimento al repository GitHub di ogni SDK per esempi di utilizzo specifici.