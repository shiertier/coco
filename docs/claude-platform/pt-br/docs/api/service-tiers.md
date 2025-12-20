# Camadas de serviço

Diferentes camadas de serviço permitem que você equilibre disponibilidade, desempenho e custos previsíveis com base nas necessidades da sua aplicação.

---

Oferecemos três camadas de serviço:
- **Camada de Prioridade:** Melhor para fluxos de trabalho implantados em produção onde tempo, disponibilidade e preços previsíveis são importantes
- **Padrão:** Camada padrão para pilotagem e dimensionamento de casos de uso cotidianos
- **Lote:** Melhor para fluxos de trabalho assíncronos que podem esperar ou se beneficiar de estar fora da sua capacidade normal

## Camada Padrão

A camada padrão é a camada de serviço padrão para todas as solicitações de API. As solicitações nesta camada são priorizadas junto com todas as outras solicitações e observam disponibilidade de melhor esforço.

## Camada de Prioridade

As solicitações nesta camada são priorizadas sobre todas as outras solicitações para Anthropic. Esta priorização ajuda a minimizar erros de ["servidor sobrecarregado"](/docs/pt-BR/api/errors#http-errors), mesmo durante horários de pico.

Para mais informações, consulte [Comece com a Camada de Prioridade](#get-started-with-priority-tier)

## Como as solicitações recebem atribuição de camadas

Ao lidar com uma solicitação, Anthropic decide atribuir uma solicitação à Camada de Prioridade nos seguintes cenários:
- Sua organização tem capacidade de camada de prioridade suficiente **input** tokens por minuto
- Sua organização tem capacidade de camada de prioridade suficiente **output** tokens por minuto

Anthropic conta o uso em relação à capacidade da Camada de Prioridade da seguinte forma:

**Tokens de Entrada**
- Leituras de cache como 0,1 tokens por token lido do cache
- Escritas de cache como 1,25 tokens por token escrito no cache com TTL de 5 minutos
- Escritas de cache como 2,00 tokens por token escrito no cache com TTL de 1 hora
- Para solicitações de [contexto longo](/docs/pt-BR/build-with-claude/context-windows) (>200k tokens de entrada), tokens de entrada são 2 tokens por token
- Todos os outros tokens de entrada são 1 token por token

**Tokens de Saída**
- Para solicitações de [contexto longo](/docs/pt-BR/build-with-claude/context-windows) (>200k tokens de entrada), tokens de saída são 1,5 tokens por token
- Todos os outros tokens de saída são 1 token por token

Caso contrário, as solicitações prosseguem na camada padrão.

<Note>
As solicitações atribuídas à Camada de Prioridade extraem tanto da capacidade da Camada de Prioridade quanto dos limites de taxa regulares.
Se atender à solicitação excederia os limites de taxa, a solicitação é recusada.
</Note>

## Usando camadas de serviço

Você pode controlar quais camadas de serviço podem ser usadas para uma solicitação definindo o parâmetro `service_tier`:

```python
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello, Claude!"}],
    service_tier="auto"  # Automatically use Priority Tier when available, fallback to standard
)
```

O parâmetro `service_tier` aceita os seguintes valores:

- `"auto"` (padrão) - Usa a capacidade da Camada de Prioridade se disponível, voltando para sua outra capacidade se não
- `"standard_only"` - Use apenas a capacidade da camada padrão, útil se você não quiser usar sua capacidade da Camada de Prioridade

O objeto `usage` da resposta também inclui a camada de serviço atribuída à solicitação:

```json
{
  "usage": {
    "input_tokens": 410,
    "cache_creation_input_tokens": 0,
    "cache_read_input_tokens": 0,
    "output_tokens": 585,
    "service_tier": "priority"
  }
}
```
Isso permite que você determine qual camada de serviço foi atribuída à solicitação.

Ao solicitar `service_tier="auto"` com um modelo com um compromisso de Camada de Prioridade, estes cabeçalhos de resposta fornecem insights:
```
anthropic-priority-input-tokens-limit: 10000
anthropic-priority-input-tokens-remaining: 9618
anthropic-priority-input-tokens-reset: 2025-01-12T23:11:59Z
anthropic-priority-output-tokens-limit: 10000
anthropic-priority-output-tokens-remaining: 6000
anthropic-priority-output-tokens-reset: 2025-01-12T23:12:21Z
```
Você pode usar a presença desses cabeçalhos para detectar se sua solicitação era elegível para a Camada de Prioridade, mesmo que estivesse acima do limite.

## Comece com a Camada de Prioridade

Você pode querer se comprometer com a capacidade da Camada de Prioridade se estiver interessado em:
- **Maior disponibilidade**: Alvo de 99,5% de tempo de atividade com recursos computacionais priorizados
- **Controle de Custos**: Gastos previsíveis e descontos para compromissos mais longos
- **Overflow Flexível**: Volta automaticamente para a camada padrão quando você excede sua capacidade comprometida

Comprometer-se com a Camada de Prioridade envolverá decidir:
- Um número de tokens de entrada por minuto
- Um número de tokens de saída por minuto
- Uma duração de compromisso (1, 3, 6 ou 12 meses)
- Uma versão de modelo específica

<Note>
A proporção de tokens de entrada para saída que você compra é importante. Dimensionar sua capacidade da Camada de Prioridade para se alinhar com seus padrões de tráfego reais ajuda você a maximizar a utilização de seus tokens comprados.
</Note>

### Modelos suportados

A Camada de Prioridade é suportada por:

- Claude Opus 4.5
- Claude Sonnet 4.5
- Claude Haiku 4.5
- Claude Opus 4.1
- Claude Opus 4
- Claude Sonnet 4
- Claude Sonnet 3.7 ([descontinuado](/docs/pt-BR/about-claude/model-deprecations))
- Claude Haiku 3.5 ([descontinuado](/docs/pt-BR/about-claude/model-deprecations))

Verifique a [página de visão geral do modelo](/docs/pt-BR/about-claude/models/overview) para mais detalhes sobre nossos modelos.

### Como acessar a Camada de Prioridade

Para começar a usar a Camada de Prioridade:

1. [Entre em contato com vendas](https://claude.com/contact-sales/priority-tier) para concluir o provisionamento
2. (Opcional) Atualize suas solicitações de API para definir opcionalmente o parâmetro `service_tier` como `auto`
3. Monitore seu uso através de cabeçalhos de resposta e do Console Claude