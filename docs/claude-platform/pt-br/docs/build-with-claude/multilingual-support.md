# Suporte multilíngue

Claude se destaca em tarefas em múltiplos idiomas, mantendo um desempenho forte entre idiomas em relação ao inglês.

---

## Visão geral

Claude demonstra capacidades multilíngues robustas, com desempenho particularmente forte em tarefas zero-shot em vários idiomas. O modelo mantém desempenho relativo consistente em idiomas amplamente falados e de recursos limitados, tornando-o uma escolha confiável para aplicações multilíngues.

Observe que Claude é capaz em muitos idiomas além dos avaliados abaixo. Encorajamos testes com quaisquer idiomas relevantes para seus casos de uso específicos.

## Dados de desempenho

Abaixo estão os scores de avaliação zero-shot chain-of-thought para modelos Claude em diferentes idiomas, mostrados como um percentual relativo ao desempenho em inglês (100%):

| Idioma | Claude Opus 4.1<sup>1</sup> | Claude Opus 4<sup>1</sup> | Claude Sonnet 4.5<sup>1</sup> | Claude Sonnet 4<sup>1</sup> | Claude Haiku 4.5<sup>1</sup> |
|----------|---------------|---------------|---------------|-----------------|------------------|
| Inglês (linha de base, fixado em 100%) | 100% | 100% | 100% | 100% | 100% |
| Espanhol | 98.1% | 98.0% | 98.2% | 97.5% | 96.4% |
| Português (Brasil) | 97.8% | 97.3% | 97.8% | 97.2% | 96.1% |
| Italiano | 97.7% | 97.5% | 97.9% | 97.3% | 96.0% |
| Francês | 97.9% | 97.7% | 97.5% | 97.1% | 95.7% |
| Indonésio | 97.3% | 97.2% | 97.3% | 96.2% | 94.2% |
| Alemão | 97.7% | 97.1% | 97.0% | 94.7% | 94.3% |
| Árabe | 97.1% | 96.9% | 97.2% | 96.1% | 92.5% |
| Chinês (Simplificado) | 97.1% | 96.7% | 96.9% | 95.9% | 94.2% |
| Coreano | 96.6% | 96.4% | 96.7% | 95.9% | 93.3% |
| Japonês | 96.9% | 96.2% | 96.8% | 95.6% | 93.5% |
| Hindi | 96.8% | 96.7% | 96.7% | 95.8% | 92.4% |
| Bengali | 95.7% | 95.2% | 95.4% | 94.4% | 90.4% |
| Suaíli | 89.8% | 89.5% | 91.1% | 87.1% | 78.3% |
| Iorubá | 80.3% | 78.9% | 79.7% | 76.4% | 52.7% |

<sup>1</sup> Com [pensamento estendido](/docs/pt-BR/build-with-claude/extended-thinking).

<Note>
Essas métricas são baseadas em conjuntos de testes em inglês [MMLU (Massive Multitask Language Understanding)](https://en.wikipedia.org/wiki/MMLU) que foram traduzidos para 14 idiomas adicionais por tradutores humanos profissionais, conforme documentado no [repositório simple-evals do OpenAI](https://github.com/openai/simple-evals/blob/main/multilingual_mmlu_benchmark_results.md). O uso de tradutores humanos para esta avaliação garante traduções de alta qualidade, particularmente importante para idiomas com menos recursos digitais.
</Note>

***

## Melhores práticas

Ao trabalhar com conteúdo multilíngue:

1. **Forneça contexto de idioma claro**: Embora Claude possa detectar o idioma de destino automaticamente, declarar explicitamente o idioma de entrada/saída desejado melhora a confiabilidade. Para maior fluência, você pode solicitar ao Claude que use "linguagem idiomática como se fosse um falante nativo."
2. **Use scripts nativos**: Envie texto em seu script nativo em vez de transliteração para obter resultados ideais
3. **Considere o contexto cultural**: A comunicação eficaz geralmente requer consciência cultural e regional além da pura tradução

Também sugerimos seguir nossas [diretrizes gerais de engenharia de prompts](/docs/pt-BR/build-with-claude/prompt-engineering/overview) para melhorar melhor o desempenho do Claude.

***

## Considerações de suporte de idioma

- Claude processa entrada e gera saída na maioria dos idiomas mundiais que usam caracteres Unicode padrão
- O desempenho varia por idioma, com capacidades particularmente fortes em idiomas amplamente falados
- Mesmo em idiomas com menos recursos digitais, Claude mantém capacidades significativas

<CardGroup cols={2}>
  <Card title="Guia de Engenharia de Prompts" icon="edit" href="/docs/pt-BR/build-with-claude/prompt-engineering/overview">
    Domine a arte de criar prompts para obter o máximo do Claude.
  </Card>
  <Card title="Biblioteca de Prompts" icon="books" href="/docs/pt-BR/resources/prompt-library">
    Encontre uma ampla gama de prompts pré-criados para várias tarefas e indústrias. Perfeito para inspiração ou inicializações rápidas.
  </Card>
</CardGroup>