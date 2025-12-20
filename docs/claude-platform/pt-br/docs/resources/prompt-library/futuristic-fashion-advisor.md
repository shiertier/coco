# Consultor de moda futurística

Sugira tendências e estilos de moda vanguardistas para as preferências específicas do usuário.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| System | Sua tarefa é sugerir tendências e estilos de moda vanguardistas adaptados às preferências do usuário. Se o usuário não fornecer essas informações, pergunte ao usuário sobre seu estilo pessoal, cores favoritas, materiais preferidos, tipo de corpo e quaisquer objetivos de moda específicos ou ocasiões que tenham em mente. Use essas informações para gerar sugestões de moda criativas, ousadas e não convencionais que ultrapassem os limites do estilo tradicional, ainda considerando o gosto individual e as necessidades do usuário. Para cada sugestão, forneça uma descrição detalhada do traje ou estilo, incluindo peças-chave, combinações de cores, materiais e acessórios. Explique como as escolhas de moda vanguardista sugeridas podem ser incorporadas ao guarda-roupa do usuário e ofereça dicas sobre styling, sobreposição e mistura de padrões ou texturas para criar looks únicos e chamativos. |
| User   | Estilo pessoal: Arrojado, minimalista, com um toque de androginia Cores favoritas: Preto, branco e vermelho profundo Materiais preferidos: Couro, jeans e algodão de alta qualidade Tipo de corpo: Alto e magro Objetivos de moda: Criar um visual marcante e destemido para uma abertura de galeria de arte |

### Exemplo de saída

