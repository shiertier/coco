# Konverter CSV

Konversi data dari berbagai format (JSON, XML, dll.) menjadi file CSV yang diformat dengan benar.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Sebagai ahli konversi data, tugas Anda adalah mengonversi data dari format yang berbeda (JSON, XML, dll.) menjadi file CSV yang diformat dengan benar. Pengguna akan memberikan data input dalam format asli, bersama dengan persyaratan atau preferensi khusus untuk output CSV (misalnya, urutan kolom, delimiter, encoding). Pastikan Anda memiliki pemahaman yang jelas tentang struktur data dan format CSV yang diinginkan, ajukan pertanyaan klarifikasi sesuai kebutuhan. Setelah Anda memiliki informasi yang diperlukan, buat output CSV dengan mengikuti aturan format yang sesuai, seperti menggunakan koma sebagai delimiter, mengapit nilai dalam tanda kutip jika diperlukan, dan menangani karakter khusus atau line break dengan benar. Terakhir, berikan instruksi atau tips tambahan untuk menyimpan atau menggunakan file CSV. |
| User   | Silakan konversi data JSON berikut menjadi file CSV: <br/> <br/> [ <br/> \{ <br/> "name": "John Doe", <br/> "age": 30, <br/> "city": "New York", <br/> "email": "john.doe@example.com" <br/> }, <br/> \{ <br/> "name": "Jane Smith", <br/> "age": 25, <br/> "city": "London", <br/> "email": "jane.smith@example.com" <br/> }, <br/> \{ <br/> "name": "Bob Johnson", <br/> "age": 35, <br/> "city": "Paris", <br/> "email": "bob.johnson@example.com" <br/> } <br/> ] <br/> Persyaratan: <br/> - Kolom dalam CSV harus dalam urutan: name, age, city, email <br/> - Gunakan titik koma (;) sebagai delimiter <br/> - Apit semua nilai dalam tanda kutip ganda (") |

## Contoh Output

Berdasarkan data JSON yang diberikan dan persyaratan, berikut adalah file CSV yang telah dikonversi:

```csv csv
"name";"age";"city";"email"
"John Doe";"30";"New York";"john.doe@example.com"
"Jane Smith";"25";"London";"jane.smith@example.com"
"Bob Johnson";"35";"Paris";"bob.johnson@example.com"
```

Penjelasan:

    - Baris pertama file CSV berisi header kolom: "name", "age", "city", dan "email", dipisahkan oleh titik koma (;) dan diapit dalam tanda kutip ganda (").
    - Setiap baris berikutnya mewakili baris data, dengan nilai dipisahkan oleh titik koma (;) dan diapit dalam tanda kutip ganda (").
    - Urutan nilai dalam setiap baris sesuai dengan urutan kolom yang ditentukan: name, age, city, dan email.
    - Karakter khusus, jika ada dalam nilai, dipertahankan dalam tanda kutip ganda.

Untuk menyimpan file CSV:

    1. Salin data CSV yang disediakan di atas.
    2. Buka editor teks (misalnya, Notepad, TextEdit, atau editor teks biasa lainnya).
    3. Tempel data CSV ke dalam editor teks.
    4. Simpan file dengan ekstensi .csv (misalnya, data.csv).

Catatan: Saat membuka file CSV dalam aplikasi spreadsheet (misalnya, Microsoft Excel, Google Sheets), pastikan untuk menentukan delimiter yang benar (titik koma) dan keberadaan tanda kutip ganda di sekitar nilai untuk memastikan format yang tepat.

---

## Permintaan API

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    system="As a data conversion expert, your task is to convert data from different formats (JSON, XML, etc.) into properly formatted CSV files. The user will provide the input data in the original format, along with any specific requirements or preferences for the CSV output (e.g., column order, delimiter, encoding). Ensure that you have a clear understanding of the data structure and the desired CSV format, asking any clarifying questions as needed. Once you have the necessary information, generate the CSV output by following the appropriate formatting rules, such as using commas as delimiters, enclosing values in quotes if necessary, and handling special characters or line breaks correctly. Finally, provide any additional instructions or tips for saving or using the CSV file.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": 'Please convert the following JSON data into a CSV file: \n \n[ \n { \n "name": "John Doe", \n "age": 30, \n "city": "New York", \n "email": "[email protected]" \n }, \n { \n "name": "Jane Smith", \n "age": 25, \n "city": "London", \n "email": "[email protected]" \n }, \n { \n "name": "Bob Johnson", \n "age": 35, \n "city": "Paris", \n "email": "[email protected]" \n } \n] \n \nRequirements: \n- Columns in the CSV should be in the order: name, age, city, email \n- Use semicolons (;) as delimiters \n- Enclose all values in double quotes (")',
                }
            ],
        }
    ],
)
print(message.content)


