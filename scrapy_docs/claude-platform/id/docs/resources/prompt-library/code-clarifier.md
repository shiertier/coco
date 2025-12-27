# Penjelas kode

Sederhanakan dan jelaskan kode kompleks dalam bahasa yang mudah dipahami.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah mengambil potongan kode yang diberikan dan menjelaskannya dalam bahasa yang sederhana dan mudah dipahami. Uraikan fungsionalitas, tujuan, dan komponen utama kode tersebut. Gunakan analogi, contoh, dan istilah yang mudah dipahami untuk membuat penjelasan dapat diakses oleh seseorang dengan pengetahuan coding minimal. Hindari menggunakan jargon teknis kecuali benar-benar diperlukan, dan berikan penjelasan yang jelas untuk setiap jargon yang digunakan. Tujuannya adalah membantu pembaca memahami apa yang dilakukan kode dan bagaimana cara kerjanya pada tingkat tinggi. |
| User   | import random def bubble*sort(arr): n = len(arr) for i in range(n-1): for j in range(n-i-1): if arr[j] > arr[j+1]: arr[j], arr[j+1] = arr[j+1], arr[j] return arr numbers = [random.randint(1, 100) for * in range(10)] print("Unsorted array:", numbers) sorted_numbers = bubble_sort(numbers) print("Sorted array:", sorted_numbers) |

### Contoh Output

> Kode ini mendemonstrasikan implementasi algoritma bubble sort, yang digunakan untuk mengurutkan daftar angka dalam urutan naik. Mari kita uraikan langkah demi langkah:
>
> 1.  Kode dimulai dengan mengimpor modul 'random', yang akan digunakan untuk menghasilkan angka acak untuk contoh.
> 2.  Fungsi 'bubble_sort' didefinisikan, yang mengambil array (daftar) angka sebagai input. Fungsi ini akan melakukan operasi pengurutan.
> 3.  Di dalam fungsi 'bubble_sort', variabel 'n' diberi nilai panjang array input. Ini akan digunakan untuk menentukan jumlah iterasi yang diperlukan untuk pengurutan.
> 4.  Loop pertama, yang dikontrol oleh variabel 'i', beriterasi 'n-1' kali. Ini karena setelah 'n-1' kali lewat, angka terbesar akan berada di akhir array, jadi kita tidak perlu membandingkannya lagi.
> 5.  Loop kedua, yang dikontrol oleh variabel 'j', beriterasi 'n-i-1' kali. Ini karena setelah setiap kali lewat, angka terbesar di antara elemen yang belum diurutkan akan ditempatkan di akhir, jadi kita tidak perlu membandingkannya di lewatan berikutnya.
> 6.  Di dalam loop kedua, ada pernyataan if yang membandingkan elemen yang berdekatan dalam array. Jika elemen pada indeks 'j' lebih besar dari elemen pada indeks 'j+1', mereka ditukar. Ini memastikan bahwa elemen yang lebih besar bergerak menuju akhir array.
> 7.  Pertukaran dilakukan menggunakan sintaks Python 'arr\[j\], arr\[j+1\] = arr\[j+1\], arr\[j\]', yang secara bersamaan menetapkan nilai 'arr\[j+1\]' ke 'arr\[j\]' dan 'arr\[j\]' ke 'arr\[j+1\]'.
> 8.  Setelah semua iterasi selesai, array yang diurutkan dikembalikan oleh fungsi 'bubble_sort'.
> 9.  Dalam kode contoh, daftar 10 angka acak antara 1 dan 100 dihasilkan menggunakan list comprehension: '\[random.randint(1, 100) for \_ in range(10)\]'. Garis bawah '\_' digunakan sebagai variabel placeholder karena kita tidak memerlukan nilai indeks.
> 10. Array yang belum diurutkan dicetak untuk menunjukkan urutan awal angka-angka.
>
> 11. Fungsi 'bubble_sort' dipanggil dengan array 'numbers' sebagai input, dan array yang diurutkan disimpan dalam variabel 'sorted_numbers'.
>
> 12. Akhirnya, array yang diurutkan dicetak untuk menampilkan angka-angka dalam urutan naik.
>
> Dalam istilah yang lebih sederhana, bayangkan Anda memiliki daftar angka yang tidak berurutan. Algoritma bubble sort bekerja dengan membandingkan setiap angka dengan yang di sebelahnya, dan jika mereka dalam urutan yang salah, algoritma menukarnya. Proses ini diulang beberapa kali sampai seluruh daftar diurutkan. Ini seperti gelembung yang naik ke permukaan, dengan angka-angka yang lebih besar secara bertahap bergerak menuju akhir daftar di setiap lewatan.

