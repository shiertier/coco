# Ringkasan dokumen hukum

Panduan ini menjelaskan cara memanfaatkan kemampuan pemrosesan bahasa alami tingkat lanjut Claude untuk merangkum dokumen hukum secara efisien, mengekstrak informasi kunci, dan mempercepat penelitian hukum. Dengan Claude, Anda dapat menyederhanakan tinjauan kontrak, persiapan litigasi, dan pekerjaan regulasi, menghemat waktu dan memastikan akurasi dalam proses hukum Anda.

---

> Kunjungi [buku resep ringkasan](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb) kami untuk melihat contoh implementasi ringkasan hukum menggunakan Claude.

## Sebelum membangun dengan Claude

### Tentukan apakah akan menggunakan Claude untuk ringkasan hukum

Berikut adalah beberapa indikator kunci bahwa Anda harus menggunakan LLM seperti Claude untuk merangkum dokumen hukum:

<section title="Anda ingin meninjau volume tinggi dokumen secara efisien dan terjangkau">
Tinjauan dokumen skala besar dapat memakan waktu dan mahal ketika dilakukan secara manual. Claude dapat memproses dan merangkum sejumlah besar dokumen hukum dengan cepat, secara signifikan mengurangi waktu dan biaya yang terkait dengan tinjauan dokumen. Kemampuan ini sangat berharga untuk tugas-tugas seperti due diligence, analisis kontrak, atau penemuan litigasi, di mana efisiensi sangat penting.
</section>
<section title="Anda memerlukan ekstraksi otomatis metadata kunci">
Claude dapat secara efisien mengekstrak dan mengkategorikan metadata penting dari dokumen hukum, seperti pihak yang terlibat, tanggal, syarat kontrak, atau klausa spesifik. Ekstraksi otomatis ini dapat membantu mengorganisir informasi, memudahkan pencarian, analisis, dan pengelolaan set dokumen besar. Ini sangat berguna untuk manajemen kontrak, pemeriksaan kepatuhan, atau pembuatan database dokumen hukum yang dapat dicari.
</section>
<section title="Anda ingin menghasilkan ringkasan yang jelas, ringkas, dan terstandar">
Claude dapat menghasilkan ringkasan terstruktur yang mengikuti format yang telah ditentukan sebelumnya, memudahkan para profesional hukum untuk dengan cepat memahami poin-poin kunci dari berbagai dokumen. Ringkasan terstandar ini dapat meningkatkan keterbacaan, memfasilitasi perbandingan antar dokumen, dan meningkatkan pemahaman keseluruhan, terutama ketika menangani bahasa hukum yang kompleks atau jargon teknis.
</section>
<section title="Anda memerlukan kutipan yang tepat untuk ringkasan Anda">
Ketika membuat ringkasan hukum, atribusi dan kutipan yang tepat sangat penting untuk memastikan kredibilitas dan kepatuhan terhadap standar hukum. Claude dapat diminta untuk menyertakan kutipan akurat untuk semua poin hukum yang direferensikan, memudahkan para profesional hukum untuk meninjau dan memverifikasi informasi yang dirangkum.
</section>
<section title="Anda ingin menyederhanakan dan mempercepat proses penelitian hukum Anda">
Claude dapat membantu dalam penelitian hukum dengan cepat menganalisis volume besar kasus hukum, undang-undang, dan komentar hukum. Ini dapat mengidentifikasi preseden yang relevan, mengekstrak prinsip hukum kunci, dan merangkum argumen hukum yang kompleks. Kemampuan ini dapat secara signifikan mempercepat proses penelitian, memungkinkan para profesional hukum untuk fokus pada analisis tingkat lebih tinggi dan pengembangan strategi.
</section>

### Tentukan detail yang ingin diekstrak ringkasan

Tidak ada ringkasan tunggal yang benar untuk dokumen apa pun. Tanpa arahan yang jelas, dapat sulit bagi Claude untuk menentukan detail mana yang akan disertakan. Untuk mencapai hasil optimal, identifikasi informasi spesifik yang ingin Anda sertakan dalam ringkasan.

Misalnya, ketika merangkum perjanjian sub-sewa, Anda mungkin ingin mengekstrak poin-poin kunci berikut:

