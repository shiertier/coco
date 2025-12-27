# SDKs de Cliente

Proporcionamos bibliotecas de cliente en varios lenguajes populares que facilitan el trabajo con la API de Claude.

---

Esta página incluye breves instrucciones de instalación y enlaces a los repositorios de GitHub de código abierto de los SDKs de Cliente de Anthropic. Para instrucciones de uso básico, consulte la [referencia de API](/docs/es/api/overview). Para instrucciones de uso detalladas, consulte el repositorio de GitHub de cada SDK.

<Note>
Se requiere configuración adicional para usar los SDKs de Cliente de Anthropic a través de una plataforma asociada. Si está utilizando Amazon Bedrock, consulte [esta guía](/docs/es/build-with-claude/claude-on-amazon-bedrock); si está utilizando Google Cloud Vertex AI, consulte [esta guía](/docs/es/build-with-claude/claude-on-vertex-ai); si está utilizando Microsoft Foundry, consulte [esta guía](/docs/es/build-with-claude/claude-in-microsoft-foundry).
</Note>

## Python

[Repositorio de GitHub de la biblioteca Python](https://github.com/anthropics/anthropic-sdk-python)

**Requisitos:** Python 3.8+

**Instalación:**

```bash
pip install anthropic
```

---

## TypeScript

[Repositorio de GitHub de la biblioteca TypeScript](https://github.com/anthropics/anthropic-sdk-typescript)

<Info>
Aunque esta biblioteca está en TypeScript, también se puede usar en bibliotecas de JavaScript.
</Info>

**Instalación:**

```bash
npm install @anthropic-ai/sdk
```

---

## Java

[Repositorio de GitHub de la biblioteca Java](https://github.com/anthropics/anthropic-sdk-java)

**Requisitos:** Java 8 o posterior

**Instalación:**

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

[Repositorio de GitHub de la biblioteca Go](https://github.com/anthropics/anthropic-sdk-go)

**Requisitos:** Go 1.22+

**Instalación:**

```bash
go get -u 'github.com/anthropics/anthropic-sdk-go@v1.17.0'
```

---

## C#

[Repositorio de GitHub de la biblioteca C#](https://github.com/anthropics/anthropic-sdk-csharp)

<Info>
El SDK de C# está actualmente en beta.
</Info>

**Requisitos:** .NET 8 o posterior

**Instalación:**

```bash
git clone git@github.com:anthropics/anthropic-sdk-csharp.git
dotnet add reference anthropic-sdk-csharp/src/Anthropic.Client
```

---

## Ruby

[Repositorio de GitHub de la biblioteca Ruby](https://github.com/anthropics/anthropic-sdk-ruby)

**Requisitos:** Ruby 3.2.0 o posterior

**Instalación:**

Agregue a su Gemfile:
```ruby
gem "anthropic", "~> 1.13.0"
```

Luego ejecute:
```bash
bundle install
```

---

## PHP

[Repositorio de GitHub de la biblioteca PHP](https://github.com/anthropics/anthropic-sdk-php)

<Info>
El SDK de PHP está actualmente en beta.
</Info>

**Requisitos:** PHP 8.1.0 o superior

**Instalación:**

```bash
composer require "anthropic-ai/sdk 0.3.0"
```

---

## Espacio de nombres beta en los SDKs de cliente

Cada SDK tiene un espacio de nombres `beta` que está disponible para acceder a nuevas características que Anthropic lanza en versiones beta. Úselo junto con [encabezados beta](/docs/es/api/beta-headers) para acceder a estas características. Consulte el repositorio de GitHub de cada SDK para ejemplos de uso específicos.