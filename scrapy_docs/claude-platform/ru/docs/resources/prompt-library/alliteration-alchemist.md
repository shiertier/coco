# Алхимик аллитерации

Генерируйте аллитеративные фразы и предложения для любой заданной темы.

---

> Скопируйте этот промпт в нашу [Консоль](/dashboard) разработчика, чтобы попробовать самостоятельно!

|        | Содержание |
| --- | --- |
| System | Ваша задача — создавать аллитеративные фразы и предложения для заданной темы. Убедитесь, что аллитерации не только звучат приятно, но также передают релевантную информацию или вызывают соответствующие эмоции, связанные с темой. |
| User   | Океан |

### Пример вывода

> 1.  Шепчущие волны омывают берег.
> 2.  Соленые морские брызги разбрызгиваются по песку.
> 3.  Разбивающиеся течения вырезают прибрежные утесы.
> 4.  Глубокие, темные глубины скрывают скрытые опасности.
> 5.  Булькающие ручьи приносят жизнь на пляж.
> 6.  Могучие, величественные млекопитающие мигрируют через воды.
> 7.  Грохочущие приливы бросают корабли туда-сюда.
> 8.  Мерцающие отмели укрывают скрытных морских существ.
> 9.  Рябящие отражения раскрывают мир под поверхностью.
> 10. Кружащиеся, бушующие моря формируют береговую линию.
> 11. Пенящиеся, пузырящиеся льдины текут свободно.
> 12. Грациозные чайки скользят над сверкающим океаном.
> 13. Игривые морские свиньи ныряют через нетронутые воды.
> 14. Соленые бризы дуют через безграничную синеву.
> 15. Ритмичная рябь катится к скалистому побережью.
> 16. Коралловые пещеры скрывают красочные водные сообщества.
> 17. Штормовые небеса и вздымающиеся моря сигнализируют о мощи природы.
> 18. Кувыркающиеся приливы преображают спокойную бухту.
> 19. Кружащиеся, свистящие ветры взбивают волны.
> 20. Безмятежные, сапфировые воды простираются до горизонта.

---

### API запрос

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
        temperature=1,
        system="Ваша задача — создавать аллитеративные фразы и предложения для заданной темы. Убедитесь, что аллитерации не только звучат приятно, но также передают релевантную информацию или вызывают соответствующие эмоции, связанные с темой.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Океан"
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
      temperature: 1,
      system: "Ваша задача — создавать аллитеративные фразы и предложения для заданной темы. Убедитесь, что аллитерации не только звучат приятно, но также передают релевантную информацию или вызывают соответствующие эмоции, связанные с темой.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Океан"
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
        temperature=1,
        system="Ваша задача — создавать аллитеративные фразы и предложения для заданной темы. Убедитесь, что аллитерации не только звучат приятно, но также передают релевантную информацию или вызывают соответствующие эмоции, связанные с темой.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Океан"
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
      temperature: 1,
      system: "Ваша задача — создавать аллитеративные фразы и предложения для заданной темы. Убедитесь, что аллитерации не только звучат приятно, но также передают релевантную информацию или вызывают соответствующие эмоции, связанные с темой.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Океан"
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
        temperature=1,
        system="Ваша задача — создавать аллитеративные фразы и предложения для заданной темы. Убедитесь, что аллитерации не только звучат приятно, но также передают релевантную информацию или вызывают соответствующие эмоции, связанные с темой.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Океан"
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
      temperature: 1,
      system: "Ваша задача — создавать аллитеративные фразы и предложения для заданной темы. Убедитесь, что аллитерации не только звучат приятно, но также передают релевантную информацию или вызывают соответствующие эмоции, связанные с темой.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Океан"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>