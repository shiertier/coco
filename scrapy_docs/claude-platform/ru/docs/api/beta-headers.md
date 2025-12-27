# Beta заголовки

Документация по использованию beta заголовков с Claude API

---

Beta заголовки позволяют получить доступ к экспериментальным функциям и новым возможностям модели до того, как они станут частью стандартного API.

Эти функции могут изменяться и могут быть модифицированы или удалены в будущих релизах.

<Info>
Beta заголовки часто используются в сочетании с [beta пространством имен в клиентских SDK](/docs/ru/api/client-sdks#beta-namespace-in-client-sdks)
</Info>

## Как использовать beta заголовки

Для доступа к beta функциям включите заголовок `anthropic-beta` в ваши API запросы:

```http
POST /v1/messages
Content-Type: application/json
X-API-Key: YOUR_API_KEY
anthropic-beta: BETA_FEATURE_NAME
```

При использовании SDK вы можете указать beta заголовки в опциях запроса:

<CodeGroup>

```python Python
from anthropic import Anthropic

client = Anthropic()

response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[
        {"role": "user", "content": "Hello, Claude"}
    ],
    betas=["beta-feature-name"]
)
```

```typescript TypeScript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

const msg = await anthropic.beta.messages.create({
  model: 'claude-sonnet-4-5',
  max_tokens: 1024,
  messages: [
    { role: 'user', content: 'Hello, Claude' }
  ],
  betas: ['beta-feature-name']
});
```

```bash cURL
curl https://api.anthropic.com/v1/messages \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "anthropic-beta: beta-feature-name" \
  -H "content-type: application/json" \
  -d '{
    "model": "claude-sonnet-4-5",
    "max_tokens": 1024,
    "messages": [
      {"role": "user", "content": "Hello, Claude"}
    ]
  }'
```

</CodeGroup>

<Warning>
Beta функции являются экспериментальными и могут:
- Иметь критические изменения без предупреждения
- Быть устаревшими или удаленными
- Иметь различные ограничения скорости или цены
- Быть недоступными во всех регионах
</Warning>

### Несколько beta функций

Для использования нескольких beta функций в одном запросе включите все названия функций в заголовок, разделенные запятыми:

```http
anthropic-beta: feature1,feature2,feature3
```

### Соглашения об именовании версий

Названия beta функций обычно следуют шаблону: `feature-name-YYYY-MM-DD`, где дата указывает, когда была выпущена beta версия. Всегда используйте точное название beta функции, как указано в документации.

## Обработка ошибок

Если вы используете недействительный или недоступный beta заголовок, вы получите ответ с ошибкой:

```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "Unsupported beta header: invalid-beta-name"
  }
}
```

## Получение помощи

По вопросам о beta функциях:

1. Проверьте документацию для конкретной функции
2. Просмотрите [журнал изменений API](/docs/ru/api/versioning) для обновлений
3. Обратитесь в службу поддержки за помощью с использованием в продакшене

Помните, что beta функции предоставляются "как есть" и могут не иметь тех же гарантий SLA, что и стабильные функции API.