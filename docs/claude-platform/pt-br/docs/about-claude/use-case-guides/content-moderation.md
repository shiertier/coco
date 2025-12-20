# Moderação de conteúdo

A moderação de conteúdo é um aspecto crítico para manter um ambiente seguro, respeitoso e produtivo em aplicações digitais. Neste guia, discutiremos como Claude pode ser usado para moderar conteúdo dentro da sua aplicação digital.

---

> Visite nosso [cookbook de moderação de conteúdo](https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fmoderation%5Ffilter.ipynb) para ver um exemplo de implementação de moderação de conteúdo usando Claude.

<Tip>Este guia está focado na moderação de conteúdo gerado pelo usuário dentro da sua aplicação. Se você está procurando orientação sobre moderação de interações com Claude, consulte nosso [guia de proteções](/docs/pt-BR/test-and-evaluate/strengthen-guardrails/reduce-hallucinations).</Tip>

## Antes de construir com Claude

### Decida se deve usar Claude para moderação de conteúdo

Aqui estão alguns indicadores-chave de que você deve usar um LLM como Claude em vez de uma abordagem tradicional de ML ou baseada em regras para moderação de conteúdo:

<section title="Você quer uma implementação econômica e rápida">
Métodos tradicionais de ML requerem recursos significativos de engenharia, expertise em ML e custos de infraestrutura. Sistemas de moderação humana incorrem em custos ainda maiores. Com Claude, você pode ter um sistema de moderação sofisticado funcionando em uma fração do tempo por uma fração do preço.
</section>
<section title="Você deseja tanto compreensão semântica quanto decisões rápidas">
Abordagens tradicionais de ML, como modelos bag-of-words ou correspondência simples de padrões, frequentemente lutam para entender o tom, intenção e contexto do conteúdo. Embora sistemas de moderação humana se destaquem na compreensão do significado semântico, eles requerem tempo para que o conteúdo seja revisado. Claude preenche a lacuna combinando compreensão semântica com a capacidade de entregar decisões de moderação rapidamente.
</section>
<section title="Você precisa de decisões de política consistentes">
Ao aproveitar suas capacidades avançadas de raciocínio, Claude pode interpretar e aplicar diretrizes complexas de moderação uniformemente. Esta consistência ajuda a garantir tratamento justo de todo o conteúdo, reduzindo o risco de decisões de moderação inconsistentes ou tendenciosas que podem minar a confiança do usuário.
</section>
<section title="Suas políticas de moderação provavelmente mudarão ou evoluirão ao longo do tempo">
Uma vez que uma abordagem tradicional de ML foi estabelecida, mudá-la é um empreendimento laborioso e intensivo em dados. Por outro lado, conforme suas necessidades de produto ou cliente evoluem, Claude pode facilmente se adaptar a mudanças ou adições às políticas de moderação sem re-rotulagem extensiva de dados de treinamento.
</section>
<section title="Você requer raciocínio interpretável para suas decisões de moderação">
Se você deseja fornecer aos usuários ou reguladores explicações claras por trás das decisões de moderação, Claude pode gerar justificativas detalhadas e coerentes. Esta transparência é importante para construir confiança e garantir responsabilidade nas práticas de moderação de conteúdo.
</section>
<section title="Você precisa de suporte multilíngue sem manter modelos separados">
Abordagens tradicionais de ML tipicamente requerem modelos separados ou processos extensivos de tradução para cada idioma suportado. Moderação humana requer contratar uma força de trabalho fluente em cada idioma suportado. As capacidades multilíngues de Claude permitem que ele classifique tickets em vários idiomas sem a necessidade de modelos separados ou processos extensivos de tradução, simplificando a moderação para bases de clientes globais.
</section>
<section title="Você requer suporte multimodal">
As capacidades multimodais de Claude permitem que ele analise e interprete conteúdo tanto em texto quanto em imagens. Isso o torna uma ferramenta versátil para moderação abrangente de conteúdo em ambientes onde diferentes tipos de mídia precisam ser avaliados juntos.
</section>

