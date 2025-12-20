# Visão geral dos modelos

Claude é uma família de modelos de linguagem grandes de última geração desenvolvidos pela Anthropic. Este guia apresenta nossos modelos e compara seu desempenho.

---

## Escolhendo um modelo

Se você não tem certeza de qual modelo usar, recomendamos começar com **Claude Sonnet 4.5**. Ele oferece o melhor equilíbrio de inteligência, velocidade e custo para a maioria dos casos de uso, com desempenho excepcional em tarefas de codificação e agentes.

Todos os modelos Claude atuais suportam entrada de texto e imagem, saída de texto, capacidades multilíngues e visão. Os modelos estão disponíveis via API Anthropic, AWS Bedrock e Google Vertex AI.

Depois de escolher um modelo, [aprenda como fazer sua primeira chamada de API](/docs/pt-BR/get-started).

### Comparação dos modelos mais recentes

| Recurso | Claude Sonnet 4.5 | Claude Haiku 4.5 | Claude Opus 4.5 |
|:--------|:------------------|:-----------------|:----------------|
| **Descrição** | Nosso modelo inteligente para agentes complexos e codificação | Nosso modelo mais rápido com inteligência próxima à fronteira | Modelo premium combinando inteligência máxima com desempenho prático |
| **Claude API ID** | claude-sonnet-4-5-20250929 | claude-haiku-4-5-20251001 | claude-opus-4-5-20251101 |
| **Claude API alias**<sup>1</sup> | claude-sonnet-4-5 | claude-haiku-4-5 | claude-opus-4-5 |
| **AWS Bedrock ID** | anthropic.claude-sonnet-4-5-20250929-v1:0 | anthropic.claude-haiku-4-5-20251001-v1:0 | anthropic.claude-opus-4-5-20251101-v1:0 |
| **GCP Vertex AI ID** | claude-sonnet-4-5@20250929 | claude-haiku-4-5@20251001 | claude-opus-4-5@20251101 |
| **Preços**<sup>2</sup> | \$3 / input MTok<br/>\$15 / output MTok | \$1 / input MTok<br/>\$5 / output MTok | \$5 / input MTok<br/>\$25 / output MTok |
| **[Pensamento estendido](/docs/pt-BR/build-with-claude/extended-thinking)** | Sim | Sim | Sim |
| **[Tier de Prioridade](/docs/pt-BR/api/service-tiers)** | Sim | Sim | Sim |
| **Latência comparativa** | Rápido | Mais rápido | Moderado |
| **Janela de contexto** | <Tooltip tooltipContent="~150K palavras \ ~680K caracteres unicode">200K tokens</Tooltip> / <br/> <Tooltip tooltipContent="~750K palavras \ ~3.4M caracteres unicode">1M tokens</Tooltip> (beta)<sup>3</sup> | <Tooltip tooltipContent="~150K palavras \ ~680K caracteres unicode">200K tokens</Tooltip> | <Tooltip tooltipContent="~150K palavras \ ~680K caracteres unicode">200K tokens</Tooltip> |
| **Saída máxima** | 64K tokens | 64K tokens | 64K tokens |
| **Corte de conhecimento confiável** | Jan 2025<sup>4</sup> | Fev 2025 | Mai 2025<sup>4</sup> |
| **Corte de dados de treinamento** | Jul 2025 | Jul 2025 | Ago 2025 |

_<sup>1 - Os aliases apontam automaticamente para o snapshot de modelo mais recente. Quando lançamos novos snapshots de modelo, migramos aliases para apontar para a versão mais recente de um modelo, normalmente dentro de uma semana do novo lançamento. Embora os aliases sejam úteis para experimentação, recomendamos usar versões de modelo específicas (por exemplo, `claude-sonnet-4-5-20250929`) em aplicações de produção para garantir comportamento consistente.</sup>_

_<sup>2 - Veja nossa [página de preços](/docs/pt-BR/about-claude/pricing) para informações completas de preços, incluindo descontos de API em lote, taxas de cache de prompt, custos de pensamento estendido e taxas de processamento de visão.</sup>_

