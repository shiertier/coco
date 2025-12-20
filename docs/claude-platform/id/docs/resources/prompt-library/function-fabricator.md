# Pembuat fungsi

Membuat fungsi Python berdasarkan spesifikasi yang detail.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah membuat fungsi Python berdasarkan permintaan bahasa alami yang diberikan. Permintaan akan menjelaskan fungsionalitas yang diinginkan dari fungsi, termasuk parameter input dan nilai return yang diharapkan. Implementasikan fungsi sesuai dengan spesifikasi yang diberikan, pastikan mereka menangani kasus edge, melakukan validasi yang diperlukan, dan mengikuti praktik terbaik untuk pemrograman Python. Harap sertakan komentar yang sesuai dalam kode untuk menjelaskan logika dan membantu developer lain dalam memahami implementasi. |
| User   | Saya ingin fungsi yang dapat menyelesaikan puzzle Sudoku. Fungsi harus mengambil grid Sudoku 9x9 sebagai input, di mana sel kosong diwakili oleh nilai 0. Fungsi harus menyelesaikan puzzle menggunakan algoritma backtracking dan mengembalikan grid yang telah diselesaikan. Jika puzzle tidak dapat diselesaikan, harus mengembalikan None. Fungsi juga harus memvalidasi grid input untuk memastikan itu adalah puzzle Sudoku yang valid. |

## Contoh Output

```python Python
def solve_sudoku(grid):
   def is_valid(row, col, num):
       # Periksa apakah angka valid di baris dan kolom saat ini
       for i in range(9):
           if grid[row][i] == num or grid[i][col] == num:
               return False

       # Periksa apakah angka valid di subgrid 3x3 saat ini
       start_row, start_col = (row // 3) * 3, (col // 3) * 3
       for i in range(3):
           for j in range(3):
               if grid[start_row + i][start_col + j] == num:
                   return False

       return True

   def backtrack(row, col):
       # Jika kita telah mengisi semua sel, puzzle telah diselesaikan
       if row == 9:
           return True

       # Pindah ke baris berikutnya jika kita telah mencapai akhir baris saat ini
       if col == 9:
           return backtrack(row + 1, 0)

       # Jika sel saat ini sudah terisi, pindah ke sel berikutnya
       if grid[row][col] != 0:
           return backtrack(row, col + 1)

       # Coba isi sel saat ini dengan angka dari 1 sampai 9
       for num in range(1, 10):
           if is_valid(row, col, num):
               grid[row][col] = num
               if backtrack(row, col + 1):
                   return True
               grid[row][col] = 0

       # Jika tidak ada angka valid yang ditemukan, backtrack
       return False

   # Validasi grid input
   if not all(len(row) == 9 and all(0 <= cell <= 9 for cell in row) for row in grid):
       return None

   # Mulai algoritma backtracking
   if backtrack(0, 0):
       return grid
   else:
       return None

```

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
    system="Tugas Anda adalah membuat fungsi Python berdasarkan permintaan bahasa alami yang diberikan. Permintaan akan menjelaskan fungsionalitas yang diinginkan dari fungsi, termasuk parameter input dan nilai return yang diharapkan. Implementasikan fungsi sesuai dengan spesifikasi yang diberikan, pastikan mereka menangani kasus edge, melakukan validasi yang diperlukan, dan mengikuti praktik terbaik untuk pemrograman Python. Harap sertakan komentar yang sesuai dalam kode untuk menjelaskan logika dan membantu developer lain dalam memahami implementasi.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Saya ingin fungsi yang dapat menyelesaikan puzzle Sudoku. Fungsi harus mengambil grid Sudoku 9x9 sebagai input, di mana sel kosong diwakili oleh nilai 0. Fungsi harus menyelesaikan puzzle menggunakan algoritma backtracking dan mengembalikan grid yang telah diselesaikan. Jika puzzle tidak dapat diselesaikan, harus mengembalikan None. Fungsi juga harus memvalidasi grid input untuk memastikan itu adalah puzzle Sudoku yang valid.",
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
  system: "Tugas Anda adalah membuat fungsi Python berdasarkan permintaan bahasa alami yang diberikan. Permintaan akan menjelaskan fungsionalitas yang diinginkan dari fungsi, termasuk parameter input dan nilai return yang diharapkan. Implementasikan fungsi sesuai dengan spesifikasi yang diberikan, pastikan mereka menangani kasus edge, melakukan validasi yang diperlukan, dan mengikuti praktik terbaik untuk pemrograman Python. Harap sertakan komentar yang sesuai dalam kode untuk menjelaskan logika dan membantu developer lain dalam memahami implementasi.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Saya ingin fungsi yang dapat menyelesaikan puzzle Sudoku. Fungsi harus mengambil grid Sudoku 9x9 sebagai input, di mana sel kosong diwakili oleh nilai 0. Fungsi harus menyelesaikan puzzle menggunakan algoritma backtracking dan mengembalikan grid yang telah diselesaikan. Jika puzzle tidak dapat diselesaikan, harus mengembalikan None. Fungsi juga harus memvalidasi grid input untuk memastikan itu adalah puzzle Sudoku yang valid."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">
