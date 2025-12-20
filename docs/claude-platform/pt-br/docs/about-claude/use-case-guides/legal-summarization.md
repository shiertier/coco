# Resumo de documentos legais

Este guia apresenta como aproveitar os recursos avançados de processamento de linguagem natural do Claude para resumir eficientemente documentos legais, extraindo informações-chave e acelerando pesquisas jurídicas. Com Claude, você pode simplificar a revisão de contratos, preparação de litígios e trabalho regulatório, economizando tempo e garantindo precisão em seus processos legais.

---

> Visite nosso [livro de receitas de resumo](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb) para ver um exemplo de implementação de resumo de documentos legais usando Claude.

## Antes de construir com Claude

### Decida se deve usar Claude para resumo de documentos legais

Aqui estão alguns indicadores-chave de que você deve usar um LLM como Claude para resumir documentos legais:

<section title="Você deseja revisar um alto volume de documentos de forma eficiente e acessível">
A revisão de documentos em larga escala pode ser demorada e cara quando feita manualmente. Claude pode processar e resumir grandes quantidades de documentos legais rapidamente, reduzindo significativamente o tempo e o custo associados à revisão de documentos. Essa capacidade é particularmente valiosa para tarefas como due diligence, análise de contratos ou descoberta de litígios, onde a eficiência é crucial.
</section>
<section title="Você requer extração automatizada de metadados-chave">
Claude pode extrair e categorizar eficientemente metadados importantes de documentos legais, como partes envolvidas, datas, termos de contrato ou cláusulas específicas. Essa extração automatizada pode ajudar a organizar informações, facilitando a busca, análise e gerenciamento de grandes conjuntos de documentos. É especialmente útil para gerenciamento de contratos, verificações de conformidade ou criação de bancos de dados pesquisáveis de informações legais.
</section>
<section title="Você deseja gerar resumos claros, concisos e padronizados">
Claude pode gerar resumos estruturados que seguem formatos predeterminados, facilitando para profissionais jurídicos compreender rapidamente os pontos-chave de vários documentos. Esses resumos padronizados podem melhorar a legibilidade, facilitar a comparação entre documentos e aprimorar a compreensão geral, especialmente ao lidar com linguagem legal complexa ou jargão técnico.
</section>
<section title="Você precisa de citações precisas para seus resumos">
Ao criar resumos legais, a atribuição adequada e a citação são cruciais para garantir credibilidade e conformidade com padrões legais. Claude pode ser instruído a incluir citações precisas para todos os pontos legais referenciados, facilitando para profissionais jurídicos revisar e verificar as informações resumidas.
</section>
<section title="Você deseja simplificar e acelerar seu processo de pesquisa jurídica">
Claude pode auxiliar na pesquisa jurídica analisando rapidamente grandes volumes de jurisprudência, estatutos e comentários legais. Pode identificar precedentes relevantes, extrair princípios legais-chave e resumir argumentos legais complexos. Essa capacidade pode acelerar significativamente o processo de pesquisa, permitindo que profissionais jurídicos se concentrem em análise de nível superior e desenvolvimento de estratégia.
</section>

### Determine os detalhes que você deseja que o resumo extraia
Não existe um único resumo correto para qualquer documento. Sem direção clara, pode ser difícil para Claude determinar quais detalhes incluir. Para obter resultados ideais, identifique as informações específicas que você deseja incluir no resumo.

Por exemplo, ao resumir um contrato de subaluguel, você pode desejar extrair os seguintes pontos-chave:

```python
details_to_extract = [
    'Parties involved (sublessor, sublessee, and original lessor)',
    'Property details (address, description, and permitted use)', 
    'Term and rent (start date, end date, monthly rent, and security deposit)',
    'Responsibilities (utilities, maintenance, and repairs)',
    'Consent and notices (landlord\'s consent, and notice requirements)',
    'Special provisions (furniture, parking, and subletting restrictions)'
]
```

### Estabeleça critérios de sucesso

Avaliar a qualidade dos resumos é uma tarefa notoriamente desafiadora. Ao contrário de muitas outras tarefas de processamento de linguagem natural, a avaliação de resumos geralmente carece de métricas claras e objetivas. O processo pode ser altamente subjetivo, com diferentes leitores valorizando diferentes aspectos de um resumo. Aqui estão critérios que você pode desejar considerar ao avaliar o desempenho do Claude no resumo de documentos legais.

