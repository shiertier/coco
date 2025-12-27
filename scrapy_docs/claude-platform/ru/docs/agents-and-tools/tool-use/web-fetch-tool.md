# Инструмент веб-выборки

Инструмент веб-выборки позволяет Claude получать полное содержимое с указанных веб-страниц и документов PDF.

---

Инструмент веб-выборки позволяет Claude получать полное содержимое с указанных веб-страниц и документов PDF.

<Note>
Инструмент веб-выборки в настоящее время находится в бета-версии. Чтобы включить его, используйте заголовок бета-версии `web-fetch-2025-09-10` в ваших запросах API.

Пожалуйста, используйте [эту форму](https://forms.gle/NhWcgmkcvPCMmPE86) для предоставления отзывов о качестве ответов модели, самом API или качестве документации.
</Note>

<Warning>
Включение инструмента веб-выборки в окружениях, где Claude обрабатывает ненадежные входные данные вместе с конфиденциальными данными, создает риски утечки данных. Мы рекомендуем использовать этот инструмент только в надежных окружениях или при работе с неконфиденциальными данными.

Чтобы минимизировать риски утечки, Claude не может динамически конструировать URL-адреса. Claude может получать только URL-адреса, которые были явно предоставлены пользователем или которые поступают из предыдущих результатов веб-поиска или веб-выборки. Однако остается остаточный риск, который следует тщательно учитывать при использовании этого инструмента.

Если утечка данных вызывает беспокойство, рассмотрите:
- Полное отключение инструмента веб-выборки
- Использование параметра `max_uses` для ограничения количества запросов
- Использование параметра `allowed_domains` для ограничения известными безопасными доменами
</Warning>

## Поддерживаемые модели

Веб-выборка доступна на:

- Claude Sonnet 4.5 (`claude-sonnet-4-5-20250929`)
- Claude Sonnet 4 (`claude-sonnet-4-20250514`)
- Claude Sonnet 3.7 ([устарела](/docs/ru/about-claude/model-deprecations)) (`claude-3-7-sonnet-20250219`)
- Claude Haiku 4.5 (`claude-haiku-4-5-20251001`)
- Claude Haiku 3.5 ([устарела](/docs/ru/about-claude/model-deprecations)) (`claude-3-5-haiku-latest`)
- Claude Opus 4.5 (`claude-opus-4-5-20251101`)
- Claude Opus 4.1 (`claude-opus-4-1-20250805`)
- Claude Opus 4 (`claude-opus-4-20250514`)

## Как работает веб-выборка

Когда вы добавляете инструмент веб-выборки в ваш запрос API:

1. Claude решает, когда получить содержимое на основе подсказки и доступных URL-адресов.
2. API получает полное текстовое содержимое с указанного URL-адреса.
3. Для PDF-файлов выполняется автоматическое извлечение текста.
4. Claude анализирует полученное содержимое и предоставляет ответ с дополнительными ссылками.

<Note>
Инструмент веб-выборки в настоящее время не поддерживает веб-сайты, динамически отображаемые через Javascript.
</Note>

## Как использовать веб-выборку

Предоставьте инструмент веб-выборки в вашем запросе API:

<CodeGroup>
```bash Shell
curl https://api.anthropic.com/v1/messages \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "anthropic-beta: web-fetch-2025-09-10" \
    --header "content-type: application/json" \
    --data '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 1024,
        "messages": [
            {
                "role": "user",
                "content": "Please analyze the content at https://example.com/article"
            }
        ],
        "tools": [{
            "type": "web_fetch_20250910",
            "name": "web_fetch",
            "max_uses": 5
        }]
    }'
```

```python Python
import anthropic

client = anthropic.Anthropic()

response = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[
        {
            "role": "user",
            "content": "Please analyze the content at https://example.com/article"
        }
    ],
    tools=[{
        "type": "web_fetch_20250910",
        "name": "web_fetch",
        "max_uses": 5
    }],
    extra_headers={
        "anthropic-beta": "web-fetch-2025-09-10"
    }
)
print(response)
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

async function main() {
  const response = await anthropic.messages.create({
    model: "claude-sonnet-4-5",
    max_tokens: 1024,
    messages: [
      {
        role: "user",
        content: "Please analyze the content at https://example.com/article"
      }
    ],
    tools: [{
      type: "web_fetch_20250910",
      name: "web_fetch",
      max_uses: 5
    }],
    headers: {
      "anthropic-beta": "web-fetch-2025-09-10"
    }
  });

  console.log(response);
}

main().catch(console.error);
```
</CodeGroup>

### Определение инструмента

Инструмент веб-выборки поддерживает следующие параметры:

```json JSON
{
  "type": "web_fetch_20250910",
  "name": "web_fetch",

  // Опционально: Ограничить количество выборок на запрос
  "max_uses": 10,

  // Опционально: Получать только с этих доменов
  "allowed_domains": ["example.com", "docs.example.com"],

  // Опционально: Никогда не получать с этих доменов
  "blocked_domains": ["private.example.com"],

  // Опционально: Включить ссылки для полученного содержимого
  "citations": {
    "enabled": true
  },

  // Опционально: Максимальная длина содержимого в токенах
  "max_content_tokens": 100000
}
```

#### Максимальное использование

Параметр `max_uses` ограничивает количество выполняемых веб-выборок. Если Claude попытается выполнить больше выборок, чем разрешено, `web_fetch_tool_result` будет ошибкой с кодом ошибки `max_uses_exceeded`. В настоящее время нет ограничения по умолчанию.

#### Фильтрация доменов

При использовании фильтров доменов:

- Домены не должны включать схему HTTP/HTTPS (используйте `example.com` вместо `https://example.com`)
- Поддомены автоматически включены (`example.com` охватывает `docs.example.com`)
- Поддерживаются подпути (`example.com/blog`)
- Вы можете использовать либо `allowed_domains`, либо `blocked_domains`, но не оба в одном запросе.

<Warning>
Имейте в виду, что символы Unicode в доменных именах могут создавать уязвимости безопасности через атаки омографов, когда визуально похожие символы из разных скриптов могут обойти фильтры доменов. Например, `аmazon.com` (с использованием кириллицы 'а') может выглядеть идентично `amazon.com`, но представляет другой домен.

При настройке списков разрешения/блокировки доменов:
- По возможности используйте доменные имена только на ASCII
- Учитывайте, что парсеры URL могут обрабатывать нормализацию Unicode по-разному
- Протестируйте фильтры доменов с потенциальными вариантами омографов
- Регулярно проверяйте конфигурации доменов на предмет подозрительных символов Unicode
</Warning>

#### Ограничения содержимого

Параметр `max_content_tokens` ограничивает объем содержимого, которое будет включено в контекст. Если полученное содержимое превышает это ограничение, оно будет усечено. Это помогает контролировать использование токенов при выборке больших документов.

<Note>
Ограничение параметра `max_content_tokens` является приблизительным. Фактическое количество используемых входных токенов может немного варьироваться.
</Note>

#### Ссылки

В отличие от веб-поиска, где ссылки всегда включены, ссылки являются опциональными для веб-выборки. Установите `"citations": {"enabled": true}` чтобы позволить Claude ссылаться на конкретные отрывки из полученных документов.

<Note>
При прямом отображении выходных данных API конечным пользователям ссылки должны быть включены на исходный источник. Если вы вносите изменения в выходные данные API, включая переработку и/или объединение их с вашим собственным материалом перед отображением конечным пользователям, отображайте ссылки надлежащим образом на основе консультации с вашей юридической командой.
</Note>

### Ответ

Вот пример структуры ответа:

```json
{
  "role": "assistant",
  "content": [
    // 1. Решение Claude получить содержимое
    {
      "type": "text",
      "text": "I'll fetch the content from the article to analyze it."
    },
    // 2. Запрос выборки
    {
      "type": "server_tool_use",
      "id": "srvtoolu_01234567890abcdef",
      "name": "web_fetch",
      "input": {
        "url": "https://example.com/article"
      }
    },
    // 3. Результаты выборки
    {
      "type": "web_fetch_tool_result",
      "tool_use_id": "srvtoolu_01234567890abcdef",
      "content": {
        "type": "web_fetch_result",
        "url": "https://example.com/article",
        "content": {
          "type": "document",
          "source": {
            "type": "text",
            "media_type": "text/plain",
            "data": "Full text content of the article..."
          },
          "title": "Article Title",
          "citations": {"enabled": true}
        },
        "retrieved_at": "2025-08-25T10:30:00Z"
      }
    },
    // 4. Анализ Claude со ссылками (если включены)
    {
      "text": "Based on the article, ",
      "type": "text"
    },
    {
      "text": "the main argument presented is that artificial intelligence will transform healthcare",
      "type": "text",
      "citations": [
        {
          "type": "char_location",
          "document_index": 0,
          "document_title": "Article Title",
          "start_char_index": 1234,
          "end_char_index": 1456,
          "cited_text": "Artificial intelligence is poised to revolutionize healthcare delivery..."
        }
      ]
    }
  ],
  "id": "msg_a930390d3a",
  "usage": {
    "input_tokens": 25039,
    "output_tokens": 931,
    "server_tool_use": {
      "web_fetch_requests": 1
    }
  },
  "stop_reason": "end_turn"
}
```

#### Результаты выборки

Результаты выборки включают:

- `url`: URL-адрес, который был получен
- `content`: Блок документа, содержащий полученное содержимое
- `retrieved_at`: Временная метка, когда содержимое было получено

<Note>
Инструмент веб-выборки кэширует результаты для улучшения производительности и уменьшения избыточных запросов. Это означает, что возвращаемое содержимое может быть не всегда последней версией, доступной по URL-адресу. Поведение кэша управляется автоматически и может изменяться со временем для оптимизации различных типов содержимого и моделей использования.
</Note>

Для документов PDF содержимое будет возвращено как данные в кодировке base64:

```json
{
  "type": "web_fetch_tool_result",
  "tool_use_id": "srvtoolu_02",
  "content": {
    "type": "web_fetch_result",
    "url": "https://example.com/paper.pdf",
    "content": {
      "type": "document",
      "source": {
        "type": "base64",
        "media_type": "application/pdf",
        "data": "JVBERi0xLjQKJcOkw7zDtsOfCjIgMCBvYmo..."
      },
      "citations": {"enabled": true}
    },
    "retrieved_at": "2025-08-25T10:30:02Z"
  }
}
```

#### Ошибки

Когда инструмент веб-выборки встречает ошибку, Claude API возвращает ответ 200 (успех) с ошибкой, представленной в теле ответа:

```json
{
  "type": "web_fetch_tool_result",
  "tool_use_id": "srvtoolu_a93jad",
  "content": {
    "type": "web_fetch_tool_error",
    "error_code": "url_not_accessible"
  }
}
```

Это возможные коды ошибок:

- `invalid_input`: Неверный формат URL
- `url_too_long`: URL превышает максимальную длину (250 символов)
- `url_not_allowed`: URL заблокирован правилами фильтрации доменов и ограничениями модели
- `url_not_accessible`: Не удалось получить содержимое (ошибка HTTP)
- `too_many_requests`: Превышен лимит частоты запросов
- `unsupported_content_type`: Тип содержимого не поддерживается (только текст и PDF)
- `max_uses_exceeded`: Превышено максимальное использование инструмента веб-выборки
- `unavailable`: Произошла внутренняя ошибка

## Проверка URL

По соображениям безопасности инструмент веб-выборки может получать только URL-адреса, которые ранее появились в контексте беседы. Это включает:

- URL-адреса в сообщениях пользователя
- URL-адреса в результатах инструментов на стороне клиента
- URL-адреса из предыдущих результатов веб-поиска или веб-выборки

Инструмент не может получать произвольные URL-адреса, которые генерирует Claude, или URL-адреса из инструментов сервера на основе контейнеров (выполнение кода, Bash и т. д.).

## Комбинированный поиск и выборка

Веб-выборка работает беспрепятственно с веб-поиском для комплексного сбора информации:

```python
import anthropic

client = anthropic.Anthropic()

response = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=4096,
    messages=[
        {
            "role": "user",
            "content": "Find recent articles about quantum computing and analyze the most relevant one in detail"
        }
    ],
    tools=[
        {
            "type": "web_search_20250305",
            "name": "web_search",
            "max_uses": 3
        },
        {
            "type": "web_fetch_20250910",
            "name": "web_fetch",
            "max_uses": 5,
            "citations": {"enabled": True}
        }
    ],
    extra_headers={
        "anthropic-beta": "web-fetch-2025-09-10"
    }
)
```

В этом рабочем процессе Claude будет:
1. Использовать веб-поиск для поиска релевантных статей
2. Выбрать наиболее перспективные результаты
3. Использовать веб-выборку для получения полного содержимого
4. Предоставить подробный анализ со ссылками

## Кэширование подсказок

Веб-выборка работает с [кэшированием подсказок](/docs/ru/build-with-claude/prompt-caching). Чтобы включить кэширование подсказок, добавьте точки разрыва `cache_control` в ваш запрос. Кэшированные результаты выборки можно повторно использовать в разных ходах беседы.

```python
import anthropic

client = anthropic.Anthropic()

# Первый запрос с веб-выборкой
messages = [
    {
        "role": "user",
        "content": "Analyze this research paper: https://arxiv.org/abs/2024.12345"
    }
]

response1 = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=messages,
    tools=[{
        "type": "web_fetch_20250910",
        "name": "web_fetch"
    }],
    extra_headers={
        "anthropic-beta": "web-fetch-2025-09-10"
    }
)

# Добавить ответ Claude в беседу
messages.append({
    "role": "assistant",
    "content": response1.content
})

# Второй запрос с точкой разрыва кэша
messages.append({
    "role": "user",
    "content": "What methodology does the paper use?",
    "cache_control": {"type": "ephemeral"}
})

response2 = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=messages,
    tools=[{
        "type": "web_fetch_20250910",
        "name": "web_fetch"
    }],
    extra_headers={
        "anthropic-beta": "web-fetch-2025-09-10"
    }
)

# Второй ответ получает выгоду от кэшированных результатов выборки
print(f"Cache read tokens: {response2.usage.get('cache_read_input_tokens', 0)}")
```

## Потоковая передача

При включенной потоковой передаче события выборки являются частью потока с паузой во время получения содержимого:

```javascript
event: message_start
data: {"type": "message_start", "message": {"id": "msg_abc123", "type": "message"}}

event: content_block_start
data: {"type": "content_block_start", "index": 0, "content_block": {"type": "text", "text": ""}}

// Решение Claude получить содержимое

event: content_block_start
data: {"type": "content_block_start", "index": 1, "content_block": {"type": "server_tool_use", "id": "srvtoolu_xyz789", "name": "web_fetch"}}

// URL выборки передается потоком
event: content_block_delta
data: {"type": "content_block_delta", "index": 1, "delta": {"type": "input_json_delta", "partial_json": "{\"url\":\"https://example.com/article\"}"}}

// Пауза во время выполнения выборки

// Результаты выборки передаются потоком
event: content_block_start
data: {"type": "content_block_start", "index": 2, "content_block": {"type": "web_fetch_tool_result", "tool_use_id": "srvtoolu_xyz789", "content": {"type": "web_fetch_result", "url": "https://example.com/article", "content": {"type": "document", "source": {"type": "text", "media_type": "text/plain", "data": "Article content..."}}}}}

// Ответ Claude продолжается...
```

## Пакетные запросы

Вы можете включить инструмент веб-выборки в [Messages Batches API](/docs/ru/build-with-claude/batch-processing). Вызовы инструмента веб-выборки через Messages Batches API оцениваются так же, как в обычных запросах Messages API.

## Использование и цены

Web fetch usage has **no additional charges** beyond standard token costs:

```json
"usage": {
  "input_tokens": 25039,
  "output_tokens": 931,
  "cache_read_input_tokens": 0,
  "cache_creation_input_tokens": 0,
  "server_tool_use": {
    "web_fetch_requests": 1
  }
}
```

The web fetch tool is available on the Claude API at **no additional cost**. You only pay standard token costs for the fetched content that becomes part of your conversation context.

To protect against inadvertently fetching large content that would consume excessive tokens, use the `max_content_tokens` parameter to set appropriate limits based on your use case and budget considerations.

Example token usage for typical content:
- Average web page (10KB): ~2,500 tokens
- Large documentation page (100KB): ~25,000 tokens  
- Research paper PDF (500KB): ~125,000 tokens