_<sup>3 - Claude Sonnet 4.5 suporta uma [janela de contexto de 1M tokens](/docs/pt-BR/build-with-claude/context-windows#1m-token-context-window) ao usar o cabeçalho beta `context-1m-2025-08-07`. [Preços de contexto longo](/docs/pt-BR/about-claude/pricing#long-context-pricing) se aplicam a solicitações que excedem 200K tokens.</sup>_

_<sup>4 - **Corte de conhecimento confiável** indica a data até a qual o conhecimento de um modelo é mais extenso e confiável. **Corte de dados de treinamento** é o intervalo de data mais amplo dos dados de treinamento usados. Por exemplo, Claude Sonnet 4.5 foi treinado em informações disponíveis publicamente até julho de 2025, mas seu conhecimento é mais extenso e confiável até janeiro de 2025. Para mais informações, veja [Hub de Transparência da Anthropic](https://www.anthropic.com/transparency).</sup>_

<Note>Modelos com a mesma data de snapshot (por exemplo, 20240620) são idênticos em todas as plataformas e não mudam. A data do snapshot no nome do modelo garante consistência e permite que os desenvolvedores contem com desempenho estável em diferentes ambientes.</Note>

<Note>A partir de **Claude Sonnet 4.5 e todos os modelos futuros**, AWS Bedrock e Google Vertex AI oferecem dois tipos de endpoint: **endpoints globais** (roteamento dinâmico para máxima disponibilidade) e **endpoints regionais** (roteamento de dados garantido através de regiões geográficas específicas). Para mais informações, veja a [seção de preços de plataforma de terceiros](/docs/pt-BR/about-claude/pricing#third-party-platform-pricing).</Note>

<section title="Modelos legados">

Os seguintes modelos ainda estão disponíveis, mas recomendamos migrar para modelos atuais para melhor desempenho:

| Recurso | Claude Opus 4.1 | Claude Sonnet 4 | Claude Sonnet 3.7 | Claude Opus 4 | Claude Haiku 3 |
|:--------|:----------------|:----------------|:------------------|:--------------|:---------------|
| **Claude API ID** | claude-opus-4-1-20250805 | claude-sonnet-4-20250514 | claude-3-7-sonnet-20250219 | claude-opus-4-20250514 | claude-3-haiku-20240307 |
| **Claude API alias** | claude-opus-4-1 | claude-sonnet-4-0 | claude-3-7-sonnet-latest | claude-opus-4-0 | — |
| **AWS Bedrock ID** | anthropic.claude-opus-4-1-20250805-v1:0 | anthropic.claude-sonnet-4-20250514-v1:0 | anthropic.claude-3-7-sonnet-20250219-v1:0 | anthropic.claude-opus-4-20250514-v1:0 | anthropic.claude-3-haiku-20240307-v1:0 |
| **GCP Vertex AI ID** | claude-opus-4-1@20250805 | claude-sonnet-4@20250514 | claude-3-7-sonnet@20250219 | claude-opus-4@20250514 | claude-3-haiku@20240307 |
| **Preços** | \$15 / input MTok<br/>\$75 / output MTok | \$3 / input MTok<br/>\$15 / output MTok | \$3 / input MTok<br/>\$15 / output MTok | \$15 / input MTok<br/>\$75 / output MTok | \$0.25 / input MTok<br/>\$1.25 / output MTok |
| **[Pensamento estendido](/docs/pt-BR/build-with-claude/extended-thinking)** | Sim | Sim | Sim | Sim | Não |
| **[Tier de Prioridade](/docs/pt-BR/api/service-tiers)** | Sim | Sim | Sim | Sim | Não |
| **Latência comparativa** | Moderado | Rápido | Rápido | Moderado | Rápido |
| **Janela de contexto** | <Tooltip tooltipContent="~150K palavras \ ~680K caracteres unicode">200K tokens</Tooltip> | <Tooltip tooltipContent="~150K palavras \ ~680K caracteres unicode">200K tokens</Tooltip> / <br/> <Tooltip tooltipContent="~750K palavras \ ~3.4M caracteres unicode">1M tokens</Tooltip> (beta)<sup>1</sup> | <Tooltip tooltipContent="~150K palavras \ ~680K caracteres unicode">200K tokens</Tooltip> | <Tooltip tooltipContent="~150K palavras \ ~680K caracteres unicode">200K tokens</Tooltip> | <Tooltip tooltipContent="~150K palavras \ ~680K caracteres unicode">200K tokens</Tooltip> |
| **Saída máxima** | 32K tokens | 64K tokens | 64K tokens / 128K tokens (beta)<sup>4</sup> | 32K tokens | 4K tokens |
| **Corte de conhecimento confiável** | Jan 2025<sup>2</sup> | Jan 2025<sup>2</sup> | Out 2024<sup>2</sup> | Jan 2025<sup>2</sup> | <sup>3</sup> |
| **Corte de dados de treinamento** | Mar 2025 | Mar 2025 | Nov 2024 | Mar 2025 | Ago 2023 |

_<sup>1 - Claude Sonnet 4 suporta uma [janela de contexto de 1M tokens](/docs/pt-BR/build-with-claude/context-windows#1m-token-context-window) ao usar o cabeçalho beta `context-1m-2025-08-07`. [Preços de contexto longo](/docs/pt-BR/about-claude/pricing#long-context-pricing) se aplicam a solicitações que excedem 200K tokens.</sup>_

_<sup>2 - **Corte de conhecimento confiável** indica a data até a qual o conhecimento de um modelo é mais extenso e confiável. **Corte de dados de treinamento** é o intervalo de data mais amplo dos dados de treinamento usados.</sup>_

_<sup>3 - Alguns modelos Haiku têm uma única data de corte de dados de treinamento.</sup>_

_<sup>4 - Inclua o cabeçalho beta `output-128k-2025-02-19` em sua solicitação de API para aumentar o comprimento máximo de token de saída para 128K tokens para Claude Sonnet 3.7. Sugerimos fortemente usar nossa [API de Mensagens com streaming](/docs/pt-BR/build-with-claude/streaming) para evitar timeouts ao gerar saídas mais longas. Veja nossa orientação sobre [solicitações longas](/docs/pt-BR/api/errors#long-requests) para mais detalhes.</sup>_

</section>

## Desempenho de prompt e saída

Os modelos Claude 4 se destacam em:
- **Desempenho**: Resultados de primeira classe em raciocínio, codificação, tarefas multilíngues, manipulação de contexto longo, honestidade e processamento de imagens. Veja o [post do blog Claude 4](http://www.anthropic.com/news/claude-4) para mais informações.
- **Respostas envolventes**: Os modelos Claude são ideais para aplicações que exigem interações ricas e semelhantes às humanas.

    - Se você preferir respostas mais concisas, pode ajustar seus prompts para guiar o modelo em direção ao comprimento de saída desejado. Consulte nossos [guias de engenharia de prompt](/docs/pt-BR/build-with-claude/prompt-engineering) para detalhes.
    - Para práticas recomendadas específicas de prompt do Claude 4, veja nosso [guia de práticas recomendadas do Claude 4](/docs/pt-BR/build-with-claude/prompt-engineering/claude-4-best-practices).
- **Qualidade de saída**: Ao migrar de gerações de modelos anteriores para Claude 4, você pode notar melhorias maiores no desempenho geral.

## Migrando para Claude 4.5

Se você está usando modelos Claude 3 no momento, recomendamos migrar para Claude 4.5 para aproveitar a inteligência melhorada e as capacidades aprimoradas. Para instruções detalhadas de migração, veja [Migrando para Claude 4.5](/docs/pt-BR/about-claude/models/migrating-to-claude-4).

## Comece com Claude

Se você está pronto para começar a explorar o que Claude pode fazer por você, vamos mergulhar! Quer você seja um desenvolvedor procurando integrar Claude em suas aplicações ou um usuário querendo experimentar o poder da IA em primeira mão, nós temos você coberto.

<Note>Procurando conversar com Claude? Visite [claude.ai](http://www.claude.ai)!</Note>

<CardGroup cols={3}>
  <Card title="Introdução ao Claude" icon="check" href="/docs/pt-BR/intro">
    Explore as capacidades do Claude e o fluxo de desenvolvimento.
  </Card>
  <Card title="Início rápido" icon="lightning" href="/docs/pt-BR/get-started">
    Aprenda como fazer sua primeira chamada de API em minutos.
  </Card>
  <Card title="Console Claude" icon="code" href="/">
    Crie e teste prompts poderosos diretamente em seu navegador.
  </Card>
</CardGroup>

Se você tiver alguma dúvida ou precisar de assistência, não hesite em entrar em contato com nossa [equipe de suporte](https://support.claude.com/) ou consultar a [comunidade Discord](https://www.anthropic.com/discord).