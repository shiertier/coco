# Limites de taxa

Para mitigar abuso e gerenciar a capacidade em nossa API, implementamos limites sobre quanto uma organização pode usar a API Claude.

---

Temos dois tipos de limites:

1. **Limites de gastos** definem um custo mensal máximo que uma organização pode incorrer pelo uso da API.
2. **Limites de taxa** definem o número máximo de solicitações de API que uma organização pode fazer durante um período definido.

Aplicamos limites configurados pelo serviço no nível da organização, mas você também pode definir limites configuráveis pelo usuário para os workspaces da sua organização.

Esses limites se aplicam ao uso de Tier Padrão e Tier Prioritário. Para mais informações sobre o Tier Prioritário, que oferece níveis de serviço aprimorados em troca de gastos comprometidos, consulte [Service Tiers](/docs/pt-BR/api/service-tiers).

## Sobre nossos limites

* Os limites são projetados para evitar abuso da API, minimizando o impacto nos padrões de uso comum dos clientes.
* Os limites são definidos por **tier de uso**, onde cada tier está associado a um conjunto diferente de limites de gastos e taxa.
* Sua organização aumentará de tier automaticamente conforme você atinge certos limites enquanto usa a API.
  Os limites são definidos no nível da organização. Você pode ver os limites da sua organização na [página Limites](/settings/limits) no [Claude Console](/).