<section title="Correção factual">
O resumo deve representar com precisão os fatos, conceitos legais e pontos-chave do documento.
</section>
<section title="Precisão legal">
A terminologia e referências a estatutos, jurisprudência ou regulamentações devem estar corretas e alinhadas com padrões legais.
</section>
<section title="Concisão">
O resumo deve condensar o documento legal aos seus pontos essenciais sem perder detalhes importantes.
</section>
<section title="Consistência">
Ao resumir múltiplos documentos, o LLM deve manter uma estrutura e abordagem consistentes para cada resumo.
</section>
<section title="Legibilidade">
O texto deve ser claro e fácil de entender. Se o público não for especialista em direito, o resumo não deve incluir jargão legal que possa confundir o público.
</section>
<section title="Viés e justiça">
O resumo deve apresentar uma representação imparcial e justa dos argumentos e posições legais.
</section>

Consulte nosso guia sobre [estabelecimento de critérios de sucesso](/docs/pt-BR/test-and-evaluate/define-success) para mais informações.

---

## Como resumir documentos legais usando Claude

### Selecione o modelo Claude correto

A precisão do modelo é extremamente importante ao resumir documentos legais. Claude Sonnet 4.5 é uma excelente escolha para casos de uso como este, onde alta precisão é necessária. Se o tamanho e a quantidade de seus documentos forem grandes, de modo que os custos começarem a se tornar uma preocupação, você também pode tentar usar um modelo menor como Claude Haiku 4.5.

Para ajudar a estimar esses custos, abaixo está uma comparação do custo para resumir 1.000 contratos de subaluguel usando Sonnet e Haiku:

* **Tamanho do conteúdo**
    * Número de contratos: 1.000
    * Caracteres por contrato: 300.000
    * Total de caracteres: 300M

* **Tokens estimados**
    * Tokens de entrada: 86M (assumindo 1 token por 3,5 caracteres)
    * Tokens de saída por resumo: 350
    * Total de tokens de saída: 350.000
 
* **Custo estimado do Claude Sonnet 4.5**
    * Custo de token de entrada: 86 MTok * \$3.00/MTok = \$258
    * Custo de token de saída: 0.35 MTok * \$15.00/MTok = \$5.25
    * Custo total: \$258.00 + \$5.25 = \$263.25

* **Custo estimado do Claude Haiku 3**
    * Custo de token de entrada: 86 MTok * \$0.25/MTok = \$21.50
    * Custo de token de saída: 0.35 MTok * \$1.25/MTok = \$0.44
    * Custo total: \$21.50 + \$0.44 = \$21.96

