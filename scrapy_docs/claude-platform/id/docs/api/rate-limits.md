# Batas laju

Untuk mengurangi penyalahgunaan dan mengelola kapasitas di API kami, kami telah menerapkan batas tentang seberapa banyak organisasi dapat menggunakan Claude API.

---

Kami memiliki dua jenis batas:

1. **Batas pengeluaran** menetapkan biaya bulanan maksimum yang dapat dikeluarkan organisasi untuk penggunaan API.
2. **Batas laju** menetapkan jumlah maksimum permintaan API yang dapat dibuat organisasi selama periode waktu yang ditentukan.

Kami memberlakukan batas yang dikonfigurasi layanan di tingkat organisasi, tetapi Anda juga dapat menetapkan batas yang dapat dikonfigurasi pengguna untuk ruang kerja organisasi Anda.

Batas ini berlaku untuk penggunaan Tier Standar dan Prioritas. Untuk informasi lebih lanjut tentang Tier Prioritas, yang menawarkan tingkat layanan yang ditingkatkan sebagai imbalan komitmen pengeluaran, lihat [Service Tiers](/docs/id/api/service-tiers).

## Tentang batas kami

* Batas dirancang untuk mencegah penyalahgunaan API, sambil meminimalkan dampak pada pola penggunaan pelanggan yang umum.
* Batas didefinisikan oleh **tingkat penggunaan**, di mana setiap tingkat dikaitkan dengan set batas pengeluaran dan laju yang berbeda.
* Organisasi Anda akan meningkat tingkat secara otomatis saat Anda mencapai ambang batas tertentu saat menggunakan API.
  Batas ditetapkan di tingkat organisasi. Anda dapat melihat batas organisasi Anda di [halaman Batas](/settings/limits) di [Claude Console](/).
