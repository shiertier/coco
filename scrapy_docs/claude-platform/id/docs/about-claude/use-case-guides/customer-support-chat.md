# Agen dukungan pelanggan

Panduan ini menjelaskan cara memanfaatkan kemampuan percakapan canggih Claude untuk menangani pertanyaan pelanggan secara real-time, menyediakan dukungan 24/7, mengurangi waktu tunggu, dan mengelola volume dukungan tinggi dengan respons akurat dan interaksi positif.

---

## Sebelum membangun dengan Claude

### Tentukan apakah akan menggunakan Claude untuk obrolan dukungan

Berikut adalah beberapa indikator utama bahwa Anda harus menggunakan LLM seperti Claude untuk mengotomatisasi bagian dari proses dukungan pelanggan Anda:

  <section title="Volume tinggi pertanyaan berulang">

    Claude unggul dalam menangani sejumlah besar pertanyaan serupa secara efisien, membebaskan agen manusia untuk masalah yang lebih kompleks.
  
</section>
  <section title="Kebutuhan untuk sintesis informasi cepat">

    Claude dapat dengan cepat mengambil, memproses, dan menggabungkan informasi dari basis pengetahuan yang luas, sementara agen manusia mungkin memerlukan waktu untuk penelitian atau berkonsultasi dengan berbagai sumber.
  
</section>
  <section title="Persyaratan ketersediaan 24/7">

    Claude dapat menyediakan dukungan sepanjang waktu tanpa kelelahan, sedangkan mempekerjakan agen manusia untuk cakupan berkelanjutan dapat mahal dan menantang.
  
</section>
  <section title="Penskalaan cepat selama periode puncak">

    Claude dapat menangani peningkatan volume pertanyaan yang tiba-tiba tanpa perlu merekrut dan melatih staf tambahan.
  
</section>
  <section title="Suara merek yang konsisten">

    Anda dapat menginstruksikan Claude untuk secara konsisten mewakili nada dan nilai merek Anda, sedangkan agen manusia mungkin berbeda dalam gaya komunikasi mereka.
  
</section>

Beberapa pertimbangan untuk memilih Claude daripada LLM lainnya:

- Anda memprioritaskan percakapan yang alami dan bernuansa: Pemahaman bahasa Claude yang canggih memungkinkan percakapan yang lebih alami dan sadar konteks yang terasa lebih manusiawi daripada obrolan dengan LLM lainnya.
- Anda sering menerima pertanyaan yang kompleks dan terbuka: Claude dapat menangani berbagai topik dan pertanyaan tanpa menghasilkan respons yang sudah jadi atau memerlukan pemrograman ekstensif tentang permutasi ucapan pengguna.
- Anda membutuhkan dukungan multibahasa yang dapat diskalakan: Kemampuan multibahasa Claude memungkinkannya untuk terlibat dalam percakapan dalam lebih dari 200 bahasa tanpa perlu chatbot terpisah atau proses terjemahan ekstensif untuk setiap bahasa yang didukung.

### Tentukan interaksi obrolan ideal Anda

Uraikan interaksi pelanggan ideal untuk menentukan bagaimana dan kapan Anda mengharapkan pelanggan berinteraksi dengan Claude. Uraian ini akan membantu menentukan persyaratan teknis solusi Anda.

Berikut adalah contoh interaksi obrolan untuk dukungan pelanggan asuransi mobil:

* **Pelanggan**: Memulai pengalaman obrolan dukungan
   * **Claude**: Menyambut pelanggan dengan hangat dan memulai percakapan
* **Pelanggan**: Menanyakan tentang asuransi untuk mobil listrik baru mereka
   * **Claude**: Memberikan informasi relevan tentang cakupan kendaraan listrik
* **Pelanggan**: Mengajukan pertanyaan yang terkait dengan kebutuhan unik untuk asuransi kendaraan listrik
   * **Claude**: Merespons dengan jawaban yang akurat dan informatif serta menyediakan tautan ke sumber
* **Pelanggan**: Mengajukan pertanyaan di luar topik yang tidak terkait dengan asuransi atau mobil
   * **Claude**: Mengklarifikasi bahwa tidak membahas topik yang tidak terkait dan mengarahkan pengguna kembali ke asuransi mobil
* **Pelanggan**: Mengekspresikan minat pada penawaran asuransi
   * **Claude**: Mengajukan serangkaian pertanyaan untuk menentukan penawaran yang sesuai, beradaptasi dengan respons mereka
   * **Claude**: Mengirim permintaan untuk menggunakan alat API pembuatan penawaran bersama dengan informasi yang diperlukan yang dikumpulkan dari pengguna
   * **Claude**: Menerima informasi respons dari penggunaan alat API, mensintesis informasi menjadi respons alami, dan menyajikan penawaran yang disediakan kepada pengguna
