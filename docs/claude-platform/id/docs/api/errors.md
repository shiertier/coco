# Kesalahan

---

## Kesalahan HTTP

API kami mengikuti format kode kesalahan HTTP yang dapat diprediksi:

* 400 - `invalid_request_error`: Ada masalah dengan format atau konten permintaan Anda. Kami juga dapat menggunakan jenis kesalahan ini untuk kode status 4XX lainnya yang tidak tercantum di bawah ini.
* 401 - `authentication_error`: Ada masalah dengan kunci API Anda.
* 403 - `permission_error`: Kunci API Anda tidak memiliki izin untuk menggunakan sumber daya yang ditentukan.
* 404 - `not_found_error`: Sumber daya yang diminta tidak ditemukan.
* 413 - `request_too_large`: Permintaan melebihi jumlah byte maksimum yang diizinkan. Ukuran permintaan maksimum adalah 32 MB untuk endpoint API standar.
* 429 - `rate_limit_error`: Akun Anda telah mencapai batas laju.
* 500 - `api_error`: Terjadi kesalahan tak terduga di dalam sistem Anthropic.
* 529 - `overloaded_error`: API sementara kelebihan beban.

  <Warning>
  Kesalahan 529 dapat terjadi ketika API mengalami lalu lintas tinggi di semua pengguna.
  
  Dalam kasus yang jarang terjadi, jika organisasi Anda mengalami peningkatan tajam dalam penggunaan, Anda mungkin melihat kesalahan 429 karena batas akselerasi pada API. Untuk menghindari mencapai batas akselerasi, tingkatkan lalu lintas Anda secara bertahap dan pertahankan pola penggunaan yang konsisten.
  </Warning>

Ketika menerima respons [streaming](/docs/id/build-with-claude/streaming) melalui SSE, ada kemungkinan kesalahan dapat terjadi setelah mengembalikan respons 200, dalam hal ini penanganan kesalahan tidak akan mengikuti mekanisme standar ini.

## Batas ukuran permintaan

API memberlakukan batas ukuran permintaan untuk memastikan kinerja optimal:

| Jenis Endpoint | Ukuran Permintaan Maksimum |
|:---|:---|
| Messages API | 32 MB |
| Token Counting API | 32 MB |
| [Batch API](/docs/id/build-with-claude/batch-processing) | 256 MB |
| [Files API](/docs/id/build-with-claude/files) | 500 MB |

Jika Anda melebihi batas ini, Anda akan menerima kesalahan 413 `request_too_large`. Kesalahan dikembalikan dari Cloudflare sebelum permintaan mencapai server API kami.

## Bentuk kesalahan

Kesalahan selalu dikembalikan sebagai JSON, dengan objek `error` tingkat atas yang selalu menyertakan nilai `type` dan `message`. Respons juga menyertakan bidang `request_id` untuk pelacakan dan debugging yang lebih mudah. Misalnya:

```json JSON
{
  "type": "error",
  "error": {
    "type": "not_found_error",
    "message": "The requested resource could not be found."
  },
  "request_id": "req_011CSHoEeqs5C35K2UUqR7Fy"
}
```

Sesuai dengan kebijakan [versioning](/docs/id/api/versioning) kami, kami dapat memperluas nilai dalam objek ini, dan ada kemungkinan bahwa nilai `type` akan bertambah seiring waktu.

## Request id

Setiap respons API menyertakan header `request-id` yang unik. Header ini berisi nilai seperti `req_018EeWyXxfu5pfWkrYcMdjWG`. Ketika menghubungi dukungan tentang permintaan tertentu, harap sertakan ID ini untuk membantu kami menyelesaikan masalah Anda dengan cepat.

SDK resmi kami menyediakan nilai ini sebagai properti pada objek respons tingkat atas, yang berisi nilai dari header `request-id`:

<CodeGroup>
  ```python Python
  import anthropic

  client = anthropic.Anthropic()

  message = client.messages.create(
      model="claude-sonnet-4-5",
      max_tokens=1024,
      messages=[
          {"role": "user", "content": "Hello, Claude"}
      ]
  )
  print(f"Request ID: {message._request_id}")
  ```

  ```typescript TypeScript
  import Anthropic from '@anthropic-ai/sdk';

  const client = new Anthropic();

  const message = await client.messages.create({
    model: 'claude-sonnet-4-5',
    max_tokens: 1024,
    messages: [
      {"role": "user", "content": "Hello, Claude"}
    ]
  });
  console.log('Request ID:', message._request_id);
  ```
</CodeGroup>

## Permintaan panjang

<Warning>
 Kami sangat mendorong penggunaan [streaming Messages API](/docs/id/build-with-claude/streaming) atau [Message Batches API](/docs/id/api/creating-message-batches) untuk permintaan yang berjalan lama, terutama yang lebih dari 10 menit.
</Warning>

Kami tidak merekomendasikan menetapkan nilai `max_tokens` yang besar tanpa menggunakan [streaming Messages API](/docs/id/build-with-claude/streaming) atau [Message Batches API](/docs/id/api/creating-message-batches) kami:

- Beberapa jaringan mungkin memutuskan koneksi yang menganggur setelah periode waktu yang bervariasi, yang dapat menyebabkan permintaan gagal atau timeout tanpa menerima respons dari Anthropic.
- Jaringan berbeda dalam keandalan; [Message Batches API](/docs/id/api/creating-message-batches) kami dapat membantu Anda mengelola risiko masalah jaringan dengan memungkinkan Anda melakukan polling untuk hasil daripada memerlukan koneksi jaringan yang tidak terputus.

Jika Anda membangun integrasi API langsung, Anda harus menyadari bahwa menetapkan [TCP socket keep-alive](https://tldp.org/HOWTO/TCP-Keepalive-HOWTO/programming.html) dapat mengurangi dampak timeout koneksi yang menganggur pada beberapa jaringan.

[SDK](/docs/id/api/client-sdks) kami akan memvalidasi bahwa permintaan Messages API non-streaming Anda tidak diharapkan melebihi timeout 10 menit dan juga akan menetapkan opsi socket untuk TCP keep-alive.