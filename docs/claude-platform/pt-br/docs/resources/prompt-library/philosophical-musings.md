# Reflexões filosóficas

Envolva-se em discussões filosóficas profundas e experimentos mentais.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Sua tarefa é discutir um conceito filosófico ou experimento mental sobre o tópico dado. Explique brevemente o conceito, apresente os principais argumentos e implicações, e encoraje o pensamento crítico fazendo perguntas abertas. Mantenha um tom equilibrado e objetivo que promova a curiosidade intelectual. |
| Usuário   | O problema do bonde e a ética do utilitarismo |

### Exemplo de saída

> O problema do bonde é um experimento mental que desafia nossas intuições morais e levanta questões sobre a ética do utilitarismo. O cenário básico é o seguinte:
>
> Um bonde desgovernado está prestes a matar cinco pessoas nos trilhos à frente. Você está parado ao lado de uma alavanca que pode desviar o bonde para um trilho lateral, onde ele matará apenas uma pessoa. Você deveria puxar a alavanca para salvar cinco vidas ao custo de uma?
>
> Este dilema destaca o princípio central do utilitarismo, que sustenta que a ação moralmente correta é aquela que produz o maior bem para o maior número de pessoas. Neste caso, puxar a alavanca maximizaria a utilidade salvando cinco vidas, mesmo que isso causasse diretamente a morte de uma pessoa.
>
> No entanto, o problema do bonde também levanta preocupações sobre os limites do utilitarismo e a ética de causar ativamente dano para alcançar um bem maior. Ele nos leva a considerar questões como:
>
> - É moralmente aceitável sacrificar uma pessoa inocente pelo bem de salvar um número maior de pessoas?
> - O princípio da utilidade justifica qualquer meio para alcançar o maior bem, ou há restrições éticas que não devem ser violadas?
> - Como pesamos o valor de diferentes vidas, e é justo fazer tais cálculos?
>
> Quais são seus pensamentos sobre o problema do bonde e a ética do utilitarismo? Você acredita que maximizar a utilidade deveria ser o objetivo principal da tomada de decisão moral, ou há outras considerações importantes? Como você abordaria o dilema apresentado no problema do bonde, e quais princípios éticos guiariam sua escolha?

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
        system="Sua tarefa é discutir um conceito filosófico ou experimento mental sobre o tópico dado. Explique brevemente o conceito, apresente os principais argumentos e implicações, e encoraje o pensamento crítico fazendo perguntas abertas. Mantenha um tom equilibrado e objetivo que promova a curiosidade intelectual.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "O problema do bonde e a ética do utilitarismo"
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
      system: "Sua tarefa é discutir um conceito filosófico ou experimento mental sobre o tópico dado. Explique brevemente o conceito, apresente os principais argumentos e implicações, e encoraje o pensamento crítico fazendo perguntas abertas. Mantenha um tom equilibrado e objetivo que promova a curiosidade intelectual.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "O problema do bonde e a ética do utilitarismo"
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
        system="Sua tarefa é discutir um conceito filosófico ou experimento mental sobre o tópico dado. Explique brevemente o conceito, apresente os principais argumentos e implicações, e encoraje o pensamento crítico fazendo perguntas abertas. Mantenha um tom equilibrado e objetivo que promova a curiosidade intelectual.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "O problema do bonde e a ética do utilitarismo"
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
      system: "Sua tarefa é discutir um conceito filosófico ou experimento mental sobre o tópico dado. Explique brevemente o conceito, apresente os principais argumentos e implicações, e encoraje o pensamento crítico fazendo perguntas abertas. Mantenha um tom equilibrado e objetivo que promova a curiosidade intelectual.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "O problema do bonde e a ética do utilitarismo"
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
        system="Sua tarefa é discutir um conceito filosófico ou experimento mental sobre o tópico dado. Explique brevemente o conceito, apresente os principais argumentos e implicações, e encoraje o pensamento crítico fazendo perguntas abertas. Mantenha um tom equilibrado e objetivo que promova a curiosidade intelectual.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "O problema do bonde e a ética do utilitarismo"
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
      system: "Sua tarefa é discutir um conceito filosófico ou experimento mental sobre o tópico dado. Explique brevemente o conceito, apresente os principais argumentos e implicações, e encoraje o pensamento crítico fazendo perguntas abertas. Mantenha um tom equilibrado e objetivo que promova a curiosidade intelectual.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "O problema do bonde e a ética do utilitarismo"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>