```python
details_to_extract = [
    'Parties involved (sublessor, sublessee, and original lessor)',
    'Property details (address, description, and permitted use)', 
    'Term and rent (start date, end date, monthly rent, and security deposit)',
    'Responsibilities (utilities, maintenance, and repairs)',
    'Consent and notices (landlord\'s consent, and notice requirements)',
    'Special provisions (furniture, parking, and subletting restrictions)'
]
```

### Tetapkan kriteria kesuksesan

Mengevaluasi kualitas ringkasan adalah tugas yang terkenal menantang. Tidak seperti banyak tugas pemrosesan bahasa alami lainnya, evaluasi ringkasan sering kali kekurangan metrik objektif yang jelas. Prosesnya dapat sangat subjektif, dengan pembaca yang berbeda menghargai aspek berbeda dari ringkasan. Berikut adalah kriteria yang mungkin ingin Anda pertimbangkan saat menilai seberapa baik Claude melakukan ringkasan hukum.

<section title="Kebenaran faktual">
Ringkasan harus secara akurat mewakili fakta, konsep hukum, dan poin-poin kunci dalam dokumen.
</section>
<section title="Presisi hukum">
Terminologi dan referensi ke undang-undang, kasus hukum, atau peraturan harus benar dan selaras dengan standar hukum.
</section>
<section title="Keringkasan">
Ringkasan harus mengondensasi dokumen hukum ke poin-poin esensialnya tanpa kehilangan detail penting.
</section>
<section title="Konsistensi">
Jika merangkum beberapa dokumen, LLM harus mempertahankan struktur dan pendekatan yang konsisten untuk setiap ringkasan.
</section>
<section title="Keterbacaan">
Teks harus jelas dan mudah dipahami. Jika audiens bukan ahli hukum, ringkasan tidak boleh menyertakan jargon hukum yang dapat membingungkan audiens.
</section>
<section title="Bias dan keadilan">
Ringkasan harus menyajikan penggambaran yang tidak bias dan adil dari argumen dan posisi hukum.
</section>

Lihat panduan kami tentang [menetapkan kriteria kesuksesan](/docs/id/test-and-evaluate/define-success) untuk informasi lebih lanjut.

---

## Cara merangkum dokumen hukum menggunakan Claude

### Pilih model Claude yang tepat

Akurasi model sangat penting ketika merangkum dokumen hukum. Claude Sonnet 4.5 adalah pilihan yang sangat baik untuk kasus penggunaan seperti ini di mana akurasi tinggi diperlukan. Jika ukuran dan jumlah dokumen Anda besar sehingga biaya mulai menjadi perhatian, Anda juga dapat mencoba menggunakan model yang lebih kecil seperti Claude Haiku 4.5.

Untuk membantu memperkirakan biaya ini, di bawah ini adalah perbandingan biaya untuk merangkum 1.000 perjanjian sub-sewa menggunakan Sonnet dan Haiku:

* **Ukuran konten**
    * Jumlah perjanjian: 1.000
    * Karakter per perjanjian: 300.000
    * Total karakter: 300M

* **Token yang diperkirakan**
    * Token input: 86M (dengan asumsi 1 token per 3,5 karakter)
    * Token output per ringkasan: 350
    * Total token output: 350.000
 
* **Biaya Claude Sonnet 4.5 yang diperkirakan**
    * Biaya token input: 86 MTok * \$3.00/MTok = \$258
    * Biaya token output: 0,35 MTok * \$15.00/MTok = \$5.25
    * Total biaya: \$258.00 + \$5.25 = \$263.25

* **Biaya Claude Haiku 3 yang diperkirakan**
    * Biaya token input: 86 MTok * \$0.25/MTok = \$21.50
    * Biaya token output: 0,35 MTok * \$1.25/MTok = \$0.44
    * Total biaya: \$21.50 + \$0.44 = \$21.96

