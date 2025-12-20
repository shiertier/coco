# Perutean tiket

Panduan ini menjelaskan cara memanfaatkan kemampuan pemahaman bahasa alami tingkat lanjut Claude untuk mengklasifikasikan tiket dukungan pelanggan dalam skala besar berdasarkan niat pelanggan, urgensi, prioritas, profil pelanggan, dan banyak lagi.

---

## Tentukan apakah akan menggunakan Claude untuk perutean tiket

Berikut adalah beberapa indikator utama bahwa Anda harus menggunakan LLM seperti Claude alih-alih pendekatan ML tradisional untuk tugas klasifikasi Anda:

    <section title="Anda memiliki data pelatihan berlabel terbatas yang tersedia">

        Proses ML tradisional memerlukan dataset berlabel besar. Model Claude yang telah dilatih sebelumnya dapat secara efektif mengklasifikasikan tiket hanya dengan beberapa puluh contoh berlabel, secara signifikan mengurangi waktu dan biaya persiapan data.
    
</section>
    <section title="Kategori klasifikasi Anda mungkin akan berubah atau berkembang seiring waktu">

        Setelah pendekatan ML tradisional ditetapkan, mengubahnya adalah tugas yang membosankan dan padat data. Di sisi lain, seiring produk atau kebutuhan pelanggan Anda berkembang, Claude dapat dengan mudah beradaptasi dengan perubahan definisi kelas atau kelas baru tanpa pelabelan ulang data pelatihan yang ekstensif.
    
</section>
    <section title="Anda perlu menangani input teks yang kompleks dan tidak terstruktur">

        Model ML tradisional sering kesulitan dengan data tidak terstruktur dan memerlukan rekayasa fitur yang ekstensif. Pemahaman bahasa tingkat lanjut Claude memungkinkan klasifikasi akurat berdasarkan konten dan konteks, daripada mengandalkan struktur ontologi yang ketat.
    
</section>
    <section title="Aturan klasifikasi Anda didasarkan pada pemahaman semantik">

        Pendekatan ML tradisional sering mengandalkan model bag-of-words atau pencocokan pola sederhana. Claude unggul dalam memahami dan menerapkan aturan mendasar ketika kelas didefinisikan oleh kondisi daripada contoh.
    
</section>
    <section title="Anda memerlukan penalaran yang dapat diinterpretasikan untuk keputusan klasifikasi">

        Banyak model ML tradisional memberikan sedikit wawasan tentang proses pengambilan keputusan mereka. Claude dapat memberikan penjelasan yang dapat dibaca manusia untuk keputusan klasifikasinya, membangun kepercayaan pada sistem otomasi dan memfasilitasi adaptasi mudah jika diperlukan.
    
</section>
    <section title="Anda ingin menangani kasus tepi dan tiket ambigu dengan lebih efektif">

        Sistem ML tradisional sering kesulitan dengan outlier dan input ambigu, sering kali salah mengklasifikasikannya atau menggunakan kategori catch-all. Kemampuan pemrosesan bahasa alami Claude memungkinkannya untuk lebih baik menginterpretasikan konteks dan nuansa dalam tiket dukungan, berpotensi mengurangi jumlah tiket yang salah rute atau tidak diklasifikasikan yang memerlukan intervensi manual.
    
</section>
    <section title="Anda memerlukan dukungan multibahasa tanpa mempertahankan model terpisah">

        Pendekatan ML tradisional biasanya memerlukan model terpisah atau proses terjemahan ekstensif untuk setiap bahasa yang didukung. Kemampuan multibahasa Claude memungkinkannya untuk mengklasifikasikan tiket dalam berbagai bahasa tanpa perlu model terpisah atau proses terjemahan ekstensif, menyederhanakan dukungan untuk basis pelanggan global.
    
</section>

***

##  Bangun dan terapkan alur kerja dukungan LLM Anda

