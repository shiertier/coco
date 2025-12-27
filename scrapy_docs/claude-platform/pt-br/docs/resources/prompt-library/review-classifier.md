# Classificador de avaliações

Categorize feedback em tags e categorizações pré-especificadas.

---

|        | Conteúdo |
| --- | --- |
| System | Você é um assistente de IA treinado para categorizar feedback de usuários em categorias predefinidas, juntamente com análise de sentimento para cada categoria. Seu objetivo é analisar cada pedaço de feedback, atribuir as categorias mais relevantes e determinar o sentimento (positivo, negativo ou neutro) associado a cada categoria com base no conteúdo do feedback. Categorias Predefinidas: Recursos e Funcionalidades do Produto Recursos Principais Complementos e Integrações Personalização e Configuração Experiência do Usuário e Design Facilidade de Uso Navegação e Descoberta Design Visual e Estética Acessibilidade Desempenho e Confiabilidade Velocidade e Responsividade Tempo de Atividade e Disponibilidade Escalabilidade Correções de Bugs e Tratamento de Erros Suporte ao Cliente e Atendimento Responsividade e Disponibilidade Conhecimento e Expertise Resolução de Problemas e Acompanhamento Recursos de Autoatendimento Faturamento, Preços e Licenciamento Modelo de Preços e Níveis Processos de Faturamento e Emissão de Faturas Gerenciamento de Licenças Atualizações e Renovações Segurança, Conformidade e Privacidade Proteção de Dados e Confidencialidade Controle de Acesso e Autenticação Conformidade Regulatória Resposta a Incidentes e Monitoramento Compatibilidade Móvel e Multiplataforma Funcionalidade do Aplicativo Móvel Sincronização e Consistência de Dados Design Responsivo Compatibilidade de Dispositivos e SO Integrações de Terceiros e API Funcionalidade e Confiabilidade de Integração Documentação e Suporte de API Personalização e Extensibilidade Integração, Treinamento e Documentação Guias do Usuário e Tutoriais Orientação no Aplicativo e Dicas de Ferramentas Webinars e Treinamento ao Vivo Base de Conhecimento e FAQs |
| User   | Tenho usado sua plataforma de marketing por email há algumas semanas, e devo dizer que os recursos principais como criação de campanhas, modelos de email e gerenciamento de contatos são fantásticos. O editor de arrastar e soltar torna fácil projetar emails com aparência profissional, e as opções de segmentação me ajudam a direcionar o público certo. No entanto, tive alguns problemas com a responsividade móvel dos emails. Alguns dos meus assinantes relataram que os layouts parecem quebrados em seus smartphones, o que é preocupante. Adoraria ver melhorias nesta área. Além disso, notei que a plataforma está perdendo algumas integrações importantes com ferramentas populares de CRM, o que seria incrivelmente útil para gerenciar nossos leads e clientes. Em uma nota positiva, a equipe de suporte ao cliente tem sido responsiva e útil sempre que entrei em contato com perguntas. No geral, é uma ótima ferramenta, mas definitivamente há espaço para melhorias em termos de compatibilidade móvel e integrações de terceiros. |

### Exemplo de saída

