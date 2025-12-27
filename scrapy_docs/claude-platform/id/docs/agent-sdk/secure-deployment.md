# Mengamankan penyebaran agen AI

Panduan untuk mengamankan penyebaran Claude Code dan Agent SDK dengan isolasi, manajemen kredensial, dan kontrol jaringan

---

Claude Code dan Agent SDK adalah alat yang ampuh yang dapat menjalankan kode, mengakses file, dan berinteraksi dengan layanan eksternal atas nama Anda. Seperti alat apa pun dengan kemampuan ini, penyebaran yang cermat memastikan Anda mendapatkan manfaat sambil mempertahankan kontrol yang sesuai.

Tidak seperti perangkat lunak tradisional yang mengikuti jalur kode yang telah ditentukan, alat ini menghasilkan tindakan mereka secara dinamis berdasarkan konteks dan tujuan. Fleksibilitas inilah yang membuat mereka berguna, tetapi ini juga berarti perilaku mereka dapat dipengaruhi oleh konten yang mereka proses: file, halaman web, atau masukan pengguna. Ini kadang-kadang disebut injeksi prompt. Misalnya, jika README repositori berisi instruksi yang tidak biasa, Claude Code mungkin menggabungkan instruksi tersebut ke dalam tindakannya dengan cara yang tidak diantisipasi oleh operator. Panduan ini mencakup cara praktis untuk mengurangi risiko ini.

Kabar baiknya adalah mengamankan penyebaran agen tidak memerlukan infrastruktur yang eksotis. Prinsip yang sama yang berlaku untuk menjalankan kode semi-terpercaya apa pun berlaku di sini: isolasi, hak istimewa minimal, dan pertahanan berlapis. Claude Code mencakup beberapa fitur keamanan yang membantu dengan masalah umum, dan panduan ini memandu melalui ini bersama dengan opsi pengerasan tambahan bagi mereka yang membutuhkannya.

Tidak setiap penyebaran memerlukan keamanan maksimal. Pengembang yang menjalankan Claude Code di laptop mereka memiliki persyaratan yang berbeda dari perusahaan yang memproses data pelanggan di lingkungan multi-penyewa. Panduan ini menyajikan opsi mulai dari fitur keamanan bawaan Claude Code hingga arsitektur produksi yang dikeraskan, sehingga Anda dapat memilih apa yang sesuai dengan situasi Anda.

## Apa yang kami lindungi?