### Pahami pendekatan dukungan Anda saat ini
Sebelum terjun ke otomasi, penting untuk memahami sistem tiket yang ada. Mulai dengan menyelidiki bagaimana tim dukungan Anda saat ini menangani perutean tiket.

Pertimbangkan pertanyaan seperti:
* Kriteria apa yang digunakan untuk menentukan SLA/penawaran layanan apa yang diterapkan?
* Apakah perutean tiket digunakan untuk menentukan tingkat dukungan atau spesialis produk mana yang akan menerima tiket?
* Apakah ada aturan atau alur kerja otomatis yang sudah ada? Dalam kasus apa mereka gagal?
* Bagaimana kasus tepi atau tiket ambigu ditangani?
* Bagaimana tim memprioritaskan tiket?

Semakin banyak Anda tahu tentang bagaimana manusia menangani kasus tertentu, semakin baik Anda akan dapat bekerja dengan Claude untuk melakukan tugas tersebut.

### Tentukan kategori niat pengguna
Daftar kategori niat pengguna yang terdefinisi dengan baik sangat penting untuk klasifikasi tiket dukungan yang akurat dengan Claude. Kemampuan Claude untuk merutkan tiket secara efektif dalam sistem Anda berbanding lurus dengan seberapa baik kategori sistem Anda didefinisikan.

Berikut adalah beberapa contoh kategori dan subkategori niat pengguna.

    <section title="Masalah teknis">

        * Masalah perangkat keras
        * Bug perangkat lunak
        * Masalah kompatibilitas
        * Masalah kinerja
    
</section>
    <section title="Manajemen akun">

        * Pengaturan ulang kata sandi
        * Masalah akses akun
        * Pertanyaan penagihan
        * Perubahan langganan
    
</section>
    <section title="Informasi produk">

        * Pertanyaan fitur
        * Pertanyaan kompatibilitas produk
        * Informasi harga
        * Pertanyaan ketersediaan
    
</section>
    <section title="Panduan pengguna">

        * Pertanyaan cara-cara
        * Bantuan penggunaan fitur
        * Saran praktik terbaik
        * Panduan pemecahan masalah
    
</section>
    <section title="Umpan balik">

        * Laporan bug
        * Permintaan fitur
        * Umpan balik umum atau saran
        * Keluhan
    
</section>
    <section title="Terkait pesanan">

        * Pertanyaan status pesanan
        * Informasi pengiriman
        * Pengembalian dan pertukaran
        * Modifikasi pesanan
    
</section>
    <section title="Permintaan layanan">

        * Bantuan instalasi
        * Permintaan upgrade
        * Penjadwalan pemeliharaan
        * Pembatalan layanan
    
</section>
    <section title="Masalah keamanan">

        * Pertanyaan privasi data
        * Laporan aktivitas mencurigakan
        * Bantuan fitur keamanan
    
</section>
    <section title="Kepatuhan dan hukum">

        * Pertanyaan kepatuhan peraturan
        * Pertanyaan syarat layanan
        * Permintaan dokumentasi hukum
    
</section>
    <section title="Dukungan darurat">

        * Kegagalan sistem kritis
        * Masalah keamanan mendesak
        * Masalah sensitif waktu
    
</section>
    <section title="Pelatihan dan pendidikan">

        * Permintaan pelatihan produk
        * Pertanyaan dokumentasi
        * Informasi webinar atau workshop
    
</section>
    <section title="Integrasi dan API">

        * Bantuan integrasi
        * Pertanyaan penggunaan API
        * Pertanyaan kompatibilitas pihak ketiga
    
</section>

Selain niat, perutean tiket dan prioritas juga dapat dipengaruhi oleh faktor lain seperti urgensi, jenis pelanggan, SLA, atau bahasa. Pastikan untuk mempertimbangkan kriteria perutean lainnya saat membangun sistem perutean otomatis Anda.

### Tetapkan kriteria kesuksesan

