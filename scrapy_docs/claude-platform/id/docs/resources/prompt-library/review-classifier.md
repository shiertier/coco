# Pengklasifikasi ulasan

Kategorikan umpan balik ke dalam tag dan kategorisasi yang telah ditentukan sebelumnya.

---

|        | Konten |
| --- | --- |
| System | Anda adalah asisten AI yang dilatih untuk mengkategorikan umpan balik pengguna ke dalam kategori yang telah ditentukan sebelumnya, bersama dengan analisis sentimen untuk setiap kategori. Tujuan Anda adalah menganalisis setiap bagian umpan balik, menetapkan kategori yang paling relevan, dan menentukan sentimen (positif, negatif, atau netral) yang terkait dengan setiap kategori berdasarkan konten umpan balik. Kategori yang Telah Ditentukan: Fitur dan Fungsi Produk Fitur Inti Add-on dan Integrasi Kustomisasi dan Konfigurasi Pengalaman Pengguna dan Desain Kemudahan Penggunaan Navigasi dan Kemudahan Ditemukan Desain Visual dan Estetika Aksesibilitas Kinerja dan Keandalan Kecepatan dan Responsivitas Uptime dan Ketersediaan Skalabilitas Perbaikan Bug dan Penanganan Error Dukungan dan Layanan Pelanggan Responsivitas dan Ketersediaan Pengetahuan dan Keahlian Resolusi Masalah dan Tindak Lanjut Sumber Daya Layanan Mandiri Penagihan, Harga, dan Lisensi Model dan Tingkat Harga Proses Penagihan dan Faktur Manajemen Lisensi Upgrade dan Perpanjangan Keamanan, Kepatuhan, dan Privasi Perlindungan dan Kerahasiaan Data Kontrol Akses dan Autentikasi Kepatuhan Regulasi Respons Insiden dan Pemantauan Kompatibilitas Mobile dan Lintas Platform Fungsi Aplikasi Mobile Sinkronisasi dan Konsistensi Data Desain Responsif Kompatibilitas Perangkat dan OS Integrasi Pihak Ketiga dan API Fungsi dan Keandalan Integrasi Dokumentasi dan Dukungan API Kustomisasi dan Ekstensibilitas Onboarding, Pelatihan, dan Dokumentasi Panduan Pengguna dan Tutorial Panduan Dalam Aplikasi dan Tooltip Webinar dan Pelatihan Langsung Knowledge Base dan FAQ |
| User   | Saya telah menggunakan platform email marketing Anda selama beberapa minggu, dan saya harus mengatakan, fitur inti seperti pembuatan kampanye, template email, dan manajemen kontak sangat fantastis. Editor drag-and-drop memudahkan untuk mendesain email yang terlihat profesional, dan opsi segmentasi membantu saya menargetkan audiens yang tepat. Namun, saya mengalami beberapa masalah dengan responsivitas mobile dari email tersebut. Beberapa pelanggan saya melaporkan bahwa tata letak terlihat rusak di smartphone mereka, yang mengkhawatirkan. Saya ingin melihat perbaikan di area ini. Juga, saya perhatikan bahwa platform ini kehilangan beberapa integrasi kunci dengan alat CRM populer, yang akan sangat membantu untuk mengelola prospek dan pelanggan kami. Pada catatan positif, tim dukungan pelanggan telah responsif dan membantu setiap kali saya menghubungi mereka dengan pertanyaan. Secara keseluruhan, ini adalah alat yang bagus, tetapi pasti ada ruang untuk perbaikan dalam hal kompatibilitas mobile dan integrasi pihak ketiga. |

### Contoh output

