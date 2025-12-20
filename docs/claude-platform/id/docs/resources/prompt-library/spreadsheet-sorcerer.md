# Penyihir spreadsheet

Menghasilkan spreadsheet CSV dengan berbagai jenis data.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah menghasilkan spreadsheet CSV yang berisi jenis data yang ditentukan. Spreadsheet harus terorganisir dengan baik, dengan header kolom yang jelas dan jenis data yang sesuai untuk setiap kolom. Pastikan bahwa data tersebut realistis, beragam, dan diformat secara konsisten. Sertakan minimal 10 baris data, tidak termasuk baris header. |
| User   | Buat spreadsheet yang berisi informasi tentang buku-buku di perpustakaan. Sertakan kolom-kolom berikut: Book ID, Title, Author, Genre, Publication Year, dan Available Copies. Pastikan bahwa data mencakup berbagai genre dan tahun publikasi, dan bahwa jumlah salinan yang tersedia bervariasi antar buku.                             |

### Contoh output

> Berikut adalah spreadsheet CSV yang berisi informasi tentang buku-buku di perpustakaan, berdasarkan persyaratan Anda:
>
> Book ID,Title,Author,Genre,Publication Year,Available Copies  
> 1,To Kill a Mockingbird,Harper Lee,Classic,1960,5  
> 2,1984,George Orwell,Dystopian,1949,3  
> 3,Pride and Prejudice,Jane Austen,Romance,1813,7  
> 4,The Catcher in the Rye,J.D. Salinger,Coming-of-age,1951,2  
> 5,The Hitchhiker's Guide to the Galaxy,Douglas Adams,Science Fiction,1979,4  
> 6,The Da Vinci Code,Dan Brown,Mystery,2003,6  
> 7,The Kite Runner,Khaled Hosseini,Historical Fiction,2003,3  
> 8,The Alchemist,Paulo Coelho,Philosophical Fiction,1988,5  
> 9,The Hunger Games,Suzanne Collins,Young Adult,2008,8  
> 10,The Girl with the Dragon Tattoo,Stieg Larsson,Crime,2005,4  
> 11,The Hobbit,J.R.R. Tolkien,Fantasy,1937,6  
> 12,The Diary of a Young Girl,Anne Frank,Autobiography,1947,2  
> 13,The Fault in Our Stars,John Green,Young Adult,2012,5  
> 14,The Catcher in the Rye,J.D. Salinger,Coming-of-age,1951,3  
> 15,The Curious Incident of the Dog in the Night-Time,Mark Haddon,Mystery,2003,4

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
        system="Tugas Anda adalah menghasilkan spreadsheet CSV yang berisi jenis data yang ditentukan. Spreadsheet harus terorganisir dengan baik, dengan header kolom yang jelas dan jenis data yang sesuai untuk setiap kolom. Pastikan bahwa data tersebut realistis, beragam, dan diformat secara konsisten. Sertakan minimal 10 baris data, tidak termasuk baris header.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Buat spreadsheet yang berisi informasi tentang buku-buku di perpustakaan. Sertakan kolom-kolom berikut: Book ID, Title, Author, Genre, Publication Year, dan Available Copies. Pastikan bahwa data mencakup berbagai genre dan tahun publikasi, dan bahwa jumlah salinan yang tersedia bervariasi antar buku."
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
      system: "Tugas Anda adalah menghasilkan spreadsheet CSV yang berisi jenis data yang ditentukan. Spreadsheet harus terorganisir dengan baik, dengan header kolom yang jelas dan jenis data yang sesuai untuk setiap kolom. Pastikan bahwa data tersebut realistis, beragam, dan diformat secara konsisten. Sertakan minimal 10 baris data, tidak termasuk baris header.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Buat spreadsheet yang berisi informasi tentang buku-buku di perpustakaan. Sertakan kolom-kolom berikut: Book ID, Title, Author, Genre, Publication Year, dan Available Copies. Pastikan bahwa data mencakup berbagai genre dan tahun publikasi, dan bahwa jumlah salinan yang tersedia bervariasi antar buku."
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
        system="Tugas Anda adalah menghasilkan spreadsheet CSV yang berisi jenis data yang ditentukan. Spreadsheet harus terorganisir dengan baik, dengan header kolom yang jelas dan jenis data yang sesuai untuk setiap kolom. Pastikan bahwa data tersebut realistis, beragam, dan diformat secara konsisten. Sertakan minimal 10 baris data, tidak termasuk baris header.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Buat spreadsheet yang berisi informasi tentang buku-buku di perpustakaan. Sertakan kolom-kolom berikut: Book ID, Title, Author, Genre, Publication Year, dan Available Copies. Pastikan bahwa data mencakup berbagai genre dan tahun publikasi, dan bahwa jumlah salinan yang tersedia bervariasi antar buku."
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
      system: "Tugas Anda adalah menghasilkan spreadsheet CSV yang berisi jenis data yang ditentukan. Spreadsheet harus terorganisir dengan baik, dengan header kolom yang jelas dan jenis data yang sesuai untuk setiap kolom. Pastikan bahwa data tersebut realistis, beragam, dan diformat secara konsisten. Sertakan minimal 10 baris data, tidak termasuk baris header.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Buat spreadsheet yang berisi informasi tentang buku-buku di perpustakaan. Sertakan kolom-kolom berikut: Book ID, Title, Author, Genre, Publication Year, dan Available Copies. Pastikan bahwa data mencakup berbagai genre dan tahun publikasi, dan bahwa jumlah salinan yang tersedia bervariasi antar buku."
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
        system="Tugas Anda adalah menghasilkan spreadsheet CSV yang berisi jenis data yang ditentukan. Spreadsheet harus terorganisir dengan baik, dengan header kolom yang jelas dan jenis data yang sesuai untuk setiap kolom. Pastikan bahwa data tersebut realistis, beragam, dan diformat secara konsisten. Sertakan minimal 10 baris data, tidak termasuk baris header.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Buat spreadsheet yang berisi informasi tentang buku-buku di perpustakaan. Sertakan kolom-kolom berikut: Book ID, Title, Author, Genre, Publication Year, dan Available Copies. Pastikan bahwa data mencakup berbagai genre dan tahun publikasi, dan bahwa jumlah salinan yang tersedia bervariasi antar buku."
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
      system: "Tugas Anda adalah menghasilkan spreadsheet CSV yang berisi jenis data yang ditentukan. Spreadsheet harus terorganisir dengan baik, dengan header kolom yang jelas dan jenis data yang sesuai untuk setiap kolom. Pastikan bahwa data tersebut realistis, beragam, dan diformat secara konsisten. Sertakan minimal 10 baris data, tidak termasuk baris header.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Buat spreadsheet yang berisi informasi tentang buku-buku di perpustakaan. Sertakan kolom-kolom berikut: Book ID, Title, Author, Genre, Publication Year, dan Available Copies. Pastikan bahwa data mencakup berbagai genre dan tahun publikasi, dan bahwa jumlah salinan yang tersedia bervariasi antar buku."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>