Bekerja dengan tim dukungan Anda untuk [mendefinisikan kriteria kesuksesan yang jelas](/docs/id/test-and-evaluate/define-success) dengan tolok ukur, ambang batas, dan tujuan yang terukur.

Berikut adalah beberapa kriteria dan tolok ukur standar saat menggunakan LLM untuk perutean tiket dukungan:

    <section title="Konsistensi klasifikasi">

        Metrik ini menilai seberapa konsisten Claude mengklasifikasikan tiket serupa seiring waktu. Ini penting untuk mempertahankan keandalan perutean. Ukur ini dengan secara berkala menguji model dengan serangkaian input standar dan bertujuan untuk tingkat konsistensi 95% atau lebih tinggi.
    
</section>
    <section title="Kecepatan adaptasi">

        Ini mengukur seberapa cepat Claude dapat beradaptasi dengan kategori baru atau pola tiket yang berubah. Uji ini dengan memperkenalkan jenis tiket baru dan mengukur waktu yang diperlukan model untuk mencapai akurasi yang memuaskan (misalnya, >90%) pada kategori baru ini. Bertujuan untuk adaptasi dalam 50-100 tiket sampel.
    
</section>
    <section title="Penanganan multibahasa">

        Ini menilai kemampuan Claude untuk secara akurat merutkan tiket dalam berbagai bahasa. Ukur akurasi perutean di berbagai bahasa, bertujuan untuk tidak lebih dari penurunan akurasi 5-10% untuk bahasa non-primer.
    
</section>
    <section title="Penanganan kasus tepi">

        Ini mengevaluasi kinerja Claude pada tiket yang tidak biasa atau kompleks. Buat set tes kasus tepi dan ukur akurasi perutean, bertujuan untuk setidaknya akurasi 80% pada input yang menantang ini.
    
</section>
    <section title="Mitigasi bias">

        Ini mengukur keadilan Claude dalam perutean di berbagai demografi pelanggan. Secara teratur audit keputusan perutean untuk potensi bias, bertujuan untuk akurasi perutean yang konsisten (dalam 2-3%) di semua kelompok pelanggan.
    
</section>
    <section title="Efisiensi prompt">

        Dalam situasi di mana meminimalkan jumlah token sangat penting, kriteria ini menilai seberapa baik Claude berkinerja dengan konteks minimal. Ukur akurasi perutean dengan jumlah konteks yang berbeda-beda, bertujuan untuk akurasi 90%+ hanya dengan judul tiket dan deskripsi singkat.
    
</section>
    <section title="Skor penjelasan">

        Ini mengevaluasi kualitas dan relevansi penjelasan Claude untuk keputusan peruteannya. Penilai manusia dapat menilai penjelasan pada skala (misalnya, 1-5), dengan tujuan mencapai skor rata-rata 4 atau lebih tinggi.
    
</section>

Berikut adalah beberapa kriteria kesuksesan umum yang mungkin berguna terlepas dari apakah LLM digunakan:

    <section title="Akurasi perutean">

        Akurasi perutean mengukur seberapa sering tiket ditugaskan dengan benar ke tim atau individu yang sesuai pada percobaan pertama. Ini biasanya diukur sebagai persentase tiket yang dirutkan dengan benar dari total tiket. Tolok ukur industri sering bertujuan untuk akurasi 90-95%, meskipun ini dapat bervariasi berdasarkan kompleksitas struktur dukungan.
    
</section>
    <section title="Waktu ke penugasan">

        Metrik ini melacak seberapa cepat tiket ditugaskan setelah dikirimkan. Waktu penugasan yang lebih cepat umumnya mengarah pada resolusi yang lebih cepat dan kepuasan pelanggan yang lebih baik. Sistem kelas terbaik sering mencapai waktu penugasan rata-rata di bawah 5 menit, dengan banyak yang bertujuan untuk perutean hampir instan (yang mungkin dengan implementasi LLM).
    
