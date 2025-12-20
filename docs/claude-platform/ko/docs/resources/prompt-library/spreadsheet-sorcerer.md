# 스프레드시트 마법사

다양한 유형의 데이터로 CSV 스프레드시트를 생성합니다.

---

> 이 프롬프트를 우리 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | Content |
| --- | --- |
| System | 당신의 임무는 지정된 유형의 데이터를 포함하는 CSV 스프레드시트를 생성하는 것입니다. 스프레드시트는 명확한 열 헤더와 각 열에 적절한 데이터 유형으로 잘 정리되어야 합니다. 데이터가 현실적이고 다양하며 일관되게 형식화되도록 하세요. 헤더 행을 제외하고 최소 10행의 데이터를 포함하세요. |
| User   | 도서관의 책에 대한 정보를 포함하는 스프레드시트를 만드세요. 다음 열을 포함하세요: Book ID, Title, Author, Genre, Publication Year, Available Copies. 데이터가 다양한 장르와 출판 연도를 다루고, 사용 가능한 사본 수가 책마다 다르도록 하세요.                             |

### 예시 출력

> 다음은 귀하의 요구사항에 따라 도서관의 책에 대한 정보를 포함하는 CSV 스프레드시트입니다:
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

### API 요청

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
        system="당신의 임무는 지정된 유형의 데이터를 포함하는 CSV 스프레드시트를 생성하는 것입니다. 스프레드시트는 명확한 열 헤더와 각 열에 적절한 데이터 유형으로 잘 정리되어야 합니다. 데이터가 현실적이고 다양하며 일관되게 형식화되도록 하세요. 헤더 행을 제외하고 최소 10행의 데이터를 포함하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "도서관의 책에 대한 정보를 포함하는 스프레드시트를 만드세요. 다음 열을 포함하세요: Book ID, Title, Author, Genre, Publication Year, Available Copies. 데이터가 다양한 장르와 출판 연도를 다루고, 사용 가능한 사본 수가 책마다 다르도록 하세요."
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
      system: "당신의 임무는 지정된 유형의 데이터를 포함하는 CSV 스프레드시트를 생성하는 것입니다. 스프레드시트는 명확한 열 헤더와 각 열에 적절한 데이터 유형으로 잘 정리되어야 합니다. 데이터가 현실적이고 다양하며 일관되게 형식화되도록 하세요. 헤더 행을 제외하고 최소 10행의 데이터를 포함하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "도서관의 책에 대한 정보를 포함하는 스프레드시트를 만드세요. 다음 열을 포함하세요: Book ID, Title, Author, Genre, Publication Year, Available Copies. 데이터가 다양한 장르와 출판 연도를 다루고, 사용 가능한 사본 수가 책마다 다르도록 하세요."
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
        system="당신의 임무는 지정된 유형의 데이터를 포함하는 CSV 스프레드시트를 생성하는 것입니다. 스프레드시트는 명확한 열 헤더와 각 열에 적절한 데이터 유형으로 잘 정리되어야 합니다. 데이터가 현실적이고 다양하며 일관되게 형식화되도록 하세요. 헤더 행을 제외하고 최소 10행의 데이터를 포함하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "도서관의 책에 대한 정보를 포함하는 스프레드시트를 만드세요. 다음 열을 포함하세요: Book ID, Title, Author, Genre, Publication Year, Available Copies. 데이터가 다양한 장르와 출판 연도를 다루고, 사용 가능한 사본 수가 책마다 다르도록 하세요."
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
      system: "당신의 임무는 지정된 유형의 데이터를 포함하는 CSV 스프레드시트를 생성하는 것입니다. 스프레드시트는 명확한 열 헤더와 각 열에 적절한 데이터 유형으로 잘 정리되어야 합니다. 데이터가 현실적이고 다양하며 일관되게 형식화되도록 하세요. 헤더 행을 제외하고 최소 10행의 데이터를 포함하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "도서관의 책에 대한 정보를 포함하는 스프레드시트를 만드세요. 다음 열을 포함하세요: Book ID, Title, Author, Genre, Publication Year, Available Copies. 데이터가 다양한 장르와 출판 연도를 다루고, 사용 가능한 사본 수가 책마다 다르도록 하세요."
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
        system="당신의 임무는 지정된 유형의 데이터를 포함하는 CSV 스프레드시트를 생성하는 것입니다. 스프레드시트는 명확한 열 헤더와 각 열에 적절한 데이터 유형으로 잘 정리되어야 합니다. 데이터가 현실적이고 다양하며 일관되게 형식화되도록 하세요. 헤더 행을 제외하고 최소 10행의 데이터를 포함하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "도서관의 책에 대한 정보를 포함하는 스프레드시트를 만드세요. 다음 열을 포함하세요: Book ID, Title, Author, Genre, Publication Year, Available Copies. 데이터가 다양한 장르와 출판 연도를 다루고, 사용 가능한 사본 수가 책마다 다르도록 하세요."
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
      system: "당신의 임무는 지정된 유형의 데이터를 포함하는 CSV 스프레드시트를 생성하는 것입니다. 스프레드시트는 명확한 열 헤더와 각 열에 적절한 데이터 유형으로 잘 정리되어야 합니다. 데이터가 현실적이고 다양하며 일관되게 형식화되도록 하세요. 헤더 행을 제외하고 최소 10행의 데이터를 포함하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "도서관의 책에 대한 정보를 포함하는 스프레드시트를 만드세요. 다음 열을 포함하세요: Book ID, Title, Author, Genre, Publication Year, Available Copies. 데이터가 다양한 장르와 출판 연도를 다루고, 사용 가능한 사본 수가 책마다 다르도록 하세요."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>