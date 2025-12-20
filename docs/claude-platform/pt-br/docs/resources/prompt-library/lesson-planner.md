# Planejador de aulas

Crie planos de aula detalhados sobre qualquer assunto.

---

> Copie este prompt em nosso [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Sua tarefa é criar um plano de aula abrangente, envolvente e bem estruturado sobre o assunto dado. O plano de aula deve ser projetado para uma sessão de aula de 60 minutos e deve atender a um nível de série ou faixa etária específica. Comece declarando os objetivos da aula, que devem ser claros, mensuráveis e alinhados com padrões educacionais relevantes. Em seguida, forneça um esboço detalhado da aula, dividindo-a em uma introdução, atividades principais e uma conclusão. Para cada seção, descreva os métodos de ensino, atividades de aprendizagem e recursos que você usará para transmitir efetivamente o conteúdo e envolver os alunos. Finalmente, descreva os métodos de avaliação que você empregará para avaliar a compreensão e domínio dos objetivos da aula pelos alunos. O plano de aula deve ser bem organizado, fácil de seguir e promover aprendizagem ativa e pensamento crítico. |
| Usuário   | Assunto: Introdução à Fotossíntese Nível de Série: 7ª Série (Idades 12-13) |

## Exemplo de saída

> Plano de Aula: Introdução à Fotossíntese
> Nível de Série: 7ª Série (Idades 12-13)
> Duração: 60 minutos
>
> Objetivos da Aula:
> Ao final da aula, os alunos serão capazes de:
>
> 1. Definir fotossíntese e explicar sua importância no ecossistema.
> 2. Identificar os componentes-chave necessários para que a fotossíntese ocorra.
> 3. Descrever o processo de fotossíntese, incluindo os papéis da energia luminosa, dióxido de carbono e água.
> 4. Reconhecer os produtos da fotossíntese e sua significância para organismos vivos.
>    Esboço da Aula:
>
> 5. Introdução (10 minutos)
>    i. Envolva os alunos mostrando um clipe de vídeo curto ou imagens de plantas e discutindo seu papel no meio ambiente.
>    ii. Introduza o conceito de fotossíntese e sua importância para sustentar a vida na Terra.
>    iii. Declare os objetivos da aula e forneça uma visão geral da estrutura da aula.
> 6. Atividades Principais (40 minutos)
>    i. Instrução Guiada (15 minutos)
>    a. Explique os componentes-chave necessários para a fotossíntese: luz solar, dióxido de carbono, água e clorofila.
>    b. Descreva o processo de fotossíntese, enfatizando os papéis da energia luminosa, dióxido de carbono e água.
>    c. Discuta os produtos da fotossíntese (glicose e oxigênio) e sua significância para organismos vivos.
>    ii. Atividade em Pequenos Grupos (15 minutos)
>    a. Divida a turma em pequenos grupos e forneça a cada grupo uma planilha ou organizador gráfico.
>    b. Peça aos alunos para trabalharem juntos para criar uma representação visual do processo de fotossíntese, rotulando os componentes-chave e produtos.
>    c. Encoraje os alunos a discutir e explicar sua compreensão da fotossíntese dentro de seus grupos.
>    iii. Discussão em Classe (10 minutos)
>    a. Convide cada grupo para compartilhar sua representação visual com a turma.
>    b. Facilite uma discussão em classe, encorajando os alunos a fazer perguntas, fornecer feedback e esclarecer quaisquer concepções errôneas.
> 7. Conclusão (10 minutos)
>    i. Resuma os pontos-chave da aula, reforçando a definição, componentes, processo e produtos da fotossíntese.
>    ii. Administre um questionário curto ou bilhete de saída para avaliar a compreensão dos alunos sobre os objetivos da aula.
>    iii. Forneça aos alunos uma tarefa de casa para explorar mais a importância da fotossíntese em diferentes ecossistemas.
>    Métodos de Avaliação:
>
> 8. Avaliação formativa: Observe a participação e compreensão dos alunos durante a instrução guiada e atividade em pequenos grupos.
> 9. Avaliação somativa: Avalie as representações visuais dos alunos e seu desempenho no questionário curto ou bilhete de saída.
> 10. Tarefa de casa: Revise as submissões de tarefa de casa dos alunos para avaliar sua capacidade de aplicar os conceitos aprendidos na aula.
>     Recursos:
>
> 11. Clipe de vídeo ou imagens de plantas e seu papel no meio ambiente
> 12. Planilhas ou organizadores gráficos para a atividade em pequenos grupos
> 13. Questionário curto ou bilhete de saída para avaliação
> 14. Instruções da tarefa de casa

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
        max_tokens=4000,
        temperature=0.5,
        system="Sua tarefa é criar um plano de aula abrangente, envolvente e bem estruturado sobre o assunto dado. O plano de aula deve ser projetado para uma sessão de aula de 60 minutos e deve atender a um nível de série ou faixa etária específica. Comece declarando os objetivos da aula, que devem ser claros, mensuráveis e alinhados com padrões educacionais relevantes. Em seguida, forneça um esboço detalhado da aula, dividindo-a em uma introdução, atividades principais e uma conclusão. Para cada seção, descreva os métodos de ensino, atividades de aprendizagem e recursos que você usará para transmitir efetivamente o conteúdo e envolver os alunos. Finalmente, descreva os métodos de avaliação que você empregará para avaliar a compreensão e domínio dos objetivos da aula pelos alunos. O plano de aula deve ser bem organizado, fácil de seguir e promover aprendizagem ativa e pensamento crítico.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Assunto: Introdução à Fotossíntese  \nNível de Série: 7ª Série (Idades 12-13)"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "Sua tarefa é criar um plano de aula abrangente, envolvente e bem estruturado sobre o assunto dado. O plano de aula deve ser projetado para uma sessão de aula de 60 minutos e deve atender a um nível de série ou faixa etária específica. Comece declarando os objetivos da aula, que devem ser claros, mensuráveis e alinhados com padrões educacionais relevantes. Em seguida, forneça um esboço detalhado da aula, dividindo-a em uma introdução, atividades principais e uma conclusão. Para cada seção, descreva os métodos de ensino, atividades de aprendizagem e recursos que você usará para transmitir efetivamente o conteúdo e envolver os alunos. Finalmente, descreva os métodos de avaliação que você empregará para avaliar a compreensão e domínio dos objetivos da aula pelos alunos. O plano de aula deve ser bem organizado, fácil de seguir e promover aprendizagem ativa e pensamento crítico.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Assunto: Introdução à Fotossíntese  \nNível de Série: 7ª Série (Idades 12-13)"
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
        max_tokens=4000,
        temperature=0.5,
        system="Sua tarefa é criar um plano de aula abrangente, envolvente e bem estruturado sobre o assunto dado. O plano de aula deve ser projetado para uma sessão de aula de 60 minutos e deve atender a um nível de série ou faixa etária específica. Comece declarando os objetivos da aula, que devem ser claros, mensuráveis e alinhados com padrões educacionais relevantes. Em seguida, forneça um esboço detalhado da aula, dividindo-a em uma introdução, atividades principais e uma conclusão. Para cada seção, descreva os métodos de ensino, atividades de aprendizagem e recursos que você usará para transmitir efetivamente o conteúdo e envolver os alunos. Finalmente, descreva os métodos de avaliação que você empregará para avaliar a compreensão e domínio dos objetivos da aula pelos alunos. O plano de aula deve ser bem organizado, fácil de seguir e promover aprendizagem ativa e pensamento crítico.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Assunto: Introdução à Fotossíntese  \nNível de Série: 7ª Série (Idades 12-13)"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "Sua tarefa é criar um plano de aula abrangente, envolvente e bem estruturado sobre o assunto dado. O plano de aula deve ser projetado para uma sessão de aula de 60 minutos e deve atender a um nível de série ou faixa etária específica. Comece declarando os objetivos da aula, que devem ser claros, mensuráveis e alinhados com padrões educacionais relevantes. Em seguida, forneça um esboço detalhado da aula, dividindo-a em uma introdução, atividades principais e uma conclusão. Para cada seção, descreva os métodos de ensino, atividades de aprendizagem e recursos que você usará para transmitir efetivamente o conteúdo e envolver os alunos. Finalmente, descreva os métodos de avaliação que você empregará para avaliar a compreensão e domínio dos objetivos da aula pelos alunos. O plano de aula deve ser bem organizado, fácil de seguir e promover aprendizagem ativa e pensamento crítico.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Assunto: Introdução à Fotossíntese  \nNível de Série: 7ª Série (Idades 12-13)"
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
        max_tokens=4000,
        temperature=0.5,
        system="Sua tarefa é criar um plano de aula abrangente, envolvente e bem estruturado sobre o assunto dado. O plano de aula deve ser projetado para uma sessão de aula de 60 minutos e deve atender a um nível de série ou faixa etária específica. Comece declarando os objetivos da aula, que devem ser claros, mensuráveis e alinhados com padrões educacionais relevantes. Em seguida, forneça um esboço detalhado da aula, dividindo-a em uma introdução, atividades principais e uma conclusão. Para cada seção, descreva os métodos de ensino, atividades de aprendizagem e recursos que você usará para transmitir efetivamente o conteúdo e envolver os alunos. Finalmente, descreva os métodos de avaliação que você empregará para avaliar a compreensão e domínio dos objetivos da aula pelos alunos. O plano de aula deve ser bem organizado, fácil de seguir e promover aprendizagem ativa e pensamento crítico.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Assunto: Introdução à Fotossíntese  \nNível de Série: 7ª Série (Idades 12-13)"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "Sua tarefa é criar um plano de aula abrangente, envolvente e bem estruturado sobre o assunto dado. O plano de aula deve ser projetado para uma sessão de aula de 60 minutos e deve atender a um nível de série ou faixa etária específica. Comece declarando os objetivos da aula, que devem ser claros, mensuráveis e alinhados com padrões educacionais relevantes. Em seguida, forneça um esboço detalhado da aula, dividindo-a em uma introdução, atividades principais e uma conclusão. Para cada seção, descreva os métodos de ensino, atividades de aprendizagem e recursos que você usará para transmitir efetivamente o conteúdo e envolver os alunos. Finalmente, descreva os métodos de avaliação que você empregará para avaliar a compreensão e domínio dos objetivos da aula pelos alunos. O plano de aula deve ser bem organizado, fácil de seguir e promover aprendizagem ativa e pensamento crítico.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Assunto: Introdução à Fotossíntese  \nNível de Série: 7ª Série (Idades 12-13)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>