* Anda mungkin mencapai batas laju selama interval waktu yang lebih pendek. Misalnya, laju 60 permintaan per menit (RPM) dapat diberlakukan sebagai 1 permintaan per detik. Lonjakan permintaan singkat dengan volume tinggi dapat melampaui batas laju dan menghasilkan kesalahan batas laju.
* Batas yang dijelaskan di bawah adalah batas tingkat standar kami. Jika Anda mencari batas yang lebih tinggi dan khusus atau Tier Prioritas untuk tingkat layanan yang ditingkatkan, hubungi penjualan melalui [Claude Console](/settings/limits).
* Kami menggunakan [algoritma token bucket](https://en.wikipedia.org/wiki/Token_bucket) untuk melakukan pembatasan laju. Ini berarti kapasitas Anda terus diisi ulang hingga batas maksimum Anda, bukan direset pada interval tetap.
* Semua batas yang dijelaskan di sini mewakili penggunaan maksimum yang diizinkan, bukan minimum yang dijamin. Batas ini dimaksudkan untuk mengurangi pengeluaran yang tidak disengaja dan memastikan distribusi sumber daya yang adil di antara pengguna.

## Batas pengeluaran

Setiap tingkat penggunaan memiliki batas tentang berapa banyak yang dapat Anda keluarkan di API setiap bulan kalender. Setelah Anda mencapai batas pengeluaran tingkat Anda, sampai Anda memenuhi syarat untuk tingkat berikutnya, Anda harus menunggu sampai bulan berikutnya untuk dapat menggunakan API lagi.

Untuk memenuhi syarat tingkat berikutnya, Anda harus memenuhi persyaratan setoran. Untuk meminimalkan risiko pendanaan berlebih akun Anda, Anda tidak dapat menyetor lebih dari batas pengeluaran bulanan Anda.

### Persyaratan untuk meningkatkan tingkat
<table>
  <thead>
    <tr>
      <th>Tingkat Penggunaan</th>
      <th>Pembelian Kredit</th>
      <th>Pembelian Kredit Maksimum</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Tier 1</td>
      <td>\$5</td>
      <td>\$100</td>
    </tr>
    <tr>
      <td>Tier 2</td>
      <td>\$40</td>
      <td>\$500</td>
    </tr>
    <tr>
      <td>Tier 3</td>
      <td>\$200</td>
      <td>\$1,000</td>
    </tr>
    <tr>
      <td>Tier 4</td>
      <td>\$400</td>
      <td>\$5,000</td>
    </tr>
    <tr>
      <td>Penagihan Bulanan</td>
      <td>N/A</td>
      <td>N/A</td>
    </tr>
  </tbody>
</table>

<Note>
**Pembelian Kredit** menunjukkan pembelian kredit kumulatif (tidak termasuk pajak) yang diperlukan untuk meningkat ke tingkat itu. Anda meningkat segera setelah mencapai ambang batas.

**Pembelian Kredit Maksimum** membatasi jumlah maksimum yang dapat Anda tambahkan ke akun Anda dalam satu transaksi untuk mencegah pendanaan berlebih akun.
</Note>

## Batas laju

Batas laju kami untuk Messages API diukur dalam permintaan per menit (RPM), token input per menit (ITPM), dan token output per menit (OTPM) untuk setiap kelas model.
Jika Anda melampaui salah satu batas laju, Anda akan mendapatkan [kesalahan 429](/docs/id/api/errors) yang menjelaskan batas laju mana yang terlampaui, bersama dengan header `retry-after` yang menunjukkan berapa lama waktu tunggu.

<Note>
Anda mungkin juga mengalami kesalahan 429 karena batas akselerasi di API jika organisasi Anda mengalami peningkatan penggunaan yang tajam. Untuk menghindari mencapai batas akselerasi, tingkatkan lalu lintas Anda secara bertahap dan pertahankan pola penggunaan yang konsisten.
</Note>

### ITPM yang menyadari cache

Banyak penyedia API menggunakan batas "token per menit" (TPM) gabungan yang mungkin mencakup semua token, baik yang di-cache maupun tidak, input dan output. **Untuk sebagian besar model Claude, hanya token input yang tidak di-cache yang dihitung terhadap batas laju ITPM Anda.** Ini adalah keuntungan utama yang membuat batas laju kami secara efektif lebih tinggi daripada yang mungkin terlihat pada awalnya.

Batas laju ITPM diperkirakan di awal setiap permintaan, dan perkiraan disesuaikan selama permintaan untuk mencerminkan jumlah sebenarnya dari token input yang digunakan.

Berikut adalah apa yang dihitung terhadap ITPM:
- `input_tokens` (token setelah titik henti cache terakhir) ✓ **Dihitung terhadap ITPM**
- `cache_creation_input_tokens` (token yang ditulis ke cache) ✓ **Dihitung terhadap ITPM**
- `cache_read_input_tokens` (token yang dibaca dari cache) ✗ **TIDAK dihitung terhadap ITPM** untuk sebagian besar model

<Note>
Bidang `input_tokens` hanya mewakili token yang muncul **setelah titik henti cache terakhir Anda**, bukan semua token input dalam permintaan Anda. Untuk menghitung total token input:

```
total_input_tokens = cache_read_input_tokens + cache_creation_input_tokens + input_tokens
```

Ini berarti ketika Anda memiliki konten yang di-cache, `input_tokens` biasanya akan jauh lebih kecil daripada total input Anda. Misalnya, dengan dokumen yang di-cache 200K token dan pertanyaan pengguna 50 token, Anda akan melihat `input_tokens: 50` meskipun total input adalah 200.050 token.

Untuk tujuan batas laju pada sebagian besar model, hanya `input_tokens` + `cache_creation_input_tokens` yang dihitung terhadap batas ITPM Anda, membuat [prompt caching](/docs/id/build-with-claude/prompt-caching) cara yang efektif untuk meningkatkan throughput efektif Anda.
</Note>

**Contoh**: Dengan batas ITPM 2.000.000 dan tingkat cache hit 80%, Anda dapat secara efektif memproses 10.000.000 total token input per menit (2M tidak di-cache + 8M di-cache), karena token yang di-cache tidak dihitung terhadap batas laju Anda.

<Note>
Beberapa model yang lebih lama (ditandai dengan † dalam tabel batas laju di bawah) juga menghitung `cache_read_input_tokens` terhadap batas laju ITPM.

Untuk semua model tanpa penanda †, token input yang di-cache tidak dihitung terhadap batas laju dan ditagih dengan tarif yang dikurangi (10% dari harga token input dasar). Ini berarti Anda dapat mencapai throughput efektif yang secara signifikan lebih tinggi dengan menggunakan [prompt caching](/docs/id/build-with-claude/prompt-caching).
</Note>

<Tip>
**Maksimalkan batas laju Anda dengan prompt caching**

Untuk mendapatkan hasil maksimal dari batas laju Anda, gunakan [prompt caching](/docs/id/build-with-claude/prompt-caching) untuk konten berulang seperti:
- Instruksi sistem dan prompt
- Dokumen konteks besar
- Definisi alat
- Riwayat percakapan

Dengan caching yang efektif, Anda dapat secara dramatis meningkatkan throughput aktual Anda tanpa meningkatkan batas laju Anda. Pantau tingkat cache hit Anda di [halaman Penggunaan](/settings/usage) untuk mengoptimalkan strategi caching Anda.
</Tip>

Batas laju OTPM diperkirakan berdasarkan `max_tokens` di awal setiap permintaan, dan perkiraan disesuaikan di akhir permintaan untuk mencerminkan jumlah sebenarnya dari token output yang digunakan.
Jika Anda mencapai batas OTPM lebih awal dari yang diharapkan, coba kurangi `max_tokens` untuk lebih baik memperkirakan ukuran penyelesaian Anda.

Batas laju diterapkan secara terpisah untuk setiap model; oleh karena itu Anda dapat menggunakan model berbeda hingga batas masing-masing secara bersamaan.
Anda dapat memeriksa batas laju saat ini dan perilaku Anda di [Claude Console](/settings/limits).

<Note>
Untuk permintaan konteks panjang (>200K token) saat menggunakan header beta `context-1m-2025-08-07` dengan Claude Sonnet 4.x, batas laju terpisah berlaku. Lihat [Batas laju konteks panjang](#long-context-rate-limits) di bawah.
</Note>

<Tabs>
<Tab title="Tier 1">
| Model                                                                                        | Permintaan maksimum per menit (RPM) | Token input maksimum per menit (ITPM) | Token output maksimum per menit (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------- | -------------------------------------- | --------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 50                                | 30.000                                 | 8.000                                   |
| Claude Sonnet 3.7 ([deprecated](/docs/id/about-claude/model-deprecations))                   | 50                                | 20.000                                 | 8.000                                   |
| Claude Haiku 4.5                                                                             | 50                                | 50.000                                 | 10.000                                  |
| Claude Haiku 3.5 ([deprecated](/docs/id/about-claude/model-deprecations))                    | 50                                | 50.000<sup>†</sup>                     | 10.000                                  |
| Claude Haiku 3                                                                               | 50                                | 50.000<sup>†</sup>                     | 10.000                                  |
| Claude Opus 4.x<sup>*</sup>                                                                  | 50                                | 30.000                                 | 8.000                                   |
| Claude Opus 3 ([deprecated](/docs/id/about-claude/model-deprecations))                      | 50                                | 20.000<sup>†</sup>                     | 4.000                                   |

</Tab>
<Tab title="Tier 2">
| Model                                                                                        | Permintaan maksimum per menit (RPM) | Token input maksimum per menit (ITPM) | Token output maksimum per menit (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------- | -------------------------------------- | --------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 1.000                             | 450.000                                | 90.000                                  |
| Claude Sonnet 3.7 ([deprecated](/docs/id/about-claude/model-deprecations))                   | 1.000                             | 40.000                                 | 16.000                                  |
| Claude Haiku 4.5                                                                             | 1.000                             | 450.000                                | 90.000                                  |
| Claude Haiku 3.5 ([deprecated](/docs/id/about-claude/model-deprecations))                    | 1.000                             | 100.000<sup>†</sup>                    | 20.000                                  |
| Claude Haiku 3                                                                               | 1.000                             | 100.000<sup>†</sup>                    | 20.000                                  |
| Claude Opus 4.x<sup>*</sup>                                                                  | 1.000                             | 450.000                                | 90.000                                  |
| Claude Opus 3 ([deprecated](/docs/id/about-claude/model-deprecations))                      | 1.000                             | 40.000<sup>†</sup>                     | 8.000                                   |

</Tab>
<Tab title="Tier 3">
| Model                                                                                        | Permintaan maksimum per menit (RPM) | Token input maksimum per menit (ITPM) | Token output maksimum per menit (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------- | -------------------------------------- | --------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 2.000                             | 800.000                                | 160.000                                 |
| Claude Sonnet 3.7 ([deprecated](/docs/id/about-claude/model-deprecations))                   | 2.000                             | 80.000                                 | 32.000                                  |
| Claude Haiku 4.5                                                                             | 2.000                             | 1.000.000                              | 200.000                                 |
| Claude Haiku 3.5 ([deprecated](/docs/id/about-claude/model-deprecations))                    | 2.000                             | 200.000<sup>†</sup>                    | 40.000                                  |
| Claude Haiku 3                                                                               | 2.000                             | 200.000<sup>†</sup>                    | 40.000                                  |
| Claude Opus 4.x<sup>*</sup>                                                                  | 2.000                             | 800.000                                | 160.000                                 |
| Claude Opus 3 ([deprecated](/docs/id/about-claude/model-deprecations))                      | 2.000                             | 80.000<sup>†</sup>                     | 16.000                                  |

</Tab>
<Tab title="Tier 4">
| Model                                                                                        | Permintaan maksimum per menit (RPM) | Token input maksimum per menit (ITPM) | Token output maksimum per menit (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------- | -------------------------------------- | --------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 4.000                             | 2.000.000                              | 400.000                                 |
| Claude Sonnet 3.7 ([deprecated](/docs/id/about-claude/model-deprecations))                   | 4.000                             | 200.000                                | 80.000                                  |
| Claude Haiku 4.5                                                                             | 4.000                             | 4.000.000                              | 800.000                                 |
| Claude Haiku 3.5 ([deprecated](/docs/id/about-claude/model-deprecations))                    | 4.000                             | 400.000<sup>†</sup>                    | 80.000                                  |
| Claude Haiku 3                                                                               | 4.000                             | 400.000<sup>†</sup>                    | 80.000                                  |
| Claude Opus 4.x<sup>*</sup>                                                                  | 4.000                             | 2.000.000                              | 400.000                                 |
| Claude Opus 3 ([deprecated](/docs/id/about-claude/model-deprecations))                      | 4.000                             | 400.000<sup>†</sup>                    | 80.000                                  |

</Tab>
<Tab title="Custom">
Jika Anda mencari batas yang lebih tinggi untuk kasus penggunaan Enterprise, hubungi penjualan melalui [Claude Console](/settings/limits).
</Tab>
</Tabs>

_<sup>* - Batas laju Opus 4.x adalah batas total yang berlaku untuk lalu lintas gabungan di seluruh Opus 4, Opus 4.1, dan Opus 4.5.</sup>_

_<sup>** - Batas laju Sonnet 4.x adalah batas total yang berlaku untuk lalu lintas gabungan di seluruh Sonnet 4 dan Sonnet 4.5.</sup>_

_<sup>† - Batas menghitung `cache_read_input_tokens` terhadap penggunaan ITPM.</sup>_

### Message Batches API

Message Batches API memiliki set batas laju sendiri yang dibagikan di semua model. Ini termasuk batas permintaan per menit (RPM) ke semua endpoint API dan batas jumlah permintaan batch yang dapat berada dalam antrian pemrosesan pada waktu yang sama. "Permintaan batch" di sini mengacu pada bagian dari Message Batch. Anda dapat membuat Message Batch yang berisi ribuan permintaan batch, masing-masing dihitung terhadap batas ini. Permintaan batch dianggap bagian dari antrian pemrosesan ketika belum berhasil diproses oleh model.

<Tabs>
<Tab title="Tier 1">
| Permintaan maksimum per menit (RPM) | Permintaan batch maksimum dalam antrian pemrosesan | Permintaan batch maksimum per batch |
| --------------------------------- | ------------------------------------------ | -------------------------------- |
| 50                                | 100.000                                    | 100.000                          |
</Tab>
<Tab title="Tier 2">
| Permintaan maksimum per menit (RPM) | Permintaan batch maksimum dalam antrian pemrosesan | Permintaan batch maksimum per batch |
| --------------------------------- | ------------------------------------------ | -------------------------------- |
| 1.000                             | 200.000                                    | 100.000                          |
</Tab>
<Tab title="Tier 3">
| Permintaan maksimum per menit (RPM) | Permintaan batch maksimum dalam antrian pemrosesan | Permintaan batch maksimum per batch |
| --------------------------------- | ------------------------------------------ | -------------------------------- |
| 2.000                             | 300.000                                    | 100.000                          |
</Tab>
<Tab title="Tier 4">
| Permintaan maksimum per menit (RPM) | Permintaan batch maksimum dalam antrian pemrosesan | Permintaan batch maksimum per batch |
| --------------------------------- | ------------------------------------------ | -------------------------------- |
| 4.000                             | 500.000                                    | 100.000                          |
</Tab>
<Tab title="Custom">
Jika Anda mencari batas yang lebih tinggi untuk kasus penggunaan Enterprise, hubungi penjualan melalui [Claude Console](/settings/limits).
</Tab>
</Tabs>

### Batas laju konteks panjang

Saat menggunakan Claude Sonnet 4 dan Sonnet 4.5 dengan [jendela konteks token 1M diaktifkan](/docs/id/build-with-claude/context-windows#1m-token-context-window), batas laju khusus berikut berlaku untuk permintaan yang melebihi 200K token.

<Note>
Jendela konteks token 1M saat ini dalam beta untuk organisasi di tingkat penggunaan 4 dan organisasi dengan batas laju khusus. Jendela konteks token 1M hanya tersedia untuk Claude Sonnet 4 dan Sonnet 4.5.
</Note>

<Tabs>
<Tab title="Tier 4">
| Token input maksimum per menit (ITPM) | Token output maksimum per menit (OTPM) |
| -------------------------------------- | --------------------------------------- |
| 1.000.000                              | 200.000                                 |
</Tab>
<Tab title="Custom">
Untuk batas laju konteks panjang khusus untuk kasus penggunaan enterprise, hubungi penjualan melalui [Claude Console](/settings/limits).
</Tab>
</Tabs>

<Tip>
Untuk mendapatkan hasil maksimal dari jendela konteks token 1M dengan batas laju, gunakan [prompt caching](/docs/id/build-with-claude/prompt-caching).
</Tip>

### Memantau batas laju Anda di Console

Anda dapat memantau penggunaan batas laju Anda di halaman [Penggunaan](/settings/usage) dari [Claude Console](/). 

Selain menyediakan bagan token dan permintaan, halaman Penggunaan menyediakan dua bagan batas laju terpisah. Gunakan bagan ini untuk melihat berapa banyak ruang yang Anda miliki untuk berkembang, kapan Anda mungkin mencapai penggunaan puncak, lebih baik memahami batas laju apa yang akan diminta, atau bagaimana Anda dapat meningkatkan tingkat caching Anda. Bagan memvisualisasikan sejumlah metrik untuk batas laju tertentu (misalnya per model):

- Bagan **Batas Laju - Token Input** mencakup:
  - Token input per menit maksimum per jam yang tidak di-cache
  - Batas laju token input per menit saat ini Anda
  - Tingkat cache untuk token input Anda (yaitu persentase token input yang dibaca dari cache)
- Bagan **Batas Laju - Token Output** mencakup:
  - Token output per menit maksimum per jam
  - Batas laju token output per menit saat ini Anda

## Menetapkan batas yang lebih rendah untuk Ruang Kerja

Untuk melindungi Ruang Kerja di Organisasi Anda dari potensi penggunaan berlebih, Anda dapat menetapkan batas pengeluaran dan laju khusus per Ruang Kerja.

Contoh: Jika batas Organisasi Anda adalah 40.000 token input per menit dan 8.000 token output per menit, Anda mungkin membatasi satu Ruang Kerja hingga 30.000 total token per menit. Ini melindungi Ruang Kerja lain dari potensi penggunaan berlebih dan memastikan distribusi sumber daya yang lebih adil di seluruh Organisasi Anda. Token per menit yang tidak digunakan yang tersisa (atau lebih, jika Ruang Kerja itu tidak menggunakan batas) kemudian tersedia untuk Ruang Kerja lain gunakan.

Catatan:
- Anda tidak dapat menetapkan batas pada Ruang Kerja default.
- Jika tidak ditetapkan, batas Ruang Kerja cocok dengan batas Organisasi.
- Batas di seluruh Organisasi selalu berlaku, bahkan jika batas Ruang Kerja ditambahkan hingga lebih banyak.
- Dukungan untuk batas token input dan output akan ditambahkan ke Ruang Kerja di masa depan.

## Header respons

Respons API mencakup header yang menunjukkan batas laju yang diberlakukan, penggunaan saat ini, dan kapan batas akan direset.

Header berikut dikembalikan:

| Header                                        | Deskripsi                                                                                                                                     |
| --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `retry-after`                                 | Jumlah detik untuk menunggu sampai Anda dapat mencoba ulang permintaan. Percobaan ulang lebih awal akan gagal.                                                      |
| `anthropic-ratelimit-requests-limit`          | Jumlah maksimum permintaan yang diizinkan dalam periode batas laju apa pun.                                                                            |
| `anthropic-ratelimit-requests-remaining`      | Jumlah permintaan yang tersisa sebelum dibatasi laju.                                                                                     |
| `anthropic-ratelimit-requests-reset`          | Waktu ketika batas laju permintaan akan sepenuhnya diisi ulang, disediakan dalam format RFC 3339.                                                    |
| `anthropic-ratelimit-tokens-limit`            | Jumlah maksimum token yang diizinkan dalam periode batas laju apa pun.                                                                              |
| `anthropic-ratelimit-tokens-remaining`        | Jumlah token yang tersisa (dibulatkan ke seribu terdekat) sebelum dibatasi laju.                                                     |
| `anthropic-ratelimit-tokens-reset`            | Waktu ketika batas laju token akan sepenuhnya diisi ulang, disediakan dalam format RFC 3339.                                                      |
| `anthropic-ratelimit-input-tokens-limit`      | Jumlah maksimum token input yang diizinkan dalam periode batas laju apa pun.                                                                        |
| `anthropic-ratelimit-input-tokens-remaining`  | Jumlah token input yang tersisa (dibulatkan ke seribu terdekat) sebelum dibatasi laju.                                               |
| `anthropic-ratelimit-input-tokens-reset`      | Waktu ketika batas laju token input akan sepenuhnya diisi ulang, disediakan dalam format RFC 3339.                                                |
| `anthropic-ratelimit-output-tokens-limit`     | Jumlah maksimum token output yang diizinkan dalam periode batas laju apa pun.                                                                       |
| `anthropic-ratelimit-output-tokens-remaining` | Jumlah token output yang tersisa (dibulatkan ke seribu terdekat) sebelum dibatasi laju.                                              |
| `anthropic-ratelimit-output-tokens-reset`     | Waktu ketika batas laju token output akan sepenuhnya diisi ulang, disediakan dalam format RFC 3339.                                               |
| `anthropic-priority-input-tokens-limit`       | Jumlah maksimum token input Tier Prioritas yang diizinkan dalam periode batas laju apa pun. (Hanya Tier Prioritas)                                     |
| `anthropic-priority-input-tokens-remaining`   | Jumlah token input Tier Prioritas yang tersisa (dibulatkan ke seribu terdekat) sebelum dibatasi laju. (Hanya Tier Prioritas)            |
| `anthropic-priority-input-tokens-reset`       | Waktu ketika batas laju token input Tier Prioritas akan sepenuhnya diisi ulang, disediakan dalam format RFC 3339. (Hanya Tier Prioritas)             |
| `anthropic-priority-output-tokens-limit`      | Jumlah maksimum token output Tier Prioritas yang diizinkan dalam periode batas laju apa pun. (Hanya Tier Prioritas)                                    |
| `anthropic-priority-output-tokens-remaining`  | Jumlah token output Tier Prioritas yang tersisa (dibulatkan ke seribu terdekat) sebelum dibatasi laju. (Hanya Tier Prioritas)           |
| `anthropic-priority-output-tokens-reset`      | Waktu ketika batas laju token output Tier Prioritas akan sepenuhnya diisi ulang, disediakan dalam format RFC 3339. (Hanya Tier Prioritas)            |

Header `anthropic-ratelimit-tokens-*` menampilkan nilai untuk batas yang paling ketat saat ini berlaku. Misalnya, jika Anda telah melampaui batas token per menit Ruang Kerja, header akan berisi nilai batas laju token per menit Ruang Kerja. Jika batas Ruang Kerja tidak berlaku, header akan mengembalikan total token yang tersisa, di mana total adalah jumlah token input dan output. Pendekatan ini memastikan bahwa Anda memiliki visibilitas ke dalam kendala yang paling relevan pada penggunaan API saat ini Anda.