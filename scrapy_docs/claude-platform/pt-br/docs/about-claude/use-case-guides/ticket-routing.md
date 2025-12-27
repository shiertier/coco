# Roteamento de tickets

Este guia apresenta como aproveitar as capacidades avançadas de compreensão de linguagem natural do Claude para classificar tickets de suporte ao cliente em escala com base na intenção do cliente, urgência, priorização, perfil do cliente e muito mais.

---

## Defina se deve usar Claude para roteamento de tickets

Aqui estão alguns indicadores-chave de que você deve usar um LLM como Claude em vez de abordagens tradicionais de ML para sua tarefa de classificação:

    <section title="Você tem dados de treinamento rotulados limitados disponíveis">

        Os processos tradicionais de ML requerem conjuntos de dados rotulados massivos. O modelo pré-treinado do Claude pode classificar efetivamente tickets com apenas algumas dezenas de exemplos rotulados, reduzindo significativamente o tempo de preparação de dados e os custos.
    
</section>
    <section title="Suas categorias de classificação provavelmente mudarão ou evoluirão ao longo do tempo">

        Uma vez que uma abordagem tradicional de ML foi estabelecida, alterá-la é uma tarefa trabalhosa e intensiva em dados. Por outro lado, conforme seu produto ou necessidades dos clientes evoluem, Claude pode se adaptar facilmente a mudanças nas definições de classe ou novas classes sem relabelagem extensiva de dados de treinamento.
    
</section>
    <section title="Você precisa lidar com entradas de texto complexas e não estruturadas">

        Os modelos tradicionais de ML frequentemente têm dificuldade com dados não estruturados e requerem engenharia de recursos extensiva. A compreensão avançada de linguagem do Claude permite classificação precisa com base no conteúdo e contexto, em vez de depender de estruturas ontológicas rígidas.
    
</section>
    <section title="Suas regras de classificação são baseadas em compreensão semântica">

        As abordagens tradicionais de ML frequentemente dependem de modelos bag-of-words ou correspondência de padrões simples. Claude se destaca em compreender e aplicar regras subjacentes quando as classes são definidas por condições em vez de exemplos.
    
</section>
    <section title="Você requer raciocínio interpretável para decisões de classificação">

        Muitos modelos tradicionais de ML fornecem pouca visão sobre seu processo de tomada de decisão. Claude pode fornecer explicações legíveis por humanos para suas decisões de classificação, construindo confiança no sistema de automação e facilitando fácil adaptação se necessário.
    
</section>
    <section title="Você quer lidar com casos extremos e tickets ambíguos de forma mais eficaz">

        Os sistemas tradicionais de ML frequentemente têm dificuldade com outliers e entradas ambíguas, frequentemente os classificando incorretamente ou padronizando para uma categoria genérica. As capacidades de processamento de linguagem natural do Claude permitem que ele interprete melhor o contexto e a nuance em tickets de suporte, potencialmente reduzindo o número de tickets mal roteados ou não classificados que requerem intervenção manual.
    
</section>
    <section title="Você precisa de suporte multilíngue sem manter modelos separados">

        As abordagens tradicionais de ML normalmente requerem modelos separados ou processos de tradução extensivos para cada idioma suportado. As capacidades multilíngues do Claude permitem que ele classifique tickets em vários idiomas sem a necessidade de modelos separados ou processos de tradução extensivos, simplificando o suporte para bases de clientes globais.
    
</section>

***

##  Construa e implante seu fluxo de trabalho de suporte LLM

### Entenda sua abordagem de suporte atual
Antes de mergulhar na automação, é crucial entender seu sistema de ticketing existente. Comece investigando como sua equipe de suporte atualmente lida com o roteamento de tickets.