</section>
    <section title="Tingkat perubahan rute">

        Tingkat perubahan rute menunjukkan seberapa sering tiket perlu ditugaskan kembali setelah perutean awal. Tingkat yang lebih rendah menunjukkan perutean awal yang lebih akurat. Bertujuan untuk tingkat perubahan rute di bawah 10%, dengan sistem berkinerja terbaik mencapai tingkat serendah 5% atau kurang.
    
</section>
    <section title="Tingkat resolusi kontak pertama">

        Ini mengukur persentase tiket yang diselesaikan selama interaksi pertama dengan pelanggan. Tingkat yang lebih tinggi menunjukkan perutean yang efisien dan tim dukungan yang siap. Tolok ukur industri biasanya berkisar dari 70-75%, dengan para pemimpin mencapai tingkat 80% atau lebih tinggi.
    
</section>
    <section title="Waktu penanganan rata-rata">

        Waktu penanganan rata-rata mengukur berapa lama waktu yang diperlukan untuk menyelesaikan tiket dari awal hingga akhir. Perutean yang efisien dapat secara signifikan mengurangi waktu ini. Tolok ukur bervariasi luas menurut industri dan kompleksitas, tetapi banyak organisasi bertujuan untuk menjaga waktu penanganan rata-rata di bawah 24 jam untuk masalah non-kritis.
    
</section>
    <section title="Skor kepuasan pelanggan">

        Sering diukur melalui survei pasca-interaksi, skor ini mencerminkan kebahagiaan pelanggan secara keseluruhan dengan proses dukungan. Perutean yang efektif berkontribusi pada kepuasan yang lebih tinggi. Bertujuan untuk skor CSAT 90% atau lebih tinggi, dengan para pemimpin sering mencapai tingkat kepuasan 95%+.
    
</section>
    <section title="Tingkat eskalasi">

        Ini mengukur seberapa sering tiket perlu ditingkatkan ke tingkat dukungan yang lebih tinggi. Tingkat eskalasi yang lebih rendah sering menunjukkan perutean awal yang lebih akurat. Berusaha untuk tingkat eskalasi di bawah 20%, dengan sistem kelas terbaik mencapai tingkat 10% atau kurang.
    
</section>
    <section title="Produktivitas agen">

        Metrik ini melihat berapa banyak tiket yang dapat ditangani agen secara efektif setelah menerapkan solusi perutean. Perutean yang ditingkatkan harus meningkatkan produktivitas. Ukur ini dengan melacak tiket yang diselesaikan per agen per hari atau jam, bertujuan untuk peningkatan 10-20% setelah menerapkan sistem perutean baru.
    
</section>
    <section title="Tingkat defleksi layanan mandiri">

        Ini mengukur persentase tiket potensial yang diselesaikan melalui opsi layanan mandiri sebelum memasuki sistem perutean. Tingkat yang lebih tinggi menunjukkan triage pra-perutean yang efektif. Bertujuan untuk tingkat defleksi 20-30%, dengan para pemimpin mencapai tingkat 40% atau lebih tinggi.
    
</section>
    <section title="Biaya per tiket">

        Metrik ini menghitung biaya rata-rata untuk menyelesaikan setiap tiket dukungan. Perutean yang efisien harus membantu mengurangi biaya ini seiring waktu. Meskipun tolok ukur bervariasi luas, banyak organisasi bertujuan untuk mengurangi biaya per tiket sebesar 10-15% setelah menerapkan sistem perutean yang ditingkatkan.
    
</section>

### Pilih model Claude yang tepat

Pilihan model tergantung pada trade-off antara biaya, akurasi, dan waktu respons.

Banyak pelanggan telah menemukan `claude-haiku-4-5-20251001` model ideal untuk perutean tiket, karena merupakan model tercepat dan paling hemat biaya dalam keluarga Claude 4 sambil tetap memberikan hasil yang sangat baik. Jika masalah klasifikasi Anda memerlukan keahlian subjek yang mendalam atau volume besar kategori niat penalaran kompleks, Anda dapat memilih [model Sonnet yang lebih besar](/docs/id/about-claude/models).

