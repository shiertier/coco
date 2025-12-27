# Compatibilidade com SDK OpenAI

Anthropic fornece uma camada de compatibilidade que permite usar o SDK OpenAI para testar a API Claude. Com algumas alterações de código, você pode avaliar rapidamente os recursos dos modelos Anthropic.

---

<Note>
Esta camada de compatibilidade é principalmente destinada a testar e comparar recursos de modelos, e não é considerada uma solução de longo prazo ou pronta para produção para a maioria dos casos de uso. Embora tenhamos a intenção de mantê-la totalmente funcional e não fazer alterações significativas, nossa prioridade é a confiabilidade e eficácia da [API Claude](/docs/pt-BR/api/overview).

Para mais informações sobre limitações conhecidas de compatibilidade, consulte [Limitações importantes de compatibilidade com OpenAI](#important-openai-compatibility-limitations).

Se você encontrar algum problema com o recurso de compatibilidade do SDK OpenAI, por favor nos informe [aqui](https://forms.gle/oQV4McQNiuuNbz9n8).
</Note>

<Tip>
Para a melhor experiência e acesso ao conjunto completo de recursos da API Claude ([processamento de PDF](/docs/pt-BR/build-with-claude/pdf-support), [citações](/docs/pt-BR/build-with-claude/citations), [pensamento estendido](/docs/pt-BR/build-with-claude/extended-thinking) e [cache de prompt](/docs/pt-BR/build-with-claude/prompt-caching)), recomendamos usar a [API Claude](/docs/pt-BR/api/overview) nativa.
</Tip>

## Começando com o SDK OpenAI

Para usar o recurso de compatibilidade do SDK OpenAI, você precisará:

1. Usar um SDK OpenAI oficial
2. Alterar o seguinte
   * Atualize sua URL base para apontar para a API Claude
   * Substitua sua chave de API por uma [chave de API Claude](/settings/keys)
   * Atualize o nome do seu modelo para usar um [modelo Claude](/docs/pt-BR/about-claude/models/overview)
3. Revise a documentação abaixo para saber quais recursos são suportados

### Exemplo de início rápido

<CodeGroup>
    ```python Python
    from openai import OpenAI

    client = OpenAI(
        api_key="ANTHROPIC_API_KEY",  # Your Claude API key
        base_url="https://api.anthropic.com/v1/"  # the Claude API endpoint
    )

    response = client.chat.completions.create(
        model="claude-sonnet-4-5", # Anthropic model name
        messages=[
            {"role": "system", "content": "You are a helpful assistant."},
            {"role": "user", "content": "Who are you?"}
        ],
    )

    print(response.choices[0].message.content)
    ```
    
    ```typescript TypeScript
    import OpenAI from 'openai';

    const openai = new OpenAI({
        apiKey: "ANTHROPIC_API_KEY",   // Your Claude API key
        baseURL: "https://api.anthropic.com/v1/",  // Claude API endpoint
    });

    const response = await openai.chat.completions.create({
        messages: [
            { role: "user", content: "Who are you?" }
        ],
        model: "claude-sonnet-4-5", // Claude model name
    });

    console.log(response.choices[0].message.content);
    ```
</CodeGroup>

## Limitações importantes de compatibilidade com OpenAI

#### Comportamento da API

Aqui estão as diferenças mais substanciais em relação ao uso do OpenAI:

* O parâmetro `strict` para chamadas de função é ignorado, o que significa que o JSON de uso de ferramentas não é garantido que siga o esquema fornecido. Para conformidade de esquema garantida, use a [API Claude nativa com Saídas Estruturadas](/docs/pt-BR/build-with-claude/structured-outputs).
* Entrada de áudio não é suportada; ela será simplesmente ignorada e removida da entrada
* Cache de prompt não é suportado, mas é suportado no [SDK Anthropic](/docs/pt-BR/api/client-sdks)
* Mensagens de sistema/desenvolvedor são elevadas e concatenadas ao início da conversa, pois Anthropic suporta apenas uma única mensagem de sistema inicial.

A maioria dos campos não suportados é silenciosamente ignorada em vez de produzir erros. Todos estes estão documentados abaixo.

#### Considerações de qualidade de saída

Se você fez muitos ajustes no seu prompt, é provável que esteja bem ajustado especificamente para OpenAI. Considere usar nosso [melhorador de prompt no Console Claude](/dashboard) como um bom ponto de partida.

#### Elevação de mensagem de sistema / desenvolvedor

A maioria das entradas para o SDK OpenAI mapeia claramente diretamente para os parâmetros da API Anthropic, mas uma diferença distinta é o tratamento de prompts de sistema / desenvolvedor. Esses dois prompts podem ser colocados em toda uma conversa de chat via OpenAI. Como Anthropic suporta apenas uma mensagem de sistema inicial, pegamos todas as mensagens de sistema/desenvolvedor e as concatenamos com uma única quebra de linha (`\n`) entre elas. Esta string completa é então fornecida como uma única mensagem de sistema no início das mensagens.

#### Suporte a pensamento estendido

Você pode habilitar recursos de [pensamento estendido](/docs/pt-BR/build-with-claude/extended-thinking) adicionando o parâmetro `thinking`. Embora isso melhore o raciocínio do Claude para tarefas complexas, o SDK OpenAI não retornará o processo de pensamento detalhado do Claude. Para recursos completos de pensamento estendido, incluindo acesso à saída de raciocínio passo a passo do Claude, use a API Claude nativa.

<CodeGroup>
    ```python Python
    response = client.chat.completions.create(
        model="claude-sonnet-4-5",
        messages=...,
        extra_body={
            "thinking": { "type": "enabled", "budget_tokens": 2000 }
        }
    )
    ```
    
    ```typescript TypeScript
    const response = await openai.chat.completions.create({
        messages: [
            { role: "user", content: "Who are you?" }
        ],
        model: "claude-sonnet-4-5",
        // @ts-expect-error
        thinking: { type: "enabled", budget_tokens: 2000 }
    });

    ```
</CodeGroup>

## Limites de taxa

Os limites de taxa seguem os [limites padrão](/docs/pt-BR/api/rate-limits) da Anthropic para o endpoint `/v1/messages`.

## Suporte detalhado de API compatível com OpenAI
### Campos de solicitação
#### Campos simples
| Campo | Status de suporte |
|--------|----------------|
| `model` | Use nomes de modelos Claude |
| `max_tokens` | Totalmente suportado |
| `max_completion_tokens` | Totalmente suportado |
| `stream` | Totalmente suportado |
| `stream_options` | Totalmente suportado |
| `top_p` | Totalmente suportado |
| `parallel_tool_calls` | Totalmente suportado |
| `stop` | Todas as sequências de parada não-espaço em branco funcionam |
| `temperature` | Entre 0 e 1 (inclusive). Valores maiores que 1 são limitados a 1. |
| `n` | Deve ser exatamente 1 |
| `logprobs` | Ignorado |
| `metadata` | Ignorado |
| `response_format` | Ignorado. Para saída JSON, use [Saídas Estruturadas](/docs/pt-BR/build-with-claude/structured-outputs) com a API Claude nativa |
| `prediction` | Ignorado |
| `presence_penalty` | Ignorado |
| `frequency_penalty` | Ignorado |
| `seed` | Ignorado |
| `service_tier` | Ignorado |
| `audio` | Ignorado |
| `logit_bias` | Ignorado |
| `store` | Ignorado |
| `user` | Ignorado |
| `modalities` | Ignorado |
| `top_logprobs` | Ignorado |
| `reasoning_effort` | Ignorado |

#### Campos `tools` / `functions`
<section title="Mostrar campos">

<Tabs>
<Tab title="Tools">
Campos `tools[n].function`
| Campo        | Status de suporte         |
|--------------|-----------------|
| `name`       | Totalmente suportado |
| `description`| Totalmente suportado |
| `parameters` | Totalmente suportado |
| `strict`     | Ignorado. Use [Saídas Estruturadas](/docs/pt-BR/build-with-claude/structured-outputs) com API Claude nativa para validação de esquema estrita |
</Tab>
<Tab title="Functions">

Campos `functions[n]`
<Info>
OpenAI descontinuou o campo `functions` e sugere usar `tools` em seu lugar.
</Info>
| Campo        | Status de suporte         |
|--------------|-----------------|
| `name`       | Totalmente suportado |
| `description`| Totalmente suportado |
| `parameters` | Totalmente suportado |
| `strict`     | Ignorado. Use [Saídas Estruturadas](/docs/pt-BR/build-with-claude/structured-outputs) com API Claude nativa para validação de esquema estrita |
</Tab>
</Tabs>

</section>

#### Campos do array `messages`
<section title="Mostrar campos">

<Tabs>
<Tab title="Função de desenvolvedor">
Campos para `messages[n].role == "developer"`
<Info>
Mensagens de desenvolvedor são elevadas ao início da conversa como parte da mensagem de sistema inicial
</Info>
| Campo | Status de suporte |
|-------|---------|
| `content` | Totalmente suportado, mas elevado |
| `name` | Ignorado |

</Tab>
<Tab title="Função de sistema">
Campos para `messages[n].role == "system"`

<Info>
Mensagens de sistema são elevadas ao início da conversa como parte da mensagem de sistema inicial
</Info>
| Campo | Status de suporte |
|-------|---------|
| `content` | Totalmente suportado, mas elevado |
| `name` | Ignorado |

</Tab>
<Tab title="Função de usuário">
Campos para `messages[n].role == "user"`

| Campo | Variante | Sub-campo | Status de suporte |
|-------|---------|-----------|----------------|
| `content` | `string` | | Totalmente suportado |
| | `array`, `type == "text"` | | Totalmente suportado |
| | `array`, `type == "image_url"` | `url` | Totalmente suportado |
| | | `detail` | Ignorado |
| | `array`, `type == "input_audio"` | | Ignorado |
| | `array`, `type == "file"` | | Ignorado |
| `name` | | | Ignorado |

</Tab>

<Tab title="Função de assistente">
Campos para `messages[n].role == "assistant"`
| Campo | Variante | Status de suporte |
|-------|---------|----------------|
| `content` | `string` | Totalmente suportado |
| | `array`, `type == "text"` | Totalmente suportado |
| | `array`, `type == "refusal"` | Ignorado |
| `tool_calls` | | Totalmente suportado |
| `function_call` | | Totalmente suportado |
| `audio` | | Ignorado |
| `refusal` | | Ignorado |

</Tab>

<Tab title="Função de ferramenta">
Campos para `messages[n].role == "tool"`
| Campo | Variante | Status de suporte |
|-------|---------|----------------|
| `content` | `string` | Totalmente suportado |
| | `array`, `type == "text"` | Totalmente suportado |
| `tool_call_id` | | Totalmente suportado |
| `tool_choice` | | Totalmente suportado |
| `name` | | Ignorado |
</Tab>

<Tab title="Função de função">
Campos para `messages[n].role == "function"`
| Campo | Variante | Status de suporte |
|-------|---------|----------------|
| `content` | `string` | Totalmente suportado |
| | `array`, `type == "text"` | Totalmente suportado |
| `tool_choice` | | Totalmente suportado |
| `name` | | Ignorado |
</Tab>
</Tabs>

</section>

### Campos de resposta

| Campo | Status de suporte |
|---------------------------|----------------|
| `id` | Totalmente suportado |
| `choices[]` | Sempre terá um comprimento de 1 |
| `choices[].finish_reason` | Totalmente suportado |
| `choices[].index` | Totalmente suportado |
| `choices[].message.role` | Totalmente suportado |
| `choices[].message.content` | Totalmente suportado |
| `choices[].message.tool_calls` | Totalmente suportado |
| `object` | Totalmente suportado |
| `created` | Totalmente suportado |
| `model` | Totalmente suportado |
| `finish_reason` | Totalmente suportado |
| `content` | Totalmente suportado |
| `usage.completion_tokens` | Totalmente suportado |
| `usage.prompt_tokens` | Totalmente suportado |
| `usage.total_tokens` | Totalmente suportado |
| `usage.completion_tokens_details` | Sempre vazio |
| `usage.prompt_tokens_details` | Sempre vazio |
| `choices[].message.refusal` | Sempre vazio |
| `choices[].message.audio` | Sempre vazio |
| `logprobs` | Sempre vazio |
| `service_tier` | Sempre vazio |
| `system_fingerprint` | Sempre vazio |

### Compatibilidade de mensagem de erro

A camada de compatibilidade mantém formatos de erro consistentes com a API OpenAI. No entanto, as mensagens de erro detalhadas não serão equivalentes. Recomendamos usar apenas as mensagens de erro para registro e depuração.

### Compatibilidade de cabeçalho

Embora o SDK OpenAI gerencie automaticamente os cabeçalhos, aqui está a lista completa de cabeçalhos suportados pela API Claude para desenvolvedores que precisam trabalhar com eles diretamente.

| Cabeçalho | Status de suporte |
|---------|----------------|
| `x-ratelimit-limit-requests` | Totalmente suportado |
| `x-ratelimit-limit-tokens` | Totalmente suportado |
| `x-ratelimit-remaining-requests` | Totalmente suportado |
| `x-ratelimit-remaining-tokens` | Totalmente suportado |
| `x-ratelimit-reset-requests` | Totalmente suportado |
| `x-ratelimit-reset-tokens` | Totalmente suportado |
| `retry-after` | Totalmente suportado |
| `request-id` | Totalmente suportado |
| `openai-version` | Sempre `2020-10-01` |
| `authorization` | Totalmente suportado |
| `openai-processing-ms` | Sempre vazio |