* **Pelanggan**: Mengajukan pertanyaan lanjutan
   * **Claude**: Menjawab pertanyaan lanjutan sesuai kebutuhan
   * **Claude**: Memandu pelanggan ke langkah berikutnya dalam proses asuransi dan menutup percakapan

<Tip>Dalam contoh nyata yang Anda tulis untuk kasus penggunaan Anda sendiri, Anda mungkin merasa berguna untuk menulis kata-kata sebenarnya dalam interaksi ini sehingga Anda juga dapat merasakan nada ideal, panjang respons, dan tingkat detail yang Anda inginkan Claude miliki.</Tip>

### Pisahkan interaksi menjadi tugas-tugas unik

Obrolan dukungan pelanggan adalah kumpulan dari berbagai tugas yang berbeda, dari menjawab pertanyaan hingga pengambilan informasi hingga mengambil tindakan atas permintaan, dibungkus dalam satu interaksi pelanggan. Sebelum Anda mulai membangun, pisahkan interaksi pelanggan ideal Anda menjadi setiap tugas yang ingin Claude lakukan. Ini memastikan Anda dapat memberi prompt dan mengevaluasi Claude untuk setiap tugas, dan memberi Anda pemahaman yang baik tentang jangkauan interaksi yang perlu Anda pertimbangkan saat menulis kasus uji.

<Tip>Pelanggan kadang-kadang merasa berguna untuk memvisualisasikan ini sebagai bagan alur interaksi dari kemungkinan titik infleksi percakapan tergantung pada permintaan pengguna.</Tip>

Berikut adalah tugas-tugas utama yang terkait dengan contoh interaksi asuransi di atas:

1. Salam dan panduan umum
   - Menyambut pelanggan dengan hangat dan memulai percakapan
   - Memberikan informasi umum tentang perusahaan dan interaksi

2. Informasi Produk
   - Memberikan informasi tentang cakupan kendaraan listrik
   <Note>Ini akan memerlukan Claude memiliki informasi yang diperlukan dalam konteksnya, dan mungkin menyiratkan bahwa [integrasi RAG](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/retrieval_augmented_generation/guide.ipynb) diperlukan.</Note>
   - Menjawab pertanyaan yang terkait dengan kebutuhan asuransi kendaraan listrik yang unik
   - Menjawab pertanyaan lanjutan tentang penawaran atau detail asuransi
   - Menawarkan tautan ke sumber jika sesuai

3. Manajemen Percakapan
   - Tetap pada topik (asuransi mobil)
   - Mengarahkan kembali pertanyaan di luar topik ke subjek yang relevan

4. Pembuatan Penawaran
   - Mengajukan pertanyaan yang sesuai untuk menentukan kelayakan penawaran
   - Beradaptasi dengan pertanyaan berdasarkan respons pelanggan
   - Mengirimkan informasi yang dikumpulkan ke API pembuatan penawaran
   - Menyajikan penawaran yang disediakan kepada pelanggan

### Tetapkan kriteria kesuksesan

Bekerja dengan tim dukungan Anda untuk [menentukan kriteria kesuksesan yang jelas](/docs/id/test-and-evaluate/define-success) dan menulis [evaluasi terperinci](/docs/id/test-and-evaluate/develop-tests) dengan tolok ukur dan tujuan yang terukur.

Berikut adalah kriteria dan tolok ukur yang dapat digunakan untuk mengevaluasi seberapa berhasil Claude melakukan tugas-tugas yang ditentukan:

  <section title="Akurasi pemahaman pertanyaan">

    Metrik ini mengevaluasi seberapa akurat Claude memahami pertanyaan pelanggan di berbagai topik. Ukur ini dengan meninjau sampel percakapan dan menilai apakah Claude memiliki interpretasi yang benar tentang niat pelanggan, langkah-langkah kritis berikutnya, seperti apa resolusi yang berhasil, dan lainnya. Targetkan akurasi pemahaman 95% atau lebih tinggi.
  
</section>
  <section title="Relevansi respons">

    Ini menilai seberapa baik respons Claude mengatasi pertanyaan atau masalah spesifik pelanggan. Evaluasi serangkaian percakapan dan nilai relevansi setiap respons (menggunakan penilaian berbasis LLM untuk skala). Targetkan skor relevansi 90% atau lebih tinggi.
  
</section>
  <section title="Akurasi respons">

    Menilai kebenaran informasi perusahaan dan produk umum yang diberikan kepada pengguna, berdasarkan informasi yang diberikan kepada Claude dalam konteks. Targetkan akurasi 100% dalam informasi pengantar ini.
  