Agen dapat mengambil tindakan yang tidak diinginkan karena injeksi prompt (instruksi yang tertanam dalam konten yang mereka proses) atau kesalahan model. Model Claude dirancang untuk menahan ini, dan seperti yang kami analisis dalam [kartu model](https://assets.anthropic.com/m/64823ba7485345a7/Claude-Opus-4-5-System-Card.pdf) kami, kami percaya Claude Opus 4.5 adalah model perbatasan paling kuat yang tersedia.

Pertahanan berlapis masih merupakan praktik yang baik. Misalnya, jika agen memproses file berbahaya yang menginstruksikannya untuk mengirim data pelanggan ke server eksternal, kontrol jaringan dapat memblokir permintaan itu sepenuhnya.

## Fitur keamanan bawaan

Claude Code mencakup beberapa fitur keamanan yang mengatasi masalah umum. Lihat [dokumentasi keamanan](https://code.claude.com/docs/id/security) untuk detail lengkap.

- **Sistem izin**: Setiap alat dan perintah bash dapat dikonfigurasi untuk mengizinkan, memblokir, atau meminta persetujuan pengguna. Gunakan pola glob untuk membuat aturan seperti "izinkan semua perintah npm" atau "blokir perintah apa pun dengan sudo". Organisasi dapat menetapkan kebijakan yang berlaku di semua pengguna. Lihat [kontrol akses dan izin](https://code.claude.com/docs/id/iam#access-control-and-permissions).
- **Analisis statis**: Sebelum menjalankan perintah bash, Claude Code menjalankan analisis statis untuk mengidentifikasi operasi yang berpotensi berisiko. Perintah yang memodifikasi file sistem atau mengakses direktori sensitif ditandai dan memerlukan persetujuan pengguna eksplisit.
- **Ringkasan pencarian web**: Hasil pencarian diringkas daripada melewatkan konten mentah langsung ke dalam konteks, mengurangi risiko injeksi prompt dari konten web berbahaya.
- **Mode sandbox**: Perintah bash dapat berjalan di lingkungan sandbox yang membatasi akses sistem file dan jaringan. Lihat [dokumentasi sandboxing](https://code.claude.com/docs/id/sandboxing) untuk detail.

## Prinsip keamanan

Untuk penyebaran yang memerlukan pengerasan tambahan di luar default Claude Code, prinsip-prinsip ini memandu opsi yang tersedia.

### Batas keamanan

Batas keamanan memisahkan komponen dengan tingkat kepercayaan yang berbeda. Untuk penyebaran keamanan tinggi, Anda dapat menempatkan sumber daya sensitif (seperti kredensial) di luar batas yang berisi agen. Jika ada yang salah di lingkungan agen, sumber daya di luar batas itu tetap terlindungi.

Misalnya, daripada memberi agen akses langsung ke kunci API, Anda dapat menjalankan proxy di luar lingkungan agen yang menyuntikkan kunci ke dalam permintaan. Agen dapat membuat panggilan API, tetapi tidak pernah melihat kredensial itu sendiri. Pola ini berguna untuk penyebaran multi-penyewa atau saat memproses konten yang tidak dipercaya.

### Hak istimewa minimal

Jika diperlukan, Anda dapat membatasi agen hanya ke kemampuan yang diperlukan untuk tugas spesifiknya:

| Sumber Daya | Opsi Pembatasan |
|----------|---------------------|
| Sistem file | Pasang hanya direktori yang diperlukan, lebih suka baca-saja |
| Jaringan | Batasi ke titik akhir tertentu melalui proxy |
| Kredensial | Suntikkan melalui proxy daripada mengekspos secara langsung |
| Kemampuan sistem | Lepaskan kemampuan Linux di kontainer |

### Pertahanan berlapis

Untuk lingkungan keamanan tinggi, menggabungkan beberapa kontrol memberikan perlindungan tambahan. Opsi termasuk:

- Isolasi kontainer
- Pembatasan jaringan
- Kontrol sistem file
- Validasi permintaan di proxy

Kombinasi yang tepat tergantung pada model ancaman dan persyaratan operasional Anda.

## Teknologi isolasi

Teknologi isolasi yang berbeda menawarkan pertukaran yang berbeda antara kekuatan keamanan, kinerja, dan kompleksitas operasional.

<Info>
Dalam semua konfigurasi ini, Claude Code (atau aplikasi Agent SDK Anda) berjalan di dalam batas isolasi—sandbox, kontainer, atau VM. Kontrol keamanan yang dijelaskan di bawah membatasi apa yang dapat diakses agen dari dalam batas itu.
</Info>

| Teknologi | Kekuatan Isolasi | Overhead Kinerja | Kompleksitas |
|------------|-------------------|---------------------|------------|
| Runtime sandbox | Baik (default aman) | Sangat rendah | Rendah |
| Kontainer (Docker) | Tergantung setup | Rendah | Sedang |
| gVisor | Sangat baik (dengan setup yang benar) | Sedang/Tinggi | Sedang |
| VM (Firecracker, QEMU) | Sangat baik (dengan setup yang benar) | Tinggi | Sedang/Tinggi |

### Runtime sandbox

Untuk isolasi ringan tanpa kontainer, [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime) memberlakukan pembatasan sistem file dan jaringan di tingkat OS.

Keuntungan utama adalah kesederhanaan: tidak ada konfigurasi Docker, gambar kontainer, atau setup jaringan yang diperlukan. Proxy dan pembatasan sistem file sudah tertanam. Anda menyediakan file pengaturan yang menentukan domain dan jalur yang diizinkan.

**Cara kerjanya:**
- **Sistem file**: Menggunakan primitif OS (`bubblewrap` di Linux, `sandbox-exec` di macOS) untuk membatasi akses baca/tulis ke jalur yang dikonfigurasi
- **Jaringan**: Menghapus namespace jaringan (Linux) atau menggunakan profil Seatbelt (macOS) untuk merutekan lalu lintas jaringan melalui proxy bawaan
- **Konfigurasi**: Daftar putih berbasis JSON untuk domain dan jalur sistem file

**Setup:**
```bash
npm install @anthropic-ai/sandbox-runtime
```

Kemudian buat file konfigurasi yang menentukan jalur dan domain yang diizinkan.

**Pertimbangan keamanan:**

1. **Kernel host yang sama**: Tidak seperti VM, proses sandbox berbagi kernel host. Kerentanan kernel secara teoritis dapat memungkinkan escape. Untuk beberapa model ancaman ini dapat diterima, tetapi jika Anda memerlukan isolasi tingkat kernel, gunakan gVisor atau VM terpisah.

2. **Tidak ada inspeksi TLS**: Proxy membuat daftar putih domain tetapi tidak menginspeksi lalu lintas terenkripsi. Jika agen memiliki kredensial permisif untuk domain yang diizinkan, pastikan tidak mungkin menggunakan domain itu untuk memicu permintaan jaringan lain atau untuk mengeksfiltrasi data.

Untuk banyak kasus penggunaan pengembang tunggal dan CI/CD, sandbox-runtime meningkatkan standar secara signifikan dengan setup minimal. Bagian di bawah mencakup kontainer dan VM untuk penyebaran yang memerlukan isolasi yang lebih kuat.

### Kontainer

Kontainer menyediakan isolasi melalui namespace Linux. Setiap kontainer memiliki pandangan sendiri tentang sistem file, pohon proses, dan tumpukan jaringan, sambil berbagi kernel host.

Konfigurasi kontainer yang dikeraskan keamanan mungkin terlihat seperti ini:

```bash
docker run \
  --cap-drop ALL \
  --security-opt no-new-privileges \
  --security-opt seccomp=/path/to/seccomp-profile.json \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /home/agent:rw,noexec,nosuid,size=500m \
  --network none \
  --memory 2g \
  --cpus 2 \
  --pids-limit 100 \
  --user 1000:1000 \
  -v /path/to/code:/workspace:ro \
  -v /var/run/proxy.sock:/var/run/proxy.sock:ro \
  agent-image
```

Berikut adalah apa yang dilakukan setiap opsi:

| Opsi | Tujuan |
|--------|---------|
| `--cap-drop ALL` | Menghapus kemampuan Linux seperti `NET_ADMIN` dan `SYS_ADMIN` yang dapat memungkinkan eskalasi hak istimewa |
| `--security-opt no-new-privileges` | Mencegah proses mendapatkan hak istimewa melalui binari setuid |
| `--security-opt seccomp=...` | Membatasi syscall yang tersedia; default Docker memblokir ~44, profil kustom dapat memblokir lebih banyak |
| `--read-only` | Membuat sistem file root kontainer tidak dapat diubah, mencegah agen dari mempertahankan perubahan |
| `--tmpfs /tmp:...` | Menyediakan direktori sementara yang dapat ditulis yang dihapus saat kontainer berhenti |
| `--network none` | Menghapus semua antarmuka jaringan; agen berkomunikasi melalui soket Unix yang dipasang di bawah |
| `--memory 2g` | Membatasi penggunaan memori untuk mencegah kelelahan sumber daya |
| `--pids-limit 100` | Membatasi jumlah proses untuk mencegah fork bomb |
| `--user 1000:1000` | Berjalan sebagai pengguna non-root |
| `-v ...:/workspace:ro` | Memasang kode baca-saja sehingga agen dapat menganalisis tetapi tidak memodifikasinya. **Hindari memasang direktori host sensitif seperti `~/.ssh`, `~/.aws`, atau `~/.config`** |
| `-v .../proxy.sock:...` | Memasang soket Unix yang terhubung ke proxy yang berjalan di luar kontainer (lihat di bawah) |

**Arsitektur soket Unix:**

Dengan `--network none`, kontainer tidak memiliki antarmuka jaringan sama sekali. Satu-satunya cara bagi agen untuk menjangkau dunia luar adalah melalui soket Unix yang dipasang, yang terhubung ke proxy yang berjalan di host. Proxy ini dapat memberlakukan daftar putih domain, menyuntikkan kredensial, dan mencatat semua lalu lintas.

Ini adalah arsitektur yang sama yang digunakan oleh [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime). Bahkan jika agen dikompromikan melalui injeksi prompt, agen tidak dapat mengeksfiltrasi data ke server arbitrer—agen hanya dapat berkomunikasi melalui proxy, yang mengontrol domain mana yang dapat dijangkau. Untuk detail lebih lanjut, lihat [posting blog sandboxing Claude Code](https://www.anthropic.com/engineering/claude-code-sandboxing).

**Opsi pengerasan tambahan:**

| Opsi | Tujuan |
|--------|---------|
| `--userns-remap` | Memetakan root kontainer ke pengguna host yang tidak istimewa; memerlukan konfigurasi daemon tetapi membatasi kerusakan dari escape kontainer |
| `--ipc private` | Mengisolasi komunikasi antar-proses untuk mencegah serangan lintas-kontainer |

### gVisor

Kontainer standar berbagi kernel host: ketika kode di dalam kontainer membuat panggilan sistem, itu langsung ke kernel yang sama yang menjalankan host. Ini berarti kerentanan kernel dapat memungkinkan escape kontainer. gVisor mengatasi ini dengan mengintersepsi panggilan sistem di userspace sebelum mencapai kernel host, menerapkan lapisan kompatibilitas sendiri yang menangani sebagian besar syscall tanpa melibatkan kernel nyata.

Jika agen menjalankan kode berbahaya (mungkin karena injeksi prompt), kode itu berjalan di kontainer dan dapat mencoba exploit kernel. Dengan gVisor, permukaan serangan jauh lebih kecil: kode berbahaya harus terlebih dahulu mengeksploitasi implementasi userspace gVisor dan akan memiliki akses terbatas ke kernel nyata.

Untuk menggunakan gVisor dengan Docker, instal runtime `runsc` dan konfigurasikan daemon:

```json
// /etc/docker/daemon.json
{
  "runtimes": {
    "runsc": {
      "path": "/usr/local/bin/runsc"
    }
  }
}
```

Kemudian jalankan kontainer dengan:

```bash
docker run --runtime=runsc agent-image
```

**Pertimbangan kinerja:**

| Beban Kerja | Overhead |
|----------|----------|
| Komputasi terikat CPU | ~0% (tidak ada intersepsi syscall) |
| Syscall sederhana | ~2× lebih lambat |
| I/O file intensif | Hingga 10-200× lebih lambat untuk pola open/close berat |

Untuk lingkungan multi-penyewa atau saat memproses konten yang tidak dipercaya, isolasi tambahan sering kali sepadan dengan overhead.

### Mesin virtual

VM menyediakan isolasi tingkat perangkat keras melalui ekstensi virtualisasi CPU. Setiap VM menjalankan kernel sendiri, menciptakan batas yang kuat—kerentanan di kernel tamu tidak secara langsung mengompromikan host. Namun, VM tidak secara otomatis "lebih aman" daripada alternatif seperti gVisor. Keamanan VM sangat bergantung pada hypervisor dan kode emulasi perangkat.

Firecracker dirancang untuk isolasi microVM ringan—dapat boot VM dalam waktu kurang dari 125ms dengan overhead memori kurang dari 5 MiB, menghilangkan emulasi perangkat yang tidak perlu untuk mengurangi permukaan serangan.

Dengan pendekatan ini, VM agen tidak memiliki antarmuka jaringan eksternal. Sebaliknya, agen berkomunikasi melalui `vsock` (soket virtual). Semua lalu lintas merutekan melalui vsock ke proxy di host, yang memberlakukan daftar putih dan menyuntikkan kredensial sebelum meneruskan permintaan.

### Penyebaran cloud

Untuk penyebaran cloud, Anda dapat menggabungkan salah satu teknologi isolasi di atas dengan kontrol jaringan asli cloud:

1. Jalankan kontainer agen di subnet pribadi tanpa gateway internet
2. Konfigurasikan aturan firewall cloud (AWS Security Groups, GCP VPC firewall) untuk memblokir semua egress kecuali ke proxy Anda
3. Jalankan proxy (seperti [Envoy](https://www.envoyproxy.io/) dengan filter `credential_injector` nya) yang memvalidasi permintaan, memberlakukan daftar putih domain, menyuntikkan kredensial, dan meneruskan ke API eksternal
4. Tetapkan izin IAM minimal ke akun layanan agen, merutekan akses sensitif melalui proxy jika memungkinkan
5. Catat semua lalu lintas di proxy untuk tujuan audit

## Manajemen kredensial

Agen sering memerlukan kredensial untuk memanggil API, mengakses repositori, atau berinteraksi dengan layanan cloud. Tantangannya adalah memberikan akses ini tanpa mengekspos kredensial itu sendiri.

### Pola proxy

Pendekatan yang direkomendasikan adalah menjalankan proxy di luar batas keamanan agen yang menyuntikkan kredensial ke dalam permintaan keluar. Agen mengirim permintaan tanpa kredensial, proxy menambahkannya, dan meneruskan permintaan ke tujuannya.

Pola ini memiliki beberapa manfaat:

1. Agen tidak pernah melihat kredensial aktual
2. Proxy dapat memberlakukan daftar putih titik akhir yang diizinkan
3. Proxy dapat mencatat semua permintaan untuk audit
4. Kredensial disimpan di satu lokasi aman daripada didistribusikan ke setiap agen

### Mengonfigurasi Claude Code untuk menggunakan proxy

Claude Code mendukung dua metode untuk merutekan permintaan sampling melalui proxy:

**Opsi 1: ANTHROPIC_BASE_URL (sederhana tetapi hanya untuk permintaan API sampling)**

```bash
export ANTHROPIC_BASE_URL="http://localhost:8080"
```

Ini memberitahu Claude Code dan Agent SDK untuk mengirim permintaan sampling ke proxy Anda daripada langsung ke API Anthropic. Proxy Anda menerima permintaan HTTP plaintext, dapat menginspeksi dan memodifikasinya (termasuk menyuntikkan kredensial), kemudian meneruskan ke API nyata.

**Opsi 2: HTTP_PROXY / HTTPS_PROXY (sistem luas)**

```bash
export HTTP_PROXY="http://localhost:8080"
export HTTPS_PROXY="http://localhost:8080"
```

Claude Code dan Agent SDK menghormati variabel lingkungan standar ini, merutekan semua lalu lintas HTTP melalui proxy. Untuk HTTPS, proxy membuat terowongan CONNECT terenkripsi: agen tidak dapat melihat atau memodifikasi konten permintaan tanpa intersepsi TLS.

### Mengimplementasikan proxy

Anda dapat membangun proxy Anda sendiri atau menggunakan yang sudah ada:

- [Envoy Proxy](https://www.envoyproxy.io/) — proxy tingkat produksi dengan filter `credential_injector` untuk menambahkan header auth
- [mitmproxy](https://mitmproxy.org/) — proxy yang mengakhiri TLS untuk menginspeksi dan memodifikasi lalu lintas HTTPS
- [Squid](http://www.squid-cache.org/) — proxy caching dengan daftar kontrol akses
- [LiteLLM](https://github.com/BerriAI/litellm) — gateway LLM dengan injeksi kredensial dan pembatasan laju

### Kredensial untuk layanan lain

Di luar sampling dari API Anthropic, agen sering memerlukan akses terautentikasi ke layanan lain—repositori git, database, API internal. Ada dua pendekatan utama:

#### Alat kustom

Sediakan akses melalui server MCP atau alat kustom yang merutekan permintaan ke layanan yang berjalan di luar batas keamanan agen. Agen memanggil alat, tetapi permintaan terautentikasi aktual terjadi di luar—alat memanggil proxy yang menyuntikkan kredensial.

Misalnya, server MCP git dapat menerima perintah dari agen tetapi meneruskannya ke proxy git yang berjalan di host, yang menambahkan autentikasi sebelum menghubungi repositori jarak jauh. Agen tidak pernah melihat kredensial.

Keuntungan:
- **Tidak ada intersepsi TLS**: Layanan eksternal membuat permintaan terautentikasi secara langsung
- **Kredensial tetap di luar**: Agen hanya melihat antarmuka alat, bukan kredensial yang mendasarinya

#### Penerusan lalu lintas

Untuk panggilan API Anthropic, `ANTHROPIC_BASE_URL` memungkinkan Anda merutekan permintaan ke proxy yang dapat menginspeksi dan memodifikasinya dalam plaintext. Tetapi untuk layanan HTTPS lainnya (GitHub, registri npm, API internal), lalu lintas sering terenkripsi end-to-end—bahkan jika Anda merutekannya melalui proxy melalui `HTTP_PROXY`, proxy hanya melihat terowongan TLS yang buram dan tidak dapat menyuntikkan kredensial.

Untuk memodifikasi lalu lintas HTTPS ke layanan arbitrer, tanpa menggunakan alat kustom, Anda memerlukan proxy yang mengakhiri TLS yang mendekripsi lalu lintas, menginspeksi atau memodifikasinya, kemudian mengenkripsi ulang sebelum meneruskan. Ini memerlukan:

1. Menjalankan proxy di luar kontainer agen
2. Memasang sertifikat CA proxy di penyimpanan kepercayaan agen (sehingga agen mempercayai sertifikat proxy)
3. Mengonfigurasi `HTTP_PROXY`/`HTTPS_PROXY` untuk merutekan lalu lintas melalui proxy

Pendekatan ini menangani layanan berbasis HTTP apa pun tanpa menulis alat kustom, tetapi menambah kompleksitas di sekitar manajemen sertifikat.

Perhatikan bahwa tidak semua program menghormati `HTTP_PROXY`/`HTTPS_PROXY`. Sebagian besar alat (curl, pip, npm, git) melakukannya, tetapi beberapa mungkin melewati variabel ini dan terhubung langsung. Misalnya, Node.js `fetch()` mengabaikan variabel ini secara default; di Node 24+ Anda dapat mengatur `NODE_USE_ENV_PROXY=1` untuk mengaktifkan dukungan. Untuk cakupan komprehensif, Anda dapat menggunakan [proxychains](https://github.com/haad/proxychains) untuk mengintersepsi panggilan jaringan, atau mengonfigurasi iptables untuk mengarahkan lalu lintas keluar ke proxy transparan.

<Info>
**Proxy transparan** mengintersepsi lalu lintas di tingkat jaringan, sehingga klien tidak perlu dikonfigurasi untuk menggunakannya. Proxy reguler memerlukan klien untuk secara eksplisit terhubung dan berbicara HTTP CONNECT atau SOCKS. Proxy transparan (seperti Squid atau mitmproxy dalam mode transparan) dapat menangani koneksi TCP yang dialihkan mentah.
</Info>

Kedua pendekatan masih memerlukan proxy yang mengakhiri TLS dan sertifikat CA terpercaya—mereka hanya memastikan lalu lintas benar-benar mencapai proxy.

## Konfigurasi sistem file

Kontrol sistem file menentukan file apa yang dapat dibaca dan ditulis agen.

### Pemasangan kode baca-saja

Ketika agen perlu menganalisis kode tetapi tidak memodifikasinya, pasang direktori baca-saja:

```bash
docker run -v /path/to/code:/workspace:ro agent-image
```

<Warning>
Bahkan akses baca-saja ke direktori kode dapat mengekspos kredensial. File umum untuk dikecualikan atau disanitasi sebelum pemasangan:

| File | Risiko |
|------|------|
| `.env`, `.env.local` | Kunci API, kata sandi database, rahasia |
| `~/.git-credentials` | Kata sandi/token git dalam plaintext |
| `~/.aws/credentials` | Kunci akses AWS |
| `~/.config/gcloud/application_default_credentials.json` | Token ADC Google Cloud |
| `~/.azure/` | Kredensial CLI Azure |
| `~/.docker/config.json` | Token auth registri Docker |
| `~/.kube/config` | Kredensial kluster Kubernetes |
| `.npmrc`, `.pypirc` | Token registri paket |
| `*-service-account.json` | Kunci akun layanan GCP |
| `*.pem`, `*.key` | Kunci pribadi |

Pertimbangkan untuk menyalin hanya file sumber yang diperlukan, atau menggunakan penyaringan gaya `.dockerignore`.
</Warning>

### Lokasi yang dapat ditulis

Jika agen perlu menulis file, Anda memiliki beberapa opsi tergantung pada apakah Anda ingin perubahan bertahan:

Untuk ruang kerja sementara di kontainer, gunakan pemasangan `tmpfs` yang hanya ada di memori dan dihapus saat kontainer berhenti:

```bash
docker run \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /workspace:rw,noexec,size=500m \
  agent-image
```

Jika Anda ingin meninjau perubahan sebelum mempertahankannya, sistem file overlay memungkinkan agen menulis tanpa memodifikasi file yang mendasar—perubahan disimpan di lapisan terpisah yang dapat Anda inspeksi, terapkan, atau buang. Untuk output yang sepenuhnya persisten, pasang volume khusus tetapi jauhkan dari direktori sensitif.

## Bacaan lebih lanjut

- [Dokumentasi keamanan Claude Code](https://code.claude.com/docs/id/security)
- [Hosting Agent SDK](/docs/id/agent-sdk/hosting)
- [Menangani izin](/docs/id/agent-sdk/permissions)
- [Runtime sandbox](https://github.com/anthropic-experimental/sandbox-runtime)
- [The Lethal Trifecta for AI Agents](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/)
- [OWASP Top 10 for LLM Applications](https://owasp.org/www-project-top-10-for-large-language-model-applications/)
- [Docker Security Best Practices](https://docs.docker.com/engine/security/)
- [gVisor Documentation](https://gvisor.dev/docs/)
- [Firecracker Documentation](https://firecracker-microvm.github.io/)