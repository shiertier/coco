# Анализатор кодов аэропортов

Найти и извлечь коды аэропортов из текста.

---

> Скопируйте этот промпт в нашу [Консоль](/dashboard) разработчика, чтобы попробовать его самостоятельно!

|        | Содержание |
| --- | --- |
| System | Ваша задача - проанализировать предоставленный текст и определить любые коды аэропортов, упомянутые в нем. Представьте эти коды аэропортов в виде списка в том порядке, в котором они появляются в тексте. Если коды аэропортов не найдены, верните пустой список. |
| User   | Моя следующая поездка включает перелет из Сиэтла в Амстердам. Я проведу несколько дней в Амстердаме, прежде чем отправиться в Париж на стыковочный рейс в Рим. |

### Пример вывода

> Вот список кодов аэропортов, упомянутых в тексте, в порядке их появления:
>
> 1. SEA (Сиэтл)
> 2. AMS (Амстердам)
> 3. CDG (Париж)
> 4. FCO (Рим)

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
        temperature=0,
        system="Ваша задача - проанализировать предоставленный текст и определить любые коды аэропортов, упомянутые в нем. Представьте эти коды аэропортов в виде списка в том порядке, в котором они появляются в тексте. Если коды аэропортов не найдены, верните пустой список.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Моя следующая поездка включает перелет из Сиэтла в Амстердам. Я проведу несколько дней в Амстердаме, прежде чем отправиться в Париж на стыковочный рейс в Рим."
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
      system: "Ваша задача - проанализировать предоставленный текст и определить любые коды аэропортов, упомянутые в нем. Представьте эти коды аэропортов в виде списка в том порядке, в котором они появляются в тексте. Если коды аэропортов не найдены, верните пустой список.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Моя следующая поездка включает перелет из Сиэтла в Амстердам. Я проведу несколько дней в Амстердаме, прежде чем отправиться в Париж на стыковочный рейс в Рим."
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
        system="Ваша задача - проанализировать предоставленный текст и определить любые коды аэропортов, упомянутые в нем. Представьте эти коды аэропортов в виде списка в том порядке, в котором они появляются в тексте. Если коды аэропортов не найдены, верните пустой список.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Моя следующая поездка включает перелет из Сиэтла в Амстердам. Я проведу несколько дней в Амстердаме, прежде чем отправиться в Париж на стыковочный рейс в Рим."
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
      system: "Ваша задача - проанализировать предоставленный текст и определить любые коды аэропортов, упомянутые в нем. Представьте эти коды аэропортов в виде списка в том порядке, в котором они появляются в тексте. Если коды аэропортов не найдены, верните пустой список.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Моя следующая поездка включает перелет из Сиэтла в Амстердам. Я проведу несколько дней в Амстердаме, прежде чем отправиться в Париж на стыковочный рейс в Рим."
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
        system="Ваша задача - проанализировать предоставленный текст и определить любые коды аэропортов, упомянутые в нем. Представьте эти коды аэропортов в виде списка в том порядке, в котором они появляются в тексте. Если коды аэропортов не найдены, верните пустой список.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Моя следующая поездка включает перелет из Сиэтла в Амстердам. Я проведу несколько дней в Амстердаме, прежде чем отправиться в Париж на стыковочный рейс в Рим."
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
      system: "Ваша задача - проанализировать предоставленный текст и определить любые коды аэропортов, упомянутые в нем. Представьте эти коды аэропортов в виде списка в том порядке, в котором они появляются в тексте. Если коды аэропортов не найдены, верните пустой список.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Моя следующая поездка включает перелет из Сиэтла в Амстердам. Я проведу несколько дней в Амстердаме, прежде чем отправиться в Париж на стыковочный рейс в Рим."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>