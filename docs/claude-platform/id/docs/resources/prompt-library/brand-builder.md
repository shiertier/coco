# Pembangun merek

Buat brief desain untuk identitas merek yang holistik.

---

> Salin prompt ini ke [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah membuat brief desain yang komprehensif untuk identitas merek holistik berdasarkan spesifikasi yang diberikan. Identitas merek harus mencakup berbagai elemen seperti saran untuk nama merek, logo, palet warna, tipografi, gaya visual, nada suara, dan kepribadian merek secara keseluruhan. Pastikan bahwa semua elemen bekerja bersama secara harmonis untuk menciptakan pengalaman merek yang kohesif dan berkesan yang secara efektif mengkomunikasikan nilai-nilai merek, misi, dan proposisi penjualan unik kepada target audiensnya. Jadilah detail dan komprehensif serta berikan detail spesifik yang cukup bagi seseorang untuk menciptakan identitas merek yang benar-benar unik. |
| User   | Spesifikasi merek: Ini adalah merek yang berfokus pada penciptaan pakaian dan aksesori berkualitas tinggi dan bergaya menggunakan bahan ramah lingkungan dan metode produksi etis. Merek ini menargetkan konsumen yang sadar lingkungan berusia 25-40 tahun yang menghargai fashion, keberlanjutan, dan tanggung jawab sosial. Identitas merek harus mencapai tujuan-tujuan berikut: 1. Mencerminkan komitmen merek terhadap keberlanjutan, praktik etis, dan pengelolaan lingkungan. 2. Menarik bagi target audiens dengan menyampaikan rasa gaya, kualitas, dan tren. 3. Membedakan merek dari pesaing di pasar fashion berkelanjutan. 4. Menciptakan koneksi emosional yang kuat dengan konsumen dan menginspirasi mereka untuk membuat pilihan yang lebih ramah lingkungan. |

### Contoh output

> Brief Desain: Identitas Merek untuk Merek Fashion Ramah Lingkungan
> Saran Nama Merek: "Sēnsus" (bahasa Latin untuk "rasa" atau "persepsi," membangkitkan rasa kesadaran dan koneksi dengan lingkungan)
>
> Logo:
>
> - Logo minimalis dan modern yang menggabungkan motif daun atau tanaman yang distilisasi dengan tipografi sans-serif yang ramping.
> - Elemen daun atau tanaman melambangkan komitmen merek terhadap keberlanjutan dan pengelolaan lingkungan, sementara tipografi yang bersih menyampaikan rasa kecanggihan dan gaya.
> - Logo harus serbaguna dan bekerja dengan baik di berbagai aplikasi, termasuk tag pakaian, kemasan, dan platform digital.
>   Palet Warna:
>
> - Palet warna netral dan earthy yang mencerminkan etos ramah lingkungan merek, seperti nuansa hijau, beige, dan abu-abu hangat.
> - Warna aksen seperti teal dalam atau kuning mustard dapat digunakan secara hemat untuk menambah kecerahan dan kedalaman.
> - Palet warna harus membangkitkan rasa harmoni dan keseimbangan alami, sambil tetap fashion-forward dan menarik bagi target audiens.
>   Tipografi:
>
> - Kombinasi tipografi sans-serif yang bersih dan modern untuk headline dan tipografi serif yang komplementer untuk body copy.
> - Tipografi harus mudah dibaca dan menarik secara visual, menyampaikan rasa kualitas dan kecanggihan.
> - Penggunaan hierarki tipografi dan spasi yang konsisten akan menciptakan identitas merek yang kohesif dan terorganisir.
>   Gaya Visual:
>
> - Gaya visual minimalis dan tidak berantakan yang menekankan fotografi produk berkualitas tinggi dan tata letak yang bersih dan modern.
> - Menggabungkan elemen alami, seperti motif tanaman atau tekstur, untuk memperkuat nilai-nilai ramah lingkungan merek dan koneksi dengan alam.
> - Menggunakan ruang negatif secara efektif untuk menciptakan rasa keterbukaan dan kemampuan bernapas, mencerminkan komitmen merek terhadap keberlanjutan.
>   Nada Suara:
>
> - Nada suara merek harus dapat didekati, berpengetahuan, dan autentik.
> - Gunakan bahasa yang mendidik dan menginspirasi konsumen tentang manfaat fashion berkelanjutan dan metode produksi etis.
> - Menggabungkan sentuhan kecanggihan dan gaya untuk menarik sensibilitas fashion-conscious target audiens.
>   Kepribadian Merek:
>
> - Sadar lingkungan dan peduli lingkungan, namun bergaya dan fashion-forward.
> - Percaya diri dan bersemangat tentang mempromosikan praktik berkelanjutan dan pilihan etis.
> - Inklusif dan ramah, mengundang konsumen untuk bergabung dengan komunitas yang berpikiran sama yang berkomitmen untuk membuat perubahan positif.
>   Secara keseluruhan, identitas merek untuk "Sēnsus" harus mencapai keseimbangan antara nilai-nilai ramah lingkungan dan estetika fashion-forward, menarik bagi konsumen yang ingin terlihat bergaya sambil membuat pilihan yang bertanggung jawab terhadap lingkungan. Elemen visual dan verbal yang kohesif harus menciptakan koneksi emosional yang kuat dengan target audiens dan menginspirasi mereka untuk merangkul gaya hidup yang lebih berkelanjutan.

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
        max_tokens=2000,
        temperature=1,
        system="Tugas Anda adalah membuat brief desain yang komprehensif untuk identitas merek holistik berdasarkan spesifikasi yang diberikan. Identitas merek harus mencakup berbagai elemen seperti saran untuk nama merek, logo, palet warna, tipografi, gaya visual, nada suara, dan kepribadian merek secara keseluruhan. Pastikan bahwa semua elemen bekerja bersama secara harmonis untuk menciptakan pengalaman merek yang kohesif dan berkesan yang secara efektif mengkomunikasikan nilai-nilai merek, misi, dan proposisi penjualan unik kepada target audiensnya. Jadilah detail dan komprehensif serta berikan detail spesifik yang cukup bagi seseorang untuk menciptakan identitas merek yang benar-benar unik.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Spesifikasi merek:\nIni adalah merek yang berfokus pada penciptaan pakaian dan aksesori berkualitas tinggi dan bergaya menggunakan bahan ramah lingkungan dan metode produksi etis\nMerek ini menargetkan konsumen yang sadar lingkungan berusia 25-40 tahun yang menghargai fashion, keberlanjutan, dan tanggung jawab sosial.\nIdentitas merek harus mencapai tujuan-tujuan berikut:\n1. Mencerminkan komitmen merek terhadap keberlanjutan, praktik etis, dan pengelolaan lingkungan.\n2. Menarik bagi target audiens dengan menyampaikan rasa gaya, kualitas, dan tren.\n3. Membedakan merek dari pesaing di pasar fashion berkelanjutan.\n4. Menciptakan koneksi emosional yang kuat dengan konsumen dan menginspirasi mereka untuk membuat pilihan yang lebih ramah lingkungan."
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
      max_tokens: 2000,
      temperature: 1,
      system: "Tugas Anda adalah membuat brief desain yang komprehensif untuk identitas merek holistik berdasarkan spesifikasi yang diberikan. Identitas merek harus mencakup berbagai elemen seperti saran untuk nama merek, logo, palet warna, tipografi, gaya visual, nada suara, dan kepribadian merek secara keseluruhan. Pastikan bahwa semua elemen bekerja bersama secara harmonis untuk menciptakan pengalaman merek yang kohesif dan berkesan yang secara efektif mengkomunikasikan nilai-nilai merek, misi, dan proposisi penjualan unik kepada target audiensnya. Jadilah detail dan komprehensif serta berikan detail spesifik yang cukup bagi seseorang untuk menciptakan identitas merek yang benar-benar unik.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Spesifikasi merek:\nIni adalah merek yang berfokus pada penciptaan pakaian dan aksesori berkualitas tinggi dan bergaya menggunakan bahan ramah lingkungan dan metode produksi etis\nMerek ini menargetkan konsumen yang sadar lingkungan berusia 25-40 tahun yang menghargai fashion, keberlanjutan, dan tanggung jawab sosial.\nIdentitas merek harus mencapai tujuan-tujuan berikut:\n1. Mencerminkan komitmen merek terhadap keberlanjutan, praktik etis, dan pengelolaan lingkungan.\n2. Menarik bagi target audiens dengan menyampaikan rasa gaya, kualitas, dan tren.\n3. Membedakan merek dari pesaing di pasar fashion berkelanjutan.\n4. Menciptakan koneksi emosional yang kuat dengan konsumen dan menginspirasi mereka untuk membuat pilihan yang lebih ramah lingkungan."
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
        max_tokens=2000,
        temperature=1,
        system="Tugas Anda adalah membuat brief desain yang komprehensif untuk identitas merek holistik berdasarkan spesifikasi yang diberikan. Identitas merek harus mencakup berbagai elemen seperti saran untuk nama merek, logo, palet warna, tipografi, gaya visual, nada suara, dan kepribadian merek secara keseluruhan. Pastikan bahwa semua elemen bekerja bersama secara harmonis untuk menciptakan pengalaman merek yang kohesif dan berkesan yang secara efektif mengkomunikasikan nilai-nilai merek, misi, dan proposisi penjualan unik kepada target audiensnya. Jadilah detail dan komprehensif serta berikan detail spesifik yang cukup bagi seseorang untuk menciptakan identitas merek yang benar-benar unik.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Spesifikasi merek:\nIni adalah merek yang berfokus pada penciptaan pakaian dan aksesori berkualitas tinggi dan bergaya menggunakan bahan ramah lingkungan dan metode produksi etis\nMerek ini menargetkan konsumen yang sadar lingkungan berusia 25-40 tahun yang menghargai fashion, keberlanjutan, dan tanggung jawab sosial.\nIdentitas merek harus mencapai tujuan-tujuan berikut:\n1. Mencerminkan komitmen merek terhadap keberlanjutan, praktik etis, dan pengelolaan lingkungan.\n2. Menarik bagi target audiens dengan menyampaikan rasa gaya, kualitas, dan tren.\n3. Membedakan merek dari pesaing di pasar fashion berkelanjutan.\n4. Menciptakan koneksi emosional yang kuat dengan konsumen dan menginspirasi mereka untuk membuat pilihan yang lebih ramah lingkungan."
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
      max_tokens: 2000,
      temperature: 1,
      system: "Tugas Anda adalah membuat brief desain yang komprehensif untuk identitas merek holistik berdasarkan spesifikasi yang diberikan. Identitas merek harus mencakup berbagai elemen seperti saran untuk nama merek, logo, palet warna, tipografi, gaya visual, nada suara, dan kepribadian merek secara keseluruhan. Pastikan bahwa semua elemen bekerja bersama secara harmonis untuk menciptakan pengalaman merek yang kohesif dan berkesan yang secara efektif mengkomunikasikan nilai-nilai merek, misi, dan proposisi penjualan unik kepada target audiensnya. Jadilah detail dan komprehensif serta berikan detail spesifik yang cukup bagi seseorang untuk menciptakan identitas merek yang benar-benar unik.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Spesifikasi merek:\nIni adalah merek yang berfokus pada penciptaan pakaian dan aksesori berkualitas tinggi dan bergaya menggunakan bahan ramah lingkungan dan metode produksi etis\nMerek ini menargetkan konsumen yang sadar lingkungan berusia 25-40 tahun yang menghargai fashion, keberlanjutan, dan tanggung jawab sosial.\nIdentitas merek harus mencapai tujuan-tujuan berikut:\n1. Mencerminkan komitmen merek terhadap keberlanjutan, praktik etis, dan pengelolaan lingkungan.\n2. Menarik bagi target audiens dengan menyampaikan rasa gaya, kualitas, dan tren.\n3. Membedakan merek dari pesaing di pasar fashion berkelanjutan.\n4. Menciptakan koneksi emosional yang kuat dengan konsumen dan menginspirasi mereka untuk membuat pilihan yang lebih ramah lingkungan."
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
        max_tokens=2000,
        temperature=1,
        system="Tugas Anda adalah membuat brief desain yang komprehensif untuk identitas merek holistik berdasarkan spesifikasi yang diberikan. Identitas merek harus mencakup berbagai elemen seperti saran untuk nama merek, logo, palet warna, tipografi, gaya visual, nada suara, dan kepribadian merek secara keseluruhan. Pastikan bahwa semua elemen bekerja bersama secara harmonis untuk menciptakan pengalaman merek yang kohesif dan berkesan yang secara efektif mengkomunikasikan nilai-nilai merek, misi, dan proposisi penjualan unik kepada target audiensnya. Jadilah detail dan komprehensif serta berikan detail spesifik yang cukup bagi seseorang untuk menciptakan identitas merek yang benar-benar unik.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Spesifikasi merek:\nIni adalah merek yang berfokus pada penciptaan pakaian dan aksesori berkualitas tinggi dan bergaya menggunakan bahan ramah lingkungan dan metode produksi etis\nMerek ini menargetkan konsumen yang sadar lingkungan berusia 25-40 tahun yang menghargai fashion, keberlanjutan, dan tanggung jawab sosial.\nIdentitas merek harus mencapai tujuan-tujuan berikut:\n1. Mencerminkan komitmen merek terhadap keberlanjutan, praktik etis, dan pengelolaan lingkungan.\n2. Menarik bagi target audiens dengan menyampaikan rasa gaya, kualitas, dan tren.\n3. Membedakan merek dari pesaing di pasar fashion berkelanjutan.\n4. Menciptakan koneksi emosional yang kuat dengan konsumen dan menginspirasi mereka untuk membuat pilihan yang lebih ramah lingkungan."
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
      max_tokens: 2000,
      temperature: 1,
      system: "Tugas Anda adalah membuat brief desain yang komprehensif untuk identitas merek holistik berdasarkan spesifikasi yang diberikan. Identitas merek harus mencakup berbagai elemen seperti saran untuk nama merek, logo, palet warna, tipografi, gaya visual, nada suara, dan kepribadian merek secara keseluruhan. Pastikan bahwa semua elemen bekerja bersama secara harmonis untuk menciptakan pengalaman merek yang kohesif dan berkesan yang secara efektif mengkomunikasikan nilai-nilai merek, misi, dan proposisi penjualan unik kepada target audiensnya. Jadilah detail dan komprehensif serta berikan detail spesifik yang cukup bagi seseorang untuk menciptakan identitas merek yang benar-benar unik.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Spesifikasi merek:\nIni adalah merek yang berfokus pada penciptaan pakaian dan aksesori berkualitas tinggi dan bergaya menggunakan bahan ramah lingkungan dan metode produksi etis\nMerek ini menargetkan konsumen yang sadar lingkungan berusia 25-40 tahun yang menghargai fashion, keberlanjutan, dan tanggung jawab sosial.\nIdentitas merek harus mencapai tujuan-tujuan berikut:\n1. Mencerminkan komitmen merek terhadap keberlanjutan, praktik etis, dan pengelolaan lingkungan.\n2. Menarik bagi target audiens dengan menyampaikan rasa gaya, kualitas, dan tren.\n3. Membedakan merek dari pesaing di pasar fashion berkelanjutan.\n4. Menciptakan koneksi emosional yang kuat dengan konsumen dan menginspirasi mereka untuk membuat pilihan yang lebih ramah lingkungan."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>