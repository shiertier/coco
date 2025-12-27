# Client SDKs

Kami menyediakan perpustakaan klien dalam sejumlah bahasa populer yang memudahkan untuk bekerja dengan Claude API.

---

Halaman ini mencakup instruksi instalasi singkat dan tautan ke repositori GitHub sumber terbuka untuk Client SDKs Anthropic. Untuk instruksi penggunaan dasar, lihat [referensi API](/docs/id/api/overview). Untuk instruksi penggunaan terperinci, lihat repositori GitHub setiap SDK.

<Note>
Konfigurasi tambahan diperlukan untuk menggunakan Client SDKs Anthropic melalui platform mitra. Jika Anda menggunakan Amazon Bedrock, lihat [panduan ini](/docs/id/build-with-claude/claude-on-amazon-bedrock); jika Anda menggunakan Google Cloud Vertex AI, lihat [panduan ini](/docs/id/build-with-claude/claude-on-vertex-ai); jika Anda menggunakan Microsoft Foundry, lihat [panduan ini](/docs/id/build-with-claude/claude-in-microsoft-foundry).
</Note>

## Python

[Repositori GitHub perpustakaan Python](https://github.com/anthropics/anthropic-sdk-python)

**Persyaratan:** Python 3.8+

**Instalasi:**

```bash
pip install anthropic
```

---

## TypeScript

[Repositori GitHub perpustakaan TypeScript](https://github.com/anthropics/anthropic-sdk-typescript)

<Info>
Meskipun perpustakaan ini dalam TypeScript, perpustakaan ini juga dapat digunakan dalam perpustakaan JavaScript.
</Info>

**Instalasi:**

```bash
npm install @anthropic-ai/sdk
```

---

## Java

[Repositori GitHub perpustakaan Java](https://github.com/anthropics/anthropic-sdk-java)

**Persyaratan:** Java 8 atau lebih baru

**Instalasi:**

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

[Repositori GitHub perpustakaan Go](https://github.com/anthropics/anthropic-sdk-go)

**Persyaratan:** Go 1.22+

**Instalasi:**

```bash
go get -u 'github.com/anthropics/anthropic-sdk-go@v1.17.0'
```

---

## C#

[Repositori GitHub perpustakaan C#](https://github.com/anthropics/anthropic-sdk-csharp)

<Info>
SDK C# saat ini dalam beta.
</Info>

**Persyaratan:** .NET 8 atau lebih baru

**Instalasi:**

```bash
git clone git@github.com:anthropics/anthropic-sdk-csharp.git
dotnet add reference anthropic-sdk-csharp/src/Anthropic.Client
```

---

## Ruby

[Repositori GitHub perpustakaan Ruby](https://github.com/anthropics/anthropic-sdk-ruby)

**Persyaratan:** Ruby 3.2.0 atau lebih baru

**Instalasi:**

Tambahkan ke Gemfile Anda:
```ruby
gem "anthropic", "~> 1.13.0"
```

Kemudian jalankan:
```bash
bundle install
```

---

## PHP

[Repositori GitHub perpustakaan PHP](https://github.com/anthropics/anthropic-sdk-php)

<Info>
SDK PHP saat ini dalam beta.
</Info>

**Persyaratan:** PHP 8.1.0 atau lebih tinggi

**Instalasi:**

```bash
composer require "anthropic-ai/sdk 0.3.0"
```

---

## Namespace beta dalam client SDKs

Setiap SDK memiliki namespace `beta` yang tersedia untuk mengakses fitur baru yang dirilis Anthropic dalam versi beta. Gunakan ini bersama dengan [header beta](/docs/id/api/beta-headers) untuk mengakses fitur-fitur ini. Lihat repositori GitHub setiap SDK untuk contoh penggunaan spesifik.