</section>
  <section title="Relevansi penyediaan kutipan">

    Lacak frekuensi dan relevansi tautan atau sumber yang ditawarkan. Targetkan menyediakan sumber yang relevan dalam 80% interaksi di mana informasi tambahan dapat bermanfaat.
  
</section>
  <section title="Kepatuhan topik">

    Ukur seberapa baik Claude tetap pada topik, seperti topik asuransi mobil dalam implementasi contoh kami. Targetkan 95% respons langsung terkait dengan asuransi mobil atau pertanyaan spesifik pelanggan.
  
</section>
  <section title="Efektivitas pembuatan konten">

    Ukur seberapa berhasil Claude dalam menentukan kapan harus menghasilkan konten informatif dan seberapa relevan konten tersebut. Misalnya, dalam implementasi kami, kami akan menentukan seberapa baik Claude memahami kapan harus menghasilkan penawaran dan seberapa akurat penawaran tersebut. Targetkan akurasi 100%, karena ini adalah informasi penting untuk interaksi pelanggan yang berhasil.
  
</section>
  <section title="Efisiensi eskalasi">

    Ini mengukur kemampuan Claude untuk mengenali kapan pertanyaan memerlukan intervensi manusia dan melakukan eskalasi dengan tepat. Lacak persentase percakapan yang di-eskalasi dengan benar versus yang seharusnya di-eskalasi tetapi tidak. Targetkan akurasi eskalasi 95% atau lebih tinggi.
  
</section>

Berikut adalah kriteria dan tolok ukur yang dapat digunakan untuk mengevaluasi dampak bisnis dari penggunaan Claude untuk dukungan:

  <section title="Pemeliharaan sentimen">

    Ini menilai kemampuan Claude untuk mempertahankan atau meningkatkan sentimen pelanggan sepanjang percakapan. Gunakan alat analisis sentimen untuk mengukur sentimen di awal dan akhir setiap percakapan. Targetkan sentimen yang dipertahankan atau ditingkatkan dalam 90% interaksi.
  
</section>
  <section title="Tingkat defleksi">

    Persentase pertanyaan pelanggan yang berhasil ditangani oleh chatbot tanpa intervensi manusia. Biasanya targetkan tingkat defleksi 70-80%, tergantung pada kompleksitas pertanyaan.
  
</section>
  <section title="Skor kepuasan pelanggan">

    Ukuran seberapa puas pelanggan dengan interaksi chatbot mereka. Biasanya dilakukan melalui survei pasca-interaksi. Targetkan skor CSAT 4 dari 5 atau lebih tinggi.
  
</section>
  <section title="Waktu penanganan rata-rata">

    Waktu rata-rata yang diperlukan chatbot untuk menyelesaikan pertanyaan. Ini bervariasi luas berdasarkan kompleksitas masalah, tetapi secara umum, targetkan AHT yang lebih rendah dibandingkan dengan agen manusia.
  
</section>

## Cara mengimplementasikan Claude sebagai agen layanan pelanggan

### Pilih model Claude yang tepat

Pilihan model tergantung pada pertukaran antara biaya, akurasi, dan waktu respons.

Untuk obrolan dukungan pelanggan, Claude Sonnet 4.5 cocok untuk menyeimbangkan kecerdasan, latensi, dan biaya. Namun, untuk kasus di mana Anda memiliki alur percakapan dengan beberapa prompt termasuk RAG, penggunaan alat, dan/atau prompt konteks panjang, Claude Haiku 4.5 mungkin lebih cocok untuk mengoptimalkan latensi.

### Bangun prompt yang kuat

Menggunakan Claude untuk dukungan pelanggan memerlukan Claude memiliki cukup arahan dan konteks untuk merespons dengan tepat, sambil memiliki fleksibilitas yang cukup untuk menangani berbagai pertanyaan pelanggan.

Mari kita mulai dengan menulis elemen-elemen prompt yang kuat, dimulai dengan prompt sistem:

```python
IDENTITY = """You are Eva, a friendly and knowledgeable AI assistant for Acme Insurance 
Company. Your role is to warmly welcome customers and provide information on 
Acme's insurance offerings, which include car insurance and electric car 
insurance. You can also help customers get quotes for their insurance needs."""
```

<Tip>Meskipun Anda mungkin tergoda untuk memasukkan semua informasi Anda di dalam prompt sistem sebagai cara untuk memisahkan instruksi dari percakapan pengguna, Claude sebenarnya bekerja paling baik dengan sebagian besar konten prompt ditulis di dalam giliran `User` pertama (dengan satu-satunya pengecualian adalah prompt peran). Baca lebih lanjut di [Memberikan Claude peran dengan prompt sistem](/docs/id/build-with-claude/prompt-engineering/system-prompts).</Tip>