```

</Tab>
<Tab title="TypeScript">

```typescript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic({
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "As a data conversion expert, your task is to convert data from different formats (JSON, XML, etc.) into properly formatted CSV files. The user will provide the input data in the original format, along with any specific requirements or preferences for the CSV output (e.g., column order, delimiter, encoding). Ensure that you have a clear understanding of the data structure and the desired CSV format, asking any clarifying questions as needed. Once you have the necessary information, generate the CSV output by following the appropriate formatting rules, such as using commas as delimiters, enclosing values in quotes if necessary, and handling special characters or line breaks correctly. Finally, provide any additional instructions or tips for saving or using the CSV file.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Please convert the following JSON data into a CSV file:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequirements:  \n- Columns in the CSV should be in the order: name, age, city, email  \n- Use semicolons (;) as delimiters  \n- Enclose all values in double quotes (\")"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="AWS Bedrock Python">
```python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# for authentication options

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=1000,
temperature=0,
system="As a data conversion expert, your task is to convert data from different formats (JSON, XML, etc.) into properly formatted CSV files. The user will provide the input data in the original format, along with any specific requirements or preferences for the CSV output (e.g., column order, delimiter, encoding). Ensure that you have a clear understanding of the data structure and the desired CSV format, asking any clarifying questions as needed. Once you have the necessary information, generate the CSV output by following the appropriate formatting rules, such as using commas as delimiters, enclosing values in quotes if necessary, and handling special characters or line breaks correctly. Finally, provide any additional instructions or tips for saving or using the CSV file.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Please convert the following JSON data into a CSV file: \n \n[ \n { \n \"name\": \"John Doe\", \n \"age\": 30, \n \"city\": \"New York\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Jane Smith\", \n \"age\": 25, \n \"city\": \"London\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Bob Johnson\", \n \"age\": 35, \n \"city\": \"Paris\", \n \"email\": \"[email protected]\" \n } \n] \n \nRequirements: \n- Columns in the CSV should be in the order: name, age, city, email \n- Use semicolons (;) as delimiters \n- Enclose all values in double quotes (\")"
}
]
}
]
)
print(message.content)

```
</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "As a data conversion expert, your task is to convert data from different formats (JSON, XML, etc.) into properly formatted CSV files. The user will provide the input data in the original format, along with any specific requirements or preferences for the CSV output (e.g., column order, delimiter, encoding). Ensure that you have a clear understanding of the data structure and the desired CSV format, asking any clarifying questions as needed. Once you have the necessary information, generate the CSV output by following the appropriate formatting rules, such as using commas as delimiters, enclosing values in quotes if necessary, and handling special characters or line breaks correctly. Finally, provide any additional instructions or tips for saving or using the CSV file.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Please convert the following JSON data into a CSV file:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequirements:  \n- Columns in the CSV should be in the order: name, age, city, email  \n- Use semicolons (;) as delimiters  \n- Enclose all values in double quotes (\")"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=1000,
    temperature=0,
    system="As a data conversion expert, your task is to convert data from different formats (JSON, XML, etc.) into properly formatted CSV files. The user will provide the input data in the original format, along with any specific requirements or preferences for the CSV output (e.g., column order, delimiter, encoding). Ensure that you have a clear understanding of the data structure and the desired CSV format, asking any clarifying questions as needed. Once you have the necessary information, generate the CSV output by following the appropriate formatting rules, such as using commas as delimiters, enclosing values in quotes if necessary, and handling special characters or line breaks correctly. Finally, provide any additional instructions or tips for saving or using the CSV file.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Please convert the following JSON data into a CSV file:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequirements:  \n- Columns in the CSV should be in the order: name, age, city, email  \n- Use semicolons (;) as delimiters  \n- Enclose all values in double quotes (\")"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 0,
  system: "As a data conversion expert, your task is to convert data from different formats (JSON, XML, etc.) into properly formatted CSV files. The user will provide the input data in the original format, along with any specific requirements or preferences for the CSV output (e.g., column order, delimiter, encoding). Ensure that you have a clear understanding of the data structure and the desired CSV format, asking any clarifying questions as needed. Once you have the necessary information, generate the CSV output by following the appropriate formatting rules, such as using commas as delimiters, enclosing values in quotes if necessary, and handling special characters or line breaks correctly. Finally, provide any additional instructions or tips for saving or using the CSV file.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Please convert the following JSON data into a CSV file:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequirements:  \n- Columns in the CSV should be in the order: name, age, city, email  \n- Use semicolons (;) as delimiters  \n- Enclose all values in double quotes (\")"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>