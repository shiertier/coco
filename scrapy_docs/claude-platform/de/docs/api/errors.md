# Fehler

---

## HTTP-Fehler

Unsere API folgt einem vorhersagbaren HTTP-Fehlercode-Format:

* 400 - `invalid_request_error`: Es gab ein Problem mit dem Format oder Inhalt Ihrer Anfrage. Wir können diesen Fehlertyp auch für andere 4XX-Statuscodes verwenden, die unten nicht aufgeführt sind.
* 401 - `authentication_error`: Es gibt ein Problem mit Ihrem API-Schlüssel.
* 403 - `permission_error`: Ihr API-Schlüssel hat keine Berechtigung, die angegebene Ressource zu verwenden.
* 404 - `not_found_error`: Die angeforderte Ressource wurde nicht gefunden.
* 413 - `request_too_large`: Die Anfrage überschreitet die maximal zulässige Anzahl von Bytes. Die maximale Anfragegröße beträgt 32 MB für Standard-API-Endpunkte.
* 429 - `rate_limit_error`: Ihr Konto hat ein Ratenlimit erreicht.
* 500 - `api_error`: Ein unerwarteter Fehler ist in Anthropics Systemen aufgetreten.
* 529 - `overloaded_error`: Die API ist vorübergehend überlastet.

  <Warning>
  529-Fehler können auftreten, wenn APIs hohen Traffic bei allen Benutzern erfahren.
  
  In seltenen Fällen, wenn Ihre Organisation einen starken Anstieg der Nutzung hat, könnten Sie 429-Fehler aufgrund von Beschleunigungslimits der API sehen. Um das Erreichen von Beschleunigungslimits zu vermeiden, steigern Sie Ihren Traffic schrittweise und halten Sie konsistente Nutzungsmuster aufrecht.
  </Warning>

Beim Empfangen einer [Streaming](/docs/de/build-with-claude/streaming)-Antwort über SSE ist es möglich, dass ein Fehler auftreten kann, nachdem eine 200-Antwort zurückgegeben wurde. In diesem Fall würde die Fehlerbehandlung nicht diesen Standardmechanismen folgen.

## Anfragegrößenlimits

Die API setzt Anfragegrößenlimits durch, um optimale Leistung zu gewährleisten:

| Endpunkt-Typ | Maximale Anfragegröße |
|:---|:---|
| Messages API | 32 MB |
| Token Counting API | 32 MB |
| [Batch API](/docs/de/build-with-claude/batch-processing) | 256 MB |
| [Files API](/docs/de/build-with-claude/files) | 500 MB |

Wenn Sie diese Limits überschreiten, erhalten Sie einen 413 `request_too_large`-Fehler. Der Fehler wird von Cloudflare zurückgegeben, bevor die Anfrage unsere API-Server erreicht.

## Fehlerformen

Fehler werden immer als JSON zurückgegeben, mit einem Top-Level-`error`-Objekt, das immer einen `type`- und `message`-Wert enthält. Die Antwort enthält auch ein `request_id`-Feld für einfachere Verfolgung und Debugging. Zum Beispiel:

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

In Übereinstimmung mit unserer [Versionierungsrichtlinie](/docs/de/api/versioning) können wir die Werte innerhalb dieser Objekte erweitern, und es ist möglich, dass die `type`-Werte mit der Zeit wachsen werden.

## Request-ID

Jede API-Antwort enthält einen eindeutigen `request-id`-Header. Dieser Header enthält einen Wert wie `req_018EeWyXxfu5pfWkrYcMdjWG`. Wenn Sie den Support bezüglich einer spezifischen Anfrage kontaktieren, fügen Sie bitte diese ID bei, um uns zu helfen, Ihr Problem schnell zu lösen.

Unsere offiziellen SDKs stellen diesen Wert als Eigenschaft auf Top-Level-Antwortobjekten bereit, die den Wert des `request-id`-Headers enthalten:

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

## Lange Anfragen

<Warning>
 Wir empfehlen dringend die Verwendung der [Streaming Messages API](/docs/de/build-with-claude/streaming) oder [Message Batches API](/docs/de/api/creating-message-batches) für lang laufende Anfragen, insbesondere solche über 10 Minuten.
</Warning>

Wir empfehlen nicht, große `max_tokens`-Werte zu setzen, ohne unsere [Streaming Messages API](/docs/de/build-with-claude/streaming)
oder [Message Batches API](/docs/de/api/creating-message-batches) zu verwenden:

- Einige Netzwerke können inaktive Verbindungen nach einer variablen Zeitspanne trennen, was
dazu führen kann, dass die Anfrage fehlschlägt oder eine Zeitüberschreitung auftritt, ohne eine Antwort von Anthropic zu erhalten.
- Netzwerke unterscheiden sich in der Zuverlässigkeit; unsere [Message Batches API](/docs/de/api/creating-message-batches) kann Ihnen helfen,
das Risiko von Netzwerkproblemen zu verwalten, indem sie es Ihnen ermöglicht, Ergebnisse abzufragen, anstatt eine ununterbrochene Netzwerkverbindung zu erfordern.

Wenn Sie eine direkte API-Integration erstellen, sollten Sie sich bewusst sein, dass das Setzen eines [TCP-Socket-Keep-Alive](https://tldp.org/HOWTO/TCP-Keepalive-HOWTO/programming.html) die Auswirkungen von Zeitüberschreitungen inaktiver Verbindungen in einigen Netzwerken reduzieren kann.

Unsere [SDKs](/docs/de/api/client-sdks) werden validieren, dass Ihre nicht-streamenden Messages API-Anfragen nicht erwartungsgemäß eine 10-Minuten-Zeitüberschreitung überschreiten werden und
werden auch eine Socket-Option für TCP-Keep-Alive setzen.