Lebih baik memecah prompt kompleks menjadi subseksi dan menulis satu bagian pada satu waktu. Untuk setiap tugas, Anda mungkin menemukan kesuksesan yang lebih besar dengan mengikuti proses langkah demi langkah untuk menentukan bagian-bagian prompt yang Claude butuhkan untuk melakukan tugas dengan baik. Untuk contoh dukungan pelanggan asuransi mobil ini, kami akan menulis secara bertahap semua bagian untuk prompt dimulai dengan tugas "Salam dan panduan umum". Ini juga membuat debugging prompt Anda lebih mudah karena Anda dapat lebih cepat menyesuaikan bagian-bagian individual dari prompt keseluruhan.

Kami akan menempatkan semua potongan ini dalam file bernama `config.py`.

```python
STATIC_GREETINGS_AND_GENERAL = """
<static_context>
Acme Auto Insurance: Your Trusted Companion on the Road

About:
At Acme Insurance, we understand that your vehicle is more than just a mode of transportation—it's your ticket to life's adventures. 
Since 1985, we've been crafting auto insurance policies that give drivers the confidence to explore, commute, and travel with peace of mind.
Whether you're navigating city streets or embarking on cross-country road trips, Acme is there to protect you and your vehicle. 
Our innovative auto insurance policies are designed to adapt to your unique needs, covering everything from fender benders to major collisions.
With Acme's award-winning customer service and swift claim resolution, you can focus on the joy of driving while we handle the rest. 
We're not just an insurance provider—we're your co-pilot in life's journeys.
Choose Acme Auto Insurance and experience the assurance that comes with superior coverage and genuine care. Because at Acme, we don't just 
insure your car—we fuel your adventures on the open road.

Note: We also offer specialized coverage for electric vehicles, ensuring that drivers of all car types can benefit from our protection.

Acme Insurance offers the following products:
- Car insurance
- Electric car insurance
- Two-wheeler insurance

Business hours: Monday-Friday, 9 AM - 5 PM EST
Customer service number: 1-800-123-4567
</static_context>
"""
```

Kami kemudian akan melakukan hal yang sama untuk informasi asuransi mobil dan asuransi mobil listrik kami.

```python
STATIC_CAR_INSURANCE="""
<static_context>
Car Insurance Coverage:
Acme's car insurance policies typically cover:
1. Liability coverage: Pays for bodily injury and property damage you cause to others.
2. Collision coverage: Pays for damage to your car in an accident.
3. Comprehensive coverage: Pays for damage to your car from non-collision incidents.
4. Medical payments coverage: Pays for medical expenses after an accident.
5. Uninsured/underinsured motorist coverage: Protects you if you're hit by a driver with insufficient insurance.

Optional coverages include:
- Rental car reimbursement
- Roadside assistance
- New car replacement
</static_context>
"""

STATIC_ELECTRIC_CAR_INSURANCE="""
<static_context>
Electric Car Insurance:
Our specialized electric car insurance goes beyond traditional auto coverage, 
offering tailored protection for your high-tech investment. In addition to standard coverages, we offer:
- Safeguarding expensive battery systems
- Covering home charging equipment
- Adapting to the rapidly evolving world of electric vehicles

Whether you're driving a Rivian, Nissan Leaf, Chevrolet Bolt, 
or any other electric vehicle, our knowledgeable agents are ready to craft a policy 
that gives you the confidence to enjoy every electron-powered mile.
</static_context>
"""
```

Sekarang kita memiliki konten statis kami, mari tambahkan setidaknya 4-5 sampel interaksi "baik" untuk memandu respons Claude. Contoh-contoh ini harus mewakili interaksi pelanggan ideal Anda dan dapat mencakup guardrail, panggilan alat, dll.

