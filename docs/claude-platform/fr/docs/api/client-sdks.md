# SDK clients

Nous fournissons des bibliothèques clientes dans un certain nombre de langages populaires qui facilitent le travail avec l'API Claude.

---

Cette page comprend des instructions d'installation brèves et des liens vers les référentiels GitHub open-source des SDK clients d'Anthropic. Pour les instructions d'utilisation de base, consultez la [référence API](/docs/fr/api/overview). Pour des instructions d'utilisation détaillées, reportez-vous au référentiel GitHub de chaque SDK.

<Note>
Une configuration supplémentaire est nécessaire pour utiliser les SDK clients d'Anthropic via une plateforme partenaire. Si vous utilisez Amazon Bedrock, consultez [ce guide](/docs/fr/build-with-claude/claude-on-amazon-bedrock) ; si vous utilisez Google Cloud Vertex AI, consultez [ce guide](/docs/fr/build-with-claude/claude-on-vertex-ai) ; si vous utilisez Microsoft Foundry, consultez [ce guide](/docs/fr/build-with-claude/claude-in-microsoft-foundry).
</Note>

## Python

[Référentiel GitHub de la bibliothèque Python](https://github.com/anthropics/anthropic-sdk-python)

**Exigences :** Python 3.8+

**Installation :**

```bash
pip install anthropic
```

---

## TypeScript

[Référentiel GitHub de la bibliothèque TypeScript](https://github.com/anthropics/anthropic-sdk-typescript)

<Info>
Bien que cette bibliothèque soit en TypeScript, elle peut également être utilisée dans les bibliothèques JavaScript.
</Info>

**Installation :**

```bash
npm install @anthropic-ai/sdk
```

---

## Java

[Référentiel GitHub de la bibliothèque Java](https://github.com/anthropics/anthropic-sdk-java)

**Exigences :** Java 8 ou version ultérieure

**Installation :**

Gradle :
```groovy
implementation("com.anthropic:anthropic-java:2.10.0")
```

Maven :
```xml
<dependency>
    <groupId>com.anthropic</groupId>
    <artifactId>anthropic-java</artifactId>
    <version>2.10.0</version>
</dependency>
```

---

## Go

[Référentiel GitHub de la bibliothèque Go](https://github.com/anthropics/anthropic-sdk-go)

**Exigences :** Go 1.22+

**Installation :**

```bash
go get -u 'github.com/anthropics/anthropic-sdk-go@v1.17.0'
```

---

## C#

[Référentiel GitHub de la bibliothèque C#](https://github.com/anthropics/anthropic-sdk-csharp)

<Info>
Le SDK C# est actuellement en version bêta.
</Info>

**Exigences :** .NET 8 ou version ultérieure

**Installation :**

```bash
git clone git@github.com:anthropics/anthropic-sdk-csharp.git
dotnet add reference anthropic-sdk-csharp/src/Anthropic.Client
```

---

## Ruby

[Référentiel GitHub de la bibliothèque Ruby](https://github.com/anthropics/anthropic-sdk-ruby)

**Exigences :** Ruby 3.2.0 ou version ultérieure

**Installation :**

Ajoutez à votre Gemfile :
```ruby
gem "anthropic", "~> 1.13.0"
```

Ensuite, exécutez :
```bash
bundle install
```

---

## PHP

[Référentiel GitHub de la bibliothèque PHP](https://github.com/anthropics/anthropic-sdk-php)

<Info>
Le SDK PHP est actuellement en version bêta.
</Info>

**Exigences :** PHP 8.1.0 ou supérieur

**Installation :**

```bash
composer require "anthropic-ai/sdk 0.3.0"
```

---

## Espace de noms bêta dans les SDK clients

Chaque SDK dispose d'un espace de noms `beta` qui est disponible pour accéder aux nouvelles fonctionnalités qu'Anthropic publie dans les versions bêta. Utilisez ceci en conjonction avec les [en-têtes bêta](/docs/fr/api/beta-headers) pour accéder à ces fonctionnalités. Reportez-vous au référentiel GitHub de chaque SDK pour des exemples d'utilisation spécifiques.