### Bangun prompt yang kuat

Perutean tiket adalah jenis tugas klasifikasi. Claude menganalisis konten tiket dukungan dan mengklasifikasikannya ke dalam kategori yang telah ditentukan sebelumnya berdasarkan jenis masalah, urgensi, keahlian yang diperlukan, atau faktor relevan lainnya.

Mari kita tulis prompt klasifikasi tiket. Prompt awal kami harus berisi konten permintaan pengguna dan mengembalikan baik penalaran maupun niat.

<Tip>
Coba [pembuat prompt](/docs/id/prompt-generator) di [Konsol Claude](/login) untuk membuat draf pertama untuk Anda.
</Tip>

Berikut adalah contoh prompt klasifikasi perutean tiket:
```python
def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. Your task is to analyze customer support requests and output the appropriate classification intent for each request, along with your reasoning. 

        Here is the customer support request you need to classify:

        <request>{ticket_contents}</request>

        Please carefully analyze the above request to determine the customer's core intent and needs. Consider what the customer is asking for has concerns about.

        First, write out your reasoning and analysis of how to classify this request inside <reasoning> tags.

        Then, output the appropriate classification label for the request inside a <intent> tag. The valid intents are:
        <intents>
        <intent>Support, Feedback, Complaint</intent>
        <intent>Order Tracking</intent>
        <intent>Refund/Exchange</intent>
        </intents>

        A request may have ONLY ONE applicable intent. Only include the intent that is most applicable to the request.

        As an example, consider the following request:
        <request>Hello! I had high-speed fiber internet installed on Saturday and my installer, Kevin, was absolutely fantastic! Where can I send my positive review? Thanks for your help!</request>

        Here is an example of how your output should be formatted (for the above example request):
        <reasoning>The user seeks information in order to leave positive feedback.</reasoning>
        <intent>Support, Feedback, Complaint</intent>

        Here are a few more examples:
        <examples>
        <example 2>
        Example 2 Input:
        <request>I wanted to write and personally thank you for the compassion you showed towards my family during my father's funeral this past weekend. Your staff was so considerate and helpful throughout this whole process; it really took a load off our shoulders. The visitation brochures were beautiful. We'll never forget the kindness you showed us and we are so appreciative of how smoothly the proceedings went. Thank you, again, Amarantha Hill on behalf of the Hill Family.</request>

        Example 2 Output:
        <reasoning>User leaves a positive review of their experience.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 2>
        <example 3>

        ...

        </example 8>
        <example 9>
        Example 9 Input:
        <request>Your website keeps sending ad-popups that block the entire screen. It took me twenty minutes just to finally find the phone number to call and complain. How can I possibly access my account information with all of these popups? Can you access my account for me, since your website is broken? I need to know what the address is on file.</request>

        Example 9 Output:
        <reasoning>The user requests help accessing their web account information.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 9>

        Remember to always include your classification reasoning before your actual intent output. The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
```

Mari kita uraikan komponen kunci dari prompt ini:
* Kami menggunakan f-string Python untuk membuat template prompt, memungkinkan `ticket_contents` untuk dimasukkan ke dalam tag `<request>`.
* Kami memberikan Claude peran yang jelas didefinisikan sebagai sistem klasifikasi yang dengan hati-hati menganalisis konten tiket untuk menentukan niat dan kebutuhan inti pelanggan.
* Kami menginstruksikan Claude tentang format output yang tepat, dalam hal ini untuk memberikan penalaran dan analisisnya di dalam tag `<reasoning>`, diikuti oleh label klasifikasi yang sesuai di dalam tag `<intent>`.
* Kami menentukan kategori niat yang valid: "Support, Feedback, Complaint", "Order Tracking", dan "Refund/Exchange".
* Kami menyertakan beberapa contoh (alias few-shot prompting) untuk mengilustrasikan bagaimana output harus diformat, yang meningkatkan akurasi dan konsistensi.

