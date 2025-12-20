# Construtor de marca

Crie um briefing de design para uma identidade de marca holística.

---

> Copie este prompt em nosso [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| System | Sua tarefa é criar um briefing de design abrangente para uma identidade de marca holística baseada nas especificações fornecidas. A identidade da marca deve abranger vários elementos, como sugestões para o nome da marca, logotipo, paleta de cores, tipografia, estilo visual, tom de voz e personalidade geral da marca. Certifique-se de que todos os elementos funcionem juntos harmoniosamente para criar uma experiência de marca coesa e memorável que comunique efetivamente os valores, missão e proposta única de venda da marca para seu público-alvo. Seja detalhado e abrangente e forneça detalhes específicos suficientes para que alguém possa criar uma identidade de marca verdadeiramente única. |
| User   | Especificações da marca: Esta é uma marca que se concentra em criar roupas e acessórios de alta qualidade e estilosos usando materiais ecológicos e métodos de produção éticos. A marca tem como alvo consumidores ambientalmente conscientes com idades entre 25-40 anos que valorizam moda, sustentabilidade e responsabilidade social. A identidade da marca deve alcançar os seguintes objetivos: 1. Refletir o compromisso da marca com sustentabilidade, práticas éticas e administração ambiental. 2. Atrair o público-alvo transmitindo uma sensação de estilo, qualidade e tendência. 3. Diferenciar a marca dos concorrentes no mercado de moda sustentável. 4. Criar uma conexão emocional forte com os consumidores e inspirá-los a fazer escolhas mais ambientalmente amigáveis. |

### Exemplo de saída

> Briefing de Design: Identidade de Marca para uma Marca de Moda Ecológica
> Sugestão de Nome da Marca: "Sēnsus" (latim para "sentido" ou "percepção", evocando uma sensação de consciência e conexão com o meio ambiente)
>
> Logotipo:
>
> - Um logotipo minimalista e moderno que combina um motivo estilizado de folha ou planta com uma tipografia elegante e sem serifa.
> - O elemento de folha ou planta simboliza o compromisso da marca com sustentabilidade e administração ambiental, enquanto a tipografia limpa transmite uma sensação de sofisticação e estilo.
> - O logotipo deve ser versátil e funcionar bem em várias aplicações, incluindo etiquetas de roupas, embalagens e plataformas digitais.
>   Paleta de Cores:
>
> - Uma paleta de cores neutra e terrosa que reflete o ethos ecológico da marca, como tons de verde, bege e cinzas quentes.
> - Cores de destaque como azul-petróleo profundo ou amarelo mostarda podem ser usadas com moderação para adicionar vibração e profundidade.
> - A paleta de cores deve evocar uma sensação de harmonia e equilíbrio natural, enquanto ainda é moderna e atraente para o público-alvo.
>   Tipografia:
>
> - Uma combinação de uma tipografia sans-serif limpa e moderna para títulos e uma tipografia serif complementar para o corpo do texto.
> - A tipografia deve ser legível e visualmente atraente, transmitindo uma sensação de qualidade e sofisticação.
> - O uso consistente de hierarquia tipográfica e espaçamento criará uma identidade de marca coesa e organizada.
>   Estilo Visual:
>
> - Um estilo visual minimalista e desorganizado que enfatiza fotografia de produto de alta qualidade e layouts limpos e modernos.
> - Incorporar elementos naturais, como motivos de plantas ou texturas, para reforçar os valores ecológicos da marca e conexão com a natureza.
> - Usar espaço negativo efetivamente para criar uma sensação de abertura e respirabilidade, refletindo o compromisso da marca com sustentabilidade.
>   Tom de Voz:
>
> - O tom de voz da marca deve ser acessível, conhecedor e autêntico.
> - Usar linguagem que eduque e inspire os consumidores sobre os benefícios da moda sustentável e métodos de produção éticos.
> - Incorporar um toque de sofisticação e estilo para atrair as sensibilidades conscientes da moda do público-alvo.
>   Personalidade da Marca:
>
> - Eco-consciente e ambientalmente consciente, mas elegante e na vanguarda da moda.
> - Confiante e apaixonada por promover práticas sustentáveis e escolhas éticas.
> - Inclusiva e acolhedora, convidando os consumidores a se juntarem a uma comunidade de pessoas com ideias semelhantes comprometidas em fazer mudanças positivas.
>   No geral, a identidade da marca para "Sēnsus" deve encontrar um equilíbrio entre valores ecológicos e estética moderna, atraindo consumidores que querem parecer elegantes enquanto fazem escolhas ambientalmente responsáveis. Os elementos visuais e verbais coesos devem criar uma conexão emocional forte com o público-alvo e inspirá-los a abraçar um estilo de vida mais sustentável.

### Solicitação de API

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
        system="Sua tarefa é criar um briefing de design abrangente para uma identidade de marca holística baseada nas especificações fornecidas. A identidade da marca deve abranger vários elementos, como sugestões para o nome da marca, logotipo, paleta de cores, tipografia, estilo visual, tom de voz e personalidade geral da marca. Certifique-se de que todos os elementos funcionem juntos harmoniosamente para criar uma experiência de marca coesa e memorável que comunique efetivamente os valores, missão e proposta única de venda da marca para seu público-alvo. Seja detalhado e abrangente e forneça detalhes específicos suficientes para que alguém possa criar uma identidade de marca verdadeiramente única.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Especificações da marca:\nEsta é uma marca que se concentra em criar roupas e acessórios de alta qualidade e estilosos usando materiais ecológicos e métodos de produção éticos\nA marca tem como alvo consumidores ambientalmente conscientes com idades entre 25-40 anos que valorizam moda, sustentabilidade e responsabilidade social.\nA identidade da marca deve alcançar os seguintes objetivos:\n1. Refletir o compromisso da marca com sustentabilidade, práticas éticas e administração ambiental.\n2. Atrair o público-alvo transmitindo uma sensação de estilo, qualidade e tendência.\n3. Diferenciar a marca dos concorrentes no mercado de moda sustentável.\n4. Criar uma conexão emocional forte com os consumidores e inspirá-los a fazer escolhas mais ambientalmente amigáveis."
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
      system: "Sua tarefa é criar um briefing de design abrangente para uma identidade de marca holística baseada nas especificações fornecidas. A identidade da marca deve abranger vários elementos, como sugestões para o nome da marca, logotipo, paleta de cores, tipografia, estilo visual, tom de voz e personalidade geral da marca. Certifique-se de que todos os elementos funcionem juntos harmoniosamente para criar uma experiência de marca coesa e memorável que comunique efetivamente os valores, missão e proposta única de venda da marca para seu público-alvo. Seja detalhado e abrangente e forneça detalhes específicos suficientes para que alguém possa criar uma identidade de marca verdadeiramente única.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Especificações da marca:\nEsta é uma marca que se concentra em criar roupas e acessórios de alta qualidade e estilosos usando materiais ecológicos e métodos de produção éticos\nA marca tem como alvo consumidores ambientalmente conscientes com idades entre 25-40 anos que valorizam moda, sustentabilidade e responsabilidade social.\nA identidade da marca deve alcançar os seguintes objetivos:\n1. Refletir o compromisso da marca com sustentabilidade, práticas éticas e administração ambiental.\n2. Atrair o público-alvo transmitindo uma sensação de estilo, qualidade e tendência.\n3. Diferenciar a marca dos concorrentes no mercado de moda sustentável.\n4. Criar uma conexão emocional forte com os consumidores e inspirá-los a fazer escolhas mais ambientalmente amigáveis."
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
        system="Sua tarefa é criar um briefing de design abrangente para uma identidade de marca holística baseada nas especificações fornecidas. A identidade da marca deve abranger vários elementos, como sugestões para o nome da marca, logotipo, paleta de cores, tipografia, estilo visual, tom de voz e personalidade geral da marca. Certifique-se de que todos os elementos funcionem juntos harmoniosamente para criar uma experiência de marca coesa e memorável que comunique efetivamente os valores, missão e proposta única de venda da marca para seu público-alvo. Seja detalhado e abrangente e forneça detalhes específicos suficientes para que alguém possa criar uma identidade de marca verdadeiramente única.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Especificações da marca:\nEsta é uma marca que se concentra em criar roupas e acessórios de alta qualidade e estilosos usando materiais ecológicos e métodos de produção éticos\nA marca tem como alvo consumidores ambientalmente conscientes com idades entre 25-40 anos que valorizam moda, sustentabilidade e responsabilidade social.\nA identidade da marca deve alcançar os seguintes objetivos:\n1. Refletir o compromisso da marca com sustentabilidade, práticas éticas e administração ambiental.\n2. Atrair o público-alvo transmitindo uma sensação de estilo, qualidade e tendência.\n3. Diferenciar a marca dos concorrentes no mercado de moda sustentável.\n4. Criar uma conexão emocional forte com os consumidores e inspirá-los a fazer escolhas mais ambientalmente amigáveis."
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
      system: "Sua tarefa é criar um briefing de design abrangente para uma identidade de marca holística baseada nas especificações fornecidas. A identidade da marca deve abranger vários elementos, como sugestões para o nome da marca, logotipo, paleta de cores, tipografia, estilo visual, tom de voz e personalidade geral da marca. Certifique-se de que todos os elementos funcionem juntos harmoniosamente para criar uma experiência de marca coesa e memorável que comunique efetivamente os valores, missão e proposta única de venda da marca para seu público-alvo. Seja detalhado e abrangente e forneça detalhes específicos suficientes para que alguém possa criar uma identidade de marca verdadeiramente única.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Especificações da marca:\nEsta é uma marca que se concentra em criar roupas e acessórios de alta qualidade e estilosos usando materiais ecológicos e métodos de produção éticos\nA marca tem como alvo consumidores ambientalmente conscientes com idades entre 25-40 anos que valorizam moda, sustentabilidade e responsabilidade social.\nA identidade da marca deve alcançar os seguintes objetivos:\n1. Refletir o compromisso da marca com sustentabilidade, práticas éticas e administração ambiental.\n2. Atrair o público-alvo transmitindo uma sensação de estilo, qualidade e tendência.\n3. Diferenciar a marca dos concorrentes no mercado de moda sustentável.\n4. Criar uma conexão emocional forte com os consumidores e inspirá-los a fazer escolhas mais ambientalmente amigáveis."
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
        system="Sua tarefa é criar um briefing de design abrangente para uma identidade de marca holística baseada nas especificações fornecidas. A identidade da marca deve abranger vários elementos, como sugestões para o nome da marca, logotipo, paleta de cores, tipografia, estilo visual, tom de voz e personalidade geral da marca. Certifique-se de que todos os elementos funcionem juntos harmoniosamente para criar uma experiência de marca coesa e memorável que comunique efetivamente os valores, missão e proposta única de venda da marca para seu público-alvo. Seja detalhado e abrangente e forneça detalhes específicos suficientes para que alguém possa criar uma identidade de marca verdadeiramente única.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Especificações da marca:\nEsta é uma marca que se concentra em criar roupas e acessórios de alta qualidade e estilosos usando materiais ecológicos e métodos de produção éticos\nA marca tem como alvo consumidores ambientalmente conscientes com idades entre 25-40 anos que valorizam moda, sustentabilidade e responsabilidade social.\nA identidade da marca deve alcançar os seguintes objetivos:\n1. Refletir o compromisso da marca com sustentabilidade, práticas éticas e administração ambiental.\n2. Atrair o público-alvo transmitindo uma sensação de estilo, qualidade e tendência.\n3. Diferenciar a marca dos concorrentes no mercado de moda sustentável.\n4. Criar uma conexão emocional forte com os consumidores e inspirá-los a fazer escolhas mais ambientalmente amigáveis."
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
      system: "Sua tarefa é criar um briefing de design abrangente para uma identidade de marca holística baseada nas especificações fornecidas. A identidade da marca deve abranger vários elementos, como sugestões para o nome da marca, logotipo, paleta de cores, tipografia, estilo visual, tom de voz e personalidade geral da marca. Certifique-se de que todos os elementos funcionem juntos harmoniosamente para criar uma experiência de marca coesa e memorável que comunique efetivamente os valores, missão e proposta única de venda da marca para seu público-alvo. Seja detalhado e abrangente e forneça detalhes específicos suficientes para que alguém possa criar uma identidade de marca verdadeiramente única.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Especificações da marca:\nEsta é uma marca que se concentra em criar roupas e acessórios de alta qualidade e estilosos usando materiais ecológicos e métodos de produção éticos\nA marca tem como alvo consumidores ambientalmente conscientes com idades entre 25-40 anos que valorizam moda, sustentabilidade e responsabilidade social.\nA identidade da marca deve alcançar os seguintes objetivos:\n1. Refletir o compromisso da marca com sustentabilidade, práticas éticas e administração ambiental.\n2. Atrair o público-alvo transmitindo uma sensação de estilo, qualidade e tendência.\n3. Diferenciar a marca dos concorrentes no mercado de moda sustentável.\n4. Criar uma conexão emocional forte com os consumidores e inspirá-los a fazer escolhas mais ambientalmente amigáveis."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>