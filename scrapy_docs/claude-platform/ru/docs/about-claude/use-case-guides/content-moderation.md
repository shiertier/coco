# Модерация контента

Модерация контента является критически важным аспектом поддержания безопасной, уважительной и продуктивной среды в цифровых приложениях. В этом руководстве мы обсудим, как Claude может использоваться для модерации контента в вашем цифровом приложении.

---

> Посетите нашу [книгу рецептов по модерации контента](https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fmoderation%5Ffilter.ipynb), чтобы увидеть пример реализации модерации контента с использованием Claude.

<Tip>Это руководство сосредоточено на модерации пользовательского контента в вашем приложении. Если вы ищете руководство по модерации взаимодействий с Claude, пожалуйста, обратитесь к нашему [руководству по защитным мерам](/docs/ru/test-and-evaluate/strengthen-guardrails/reduce-hallucinations).</Tip>

## Перед созданием с Claude

### Решите, использовать ли Claude для модерации контента

Вот некоторые ключевые индикаторы того, что вам следует использовать LLM, такую как Claude, вместо традиционного ML или основанного на правилах подхода для модерации контента:

<section title="Вы хотите экономически эффективную и быструю реализацию">
Традиционные методы ML требуют значительных инженерных ресурсов, экспертизы в области ML и затрат на инфраструктуру. Системы человеческой модерации влекут еще более высокие затраты. С Claude вы можете запустить сложную систему модерации за долю времени и за долю цены.
</section>
<section title="Вы желаете как семантическое понимание, так и быстрые решения">
Традиционные подходы ML, такие как модели мешка слов или простое сопоставление шаблонов, часто испытывают трудности с пониманием тона, намерения и контекста контента. В то время как системы человеческой модерации превосходно понимают семантическое значение, они требуют времени для рассмотрения контента. Claude преодолевает разрыв, сочетая семантическое понимание со способностью быстро принимать решения о модерации.
</section>
<section title="Вам нужны последовательные политические решения">
Используя свои продвинутые возможности рассуждения, Claude может интерпретировать и применять сложные руководящие принципы модерации единообразно. Эта последовательность помогает обеспечить справедливое обращение со всем контентом, снижая риск непоследовательных или предвзятых решений модерации, которые могут подорвать доверие пользователей.
</section>
<section title="Ваши политики модерации, вероятно, изменятся или эволюционируют со временем">
Как только традиционный подход ML был установлен, его изменение является трудоемким и требующим больших данных предприятием. С другой стороны, по мере развития ваших продуктов или потребностей клиентов, Claude может легко адаптироваться к изменениям или дополнениям в политиках модерации без обширной переразметки обучающих данных.
</section>
<section title="Вам требуется интерпретируемое рассуждение для ваших решений модерации">
Если вы хотите предоставить пользователям или регуляторам четкие объяснения решений модерации, Claude может генерировать подробные и связные обоснования. Эта прозрачность важна для построения доверия и обеспечения подотчетности в практиках модерации контента.
</section>
<section title="Вам нужна многоязычная поддержка без поддержания отдельных моделей">
Традиционные подходы ML обычно требуют отдельных моделей или обширных процессов перевода для каждого поддерживаемого языка. Человеческая модерация требует найма рабочей силы, владеющей каждым поддерживаемым языком. Многоязычные возможности Claude позволяют ему классифицировать тикеты на различных языках без необходимости в отдельных моделях или обширных процессах перевода, упрощая модерацию для глобальных клиентских баз.
</section>
<section title="Вам требуется мультимодальная поддержка">
Мультимодальные возможности Claude позволяют ему анализировать и интерпретировать контент как в тексте, так и в изображениях. Это делает его универсальным инструментом для комплексной модерации контента в средах, где различные типы медиа должны оцениваться вместе.
</section>