Considere perguntas como:
* Quais critérios são usados para determinar qual SLA/oferta de serviço é aplicada?
* O roteamento de tickets é usado para determinar qual nível de suporte ou especialista em produto um ticket vai?
* Existem regras ou fluxos de trabalho automatizados já em vigor? Em quais casos eles falham?
* Como os casos extremos ou tickets ambíguos são tratados?
* Como a equipe prioriza os tickets?

Quanto mais você souber sobre como os humanos lidam com certos casos, melhor você será capaz de trabalhar com Claude para fazer a tarefa.

### Defina categorias de intenção do usuário
Uma lista bem definida de categorias de intenção do usuário é crucial para classificação precisa de tickets de suporte com Claude. A capacidade do Claude de rotear tickets efetivamente dentro de seu sistema é diretamente proporcional ao quão bem definidas estão as categorias do seu sistema.

Aqui estão algumas categorias e subcategorias de intenção do usuário de exemplo.

    <section title="Problema técnico">

        * Problema de hardware
        * Bug de software
        * Problema de compatibilidade
        * Problema de desempenho
    
</section>
    <section title="Gerenciamento de conta">

        * Redefinição de senha
        * Problemas de acesso à conta
        * Consultas de faturamento
        * Mudanças de assinatura
    
</section>
    <section title="Informações do produto">

        * Consultas de recursos
        * Perguntas sobre compatibilidade de produtos
        * Informações de preços
        * Consultas de disponibilidade
    
</section>
    <section title="Orientação do usuário">

        * Perguntas como fazer
        * Assistência de uso de recursos
        * Conselhos de melhores práticas
        * Orientação de solução de problemas
    
</section>
    <section title="Feedback">

        * Relatórios de bugs
        * Solicitações de recursos
        * Feedback geral ou sugestões
        * Reclamações
    
</section>
    <section title="Relacionado a pedidos">

        * Consultas de status de pedido
        * Informações de envio
        * Devoluções e trocas
        * Modificações de pedido
    
</section>
    <section title="Solicitação de serviço">

        * Assistência de instalação
        * Solicitações de atualização
        * Agendamento de manutenção
        * Cancelamento de serviço
    
</section>
    <section title="Preocupações de segurança">

        * Consultas de privacidade de dados
        * Relatórios de atividade suspeita
        * Assistência de recursos de segurança
    
</section>
    <section title="Conformidade e legal">

        * Perguntas sobre conformidade regulatória
        * Consultas de termos de serviço
        * Solicitações de documentação legal
    
</section>
    <section title="Suporte de emergência">

        * Falhas críticas do sistema
        * Problemas de segurança urgentes
        * Problemas sensíveis ao tempo
    
</section>
    <section title="Treinamento e educação">

        * Solicitações de treinamento de produto
        * Consultas de documentação
        * Informações de webinar ou workshop
    
</section>
    <section title="Integração e API">

        * Assistência de integração
        * Perguntas sobre uso de API
        * Consultas de compatibilidade com terceiros
    
</section>

Além da intenção, o roteamento e priorização de tickets também podem ser influenciados por outros fatores, como urgência, tipo de cliente, SLAs ou idioma. Certifique-se de considerar outros critérios de roteamento ao construir seu sistema de roteamento automatizado.

### Estabeleça critérios de sucesso

Trabalhe com sua equipe de suporte para [definir critérios de sucesso claros](/docs/pt-BR/test-and-evaluate/define-success) com benchmarks, limites e objetivos mensuráveis.

Aqui estão alguns critérios e benchmarks padrão ao usar LLMs para roteamento de tickets de suporte:

    <section title="Consistência de classificação">

        Esta métrica avalia como Claude classifica consistentemente tickets semelhantes ao longo do tempo. É crucial para manter a confiabilidade do roteamento. Meça isso testando periodicamente o modelo com um conjunto de entradas padronizadas e visando uma taxa de consistência de 95% ou superior.
    
