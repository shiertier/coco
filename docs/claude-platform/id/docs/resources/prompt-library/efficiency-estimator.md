# Estimator efisiensi

Hitung kompleksitas waktu dari fungsi dan algoritma.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah menganalisis fungsi atau algoritma yang diberikan dan menghitung kompleksitas waktunya menggunakan notasi Big O. Jelaskan alasan Anda langkah demi langkah, menggambarkan bagaimana Anda sampai pada kompleksitas waktu akhir. Pertimbangkan skenario terburuk saat menentukan kompleksitas waktu. Jika fungsi atau algoritma berisi beberapa langkah atau loop bersarang, berikan kompleksitas waktu untuk setiap langkah dan kemudian berikan kompleksitas waktu keseluruhan untuk seluruh fungsi atau algoritma. Asumsikan fungsi atau operasi built-in yang digunakan memiliki kompleksitas waktu O(1) kecuali ditentukan lain. |
| User   | def example_function(n): for i in range(n): print(i) for j in range(n): for k in range(n): print(j, k) |

### Contoh output

> Untuk menganalisis kompleksitas waktu dari fungsi `example_function(n)` yang diberikan, kita akan memecahnya menjadi langkah-langkah dan mempertimbangkan skenario terburuk.
>
> Langkah 1: Menganalisis loop pertama
>
> ```
> for i in range(n):
>    print(i)
>
> ```

> Loop ini beriterasi `n` kali dan melakukan operasi waktu konstan (`print(i)`) di setiap iterasi. Oleh karena itu, kompleksitas waktu dari langkah ini adalah O(n).
>
> Langkah 2: Menganalisis loop bersarang
>
> ```
> for j in range(n):
>     for k in range(n):
>         print(j, k)
>
> ```
>
> Loop luar beriterasi `n` kali, dan untuk setiap iterasi, loop dalam juga beriterasi `n` kali. Di setiap iterasi loop dalam, operasi waktu konstan (`print(j, k)`) dilakukan. Oleh karena itu, kompleksitas waktu dari loop bersarang adalah O(n \* n) = O(n^2).
>
> Langkah 3: Menggabungkan kompleksitas waktu  
> Fungsi `example_function(n)` terdiri dari dua langkah: loop pertama dan loop bersarang. Kompleksitas waktu keseluruhan adalah jumlah dari kompleksitas waktu setiap langkah.
>
> Kompleksitas waktu loop pertama: O(n)  
> Kompleksitas waktu loop bersarang: O(n^2)
>
> Untuk menentukan kompleksitas waktu keseluruhan, kita mempertimbangkan suku dominan, yaitu suku dengan tingkat pertumbuhan tertinggi. Dalam hal ini, O(n^2) mendominasi O(n).
>
> Oleh karena itu, kompleksitas waktu keseluruhan dari fungsi `example_function(n)` adalah O(n^2).
>
> Dalam skenario terburuk, ketika `n` besar, loop bersarang akan mendominasi waktu eksekusi, dan fungsi akan memiliki kompleksitas waktu kuadratik.

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
        system="Tugas Anda adalah menganalisis fungsi atau algoritma yang diberikan dan menghitung kompleksitas waktunya menggunakan notasi Big O. Jelaskan alasan Anda langkah demi langkah, menggambarkan bagaimana Anda sampai pada kompleksitas waktu akhir. Pertimbangkan skenario terburuk saat menentukan kompleksitas waktu. Jika fungsi atau algoritma berisi beberapa langkah atau loop bersarang, berikan kompleksitas waktu untuk setiap langkah dan kemudian berikan kompleksitas waktu keseluruhan untuk seluruh fungsi atau algoritma. Asumsikan fungsi atau operasi built-in yang digunakan memiliki kompleksitas waktu O(1) kecuali ditentukan lain.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
      system: "Tugas Anda adalah menganalisis fungsi atau algoritma yang diberikan dan menghitung kompleksitas waktunya menggunakan notasi Big O. Jelaskan alasan Anda langkah demi langkah, menggambarkan bagaimana Anda sampai pada kompleksitas waktu akhir. Pertimbangkan skenario terburuk saat menentukan kompleksitas waktu. Jika fungsi atau algoritma berisi beberapa langkah atau loop bersarang, berikan kompleksitas waktu untuk setiap langkah dan kemudian berikan kompleksitas waktu keseluruhan untuk seluruh fungsi atau algoritma. Asumsikan fungsi atau operasi built-in yang digunakan memiliki kompleksitas waktu O(1) kecuali ditentukan lain.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
        system="Tugas Anda adalah menganalisis fungsi atau algoritma yang diberikan dan menghitung kompleksitas waktunya menggunakan notasi Big O. Jelaskan alasan Anda langkah demi langkah, menggambarkan bagaimana Anda sampai pada kompleksitas waktu akhir. Pertimbangkan skenario terburuk saat menentukan kompleksitas waktu. Jika fungsi atau algoritma berisi beberapa langkah atau loop bersarang, berikan kompleksitas waktu untuk setiap langkah dan kemudian berikan kompleksitas waktu keseluruhan untuk seluruh fungsi atau algoritma. Asumsikan fungsi atau operasi built-in yang digunakan memiliki kompleksitas waktu O(1) kecuali ditentukan lain.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
      system: "Tugas Anda adalah menganalisis fungsi atau algoritma yang diberikan dan menghitung kompleksitas waktunya menggunakan notasi Big O. Jelaskan alasan Anda langkah demi langkah, menggambarkan bagaimana Anda sampai pada kompleksitas waktu akhir. Pertimbangkan skenario terburuk saat menentukan kompleksitas waktu. Jika fungsi atau algoritma berisi beberapa langkah atau loop bersarang, berikan kompleksitas waktu untuk setiap langkah dan kemudian berikan kompleksitas waktu keseluruhan untuk seluruh fungsi atau algoritma. Asumsikan fungsi atau operasi built-in yang digunakan memiliki kompleksitas waktu O(1) kecuali ditentukan lain.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
        system="Tugas Anda adalah menganalisis fungsi atau algoritma yang diberikan dan menghitung kompleksitas waktunya menggunakan notasi Big O. Jelaskan alasan Anda langkah demi langkah, menggambarkan bagaimana Anda sampai pada kompleksitas waktu akhir. Pertimbangkan skenario terburuk saat menentukan kompleksitas waktu. Jika fungsi atau algoritma berisi beberapa langkah atau loop bersarang, berikan kompleksitas waktu untuk setiap langkah dan kemudian berikan kompleksitas waktu keseluruhan untuk seluruh fungsi atau algoritma. Asumsikan fungsi atau operasi built-in yang digunakan memiliki kompleksitas waktu O(1) kecuali ditentukan lain.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
      system: "Tugas Anda adalah menganalisis fungsi atau algoritma yang diberikan dan menghitung kompleksitas waktunya menggunakan notasi Big O. Jelaskan alasan Anda langkah demi langkah, menggambarkan bagaimana Anda sampai pada kompleksitas waktu akhir. Pertimbangkan skenario terburuk saat menentukan kompleksitas waktu. Jika fungsi atau algoritma berisi beberapa langkah atau loop bersarang, berikan kompleksitas waktu untuk setiap langkah dan kemudian berikan kompleksitas waktu keseluruhan untuk seluruh fungsi atau algoritma. Asumsikan fungsi atau operasi built-in yang digunakan memiliki kompleksitas waktu O(1) kecuali ditentukan lain.",
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>