# SDKs do Cliente

Fornecemos bibliotecas de cliente em várias linguagens populares que facilitam o trabalho com a API Claude.

---

Esta página inclui breves instruções de instalação e links para os repositórios GitHub de código aberto dos SDKs do Cliente da Anthropic. Para instruções de uso básico, consulte a [referência da API](/docs/pt-BR/api/overview). Para instruções de uso detalhadas, consulte o repositório GitHub de cada SDK.

<Note>
Configuração adicional é necessária para usar os SDKs do Cliente da Anthropic através de uma plataforma parceira. Se você estiver usando Amazon Bedrock, consulte [este guia](/docs/pt-BR/build-with-claude/claude-on-amazon-bedrock); se você estiver usando Google Cloud Vertex AI, consulte [este guia](/docs/pt-BR/build-with-claude/claude-on-vertex-ai); se você estiver usando Microsoft Foundry, consulte [este guia](/docs/pt-BR/build-with-claude/claude-in-microsoft-foundry).
</Note>

## Python

[Repositório GitHub da biblioteca Python](https://github.com/anthropics/anthropic-sdk-python)

**Requisitos:** Python 3.8+

**Instalação:**

```bash
pip install anthropic
```

---

## TypeScript

[Repositório GitHub da biblioteca TypeScript](https://github.com/anthropics/anthropic-sdk-typescript)

<Info>
Embora esta biblioteca seja em TypeScript, ela também pode ser usada em bibliotecas JavaScript.
</Info>

**Instalação:**

```bash
npm install @anthropic-ai/sdk
```

---

## Java

[Repositório GitHub da biblioteca Java](https://github.com/anthropics/anthropic-sdk-java)

**Requisitos:** Java 8 ou posterior

**Instalação:**

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

[Repositório GitHub da biblioteca Go](https://github.com/anthropics/anthropic-sdk-go)

**Requisitos:** Go 1.22+

**Instalação:**

```bash
go get -u 'github.com/anthropics/anthropic-sdk-go@v1.17.0'
```

---

## C#

[Repositório GitHub da biblioteca C#](https://github.com/anthropics/anthropic-sdk-csharp)

<Info>
O SDK C# está atualmente em beta.
</Info>

**Requisitos:** .NET 8 ou posterior

**Instalação:**

```bash
git clone git@github.com:anthropics/anthropic-sdk-csharp.git
dotnet add reference anthropic-sdk-csharp/src/Anthropic.Client
```

---

## Ruby

[Repositório GitHub da biblioteca Ruby](https://github.com/anthropics/anthropic-sdk-ruby)

**Requisitos:** Ruby 3.2.0 ou posterior

**Instalação:**

Adicione ao seu Gemfile:
```ruby
gem "anthropic", "~> 1.13.0"
```

Depois execute:
```bash
bundle install
```

---

## PHP

[Repositório GitHub da biblioteca PHP](https://github.com/anthropics/anthropic-sdk-php)

<Info>
O SDK PHP está atualmente em beta.
</Info>

**Requisitos:** PHP 8.1.0 ou superior

**Instalação:**

```bash
composer require "anthropic-ai/sdk 0.3.0"
```

---

## Namespace beta nos SDKs do cliente

Cada SDK possui um namespace `beta` que está disponível para acessar novos recursos que a Anthropic lança em versões beta. Use isso em conjunto com [cabeçalhos beta](/docs/pt-BR/api/beta-headers) para acessar esses recursos. Consulte o repositório GitHub de cada SDK para exemplos de uso específicos.