</section>
    <section title="Velocidade de adaptação">

        Isso mede com que rapidez Claude pode se adaptar a novas categorias ou padrões de tickets em mudança. Teste isso introduzindo novos tipos de tickets e medindo o tempo que leva para o modelo alcançar precisão satisfatória (por exemplo, >90%) nessas novas categorias. Vise adaptação dentro de 50-100 tickets de amostra.
    
</section>
    <section title="Tratamento multilíngue">

        Isso avalia a capacidade do Claude de rotear com precisão tickets em vários idiomas. Meça a precisão de roteamento em diferentes idiomas, visando não mais que uma queda de 5-10% na precisão para idiomas não primários.
    
</section>
    <section title="Tratamento de casos extremos">

        Isso avalia o desempenho do Claude em tickets incomuns ou complexos. Crie um conjunto de testes de casos extremos e meça a precisão de roteamento, visando pelo menos 80% de precisão nessas entradas desafiadoras.
    
</section>
    <section title="Mitigação de viés">

        Isso mede a justiça do Claude no roteamento entre diferentes dados demográficos de clientes. Audite regularmente as decisões de roteamento para possíveis vieses, visando consistência na precisão de roteamento (dentro de 2-3%) em todos os grupos de clientes.
    
</section>
    <section title="Eficiência de prompt">

        Em situações onde minimizar a contagem de tokens é crucial, este critério avalia o desempenho do Claude com contexto mínimo. Meça a precisão de roteamento com quantidades variáveis de contexto fornecido, visando 90%+ de precisão com apenas o título do ticket e uma breve descrição.
    
</section>
    <section title="Pontuação de explicabilidade">

        Isso avalia a qualidade e relevância das explicações do Claude para suas decisões de roteamento. Avaliadores humanos podem pontuar explicações em uma escala (por exemplo, 1-5), com o objetivo de alcançar uma pontuação média de 4 ou superior.
    
</section>

Aqui estão alguns critérios de sucesso comuns que podem ser úteis independentemente de um LLM ser usado:

    <section title="Precisão de roteamento">

        A precisão de roteamento mede com que frequência os tickets são atribuídos corretamente à equipe ou indivíduo apropriado na primeira tentativa. Isso é normalmente medido como uma porcentagem de tickets corretamente roteados do total de tickets. Os benchmarks da indústria geralmente visam 90-95% de precisão, embora isso possa variar com base na complexidade da estrutura de suporte.
    
</section>
    <section title="Tempo até atribuição">

        Esta métrica rastreia com que rapidez os tickets são atribuídos após serem enviados. Tempos de atribuição mais rápidos geralmente levam a resoluções mais rápidas e satisfação do cliente melhorada. Os sistemas de melhor classe geralmente alcançam tempos de atribuição médios de menos de 5 minutos, com muitos visando roteamento quase instantâneo (que é possível com implementações de LLM).
    
</section>
    <section title="Taxa de reroteamento">

        A taxa de reroteamento indica com que frequência os tickets precisam ser reatribuídos após o roteamento inicial. Uma taxa mais baixa sugere roteamento inicial mais preciso. Vise uma taxa de reroteamento abaixo de 10%, com sistemas de melhor desempenho alcançando taxas tão baixas quanto 5% ou menos.
    
</section>
    <section title="Taxa de resolução no primeiro contato">

        Isso mede a porcentagem de tickets resolvidos durante a primeira interação com o cliente. Taxas mais altas indicam roteamento eficiente e equipes de suporte bem preparadas. Os benchmarks da indústria normalmente variam de 70-75%, com os melhores desempenhos alcançando taxas de 80% ou superior.
    
</section>
    <section title="Tempo médio de tratamento">

        O tempo médio de tratamento mede quanto tempo leva para resolver um ticket do início ao fim. O roteamento eficiente pode reduzir significativamente esse tempo. Os benchmarks variam amplamente por indústria e complexidade, mas muitas organizações visam manter o tempo médio de tratamento abaixo de 24 horas para problemas não críticos.
    
