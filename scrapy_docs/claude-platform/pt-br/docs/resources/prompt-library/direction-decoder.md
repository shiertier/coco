# Decodificador de direções

Transforme linguagem natural em instruções passo a passo.

---

> Copie este prompt para nosso [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F) de desenvolvedor para experimentar você mesmo!

|        | Conteúdo |
| --- | --- |
| System | Sua tarefa é pegar a descrição em linguagem natural fornecida de um processo ou tarefa e transformá-la em instruções passo a passo claras, concisas, lógicas, sequenciais e fáceis de seguir. Use linguagem imperativa e comece cada passo com um verbo de ação. Forneça detalhes e explicações necessários para garantir que o leitor possa completar a tarefa com sucesso. Se a descrição original for pouco clara, ambígua ou carecer de informações suficientes, peça esclarecimentos ou detalhes adicionais. |
| User   | Para fazer uma xícara de chá, comece fervendo um pouco de água em uma chaleira. Enquanto a água está esquentando, pegue uma caneca e coloque um saquinho de chá nela. Uma vez que a água esteja fervendo, despeje-a cuidadosamente na caneca, sobre o saquinho de chá. Deixe o chá em infusão por alguns minutos, dependendo de quão forte você gosta. Após a infusão, remova o saquinho de chá e adicione leite e açúcar a gosto. Misture tudo junto e aproveite sua xícara de chá quente. |

### Exemplo de saída

> 1. Encha uma chaleira com água e coloque-a no fogão para ferver.
> 2. Enquanto espera a água ferver, selecione uma caneca e coloque um saquinho de chá dentro dela.
> 3. Uma vez que a água atinja a fervura, despeje cuidadosamente a água quente na caneca, sobre o saquinho de chá.
> 4. Permita que o chá faça infusão na água quente pelo tempo desejado, tipicamente 2-5 minutos, dependendo da força preferida.
> 5. Após o tempo de infusão ter decorrido, remova o saquinho de chá da caneca e descarte-o.
> 6. Adicione leite e açúcar ao chá, de acordo com suas preferências de gosto.
> 7. Usando uma colher, misture o chá, leite e açúcar juntos até ficarem bem combinados.
> 8. Sua xícara de chá agora está pronta para ser apreciada. Beba-a enquanto ainda está quente.

---