---

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
        max_tokens=1000,
        temperature=0,
        system="Tugas Anda adalah mengambil potongan kode yang diberikan dan menjelaskannya dalam bahasa yang sederhana dan mudah dipahami. Uraikan fungsionalitas, tujuan, dan komponen utama kode tersebut. Gunakan analogi, contoh, dan istilah yang mudah dipahami untuk membuat penjelasan dapat diakses oleh seseorang dengan pengetahuan coding minimal. Hindari menggunakan jargon teknis kecuali benar-benar diperlukan, dan berikan penjelasan yang jelas untuk setiap jargon yang digunakan. Tujuannya adalah membantu pembaca memahami apa yang dilakukan kode dan bagaimana cara kerjanya pada tingkat tinggi.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
      temperature: 0,
      system: "Tugas Anda adalah mengambil potongan kode yang diberikan dan menjelaskannya dalam bahasa yang sederhana dan mudah dipahami. Uraikan fungsionalitas, tujuan, dan komponen utama kode tersebut. Gunakan analogi, contoh, dan istilah yang mudah dipahami untuk membuat penjelasan dapat diakses oleh seseorang dengan pengetahuan coding minimal. Hindari menggunakan jargon teknis kecuali benar-benar diperlukan, dan berikan penjelasan yang jelas untuk setiap jargon yang digunakan. Tujuannya adalah membantu pembaca memahami apa yang dilakukan kode dan bagaimana cara kerjanya pada tingkat tinggi.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
        temperature=0,
        system="Tugas Anda adalah mengambil potongan kode yang diberikan dan menjelaskannya dalam bahasa yang sederhana dan mudah dipahami. Uraikan fungsionalitas, tujuan, dan komponen utama kode tersebut. Gunakan analogi, contoh, dan istilah yang mudah dipahami untuk membuat penjelasan dapat diakses oleh seseorang dengan pengetahuan coding minimal. Hindari menggunakan jargon teknis kecuali benar-benar diperlukan, dan berikan penjelasan yang jelas untuk setiap jargon yang digunakan. Tujuannya adalah membantu pembaca memahami apa yang dilakukan kode dan bagaimana cara kerjanya pada tingkat tinggi.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
      temperature: 0,
      system: "Tugas Anda adalah mengambil potongan kode yang diberikan dan menjelaskannya dalam bahasa yang sederhana dan mudah dipahami. Uraikan fungsionalitas, tujuan, dan komponen utama kode tersebut. Gunakan analogi, contoh, dan istilah yang mudah dipahami untuk membuat penjelasan dapat diakses oleh seseorang dengan pengetahuan coding minimal. Hindari menggunakan jargon teknis kecuali benar-benar diperlukan, dan berikan penjelasan yang jelas untuk setiap jargon yang digunakan. Tujuannya adalah membantu pembaca memahami apa yang dilakukan kode dan bagaimana cara kerjanya pada tingkat tinggi.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
        temperature=0,
        system="Tugas Anda adalah mengambil potongan kode yang diberikan dan menjelaskannya dalam bahasa yang sederhana dan mudah dipahami. Uraikan fungsionalitas, tujuan, dan komponen utama kode tersebut. Gunakan analogi, contoh, dan istilah yang mudah dipahami untuk membuat penjelasan dapat diakses oleh seseorang dengan pengetahuan coding minimal. Hindari menggunakan jargon teknis kecuali benar-benar diperlukan, dan berikan penjelasan yang jelas untuk setiap jargon yang digunakan. Tujuannya adalah membantu pembaca memahami apa yang dilakukan kode dan bagaimana cara kerjanya pada tingkat tinggi.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
      temperature: 0,
      system: "Tugas Anda adalah mengambil potongan kode yang diberikan dan menjelaskannya dalam bahasa yang sederhana dan mudah dipahami. Uraikan fungsionalitas, tujuan, dan komponen utama kode tersebut. Gunakan analogi, contoh, dan istilah yang mudah dipahami untuk membuat penjelasan dapat diakses oleh seseorang dengan pengetahuan coding minimal. Hindari menggunakan jargon teknis kecuali benar-benar diperlukan, dan berikan penjelasan yang jelas untuk setiap jargon yang digunakan. Tujuannya adalah membantu pembaca memahami apa yang dilakukan kode dan bagaimana cara kerjanya pada tingkat tinggi.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>