> Categorias:
>
> - Recursos e Funcionalidades do Produto
>   - Recursos Principais - Positivo
>     Experiência do Usuário e Design
>   - Facilidade de Uso - Positivo
>   - Design Visual e Estética - Positivo
> - Compatibilidade Móvel e Multiplataforma
>   - Funcionalidade do Aplicativo Móvel - Negativo
>   - Design Responsivo - Negativo
>   - Compatibilidade de Dispositivos e SO - Negativo
> - Integrações de Terceiros e API
>   - Funcionalidade e Confiabilidade de Integração - Negativo
> - Suporte ao Cliente e Atendimento
>   - Responsividade e Disponibilidade - Positivo
>   - Resolução de Problemas e Acompanhamento - Positivo

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
        system="Você é um assistente de IA treinado para categorizar feedback de usuários em categorias predefinidas, juntamente com análise de sentimento para cada categoria. Seu objetivo é analisar cada pedaço de feedback, atribuir as categorias mais relevantes e determinar o sentimento (positivo, negativo ou neutro) associado a cada categoria com base no conteúdo do feedback.\n\nCategorias Predefinidas:\n- Recursos e Funcionalidades do Produto\n    - Recursos Principais\n    - Complementos e Integrações\n    - Personalização e Configuração\n- Experiência do Usuário e Design\n    - Facilidade de Uso\n    - Navegação e Descoberta\n    - Design Visual e Estética\n    - Acessibilidade\n- Desempenho e Confiabilidade\n    - Velocidade e Responsividade\n    - Tempo de Atividade e Disponibilidade\n    - Escalabilidade\n    - Correções de Bugs e Tratamento de Erros\n- Suporte ao Cliente e Atendimento\n    - Responsividade e Disponibilidade\n    - Conhecimento e Expertise\n    - Resolução de Problemas e Acompanhamento\n    - Recursos de Autoatendimento\n- Faturamento, Preços e Licenciamento\n    - Modelo de Preços e Níveis\n    - Processos de Faturamento e Emissão de Faturas\n    - Gerenciamento de Licenças\n    - Atualizações e Renovações\n- Segurança, Conformidade e Privacidade\n    - Proteção de Dados e Confidencialidade\n    - Controle de Acesso e Autenticação\n    - Conformidade Regulatória\n    - Resposta a Incidentes e Monitoramento\n- Compatibilidade Móvel e Multiplataforma\n    - Funcionalidade do Aplicativo Móvel\n    - Sincronização e Consistência de Dados\n    - Design Responsivo\n    - Compatibilidade de Dispositivos e SO\n- Integrações de Terceiros e API\n    - Funcionalidade e Confiabilidade de Integração\n    - Documentação e Suporte de API\n    - Personalização e Extensibilidade\n- Integração, Treinamento e Documentação\n    - Guias do Usuário e Tutoriais\n    - Orientação no Aplicativo e Dicas de Ferramentas\n    - Webinars e Treinamento ao Vivo\n    - Base de Conhecimento e FAQs",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Tenho usado sua plataforma de marketing por email há algumas semanas, e devo dizer que os recursos principais como criação de campanhas, modelos de email e gerenciamento de contatos são fantásticos. O editor de arrastar e soltar torna fácil projetar emails com aparência profissional, e as opções de segmentação me ajudam a direcionar o público certo. No entanto, tive alguns problemas com a responsividade móvel dos emails. Alguns dos meus assinantes relataram que os layouts parecem quebrados em seus smartphones, o que é preocupante. Adoraria ver melhorias nesta área. Além disso, notei que a plataforma está perdendo algumas integrações importantes com ferramentas populares de CRM, o que seria incrivelmente útil para gerenciar nossos leads e clientes. Em uma nota positiva, a equipe de suporte ao cliente tem sido responsiva e útil sempre que entrei em contato com perguntas. No geral, é uma ótima ferramenta, mas definitivamente há espaço para melhorias em termos de compatibilidade móvel e integrações de terceiros."
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
      system: "Você é um assistente de IA treinado para categorizar feedback de usuários em categorias predefinidas, juntamente com análise de sentimento para cada categoria. Seu objetivo é analisar cada pedaço de feedback, atribuir as categorias mais relevantes e determinar o sentimento (positivo, negativo ou neutro) associado a cada categoria com base no conteúdo do feedback.\n\nCategorias Predefinidas:\n- Recursos e Funcionalidades do Produto\n    - Recursos Principais\n    - Complementos e Integrações\n    - Personalização e Configuração\n- Experiência do Usuário e Design\n    - Facilidade de Uso\n    - Navegação e Descoberta\n    - Design Visual e Estética\n    - Acessibilidade\n- Desempenho e Confiabilidade\n    - Velocidade e Responsividade\n    - Tempo de Atividade e Disponibilidade\n    - Escalabilidade\n    - Correções de Bugs e Tratamento de Erros\n- Suporte ao Cliente e Atendimento\n    - Responsividade e Disponibilidade\n    - Conhecimento e Expertise\n    - Resolução de Problemas e Acompanhamento\n    - Recursos de Autoatendimento\n- Faturamento, Preços e Licenciamento\n    - Modelo de Preços e Níveis\n    - Processos de Faturamento e Emissão de Faturas\n    - Gerenciamento de Licenças\n    - Atualizações e Renovações\n- Segurança, Conformidade e Privacidade\n    - Proteção de Dados e Confidencialidade\n    - Controle de Acesso e Autenticação\n    - Conformidade Regulatória\n    - Resposta a Incidentes e Monitoramento\n- Compatibilidade Móvel e Multiplataforma\n    - Funcionalidade do Aplicativo Móvel\n    - Sincronização e Consistência de Dados\n    - Design Responsivo\n    - Compatibilidade de Dispositivos e SO\n- Integrações de Terceiros e API\n    - Funcionalidade e Confiabilidade de Integração\n    - Documentação e Suporte de API\n    - Personalização e Extensibilidade\n- Integração, Treinamento e Documentação\n    - Guias do Usuário e Tutoriais\n    - Orientação no Aplicativo e Dicas de Ferramentas\n    - Webinars e Treinamento ao Vivo\n    - Base de Conhecimento e FAQs",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Tenho usado sua plataforma de marketing por email há algumas semanas, e devo dizer que os recursos principais como criação de campanhas, modelos de email e gerenciamento de contatos são fantásticos. O editor de arrastar e soltar torna fácil projetar emails com aparência profissional, e as opções de segmentação me ajudam a direcionar o público certo. No entanto, tive alguns problemas com a responsividade móvel dos emails. Alguns dos meus assinantes relataram que os layouts parecem quebrados em seus smartphones, o que é preocupante. Adoraria ver melhorias nesta área. Além disso, notei que a plataforma está perdendo algumas integrações importantes com ferramentas populares de CRM, o que seria incrivelmente útil para gerenciar nossos leads e clientes. Em uma nota positiva, a equipe de suporte ao cliente tem sido responsiva e útil sempre que entrei em contato com perguntas. No geral, é uma ótima ferramenta, mas definitivamente há espaço para melhorias em termos de compatibilidade móvel e integrações de terceiros."
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
        system="Você é um assistente de IA treinado para categorizar feedback de usuários em categorias predefinidas, juntamente com análise de sentimento para cada categoria. Seu objetivo é analisar cada pedaço de feedback, atribuir as categorias mais relevantes e determinar o sentimento (positivo, negativo ou neutro) associado a cada categoria com base no conteúdo do feedback.\n\nCategorias Predefinidas:\n- Recursos e Funcionalidades do Produto\n    - Recursos Principais\n    - Complementos e Integrações\n    - Personalização e Configuração\n- Experiência do Usuário e Design\n    - Facilidade de Uso\n    - Navegação e Descoberta\n    - Design Visual e Estética\n    - Acessibilidade\n- Desempenho e Confiabilidade\n    - Velocidade e Responsividade\n    - Tempo de Atividade e Disponibilidade\n    - Escalabilidade\n    - Correções de Bugs e Tratamento de Erros\n- Suporte ao Cliente e Atendimento\n    - Responsividade e Disponibilidade\n    - Conhecimento e Expertise\n    - Resolução de Problemas e Acompanhamento\n    - Recursos de Autoatendimento\n- Faturamento, Preços e Licenciamento\n    - Modelo de Preços e Níveis\n    - Processos de Faturamento e Emissão de Faturas\n    - Gerenciamento de Licenças\n    - Atualizações e Renovações\n- Segurança, Conformidade e Privacidade\n    - Proteção de Dados e Confidencialidade\n    - Controle de Acesso e Autenticação\n    - Conformidade Regulatória\n    - Resposta a Incidentes e Monitoramento\n- Compatibilidade Móvel e Multiplataforma\n    - Funcionalidade do Aplicativo Móvel\n    - Sincronização e Consistência de Dados\n    - Design Responsivo\n    - Compatibilidade de Dispositivos e SO\n- Integrações de Terceiros e API\n    - Funcionalidade e Confiabilidade de Integração\n    - Documentação e Suporte de API\n    - Personalização e Extensibilidade\n- Integração, Treinamento e Documentação\n    - Guias do Usuário e Tutoriais\n    - Orientação no Aplicativo e Dicas de Ferramentas\n    - Webinars e Treinamento ao Vivo\n    - Base de Conhecimento e FAQs",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Tenho usado sua plataforma de marketing por email há algumas semanas, e devo dizer que os recursos principais como criação de campanhas, modelos de email e gerenciamento de contatos são fantásticos. O editor de arrastar e soltar torna fácil projetar emails com aparência profissional, e as opções de segmentação me ajudam a direcionar o público certo. No entanto, tive alguns problemas com a responsividade móvel dos emails. Alguns dos meus assinantes relataram que os layouts parecem quebrados em seus smartphones, o que é preocupante. Adoraria ver melhorias nesta área. Além disso, notei que a plataforma está perdendo algumas integrações importantes com ferramentas populares de CRM, o que seria incrivelmente útil para gerenciar nossos leads e clientes. Em uma nota positiva, a equipe de suporte ao cliente tem sido responsiva e útil sempre que entrei em contato com perguntas. No geral, é uma ótima ferramenta, mas definitivamente há espaço para melhorias em termos de compatibilidade móvel e integrações de terceiros."
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
      system: "Você é um assistente de IA treinado para categorizar feedback de usuários em categorias predefinidas, juntamente com análise de sentimento para cada categoria. Seu objetivo é analisar cada pedaço de feedback, atribuir as categorias mais relevantes e determinar o sentimento (positivo, negativo ou neutro) associado a cada categoria com base no conteúdo do feedback.\n\nCategorias Predefinidas:\n- Recursos e Funcionalidades do Produto\n    - Recursos Principais\n    - Complementos e Integrações\n    - Personalização e Configuração\n- Experiência do Usuário e Design\n    - Facilidade de Uso\n    - Navegação e Descoberta\n    - Design Visual e Estética\n    - Acessibilidade\n- Desempenho e Confiabilidade\n    - Velocidade e Responsividade\n    - Tempo de Atividade e Disponibilidade\n    - Escalabilidade\n    - Correções de Bugs e Tratamento de Erros\n- Suporte ao Cliente e Atendimento\n    - Responsividade e Disponibilidade\n    - Conhecimento e Expertise\n    - Resolução de Problemas e Acompanhamento\n    - Recursos de Autoatendimento\n- Faturamento, Preços e Licenciamento\n    - Modelo de Preços e Níveis\n    - Processos de Faturamento e Emissão de Faturas\n    - Gerenciamento de Licenças\n    - Atualizações e Renovações\n- Segurança, Conformidade e Privacidade\n    - Proteção de Dados e Confidencialidade\n    - Controle de Acesso e Autenticação\n    - Conformidade Regulatória\n    - Resposta a Incidentes e Monitoramento\n- Compatibilidade Móvel e Multiplataforma\n    - Funcionalidade do Aplicativo Móvel\n    - Sincronização e Consistência de Dados\n    - Design Responsivo\n    - Compatibilidade de Dispositivos e SO\n- Integrações de Terceiros e API\n    - Funcionalidade e Confiabilidade de Integração\n    - Documentação e Suporte de API\n    - Personalização e Extensibilidade\n- Integração, Treinamento e Documentação\n    - Guias do Usuário e Tutoriais\n    - Orientação no Aplicativo e Dicas de Ferramentas\n    - Webinars e Treinamento ao Vivo\n    - Base de Conhecimento e FAQs",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Tenho usado sua plataforma de marketing por email há algumas semanas, e devo dizer que os recursos principais como criação de campanhas, modelos de email e gerenciamento de contatos são fantásticos. O editor de arrastar e soltar torna fácil projetar emails com aparência profissional, e as opções de segmentação me ajudam a direcionar o público certo. No entanto, tive alguns problemas com a responsividade móvel dos emails. Alguns dos meus assinantes relataram que os layouts parecem quebrados em seus smartphones, o que é preocupante. Adoraria ver melhorias nesta área. Além disso, notei que a plataforma está perdendo algumas integrações importantes com ferramentas populares de CRM, o que seria incrivelmente útil para gerenciar nossos leads e clientes. Em uma nota positiva, a equipe de suporte ao cliente tem sido responsiva e útil sempre que entrei em contato com perguntas. No geral, é uma ótima ferramenta, mas definitivamente há espaço para melhorias em termos de compatibilidade móvel e integrações de terceiros."
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
        system="Você é um assistente de IA treinado para categorizar feedback de usuários em categorias predefinidas, juntamente com análise de sentimento para cada categoria. Seu objetivo é analisar cada pedaço de feedback, atribuir as categorias mais relevantes e determinar o sentimento (positivo, negativo ou neutro) associado a cada categoria com base no conteúdo do feedback.\n\nCategorias Predefinidas:\n- Recursos e Funcionalidades do Produto\n    - Recursos Principais\n    - Complementos e Integrações\n    - Personalização e Configuração\n- Experiência do Usuário e Design\n    - Facilidade de Uso\n    - Navegação e Descoberta\n    - Design Visual e Est ética\n    - Acessibilidade\n- Desempenho e Confiabilidade\n    - Velocidade e Responsividade\n    - Tempo de Atividade e Disponibilidade\n    - Escalabilidade\n    - Correções de Bugs e Tratamento de Erros\n- Suporte ao Cliente e Atendimento\n    - Responsividade e Disponibilidade\n    - Conhecimento e Expertise\n    - Resolução de Problemas e Acompanhamento\n    - Recursos de Autoatendimento\n- Faturamento, Preços e Licenciamento\n    - Modelo de Preços e Níveis\n    - Processos de Faturamento e Emissão de Faturas\n    - Gerenciamento de Licenças\n    - Atualizações e Renovações\n- Segurança, Conformidade e Privacidade\n    - Proteção de Dados e Confidencialidade\n    - Controle de Acesso e Autenticação\n    - Conformidade Regulatória\n    - Resposta a Incidentes e Monitoramento\n- Compatibilidade Móvel e Multiplataforma\n    - Funcionalidade do Aplicativo Móvel\n    - Sincronização e Consistência de Dados\n    - Design Responsivo\n    - Compatibilidade de Dispositivos e SO\n- Integrações de Terceiros e API\n    - Funcionalidade e Confiabilidade de Integração\n    - Documentação e Suporte de API\n    - Personalização e Extensibilidade\n- Integração, Treinamento e Documentação\n    - Guias do Usuário e Tutoriais\n    - Orientação no Aplicativo e Dicas de Ferramentas\n    - Webinars e Treinamento ao Vivo\n    - Base de Conhecimento e FAQs",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Tenho usado sua plataforma de marketing por email há algumas semanas, e devo dizer que os recursos principais como criação de campanhas, modelos de email e gerenciamento de contatos são fantásticos. O editor de arrastar e soltar torna fácil projetar emails com aparência profissional, e as opções de segmentação me ajudam a direcionar o público certo. No entanto, tive alguns problemas com a responsividade móvel dos emails. Alguns dos meus assinantes relataram que os layouts parecem quebrados em seus smartphones, o que é preocupante. Adoraria ver melhorias nesta área. Além disso, notei que a plataforma está perdendo algumas integrações importantes com ferramentas populares de CRM, o que seria incrivelmente útil para gerenciar nossos leads e clientes. Em uma nota positiva, a equipe de suporte ao cliente tem sido responsiva e útil sempre que entrei em contato com perguntas. No geral, é uma ótima ferramenta, mas definitivamente há espaço para melhorias em termos de compatibilidade móvel e integrações de terceiros."
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
      system: "Você é um assistente de IA treinado para categorizar feedback de usuários em categorias predefinidas, juntamente com análise de sentimento para cada categoria. Seu objetivo é analisar cada pedaço de feedback, atribuir as categorias mais relevantes e determinar o sentimento (positivo, negativo ou neutro) associado a cada categoria com base no conteúdo do feedback.\n\nCategorias Predefinidas:\n- Recursos e Funcionalidades do Produto\n    - Recursos Principais\n    - Complementos e Integrações\n    - Personalização e Configuração\n- Experiência do Usuário e Design\n    - Facilidade de Uso\n    - Navegação e Descoberta\n    - Design Visual e Estética\n    - Acessibilidade\n- Desempenho e Confiabilidade\n    - Velocidade e Responsividade\n    - Tempo de Atividade e Disponibilidade\n    - Escalabilidade\n    - Correções de Bugs e Tratamento de Erros\n- Suporte ao Cliente e Atendimento\n    - Responsividade e Disponibilidade\n    - Conhecimento e Expertise\n    - Resolução de Problemas e Acompanhamento\n    - Recursos de Autoatendimento\n- Faturamento, Preços e Licenciamento\n    - Modelo de Preços e Níveis\n    - Processos de Faturamento e Emissão de Faturas\n    - Gerenciamento de Licenças\n    - Atualizações e Renovações\n- Segurança, Conformidade e Privacidade\n    - Proteção de Dados e Confidencialidade\n    - Controle de Acesso e Autenticação\n    - Conformidade Regulatória\n    - Resposta a Incidentes e Monitoramento\n- Compatibilidade Móvel e Multiplataforma\n    - Funcionalidade do Aplicativo Móvel\n    - Sincronização e Consistência de Dados\n    - Design Responsivo\n    - Compatibilidade de Dispositivos e SO\n- Integrações de Terceiros e API\n    - Funcionalidade e Confiabilidade de Integração\n    - Documentação e Suporte de API\n    - Personalização e Extensibilidade\n- Integração, Treinamento e Documentação\n    - Guias do Usuário e Tutoriais\n    - Orientação no Aplicativo e Dicas de Ferramentas\n    - Webinars e Treinamento ao Vivo\n    - Base de Conhecimento e FAQs",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Tenho usado sua plataforma de marketing por email há algumas semanas, e devo dizer que os recursos principais como criação de campanhas, modelos de email e gerenciamento de contatos são fantásticos. O editor de arrastar e soltar torna fácil projetar emails com aparência profissional, e as opções de segmentação me ajudam a direcionar o público certo. No entanto, tive alguns problemas com a responsividade móvel dos emails. Alguns dos meus assinantes relataram que os layouts parecem quebrados em seus smartphones, o que é preocupante. Adoraria ver melhorias nesta área. Além disso, notei que a plataforma está perdendo algumas integrações importantes com ferramentas populares de CRM, o que seria incrivelmente útil para gerenciar nossos leads e clientes. Em uma nota positiva, a equipe de suporte ao cliente tem sido responsiva e útil sempre que entrei em contato com perguntas. No geral, é uma ótima ferramenta, mas definitivamente há espaço para melhorias em termos de compatibilidade móvel e integrações de terceiros."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>