## Solicitação da API

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
        system="Sua tarefa é pegar a descrição em linguagem natural fornecida de um processo ou tarefa e transformá-la em instruções passo a passo claras, concisas, lógicas, sequenciais e fáceis de seguir. Use linguagem imperativa e comece cada passo com um verbo de ação. Forneça detalhes e explicações necessários para garantir que o leitor possa completar a tarefa com sucesso. Se a descrição original for pouco clara, ambígua ou carecer de informações suficientes, peça esclarecimentos ou detalhes adicionais.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Para fazer uma xícara de chá, comece fervendo um pouco de água em uma chaleira. Enquanto a água está esquentando, pegue uma caneca e coloque um saquinho de chá nela. Uma vez que a água esteja fervendo, despeje-a cuidadosamente na caneca, sobre o saquinho de chá. Deixe o chá em infusão por alguns minutos, dependendo de quão forte você gosta. Após a infusão, remova o saquinho de chá e adicione leite e açúcar a gosto. Misture tudo junto e aproveite sua xícara de chá quente."
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
      system: "Sua tarefa é pegar a descrição em linguagem natural fornecida de um processo ou tarefa e transformá-la em instruções passo a passo claras, concisas, lógicas, sequenciais e fáceis de seguir. Use linguagem imperativa e comece cada passo com um verbo de ação. Forneça detalhes e explicações necessários para garantir que o leitor possa completar a tarefa com sucesso. Se a descrição original for pouco clara, ambígua ou carecer de informações suficientes, peça esclarecimentos ou detalhes adicionais.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Para fazer uma xícara de chá, comece fervendo um pouco de água em uma chaleira. Enquanto a água está esquentando, pegue uma caneca e coloque um saquinho de chá nela. Uma vez que a água esteja fervendo, despeje-a cuidadosamente na caneca, sobre o saquinho de chá. Deixe o chá em infusão por alguns minutos, dependendo de quão forte você gosta. Após a infusão, remova o saquinho de chá e adicione leite e açúcar a gosto. Misture tudo junto e aproveite sua xícara de chá quente."
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
        system="Sua tarefa é pegar a descrição em linguagem natural fornecida de um processo ou tarefa e transformá-la em instruções passo a passo claras, concisas, lógicas, sequenciais e fáceis de seguir. Use linguagem imperativa e comece cada passo com um verbo de ação. Forneça detalhes e explicações necessários para garantir que o leitor possa completar a tarefa com sucesso. Se a descrição original for pouco clara, ambígua ou carecer de informações suficientes, peça esclarecimentos ou detalhes adicionais.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Para fazer uma xícara de chá, comece fervendo um pouco de água em uma chaleira. Enquanto a água está esquentando, pegue uma caneca e coloque um saquinho de chá nela. Uma vez que a água esteja fervendo, despeje-a cuidadosamente na caneca, sobre o saquinho de chá. Deixe o chá em infusão por alguns minutos, dependendo de quão forte você gosta. Após a infusão, remova o saquinho de chá e adicione leite e açúcar a gosto. Misture tudo junto e aproveite sua xícara de chá quente."
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
      system: "Sua tarefa é pegar a descrição em linguagem natural fornecida de um processo ou tarefa e transformá-la em instruções passo a passo claras, concisas, lógicas, sequenciais e fáceis de seguir. Use linguagem imperativa e comece cada passo com um verbo de ação. Forneça detalhes e explicações necessários para garantir que o leitor possa completar a tarefa com sucesso. Se a descrição original for pouco clara, ambígua ou carecer de informações suficientes, peça esclarecimentos ou detalhes adicionais.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Para fazer uma xícara de chá, comece fervendo um pouco de água em uma chaleira. Enquanto a água está esquentando, pegue uma caneca e coloque um saquinho de chá nela. Uma vez que a água esteja fervendo, despeje-a cuidadosamente na caneca, sobre o saquinho de chá. Deixe o chá em infusão por alguns minutos, dependendo de quão forte você gosta. Após a infusão, remova o saquinho de chá e adicione leite e açúcar a gosto. Misture tudo junto e aproveite sua xícara de chá quente."
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
        system="Sua tarefa é pegar a descrição em linguagem natural fornecida de um processo ou tarefa e transformá-la em instruções passo a passo claras, concisas, lógicas, sequenciais e fáceis de seguir. Use linguagem imperativa e comece cada passo com um verbo de ação. Forneça detalhes e explicações necessários para garantir que o leitor possa completar a tarefa com sucesso. Se a descrição original for pouco clara, ambígua ou carecer de informações suficientes, peça esclarecimentos ou detalhes adicionais.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Para fazer uma xícara de chá, comece fervendo um pouco de água em uma chaleira. Enquanto a água está esquentando, pegue uma caneca e coloque um saquinho de chá nela. Uma vez que a água esteja fervendo, despeje-a cuidadosamente na caneca, sobre o saquinho de chá. Deixe o chá em infusão por alguns minutos, dependendo de quão forte você gosta. Após a infusão, remova o saquinho de chá e adicione leite e açúcar a gosto. Misture tudo junto e aproveite sua xícara de chá quente."
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
      system: "Sua tarefa é pegar a descrição em linguagem natural fornecida de um processo ou tarefa e transformá-la em instruções passo a passo claras, concisas, lógicas, sequenciais e fáceis de seguir. Use linguagem imperativa e comece cada passo com um verbo de ação. Forneça detalhes e explicações necessários para garantir que o leitor possa completar a tarefa com sucesso. Se a descrição original for pouco clara, ambígua ou carecer de informações suficientes, peça esclarecimentos ou detalhes adicionais.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Para fazer uma xícara de chá, comece fervendo um pouco de água em uma chaleira. Enquanto a água está esquentando, pegue uma caneca e coloque um saquinho de chá nela. Uma vez que a água esteja fervendo, despeje-a cuidadosamente na caneca, sobre o saquinho de chá. Deixe o chá em infusão por alguns minutos, dependendo de quão forte você gosta. Após a infusão, remova o saquinho de chá e adicione leite e açúcar a gosto. Misture tudo junto e aproveite sua xícara de chá quente."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>