</section>
    <section title="Pontuações de satisfação do cliente">

        Frequentemente medidas através de pesquisas pós-interação, essas pontuações refletem a felicidade geral do cliente com o processo de suporte. O roteamento eficaz contribui para maior satisfação. Vise pontuações CSAT de 90% ou superior, com os melhores desempenhos frequentemente alcançando taxas de satisfação de 95%+.
    
</section>
    <section title="Taxa de escalação">

        Isso mede com que frequência os tickets precisam ser escalados para níveis mais altos de suporte. Taxas de escalação mais baixas geralmente indicam roteamento inicial mais preciso. Esforce-se por uma taxa de escalação abaixo de 20%, com sistemas de melhor classe alcançando taxas de 10% ou menos.
    
</section>
    <section title="Produtividade do agente">

        Esta métrica analisa quantos tickets os agentes podem lidar efetivamente após implementar a solução de roteamento. O roteamento melhorado deve aumentar a produtividade. Meça isso rastreando tickets resolvidos por agente por dia ou hora, visando uma melhoria de 10-20% após implementar um novo sistema de roteamento.
    
</section>
    <section title="Taxa de deflexão de autoatendimento">

        Isso mede a porcentagem de tickets potenciais resolvidos através de opções de autoatendimento antes de entrar no sistema de roteamento. Taxas mais altas indicam triagem pré-roteamento eficaz. Vise uma taxa de deflexão de 20-30%, com os melhores desempenhos alcançando taxas de 40% ou superior.
    
</section>
    <section title="Custo por ticket">

        Esta métrica calcula o custo médio para resolver cada ticket de suporte. O roteamento eficiente deve ajudar a reduzir esse custo ao longo do tempo. Embora os benchmarks variem amplamente, muitas organizações visam reduzir o custo por ticket em 10-15% após implementar um sistema de roteamento melhorado.
    
</section>

### Escolha o modelo Claude certo

A escolha do modelo depende dos trade-offs entre custo, precisão e tempo de resposta.

Muitos clientes descobriram que `claude-haiku-4-5-20251001` é um modelo ideal para roteamento de tickets, pois é o modelo mais rápido e mais econômico da família Claude 4, enquanto ainda oferece excelentes resultados. Se seu problema de classificação requer expertise profunda em assuntos ou um grande volume de categorias de intenção com raciocínio complexo, você pode optar pelo [modelo Sonnet maior](/docs/pt-BR/about-claude/models).

### Construa um prompt forte

O roteamento de tickets é um tipo de tarefa de classificação. Claude analisa o conteúdo de um ticket de suporte e o classifica em categorias predefinidas com base no tipo de problema, urgência, expertise necessária ou outros fatores relevantes.

Vamos escrever um prompt de classificação de tickets. Nosso prompt inicial deve conter o conteúdo da solicitação do usuário e retornar tanto o raciocínio quanto a intenção.

<Tip>
Tente o [gerador de prompt](/docs/pt-BR/prompt-generator) no [Claude Console](/login) para ter Claude escrever um primeiro rascunho para você.
</Tip>

