# Ошибки

---

## HTTP ошибки

Наш API следует предсказуемому формату HTTP кодов ошибок:

* 400 - `invalid_request_error`: Возникла проблема с форматом или содержанием вашего запроса. Мы также можем использовать этот тип ошибки для других статус-кодов 4XX, не перечисленных ниже.
* 401 - `authentication_error`: Проблема с вашим API ключом.
* 403 - `permission_error`: Ваш API ключ не имеет разрешения на использование указанного ресурса.
* 404 - `not_found_error`: Запрашиваемый ресурс не найден.
* 413 - `request_too_large`: Запрос превышает максимально допустимое количество байт. Максимальный размер запроса составляет 32 МБ для стандартных API эндпоинтов.
* 429 - `rate_limit_error`: Ваш аккаунт достиг лимита скорости.
* 500 - `api_error`: Произошла неожиданная ошибка внутри систем Anthropic.
* 529 - `overloaded_error`: API временно перегружен.

  <Warning>
  Ошибки 529 могут возникать, когда API испытывают высокий трафик среди всех пользователей.
  
  В редких случаях, если ваша организация имеет резкое увеличение использования, вы можете увидеть ошибки 429 из-за ограничений ускорения на API. Чтобы избежать достижения ограничений ускорения, постепенно увеличивайте ваш трафик и поддерживайте последовательные паттерны использования.
  </Warning>

При получении [потокового](/docs/ru/build-with-claude/streaming) ответа через SSE возможно, что ошибка может произойти после возврата ответа 200, в таком случае обработка ошибок не будет следовать этим стандартным механизмам.

## Ограничения размера запроса

API применяет ограничения размера запроса для обеспечения оптимальной производительности:

| Тип эндпоинта | Максимальный размер запроса |
|:---|:---|
| Messages API | 32 МБ |
| Token Counting API | 32 МБ |
| [Batch API](/docs/ru/build-with-claude/batch-processing) | 256 МБ |
| [Files API](/docs/ru/build-with-claude/files) | 500 МБ |

Если вы превысите эти ограничения, вы получите ошибку 413 `request_too_large`. Ошибка возвращается от Cloudflare до того, как запрос достигнет наших API серверов.

## Формы ошибок

Ошибки всегда возвращаются как JSON, с объектом верхнего уровня `error`, который всегда включает значения `type` и `message`. Ответ также включает поле `request_id` для более легкого отслеживания и отладки. Например:

```json JSON
{
  "type": "error",
  "error": {
    "type": "not_found_error",
    "message": "The requested resource could not be found."
  },
  "request_id": "req_011CSHoEeqs5C35K2UUqR7Fy"
}
```

В соответствии с нашей политикой [версионирования](/docs/ru/api/versioning), мы можем расширить значения внутри этих объектов, и возможно, что значения `type` будут расти со временем.

## Request id

Каждый API ответ включает уникальный заголовок `request-id`. Этот заголовок содержит значение, такое как `req_018EeWyXxfu5pfWkrYcMdjWG`. При обращении в службу поддержки по поводу конкретного запроса, пожалуйста, включите этот ID, чтобы помочь нам быстро решить вашу проблему.

Наши официальные SDK предоставляют это значение как свойство объектов ответа верхнего уровня, содержащее значение заголовка `request-id`:

<CodeGroup>
  ```python Python
  import anthropic

  client = anthropic.Anthropic()

  message = client.messages.create(
      model="claude-sonnet-4-5",
      max_tokens=1024,
      messages=[
          {"role": "user", "content": "Hello, Claude"}
      ]
  )
  print(f"Request ID: {message._request_id}")
  ```

  ```typescript TypeScript
  import Anthropic from '@anthropic-ai/sdk';

  const client = new Anthropic();

  const message = await client.messages.create({
    model: 'claude-sonnet-4-5',
    max_tokens: 1024,
    messages: [
      {"role": "user", "content": "Hello, Claude"}
    ]
  });
  console.log('Request ID:', message._request_id);
  ```
</CodeGroup>

## Длинные запросы

<Warning>
 Мы настоятельно рекомендуем использовать [потоковый Messages API](/docs/ru/build-with-claude/streaming) или [Message Batches API](/docs/ru/api/creating-message-batches) для длительных запросов, особенно тех, которые превышают 10 минут.
</Warning>

Мы не рекомендуем устанавливать большие значения `max_tokens` без использования нашего [потокового Messages API](/docs/ru/build-with-claude/streaming)
или [Message Batches API](/docs/ru/api/creating-message-batches):

- Некоторые сети могут разрывать неактивные соединения через переменный период времени, что
может привести к сбою запроса или таймауту без получения ответа от Anthropic.
- Сети различаются по надежности; наш [Message Batches API](/docs/ru/api/creating-message-batches) может помочь вам
управлять риском сетевых проблем, позволяя вам опрашивать результаты вместо требования непрерывного сетевого соединения.

Если вы создаете прямую API интеграцию, вы должны знать, что установка [TCP socket keep-alive](https://tldp.org/HOWTO/TCP-Keepalive-HOWTO/programming.html) может уменьшить влияние таймаутов неактивных соединений в некоторых сетях.

Наши [SDK](/docs/ru/api/client-sdks) будут проверять, что ваши не-потоковые Messages API запросы не ожидаются превысить 10-минутный таймаут и
также установят опцию сокета для TCP keep-alive.