```python
EXAMPLES="""
Here are a few examples of how you can interact with customers:

<example 1>
H: Hi, do you offer commercial insurance for small businesses?

A: Ah! Unfortunately, we don't offer commercial insurance at this time. 
However, we do provide a range of personal insurance products including car insurance and electric car insurance. 
Would you like to know more about these offerings?
</example 1>

<example 2>
H: Hi, what are your business hours?

A: Our business hours are Monday to Friday, 9 AM to 5 PM EST. 
If you have any questions or need assistance, feel free to reach out during those times. 
How else can I assist you today?
</example 2>

<example 3>
H: I'm shopping for car insurance. What does Acme offer?

A: Great choice! Acme's car insurance policies typically include:
- Liability coverage: Protects you if you're at fault in an accident
- Collision coverage: Pays for damage to your car in an accident
- Comprehensive coverage: Covers non-collision incidents like theft or vandalism
- Medical payments coverage: Pays for medical expenses after an accident
- Uninsured/underinsured motorist coverage: Protects you from drivers with insufficient insurance

We also offer optional coverages like rental car reimbursement, roadside assistance, and new car replacement. 
Would you like more details on any of these coverages, or do you have specific questions about insuring your vehicle?
</example 3>

<example 4>
H: I'd like to hear about your electric car insurance.

A: Certainly! Our electric car insurance is specially designed to meet the unique needs of electric vehicle owners. 
In addition to all the standard coverages we offer for regular cars, our electric car policies include:

- Battery coverage: Protection against damage or degradation of your vehicle's expensive battery system
- Tech package: Covers repairs or replacement of advanced electronics and onboard computer systems
- Charging station coverage: Protects both home and portable charging equipment
- Cybersecurity protection: Coverage against hacking or digital threats
- Specialized roadside assistance: Includes flatbed towing to protect the vehicle's undercarriage

We cover a wide range of electric vehicles, including popular models from Tesla, Rivian, Nissan, Chevrolet, and more. 
Is there a specific electric vehicle you're looking to insure, or would you like more details on any of these coverages?
</example 4>

<example 5>
H: I'd like to get a quote for my car insurance.

A: Certainly! I'd be happy to help you get a quote for your car insurance. 
To provide you with an accurate quote, I'll need to collect some information about your vehicle and the primary driver. 
Let's start with the basics:

1. What is the make and model of your vehicle?
2. What year was it manufactured?
3. Approximately how many miles have you driven?
4. What is the age of the primary driver?

Once you provide this information, I'll use our quoting tool to generate a personalized insurance quote for you.
</example 5>
"""
```

Anda juga akan ingin menyertakan instruksi penting yang menguraikan Do's dan Don'ts tentang cara Claude harus berinteraksi dengan pelanggan. 
Ini mungkin diambil dari guardrail merek atau kebijakan dukungan. 

```python
ADDITIONAL_GUARDRAILS = """Please adhere to the following guardrails:
1. Only provide information about insurance types listed in our offerings.
2. If asked about an insurance type we don't offer, politely state 
that we don't provide that service.
3. Do not speculate about future product offerings or company plans.
4. Don't make promises or enter into agreements it's not authorized to make.
You only provide information and guidance.
5. Do not mention any competitor's products or services.
"""
```

Sekarang mari kita gabungkan semua bagian ini menjadi satu string untuk digunakan sebagai prompt kami.

```python
TASK_SPECIFIC_INSTRUCTIONS = ' '.join([
   STATIC_GREETINGS_AND_GENERAL,
   STATIC_CAR_INSURANCE,
   STATIC_ELECTRIC_CAR_INSURANCE,
   EXAMPLES,
   ADDITIONAL_GUARDRAILS,
])
```

### Tambahkan kemampuan dinamis dan agentic dengan penggunaan alat

Claude mampu mengambil tindakan dan mengambil informasi secara dinamis menggunakan fungsionalitas penggunaan alat sisi klien. Mulai dengan membuat daftar alat eksternal atau API apa pun yang harus digunakan prompt.

Untuk contoh ini, kami akan mulai dengan satu alat untuk menghitung penawaran. 

<Tip>Sebagai pengingat, alat ini tidak akan melakukan perhitungan sebenarnya, itu hanya akan memberi sinyal kepada aplikasi bahwa alat harus digunakan dengan argumen apa pun yang ditentukan.</Tip>

Contoh kalkulator penawaran asuransi:

```python
TOOLS = [{
  "name": "get_quote",
  "description": "Calculate the insurance quote based on user input. Returned value is per month premium.",
  "input_schema": {
    "type": "object",
    "properties": {
      "make": {"type": "string", "description": "The make of the vehicle."},
      "model": {"type": "string", "description": "The model of the vehicle."},
      "year": {"type": "integer", "description": "The year the vehicle was manufactured."},
      "mileage": {"type": "integer", "description": "The mileage on the vehicle."},
      "driver_age": {"type": "integer", "description": "The age of the primary driver."}
    },
    "required": ["make", "model", "year", "mileage", "driver_age"]
  }
}]

def get_quote(make, model, year, mileage, driver_age):
    """Returns the premium per month in USD"""
    # You can call an http endpoint or a database to get the quote.
    # Here, we simulate a delay of 1 seconds and return a fixed quote of 100.
    time.sleep(1)
    return 100
```

### Terapkan prompt Anda