Aqui está um exemplo de prompt de classificação de roteamento de tickets:
```python
def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. Your task is to analyze customer support requests and output the appropriate classification intent for each request, along with your reasoning. 

        Here is the customer support request you need to classify:

        <request>{ticket_contents}</request>

        Please carefully analyze the above request to determine the customer's core intent and needs. Consider what the customer is asking for has concerns about.

        First, write out your reasoning and analysis of how to classify this request inside <reasoning> tags.

        Then, output the appropriate classification label for the request inside a <intent> tag. The valid intents are:
        <intents>
        <intent>Support, Feedback, Complaint</intent>
        <intent>Order Tracking</intent>
        <intent>Refund/Exchange</intent>
        </intents>

        A request may have ONLY ONE applicable intent. Only include the intent that is most applicable to the request.

        As an example, consider the following request:
        <request>Hello! I had high-speed fiber internet installed on Saturday and my installer, Kevin, was absolutely fantastic! Where can I send my positive review? Thanks for your help!</request>

        Here is an example of how your output should be formatted (for the above example request):
        <reasoning>The user seeks information in order to leave positive feedback.</reasoning>
        <intent>Support, Feedback, Complaint</intent>

        Here are a few more examples:
        <examples>
        <example 2>
        Example 2 Input:
        <request>I wanted to write and personally thank you for the compassion you showed towards my family during my father's funeral this past weekend. Your staff was so considerate and helpful throughout this whole process; it really took a load off our shoulders. The visitation brochures were beautiful. We'll never forget the kindness you showed us and we are so appreciative of how smoothly the proceedings went. Thank you, again, Amarantha Hill on behalf of the Hill Family.</request>

        Example 2 Output:
        <reasoning>User leaves a positive review of their experience.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 2>
        <example 3>

        ...

        </example 8>
        <example 9>
        Example 9 Input:
        <request>Your website keeps sending ad-popups that block the entire screen. It took me twenty minutes just to finally find the phone number to call and complain. How can I possibly access my account information with all of these popups? Can you access my account for me, since your website is broken? I need to know what the address is on file.</request>

        Example 9 Output:
        <reasoning>The user requests help accessing their web account information.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 9>

        Remember to always include your classification reasoning before your actual intent output. The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
```

Vamos decompor os componentes-chave deste prompt:
* Usamos f-strings do Python para criar o modelo de prompt, permitindo que `ticket_contents` seja inserido nas tags `<request>`.
* Damos ao Claude um papel claramente definido como um sistema de classificação que analisa cuidadosamente o conteúdo do ticket para determinar a intenção e necessidades principais do cliente.
* Instruímos Claude sobre formatação apropriada de saída, neste caso para fornecer seu raciocínio e análise dentro de tags `<reasoning>`, seguidas pelo rótulo de classificação apropriado dentro de tags `<intent>`.
* Especificamos as categorias de intenção válidas: "Support, Feedback, Complaint", "Order Tracking" e "Refund/Exchange".
* Incluímos alguns exemplos (também conhecidos como few-shot prompting) para ilustrar como a saída deve ser formatada, o que melhora a precisão e consistência.

A razão pela qual queremos que Claude divida sua resposta em várias seções de tags XML é para que possamos usar expressões regulares para extrair separadamente o raciocínio e a intenção da saída. Isso nos permite criar próximos passos direcionados no fluxo de trabalho de roteamento de tickets, como usar apenas a intenção para decidir para qual pessoa rotear o ticket.

### Implante seu prompt

É difícil saber como seu prompt funciona bem sem implantá-lo em um ambiente de produção de teste e [executar avaliações](/docs/pt-BR/test-and-evaluate/develop-tests).

Vamos construir a estrutura de implantação. Comece definindo a assinatura do método para envolver nossa chamada ao Claude. Pegaremos o método que já começamos a escrever, que tem `ticket_contents` como entrada, e agora retornaremos uma tupla de `reasoning` e `intent` como saída. Se você tiver uma automação existente usando ML tradicional, você vai querer seguir essa assinatura de método.

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ... The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
    # Send the prompt to the API to classify the support request.
    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
        stream=False,
    )
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

    return reasoning, intent
