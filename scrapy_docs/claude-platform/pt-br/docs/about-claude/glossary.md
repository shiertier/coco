# Glossário

Estes conceitos não são únicos aos modelos de linguagem da Anthropic, mas apresentamos um breve resumo dos termos-chave abaixo.

---

## Janela de contexto

A "janela de contexto" refere-se à quantidade de texto que um modelo de linguagem pode consultar e referenciar ao gerar novo texto. Isso é diferente do grande corpus de dados em que o modelo de linguagem foi treinado, e em vez disso representa uma "memória de trabalho" para o modelo. Uma janela de contexto maior permite que o modelo compreenda e responda a prompts mais complexos e longos, enquanto uma janela de contexto menor pode limitar a capacidade do modelo de lidar com prompts mais longos ou manter coerência ao longo de conversas extensas.

Veja nosso [guia para entender janelas de contexto](/docs/pt-BR/build-with-claude/context-windows) para saber mais.

## Fine-tuning

Fine-tuning é o processo de treinar ainda mais um modelo de linguagem pré-treinado usando dados adicionais. Isso faz com que o modelo comece a representar e imitar os padrões e características do conjunto de dados de fine-tuning. Claude não é um modelo de linguagem básico; ele já foi submetido a fine-tuning para ser um assistente útil. Nossa API atualmente não oferece fine-tuning, mas por favor pergunte ao seu contato da Anthropic se você estiver interessado em explorar essa opção. O fine-tuning pode ser útil para adaptar um modelo de linguagem a um domínio específico, tarefa ou estilo de escrita, mas requer consideração cuidadosa dos dados de fine-tuning e do impacto potencial no desempenho e vieses do modelo.

## HHH

Estes três H's representam os objetivos da Anthropic em garantir que Claude seja benéfico para a sociedade:

- Uma IA **útil** tentará executar a tarefa ou responder à pergunta feita da melhor forma possível, fornecendo informações relevantes e úteis.
- Uma IA **honesta** dará informações precisas, e não alucinará ou confabulará. Ela reconhecerá suas limitações e incertezas quando apropriado.
- Uma IA **inofensiva** não será ofensiva ou discriminatória, e quando solicitada a ajudar em um ato perigoso ou antiético, a IA deve recusar educadamente e explicar por que não pode cumprir.

## Latência

Latência, no contexto de IA generativa e grandes modelos de linguagem, refere-se ao tempo que leva para o modelo responder a um determinado prompt. É o atraso entre enviar um prompt e receber a saída gerada. Menor latência indica tempos de resposta mais rápidos, o que é crucial para aplicações em tempo real, chatbots e experiências interativas. Fatores que podem afetar a latência incluem tamanho do modelo, capacidades de hardware, condições de rede e a complexidade do prompt e da resposta gerada.

## LLM

Grandes modelos de linguagem (LLMs) são modelos de linguagem de IA com muitos parâmetros que são capazes de executar uma variedade de tarefas surpreendentemente úteis. Esses modelos são treinados em vastas quantidades de dados de texto e podem gerar texto semelhante ao humano, responder perguntas, resumir informações e muito mais. Claude é um assistente conversacional baseado em um grande modelo de linguagem que foi submetido a fine-tuning e treinado usando RLHF para ser mais útil, honesto e inofensivo.

## MCP (Model Context Protocol)

Model Context Protocol (MCP) é um protocolo aberto que padroniza como aplicações fornecem contexto aos LLMs. Como uma porta USB-C para aplicações de IA, o MCP fornece uma maneira unificada de conectar modelos de IA a diferentes fontes de dados e ferramentas. O MCP permite que sistemas de IA mantenham contexto consistente através de interações e acessem recursos externos de maneira padronizada. Veja nossa [documentação do MCP](/docs/pt-BR/mcp) para saber mais.

## Conector MCP

O conector MCP é um recurso que permite aos usuários da API conectar-se a servidores MCP diretamente da Messages API sem construir um cliente MCP. Isso permite integração perfeita com ferramentas e serviços compatíveis com MCP através da API do Claude. O conector MCP suporta recursos como chamada de ferramentas e está disponível em beta público. Veja nossa [documentação do conector MCP](/docs/pt-BR/agents-and-tools/mcp-connector) para saber mais.

## Pré-treinamento

Pré-treinamento é o processo inicial de treinar modelos de linguagem em um grande corpus não rotulado de texto. No caso do Claude, modelos de linguagem autorregressivos (como o modelo subjacente do Claude) são pré-treinados para prever a próxima palavra, dado o contexto anterior de texto no documento. Esses modelos pré-treinados não são inerentemente bons em responder perguntas ou seguir instruções, e frequentemente requerem habilidade profunda em engenharia de prompt para elicitar comportamentos desejados. Fine-tuning e RLHF são usados para refinar esses modelos pré-treinados, tornando-os mais úteis para uma ampla gama de tarefas.

