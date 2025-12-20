# Dukungan multibahasa

Claude unggul dalam tugas di berbagai bahasa, mempertahankan kinerja lintas bahasa yang kuat relatif terhadap bahasa Inggris.

---

## Ikhtisar

Claude menunjukkan kemampuan multibahasa yang kuat, dengan kinerja yang sangat baik dalam tugas zero-shot di berbagai bahasa. Model mempertahankan kinerja relatif yang konsisten di seluruh bahasa yang banyak digunakan dan bahasa dengan sumber daya terbatas, menjadikannya pilihan yang andal untuk aplikasi multibahasa.

Perhatikan bahwa Claude mampu dalam banyak bahasa di luar yang dijadwal acuan di bawah. Kami mendorong pengujian dengan bahasa apa pun yang relevan dengan kasus penggunaan spesifik Anda.

## Data kinerja

Di bawah ini adalah skor evaluasi chain-of-thought zero-shot untuk model Claude di berbagai bahasa, ditampilkan sebagai persentase relatif terhadap kinerja bahasa Inggris (100%):

| Bahasa | Claude Opus 4.1<sup>1</sup> | Claude Opus 4<sup>1</sup> | Claude Sonnet 4.5<sup>1</sup> | Claude Sonnet 4<sup>1</sup> | Claude Haiku 4.5<sup>1</sup> |
|----------|---------------|---------------|---------------|-----------------|------------------|
| Bahasa Inggris (baseline, tetap 100%) | 100% | 100% | 100% | 100% | 100% |
| Spanyol | 98.1% | 98.0% | 98.2% | 97.5% | 96.4% |
| Portugis (Brasil) | 97.8% | 97.3% | 97.8% | 97.2% | 96.1% |
| Italia | 97.7% | 97.5% | 97.9% | 97.3% | 96.0% |
| Prancis | 97.9% | 97.7% | 97.5% | 97.1% | 95.7% |
| Indonesia | 97.3% | 97.2% | 97.3% | 96.2% | 94.2% |
| Jerman | 97.7% | 97.1% | 97.0% | 94.7% | 94.3% |
| Arab | 97.1% | 96.9% | 97.2% | 96.1% | 92.5% |
| Cina (Sederhana) | 97.1% | 96.7% | 96.9% | 95.9% | 94.2% |
| Korea | 96.6% | 96.4% | 96.7% | 95.9% | 93.3% |
| Jepang | 96.9% | 96.2% | 96.8% | 95.6% | 93.5% |
| Hindi | 96.8% | 96.7% | 96.7% | 95.8% | 92.4% |
| Bengali | 95.7% | 95.2% | 95.4% | 94.4% | 90.4% |
| Swahili | 89.8% | 89.5% | 91.1% | 87.1% | 78.3% |
| Yoruba | 80.3% | 78.9% | 79.7% | 76.4% | 52.7% |

<sup>1</sup> Dengan [extended thinking](/docs/id/build-with-claude/extended-thinking).

<Note>
Metrik ini didasarkan pada set tes bahasa Inggris [MMLU (Massive Multitask Language Understanding)](https://en.wikipedia.org/wiki/MMLU) yang diterjemahkan ke 14 bahasa tambahan oleh penerjemah manusia profesional, seperti yang didokumentasikan dalam [repositori simple-evals OpenAI](https://github.com/openai/simple-evals/blob/main/multilingual_mmlu_benchmark_results.md). Penggunaan penerjemah manusia untuk evaluasi ini memastikan terjemahan berkualitas tinggi, sangat penting untuk bahasa dengan sumber daya digital yang lebih sedikit.
</Note>

***

## Praktik terbaik

Saat bekerja dengan konten multibahasa:

1. **Berikan konteks bahasa yang jelas**: Meskipun Claude dapat mendeteksi bahasa target secara otomatis, menyatakan secara eksplisit bahasa input/output yang diinginkan meningkatkan keandalan. Untuk kelancaran yang ditingkatkan, Anda dapat meminta Claude menggunakan "ucapan idiomatis seolah-olah itu adalah penutur asli."
2. **Gunakan skrip asli**: Kirimkan teks dalam skrip aslinya daripada transliterasi untuk hasil optimal
3. **Pertimbangkan konteks budaya**: Komunikasi yang efektif sering kali memerlukan kesadaran budaya dan regional di luar sekadar terjemahan

Kami juga menyarankan mengikuti [panduan rekayasa prompt](/docs/id/build-with-claude/prompt-engineering/overview) umum kami untuk lebih meningkatkan kinerja Claude.

***

## Pertimbangan dukungan bahasa

- Claude memproses input dan menghasilkan output dalam sebagian besar bahasa dunia yang menggunakan karakter Unicode standar
- Kinerja bervariasi menurut bahasa, dengan kemampuan yang sangat kuat dalam bahasa yang banyak digunakan
- Bahkan dalam bahasa dengan sumber daya digital yang lebih sedikit, Claude mempertahankan kemampuan yang bermakna

<CardGroup cols={2}>
  <Card title="Panduan Rekayasa Prompt" icon="edit" href="/docs/id/build-with-claude/prompt-engineering/overview">
    Kuasai seni pembuatan prompt untuk mendapatkan hasil maksimal dari Claude.
  </Card>
  <Card title="Perpustakaan Prompt" icon="books" href="/docs/id/resources/prompt-library">
    Temukan berbagai prompt yang telah dirancang sebelumnya untuk berbagai tugas dan industri. Sempurna untuk inspirasi atau awal cepat.
  </Card>
</CardGroup>