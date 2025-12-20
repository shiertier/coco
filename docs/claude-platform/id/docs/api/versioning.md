# Versi

Saat membuat permintaan API, Anda harus mengirim header permintaan `anthropic-version`. Misalnya, `anthropic-version: 2023-06-01`. Jika Anda menggunakan [SDK klien](/docs/id/api/client-sdks) kami, ini ditangani untuk Anda secara otomatis.

---

Untuk versi API tertentu, kami akan mempertahankan:

* Parameter input yang ada
* Parameter output yang ada

Namun, kami dapat melakukan hal berikut:

* Menambahkan input opsional tambahan
* Menambahkan nilai tambahan ke output
* Mengubah kondisi untuk jenis error tertentu
* Menambahkan varian baru ke nilai output seperti enum (misalnya, jenis event streaming)

Secara umum, jika Anda menggunakan API seperti yang didokumentasikan dalam referensi ini, kami tidak akan merusak penggunaan Anda.

## Riwayat versi

Kami selalu merekomendasikan menggunakan versi API terbaru kapan pun memungkinkan. Versi sebelumnya dianggap usang dan mungkin tidak tersedia untuk pengguna baru.

* `2023-06-01`  
   * Format baru untuk [streaming](/docs/id/api/streaming) server-sent events (SSE):  
         * Penyelesaian bersifat incremental. Misalnya, `" Hello"`, `" my"`, `" name"`, `" is"`, `" Claude." ` alih-alih `" Hello"`, `" Hello my"`, `" Hello my name"`, `" Hello my name is"`, `" Hello my name is Claude."`.  
         * Semua event adalah [named events](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#named%5Fevents), bukan [data-only events](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#data-only%5Fmessages).  
         * Menghapus event `data: [DONE]` yang tidak perlu.  
   * Menghapus nilai `exception` dan `truncated` lama dalam respons.
* `2023-01-01`: Rilis awal.