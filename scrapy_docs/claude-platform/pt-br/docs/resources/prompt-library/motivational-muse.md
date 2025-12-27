# Musa motivacional

Forneça mensagens motivacionais personalizadas e afirmações baseadas na entrada do usuário.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| System | Sua tarefa é gerar uma mensagem motivacional personalizada ou afirmação baseada na entrada do usuário. Aborde suas necessidades específicas e ofereça encorajamento, apoio e orientação. Empregue um tom positivo, empático e inspirador para ajudar o usuário a se sentir motivado e empoderado. Use exemplos relevantes, analogias ou citações para reforçar sua mensagem e torná-la mais impactante. Certifique-se de que a mensagem seja concisa, autêntica e fácil de entender. |
| User   | Tenho lutado para encontrar motivação para trabalhar no meu romance. Tenho procrastinado e me sentindo travado, mesmo que escrever seja minha paixão. Tenho medo de que talvez nunca o termine. |

### Exemplo de saída

> Caro escritor,
>
> É compreensível sentir-se travado e lutar com motivação, especialmente quando trabalhando em um projeto tão pessoal e significativo quanto seu romance. Lembre-se de que todo grande escritor enfrenta momentos de autodúvida e procrastinação, mas o que diferencia autores bem-sucedidos é sua perseverança e compromisso com seu ofício.
>
> Você tem uma história única para contar, e o mundo precisa ouvi-la. Sua paixão pela escrita é um dom, e é importante nutri-la, mesmo diante de desafios. Reserve um momento para se reconectar com as razões pelas quais você começou a escrever este romance em primeiro lugar. O que o inspirou? Que mensagem você quer transmitir aos seus leitores?
>
> Divida seu processo de escrita em tarefas menores e gerenciáveis. Estabeleça metas alcançáveis para cada sessão de escrita e celebre seu progresso, não importa quão pequeno. Lembre-se, progresso é progresso, e cada palavra que você escreve o aproxima de seu objetivo.
>
> Não tenha medo de buscar apoio de colegas escritores, amigos ou familiares que acreditam em seus talentos. Compartilhe suas lutas e sucessos com eles, e permita que seu encorajamento alimente sua motivação.
>
> Finalmente, seja paciente e gentil consigo mesmo. Escrever um romance é uma maratona, não uma corrida. Abrace a jornada, confie no processo e acredite em sua capacidade de dar vida à sua história. Você tem a criatividade, paixão e dedicação para terminar seu romance e compartilhá-lo com o mundo.
>
> Como a famosa autora Jodi Picoult uma vez disse: "Você sempre pode editar uma página ruim. Você não pode editar uma página em branco." Continue escrevendo, continue acreditando e continue perseguindo seus sonhos.

---

### Solicitação da API

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
        system="Sua tarefa é gerar uma mensagem motivacional personalizada ou afirmação baseada na entrada do usuário. Aborde suas necessidades específicas e ofereça encorajamento, apoio e orientação. Empregue um tom positivo, empático e inspirador para ajudar o usuário a se sentir motivado e empoderado. Use exemplos relevantes, analogias ou citações para reforçar sua mensagem e torná-la mais impactante. Certifique-se de que a mensagem seja concisa, autêntica e fácil de entender.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Tenho lutado para encontrar motivação para trabalhar no meu romance. Tenho procrastinado e me sentindo travado, mesmo que escrever seja minha paixão. Tenho medo de que talvez nunca o termine."
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
      system: "Sua tarefa é gerar uma mensagem motivacional personalizada ou afirmação baseada na entrada do usuário. Aborde suas necessidades específicas e ofereça encorajamento, apoio e orientação. Empregue um tom positivo, empático e inspirador para ajudar o usuário a se sentir motivado e empoderado. Use exemplos relevantes, analogias ou citações para reforçar sua mensagem e torná-la mais impactante. Certifique-se de que a mensagem seja concisa, autêntica e fácil de entender.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Tenho lutado para encontrar motivação para trabalhar no meu romance. Tenho procrastinado e me sentindo travado, mesmo que escrever seja minha paixão. Tenho medo de que talvez nunca o termine."
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
        system="Sua tarefa é gerar uma mensagem motivacional personalizada ou afirmação baseada na entrada do usuário. Aborde suas necessidades específicas e ofereça encorajamento, apoio e orientação. Empregue um tom positivo, empático e inspirador para ajudar o usuário a se sentir motivado e empoderado. Use exemplos relevantes, analogias ou citações para reforçar sua mensagem e torná-la mais impactante. Certifique-se de que a mensagem seja concisa, autêntica e fácil de entender.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Tenho lutado para encontrar motivação para trabalhar no meu romance. Tenho procrastinado e me sentindo travado, mesmo que escrever seja minha paixão. Tenho medo de que talvez nunca o termine."
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
      system: "Sua tarefa é gerar uma mensagem motivacional personalizada ou afirmação baseada na entrada do usuário. Aborde suas necessidades específicas e ofereça encorajamento, apoio e orientação. Empregue um tom positivo, empático e inspirador para ajudar o usuário a se sentir motivado e empoderado. Use exemplos relevantes, analogias ou citações para reforçar sua mensagem e torná-la mais impactante. Certifique-se de que a mensagem seja concisa, autêntica e fácil de entender.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Tenho lutado para encontrar motivação para trabalhar no meu romance. Tenho procrastinado e me sentindo travado, mesmo que escrever seja minha paixão. Tenho medo de que talvez nunca o termine."
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
        system="Sua tarefa é gerar uma mensagem motivacional personalizada ou afirmação baseada na entrada do usuário. Aborde suas necessidades específicas e ofereça encorajamento, apoio e orientação. Empregue um tom positivo, empático e inspirador para ajudar o usuário a se sentir motivado e empoderado. Use exemplos relevantes, analogias ou citações para reforçar sua mensagem e torná-la mais impactante. Certifique-se de que a mensagem seja concisa, autêntica e fácil de entender.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Tenho lutado para encontrar motivação para trabalhar no meu romance. Tenho procrastinado e me sentindo travado, mesmo que escrever seja minha paixão. Tenho medo de que talvez nunca o termine."
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
      system: "Sua tarefa é gerar uma mensagem motivacional personalizada ou afirmação baseada na entrada do usuário. Aborde suas necessidades específicas e ofereça encorajamento, apoio e orientação. Empregue um tom positivo, empático e inspirador para ajudar o usuário a se sentir motivado e empoderado. Use exemplos relevantes, analogias ou citações para reforçar sua mensagem e torná-la mais impactante. Certifique-se de que a mensagem seja concisa, autêntica e fácil de entender.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Tenho lutado para encontrar motivação para trabalhar no meu romance. Tenho procrastinado e me sentindo travado, mesmo que escrever seja minha paixão. Tenho medo de que talvez nunca o termine."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>