```

Este código:
* Importa a biblioteca Anthropic e cria uma instância de cliente usando sua chave de API.
* Define uma função `classify_support_request` que recebe uma string `ticket_contents`.
* Envia `ticket_contents` para Claude para classificação usando `classification_prompt`
* Retorna o `reasoning` e `intent` do modelo extraído da resposta.

Como precisamos esperar que todo o texto de raciocínio e intenção seja gerado antes de analisar, definimos `stream=False` (o padrão).

***

## Avalie seu prompt

O prompting frequentemente requer testes e otimização para estar pronto para produção. Para determinar a prontidão de sua solução, avalie o desempenho com base nos critérios de sucesso e limites que você estabeleceu anteriormente.

Para executar sua avaliação, você precisará de casos de teste para executá-la. O resto deste guia assume que você já [desenvolveu seus casos de teste](/docs/pt-BR/test-and-evaluate/develop-tests).

### Construa uma função de avaliação

Nossa avaliação de exemplo para este guia mede o desempenho do Claude ao longo de três métricas-chave:
* Precisão
* Custo por classificação

Você pode precisar avaliar Claude em outros eixos dependendo de quais fatores são importantes para você.

Para avaliar isso, primeiro temos que modificar o script que escrevemos e adicionar uma função para comparar a intenção prevista com a intenção real e calcular a porcentagem de previsões corretas. Também temos que adicionar funcionalidade de cálculo de custo e medição de tempo.

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(request, actual_intent):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ...The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """

    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
    )
    usage = message.usage  # Get the usage statistics for the API call for how many input and output tokens were used.
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

      # Check if the model's prediction is correct.
    correct = actual_intent.strip() == intent.strip()

    # Return the reasoning, intent, correct, and usage.
    return reasoning, intent, correct, usage
```

Vamos decompor as edições que fizemos:
* Adicionamos `actual_intent` de nossos casos de teste ao método `classify_support_request` e configuramos uma comparação para avaliar se a classificação de intenção do Claude corresponde à nossa classificação de intenção dourada.
* Extraímos estatísticas de uso para a chamada de API para calcular o custo com base nos tokens de entrada e saída usados

### Execute sua avaliação

Uma avaliação adequada requer limites e benchmarks claros para determinar o que é um bom resultado. O script acima nos dará os valores de tempo de execução para precisão, tempo de resposta e custo por classificação, mas ainda precisaríamos de limites claramente estabelecidos. Por exemplo:
* **Precisão:** 95% (de 100 testes)
* **Custo por classificação:** Redução de 50% em média (em 100 testes) do método de roteamento atual

Ter esses limites permite que você diga rápida e facilmente em escala, e com empirismo imparcial, qual método é melhor para você e quais mudanças podem precisar ser feitas para se adequar melhor aos seus requisitos.

***

## Melhore o desempenho

Em cenários complexos, pode ser útil considerar estratégias adicionais para melhorar o desempenho além das [técnicas padrão de engenharia de prompt](/docs/pt-BR/build-with-claude/prompt-engineering/overview) & [estratégias de implementação de guardrails](/docs/pt-BR/test-and-evaluate/strengthen-guardrails/reduce-hallucinations). Aqui estão alguns cenários comuns:

### Use uma hierarquia taxonômica para casos com 20+ categorias de intenção

Conforme o número de classes cresce, o número de exemplos necessários também se expande, potencialmente tornando o prompt incontrolável. Como alternativa, você pode considerar implementar um sistema de classificação hierárquica usando uma mistura de classificadores.
1. Organize suas intenções em uma estrutura de árvore taxonômica.
2. Crie uma série de classificadores em cada nível da árvore, permitindo uma abordagem de roteamento em cascata.

Por exemplo, você pode ter um classificador de nível superior que categoriza amplamente os tickets em "Problemas Técnicos", "Perguntas de Faturamento" e "Consultas Gerais". Cada uma dessas categorias pode então ter seu próprio subclassificador para refinar ainda mais a classificação.

![](/docs/images/ticket-hierarchy.png)

* **Prós - maior nuance e precisão:** Você pode criar prompts diferentes para cada caminho pai, permitindo classificação mais direcionada e específica do contexto. Isso pode levar a melhor precisão e tratamento mais nuançado de solicitações de clientes.

* **Contras - latência aumentada:** Esteja ciente de que múltiplos classificadores podem levar a latência aumentada, e recomendamos implementar essa abordagem com nosso modelo mais rápido, Haiku.

### Use bancos de dados vetoriais e busca de similaridade de recuperação para lidar com tickets altamente variáveis