> Kategori:
>
> - Fitur dan Fungsi Produk
>   - Fitur Inti - Positif
>     Pengalaman Pengguna dan Desain
>   - Kemudahan Penggunaan - Positif
>   - Desain Visual dan Estetika - Positif
> - Kompatibilitas Mobile dan Lintas Platform
>   - Fungsi Aplikasi Mobile - Negatif
>   - Desain Responsif - Negatif
>   - Kompatibilitas Perangkat dan OS - Negatif
> - Integrasi Pihak Ketiga dan API
>   - Fungsi dan Keandalan Integrasi - Negatif
> - Dukungan dan Layanan Pelanggan
>   - Responsivitas dan Ketersediaan - Positif
>   - Resolusi Masalah dan Tindak Lanjut - Positif

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # defaults to os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=1000,
        temperature=1,
        system="Anda adalah asisten AI yang dilatih untuk mengkategorikan umpan balik pengguna ke dalam kategori yang telah ditentukan sebelumnya, bersama dengan analisis sentimen untuk setiap kategori. Tujuan Anda adalah menganalisis setiap bagian umpan balik, menetapkan kategori yang paling relevan, dan menentukan sentimen (positif, negatif, atau netral) yang terkait dengan setiap kategori berdasarkan konten umpan balik.\n\nKategori yang Telah Ditentukan:\n- Fitur dan Fungsi Produk\n    - Fitur Inti\n    - Add-on dan Integrasi\n    - Kustomisasi dan Konfigurasi\n- Pengalaman Pengguna dan Desain\n    - Kemudahan Penggunaan\n    - Navigasi dan Kemudahan Ditemukan\n    - Desain Visual dan Estetika\n    - Aksesibilitas\n- Kinerja dan Keandalan\n    - Kecepatan dan Responsivitas\n    - Uptime dan Ketersediaan\n    - Skalabilitas\n    - Perbaikan Bug dan Penanganan Error\n- Dukungan dan Layanan Pelanggan\n    - Responsivitas dan Ketersediaan\n    - Pengetahuan dan Keahlian\n    - Resolusi Masalah dan Tindak Lanjut\n    - Sumber Daya Layanan Mandiri\n- Penagihan, Harga, dan Lisensi\n    - Model dan Tingkat Harga\n    - Proses Penagihan dan Faktur\n    - Manajemen Lisensi\n    - Upgrade dan Perpanjangan\n- Keamanan, Kepatuhan, dan Privasi\n    - Perlindungan dan Kerahasiaan Data\n    - Kontrol Akses dan Autentikasi\n    - Kepatuhan Regulasi\n    - Respons Insiden dan Pemantauan\n- Kompatibilitas Mobile dan Lintas Platform\n    - Fungsi Aplikasi Mobile\n    - Sinkronisasi dan Konsistensi Data\n    - Desain Responsif\n    - Kompatibilitas Perangkat dan OS\n- Integrasi Pihak Ketiga dan API\n    - Fungsi dan Keandalan Integrasi\n    - Dokumentasi dan Dukungan API\n    - Kustomisasi dan Ekstensibilitas\n- Onboarding, Pelatihan, dan Dokumentasi\n    - Panduan Pengguna dan Tutorial\n    - Panduan Dalam Aplikasi dan Tooltip\n    - Webinar dan Pelatihan Langsung\n    - Knowledge Base dan FAQ",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Saya telah menggunakan platform email marketing Anda selama beberapa minggu, dan saya harus mengatakan, fitur inti seperti pembuatan kampanye, template email, dan manajemen kontak sangat fantastis. Editor drag-and-drop memudahkan untuk mendesain email yang terlihat profesional, dan opsi segmentasi membantu saya menargetkan audiens yang tepat. Namun, saya mengalami beberapa masalah dengan responsivitas mobile dari email tersebut. Beberapa pelanggan saya melaporkan bahwa tata letak terlihat rusak di smartphone mereka, yang mengkhawatirkan. Saya ingin melihat perbaikan di area ini. Juga, saya perhatikan bahwa platform ini kehilangan beberapa integrasi kunci dengan alat CRM populer, yang akan sangat membantu untuk mengelola prospek dan pelanggan kami. Pada catatan positif, tim dukungan pelanggan telah responsif dan membantu setiap kali saya menghubungi mereka dengan pertanyaan. Secara keseluruhan, ini adalah alat yang bagus, tetapi pasti ada ruang untuk perbaikan dalam hal kompatibilitas mobile dan integrasi pihak ketiga."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript TypeScript
    import Anthropic from "@anthropic-ai/sdk";
    
    const anthropic = new Anthropic({
      apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
    });
    
    const msg = await anthropic.messages.create({
      model: "claude-sonnet-4-5",
      max_tokens: 1000,
      temperature: 1,
      system: "Anda adalah asisten AI yang dilatih untuk mengkategorikan umpan balik pengguna ke dalam kategori yang telah ditentukan sebelumnya, bersama dengan analisis sentimen untuk setiap kategori. Tujuan Anda adalah menganalisis setiap bagian umpan balik, menetapkan kategori yang paling relevan, dan menentukan sentimen (positif, negatif, atau netral) yang terkait dengan setiap kategori berdasarkan konten umpan balik.\n\nKategori yang Telah Ditentukan:\n- Fitur dan Fungsi Produk\n    - Fitur Inti\n    - Add-on dan Integrasi\n    - Kustomisasi dan Konfigurasi\n- Pengalaman Pengguna dan Desain\n    - Kemudahan Penggunaan\n    - Navigasi dan Kemudahan Ditemukan\n    - Desain Visual dan Estetika\n    - Aksesibilitas\n- Kinerja dan Keandalan\n    - Kecepatan dan Responsivitas\n    - Uptime dan Ketersediaan\n    - Skalabilitas\n    - Perbaikan Bug dan Penanganan Error\n- Dukungan dan Layanan Pelanggan\n    - Responsivitas dan Ketersediaan\n    - Pengetahuan dan Keahlian\n    - Resolusi Masalah dan Tindak Lanjut\n    - Sumber Daya Layanan Mandiri\n- Penagihan, Harga, dan Lisensi\n    - Model dan Tingkat Harga\n    - Proses Penagihan dan Faktur\n    - Manajemen Lisensi\n    - Upgrade dan Perpanjangan\n- Keamanan, Kepatuhan, dan Privasi\n    - Perlindungan dan Kerahasiaan Data\n    - Kontrol Akses dan Autentikasi\n    - Kepatuhan Regulasi\n    - Respons Insiden dan Pemantauan\n- Kompatibilitas Mobile dan Lintas Platform\n    - Fungsi Aplikasi Mobile\n    - Sinkronisasi dan Konsistensi Data\n    - Desain Responsif\n    - Kompatibilitas Perangkat dan OS\n- Integrasi Pihak Ketiga dan API\n    - Fungsi dan Keandalan Integrasi\n    - Dokumentasi dan Dukungan API\n    - Kustomisasi dan Ekstensibilitas\n- Onboarding, Pelatihan, dan Dokumentasi\n    - Panduan Pengguna dan Tutorial\n    - Panduan Dalam Aplikasi dan Tooltip\n    - Webinar dan Pelatihan Langsung\n    - Knowledge Base dan FAQ",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Saya telah menggunakan platform email marketing Anda selama beberapa minggu, dan saya harus mengatakan, fitur inti seperti pembuatan kampanye, template email, dan manajemen kontak sangat fantastis. Editor drag-and-drop memudahkan untuk mendesain email yang terlihat profesional, dan opsi segmentasi membantu saya menargetkan audiens yang tepat. Namun, saya mengalami beberapa masalah dengan responsivitas mobile dari email tersebut. Beberapa pelanggan saya melaporkan bahwa tata letak terlihat rusak di smartphone mereka, yang mengkhawatirkan. Saya ingin melihat perbaikan di area ini. Juga, saya perhatikan bahwa platform ini kehilangan beberapa integrasi kunci dengan alat CRM populer, yang akan sangat membantu untuk mengelola prospek dan pelanggan kami. Pada catatan positif, tim dukungan pelanggan telah responsif dan membantu setiap kali saya menghubungi mereka dengan pertanyaan. Secara keseluruhan, ini adalah alat yang bagus, tetapi pasti ada ruang untuk perbaikan dalam hal kompatibilitas mobile dan integrasi pihak ketiga."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
    
    ```python AWS Bedrock Python
    from anthropic import AnthropicBedrock
    
    # See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # for authentication options
    client = AnthropicBedrock()
    
    message = client.messages.create(
        model="anthropic.claude-sonnet-4-5-20250929-v1:0",
        max_tokens=1000,
        temperature=1,
        system="Anda adalah asisten AI yang dilatih untuk mengkategorikan umpan balik pengguna ke dalam kategori yang telah ditentukan sebelumnya, bersama dengan analisis sentimen untuk setiap kategori. Tujuan Anda adalah menganalisis setiap bagian umpan balik, menetapkan kategori yang paling relevan, dan menentukan sentimen (positif, negatif, atau netral) yang terkait dengan setiap kategori berdasarkan konten umpan balik.\n\nKategori yang Telah Ditentukan:\n- Fitur dan Fungsi Produk\n    - Fitur Inti\n    - Add-on dan Integrasi\n    - Kustomisasi dan Konfigurasi\n- Pengalaman Pengguna dan Desain\n    - Kemudahan Penggunaan\n    - Navigasi dan Kemudahan Ditemukan\n    - Desain Visual dan Estetika\n    - Aksesibilitas\n- Kinerja dan Keandalan\n    - Kecepatan dan Responsivitas\n    - Uptime dan Ketersediaan\n    - Skalabilitas\n    - Perbaikan Bug dan Penanganan Error\n- Dukungan dan Layanan Pelanggan\n    - Responsivitas dan Ketersediaan\n    - Pengetahuan dan Keahlian\n    - Resolusi Masalah dan Tindak Lanjut\n    - Sumber Daya Layanan Mandiri\n- Penagihan, Harga, dan Lisensi\n    - Model dan Tingkat Harga\n    - Proses Penagihan dan Faktur\n    - Manajemen Lisensi\n    - Upgrade dan Perpanjangan\n- Keamanan, Kepatuhan, dan Privasi\n    - Perlindungan dan Kerahasiaan Data\n    - Kontrol Akses dan Autentikasi\n    - Kepatuhan Regulasi\n    - Respons Insiden dan Pemantauan\n- Kompatibilitas Mobile dan Lintas Platform\n    - Fungsi Aplikasi Mobile\n    - Sinkronisasi dan Konsistensi Data\n    - Desain Responsif\n    - Kompatibilitas Perangkat dan OS\n- Integrasi Pihak Ketiga dan API\n    - Fungsi dan Keandalan Integrasi\n    - Dokumentasi dan Dukungan API\n    - Kustomisasi dan Ekstensibilitas\n- Onboarding, Pelatihan, dan Dokumentasi\n    - Panduan Pengguna dan Tutorial\n    - Panduan Dalam Aplikasi dan Tooltip\n    - Webinar dan Pelatihan Langsung\n    - Knowledge Base dan FAQ",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Saya telah menggunakan platform email marketing Anda selama beberapa minggu, dan saya harus mengatakan, fitur inti seperti pembuatan kampanye, template email, dan manajemen kontak sangat fantastis. Editor drag-and-drop memudahkan untuk mendesain email yang terlihat profesional, dan opsi segmentasi membantu saya menargetkan audiens yang tepat. Namun, saya mengalami beberapa masalah dengan responsivitas mobile dari email tersebut. Beberapa pelanggan saya melaporkan bahwa tata letak terlihat rusak di smartphone mereka, yang mengkhawatirkan. Saya ingin melihat perbaikan di area ini. Juga, saya perhatikan bahwa platform ini kehilangan beberapa integrasi kunci dengan alat CRM populer, yang akan sangat membantu untuk mengelola prospek dan pelanggan kami. Pada catatan positif, tim dukungan pelanggan telah responsif dan membantu setiap kali saya menghubungi mereka dengan pertanyaan. Secara keseluruhan, ini adalah alat yang bagus, tetapi pasti ada ruang untuk perbaikan dalam hal kompatibilitas mobile dan integrasi pihak ketiga."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    // See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    // for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 1000,
      temperature: 1,
      system: "Anda adalah asisten AI yang dilatih untuk mengkategorikan umpan balik pengguna ke dalam kategori yang telah ditentukan sebelumnya, bersama dengan analisis sentimen untuk setiap kategori. Tujuan Anda adalah menganalisis setiap bagian umpan balik, menetapkan kategori yang paling relevan, dan menentukan sentimen (positif, negatif, atau netral) yang terkait dengan setiap kategori berdasarkan konten umpan balik.\n\nKategori yang Telah Ditentukan:\n- Fitur dan Fungsi Produk\n    - Fitur Inti\n    - Add-on dan Integrasi\n    - Kustomisasi dan Konfigurasi\n- Pengalaman Pengguna dan Desain\n    - Kemudahan Penggunaan\n    - Navigasi dan Kemudahan Ditemukan\n    - Desain Visual dan Estetika\n    - Aksesibilitas\n- Kinerja dan Keandalan\n    - Kecepatan dan Responsivitas\n    - Uptime dan Ketersediaan\n    - Skalabilitas\n    - Perbaikan Bug dan Penanganan Error\n- Dukungan dan Layanan Pelanggan\n    - Responsivitas dan Ketersediaan\n    - Pengetahuan dan Keahlian\n    - Resolusi Masalah dan Tindak Lanjut\n    - Sumber Daya Layanan Mandiri\n- Penagihan, Harga, dan Lisensi\n    - Model dan Tingkat Harga\n    - Proses Penagihan dan Faktur\n    - Manajemen Lisensi\n    - Upgrade dan Perpanjangan\n- Keamanan, Kepatuhan, dan Privasi\n    - Perlindungan dan Kerahasiaan Data\n    - Kontrol Akses dan Autentikasi\n    - Kepatuhan Regulasi\n    - Respons Insiden dan Pemantauan\n- Kompatibilitas Mobile dan Lintas Platform\n    - Fungsi Aplikasi Mobile\n    - Sinkronisasi dan Konsistensi Data\n    - Desain Responsif\n    - Kompatibilitas Perangkat dan OS\n- Integrasi Pihak Ketiga dan API\n    - Fungsi dan Keandalan Integrasi\n    - Dokumentasi dan Dukungan API\n    - Kustomisasi dan Ekstensibilitas\n- Onboarding, Pelatihan, dan Dokumentasi\n    - Panduan Pengguna dan Tutorial\n    - Panduan Dalam Aplikasi dan Tooltip\n    - Webinar dan Pelatihan Langsung\n    - Knowledge Base dan FAQ",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Saya telah menggunakan platform email marketing Anda selama beberapa minggu, dan saya harus mengatakan, fitur inti seperti pembuatan kampanye, template email, dan manajemen kontak sangat fantastis. Editor drag-and-drop memudahkan untuk mendesain email yang terlihat profesional, dan opsi segmentasi membantu saya menargetkan audiens yang tepat. Namun, saya mengalami beberapa masalah dengan responsivitas mobile dari email tersebut. Beberapa pelanggan saya mel aporkan bahwa tata letak terlihat rusak di smartphone mereka, yang mengkhawatirkan. Saya ingin melihat perbaikan di area ini. Juga, saya perhatikan bahwa platform ini kehilangan beberapa integrasi kunci dengan alat CRM populer, yang akan sangat membantu untuk mengelola prospek dan pelanggan kami. Pada catatan positif, tim dukungan pelanggan telah responsif dan membantu setiap kali saya menghubungi mereka dengan pertanyaan. Secara keseluruhan, ini adalah alat yang bagus, tetapi pasti ada ruang untuk perbaikan dalam hal kompatibilitas mobile dan integrasi pihak ketiga."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
    
    ```python Vertex AI Python
    from anthropic import AnthropicVertex
    
    client = AnthropicVertex()
    
    message = client.messages.create(
        model="claude-sonnet-4@20250514",
        max_tokens=1000,
        temperature=1,
        system="Anda adalah asisten AI yang dilatih untuk mengkategorikan umpan balik pengguna ke dalam kategori yang telah ditentukan sebelumnya, bersama dengan analisis sentimen untuk setiap kategori. Tujuan Anda adalah menganalisis setiap bagian umpan balik, menetapkan kategori yang paling relevan, dan menentukan sentimen (positif, negatif, atau netral) yang terkait dengan setiap kategori berdasarkan konten umpan balik.\n\nKategori yang Telah Ditentukan:\n- Fitur dan Fungsi Produk\n    - Fitur Inti\n    - Add-on dan Integrasi\n    - Kustomisasi dan Konfigurasi\n- Pengalaman Pengguna dan Desain\n    - Kemudahan Penggunaan\n    - Navigasi dan Kemudahan Ditemukan\n    - Desain Visual dan Estetika\n    - Aksesibilitas\n- Kinerja dan Keandalan\n    - Kecepatan dan Responsivitas\n    - Uptime dan Ketersediaan\n    - Skalabilitas\n    - Perbaikan Bug dan Penanganan Error\n- Dukungan dan Layanan Pelanggan\n    - Responsivitas dan Ketersediaan\n    - Pengetahuan dan Keahlian\n    - Resolusi Masalah dan Tindak Lanjut\n    - Sumber Daya Layanan Mandiri\n- Penagihan, Harga, dan Lisensi\n    - Model dan Tingkat Harga\n    - Proses Penagihan dan Faktur\n    - Manajemen Lisensi\n    - Upgrade dan Perpanjangan\n- Keamanan, Kepatuhan, dan Privasi\n    - Perlindungan dan Kerahasiaan Data\n    - Kontrol Akses dan Autentikasi\n    - Kepatuhan Regulasi\n    - Respons Insiden dan Pemantauan\n- Kompatibilitas Mobile dan Lintas Platform\n    - Fungsi Aplikasi Mobile\n    - Sinkronisasi dan Konsistensi Data\n    - Desain Responsif\n    - Kompatibilitas Perangkat dan OS\n- Integrasi Pihak Ketiga dan API\n    - Fungsi dan Keandalan Integrasi\n    - Dokumentasi dan Dukungan API\n    - Kustomisasi dan Ekstensibilitas\n- Onboarding, Pelatihan, dan Dokumentasi\n    - Panduan Pengguna dan Tutorial\n    - Panduan Dalam Aplikasi dan Tooltip\n    - Webinar dan Pelatihan Langsung\n    - Knowledge Base dan FAQ",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Saya telah menggunakan platform email marketing Anda selama beberapa minggu, dan saya harus mengatakan, fitur inti seperti pembuatan kampanye, template email, dan manajemen kontak sangat fantastis. Editor drag-and-drop memudahkan untuk mendesain email yang terlihat profesional, dan opsi segmentasi membantu saya menargetkan audiens yang tepat. Namun, saya mengalami beberapa masalah dengan responsivitas mobile dari email tersebut. Beberapa pelanggan saya melaporkan bahwa tata letak terlihat rusak di smartphone mereka, yang mengkhawatirkan. Saya ingin melihat perbaikan di area ini. Juga, saya perhatikan bahwa platform ini kehilangan beberapa integrasi kunci dengan alat CRM populer, yang akan sangat membantu untuk mengelola prospek dan pelanggan kami. Pada catatan positif, tim dukungan pelanggan telah responsif dan membantu setiap kali saya menghubungi mereka dengan pertanyaan. Secara keseluruhan, ini adalah alat yang bagus, tetapi pasti ada ruang untuk perbaikan dalam hal kompatibilitas mobile dan integrasi pihak ketiga."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 1,
      system: "Anda adalah asisten AI yang dilatih untuk mengkategorikan umpan balik pengguna ke dalam kategori yang telah ditentukan sebelumnya, bersama dengan analisis sentimen untuk setiap kategori. Tujuan Anda adalah menganalisis setiap bagian umpan balik, menetapkan kategori yang paling relevan, dan menentukan sentimen (positif, negatif, atau netral) yang terkait dengan setiap kategori berdasarkan konten umpan balik.\n\nKategori yang Telah Ditentukan:\n- Fitur dan Fungsi Produk\n    - Fitur Inti\n    - Add-on dan Integrasi\n    - Kustomisasi dan Konfigurasi\n- Pengalaman Pengguna dan Desain\n    - Kemudahan Penggunaan\n    - Navigasi dan Kemudahan Ditemukan\n    - Desain Visual dan Estetika\n    - Aksesibilitas\n- Kinerja dan Keandalan\n    - Kecepatan dan Responsivitas\n    - Uptime dan Ketersediaan\n    - Skalabilitas\n    - Perbaikan Bug dan Penanganan Error\n- Dukungan dan Layanan Pelanggan\n    - Responsivitas dan Ketersediaan\n    - Pengetahuan dan Keahlian\n    - Resolusi Masalah dan Tindak Lanjut\n    - Sumber Daya Layanan Mandiri\n- Penagihan, Harga, dan Lisensi\n    - Model dan Tingkat Harga\n    - Proses Penagihan dan Faktur\n    - Manajemen Lisensi\n    - Upgrade dan Perpanjangan\n- Keamanan, Kepatuhan, dan Privasi\n    - Perlindungan dan Kerahasiaan Data\n    - Kontrol Akses dan Autentikasi\n    - Kepatuhan Regulasi\n    - Respons Insiden dan Pemantauan\n- Kompatibilitas Mobile dan Lintas Platform\n    - Fungsi Aplikasi Mobile\n    - Sinkronisasi dan Konsistensi Data\n    - Desain Responsif\n    - Kompatibilitas Perangkat dan OS\n- Integrasi Pihak Ketiga dan API\n    - Fungsi dan Keandalan Integrasi\n    - Dokumentasi dan Dukungan API\n    - Kustomisasi dan Ekstensibilitas\n- Onboarding, Pelatihan, dan Dokumentasi\n    - Panduan Pengguna dan Tutorial\n    - Panduan Dalam Aplikasi dan Tooltip\n    - Webinar dan Pelatihan Langsung\n    - Knowledge Base dan FAQ",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Saya telah menggunakan platform email marketing Anda selama beberapa minggu, dan saya harus mengatakan, fitur inti seperti pembuatan kampanye, template email, dan manajemen kontak sangat fantastis. Editor drag-and-drop memudahkan untuk mendesain email yang terlihat profesional, dan opsi segmentasi membantu saya menargetkan audiens yang tepat. Namun, saya mengalami beberapa masalah dengan responsivitas mobile dari email tersebut. Beberapa pelanggan saya melaporkan bahwa tata letak terlihat rusak di smartphone mereka, yang mengkhawatirkan. Saya ingin melihat perbaikan di area ini. Juga, saya perhatikan bahwa platform ini kehilangan beberapa integrasi kunci dengan alat CRM populer, yang akan sangat membantu untuk mengelola prospek dan pelanggan kami. Pada catatan positif, tim dukungan pelanggan telah responsif dan membantu setiap kali saya menghubungi mereka dengan pertanyaan. Secara keseluruhan, ini adalah alat yang bagus, tetapi pasti ada ruang untuk perbaikan dalam hal kompatibilitas mobile dan integrasi pihak ketiga."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>