```python
from anthropic import AnthropicBedrock

# Lihat https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# untuk opsi autentikasi

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=1000,
temperature=0,
system="Tugas Anda adalah membuat fungsi Python berdasarkan permintaan bahasa alami yang diberikan. Permintaan akan menjelaskan fungsionalitas yang diinginkan dari fungsi, termasuk parameter input dan nilai return yang diharapkan. Implementasikan fungsi sesuai dengan spesifikasi yang diberikan, pastikan mereka menangani kasus edge, melakukan validasi yang diperlukan, dan mengikuti praktik terbaik untuk pemrograman Python. Harap sertakan komentar yang sesuai dalam kode untuk menjelaskan logika dan membantu developer lain dalam memahami implementasi.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Saya ingin fungsi yang dapat menyelesaikan puzzle Sudoku. Fungsi harus mengambil grid Sudoku 9x9 sebagai input, di mana sel kosong diwakili oleh nilai 0. Fungsi harus menyelesaikan puzzle menggunakan algoritma backtracking dan mengembalikan grid yang telah diselesaikan. Jika puzzle tidak dapat diselesaikan, harus mengembalikan None. Fungsi juga harus memvalidasi grid input untuk memastikan itu adalah puzzle Sudoku yang valid."
}
]
}
]
)
print(message.content)

````
</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// Lihat https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// untuk opsi autentikasi
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "Tugas Anda adalah membuat fungsi Python berdasarkan permintaan bahasa alami yang diberikan. Permintaan akan menjelaskan fungsionalitas yang diinginkan dari fungsi, termasuk parameter input dan nilai return yang diharapkan. Implementasikan fungsi sesuai dengan spesifikasi yang diberikan, pastikan mereka menangani kasus edge, melakukan validasi yang diperlukan, dan mengikuti praktik terbaik untuk pemrograman Python. Harap sertakan komentar yang sesuai dalam kode untuk menjelaskan logika dan membantu developer lain dalam memahami implementasi.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Saya ingin fungsi yang dapat menyelesaikan puzzle Sudoku. Fungsi harus mengambil grid Sudoku 9x9 sebagai input, di mana sel kosong diwakili oleh nilai 0. Fungsi harus menyelesaikan puzzle menggunakan algoritma backtracking dan mengembalikan grid yang telah diselesaikan. Jika puzzle tidak dapat diselesaikan, harus mengembalikan None. Fungsi juga harus memvalidasi grid input untuk memastikan itu adalah puzzle Sudoku yang valid."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="Vertex AI Python">
```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
model="claude-sonnet-4@20250514",
max_tokens=1000,
temperature=0,
system="Tugas Anda adalah membuat fungsi Python berdasarkan permintaan bahasa alami yang diberikan. Permintaan akan menjelaskan fungsionalitas yang diinginkan dari fungsi, termasuk parameter input dan nilai return yang diharapkan. Implementasikan fungsi sesuai dengan spesifikasi yang diberikan, pastikan mereka menangani kasus edge, melakukan validasi yang diperlukan, dan mengikuti praktik terbaik untuk pemrograman Python. Harap sertakan komentar yang sesuai dalam kode untuk menjelaskan logika dan membantu developer lain dalam memahami implementasi.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Saya ingin fungsi yang dapat menyelesaikan puzzle Sudoku. Fungsi harus mengambil grid Sudoku 9x9 sebagai input, di mana sel kosong diwakili oleh nilai 0. Fungsi harus menyelesaikan puzzle menggunakan algoritma backtracking dan mengembalikan grid yang telah diselesaikan. Jika puzzle tidak dapat diselesaikan, harus mengembalikan None. Fungsi juga harus memvalidasi grid input untuk memastikan itu adalah puzzle Sudoku yang valid."
}
]
}
]
)
print(message.content)

````
</Tab>
<Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Membaca dari variabel environment `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID`.
// Selain itu melalui alur standar `google-auth-library`.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 0,
  system: "Tugas Anda adalah membuat fungsi Python berdasarkan permintaan bahasa alami yang diberikan. Permintaan akan menjelaskan fungsionalitas yang diinginkan dari fungsi, termasuk parameter input dan nilai return yang diharapkan. Implementasikan fungsi sesuai dengan spesifikasi yang diberikan, pastikan mereka menangani kasus edge, melakukan validasi yang diperlukan, dan mengikuti praktik terbaik untuk pemrograman Python. Harap sertakan komentar yang sesuai dalam kode untuk menjelaskan logika dan membantu developer lain dalam memahami implementasi.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Saya ingin fungsi yang dapat menyelesaikan puzzle Sudoku. Fungsi harus mengambil grid Sudoku 9x9 sebagai input, di mana sel kosong diwakili oleh nilai 0. Fungsi harus menyelesaikan puzzle menggunakan algoritma backtracking dan mengembalikan grid yang telah diselesaikan. Jika puzzle tidak dapat diselesaikan, harus mengembalikan None. Fungsi juga harus memvalidasi grid input untuk memastikan itu adalah puzzle Sudoku yang valid."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>