Apesar de fornecer exemplos ser a forma mais eficaz de melhorar o desempenho, se as solicitações de suporte forem altamente variáveis, pode ser difícil incluir exemplos suficientes em um único prompt.

Neste cenário, você pode empregar um banco de dados vetorial para fazer buscas de similaridade a partir de um conjunto de dados de exemplos e recuperar os exemplos mais relevantes para uma determinada consulta.

Esta abordagem, descrita em detalhes em nossa [receita de classificação](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb), foi mostrada para melhorar o desempenho de 71% de precisão para 93% de precisão.

### Considere especificamente os casos extremos esperados

Aqui estão alguns cenários onde Claude pode classificar incorretamente tickets (pode haver outros que são únicos para sua situação). Nestes cenários, considere fornecer instruções explícitas ou exemplos no prompt de como Claude deve lidar com o caso extremo:

    <section title="Clientes fazem solicitações implícitas">

        Os clientes frequentemente expressam necessidades indiretamente. Por exemplo, "Estou esperando meu pacote há mais de duas semanas" pode ser uma solicitação indireta de status do pedido.
        * **Solução:** Forneça ao Claude alguns exemplos reais de clientes desses tipos de solicitações, junto com qual é a intenção subjacente. Você pode obter resultados ainda melhores se incluir uma rationale de classificação para intenções de tickets particularmente nuançadas, para que Claude possa generalizar melhor a lógica para outros tickets.
    
</section>
    <section title="Claude prioriza emoção sobre intenção">

        Quando os clientes expressam insatisfação, Claude pode priorizar abordar a emoção em vez de resolver o problema subjacente.
        * **Solução:** Forneça ao Claude direções sobre quando priorizar o sentimento do cliente ou não. Pode ser algo tão simples quanto "Ignore todas as emoções do cliente. Concentre-se apenas em analisar a intenção da solicitação do cliente e que informações o cliente pode estar pedindo."
    
</section>
    <section title="Múltiplos problemas causam confusão na priorização de problemas">

        Quando os clientes apresentam múltiplos problemas em uma única interação, Claude pode ter dificuldade em identificar a preocupação principal.
        * **Solução:** Esclareça a priorização de intenções para que Claude possa melhor classificar as intenções extraídas e identificar a preocupação principal.
    
</section>

***

## Integre Claude em seu fluxo de trabalho de suporte maior

A integração adequada requer que você tome algumas decisões sobre como seu script de roteamento de tickets baseado em Claude se encaixa na arquitetura de seu sistema de roteamento de tickets maior. Existem duas maneiras que você poderia fazer isso:
* **Baseado em push:** O sistema de tickets de suporte que você está usando (por exemplo, Zendesk) dispara seu código enviando um evento webhook para seu serviço de roteamento, que então classifica a intenção e a roteia.
    * Esta abordagem é mais escalável na web, mas requer que você exponha um endpoint público.
* **Baseado em pull:** Seu código puxa os tickets mais recentes com base em um cronograma determinado e os roteia no momento do pull.
    * Esta abordagem é mais fácil de implementar, mas pode fazer chamadas desnecessárias para o sistema de tickets de suporte quando a frequência de pull é muito alta ou pode ser excessivamente lenta quando a frequência de pull é muito baixa.

Para qualquer uma dessas abordagens, você precisará envolver seu script em um serviço. A escolha da abordagem depende de quais APIs seu sistema de ticketing de suporte fornece.

***

<CardGroup cols={2}>
    <Card title="Livro de receitas de classificação" icon="link" href="https://github.com/anthropics/anthropic-cookbook/tree/main/capabilities/classification">
        Visite nosso livro de receitas de classificação para mais código de exemplo e orientação de avaliação detalhada.
    </Card>
    <Card title="Claude Console" icon="link" href="/dashboard">
        Comece a construir e avaliar seu fluxo de trabalho no Claude Console.
    </Card>
</CardGroup>