<Tip>Os custos reais podem diferir dessas estimativas. Essas estimativas são baseadas no exemplo destacado na seção sobre [prompting](#build-a-strong-prompt).</Tip>

### Transforme documentos em um formato que Claude possa processar

Antes de começar a resumir documentos, você precisa preparar seus dados. Isso envolve extrair texto de PDFs, limpar o texto e garantir que esteja pronto para ser processado por Claude.

Aqui está uma demonstração desse processo em um PDF de amostra:

```python
from io import BytesIO
import re

import pypdf
import requests

def get_llm_text(pdf_file):
    reader = pypdf.PdfReader(pdf_file)
    text = "\n".join([page.extract_text() for page in reader.pages])

    # Remove extra whitespace
    text = re.sub(r'\s+', ' ', text) 

    # Remove page numbers
    text = re.sub(r'\n\s*\d+\s*\n', '\n', text) 

    return text


# Create the full URL from the GitHub repository
url = "https://raw.githubusercontent.com/anthropics/anthropic-cookbook/main/skills/summarization/data/Sample Sublease Agreement.pdf"
url = url.replace(" ", "%20")

# Download the PDF file into memory
response = requests.get(url)

# Load the PDF from memory
pdf_file = BytesIO(response.content)

document_text = get_llm_text(pdf_file) 
print(document_text[:50000]) 
```

Neste exemplo, primeiro baixamos um PDF de um contrato de subaluguel de amostra usado no [livro de receitas de resumo](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/data/Sample%20Sublease%20Agreement.pdf). Este contrato foi obtido de um contrato de subaluguel disponível publicamente do [site sec.gov](https://www.sec.gov/Archives/edgar/data/1045425/000119312507044370/dex1032.htm).

Usamos a biblioteca pypdf para extrair o conteúdo do PDF e convertê-lo em texto. Os dados de texto são então limpos removendo espaços em branco extras e números de página.

### Construa um prompt forte

Claude pode se adaptar a vários estilos de resumo. Você pode alterar os detalhes do prompt para guiar Claude a ser mais ou menos verboso, incluir mais ou menos terminologia técnica, ou fornecer um resumo de nível superior ou inferior do contexto em questão.

Aqui está um exemplo de como criar um prompt que garanta que os resumos gerados sigam uma estrutura consistente ao analisar contratos de subaluguel:

```python
import anthropic

# Initialize the Anthropic client
client = anthropic.Anthropic()

def summarize_document(text, details_to_extract, model="claude-sonnet-4-5", max_tokens=1000):

    # Format the details to extract to be placed within the prompt's context
    details_to_extract_str = '\n'.join(details_to_extract)
    
    # Prompt the model to summarize the sublease agreement
    prompt = f"""Summarize the following sublease agreement. Focus on these key aspects:

    {details_to_extract_str}

    Provide the summary in bullet points nested within the XML header for each section. For example:

    <parties involved>
    - Sublessor: [Name]
    // Add more details as needed
    </parties involved>
    
    If any information is not explicitly stated in the document, note it as "Not specified". Do not preamble.

    Sublease agreement text:
    {text}
    """

    response = client.messages.create(
        model=model,
        max_tokens=max_tokens,
        system="You are a legal analyst specializing in real estate law, known for highly accurate and detailed summaries of sublease agreements.",
        messages=[
            {"role": "user", "content": prompt},
            {"role": "assistant", "content": "Here is the summary of the sublease agreement: <summary>"}
        ],
        stop_sequences=["</summary>"]
    )

    return response.content[0].text

sublease_summary = summarize_document(document_text, details_to_extract)
print(sublease_summary)
```

Este código implementa uma função `summarize_document` que usa Claude para resumir o conteúdo de um contrato de subaluguel. A função aceita uma string de texto e uma lista de detalhes a extrair como entradas. Neste exemplo, chamamos a função com as variáveis `document_text` e `details_to_extract` que foram definidas nos trechos de código anteriores.

Dentro da função, um prompt é gerado para Claude, incluindo o documento a ser resumido, os detalhes a extrair e instruções específicas para resumir o documento. O prompt instrui Claude a responder com um resumo de cada detalhe a extrair aninhado dentro de cabeçalhos XML.

Como decidimos produzir cada seção do resumo dentro de tags, cada seção pode ser facilmente analisada como uma etapa de pós-processamento. Essa abordagem permite resumos estruturados que podem ser adaptados para seu caso de uso, de modo que cada resumo siga o mesmo padrão.

### Avalie seu prompt

O prompting geralmente requer testes e otimização para estar pronto para produção. Para determinar a prontidão de sua solução, avalie a qualidade de seus resumos usando um processo sistemático que combine métodos quantitativos e qualitativos. Criar uma [avaliação empírica forte](/docs/pt-BR/test-and-evaluate/develop-tests#building-evals-and-test-cases) baseada em seus critérios de sucesso definidos permitirá que você otimize seus prompts. Aqui estão algumas métricas que você pode desejar incluir em sua avaliação empírica:

<section title="Pontuações ROUGE">
Isso mede a sobreposição entre o resumo gerado e um resumo de referência criado por especialista. Essa métrica se concentra principalmente em recall e é útil para avaliar a cobertura de conteúdo.
</section>
<section title="Pontuações BLEU">
Embora originalmente desenvolvida para tradução automática, essa métrica pode ser adaptada para tarefas de resumo. As pontuações BLEU medem a precisão de correspondências de n-gramas entre o resumo gerado e resumos de referência. Uma pontuação mais alta indica que o resumo gerado contém frases e terminologia semelhantes ao resumo de referência.
</section>
<section title="Similaridade de incorporação contextual">
Essa métrica envolve criar representações vetoriais (incorporações) de resumos gerados e de referência. A similaridade entre essas incorporações é então calculada, geralmente usando similaridade de cosseno. Pontuações de similaridade mais altas indicam que o resumo gerado captura o significado semântico e o contexto do resumo de referência, mesmo que a redação exata seja diferente.
</section>
<section title="Classificação baseada em LLM">
Este método envolve usar um LLM como Claude para avaliar a qualidade dos resumos gerados em relação a uma rubrica de pontuação. A rubrica pode ser adaptada às suas necessidades específicas, avaliando fatores-chave como precisão, completude e coerência. Para orientação sobre como implementar classificação baseada em LLM, consulte estas [dicas](/docs/pt-BR/test-and-evaluate/develop-tests#tips-for-llm-based-grading).
</section>
<section title="Avaliação humana">
Além de criar os resumos de referência, especialistas jurídicos também podem avaliar a qualidade dos resumos gerados. Embora isso seja caro e demorado em escala, isso geralmente é feito em alguns resumos como uma verificação de sanidade antes de implantar em produção.
</section>

### Implante seu prompt

Aqui estão algumas considerações adicionais a ter em mente ao implantar sua solução em produção.

1. **Garanta nenhuma responsabilidade:** Compreenda as implicações legais de erros nos resumos, que podem levar a responsabilidade legal para sua organização ou clientes. Forneça isenções de responsabilidade ou avisos legais esclarecendo que os resumos são gerados por IA e devem ser revisados por profissionais jurídicos.

2. **Lidar com tipos de documentos diversos:** Neste guia, discutimos como extrair texto de PDFs. No mundo real, os documentos podem vir em uma variedade de formatos (PDFs, documentos do Word, arquivos de texto, etc.). Garanta que seu pipeline de extração de dados possa converter todos os formatos de arquivo que você espera receber.

3. **Paralelizar chamadas de API para Claude:** Documentos longos com um grande número de tokens podem levar até um minuto para Claude gerar um resumo. Para grandes coleções de documentos, você pode desejar enviar chamadas de API para Claude em paralelo para que os resumos possam ser concluídos em um período de tempo razoável. Consulte os [limites de taxa](/docs/pt-BR/api/rate-limits#rate-limits) da Anthropic para determinar a quantidade máxima de chamadas de API que podem ser executadas em paralelo.

---

## Melhorar o desempenho

Em cenários complexos, pode ser útil considerar estratégias adicionais para melhorar o desempenho além das [técnicas padrão de engenharia de prompt](/docs/pt-BR/build-with-claude/prompt-engineering/overview). Aqui estão algumas estratégias avançadas:

### Execute meta-resumo para resumir documentos longos

O resumo de documentos legais geralmente envolve o tratamento de documentos longos ou muitos documentos relacionados de uma vez, de modo que você ultrapasse a janela de contexto do Claude. Você pode usar um método de chunking conhecido como meta-resumo para lidar com esse caso de uso. Essa técnica envolve dividir documentos em chunks menores e gerenciáveis e depois processar cada chunk separadamente. Você pode então combinar os resumos de cada chunk para criar um meta-resumo de todo o documento.

Aqui está um exemplo de como executar meta-resumo:

```python
import anthropic

# Initialize the Anthropic client
client = anthropic.Anthropic()

def chunk_text(text, chunk_size=20000):
    return [text[i:i+chunk_size] for i in range(0, len(text), chunk_size)]

def summarize_long_document(text, details_to_extract, model="claude-sonnet-4-5", max_tokens=1000):

    # Format the details to extract to be placed within the prompt's context
    details_to_extract_str = '\n'.join(details_to_extract)

    # Iterate over chunks and summarize each one
    chunk_summaries = [summarize_document(chunk, details_to_extract, model=model, max_tokens=max_tokens) for chunk in chunk_text(text)]
    
    final_summary_prompt = f"""
    
    You are looking at the chunked summaries of multiple documents that are all related. 
    Combine the following summaries of the document from different truthful sources into a coherent overall summary:

    <chunked_summaries>
    {"".join(chunk_summaries)}
    </chunked_summaries>

    Focus on these key aspects:
    {details_to_extract_str})

    Provide the summary in bullet points nested within the XML header for each section. For example:

    <parties involved>
    - Sublessor: [Name]
    // Add more details as needed
    </parties involved>
    
    If any information is not explicitly stated in the document, note it as "Not specified". Do not preamble.
    """

    response = client.messages.create(
        model=model,
        max_tokens=max_tokens,
        system="You are a legal expert that summarizes notes on one document.",
        messages=[
            {"role": "user",  "content": final_summary_prompt},
            {"role": "assistant", "content": "Here is the summary of the sublease agreement: <summary>"}

        ],
        stop_sequences=["</summary>"]
    )
    
    return response.content[0].text

long_summary = summarize_long_document(document_text, details_to_extract)
print(long_summary)
```

A função `summarize_long_document` se baseia na função `summarize_document` anterior dividindo o documento em chunks menores e resumindo cada chunk individualmente.

O código consegue isso aplicando a função `summarize_document` a cada chunk de 20.000 caracteres dentro do documento original. Os resumos individuais são então combinados e um resumo final é criado a partir desses resumos de chunk.

Observe que a função `summarize_long_document` não é estritamente necessária para nosso PDF de exemplo, pois o documento inteiro se encaixa na janela de contexto do Claude. No entanto, torna-se essencial para documentos que excedem a janela de contexto do Claude ou ao resumir múltiplos documentos relacionados juntos. Independentemente disso, essa técnica de meta-resumo geralmente captura detalhes adicionais importantes no resumo final que foram perdidos na abordagem de resumo único anterior.

### Use documentos indexados por resumo para explorar uma grande coleção de documentos

Pesquisar uma coleção de documentos com um LLM geralmente envolve geração aumentada por recuperação (RAG). No entanto, em cenários envolvendo documentos grandes ou quando a recuperação precisa de informações é crucial, uma abordagem RAG básica pode ser insuficiente. Documentos indexados por resumo é uma abordagem RAG avançada que fornece uma maneira mais eficiente de classificar documentos para recuperação, usando menos contexto do que os métodos RAG tradicionais. Nessa abordagem, você primeiro usa Claude para gerar um resumo conciso para cada documento em seu corpus e depois usa Claude para classificar a relevância de cada resumo para a consulta sendo feita. Para mais detalhes sobre essa abordagem, incluindo um exemplo baseado em código, consulte a seção de documentos indexados por resumo no [livro de receitas de resumo](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb).

### Ajuste fino do Claude para aprender com seu conjunto de dados

Outra técnica avançada para melhorar a capacidade do Claude de gerar resumos é o ajuste fino. O ajuste fino envolve treinar Claude em um conjunto de dados personalizado que se alinha especificamente com suas necessidades de resumo de documentos legais, garantindo que Claude se adapte ao seu caso de uso. Aqui está uma visão geral de como executar o ajuste fino:

1. **Identifique erros:** Comece coletando instâncias onde os resumos do Claude ficam aquém - isso pode incluir perda de detalhes legais críticos, incompreensão de contexto ou uso de terminologia legal inadequada.

2. **Organize um conjunto de dados:** Depois de identificar esses problemas, compile um conjunto de dados desses exemplos problemáticos. Este conjunto de dados deve incluir os documentos legais originais ao lado de seus resumos corrigidos, garantindo que Claude aprenda o comportamento desejado.

3. **Execute o ajuste fino:** O ajuste fino envolve retreinar o modelo em seu conjunto de dados organizado para ajustar seus pesos e parâmetros. Este retreinamento ajuda Claude a entender melhor os requisitos específicos de seu domínio legal, melhorando sua capacidade de resumir documentos de acordo com seus padrões.

4. **Melhoria iterativa:** O ajuste fino não é um processo único. Conforme Claude continua a gerar resumos, você pode iterativamente adicionar novos exemplos onde teve desempenho inferior, refinando ainda mais suas capacidades. Com o tempo, esse loop de feedback contínuo resultará em um modelo altamente especializado para suas tarefas de resumo de documentos legais.

<Tip>O ajuste fino está atualmente disponível apenas via Amazon Bedrock. Detalhes adicionais estão disponíveis no [blog de lançamento da AWS](https://aws.amazon.com/blogs/machine-learning/fine-tune-anthropics-claude-3-haiku-in-amazon-bedrock-to-boost-model-accuracy-and-quality/).</Tip>

<CardGroup cols={2}> 
  <Card title="Livro de receitas de resumo" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb">
    Veja um exemplo de código totalmente implementado de como usar Claude para resumir contratos.
  </Card>
  <Card title="Livro de receitas de citações" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb">
    Explore nosso livro de receitas de citações para orientação sobre como garantir precisão e explicabilidade das informações.
  </Card>
</CardGroup>