Sulit untuk mengetahui seberapa baik prompt Anda bekerja tanpa menerapkannya dalam pengaturan produksi uji dan [menjalankan evaluasi](/docs/id/test-and-evaluate/develop-tests) jadi mari kita bangun aplikasi kecil menggunakan prompt kami, SDK Anthropic, dan streamlit untuk antarmuka pengguna.

Dalam file bernama `chatbot.py`, mulai dengan menyiapkan kelas ChatBot, yang akan merangkum interaksi dengan SDK Anthropic. 

Kelas harus memiliki dua metode utama: `generate_message` dan `process_user_input`. 

```python
from anthropic import Anthropic
from config import IDENTITY, TOOLS, MODEL, get_quote
from dotenv import load_dotenv

load_dotenv()

class ChatBot:
   def __init__(self, session_state):
       self.anthropic = Anthropic()
       self.session_state = session_state

   def generate_message(
       self,
       messages,
       max_tokens,
   ):
       try:
           response = self.anthropic.messages.create(
               model=MODEL,
               system=IDENTITY,
               max_tokens=max_tokens,
               messages=messages,
               tools=TOOLS,
           )
           return response
       except Exception as e:
           return {"error": str(e)}

   def process_user_input(self, user_input):
       self.session_state.messages.append({"role": "user", "content": user_input})

       response_message = self.generate_message(
           messages=self.session_state.messages,
           max_tokens=2048,
       )

       if "error" in response_message:
           return f"An error occurred: {response_message['error']}"

       if response_message.content[-1].type == "tool_use":
           tool_use = response_message.content[-1]
           func_name = tool_use.name
           func_params = tool_use.input
           tool_use_id = tool_use.id

           result = self.handle_tool_use(func_name, func_params)
           self.session_state.messages.append(
               {"role": "assistant", "content": response_message.content}
           )
           self.session_state.messages.append({
               "role": "user",
               "content": [{
                   "type": "tool_result",
                   "tool_use_id": tool_use_id,
                   "content": f"{result}",
               }],
           })

           follow_up_response = self.generate_message(
               messages=self.session_state.messages,
               max_tokens=2048,
           )

           if "error" in follow_up_response:
               return f"An error occurred: {follow_up_response['error']}"

           response_text = follow_up_response.content[0].text
           self.session_state.messages.append(
               {"role": "assistant", "content": response_text}
           )
           return response_text
      
       elif response_message.content[0].type == "text":
           response_text = response_message.content[0].text
           self.session_state.messages.append(
               {"role": "assistant", "content": response_text}
           )
           return response_text
      
       else:
           raise Exception("An error occurred: Unexpected response type")

   def handle_tool_use(self, func_name, func_params):
       if func_name == "get_quote":
           premium = get_quote(**func_params)
           return f"Quote generated: ${premium:.2f} per month"
      
       raise Exception("An unexpected tool was used")
```

### Bangun antarmuka pengguna Anda

Uji penerapan kode ini dengan Streamlit menggunakan metode utama. Fungsi `main()` ini menyiapkan antarmuka obrolan berbasis Streamlit.

Kami akan melakukan ini dalam file bernama `app.py`

```python
import streamlit as st
from chatbot import ChatBot
from config import TASK_SPECIFIC_INSTRUCTIONS

def main():
   st.title("Chat with Eva, Acme Insurance Company's Assistant🤖")

   if "messages" not in st.session_state:
       st.session_state.messages = [
           {'role': "user", "content": TASK_SPECIFIC_INSTRUCTIONS},
           {'role': "assistant", "content": "Understood"},
       ]

   chatbot = ChatBot(st.session_state)

   # Display user and assistant messages skipping the first two
   for message in st.session_state.messages[2:]:
       # ignore tool use blocks
       if isinstance(message["content"], str):
           with st.chat_message(message["role"]):
               st.markdown(message["content"])

   if user_msg := st.chat_input("Type your message here..."):
       st.chat_message("user").markdown(user_msg)

       with st.chat_message("assistant"):
           with st.spinner("Eva is thinking..."):
               response_placeholder = st.empty()
               full_response = chatbot.process_user_input(user_msg)
               response_placeholder.markdown(full_response)

if __name__ == "__main__":
   main()
```

Jalankan program dengan:

```
streamlit run app.py
```

### Evaluasi prompt Anda