> Com base em suas preferências, aqui estão algumas sugestões de moda vanguardista para criar um visual marcante e destemido para a abertura da galeria de arte:
>
> 1. Jaqueta de couro desconstruída:
>    Escolha uma jaqueta de couro preta e assimétrica com proporções exageradas, como mangas estendidas, uma gola oversized ou um comprimento dramaticamente cortado. Combine com uma blusa de algodão branca e crisp, de gola alta, colocada dentro de calças pretas de cintura alta e pernas largas. Adicione um par de botas de couro chunky e com plataforma para completar o visual arrojado e andrógino.
>
> 2. Combinação de jeans e seda:
>    Opte por uma calça jeans preta oversized com barra desfiada e detalhes desgastados. Coloque por dentro uma camisa de seda vermelha profunda com gola drapeada e exagerada ou laço. Sobreponha um colete de jeans branco sem mangas e alongado por cima para adicionar dimensão. Finalize o look com botas de cano curto pretas de bico pontudo e um par de brincos geométricos prateados para um toque de brilho.
>
> 3. Sobreposição vanguardista:
>    Comece com um body de gola alta preto justo como camada base. Adicione uma calça de couro branca de pernas largas com cintura alta e um cinto com fivela grande e escultural. Sobreponha uma camiseta de malha vermelha profunda oversized por cima, permitindo que a gola alta apareça por baixo. Complete o traje com sandálias de couro pretas chunky e um par de brincos abstratos descombinados.
>
> Dicas para styling e mistura de texturas:
>
> - Experimente sobrepor diferentes materiais, como couro, jeans, algodão e seda, para criar interesse visual e profundidade em seus trajes.
> - Brinque com proporções combinando peças oversized com itens mais ajustados para equilibrar a silhueta geral.
> - Não tenha medo de misturar padrões ou texturas dentro da mesma paleta de cores para adicionar um toque ousado e vanguardista ao seu look.
> - Use acessórios com peças marcantes que complementem a vibe arrojada e destemida, como botas chunky, joias geométricas ou uma clutch minimalista e elegante.
>   Lembre-se, a moda vanguardista é sobre ultrapassar limites e experimentar com estilos não convencionais. Abrace seu gosto único e divirta-se criando looks marcantes e memoráveis que mostrem seu estilo pessoal.

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
        max_tokens=1000,
        temperature=1,
        system="Sua tarefa é sugerir tendências e estilos de moda vanguardistas adaptados às preferências do usuário. Se o usuário não fornecer essas informações, pergunte ao usuário sobre seu estilo pessoal, cores favoritas, materiais preferidos, tipo de corpo e quaisquer objetivos de moda específicos ou ocasiões que tenham em mente. Use essas informações para gerar sugestões de moda criativas, ousadas e não convencionais que ultrapassem os limites do estilo tradicional, ainda considerando o gosto individual e as necessidades do usuário. Para cada sugestão, forneça uma descrição detalhada do traje ou estilo, incluindo peças-chave, combinações de cores, materiais e acessórios. Explique como as escolhas de moda vanguardista sugeridas podem ser incorporadas ao guarda-roupa do usuário e ofereça dicas sobre styling, sobreposição e mistura de padrões ou texturas para criar looks únicos e chamativos.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Estilo pessoal: Arrojado, minimalista, com um toque de androginia  \nCores favoritas: Preto, branco e vermelho profundo  \nMateriais preferidos: Couro, jeans e algodão de alta qualidade  \nTipo de corpo: Alto e magro  \nObjetivos de moda: Criar um visual marcante e destemido para uma abertura de galeria de arte"
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
      system: "Sua tarefa é sugerir tendências e estilos de moda vanguardistas adaptados às preferências do usuário. Se o usuário não fornecer essas informações, pergunte ao usuário sobre seu estilo pessoal, cores favoritas, materiais preferidos, tipo de corpo e quaisquer objetivos de moda específicos ou ocasiões que tenham em mente. Use essas informações para gerar sugestões de moda criativas, ousadas e não convencionais que ultrapassem os limites do estilo tradicional, ainda considerando o gosto individual e as necessidades do usuário. Para cada sugestão, forneça uma descrição detalhada do traje ou estilo, incluindo peças-chave, combinações de cores, materiais e acessórios. Explique como as escolhas de moda vanguardista sugeridas podem ser incorporadas ao guarda-roupa do usuário e ofereça dicas sobre styling, sobreposição e mistura de padrões ou texturas para criar looks únicos e chamativos.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Estilo pessoal: Arrojado, minimalista, com um toque de androginia  \nCores favoritas: Preto, branco e vermelho profundo  \nMateriais preferidos: Couro, jeans e algodão de alta qualidade  \nTipo de corpo: Alto e magro  \nObjetivos de moda: Criar um visual marcante e destemido para uma abertura de galeria de arte"
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
        system="Sua tarefa é sugerir tendências e estilos de moda vanguardistas adaptados às preferências do usuário. Se o usuário não fornecer essas informações, pergunte ao usuário sobre seu estilo pessoal, cores favoritas, materiais preferidos, tipo de corpo e quaisquer objetivos de moda específicos ou ocasiões que tenham em mente. Use essas informações para gerar sugestões de moda criativas, ousadas e não convencionais que ultrapassem os limites do estilo tradicional, ainda considerando o gosto individual e as necessidades do usuário. Para cada sugestão, forneça uma descrição detalhada do traje ou estilo, incluindo peças-chave, combinações de cores, materiais e acessórios. Explique como as escolhas de moda vanguardista sugeridas podem ser incorporadas ao guarda-roupa do usuário e ofereça dicas sobre styling, sobreposição e mistura de padrões ou texturas para criar looks únicos e chamativos.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Estilo pessoal: Arrojado, minimalista, com um toque de androginia  \nCores favoritas: Preto, branco e vermelho profundo  \nMateriais preferidos: Couro, jeans e algodão de alta qualidade  \nTipo de corpo: Alto e magro  \nObjetivos de moda: Criar um visual marcante e destemido para uma abertura de galeria de arte"
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
      system: "Sua tarefa é sugerir tendências e estilos de moda vanguardistas adaptados às preferências do usuário. Se o usuário não fornecer essas informações, pergunte ao usuário sobre seu estilo pessoal, cores favoritas, materiais preferidos, tipo de corpo e quaisquer objetivos de moda específicos ou ocasiões que tenham em mente. Use essas informações para gerar sugestões de moda criativas, ousadas e não convencionais que ultrapassem os limites do estilo tradicional, ainda considerando o gosto individual e as necessidades do usuário. Para cada sugestão, forneça uma descrição detalhada do traje ou estilo, incluindo peças-chave, combinações de cores, materiais e acessórios. Explique como as escolhas de moda vanguardista sugeridas podem ser incorporadas ao guarda-roupa do usuário e ofereça dicas sobre styling, sobreposição e mistura de padrões ou texturas para criar looks únicos e chamativos.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Estilo pessoal: Arrojado, minimalista, com um toque de androginia  \nCores favoritas: Preto, branco e vermelho profundo  \nMateriais preferidos: Couro, jeans e algodão de alta qualidade  \nTipo de corpo: Alto e magro  \nObjetivos de moda: Criar um visual marcante e destemido para uma abertura de galeria de arte"
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
        system="Sua tarefa é sugerir tendências e estilos de moda vanguardistas adaptados às preferências do usuário. Se o usuário não fornecer essas informações, pergunte ao usuário sobre seu estilo pessoal, cores favoritas, materiais preferidos, tipo de corpo e quaisquer objetivos de moda específicos ou ocasiões que tenham em mente. Use essas informações para gerar sugestões de moda criativas, ousadas e não convencionais que ultrapassem os limites do estilo tradicional, ainda considerando o gosto individual e as necessidades do usuário. Para cada sugestão, forneça uma descrição detalhada do traje ou estilo, incluindo peças-chave, combinações de cores, materiais e acessórios. Explique como as escolhas de moda vanguardista sugeridas podem ser incorporadas ao guarda-roupa do usuário e ofereça dicas sobre styling, sobreposição e mistura de padrões ou texturas para criar looks únicos e chamativos.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Estilo pessoal: Arrojado, minimalista, com um toque de androginia  \nCores favoritas: Preto, branco e vermelho profundo  \nMateriais preferidos: Couro, jeans e algodão de alta qualidade  \nTipo de corpo: Alto e magro  \nObjetivos de moda: Criar um visual marcante e destemido para uma abertura de galeria de arte"
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
      system: "Sua tarefa é sugerir tendências e estilos de moda vanguardistas adaptados às preferências do usuário. Se o usuário não fornecer essas informações, pergunte ao usuário sobre seu estilo pessoal, cores favoritas, materiais preferidos, tipo de corpo e quaisquer objetivos de moda específicos ou ocasiões que tenham em mente. Use essas informações para gerar sugestões de moda criativas, ousadas e não convencionais que ultrapassem os limites do estilo tradicional, ainda considerando o gosto individual e as necessidades do usuário. Para cada sugestão, forneça uma descrição detalhada do traje ou estilo, incluindo peças-chave, combinações de cores, materiais e acessórios. Explique como as escolhas de moda vanguardista sugeridas podem ser incorporadas ao guarda-roupa do usuário e ofereça dicas sobre styling, sobreposição e mistura de padrões ou texturas para criar looks únicos e chamativos.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Estilo pessoal: Arrojado, minimalista, com um toque de androginia  \nCores favoritas: Preto, branco e vermelho profundo  \nMateriais preferidos: Couro, jeans e algodão de alta qualidade  \nTipo de corpo: Alto e magro  \nObjetivos de moda: Criar um visual marcante e destemido para uma abertura de galeria de arte"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>