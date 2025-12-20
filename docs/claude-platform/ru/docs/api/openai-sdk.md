# Совместимость с OpenAI SDK

Anthropic предоставляет уровень совместимости, который позволяет вам использовать OpenAI SDK для тестирования Claude API. С несколькими изменениями кода вы можете быстро оценить возможности моделей Anthropic.

---

<Note>
Этот уровень совместимости предназначен в первую очередь для тестирования и сравнения возможностей моделей и не считается долгосрочным или готовым к производству решением для большинства случаев использования. Хотя мы намерены сохранить его полностью функциональным и не вносить критические изменения, наш приоритет — надежность и эффективность [Claude API](/docs/ru/api/overview). 

Для получения дополнительной информации об известных ограничениях совместимости см. [Важные ограничения совместимости OpenAI](#important-openai-compatibility-limitations).

Если у вас возникнут проблемы с функцией совместимости OpenAI SDK, пожалуйста, дайте нам знать [здесь](https://forms.gle/oQV4McQNiuuNbz9n8).
</Note>

<Tip>
Для лучшего опыта и доступа к полному набору функций Claude API ([обработка PDF](/docs/ru/build-with-claude/pdf-support), [цитирования](/docs/ru/build-with-claude/citations), [расширенное мышление](/docs/ru/build-with-claude/extended-thinking) и [кэширование подсказок](/docs/ru/build-with-claude/prompt-caching)), мы рекомендуем использовать встроенный [Claude API](/docs/ru/api/overview).
</Tip>

## Начало работы с OpenAI SDK

Чтобы использовать функцию совместимости OpenAI SDK, вам необходимо:

1. Использовать официальный OpenAI SDK  
2. Изменить следующее  
   * Обновить базовый URL для указания на Claude API  
   * Заменить ваш ключ API на [ключ Claude API](/settings/keys)  
   * Обновить имя модели для использования [модели Claude](/docs/ru/about-claude/models/overview)  
3. Просмотреть документацию ниже, чтобы узнать, какие функции поддерживаются

### Пример быстрого старта

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

## Важные ограничения совместимости OpenAI

#### Поведение API

Вот наиболее существенные различия при использовании OpenAI:

* Параметр `strict` для вызова функций игнорируется, что означает, что JSON использования инструмента не гарантирует соответствие предоставленной схеме. Для гарантированного соответствия схеме используйте встроенный [Claude API со структурированными выходами](/docs/ru/build-with-claude/structured-outputs).
* Аудиовход не поддерживается; он просто игнорируется и удаляется из входных данных  
* Кэширование подсказок не поддерживается, но оно поддерживается в [Anthropic SDK](/docs/ru/api/client-sdks)  
* Системные/сообщения разработчика поднимаются и объединяются в начало разговора, так как Anthropic поддерживает только одно начальное системное сообщение.

Большинство неподдерживаемых полей молча игнорируются, а не вызывают ошибки. Все они задокументированы ниже.

#### Рассмотрения качества вывода

Если вы много работали над настройкой вашей подсказки, она, вероятно, хорошо настроена специально для OpenAI. Рассмотрите возможность использования нашего [улучшителя подсказок в Claude Console](/dashboard) в качестве хорошей отправной точки.

#### Поднятие системного / сообщения разработчика

Большинство входных данных для OpenAI SDK четко отображаются непосредственно на параметры API Anthropic, но одно отличие заключается в обработке системных / сообщений разработчика. Эти два сообщения могут быть размещены по всему диалогу чата через OpenAI. Поскольку Anthropic поддерживает только начальное системное сообщение, мы берем все системные/сообщения разработчика и объединяем их вместе с одним переводом строки (`\n`) между ними. Эта полная строка затем предоставляется как одно системное сообщение в начале сообщений.

#### Поддержка расширенного мышления

Вы можете включить возможности [расширенного мышления](/docs/ru/build-with-claude/extended-thinking), добавив параметр `thinking`. Хотя это улучшит рассуждения Claude для сложных задач, OpenAI SDK не вернет подробный процесс мышления Claude. Для полных функций расширенного мышления, включая доступ к пошаговому выводу рассуждений Claude, используйте встроенный Claude API.

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

## Ограничения скорости

Ограничения скорости следуют [стандартным ограничениям](/docs/ru/api/rate-limits) Anthropic для конечной точки `/v1/messages`.

## Подробная поддержка совместимого API OpenAI
### Поля запроса
#### Простые поля
| Поле | Статус поддержки |
|--------|----------------|
| `model` | Используйте имена моделей Claude |
| `max_tokens` | Полностью поддерживается |
| `max_completion_tokens` | Полностью поддерживается |
| `stream` | Полностью поддерживается |
| `stream_options` | Полностью поддерживается |
| `top_p` | Полностью поддерживается |
| `parallel_tool_calls` | Полностью поддерживается |
| `stop` | Все последовательности остановки без пробелов работают |
| `temperature` | От 0 до 1 (включительно). Значения больше 1 ограничены 1. |
| `n` | Должно быть ровно 1 |
| `logprobs` | Игнорируется |
| `metadata` | Игнорируется |
| `response_format` | Игнорируется. Для вывода JSON используйте [Структурированные выходы](/docs/ru/build-with-claude/structured-outputs) с встроенным Claude API |
| `prediction` | Игнорируется |
| `presence_penalty` | Игнорируется |
| `frequency_penalty` | Игнорируется |
| `seed` | Игнорируется |
| `service_tier` | Игнорируется |
| `audio` | Игнорируется |
| `logit_bias` | Игнорируется |
| `store` | Игнорируется |
| `user` | Игнорируется |
| `modalities` | Игнорируется |
| `top_logprobs` | Игнорируется |
| `reasoning_effort` | Игнорируется |

#### Поля `tools` / `functions`
<section title="Показать поля">

<Tabs>
<Tab title="Tools">
Поля `tools[n].function`
| Поле        | Статус поддержки         |
|--------------|-----------------|
| `name`       | Полностью поддерживается |
| `description`| Полностью поддерживается |
| `parameters` | Полностью поддерживается |
| `strict`     | Игнорируется. Используйте [Структурированные выходы](/docs/ru/build-with-claude/structured-outputs) с встроенным Claude API для строгой валидации схемы |
</Tab>
<Tab title="Functions">

Поля `functions[n]`
<Info>
OpenAI объявил поле `functions` устаревшим и предлагает использовать вместо этого `tools`.
</Info>
| Поле        | Статус поддержки         |
|--------------|-----------------|
| `name`       | Полностью поддерживается |
| `description`| Полностью поддерживается |
| `parameters` | Полностью поддерживается |
| `strict`     | Игнорируется. Используйте [Структурированные выходы](/docs/ru/build-with-claude/structured-outputs) с встроенным Claude API для строгой валидации схемы |
</Tab>
</Tabs>

</section>

#### Поля массива `messages`
<section title="Показать поля">

<Tabs>
<Tab title="Developer role">
Поля для `messages[n].role == "developer"`
<Info>
Сообщения разработчика поднимаются в начало разговора как часть начального системного сообщения
</Info>
| Поле | Статус поддержки |
|-------|---------|
| `content` | Полностью поддерживается, но поднято |
| `name` | Игнорируется |

</Tab>
<Tab title="System role">
Поля для `messages[n].role == "system"`

<Info>
Системные сообщения поднимаются в начало разговора как часть начального системного сообщения
</Info>
| Поле | Статус поддержки |
|-------|---------|
| `content` | Полностью поддерживается, но поднято |
| `name` | Игнорируется |

</Tab>
<Tab title="User role">
Поля для `messages[n].role == "user"`

| Поле | Вариант | Подполе | Статус поддержки |
|-------|---------|-----------|----------------|
| `content` | `string` | | Полностью поддерживается |
| | `array`, `type == "text"` | | Полностью поддерживается |
| | `array`, `type == "image_url"` | `url` | Полностью поддерживается |
| | | `detail` | Игнорируется |
| | `array`, `type == "input_audio"` | | Игнорируется |
| | `array`, `type == "file"` | | Игнорируется |
| `name` | | | Игнорируется |

</Tab>

<Tab title="Assistant role">
Поля для `messages[n].role == "assistant"`
| Поле | Вариант | Статус поддержки |
|-------|---------|----------------|
| `content` | `string` | Полностью поддерживается |
| | `array`, `type == "text"` | Полностью поддерживается |
| | `array`, `type == "refusal"` | Игнорируется |
| `tool_calls` | | Полностью поддерживается |
| `function_call` | | Полностью поддерживается |
| `audio` | | Игнорируется |
| `refusal` | | Игнорируется |

</Tab>

<Tab title="Tool role">
Поля для `messages[n].role == "tool"`
| Поле | Вариант | Статус поддержки |
|-------|---------|----------------|
| `content` | `string` | Полностью поддерживается |
| | `array`, `type == "text"` | Полностью поддерживается |
| `tool_call_id` | | Полностью поддерживается |
| `tool_choice` | | Полностью поддерживается |
| `name` | | Игнорируется |
</Tab>

<Tab title="Function role">
Поля для `messages[n].role == "function"`
| Поле | Вариант | Статус поддержки |
|-------|---------|----------------|
| `content` | `string` | Полностью поддерживается |
| | `array`, `type == "text"` | Полностью поддерживается |
| `tool_choice` | | Полностью поддерживается |
| `name` | | Игнорируется |
</Tab>
</Tabs>

</section>

### Поля ответа

| Поле | Статус поддержки |
|---------------------------|----------------|
| `id` | Полностью поддерживается |
| `choices[]` | Всегда будет иметь длину 1 |
| `choices[].finish_reason` | Полностью поддерживается |
| `choices[].index` | Полностью поддерживается |
| `choices[].message.role` | Полностью поддерживается |
| `choices[].message.content` | Полностью поддерживается |
| `choices[].message.tool_calls` | Полностью поддерживается |
| `object` | Полностью поддерживается |
| `created` | Полностью поддерживается |
| `model` | Полностью поддерживается |
| `finish_reason` | Полностью поддерживается |
| `content` | Полностью поддерживается |
| `usage.completion_tokens` | Полностью поддерживается |
| `usage.prompt_tokens` | Полностью поддерживается |
| `usage.total_tokens` | Полностью поддерживается |
| `usage.completion_tokens_details` | Всегда пусто |
| `usage.prompt_tokens_details` | Всегда пусто |
| `choices[].message.refusal` | Всегда пусто |
| `choices[].message.audio` | Всегда пусто |
| `logprobs` | Всегда пусто |
| `service_tier` | Всегда пусто |
| `system_fingerprint` | Всегда пусто |

### Совместимость сообщений об ошибках

Уровень совместимости поддерживает согласованные форматы ошибок с API OpenAI. Однако подробные сообщения об ошибках не будут эквивалентны. Мы рекомендуем использовать сообщения об ошибках только для логирования и отладки.

### Совместимость заголовков

Хотя OpenAI SDK автоматически управляет заголовками, вот полный список заголовков, поддерживаемых Claude API для разработчиков, которым нужно работать с ними напрямую.

| Заголовок | Статус поддержки |
|---------|----------------|
| `x-ratelimit-limit-requests` | Полностью поддерживается |
| `x-ratelimit-limit-tokens` | Полностью поддерживается |
| `x-ratelimit-remaining-requests` | Полностью поддерживается |
| `x-ratelimit-remaining-tokens` | Полностью поддерживается |
| `x-ratelimit-reset-requests` | Полностью поддерживается |
| `x-ratelimit-reset-tokens` | Полностью поддерживается |
| `retry-after` | Полностью поддерживается |
| `request-id` | Полностью поддерживается |
| `openai-version` | Всегда `2020-10-01` |
| `authorization` | Полностью поддерживается |
| `openai-processing-ms` | Всегда пусто |