## RAG (Retrieval augmented generation)

Geração aumentada por recuperação (RAG) é uma técnica que combina recuperação de informações com geração de modelo de linguagem para melhorar a precisão e relevância do texto gerado, e para fundamentar melhor a resposta do modelo em evidências. No RAG, um modelo de linguagem é aumentado com uma base de conhecimento externa ou um conjunto de documentos que é passado para a janela de contexto. Os dados são recuperados em tempo de execução quando uma consulta é enviada ao modelo, embora o modelo em si não necessariamente recupere os dados (mas pode com [uso de ferramentas](/docs/pt-BR/agents-and-tools/tool-use/overview) e uma função de recuperação). Ao gerar texto, informações relevantes primeiro devem ser recuperadas da base de conhecimento com base no prompt de entrada, e então passadas ao modelo junto com a consulta original. O modelo usa essas informações para guiar a saída que gera. Isso permite que o modelo acesse e utilize informações além de seus dados de treinamento, reduzindo a dependência de memorização e melhorando a precisão factual do texto gerado. RAG pode ser particularmente útil para tarefas que requerem informações atualizadas, conhecimento específico de domínio ou citação explícita de fontes. No entanto, a eficácia do RAG depende da qualidade e relevância da base de conhecimento externa e do conhecimento que é recuperado em tempo de execução.

## RLHF

Aprendizado por Reforço a partir de Feedback Humano (RLHF) é uma técnica usada para treinar um modelo de linguagem pré-treinado a se comportar de maneiras que são consistentes com preferências humanas. Isso pode incluir ajudar o modelo a seguir instruções mais efetivamente ou agir mais como um chatbot. O feedback humano consiste em classificar um conjunto de dois ou mais textos de exemplo, e o processo de aprendizado por reforço encoraja o modelo a preferir saídas que são similares às classificadas mais alto. Claude foi treinado usando RLHF para ser um assistente mais útil. Para mais detalhes, você pode ler [o artigo da Anthropic sobre o assunto](https://arxiv.org/abs/2204.05862).

## Temperatura

Temperatura é um parâmetro que controla a aleatoriedade das previsões de um modelo durante a geração de texto. Temperaturas mais altas levam a saídas mais criativas e diversas, permitindo múltiplas variações na formulação e, no caso de ficção, variação nas respostas também. Temperaturas mais baixas resultam em saídas mais conservadoras e determinísticas que se aderem à formulação e respostas mais prováveis. Ajustar a temperatura permite aos usuários encorajar um modelo de linguagem a explorar escolhas e sequências de palavras raras, incomuns ou surpreendentes, em vez de apenas selecionar as previsões mais prováveis.

## TTFT (Time to first token)

Tempo para Primeiro Token (TTFT) é uma métrica de desempenho que mede o tempo que leva para um modelo de linguagem gerar o primeiro token de sua saída após receber um prompt. É um indicador importante da responsividade do modelo e é particularmente relevante para aplicações interativas, chatbots e sistemas em tempo real onde os usuários esperam feedback inicial rápido. Um TTFT menor indica que o modelo pode começar a gerar uma resposta mais rapidamente, proporcionando uma experiência de usuário mais fluida e envolvente. Fatores que podem influenciar o TTFT incluem tamanho do modelo, capacidades de hardware, condições de rede e a complexidade do prompt.

## Tokens

Tokens são as menores unidades individuais de um modelo de linguagem, e podem corresponder a palavras, subpalavras, caracteres ou mesmo bytes (no caso de Unicode). Para Claude, um token representa aproximadamente 3,5 caracteres em inglês, embora o número exato possa variar dependendo do idioma usado. Tokens são tipicamente ocultos ao interagir com modelos de linguagem no nível de "texto", mas se tornam relevantes ao examinar as entradas e saídas exatas de um modelo de linguagem. Quando Claude recebe texto para avaliar, o texto (consistindo de uma série de caracteres) é codificado em uma série de tokens para o modelo processar. Tokens maiores permitem eficiência de dados durante inferência e pré-treinamento (e são utilizados quando possível), enquanto tokens menores permitem que um modelo lide com palavras incomuns ou nunca vistas antes. A escolha do método de tokenização pode impactar o desempenho do modelo, tamanho do vocabulário e capacidade de lidar com palavras fora do vocabulário.