Prompting sering kali memerlukan pengujian dan optimasi agar siap produksi. Untuk menentukan kesiapan solusi Anda, evaluasi kinerja chatbot menggunakan proses sistematis yang menggabungkan metode kuantitatif dan kualitatif. Membuat [evaluasi empiris yang kuat](/docs/id/test-and-evaluate/develop-tests#building-evals-and-test-cases) berdasarkan kriteria kesuksesan yang ditentukan akan memungkinkan Anda mengoptimalkan prompt Anda. 

<Tip>[Konsol Claude](/dashboard) sekarang menampilkan alat Evaluasi yang memungkinkan Anda menguji prompt Anda dalam berbagai skenario.</Tip>

### Tingkatkan kinerja

Dalam skenario kompleks, mungkin berguna untuk mempertimbangkan strategi tambahan untuk meningkatkan kinerja di luar [teknik prompt engineering](/docs/id/build-with-claude/prompt-engineering/overview) standar & [strategi implementasi guardrail](/docs/id/test-and-evaluate/strengthen-guardrails/reduce-hallucinations). Berikut adalah beberapa skenario umum:

#### Kurangi latensi konteks panjang dengan RAG

Ketika menangani sejumlah besar konteks statis dan dinamis, memasukkan semua informasi dalam prompt dapat menyebabkan biaya tinggi, waktu respons lebih lambat, dan mencapai batas jendela konteks. Dalam skenario ini, mengimplementasikan teknik Retrieval Augmented Generation (RAG) dapat secara signifikan meningkatkan kinerja dan efisiensi.

Dengan menggunakan [model embedding seperti Voyage](/docs/id/build-with-claude/embeddings) untuk mengonversi informasi menjadi representasi vektor, Anda dapat membuat sistem yang lebih dapat diskalakan dan responsif. Pendekatan ini memungkinkan pengambilan informasi yang relevan secara dinamis berdasarkan kueri saat ini, daripada memasukkan semua konteks yang mungkin dalam setiap prompt.

Mengimplementasikan RAG untuk kasus penggunaan dukungan [resep RAG](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb) telah terbukti meningkatkan akurasi, mengurangi waktu respons, dan mengurangi biaya API dalam sistem dengan persyaratan konteks yang luas.

#### Integrasikan data real-time dengan penggunaan alat

Ketika menangani pertanyaan yang memerlukan informasi real-time, seperti saldo akun atau detail kebijakan, pendekatan RAG berbasis embedding tidak cukup. Sebaliknya, Anda dapat memanfaatkan penggunaan alat untuk secara signifikan meningkatkan kemampuan chatbot Anda untuk memberikan respons yang akurat dan real-time. Misalnya, Anda dapat menggunakan penggunaan alat untuk mencari informasi pelanggan, mengambil detail pesanan, dan membatalkan pesanan atas nama pelanggan.

Pendekatan ini, [dijelaskan dalam resep agen layanan pelanggan penggunaan alat kami](https://github.com/anthropics/anthropic-cookbook/blob/main/tool_use/customer_service_agent.ipynb), memungkinkan Anda mengintegrasikan data langsung dengan mulus ke dalam respons Claude dan memberikan pengalaman pelanggan yang lebih personal dan efisien.

#### Perkuat guardrail input dan output

Saat menerapkan chatbot, terutama dalam skenario layanan pelanggan, sangat penting untuk mencegah risiko yang terkait dengan penyalahgunaan, pertanyaan di luar cakupan, dan respons yang tidak sesuai. Meskipun Claude secara inheren tahan terhadap skenario seperti itu, berikut adalah langkah-langkah tambahan untuk memperkuat guardrail chatbot Anda:

- [Kurangi halusinasi](/docs/id/test-and-evaluate/strengthen-guardrails/reduce-hallucinations): Implementasikan mekanisme pemeriksaan fakta dan [kutipan](https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb) untuk mendasarkan respons pada informasi yang disediakan.
- Verifikasi silang informasi: Verifikasi bahwa respons agen selaras dengan kebijakan perusahaan Anda dan fakta yang diketahui.
- Hindari komitmen kontraktual: Pastikan agen tidak membuat janji atau memasuki perjanjian yang tidak diotorisasi untuk dibuat.
- [Mitigasi jailbreak](/docs/id/test-and-evaluate/strengthen-guardrails/mitigate-jailbreaks): Gunakan metode seperti layar kerusakan dan validasi input untuk mencegah pengguna mengeksploitasi kerentanan model, bertujuan untuk menghasilkan konten yang tidak sesuai.
- Hindari menyebutkan pesaing: Implementasikan filter penyebutan pesaing untuk mempertahankan fokus merek dan tidak menyebutkan produk atau layanan pesaing apa pun.
- [Jaga Claude tetap dalam karakter](/docs/id/test-and-evaluate/strengthen-guardrails/keep-claude-in-character): Cegah Claude mengubah gaya konteks mereka, bahkan selama interaksi yang panjang dan kompleks.
- Hapus Informasi Pengenal Pribadi (PII): Kecuali secara eksplisit diperlukan dan diotorisasi, hapus PII apa pun dari respons.

#### Kurangi waktu respons yang dirasakan dengan streaming

Ketika menangani respons yang berpotensi panjang, mengimplementasikan streaming dapat secara signifikan meningkatkan keterlibatan dan kepuasan pengguna. Dalam skenario ini, pengguna menerima jawaban secara progresif daripada menunggu seluruh respons dihasilkan.

Berikut adalah cara mengimplementasikan streaming:
1. Gunakan [API Streaming Anthropic](/docs/id/build-with-claude/streaming) untuk mendukung respons streaming.
2. Siapkan frontend Anda untuk menangani potongan teks yang masuk.
3. Tampilkan setiap potongan saat tiba, mensimulasikan pengetikan real-time.
4. Implementasikan mekanisme untuk menyimpan respons lengkap, memungkinkan pengguna untuk melihatnya jika mereka menavigasi dan kembali.

Dalam beberapa kasus, streaming memungkinkan penggunaan model yang lebih canggih dengan latensi dasar yang lebih tinggi, karena tampilan progresif mengurangi dampak waktu pemrosesan yang lebih lama.

#### Skalakan Chatbot Anda

Seiring dengan meningkatnya kompleksitas Chatbot Anda, arsitektur aplikasi Anda dapat berkembang untuk cocok. Sebelum Anda menambahkan lapisan lebih lanjut ke arsitektur Anda, pertimbangkan opsi berikut yang kurang lengkap:

- Pastikan Anda memanfaatkan prompt Anda sebaik-baiknya dan mengoptimalkan melalui prompt engineering. Gunakan [panduan prompt engineering](/docs/id/build-with-claude/prompt-engineering/overview) kami untuk menulis prompt yang paling efektif.
- Tambahkan [alat](/docs/id/build-with-claude/tool-use) tambahan ke prompt (yang dapat mencakup [rantai prompt](/docs/id/build-with-claude/prompt-engineering/chain-prompts)) dan lihat apakah Anda dapat mencapai fungsionalitas yang diperlukan.

Jika Chatbot Anda menangani tugas-tugas yang sangat beragam, Anda mungkin ingin mempertimbangkan penambahan [pengklasifikasi niat terpisah](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/classification/guide.ipynb) untuk merutekan pertanyaan pelanggan awal. Untuk aplikasi yang ada, ini akan melibatkan pembuatan pohon keputusan yang akan merutekan pertanyaan pelanggan melalui pengklasifikasi dan kemudian ke percakapan khusus (dengan set alat dan prompt sistem mereka sendiri). Perhatikan, metode ini memerlukan panggilan tambahan ke Claude yang dapat meningkatkan latensi.

### Integrasikan Claude ke dalam alur kerja dukungan Anda

Meskipun contoh kami telah fokus pada fungsi Python yang dapat dipanggil dalam lingkungan Streamlit, menerapkan Claude untuk chatbot dukungan real-time memerlukan layanan API. 

Berikut adalah cara Anda dapat mendekati ini:

1. Buat pembungkus API: Kembangkan pembungkus API sederhana di sekitar fungsi klasifikasi Anda. Misalnya, Anda dapat menggunakan Flask API atau Fast API untuk membungkus kode Anda menjadi Layanan HTTP. Layanan HTTP Anda dapat menerima input pengguna dan mengembalikan respons Asisten secara keseluruhan. Dengan demikian, layanan Anda dapat memiliki karakteristik berikut:
   - Server-Sent Events (SSE): SSE memungkinkan streaming respons real-time dari server ke klien. Ini sangat penting untuk memberikan pengalaman yang mulus dan interaktif saat bekerja dengan LLM.
   - Caching: Mengimplementasikan caching dapat secara signifikan meningkatkan waktu respons dan mengurangi panggilan API yang tidak perlu.
   - Retensi konteks: Mempertahankan konteks ketika pengguna menavigasi dan kembali penting untuk kontinuitas dalam percakapan.

2. Bangun antarmuka web: Implementasikan UI web yang ramah pengguna untuk berinteraksi dengan agen bertenaga Claude.

<CardGroup cols={2}>
  <Card title="Retrieval Augmented Generation (RAG) cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/skills/retrieval_augmented_generation/guide.ipynb">
    Kunjungi resep cookbook RAG kami untuk kode contoh lebih lanjut dan panduan terperinci.
  </Card>
  <Card title="Citations cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb">
    Jelajahi resep cookbook Kutipan kami untuk cara memastikan akurasi dan penjelasan informasi.
  </Card>
</CardGroup>