Alasan kami ingin Claude membagi responsnya ke dalam berbagai bagian tag XML adalah sehingga kami dapat menggunakan ekspresi reguler untuk secara terpisah mengekstrak penalaran dan niat dari output. Ini memungkinkan kami untuk membuat langkah berikutnya yang ditargetkan dalam alur kerja perutean tiket, seperti menggunakan hanya niat untuk memutuskan orang mana yang akan merutkan tiket.

### Terapkan prompt Anda

Sulit untuk mengetahui seberapa baik prompt Anda bekerja tanpa menerapkannya dalam pengaturan produksi tes dan [menjalankan evaluasi](/docs/id/test-and-evaluate/develop-tests).

Mari kita bangun struktur penyebaran. Mulai dengan mendefinisikan tanda tangan metode untuk membungkus panggilan kami ke Claude. Kami akan mengambil metode yang sudah kami mulai tulis, yang memiliki `ticket_contents` sebagai input, dan sekarang mengembalikan tuple `reasoning` dan `intent` sebagai output. Jika Anda memiliki otomasi yang ada menggunakan ML tradisional, Anda akan ingin mengikuti tanda tangan metode itu.

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ... The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
    # Send the prompt to the API to classify the support request.
    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
        stream=False,
    )
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

    return reasoning, intent
```

Kode ini:
* Mengimpor perpustakaan Anthropic dan membuat instance klien menggunakan kunci API Anda.
* Mendefinisikan fungsi `classify_support_request` yang mengambil string `ticket_contents`.
* Mengirimkan `ticket_contents` ke Claude untuk klasifikasi menggunakan `classification_prompt`
* Mengembalikan `reasoning` dan `intent` model yang diekstrak dari respons.

Karena kami perlu menunggu seluruh teks penalaran dan niat dihasilkan sebelum menguraikan, kami menetapkan `stream=False` (default).

***

## Evaluasi prompt Anda

Prompting sering memerlukan pengujian dan optimisasi agar siap produksi. Untuk menentukan kesiapan solusi Anda, evaluasi kinerja berdasarkan kriteria kesuksesan dan ambang batas yang Anda tetapkan sebelumnya.

Untuk menjalankan evaluasi Anda, Anda akan memerlukan kasus uji untuk menjalankannya. Sisa panduan ini mengasumsikan Anda telah [mengembangkan kasus uji Anda](/docs/id/test-and-evaluate/develop-tests).

### Bangun fungsi evaluasi

Evaluasi contoh kami untuk panduan ini mengukur kinerja Claude di sepanjang tiga metrik utama:
* Akurasi
* Biaya per klasifikasi

Anda mungkin perlu menilai Claude pada sumbu lain tergantung pada faktor apa yang penting bagi Anda.

Untuk menilai ini, pertama-tama kami harus memodifikasi skrip yang kami tulis dan menambahkan fungsi untuk membandingkan niat yang diprediksi dengan niat aktual dan menghitung persentase prediksi yang benar. Kami juga harus menambahkan fungsionalitas perhitungan biaya dan pengukuran waktu.

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(request, actual_intent):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ...The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """

    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
    )
    usage = message.usage  # Get the usage statistics for the API call for how many input and output tokens were used.
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

      # Check if the model's prediction is correct.
    correct = actual_intent.strip() == intent.strip()

    # Return the reasoning, intent, correct, and usage.
    return reasoning, intent, correct, usage
```

Mari kita uraikan edit yang kami buat:
* Kami menambahkan `actual_intent` dari kasus uji kami ke dalam metode `classify_support_request` dan menyiapkan perbandingan untuk menilai apakah klasifikasi niat Claude cocok dengan klasifikasi niat emas kami.
* Kami mengekstrak statistik penggunaan untuk panggilan API untuk menghitung biaya berdasarkan token input dan output yang digunakan

