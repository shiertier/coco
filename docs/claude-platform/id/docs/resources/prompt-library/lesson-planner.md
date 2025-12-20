# Perencana pelajaran

Buat rencana pelajaran mendalam tentang subjek apa pun.

---

> Salin prompt ini ke [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah membuat rencana pelajaran yang komprehensif, menarik, dan terstruktur dengan baik tentang subjek yang diberikan. Rencana pelajaran harus dirancang untuk sesi kelas 60 menit dan harus disesuaikan dengan tingkat kelas atau kelompok usia tertentu. Mulailah dengan menyatakan tujuan pelajaran, yang harus jelas, terukur, dan selaras dengan standar pendidikan yang relevan. Selanjutnya, berikan garis besar rinci dari pelajaran, membaginya menjadi pengantar, kegiatan utama, dan kesimpulan. Untuk setiap bagian, jelaskan metode pengajaran, kegiatan pembelajaran, dan sumber daya yang akan Anda gunakan untuk menyampaikan konten secara efektif dan melibatkan siswa. Terakhir, jelaskan metode penilaian yang akan Anda gunakan untuk mengevaluasi pemahaman dan penguasaan siswa terhadap tujuan pelajaran. Rencana pelajaran harus terorganisir dengan baik, mudah diikuti, dan mendorong pembelajaran aktif dan pemikiran kritis. |
| User   | Subjek: Pengantar Fotosintesis Tingkat Kelas: Kelas 7 (Usia 12-13 tahun) |

## Contoh output

> Rencana Pelajaran: Pengantar Fotosintesis
> Tingkat Kelas: Kelas 7 (Usia 12-13 tahun)
> Durasi: 60 menit
>
> Tujuan Pelajaran:
> Pada akhir pelajaran, siswa akan mampu:
>
> 1. Mendefinisikan fotosintesis dan menjelaskan pentingnya dalam ekosistem.
> 2. Mengidentifikasi komponen kunci yang diperlukan agar fotosintesis terjadi.
> 3. Menjelaskan proses fotosintesis, termasuk peran energi cahaya, karbon dioksida, dan air.
> 4. Mengenali produk fotosintesis dan signifikansinya bagi organisme hidup.
>    Garis Besar Pelajaran:
>
> 5. Pengantar (10 menit)
>    i. Libatkan siswa dengan menunjukkan klip video pendek atau gambar tumbuhan dan mendiskusikan peran mereka di lingkungan.
>    ii. Perkenalkan konsep fotosintesis dan pentingnya dalam mempertahankan kehidupan di Bumi.
>    iii. Nyatakan tujuan pelajaran dan berikan gambaran umum struktur pelajaran.
> 6. Kegiatan Utama (40 menit)
>    i. Instruksi Terpandu (15 menit)
>    a. Jelaskan komponen kunci yang diperlukan untuk fotosintesis: sinar matahari, karbon dioksida, air, dan klorofil.
>    b. Jelaskan proses fotosintesis, menekankan peran energi cahaya, karbon dioksida, dan air.
>    c. Diskusikan produk fotosintesis (glukosa dan oksigen) dan signifikansinya bagi organisme hidup.
>    ii. Kegiatan Kelompok Kecil (15 menit)
>    a. Bagi kelas menjadi kelompok kecil dan berikan setiap kelompok lembar kerja atau pengorganisir grafis.
>    b. Minta siswa bekerja sama untuk membuat representasi visual dari proses fotosintesis, memberi label pada komponen kunci dan produk.
>    c. Dorong siswa untuk mendiskusikan dan menjelaskan pemahaman mereka tentang fotosintesis dalam kelompok mereka.
>    iii. Diskusi Kelas (10 menit)
>    a. Undang setiap kelompok untuk berbagi representasi visual mereka dengan kelas.
>    b. Fasilitasi diskusi kelas, dorong siswa untuk mengajukan pertanyaan, memberikan umpan balik, dan mengklarifikasi kesalahpahaman apa pun.
> 7. Kesimpulan (10 menit)
>    i. Ringkas poin-poin kunci pelajaran, memperkuat definisi, komponen, proses, dan produk fotosintesis.
>    ii. Berikan kuis singkat atau tiket keluar untuk menilai pemahaman siswa terhadap tujuan pelajaran.
>    iii. Berikan siswa tugas rumah untuk lebih mengeksplorasi pentingnya fotosintesis dalam ekosistem yang berbeda.
>    Metode Penilaian:
>
> 8. Penilaian formatif: Amati partisipasi dan pemahaman siswa selama instruksi terpandu dan kegiatan kelompok kecil.
> 9. Penilaian sumatif: Evaluasi representasi visual siswa dan kinerja mereka pada kuis singkat atau tiket keluar.
> 10. Tugas rumah: Tinjau pengajuan tugas rumah siswa untuk mengukur kemampuan mereka menerapkan konsep yang dipelajari dalam pelajaran.
>     Sumber Daya:
>
> 11. Klip video atau gambar tumbuhan dan peran mereka di lingkungan
> 12. Lembar kerja atau pengorganisir grafis untuk kegiatan kelompok kecil
> 13. Kuis singkat atau tiket keluar untuk penilaian
> 14. Instruksi tugas rumah

### Permintaan API

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # defaults to os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=4000,
        temperature=0.5,
        system="Tugas Anda adalah membuat rencana pelajaran yang komprehensif, menarik, dan terstruktur dengan baik tentang subjek yang diberikan. Rencana pelajaran harus dirancang untuk sesi kelas 60 menit dan harus disesuaikan dengan tingkat kelas atau kelompok usia tertentu. Mulailah dengan menyatakan tujuan pelajaran, yang harus jelas, terukur, dan selaras dengan standar pendidikan yang relevan. Selanjutnya, berikan garis besar rinci dari pelajaran, membaginya menjadi pengantar, kegiatan utama, dan kesimpulan. Untuk setiap bagian, jelaskan metode pengajaran, kegiatan pembelajaran, dan sumber daya yang akan Anda gunakan untuk menyampaikan konten secara efektif dan melibatkan siswa. Terakhir, jelaskan metode penilaian yang akan Anda gunakan untuk mengevaluasi pemahaman dan penguasaan siswa terhadap tujuan pelajaran. Rencana pelajaran harus terorganisir dengan baik, mudah diikuti, dan mendorong pembelajaran aktif dan pemikiran kritis.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Subjek: Pengantar Fotosintesis  \nTingkat Kelas: Kelas 7 (Usia 12-13 tahun)"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "Tugas Anda adalah membuat rencana pelajaran yang komprehensif, menarik, dan terstruktur dengan baik tentang subjek yang diberikan. Rencana pelajaran harus dirancang untuk sesi kelas 60 menit dan harus disesuaikan dengan tingkat kelas atau kelompok usia tertentu. Mulailah dengan menyatakan tujuan pelajaran, yang harus jelas, terukur, dan selaras dengan standar pendidikan yang relevan. Selanjutnya, berikan garis besar rinci dari pelajaran, membaginya menjadi pengantar, kegiatan utama, dan kesimpulan. Untuk setiap bagian, jelaskan metode pengajaran, kegiatan pembelajaran, dan sumber daya yang akan Anda gunakan untuk menyampaikan konten secara efektif dan melibatkan siswa. Terakhir, jelaskan metode penilaian yang akan Anda gunakan untuk mengevaluasi pemahaman dan penguasaan siswa terhadap tujuan pelajaran. Rencana pelajaran harus terorganisir dengan baik, mudah diikuti, dan mendorong pembelajaran aktif dan pemikiran kritis.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Subjek: Pengantar Fotosintesis  \nTingkat Kelas: Kelas 7 (Usia 12-13 tahun)"
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
        max_tokens=4000,
        temperature=0.5,
        system="Tugas Anda adalah membuat rencana pelajaran yang komprehensif, menarik, dan terstruktur dengan baik tentang subjek yang diberikan. Rencana pelajaran harus dirancang untuk sesi kelas 60 menit dan harus disesuaikan dengan tingkat kelas atau kelompok usia tertentu. Mulailah dengan menyatakan tujuan pelajaran, yang harus jelas, terukur, dan selaras dengan standar pendidikan yang relevan. Selanjutnya, berikan garis besar rinci dari pelajaran, membaginya menjadi pengantar, kegiatan utama, dan kesimpulan. Untuk setiap bagian, jelaskan metode pengajaran, kegiatan pembelajaran, dan sumber daya yang akan Anda gunakan untuk menyampaikan konten secara efektif dan melibatkan siswa. Terakhir, jelaskan metode penilaian yang akan Anda gunakan untuk mengevaluasi pemahaman dan penguasaan siswa terhadap tujuan pelajaran. Rencana pelajaran harus terorganisir dengan baik, mudah diikuti, dan mendorong pembelajaran aktif dan pemikiran kritis.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Subjek: Pengantar Fotosintesis  \nTingkat Kelas: Kelas 7 (Usia 12-13 tahun)"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "Tugas Anda adalah membuat rencana pelajaran yang komprehensif, menarik, dan terstruktur dengan baik tentang subjek yang diberikan. Rencana pelajaran harus dirancang untuk sesi kelas 60 menit dan harus disesuaikan dengan tingkat kelas atau kelompok usia tertentu. Mulailah dengan menyatakan tujuan pelajaran, yang harus jelas, terukur, dan selaras dengan standar pendidikan yang relevan. Selanjutnya, berikan garis besar rinci dari pelajaran, membaginya menjadi pengantar, kegiatan utama, dan kesimpulan. Untuk setiap bagian, jelaskan metode pengajaran, kegiatan pembelajaran, dan sumber daya yang akan Anda gunakan untuk menyampaikan konten secara efektif dan melibatkan siswa. Terakhir, jelaskan metode penilaian yang akan Anda gunakan untuk mengevaluasi pemahaman dan penguasaan siswa terhadap tujuan pelajaran. Rencana pelajaran harus terorganisir dengan baik, mudah diikuti, dan mendorong pembelajaran aktif dan pemikiran kritis.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Subjek: Pengantar Fotosintesis  \nTingkat Kelas: Kelas 7 (Usia 12-13 tahun)"
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
        max_tokens=4000,
        temperature=0.5,
        system="Tugas Anda adalah membuat rencana pelajaran yang komprehensif, menarik, dan terstruktur dengan baik tentang subjek yang diberikan. Rencana pelajaran harus dirancang untuk sesi kelas 60 menit dan harus disesuaikan dengan tingkat kelas atau kelompok usia tertentu. Mulailah dengan menyatakan tujuan pelajaran, yang harus jelas, terukur, dan selaras dengan standar pendidikan yang relevan. Selanjutnya, berikan garis besar rinci dari pelajaran, membaginya menjadi pengantar, kegiatan utama, dan kesimpulan. Untuk setiap bagian, jelaskan metode pengajaran, kegiatan pembelajaran, dan sumber daya yang akan Anda gunakan untuk menyampaikan konten secara efektif dan melibatkan siswa. Terakhir, jelaskan metode penilaian yang akan Anda gunakan untuk mengevaluasi pemahaman dan penguasaan siswa terhadap tujuan pelajaran. Rencana pelajaran harus terorganisir dengan baik, mudah diikuti, dan mendorong pembelajaran aktif dan pemikiran kritis.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Subjek: Pengantar Fotosintesis  \nTingkat Kelas: Kelas 7 (Usia 12-13 tahun)"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "Tugas Anda adalah membuat rencana pelajaran yang komprehensif, menarik, dan terstruktur dengan baik tentang subjek yang diberikan. Rencana pelajaran harus dirancang untuk sesi kelas 60 menit dan harus disesuaikan dengan tingkat kelas atau kelompok usia tertentu. Mulailah dengan menyatakan tujuan pelajaran, yang harus jelas, terukur, dan selaras dengan standar pendidikan yang relevan. Selanjutnya, berikan garis besar rinci dari pelajaran, membaginya menjadi pengantar, kegiatan utama, dan kesimpulan. Untuk setiap bagian, jelaskan metode pengajaran, kegiatan pembelajaran, dan sumber daya yang akan Anda gunakan untuk menyampaikan konten secara efektif dan melibatkan siswa. Terakhir, jelaskan metode penilaian yang akan Anda gunakan untuk mengevaluasi pemahaman dan penguasaan siswa terhadap tujuan pelajaran. Rencana pelajaran harus terorganisir dengan baik, mudah diikuti, dan mendorong pembelajaran aktif dan pemikiran kritis.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Subjek: Pengantar Fotosintesis  \nTingkat Kelas: Kelas 7 (Usia 12-13 tahun)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>