<Tip>Biaya aktual mungkin berbeda dari perkiraan ini. Perkiraan ini didasarkan pada contoh yang disorot di bagian tentang [prompting](#build-a-strong-prompt).</Tip>

### Ubah dokumen menjadi format yang dapat diproses Claude

Sebelum Anda mulai merangkum dokumen, Anda perlu menyiapkan data Anda. Ini melibatkan ekstraksi teks dari PDF, pembersihan teks, dan memastikan siap diproses oleh Claude.

Berikut adalah demonstrasi proses ini pada pdf sampel:

```python
from io import BytesIO
import re

import pypdf
import requests

def get_llm_text(pdf_file):
    reader = pypdf.PdfReader(pdf_file)
    text = "\n".join([page.extract_text() for page in reader.pages])

    # Remove extra whitespace
    text = re.sub(r'\s+', ' ', text) 

    # Remove page numbers
    text = re.sub(r'\n\s*\d+\s*\n', '\n', text) 

    return text


# Create the full URL from the GitHub repository
url = "https://raw.githubusercontent.com/anthropics/anthropic-cookbook/main/skills/summarization/data/Sample Sublease Agreement.pdf"
url = url.replace(" ", "%20")

# Download the PDF file into memory
response = requests.get(url)

# Load the PDF from memory
pdf_file = BytesIO(response.content)

document_text = get_llm_text(pdf_file) 
print(document_text[:50000]) 
```

Dalam contoh ini, kami pertama-tama mengunduh pdf perjanjian sub-sewa sampel yang digunakan dalam [buku resep ringkasan](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/data/Sample%20Sublease%20Agreement.pdf). Perjanjian ini bersumber dari perjanjian sub-sewa yang tersedia untuk umum dari [situs web sec.gov](https://www.sec.gov/Archives/edgar/data/1045425/000119312507044370/dex1032.htm).

Kami menggunakan perpustakaan pypdf untuk mengekstrak konten pdf dan mengubahnya menjadi teks. Data teks kemudian dibersihkan dengan menghapus spasi putih ekstra dan nomor halaman.

### Bangun prompt yang kuat

Claude dapat beradaptasi dengan berbagai gaya ringkasan. Anda dapat mengubah detail prompt untuk memandu Claude agar lebih atau kurang verbose, menyertakan lebih atau kurang terminologi teknis, atau memberikan ringkasan tingkat lebih tinggi atau lebih rendah dari konteks yang ada.

Berikut adalah contoh cara membuat prompt yang memastikan ringkasan yang dihasilkan mengikuti struktur yang konsisten saat menganalisis perjanjian sub-sewa:

```python
import anthropic

# Initialize the Anthropic client
client = anthropic.Anthropic()

def summarize_document(text, details_to_extract, model="claude-sonnet-4-5", max_tokens=1000):

    # Format the details to extract to be placed within the prompt's context
    details_to_extract_str = '\n'.join(details_to_extract)
    
    # Prompt the model to summarize the sublease agreement
    prompt = f"""Summarize the following sublease agreement. Focus on these key aspects:

    {details_to_extract_str}

    Provide the summary in bullet points nested within the XML header for each section. For example:

    <parties involved>
    - Sublessor: [Name]
    // Add more details as needed
    </parties involved>
    
    If any information is not explicitly stated in the document, note it as "Not specified". Do not preamble.

    Sublease agreement text:
    {text}
    """

    response = client.messages.create(
        model=model,
        max_tokens=max_tokens,
        system="You are a legal analyst specializing in real estate law, known for highly accurate and detailed summaries of sublease agreements.",
        messages=[
            {"role": "user", "content": prompt},
            {"role": "assistant", "content": "Here is the summary of the sublease agreement: <summary>"}
        ],
        stop_sequences=["</summary>"]
    )

    return response.content[0].text

sublease_summary = summarize_document(document_text, details_to_extract)
print(sublease_summary)
```

Kode ini mengimplementasikan fungsi `summarize_document` yang menggunakan Claude untuk merangkum konten perjanjian sub-sewa. Fungsi menerima string teks dan daftar detail untuk diekstrak sebagai input. Dalam contoh ini, kami memanggil fungsi dengan variabel `document_text` dan `details_to_extract` yang didefinisikan dalam cuplikan kode sebelumnya.

Dalam fungsi, prompt dihasilkan untuk Claude, termasuk dokumen yang akan dirangkum, detail untuk diekstrak, dan instruksi spesifik untuk merangkum dokumen. Prompt menginstruksikan Claude untuk merespons dengan ringkasan setiap detail untuk diekstrak bersarang dalam header XML.

Karena kami memutuskan untuk mengeluarkan setiap bagian ringkasan dalam tag, setiap bagian dapat dengan mudah diurai sebagai langkah pasca-pemrosesan. Pendekatan ini memungkinkan ringkasan terstruktur yang dapat disesuaikan untuk kasus penggunaan Anda, sehingga setiap ringkasan mengikuti pola yang sama.

### Evaluasi prompt Anda

Prompting sering kali memerlukan pengujian dan optimasi agar siap produksi. Untuk menentukan kesiapan solusi Anda, evaluasi kualitas ringkasan Anda menggunakan proses sistematis yang menggabungkan metode kuantitatif dan kualitatif. Membuat [evaluasi empiris yang kuat](/docs/id/test-and-evaluate/develop-tests#building-evals-and-test-cases) berdasarkan kriteria kesuksesan yang ditentukan akan memungkinkan Anda mengoptimalkan prompt Anda. Berikut adalah beberapa metrik yang mungkin ingin Anda sertakan dalam evaluasi empiris Anda:

<section title="Skor ROUGE">
Ini mengukur tumpang tindih antara ringkasan yang dihasilkan dan ringkasan referensi yang dibuat oleh ahli. Metrik ini terutama fokus pada recall dan berguna untuk mengevaluasi cakupan konten.
</section>
<section title="Skor BLEU">
Meskipun awalnya dikembangkan untuk terjemahan mesin, metrik ini dapat disesuaikan untuk tugas ringkasan. Skor BLEU mengukur presisi kecocokan n-gram antara ringkasan yang dihasilkan dan ringkasan referensi. Skor yang lebih tinggi menunjukkan bahwa ringkasan yang dihasilkan berisi frasa dan terminologi serupa dengan ringkasan referensi.
</section>
<section title="Kesamaan embedding kontekstual">
Metrik ini melibatkan pembuatan representasi vektor (embedding) dari ringkasan yang dihasilkan dan referensi. Kesamaan antara embedding ini kemudian dihitung, sering kali menggunakan kesamaan kosinus. Skor kesamaan yang lebih tinggi menunjukkan bahwa ringkasan yang dihasilkan menangkap makna semantik dan konteks ringkasan referensi, bahkan jika kata-kata pastinya berbeda.
</section>
<section title="Penilaian berbasis LLM">
Metode ini melibatkan penggunaan LLM seperti Claude untuk mengevaluasi kualitas ringkasan yang dihasilkan terhadap rubrik penilaian. Rubrik dapat disesuaikan dengan kebutuhan spesifik Anda, menilai faktor-faktor kunci seperti akurasi, kelengkapan, dan koherensi. Untuk panduan tentang implementasi penilaian berbasis LLM, lihat [tips](/docs/id/test-and-evaluate/develop-tests#tips-for-llm-based-grading) ini.
</section>
<section title="Evaluasi manusia">
Selain membuat ringkasan referensi, para ahli hukum juga dapat mengevaluasi kualitas ringkasan yang dihasilkan. Meskipun ini mahal dan memakan waktu dalam skala besar, ini sering dilakukan pada beberapa ringkasan sebagai pemeriksaan akal sehat sebelum diterapkan ke produksi.
</section>

### Terapkan prompt Anda

Berikut adalah beberapa pertimbangan tambahan yang perlu diingat saat Anda menerapkan solusi Anda ke produksi.

1. **Pastikan tidak ada tanggung jawab:** Pahami implikasi hukum dari kesalahan dalam ringkasan, yang dapat menyebabkan tanggung jawab hukum bagi organisasi atau klien Anda. Berikan penafian atau pemberitahuan hukum yang menjelaskan bahwa ringkasan dihasilkan oleh AI dan harus ditinjau oleh para profesional hukum.

2. **Tangani berbagai jenis dokumen:** Dalam panduan ini, kami telah membahas cara mengekstrak teks dari PDF. Di dunia nyata, dokumen mungkin datang dalam berbagai format (PDF, dokumen Word, file teks, dll.). Pastikan pipeline ekstraksi data Anda dapat mengonversi semua format file yang Anda harapkan untuk diterima.

3. **Paralelkan panggilan API ke Claude:** Dokumen panjang dengan jumlah token besar mungkin memerlukan waktu hingga satu menit bagi Claude untuk menghasilkan ringkasan. Untuk koleksi dokumen besar, Anda mungkin ingin mengirim panggilan API ke Claude secara paralel sehingga ringkasan dapat diselesaikan dalam kerangka waktu yang wajar. Lihat [batas laju](/docs/id/api/rate-limits#rate-limits) Anthropic untuk menentukan jumlah maksimum panggilan API yang dapat dilakukan secara paralel.

---

## Tingkatkan kinerja

Dalam skenario kompleks, mungkin membantu untuk mempertimbangkan strategi tambahan untuk meningkatkan kinerja di luar teknik [rekayasa prompt](/docs/id/build-with-claude/prompt-engineering/overview) standar. Berikut adalah beberapa strategi lanjutan:

### Lakukan meta-ringkasan untuk merangkum dokumen panjang

Ringkasan hukum sering melibatkan penanganan dokumen panjang atau banyak dokumen terkait sekaligus, sehingga Anda melampaui jendela konteks Claude. Anda dapat menggunakan metode chunking yang dikenal sebagai meta-ringkasan untuk menangani kasus penggunaan ini. Teknik ini melibatkan pemecahan dokumen menjadi potongan yang lebih kecil dan dapat dikelola dan kemudian memproses setiap potongan secara terpisah. Anda kemudian dapat menggabungkan ringkasan setiap potongan untuk membuat meta-ringkasan dari seluruh dokumen.

Berikut adalah contoh cara melakukan meta-ringkasan:

```python
import anthropic

# Initialize the Anthropic client
client = anthropic.Anthropic()

def chunk_text(text, chunk_size=20000):
    return [text[i:i+chunk_size] for i in range(0, len(text), chunk_size)]

def summarize_long_document(text, details_to_extract, model="claude-sonnet-4-5", max_tokens=1000):

    # Format the details to extract to be placed within the prompt's context
    details_to_extract_str = '\n'.join(details_to_extract)

    # Iterate over chunks and summarize each one
    chunk_summaries = [summarize_document(chunk, details_to_extract, model=model, max_tokens=max_tokens) for chunk in chunk_text(text)]
    
    final_summary_prompt = f"""
    
    You are looking at the chunked summaries of multiple documents that are all related. 
    Combine the following summaries of the document from different truthful sources into a coherent overall summary:

    <chunked_summaries>
    {"".join(chunk_summaries)}
    </chunked_summaries>

    Focus on these key aspects:
    {details_to_extract_str})

    Provide the summary in bullet points nested within the XML header for each section. For example:

    <parties involved>
    - Sublessor: [Name]
    // Add more details as needed
    </parties involved>
    
    If any information is not explicitly stated in the document, note it as "Not specified". Do not preamble.
    """

    response = client.messages.create(
        model=model,
        max_tokens=max_tokens,
        system="You are a legal expert that summarizes notes on one document.",
        messages=[
            {"role": "user",  "content": final_summary_prompt},
            {"role": "assistant", "content": "Here is the summary of the sublease agreement: <summary>"}

        ],
        stop_sequences=["</summary>"]
    )
    
    return response.content[0].text

long_summary = summarize_long_document(document_text, details_to_extract)
print(long_summary)
```

Fungsi `summarize_long_document` dibangun di atas fungsi `summarize_document` sebelumnya dengan membagi dokumen menjadi potongan yang lebih kecil dan merangkum setiap potongan secara individual.

Kode mencapai ini dengan menerapkan fungsi `summarize_document` ke setiap potongan 20.000 karakter dalam dokumen asli. Ringkasan individual kemudian digabungkan, dan ringkasan akhir dibuat dari ringkasan potongan ini.

Perhatikan bahwa fungsi `summarize_long_document` tidak benar-benar diperlukan untuk pdf contoh kami, karena seluruh dokumen sesuai dengan jendela konteks Claude. Namun, ini menjadi penting untuk dokumen yang melebihi jendela konteks Claude atau ketika merangkum beberapa dokumen terkait bersama-sama. Terlepas dari itu, teknik meta-ringkasan ini sering menangkap detail penting tambahan dalam ringkasan akhir yang terlewatkan dalam pendekatan ringkasan tunggal sebelumnya.

### Gunakan dokumen yang diindeks ringkasan untuk menjelajahi koleksi dokumen besar

Mencari koleksi dokumen dengan LLM biasanya melibatkan retrieval-augmented generation (RAG). Namun, dalam skenario yang melibatkan dokumen besar atau ketika pengambilan informasi yang tepat sangat penting, pendekatan RAG dasar mungkin tidak cukup. Dokumen yang diindeks ringkasan adalah pendekatan RAG lanjutan yang menyediakan cara yang lebih efisien untuk menentukan peringkat dokumen untuk pengambilan, menggunakan konteks lebih sedikit daripada metode RAG tradisional. Dalam pendekatan ini, Anda pertama-tama menggunakan Claude untuk menghasilkan ringkasan ringkas untuk setiap dokumen dalam corpus Anda, dan kemudian menggunakan Claude untuk menentukan peringkat relevansi setiap ringkasan terhadap pertanyaan yang diajukan. Untuk detail lebih lanjut tentang pendekatan ini, termasuk contoh berbasis kode, lihat bagian dokumen yang diindeks ringkasan dalam [buku resep ringkasan](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb).

### Fine-tune Claude untuk belajar dari dataset Anda

Teknik lanjutan lainnya untuk meningkatkan kemampuan Claude menghasilkan ringkasan adalah fine-tuning. Fine-tuning melibatkan pelatihan Claude pada dataset khusus yang secara khusus selaras dengan kebutuhan ringkasan hukum Anda, memastikan bahwa Claude beradaptasi dengan kasus penggunaan Anda. Berikut adalah gambaran umum tentang cara melakukan fine-tuning:

1. **Identifikasi kesalahan:** Mulai dengan mengumpulkan contoh di mana ringkasan Claude kurang - ini dapat mencakup detail hukum kritis yang hilang, salah memahami konteks, atau menggunakan terminologi hukum yang tidak tepat.

2. **Kurasi dataset:** Setelah Anda mengidentifikasi masalah ini, kompilasi dataset contoh bermasalah ini. Dataset ini harus mencakup dokumen hukum asli bersama dengan ringkasan yang dikoreksi, memastikan bahwa Claude mempelajari perilaku yang diinginkan.

3. **Lakukan fine-tuning:** Fine-tuning melibatkan pelatihan ulang model pada dataset yang dikurasi untuk menyesuaikan bobot dan parameternya. Pelatihan ulang ini membantu Claude lebih memahami persyaratan spesifik domain hukum Anda, meningkatkan kemampuannya untuk merangkum dokumen sesuai dengan standar Anda.

4. **Peningkatan iteratif:** Fine-tuning bukan proses satu kali. Saat Claude terus menghasilkan ringkasan, Anda dapat secara iteratif menambahkan contoh baru di mana kinerjanya kurang, lebih lanjut menyempurnakan kemampuannya. Seiring waktu, loop umpan balik berkelanjutan ini akan menghasilkan model yang sangat khusus untuk tugas ringkasan hukum Anda.

<Tip>Fine-tuning saat ini hanya tersedia melalui Amazon Bedrock. Detail tambahan tersedia di [blog peluncuran AWS](https://aws.amazon.com/blogs/machine-learning/fine-tune-anthropics-claude-3-haiku-in-amazon-bedrock-to-boost-model-accuracy-and-quality/).</Tip>

<CardGroup cols={2}> 
  <Card title="Buku resep ringkasan" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb">
    Lihat contoh berbasis kode yang sepenuhnya diimplementasikan tentang cara menggunakan Claude untuk merangkum kontrak.
  </Card>
  <Card title="Buku resep kutipan" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb">
    Jelajahi resep buku resep Kutipan kami untuk panduan tentang cara memastikan akurasi dan penjelasan informasi.
  </Card>
</CardGroup>