<Note>Anthropic обучила все модели Claude быть честными, полезными и безвредными. Это может привести к тому, что Claude будет модерировать контент, считающийся особенно опасным (в соответствии с нашей [Политикой допустимого использования](https://www.anthropic.com/legal/aup)), независимо от используемого промпта. Например, веб-сайт для взрослых, который хочет разрешить пользователям публиковать откровенный сексуальный контент, может обнаружить, что Claude все еще помечает откровенный контент как требующий модерации, даже если они указывают в своем промпте не модерировать откровенный сексуальный контент. Мы рекомендуем ознакомиться с нашей AUP заранее, прежде чем создавать решение для модерации.</Note>

### Генерируйте примеры контента для модерации
Перед разработкой решения для модерации контента сначала создайте примеры контента, который должен быть помечен, и контента, который не должен быть помечен. Убедитесь, что вы включили пограничные случаи и сложные сценарии, которые могут быть трудными для эффективной обработки системой модерации контента. После этого просмотрите свои примеры, чтобы создать четко определенный список категорий модерации.
Например, примеры, сгенерированные платформой социальных медиа, могут включать следующее:

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

Эффективная модерация этих примеров требует тонкого понимания языка. В комментарии `This movie was great, I really enjoyed it. The main actor really killed it!` система модерации контента должна распознать, что "killed it" является метафорой, а не указанием на реальное насилие. Наоборот, несмотря на отсутствие явных упоминаний насилия, комментарий `Delete this post now or you better hide. I am coming after you and your family.` должен быть помечен системой модерации контента.

Список `unsafe_categories` может быть настроен в соответствии с вашими конкретными потребностями. Например, если вы хотите предотвратить создание контента несовершеннолетними на вашем веб-сайте, вы можете добавить "Underage Posting" к списку.

___

## Как модерировать контент с помощью Claude

### Выберите правильную модель Claude
При выборе модели важно учитывать размер ваших данных. Если затраты вызывают беспокойство, меньшая модель, такая как Claude Haiku 3, является отличным выбором из-за своей экономической эффективности. Ниже приведена оценка стоимости модерации текста для платформы социальных медиа, которая получает один миллиард постов в месяц:

* **Размер контента**
    * Постов в месяц: 1 млрд
    * Символов на пост: 100
    * Общее количество символов: 100 млрд

* **Оценочные токены**
    * Входные токены: 28,6 млрд (предполагая 1 токен на 3,5 символа)
    * Процент помеченных сообщений: 3%
    * Выходные токены на помеченное сообщение: 50
    * Общие выходные токены: 1,5 млрд

* **Оценочная стоимость Claude Haiku 3**
    * Стоимость входных токенов: 2,860 MTok * \$0.25/MTok = \$715
    * Стоимость выходных токенов: 1,500 MTok * \$1.25/MTok = \$1,875
    * Месячная стоимость: \$715 + \$1,875 = \$2,590

* **Оценочная стоимость Claude Sonnet 4.5**
    * Стоимость входных токенов: 2,860 MTok * \$3.00/MTok = \$8,580
    * Стоимость выходных токенов: 1,500 MTok * \$15.00/MTok = \$22,500
    * Месячная стоимость: \$8,580 + \$22,500 = \$31,080

<Tip>Фактические затраты могут отличаться от этих оценок. Эти оценки основаны на промпте, выделенном в разделе о [пакетной обработке](#consider-batch-processing). Выходные токены могут быть еще больше сокращены путем удаления поля `explanation` из ответа.</Tip>  

### Создайте сильный промпт

Чтобы использовать Claude для модерации контента, Claude должен понимать требования модерации вашего приложения. Давайте начнем с написания промпта, который позволяет вам определить ваши потребности в модерации:

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

В этом примере функция `moderate_message` содержит промпт оценки, который включает категории небезопасного контента и сообщение, которое мы хотим оценить. Промпт просит Claude оценить, должно ли сообщение быть модерировано, основываясь на небезопасных категориях, которые мы определили.

Оценка модели затем анализируется, чтобы определить, есть ли нарушение. Если есть нарушение, Claude также возвращает список нарушенных категорий, а также объяснение того, почему сообщение небезопасно.

### Оцените ваш промпт

Модерация контента - это проблема классификации. Таким образом, вы можете использовать те же техники, изложенные в нашей [книге рецептов по классификации](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/classification/guide.ipynb), чтобы определить точность вашей системы модерации контента.

Одно дополнительное соображение заключается в том, что вместо рассмотрения модерации контента как проблемы бинарной классификации, вы можете вместо этого создать несколько категорий для представления различных уровней риска. Создание нескольких уровней риска позволяет вам настроить агрессивность вашей модерации. Например, вы можете автоматически блокировать пользовательские запросы, которые считаются высокорисковыми, в то время как пользователи с множеством среднерисковых запросов помечаются для человеческого рассмотрения.

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

Этот код реализует функцию `assess_risk_level`, которая использует Claude для оценки уровня риска сообщения. Функция принимает сообщение и список небезопасных категорий в качестве входных данных.

Внутри функции генерируется промпт для Claude, включающий сообщение для оценки, небезопасные категории и конкретные инструкции для оценки уровня риска. Промпт инструктирует Claude отвечать объектом JSON, который включает уровень риска, нарушенные категории и необязательное объяснение.

Этот подход обеспечивает гибкую модерацию контента путем назначения уровней риска. Он может быть легко интегрирован в более крупную систему для автоматизации фильтрации контента или пометки комментариев для человеческого рассмотрения на основе их оцененного уровня риска. Например, при выполнении этого кода комментарий `Delete this post now or you better hide. I am coming after you and your family.` идентифицируется как высокорисковый из-за его опасной угрозы. Наоборот, комментарий `Stay away from the 5G cellphones!! They are using 5G to control you.` категоризируется как среднерисковый.

### Разверните ваш промпт

Как только вы уверены в качестве вашего решения, пришло время развернуть его в продакшене. Вот некоторые лучшие практики, которым следует следовать при использовании модерации контента в продакшене:

1. **Предоставляйте четкую обратную связь пользователям:** Когда пользовательский ввод блокируется или ответ помечается из-за модерации контента, предоставляйте информативную и конструктивную обратную связь, чтобы помочь пользователям понять, почему их сообщение было помечено и как они могут переформулировать его соответствующим образом. В примерах кода выше это делается через тег `explanation` в ответе Claude.

2. **Анализируйте модерируемый контент:** Отслеживайте типы контента, помечаемого вашей системой модерации, чтобы выявить тенденции и потенциальные области для улучшения.

3. **Непрерывно оценивайте и улучшайте:** Регулярно оценивайте производительность вашей системы модерации контента, используя метрики, такие как отслеживание точности и полноты. Используйте эти данные для итеративного улучшения ваших промптов модерации, ключевых слов и критериев оценки.

___

## Улучшите производительность

В сложных сценариях может быть полезно рассмотреть дополнительные стратегии для улучшения производительности помимо стандартных [техник промпт-инжиниринга](/docs/ru/build-with-claude/prompt-engineering/overview). Вот некоторые продвинутые стратегии:

### Определите темы и предоставьте примеры

В дополнение к перечислению небезопасных категорий в промпте, дальнейшие улучшения могут быть сделаны путем предоставления определений и фраз, связанных с каждой категорией.

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

Функция `moderate_message_with_definitions` расширяет более раннюю функцию `moderate_message`, позволяя каждой небезопасной категории быть сопряженной с подробным определением. Это происходит в коде путем замены списка `unsafe_categories` из исходной функции словарем `unsafe_category_definitions`. Этот словарь сопоставляет каждую небезопасную категорию с ее соответствующим определением. И названия категорий, и их определения включены в промпт.

Примечательно, что определение для категории `Specialized Advice` теперь указывает типы финансовых советов, которые должны быть запрещены. В результате комментарий `It's a great time to invest in gold!`, который ранее прошел оценку `moderate_message`, теперь вызывает нарушение.

### Рассмотрите пакетную обработку

Чтобы снизить затраты в ситуациях, когда модерация в реальном времени не нужна, рассмотрите модерацию сообщений пакетами. Включите несколько сообщений в контекст промпта и попросите Claude оценить, какие сообщения должны быть модерированы.

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
В этом примере функция `batch_moderate_messages` обрабатывает модерацию целого пакета сообщений одним вызовом API Claude.
Внутри функции создается промпт, который включает список сообщений для оценки, определенные категории небезопасного контента и их описания. Промпт направляет Claude вернуть объект JSON, перечисляющий все сообщения, содержащие нарушения. Каждое сообщение в ответе идентифицируется по его id, который соответствует позиции сообщения во входном списке.
Имейте в виду, что поиск оптимального размера пакета для ваших конкретных потребностей может потребовать некоторых экспериментов. Хотя большие размеры пакетов могут снизить затраты, они также могут привести к небольшому снижению качества. Кроме того, вам может потребоваться увеличить параметр `max_tokens` в вызове API Claude для размещения более длинных ответов. Для получения подробной информации о максимальном количестве токенов, которые может выводить выбранная вами модель, обратитесь к [странице сравнения моделей](/docs/ru/about-claude/models#model-comparison-table).

<CardGroup cols={2}> 
  <Card title="Книга рецептов по модерации контента" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fmoderation%5Ffilter.ipynb">
    Посмотрите полностью реализованный пример на основе кода того, как использовать Claude для модерации контента.
  </Card>
  <Card title="Руководство по защитным мерам" icon="link" href="/docs/ru/test-and-evaluate/strengthen-guardrails/reduce-hallucinations">
    Изучите наше руководство по защитным мерам для техник модерации взаимодействий с Claude.
  </Card>
</CardGroup>