### Jalankan evaluasi Anda

Evaluasi yang tepat memerlukan ambang batas dan tolok ukur yang jelas untuk menentukan apa hasil yang baik. Skrip di atas akan memberi kami nilai runtime untuk akurasi, waktu respons, dan biaya per klasifikasi, tetapi kami masih perlu menetapkan ambang batas yang jelas. Sebagai contoh:
* **Akurasi:** 95% (dari 100 tes)
* **Biaya per klasifikasi:** Pengurangan 50% rata-rata (di seluruh 100 tes) dari metode perutean saat ini

Memiliki ambang batas ini memungkinkan Anda dengan cepat dan mudah mengatakan dalam skala, dan dengan empirisme yang tidak memihak, metode mana yang terbaik untuk Anda dan perubahan apa yang mungkin perlu dilakukan untuk lebih sesuai dengan persyaratan Anda.

***

## Tingkatkan kinerja

Dalam skenario kompleks, mungkin berguna untuk mempertimbangkan strategi tambahan untuk meningkatkan kinerja di luar [teknik rekayasa prompt](/docs/id/build-with-claude/prompt-engineering/overview) standar & [strategi implementasi guardrail](/docs/id/test-and-evaluate/strengthen-guardrails/reduce-hallucinations). Berikut adalah beberapa skenario umum:

### Gunakan hierarki taksonomi untuk kasus dengan 20+ kategori niat

Seiring jumlah kelas tumbuh, jumlah contoh yang diperlukan juga berkembang, berpotensi membuat prompt menjadi tidak praktis. Sebagai alternatif, Anda dapat mempertimbangkan untuk menerapkan sistem klasifikasi hierarki menggunakan campuran pengklasifikasi.
1. Atur niat Anda dalam struktur pohon taksonomi.
2. Buat serangkaian pengklasifikasi di setiap tingkat pohon, memungkinkan pendekatan perutean bertingkat.

Sebagai contoh, Anda mungkin memiliki pengklasifikasi tingkat atas yang secara luas mengkategorikan tiket menjadi "Masalah Teknis," "Pertanyaan Penagihan," dan "Pertanyaan Umum." Masing-masing kategori ini kemudian dapat memiliki sub-pengklasifikasi sendiri untuk lebih menyempurnakan klasifikasi.

![](/docs/images/ticket-hierarchy.png)

* **Pros - nuansa dan akurasi yang lebih besar:** Anda dapat membuat prompt berbeda untuk setiap jalur induk, memungkinkan klasifikasi yang lebih ditargetkan dan spesifik konteks. Ini dapat menghasilkan akurasi yang ditingkatkan dan penanganan permintaan pelanggan yang lebih bernuansa.

* **Cons - latensi yang meningkat:** Perlu diketahui bahwa beberapa pengklasifikasi dapat menyebabkan latensi yang meningkat, dan kami merekomendasikan menerapkan pendekatan ini dengan model tercepat kami, Haiku.

### Gunakan database vektor dan pencarian kesamaan retrieval untuk menangani tiket yang sangat variabel

Meskipun memberikan contoh adalah cara paling efektif untuk meningkatkan kinerja, jika permintaan dukungan sangat variabel, sulit untuk menyertakan cukup contoh dalam satu prompt.

Dalam skenario ini, Anda dapat menggunakan database vektor untuk melakukan pencarian kesamaan dari dataset contoh dan mengambil contoh paling relevan untuk kueri tertentu.

Pendekatan ini, yang dijelaskan secara detail dalam [resep klasifikasi](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb) kami, telah terbukti meningkatkan kinerja dari akurasi 71% menjadi akurasi 93%.

### Akun khusus untuk kasus tepi yang diharapkan

