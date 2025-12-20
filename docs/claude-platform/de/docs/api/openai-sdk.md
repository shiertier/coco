# OpenAI SDK-Kompatibilität

Anthropic bietet eine Kompatibilitätsschicht, mit der Sie das OpenAI SDK verwenden können, um die Claude API zu testen. Mit einigen wenigen Codeänderungen können Sie die Funktionen von Anthropic-Modellen schnell evaluieren.

---

<Note>
Diese Kompatibilitätsschicht ist in erster Linie zum Testen und Vergleichen von Modellfunktionen gedacht und wird für die meisten Anwendungsfälle nicht als langfristige oder produktionsreife Lösung betrachtet. Obwohl wir beabsichtigen, sie vollständig funktionsfähig zu halten und keine Breaking Changes vorzunehmen, liegt unsere Priorität auf der Zuverlässigkeit und Effektivität der [Claude API](/docs/de/api/overview).

Weitere Informationen zu bekannten Kompatibilitätsbeschränkungen finden Sie unter [Wichtige OpenAI-Kompatibilitätsbeschränkungen](#important-openai-compatibility-limitations).

Wenn Sie Probleme mit der OpenAI SDK-Kompatibilitätsfunktion haben, teilen Sie uns dies bitte [hier](https://forms.gle/oQV4McQNiuuNbz9n8) mit.
</Note>

<Tip>
Für die beste Erfahrung und Zugriff auf den vollständigen Funktionsumfang der Claude API ([PDF-Verarbeitung](/docs/de/build-with-claude/pdf-support), [Zitate](/docs/de/build-with-claude/citations), [erweitertes Denken](/docs/de/build-with-claude/extended-thinking) und [Prompt-Caching](/docs/de/build-with-claude/prompt-caching)) empfehlen wir die Verwendung der nativen [Claude API](/docs/de/api/overview).
</Tip>

## Erste Schritte mit dem OpenAI SDK

Um die OpenAI SDK-Kompatibilitätsfunktion zu verwenden, müssen Sie:

1. Ein offizielles OpenAI SDK verwenden
2. Folgendes ändern
   * Aktualisieren Sie Ihre Basis-URL, um auf die Claude API zu verweisen
   * Ersetzen Sie Ihren API-Schlüssel durch einen [Claude API-Schlüssel](/settings/keys)
   * Aktualisieren Sie Ihren Modellnamen, um ein [Claude-Modell](/docs/de/about-claude/models/overview) zu verwenden
3. Lesen Sie die Dokumentation unten durch, um zu sehen, welche Funktionen unterstützt werden

### Schnellstart-Beispiel

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

## Wichtige OpenAI-Kompatibilitätsbeschränkungen

#### API-Verhalten

Hier sind die wesentlichsten Unterschiede zur Verwendung von OpenAI:

* Der Parameter `strict` für Function Calling wird ignoriert, was bedeutet, dass das Tool-Use-JSON nicht garantiert dem bereitgestellten Schema entspricht. Für garantierte Schemakonformität verwenden Sie die native [Claude API mit strukturierten Ausgaben](/docs/de/build-with-claude/structured-outputs).
* Audio-Eingabe wird nicht unterstützt; sie wird einfach ignoriert und aus der Eingabe entfernt
* Prompt-Caching wird nicht unterstützt, ist aber in [dem Anthropic SDK](/docs/de/api/client-sdks) unterstützt
* System-/Entwicklernachrichten werden an den Anfang der Konversation verschoben und verkettet, da Anthropic nur eine einzelne anfängliche Systemnachricht unterstützt.

Die meisten nicht unterstützten Felder werden stillschweigend ignoriert, anstatt Fehler zu erzeugen. Diese sind alle unten dokumentiert.

#### Überlegungen zur Ausgabequalität

Wenn Sie viel Zeit damit verbracht haben, Ihren Prompt zu optimieren, ist es wahrscheinlich, dass er speziell auf OpenAI abgestimmt ist. Erwägen Sie, unseren [Prompt-Verbesserer in der Claude Console](/dashboard) als guten Ausgangspunkt zu verwenden.

#### System-/Entwicklernachricht-Hoisting

Die meisten Eingaben für das OpenAI SDK werden direkt auf die API-Parameter von Anthropic abgebildet, aber ein deutlicher Unterschied ist die Behandlung von System-/Entwicklerprompts. Diese beiden Prompts können in einer Chat-Konversation über OpenAI platziert werden. Da Anthropic nur eine anfängliche Systemnachricht unterstützt, nehmen wir alle System-/Entwicklernachrichten und verketten sie mit einem einzelnen Zeilenumbruch (`\n`) dazwischen. Diese vollständige Zeichenkette wird dann als einzelne Systemnachricht am Anfang der Nachrichten bereitgestellt.

#### Unterstützung für erweitertes Denken

Sie können die Funktionen des [erweiterten Denkens](/docs/de/build-with-claude/extended-thinking) aktivieren, indem Sie den Parameter `thinking` hinzufügen. Obwohl dies Claudes Argumentation für komplexe Aufgaben verbessert, gibt das OpenAI SDK Claudes detaillierten Gedankenprozess nicht zurück. Für vollständige Funktionen des erweiterten Denkens, einschließlich Zugriff auf Claudes schrittweise Ausgabe des Denkprozesses, verwenden Sie die native Claude API.

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

## Rate Limits

Rate Limits folgen den [Standardlimits](/docs/de/api/rate-limits) von Anthropic für den Endpunkt `/v1/messages`.

## Detaillierte OpenAI-kompatible API-Unterstützung
### Anfragfelder
#### Einfache Felder
| Feld | Unterstützungsstatus |
|--------|----------------|
| `model` | Claude-Modellnamen verwenden |
| `max_tokens` | Vollständig unterstützt |
| `max_completion_tokens` | Vollständig unterstützt |
| `stream` | Vollständig unterstützt |
| `stream_options` | Vollständig unterstützt |
| `top_p` | Vollständig unterstützt |
| `parallel_tool_calls` | Vollständig unterstützt |
| `stop` | Alle Stoppsequenzen ohne Leerzeichen funktionieren |
| `temperature` | Zwischen 0 und 1 (einschließlich). Werte größer als 1 werden auf 1 begrenzt. |
| `n` | Muss genau 1 sein |
| `logprobs` | Ignoriert |
| `metadata` | Ignoriert |
| `response_format` | Ignoriert. Für JSON-Ausgabe verwenden Sie [Strukturierte Ausgaben](/docs/de/build-with-claude/structured-outputs) mit der nativen Claude API |
| `prediction` | Ignoriert |
| `presence_penalty` | Ignoriert |
| `frequency_penalty` | Ignoriert |
| `seed` | Ignoriert |
| `service_tier` | Ignoriert |
| `audio` | Ignoriert |
| `logit_bias` | Ignoriert |
| `store` | Ignoriert |
| `user` | Ignoriert |
| `modalities` | Ignoriert |
| `top_logprobs` | Ignoriert |
| `reasoning_effort` | Ignoriert |

#### `tools` / `functions` Felder
<section title="Felder anzeigen">

<Tabs>
<Tab title="Tools">
`tools[n].function` Felder
| Feld        | Unterstützungsstatus         |
|--------------|-----------------|
| `name`       | Vollständig unterstützt |
| `description`| Vollständig unterstützt |
| `parameters` | Vollständig unterstützt |
| `strict`     | Ignoriert. Verwenden Sie [Strukturierte Ausgaben](/docs/de/build-with-claude/structured-outputs) mit nativer Claude API für strikte Schemavalidierung |
</Tab>
<Tab title="Functions">

`functions[n]` Felder
<Info>
OpenAI hat das Feld `functions` veraltet und schlägt vor, stattdessen `tools` zu verwenden.
</Info>
| Feld        | Unterstützungsstatus         |
|--------------|-----------------|
| `name`       | Vollständig unterstützt |
| `description`| Vollständig unterstützt |
| `parameters` | Vollständig unterstützt |
| `strict`     | Ignoriert. Verwenden Sie [Strukturierte Ausgaben](/docs/de/build-with-claude/structured-outputs) mit nativer Claude API für strikte Schemavalidierung |
</Tab>
</Tabs>

</section>

#### `messages` Array-Felder
<section title="Felder anzeigen">

<Tabs>
<Tab title="Developer-Rolle">
Felder für `messages[n].role == "developer"`
<Info>
Entwicklernachrichten werden an den Anfang der Konversation als Teil der anfänglichen Systemnachricht verschoben
</Info>
| Feld | Unterstützungsstatus |
|-------|---------|
| `content` | Vollständig unterstützt, aber verschoben |
| `name` | Ignoriert |

</Tab>
<Tab title="System-Rolle">
Felder für `messages[n].role == "system"`

<Info>
Systemnachrichten werden an den Anfang der Konversation als Teil der anfänglichen Systemnachricht verschoben
</Info>
| Feld | Unterstützungsstatus |
|-------|---------|
| `content` | Vollständig unterstützt, aber verschoben |
| `name` | Ignoriert |

</Tab>
<Tab title="User-Rolle">
Felder für `messages[n].role == "user"`

| Feld | Variante | Unterfeld | Unterstützungsstatus |
|-------|---------|-----------|----------------|
| `content` | `string` | | Vollständig unterstützt |
| | `array`, `type == "text"` | | Vollständig unterstützt |
| | `array`, `type == "image_url"` | `url` | Vollständig unterstützt |
| | | `detail` | Ignoriert |
| | `array`, `type == "input_audio"` | | Ignoriert |
| | `array`, `type == "file"` | | Ignoriert |
| `name` | | | Ignoriert |

</Tab>

<Tab title="Assistant-Rolle">
Felder für `messages[n].role == "assistant"`
| Feld | Variante | Unterstützungsstatus |
|-------|---------|----------------|
| `content` | `string` | Vollständig unterstützt |
| | `array`, `type == "text"` | Vollständig unterstützt |
| | `array`, `type == "refusal"` | Ignoriert |
| `tool_calls` | | Vollständig unterstützt |
| `function_call` | | Vollständig unterstützt |
| `audio` | | Ignoriert |
| `refusal` | | Ignoriert |

</Tab>

<Tab title="Tool-Rolle">
Felder für `messages[n].role == "tool"`
| Feld | Variante | Unterstützungsstatus |
|-------|---------|----------------|
| `content` | `string` | Vollständig unterstützt |
| | `array`, `type == "text"` | Vollständig unterstützt |
| `tool_call_id` | | Vollständig unterstützt |
| `tool_choice` | | Vollständig unterstützt |
| `name` | | Ignoriert |
</Tab>

<Tab title="Function-Rolle">
Felder für `messages[n].role == "function"`
| Feld | Variante | Unterstützungsstatus |
|-------|---------|----------------|
| `content` | `string` | Vollständig unterstützt |
| | `array`, `type == "text"` | Vollständig unterstützt |
| `tool_choice` | | Vollständig unterstützt |
| `name` | | Ignoriert |
</Tab>
</Tabs>

</section>

### Antwortfelder

| Feld | Unterstützungsstatus |
|---------------------------|----------------|
| `id` | Vollständig unterstützt |
| `choices[]` | Hat immer eine Länge von 1 |
| `choices[].finish_reason` | Vollständig unterstützt |
| `choices[].index` | Vollständig unterstützt |
| `choices[].message.role` | Vollständig unterstützt |
| `choices[].message.content` | Vollständig unterstützt |
| `choices[].message.tool_calls` | Vollständig unterstützt |
| `object` | Vollständig unterstützt |
| `created` | Vollständig unterstützt |
| `model` | Vollständig unterstützt |
| `finish_reason` | Vollständig unterstützt |
| `content` | Vollständig unterstützt |
| `usage.completion_tokens` | Vollständig unterstützt |
| `usage.prompt_tokens` | Vollständig unterstützt |
| `usage.total_tokens` | Vollständig unterstützt |
| `usage.completion_tokens_details` | Immer leer |
| `usage.prompt_tokens_details` | Immer leer |
| `choices[].message.refusal` | Immer leer |
| `choices[].message.audio` | Immer leer |
| `logprobs` | Immer leer |
| `service_tier` | Immer leer |
| `system_fingerprint` | Immer leer |

### Fehlermeldekompatibilität

Die Kompatibilitätsschicht behält konsistente Fehlerformate mit der OpenAI API bei. Die detaillierten Fehlermeldungen werden jedoch nicht gleichwertig sein. Wir empfehlen, die Fehlermeldungen nur zum Protokollieren und Debuggen zu verwenden.

### Header-Kompatibilität

Obwohl das OpenAI SDK Header automatisch verwaltet, finden Sie hier die vollständige Liste der Header, die von der Claude API unterstützt werden, für Entwickler, die direkt damit arbeiten müssen.

| Header | Unterstützungsstatus |
|---------|----------------|
| `x-ratelimit-limit-requests` | Vollständig unterstützt |
| `x-ratelimit-limit-tokens` | Vollständig unterstützt |
| `x-ratelimit-remaining-requests` | Vollständig unterstützt |
| `x-ratelimit-remaining-tokens` | Vollständig unterstützt |
| `x-ratelimit-reset-requests` | Vollständig unterstützt |
| `x-ratelimit-reset-tokens` | Vollständig unterstützt |
| `retry-after` | Vollständig unterstützt |
| `request-id` | Vollständig unterstützt |
| `openai-version` | Immer `2020-10-01` |
| `authorization` | Vollständig unterstützt |
| `openai-processing-ms` | Immer leer |