<Note>A Anthropic treinou todos os modelos Claude para serem honestos, úteis e inofensivos. Isso pode resultar em Claude moderando conteúdo considerado particularmente perigoso (em linha com nossa [Política de Uso Aceitável](https://www.anthropic.com/legal/aup)), independentemente do prompt usado. Por exemplo, um site adulto que quer permitir que usuários postem conteúdo sexual explícito pode descobrir que Claude ainda sinaliza conteúdo explícito como requerendo moderação, mesmo se especificarem em seu prompt para não moderar conteúdo sexual explícito. Recomendamos revisar nossa AUP antes de construir uma solução de moderação.</Note>

### Gere exemplos de conteúdo para moderar
Antes de desenvolver uma solução de moderação de conteúdo, primeiro crie exemplos de conteúdo que deve ser sinalizado e conteúdo que não deve ser sinalizado. Certifique-se de incluir casos extremos e cenários desafiadores que podem ser difíceis para um sistema de moderação de conteúdo lidar efetivamente. Depois, revise seus exemplos para criar uma lista bem definida de categorias de moderação.
Por exemplo, os exemplos gerados por uma plataforma de mídia social podem incluir o seguinte:

```python
allowed_user_comments = [
    'This movie was great, I really enjoyed it. The main actor really killed it!',
    'I hate Mondays.',
    'It is a great time to invest in gold!'
]

disallowed_user_comments = [
    'Delete this post now or you better hide. I am coming after you and your family.',
    'Stay away from the 5G cellphones!! They are using 5G to control you.',
    'Congratulations! You have won a $1,000 gift card. Click here to claim your prize!'
]

# Sample user comments to test the content moderation
user_comments = allowed_user_comments + disallowed_user_comments

# List of categories considered unsafe for content moderation
unsafe_categories = [
    'Child Exploitation',
    'Conspiracy Theories',
    'Hate',
    'Indiscriminate Weapons', 
    'Intellectual Property',
    'Non-Violent Crimes', 
    'Privacy',
    'Self-Harm',
    'Sex Crimes',
    'Sexual Content',
    'Specialized Advice',
    'Violent Crimes'
]
```

Moderar efetivamente esses exemplos requer uma compreensão nuançada da linguagem. No comentário, `This movie was great, I really enjoyed it. The main actor really killed it!`, o sistema de moderação de conteúdo precisa reconhecer que "killed it" é uma metáfora, não uma indicação de violência real. Conversamente, apesar da falta de menções explícitas de violência, o comentário `Delete this post now or you better hide. I am coming after you and your family.` deve ser sinalizado pelo sistema de moderação de conteúdo.

A lista `unsafe_categories` pode ser customizada para atender suas necessidades específicas. Por exemplo, se você deseja prevenir que menores criem conteúdo em seu site, você poderia adicionar "Underage Posting" à lista.

___

## Como moderar conteúdo usando Claude

### Selecione o modelo Claude correto
Ao selecionar um modelo, é importante considerar o tamanho dos seus dados. Se custos são uma preocupação, um modelo menor como Claude Haiku 3 é uma excelente escolha devido à sua relação custo-benefício. Abaixo está uma estimativa do custo para moderar texto para uma plataforma de mídia social que recebe um bilhão de posts por mês:

* **Tamanho do conteúdo**
    * Posts por mês: 1bi
    * Caracteres por post: 100
    * Total de caracteres: 100bi

* **Tokens estimados**
    * Tokens de entrada: 28,6bi (assumindo 1 token por 3,5 caracteres)
    * Porcentagem de mensagens sinalizadas: 3%
    * Tokens de saída por mensagem sinalizada: 50
    * Total de tokens de saída: 1,5bi

* **Custo estimado Claude Haiku 3**
    * Custo de token de entrada: 2.860 MTok * \$0,25/MTok = \$715
    * Custo de token de saída: 1.500 MTok * \$1,25/MTok = \$1.875
    * Custo mensal: \$715 + \$1.875 = \$2.590

* **Custo estimado Claude Sonnet 4.5**
    * Custo de token de entrada: 2.860 MTok * \$3,00/MTok = \$8.580
    * Custo de token de saída: 1.500 MTok * \$15,00/MTok = \$22.500
    * Custo mensal: \$8.580 + \$22.500 = \$31.080

<Tip>Custos reais podem diferir dessas estimativas. Essas estimativas são baseadas no prompt destacado na seção sobre [processamento em lote](#consider-batch-processing). Tokens de saída podem ser reduzidos ainda mais removendo o campo `explanation` da resposta.</Tip>  

### Construa um prompt forte

Para usar Claude para moderação de conteúdo, Claude deve entender os requisitos de moderação da sua aplicação. Vamos começar escrevendo um prompt que permite definir suas necessidades de moderação:

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def moderate_message(message, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Construct the prompt for Claude, including the message and unsafe categories
    assessment_prompt = f"""
    Determine whether the following message warrants moderation, 
    based on the unsafe categories outlined below.
    
    Message:
    <message>{message}</message>

    Unsafe Categories:
    <categories>
    {unsafe_category_str}
    </categories>

    Respond with ONLY a JSON object, using the format below:
    {{
    "violation": <Boolean field denoting whether the message should be moderated>,
    "categories": [Comma-separated list of violated categories],
    "explanation": [Optional. Only include if there is a violation.]
    }}"""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the violation status from the assessment
    contains_violation = assessment['violation']
    
    # If there's a violation, get the categories and explanation; otherwise, use empty defaults
    violated_categories = assessment.get('categories', []) if contains_violation else []
    explanation = assessment.get('explanation') if contains_violation else None
    
    return contains_violation, violated_categories, explanation

# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    violation, violated_categories, explanation = moderate_message(comment, unsafe_categories)
    
    if violation:
        print(f"Violated Categories: {', '.join(violated_categories)}")
        print(f"Explanation: {explanation}")
    else:
        print("No issues detected.")
```

Neste exemplo, a função `moderate_message` contém um prompt de avaliação que inclui as categorias de conteúdo inseguro e a mensagem que desejamos avaliar. O prompt pede ao Claude para avaliar se a mensagem deve ser moderada, baseado nas categorias inseguras que definimos.

A avaliação do modelo é então analisada para determinar se há uma violação. Se há uma violação, Claude também retorna uma lista de categorias violadas, bem como uma explicação de por que a mensagem é insegura.

### Avalie seu prompt

Moderação de conteúdo é um problema de classificação. Assim, você pode usar as mesmas técnicas descritas em nosso [cookbook de classificação](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/classification/guide.ipynb) para determinar a precisão do seu sistema de moderação de conteúdo.

Uma consideração adicional é que em vez de tratar moderação de conteúdo como um problema de classificação binária, você pode criar múltiplas categorias para representar vários níveis de risco. Criar múltiplos níveis de risco permite ajustar a agressividade da sua moderação. Por exemplo, você pode querer bloquear automaticamente consultas de usuário que são consideradas de alto risco, enquanto usuários com muitas consultas de risco médio são sinalizados para revisão humana.

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def assess_risk_level(message, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Construct the prompt for Claude, including the message, unsafe categories, and risk level definitions
    assessment_prompt = f"""
    Assess the risk level of the following message warrants moderation, 
    based on the unsafe categories listed below.
    
Message:
<message>{message}</message>

Unsafe Categories:
<categories>
{unsafe_category_str}
</categories>

Assign a risk level based on your confidence that the user's message should be moderated 
based on the defined unsafe categories:

0 - No risk
1 - Low risk
2 - Medium risk
3 - High risk

Respond with ONLY a JSON object, using the format below:
{{
  "risk_level": <Numerical field denoting the risk level>,
  "categories": [Comma-separated list of violated categories],
  "explanation": <Optional. Only include if risk level is greater than 0>
}}"""

    # Send the request to Claude for risk assessment
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the risk level, violated categories, and explanation from the assessment
    risk_level = assessment["risk_level"]
    violated_categories = assessment["categories"]
    explanation = assessment.get("explanation")
    
    return risk_level, violated_categories, explanation

# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    risk_level, violated_categories, explanation = assess_risk_level(comment, unsafe_categories)
    
    print(f"Risk Level: {risk_level}")
    if violated_categories:
        print(f"Violated Categories: {', '.join(violated_categories)}")
    if explanation:
        print(f"Explanation: {explanation}")
```

Este código implementa uma função `assess_risk_level` que usa Claude para avaliar o nível de risco de uma mensagem. A função aceita uma mensagem e uma lista de categorias inseguras como entradas.

Dentro da função, um prompt é gerado para Claude, incluindo a mensagem a ser avaliada, as categorias inseguras e instruções específicas para avaliar o nível de risco. O prompt instrui Claude a responder com um objeto JSON que inclui o nível de risco, as categorias violadas e uma explicação opcional.

Esta abordagem permite moderação flexível de conteúdo atribuindo níveis de risco. Pode ser perfeitamente integrada em um sistema maior para automatizar filtragem de conteúdo ou sinalizar comentários para revisão humana baseado em seu nível de risco avaliado. Por exemplo, ao executar este código, o comentário `Delete this post now or you better hide. I am coming after you and your family.` é identificado como alto risco devido à sua ameaça perigosa. Conversamente, o comentário `Stay away from the 5G cellphones!! They are using 5G to control you.` é categorizado como risco médio.

### Implante seu prompt

Uma vez que você está confiante na qualidade da sua solução, é hora de implantá-la em produção. Aqui estão algumas melhores práticas a seguir ao usar moderação de conteúdo em produção:

1. **Forneça feedback claro aos usuários:** Quando entrada do usuário é bloqueada ou uma resposta é sinalizada devido à moderação de conteúdo, forneça feedback informativo e construtivo para ajudar usuários a entender por que sua mensagem foi sinalizada e como eles podem reformulá-la apropriadamente. Nos exemplos de código acima, isso é feito através da tag `explanation` na resposta de Claude.

2. **Analise conteúdo moderado:** Mantenha registro dos tipos de conteúdo sendo sinalizados pelo seu sistema de moderação para identificar tendências e áreas potenciais para melhoria.

3. **Avalie e melhore continuamente:** Avalie regularmente o desempenho do seu sistema de moderação de conteúdo usando métricas como rastreamento de precisão e recall. Use esses dados para refinar iterativamente seus prompts de moderação, palavras-chave e critérios de avaliação.

___

## Melhore o desempenho

Em cenários complexos, pode ser útil considerar estratégias adicionais para melhorar o desempenho além das [técnicas padrão de engenharia de prompt](/docs/pt-BR/build-with-claude/prompt-engineering/overview). Aqui estão algumas estratégias avançadas:

### Defina tópicos e forneça exemplos

Além de listar as categorias inseguras no prompt, melhorias adicionais podem ser feitas fornecendo definições e frases relacionadas a cada categoria.

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

# Dictionary of categories considered unsafe for content moderation, with their definitions
unsafe_category_definitions = {
    'Child Exploitation': 'Content that depicts child nudity or that enables, encourages, excuses, or depicts the sexual abuse of children.',
    'Conspiracy Theories': """Content that promotes or endorses unfounded, false, or misleading theories about events, situations, or individuals, 
        which can incite harm or undermine public trust in institutions or facts.""",
    'Hate': """Content that is hateful toward people on the basis of their protected characteristics 
        (race, color, ethnicity, national origin, disability, religious affiliation, caste, sexual orientation, sex, gender identity and serious disease), 
        or content that perpetuates negative stereotypes.""",
    'Indiscriminate Weapons': 'Content that enables the creation of weapons of mass destruction.', 
    'Intellectual Property': 'Content that violates the intellectual property rights of third parties.',
    'Non-Violent Crimes': 'Content that enables, encourages, or excuses the commission of non-violent crimes.', 
    'Privacy': 'Content that contains sensitive, personal information about private individuals.',
    'Self-Harm': 'Content that enables, encourages, or excuses acts of intentional self-harm.',
    'Sex Crimes': 'Content that enables, encourages, or excuses the commission of sex crimes.',
    'Sexual Content': 'Content that contains erotic descriptions or explicit depictions of sex acts.',
    'Specialized Advice': """Content that contains financial, medical, or legal advice. 
        Financial advice includes guidance on investments, stocks, bonds, or any financial planning.""",
    'Violent Crimes': 'Content that enables, encourages, or excuses the commission of violent crimes.', 
}


def moderate_message_with_definitions(message, unsafe_category_definitions):
    # Format unsafe categories string, with each category and its definition on a new line
    unsafe_category_str = '\n'.join(f'{category}: {definition}' 
        for category, definition in unsafe_category_definitions.items()
    )
    
    # Construct the prompt for Claude, including the message and unsafe categories
    assessment_prompt = f"""Determine whether the following message warrants moderation, based on the unsafe categories outlined below.

Message:
<message>{message}</message>

Unsafe Categories and Their Definitions:
<categories>
{unsafe_category_str}
</categories>

It's important that you remember all unsafe categories and their definitions.

Respond with ONLY a JSON object, using the format below:
{{
  "violation": <Boolean field denoting whether the message should be moderated>,
  "categories": [Comma-separated list of violated categories],
  "explanation": [Optional. Only include if there is a violation.]
}}"""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the violation status from the assessment
    contains_violation = assessment['violation']
    
    # If there's a violation, get the categories and explanation; otherwise, use empty defaults
    violated_categories = assessment.get('categories', []) if contains_violation else []
    explanation = assessment.get('explanation') if contains_violation else None
    
    return contains_violation, violated_categories, explanation


# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    violation, violated_categories, explanation = moderate_message_with_definitions(comment, unsafe_category_definitions)
    
    if violation:
        print(f"Violated Categories: {', '.join(violated_categories)}")
        print(f"Explanation: {explanation}")
    else:
        print("No issues detected.")
```

A função `moderate_message_with_definitions` expande a função anterior `moderate_message` permitindo que cada categoria insegura seja pareada com uma definição detalhada. Isso ocorre no código substituindo a lista `unsafe_categories` da função original por um dicionário `unsafe_category_definitions`. Este dicionário mapeia cada categoria insegura para sua definição correspondente. Tanto os nomes das categorias quanto suas definições são incluídos no prompt.

Notavelmente, a definição para a categoria `Specialized Advice` agora especifica os tipos de conselho financeiro que devem ser proibidos. Como resultado, o comentário `It's a great time to invest in gold!`, que anteriormente passou na avaliação `moderate_message`, agora dispara uma violação.

### Considere processamento em lote

Para reduzir cus tos em situações onde moderação em tempo real não é necessária, considere moderar mensagens em lotes. Inclua múltiplas mensagens dentro do contexto do prompt, e peça ao Claude para avaliar quais mensagens devem ser moderadas.

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def batch_moderate_messages(messages, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Format messages string, with each message wrapped in XML-like tags and given an ID
    messages_str = '\n'.join([f'<message id={idx}>{msg}</message>' for idx, msg in enumerate(messages)])
    
    # Construct the prompt for Claude, including the messages and unsafe categories
    assessment_prompt = f"""Determine the messages to moderate, based on the unsafe categories outlined below.

Messages:
<messages>
{messages_str}
</messages>

Unsafe categories and their definitions:
<categories>
{unsafe_category_str}
</categories>

Respond with ONLY a JSON object, using the format below:
{{
  "violations": [
    {{
      "id": <message id>,
      "categories": [list of violated categories],
      "explanation": <Explanation of why there's a violation>
    }},
    ...
  ]
}}

Important Notes:
- Remember to analyze every message for a violation.
- Select any number of violations that reasonably apply."""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=2048,  # Increased max token count to handle batches
        temperature=0,    # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    return assessment


# Process the batch of comments and get the response
response_obj = batch_moderate_messages(user_comments, unsafe_categories)

# Print the results for each detected violation
for violation in response_obj['violations']:
    print(f"""Comment: {user_comments[violation['id']]}
Violated Categories: {', '.join(violation['categories'])}
Explanation: {violation['explanation']}
""")
```
Neste exemplo, a função `batch_moderate_messages` lida com a moderação de um lote inteiro de mensagens com uma única chamada da API Claude.
Dentro da função, um prompt é criado que inclui a lista de mensagens para avaliar, as categorias de conteúdo inseguro definidas e suas descrições. O prompt direciona Claude a retornar um objeto JSON listando todas as mensagens que contêm violações. Cada mensagem na resposta é identificada por seu id, que corresponde à posição da mensagem na lista de entrada.
Tenha em mente que encontrar o tamanho de lote ótimo para suas necessidades específicas pode requerer alguma experimentação. Embora tamanhos de lote maiores possam reduzir custos, eles também podem levar a uma ligeira diminuição na qualidade. Adicionalmente, você pode precisar aumentar o parâmetro `max_tokens` na chamada da API Claude para acomodar respostas mais longas. Para detalhes sobre o número máximo de tokens que seu modelo escolhido pode produzir, consulte a [página de comparação de modelos](/docs/pt-BR/about-claude/models#model-comparison-table).

<CardGroup cols={2}> 
  <Card title="Cookbook de moderação de conteúdo" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fmoderation%5Ffilter.ipynb">
    Veja um exemplo totalmente implementado baseado em código de como usar Claude para moderação de conteúdo.
  </Card>
  <Card title="Guia de proteções" icon="link" href="/docs/pt-BR/test-and-evaluate/strengthen-guardrails/reduce-hallucinations">
    Explore nosso guia de proteções para técnicas de moderação de interações com Claude.
  </Card>
</CardGroup>