Berikut adalah beberapa skenario di mana Claude mungkin salah mengklasifikasikan tiket (mungkin ada yang lain yang unik untuk situasi Anda). Dalam skenario ini, pertimbangkan untuk memberikan instruksi eksplisit atau contoh dalam prompt tentang bagaimana Claude harus menangani kasus tepi:

    <section title="Pelanggan membuat permintaan implisit">

        Pelanggan sering mengekspresikan kebutuhan secara tidak langsung. Sebagai contoh, "Saya telah menunggu paket saya selama lebih dari dua minggu" mungkin merupakan permintaan tidak langsung untuk status pesanan.
        * **Solusi:** Berikan Claude dengan beberapa contoh pelanggan nyata dari jenis permintaan ini, bersama dengan apa niat yang mendasarinya. Anda dapat mendapatkan hasil yang bahkan lebih baik jika Anda menyertakan rasionalisasi klasifikasi untuk niat tiket yang sangat bernuansa, sehingga Claude dapat lebih baik menggeneralisasi logika ke tiket lain.
    
</section>
    <section title="Claude memprioritaskan emosi daripada niat">

        Ketika pelanggan mengekspresikan ketidakpuasan, Claude mungkin memprioritaskan mengatasi emosi daripada menyelesaikan masalah yang mendasar.
        * **Solusi:** Berikan Claude dengan arahan tentang kapan harus memprioritaskan sentimen pelanggan atau tidak. Bisa sesederhana "Abaikan semua emosi pelanggan. Fokus hanya pada menganalisis niat permintaan pelanggan dan informasi apa yang mungkin diminta pelanggan."
    
</section>
    <section title="Beberapa masalah menyebabkan kebingungan prioritas masalah">

        Ketika pelanggan menyajikan beberapa masalah dalam satu interaksi, Claude mungkin mengalami kesulitan mengidentifikasi kekhawatiran utama.
        * **Solusi:** Jelaskan prioritas niat sehingga Claude dapat lebih baik menentingkatkan niat yang diekstrak dan mengidentifikasi kekhawatiran utama.
    
</section>

***

## Integrasikan Claude ke dalam alur kerja dukungan yang lebih besar

Integrasi yang tepat memerlukan bahwa Anda membuat beberapa keputusan mengenai bagaimana skrip perutean berbasis Claude Anda cocok dengan arsitektur sistem perutean tiket yang lebih besar. Ada dua cara Anda bisa melakukan ini:
* **Berbasis push:** Sistem tiket dukungan yang Anda gunakan (misalnya Zendesk) memicu kode Anda dengan mengirimkan acara webhook ke layanan perutean Anda, yang kemudian mengklasifikasikan niat dan meruteannya.
    * Pendekatan ini lebih dapat diskalakan web, tetapi memerlukan Anda untuk mengekspos titik akhir publik.
* **Berbasis pull:** Kode Anda menarik tiket terbaru berdasarkan jadwal tertentu dan meruteannya pada waktu pull.
    * Pendekatan ini lebih mudah untuk diimplementasikan tetapi mungkin membuat panggilan yang tidak perlu ke sistem tiket dukungan ketika frekuensi pull terlalu tinggi atau mungkin terlalu lambat ketika frekuensi pull terlalu rendah.

Untuk salah satu pendekatan ini, Anda akan perlu membungkus skrip Anda dalam layanan. Pilihan pendekatan tergantung pada API apa yang disediakan sistem tiket dukungan Anda.

***

<CardGroup cols={2}>
    <Card title="Buku resep klasifikasi" icon="link" href="https://github.com/anthropics/anthropic-cookbook/tree/main/capabilities/classification">
        Kunjungi buku resep klasifikasi kami untuk contoh kode lebih lanjut dan panduan eval terperinci.
    </Card>
    <Card title="Konsol Claude" icon="link" href="/dashboard">
        Mulai membangun dan mengevaluasi alur kerja Anda di Konsol Claude.
    </Card>
</CardGroup>