* Você pode atingir limites de taxa em intervalos de tempo mais curtos. Por exemplo, uma taxa de 60 solicitações por minuto (RPM) pode ser aplicada como 1 solicitação por segundo. Rajadas curtas de solicitações em alto volume podem exceder o limite de taxa e resultar em erros de limite de taxa.
* Os limites descritos abaixo são nossos limites de tier padrão. Se você está procurando limites mais altos e personalizados ou Tier Prioritário para níveis de serviço aprimorados, entre em contato com vendas através do [Claude Console](/settings/limits).
* Usamos o [algoritmo token bucket](https://en.wikipedia.org/wiki/Token_bucket) para fazer limitação de taxa. Isso significa que sua capacidade é continuamente reabastecida até seu limite máximo, em vez de ser redefinida em intervalos fixos.
* Todos os limites descritos aqui representam uso máximo permitido, não mínimos garantidos. Esses limites destinam-se a reduzir gastos excessivos não intencionais e garantir distribuição justa de recursos entre usuários.

## Limites de gastos

Cada tier de uso tem um limite sobre quanto você pode gastar na API a cada mês do calendário. Depois de atingir o limite de gastos do seu tier, até que você se qualifique para o próximo tier, você terá que esperar até o próximo mês para poder usar a API novamente.

Para se qualificar para o próximo tier, você deve atender a um requisito de depósito. Para minimizar o risco de financiar excessivamente sua conta, você não pode depositar mais do que seu limite de gastos mensal.

### Requisitos para avançar de tier
<table>
  <thead>
    <tr>
      <th>Tier de Uso</th>
      <th>Compra de Crédito</th>
      <th>Compra Máxima de Crédito</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Tier 1</td>
      <td>\$5</td>
      <td>\$100</td>
    </tr>
    <tr>
      <td>Tier 2</td>
      <td>\$40</td>
      <td>\$500</td>
    </tr>
    <tr>
      <td>Tier 3</td>
      <td>\$200</td>
      <td>\$1.000</td>
    </tr>
    <tr>
      <td>Tier 4</td>
      <td>\$400</td>
      <td>\$5.000</td>
    </tr>
    <tr>
      <td>Faturamento Mensal</td>
      <td>N/A</td>
      <td>N/A</td>
    </tr>
  </tbody>
</table>

<Note>
**Compra de Crédito** mostra as compras de crédito cumulativas (excluindo impostos) necessárias para avançar para esse tier. Você avança imediatamente ao atingir o limite.

**Compra Máxima de Crédito** limita o valor máximo que você pode adicionar à sua conta em uma única transação para evitar financiamento excessivo da conta.
</Note>

## Limites de taxa

Nossos limites de taxa para a API Messages são medidos em solicitações por minuto (RPM), tokens de entrada por minuto (ITPM) e tokens de saída por minuto (OTPM) para cada classe de modelo.
Se você exceder qualquer um dos limites de taxa, receberá um [erro 429](/docs/pt-BR/api/errors) descrevendo qual limite de taxa foi excedido, junto com um cabeçalho `retry-after` indicando quanto tempo esperar.

<Note>
Você também pode encontrar erros 429 devido a limites de aceleração na API se sua organização tiver um aumento acentuado no uso. Para evitar atingir limites de aceleração, aumente seu tráfego gradualmente e mantenha padrões de uso consistentes.
</Note>

### ITPM com reconhecimento de cache

Muitos provedores de API usam um limite combinado de "tokens por minuto" (TPM) que pode incluir todos os tokens, tanto em cache quanto não em cache, entrada e saída. **Para a maioria dos modelos Claude, apenas tokens de entrada não em cache contam para seus limites de taxa ITPM.** Esta é uma vantagem chave que torna nossos limites de taxa efetivamente mais altos do que podem parecer inicialmente.

Os limites de taxa ITPM são estimados no início de cada solicitação, e a estimativa é ajustada durante a solicitação para refletir o número real de tokens de entrada usados.

Aqui está o que conta para ITPM:
- `input_tokens` (tokens após o último ponto de quebra de cache) ✓ **Contam para ITPM**
- `cache_creation_input_tokens` (tokens sendo escritos no cache) ✓ **Contam para ITPM**
- `cache_read_input_tokens` (tokens lidos do cache) ✗ **NÃO contam para ITPM** para a maioria dos modelos

<Note>
O campo `input_tokens` representa apenas tokens que aparecem **após seu último ponto de quebra de cache**, não todos os tokens de entrada em sua solicitação. Para calcular o total de tokens de entrada:

```
total_input_tokens = cache_read_input_tokens + cache_creation_input_tokens + input_tokens
```

Isso significa que quando você tem conteúdo em cache, `input_tokens` será tipicamente muito menor do que sua entrada total. Por exemplo, com um documento em cache de 200K tokens e uma pergunta do usuário de 50 tokens, você veria `input_tokens: 50` mesmo que a entrada total seja 200.050 tokens.

Para fins de limite de taxa na maioria dos modelos, apenas `input_tokens` + `cache_creation_input_tokens` contam para seu limite ITPM, tornando [prompt caching](/docs/pt-BR/build-with-claude/prompt-caching) uma forma eficaz de aumentar sua taxa de transferência efetiva.
</Note>

**Exemplo**: Com um limite ITPM de 2.000.000 e uma taxa de acerto de cache de 80%, você poderia processar efetivamente 10.000.000 tokens de entrada total por minuto (2M não em cache + 8M em cache), já que tokens em cache não contam para seu limite de taxa.

<Note>
Alguns modelos mais antigos (marcados com † nas tabelas de limite de taxa abaixo) também contam `cache_read_input_tokens` para limites de taxa ITPM.

Para todos os modelos sem o marcador †, tokens de entrada em cache não contam para limites de taxa e são cobrados a uma taxa reduzida (10% do preço do token de entrada base). Isso significa que você pode alcançar taxa de transferência efetiva significativamente mais alta usando [prompt caching](/docs/pt-BR/build-with-claude/prompt-caching).
</Note>

<Tip>
**Maximize seus limites de taxa com prompt caching**

Para aproveitar ao máximo seus limites de taxa, use [prompt caching](/docs/pt-BR/build-with-claude/prompt-caching) para conteúdo repetido como:
- Instruções de sistema e prompts
- Documentos de contexto grande
- Definições de ferramentas
- Histórico de conversa

Com caching eficaz, você pode aumentar dramaticamente sua taxa de transferência real sem aumentar seus limites de taxa. Monitore sua taxa de acerto de cache na [página Uso](/settings/usage) para otimizar sua estratégia de caching.
</Tip>

Os limites de taxa OTPM são estimados com base em `max_tokens` no início de cada solicitação, e a estimativa é ajustada no final da solicitação para refletir o número real de tokens de saída usados.
Se você está atingindo limites OTPM mais cedo do que esperado, tente reduzir `max_tokens` para aproximar melhor o tamanho de suas conclusões.

Os limites de taxa são aplicados separadamente para cada modelo; portanto, você pode usar diferentes modelos até seus respectivos limites simultaneamente.
Você pode verificar seus limites de taxa atuais e comportamento no [Claude Console](/settings/limits).

<Note>
Para solicitações de contexto longo (>200K tokens) ao usar o cabeçalho beta `context-1m-2025-08-07` com Claude Sonnet 4.x, limites de taxa separados se aplicam. Consulte [Limites de taxa de contexto longo](#long-context-rate-limits) abaixo.
</Note>

<Tabs>
<Tab title="Tier 1">
| Modelo                                                                                       | Máximo de solicitações por minuto (RPM) | Máximo de tokens de entrada por minuto (ITPM) | Máximo de tokens de saída por minuto (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------------- | --------------------------------------------- | ------------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 50                                      | 30.000                                        | 8.000                                       |
| Claude Sonnet 3.7 ([deprecated](/docs/pt-BR/about-claude/model-deprecations))                | 50                                      | 20.000                                        | 8.000                                       |
| Claude Haiku 4.5                                                                             | 50                                      | 50.000                                        | 10.000                                      |
| Claude Haiku 3.5 ([deprecated](/docs/pt-BR/about-claude/model-deprecations))                 | 50                                      | 50.000<sup>†</sup>                            | 10.000                                      |
| Claude Haiku 3                                                                               | 50                                      | 50.000<sup>†</sup>                            | 10.000                                      |
| Claude Opus 4.x<sup>*</sup>                                                                  | 50                                      | 30.000                                        | 8.000                                       |
| Claude Opus 3 ([deprecated](/docs/pt-BR/about-claude/model-deprecations))                   | 50                                      | 20.000<sup>†</sup>                            | 4.000                                       |

</Tab>
<Tab title="Tier 2">
| Modelo                                                                                       | Máximo de solicitações por minuto (RPM) | Máximo de tokens de entrada por minuto (ITPM) | Máximo de tokens de saída por minuto (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------------- | --------------------------------------------- | ------------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 1.000                                   | 450.000                                       | 90.000                                      |
| Claude Sonnet 3.7 ([deprecated](/docs/pt-BR/about-claude/model-deprecations))                | 1.000                                   | 40.000                                        | 16.000                                      |
| Claude Haiku 4.5                                                                             | 1.000                                   | 450.000                                       | 90.000                                      |
| Claude Haiku 3.5 ([deprecated](/docs/pt-BR/about-claude/model-deprecations))                 | 1.000                                   | 100.000<sup>†</sup>                           | 20.000                                      |
| Claude Haiku 3                                                                               | 1.000                                   | 100.000<sup>†</sup>                           | 20.000                                      |
| Claude Opus 4.x<sup>*</sup>                                                                  | 1.000                                   | 450.000                                       | 90.000                                      |
| Claude Opus 3 ([deprecated](/docs/pt-BR/about-claude/model-deprecations))                   | 1.000                                   | 40.000<sup>†</sup>                            | 8.000                                       |

</Tab>
<Tab title="Tier 3">
| Modelo                                                                                       | Máximo de solicitações por minuto (RPM) | Máximo de tokens de entrada por minuto (ITPM) | Máximo de tokens de saída por minuto (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------------- | --------------------------------------------- | ------------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 2.000                                   | 800.000                                       | 160.000                                     |
| Claude Sonnet 3.7 ([deprecated](/docs/pt-BR/about-claude/model-deprecations))                | 2.000                                   | 80.000                                        | 32.000                                      |
| Claude Haiku 4.5                                                                             | 2.000                                   | 1.000.000                                     | 200.000                                     |
| Claude Haiku 3.5 ([deprecated](/docs/pt-BR/about-claude/model-deprecations))                 | 2.000                                   | 200.000<sup>†</sup>                           | 40.000                                      |
| Claude Haiku 3                                                                               | 2.000                                   | 200.000<sup>†</sup>                           | 40.000                                      |
| Claude Opus 4.x<sup>*</sup>                                                                  | 2.000                                   | 800.000                                       | 160.000                                     |
| Claude Opus 3 ([deprecated](/docs/pt-BR/about-claude/model-deprecations))                   | 2.000                                   | 80.000<sup>†</sup>                            | 16.000                                      |

</Tab>
<Tab title="Tier 4">
| Modelo                                                                                       | Máximo de solicitações por minuto (RPM) | Máximo de tokens de entrada por minuto (ITPM) | Máximo de tokens de saída por minuto (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------------- | --------------------------------------------- | ------------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 4.000                                   | 2.000.000                                     | 400.000                                     |
| Claude Sonnet 3.7 ([deprecated](/docs/pt-BR/about-claude/model-deprecations))                | 4.000                                   | 200.000                                       | 80.000                                      |
| Claude Haiku 4.5                                                                             | 4.000                                   | 4.000.000                                     | 800.000                                     |
| Claude Haiku 3.5 ([deprecated](/docs/pt-BR/about-claude/model-deprecations))                 | 4.000                                   | 400.000<sup>†</sup>                           | 80.000                                      |
| Claude Haiku 3                                                                               | 4.000                                   | 400.000<sup>†</sup>                           | 80.000                                      |
| Claude Opus 4.x<sup>*</sup>                                                                  | 4.000                                   | 2.000.000                                     | 400.000                                     |
| Claude Opus 3 ([deprecated](/docs/pt-BR/about-claude/model-deprecations))                   | 4.000                                   | 400.000<sup>†</sup>                           | 80.000                                      |

</Tab>
<Tab title="Personalizado">
Se você está procurando limites mais altos para um caso de uso Enterprise, entre em contato com vendas através do [Claude Console](/settings/limits).
</Tab>
</Tabs>

_<sup>* - O limite de taxa Opus 4.x é um limite total que se aplica ao tráfego combinado entre Opus 4, Opus 4.1 e Opus 4.5.</sup>_

_<sup>** - O limite de taxa Sonnet 4.x é um limite total que se aplica ao tráfego combinado entre Sonnet 4 e Sonnet 4.5.</sup>_

_<sup>† - O limite conta `cache_read_input_tokens` para uso ITPM.</sup>_

### API de Lotes de Mensagens

A API de Lotes de Mensagens tem seu próprio conjunto de limites de taxa que são compartilhados entre todos os modelos. Estes incluem um limite de solicitações por minuto (RPM) para todos os endpoints da API e um limite no número de solicitações de lote que podem estar na fila de processamento ao mesmo tempo. Uma "solicitação de lote" aqui se refere a parte de um Lote de Mensagens. Você pode criar um Lote de Mensagens contendo milhares de solicitações de lote, cada uma das quais conta para esse limite. Uma solicitação de lote é considerada parte da fila de processamento quando ainda não foi processada com sucesso pelo modelo.

<Tabs>
<Tab title="Tier 1">
| Máximo de solicitações por minuto (RPM) | Máximo de solicitações de lote em fila de processamento | Máximo de solicitações de lote por lote |
| --------------------------------------- | ------------------------------------------------------ | --------------------------------------- |
| 50                                      | 100.000                                                | 100.000                                 |
</Tab>
<Tab title="Tier 2">
| Máximo de solicitações por minuto (RPM) | Máximo de solicitações de lote em fila de processamento | Máximo de solicitações de lote por lote |
| --------------------------------------- | ------------------------------------------------------ | --------------------------------------- |
| 1.000                                   | 200.000                                                | 100.000                                 |
</Tab>
<Tab title="Tier 3">
| Máximo de solicitações por minuto (RPM) | Máximo de solicitações de lote em fila de processamento | Máximo de solicitações de lote por lote |
| --------------------------------------- | ------------------------------------------------------ | --------------------------------------- |
| 2.000                                   | 300.000                                                | 100.000                                 |
</Tab>
<Tab title="Tier 4">
| Máximo de solicitações por minuto (RPM) | Máximo de solicitações de lote em fila de processamento | Máximo de solicitações de lote por lote |
| --------------------------------------- | ------------------------------------------------------ | --------------------------------------- |
| 4.000                                   | 500.000                                                | 100.000                                 |
</Tab>
<Tab title="Personalizado">
Se você está procurando limites mais altos para um caso de uso Enterprise, entre em contato com vendas através do [Claude Console](/settings/limits).
</Tab>
</Tabs>

### Limites de taxa de contexto longo

Ao usar Claude Sonnet 4 e Sonnet 4.5 com a [janela de contexto de 1M tokens habilitada](/docs/pt-BR/build-with-claude/context-windows#1m-token-context-window), os seguintes limites de taxa dedicados se aplicam a solicitações excedendo 200K tokens.

<Note>
A janela de contexto de 1M tokens está atualmente em beta para organizações no tier de uso 4 e organizações com limites de taxa personalizados. A janela de contexto de 1M tokens está disponível apenas para Claude Sonnet 4 e Sonnet 4.5.
</Note>

<Tabs>
<Tab title="Tier 4">
| Máximo de tokens de entrada por minuto (ITPM) | Máximo de tokens de saída por minuto (OTPM) |
| --------------------------------------------- | ------------------------------------------- |
| 1.000.000                                     | 200.000                                     |
</Tab>
<Tab title="Personalizado">
Para limites de taxa de contexto longo personalizados para casos de uso enterprise, entre em contato com vendas através do [Claude Console](/settings/limits).
</Tab>
</Tabs>

<Tip>
Para aproveitar ao máximo a janela de contexto de 1M tokens com limites de taxa, use [prompt caching](/docs/pt-BR/build-with-claude/prompt-caching).
</Tip>

### Monitorando seus limites de taxa no Console

Você pode monitorar seu uso de limite de taxa na página [Uso](/settings/usage) do [Claude Console](/). 

Além de fornecer gráficos de token e solicitação, a página Uso fornece dois gráficos de limite de taxa separados. Use esses gráficos para ver que espaço você tem para crescer, quando você pode estar atingindo pico de uso, entender melhor quais limites de taxa solicitar, ou como você pode melhorar suas taxas de caching. Os gráficos visualizam um número de métricas para um determinado limite de taxa (por exemplo, por modelo):

- O gráfico **Rate Limit - Input Tokens** inclui:
  - Máximo horário de tokens de entrada não em cache por minuto
  - Seu limite de taxa atual de tokens de entrada por minuto
  - A taxa de cache para seus tokens de entrada (ou seja, a porcentagem de tokens de entrada lidos do cache)
- O gráfico **Rate Limit - Output Tokens** inclui:
  - Máximo horário de tokens de saída por minuto
  - Seu limite de taxa atual de tokens de saída por minuto

## Definindo limites mais baixos para Workspaces

Para proteger Workspaces em sua Organização do possível uso excessivo, você pode definir limites de gastos e taxa personalizados por Workspace.

Exemplo: Se o limite da sua Organização é 40.000 tokens de entrada por minuto e 8.000 tokens de saída por minuto, você pode limitar um Workspace a 30.000 tokens totais por minuto. Isso protege outros Workspaces do possível uso excessivo e garante uma distribuição mais equitativa de recursos em sua Organização. Os tokens por minuto não utilizados restantes (ou mais, se esse Workspace não usar o limite) ficam disponíveis para outros Workspaces usarem.

Nota:
- Você não pode definir limites no Workspace padrão.
- Se não definido, os limites do Workspace correspondem ao limite da Organização.
- Os limites em toda a Organização sempre se aplicam, mesmo que os limites do Workspace somem mais.
- O suporte para limites de tokens de entrada e saída será adicionado aos Workspaces no futuro.

## Cabeçalhos de resposta

A resposta da API inclui cabeçalhos que mostram o limite de taxa aplicado, uso atual e quando o limite será redefinido.

Os seguintes cabeçalhos são retornados:

| Cabeçalho                                     | Descrição                                                                                                                                     |
| --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `retry-after`                                 | O número de segundos a esperar até que você possa tentar novamente a solicitação. Tentativas anteriores falharão.                             |
| `anthropic-ratelimit-requests-limit`          | O número máximo de solicitações permitidas dentro de qualquer período de limite de taxa.                                                       |
| `anthropic-ratelimit-requests-remaining`      | O número de solicitações restantes antes de ser limitado por taxa.                                                                             |
| `anthropic-ratelimit-requests-reset`          | A hora em que o limite de taxa de solicitação será totalmente reabastecido, fornecido em formato RFC 3339.                                    |
| `anthropic-ratelimit-tokens-limit`            | O número máximo de tokens permitidos dentro de qualquer período de limite de taxa.                                                             |
| `anthropic-ratelimit-tokens-remaining`        | O número de tokens restantes (arredondado para o milhar mais próximo) antes de ser limitado por taxa.                                         |
| `anthropic-ratelimit-tokens-reset`            | A hora em que o limite de taxa de token será totalmente reabastecido, fornecido em formato RFC 3339.                                          |
| `anthropic-ratelimit-input-tokens-limit`      | O número máximo de tokens de entrada permitidos dentro de qualquer período de limite de taxa.                                                  |
| `anthropic-ratelimit-input-tokens-remaining`  | O número de tokens de entrada restantes (arredondado para o milhar mais próximo) antes de ser limitado por taxa.                              |
| `anthropic-ratelimit-input-tokens-reset`      | A hora em que o limite de taxa de token de entrada será totalmente reabastecido, fornecido em formato RFC 3339.                               |
| `anthropic-ratelimit-output-tokens-limit`     | O número máximo de tokens de saída permitidos dentro de qualquer período de limite de taxa.                                                    |
| `anthropic-ratelimit-output-tokens-remaining` | O número de tokens de saída restantes (arredondado para o milhar mais próximo) antes de ser limitado por taxa.                                |
| `anthropic-ratelimit-output-tokens-reset`     | A hora em que o limite de taxa de token de saída será totalmente reabastecido, fornecido em formato RFC 3339.                                 |
| `anthropic-priority-input-tokens-limit`       | O número máximo de tokens de entrada do Tier Prioritário permitidos dentro de qualquer período de limite de taxa. (Apenas Tier Prioritário)     |
| `anthropic-priority-input-tokens-remaining`   | O número de tokens de entrada do Tier Prioritário restantes (arredondado para o milhar mais próximo) antes de ser limitado por taxa. (Apenas Tier Prioritário) |
| `anthropic-priority-input-tokens-reset`       | A hora em que o limite de taxa de token de entrada do Tier Prioritário será totalmente reabastecido, fornecido em formato RFC 3339. (Apenas Tier Prioritário) |
| `anthropic-priority-output-tokens-limit`      | O número máximo de tokens de saída do Tier Prioritário permitidos dentro de qualquer período de limite de taxa. (Apenas Tier Prioritário)      |
| `anthropic-priority-output-tokens-remaining`  | O número de tokens de saída do Tier Prioritário restantes (arredondado para o milhar mais próximo) antes de ser limitado por taxa. (Apenas Tier Prioritário) |
| `anthropic-priority-output-tokens-reset`      | A hora em que o limite de taxa de token de saída do Tier Prioritário será totalmente reabastecido, fornecido em formato RFC 3339. (Apenas Tier Prioritário) |

Os cabeçalhos `anthropic-ratelimit-tokens-*` exibem os valores para o limite mais restritivo atualmente em vigor. Por exemplo, se você excedeu o limite de token por minuto do Workspace, os cabeçalhos conterão os valores de limite de taxa de token por minuto do Workspace. Se os limites do Workspace não se aplicarem, os cabeçalhos retornarão o total de tokens restantes, onde total é a soma de tokens de entrada e saída. Esta abordagem garante que você tenha visibilidade da restrição mais relevante no seu uso atual da API.