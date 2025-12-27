# Compatibilità con OpenAI SDK

Anthropic fornisce un livello di compatibilità che ti consente di utilizzare OpenAI SDK per testare l'API Claude. Con pochi cambiamenti al codice, puoi valutare rapidamente le capacità dei modelli Anthropic.

---

<Note>
Questo livello di compatibilità è principalmente destinato a testare e confrontare le capacità dei modelli e non è considerato una soluzione a lungo termine o pronta per la produzione per la maggior parte dei casi d'uso. Sebbene intendiamo mantenerlo completamente funzionale e non apportare modifiche che interrompano la compatibilità, la nostra priorità è l'affidabilità e l'efficacia dell'[API Claude](/docs/it/api/overview).

Per ulteriori informazioni sulle limitazioni di compatibilità note, consulta [Importanti limitazioni di compatibilità con OpenAI](#important-openai-compatibility-limitations).

Se riscontri problemi con la funzione di compatibilità OpenAI SDK, faccelo sapere [qui](https://forms.gle/oQV4McQNiuuNbz9n8).
</Note>

<Tip>
Per la migliore esperienza e accesso al set completo di funzionalità dell'API Claude ([elaborazione PDF](/docs/it/build-with-claude/pdf-support), [citazioni](/docs/it/build-with-claude/citations), [pensiero esteso](/docs/it/build-with-claude/extended-thinking) e [memorizzazione nella cache dei prompt](/docs/it/build-with-claude/prompt-caching)), consigliamo di utilizzare l'[API Claude](/docs/it/api/overview) nativa.
</Tip>

## Iniziare con OpenAI SDK

Per utilizzare la funzione di compatibilità OpenAI SDK, dovrai:

1. Utilizzare un OpenAI SDK ufficiale
2. Modificare quanto segue
   * Aggiorna il tuo URL di base per puntare all'API Claude
   * Sostituisci la tua chiave API con una [chiave API Claude](/settings/keys)
   * Aggiorna il nome del tuo modello per utilizzare un [modello Claude](/docs/it/about-claude/models/overview)
3. Consulta la documentazione di seguito per scoprire quali funzionalità sono supportate

### Esempio di avvio rapido

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

## Importanti limitazioni di compatibilità con OpenAI

#### Comportamento dell'API

Ecco le differenze più sostanziali rispetto all'utilizzo di OpenAI:

* Il parametro `strict` per il function calling viene ignorato, il che significa che il JSON di utilizzo dello strumento non è garantito che segua lo schema fornito. Per la conformità dello schema garantita, utilizza l'[API Claude nativa con Output Strutturati](/docs/it/build-with-claude/structured-outputs).
* L'input audio non è supportato; verrà semplicemente ignorato e rimosso dall'input
* La memorizzazione nella cache dei prompt non è supportata, ma è supportata nell'[SDK Anthropic](/docs/it/api/client-sdks)
* I messaggi di sistema/sviluppatore vengono sollevati e concatenati all'inizio della conversazione, poiché Anthropic supporta solo un singolo messaggio di sistema iniziale.

La maggior parte dei campi non supportati viene ignorata silenziosamente piuttosto che produrre errori. Tutti questi sono documentati di seguito.

#### Considerazioni sulla qualità dell'output

Se hai fatto molti aggiustamenti al tuo prompt, è probabile che sia ben sintonizzato specificamente per OpenAI. Considera di utilizzare il nostro [miglioratore di prompt nella Claude Console](/dashboard) come buon punto di partenza.

#### Sollevamento dei messaggi di sistema / sviluppatore

La maggior parte degli input all'SDK OpenAI si mappano chiaramente direttamente ai parametri dell'API Anthropic, ma una differenza distinta è la gestione dei prompt di sistema / sviluppatore. Questi due prompt possono essere inseriti in tutta una conversazione di chat tramite OpenAI. Poiché Anthropic supporta solo un messaggio di sistema iniziale, prendiamo tutti i messaggi di sistema/sviluppatore e li concateniamo insieme con una singola nuova riga (`\n`) tra di loro. Questa stringa completa viene quindi fornita come un singolo messaggio di sistema all'inizio dei messaggi.

#### Supporto del pensiero esteso

Puoi abilitare le capacità di [pensiero esteso](/docs/it/build-with-claude/extended-thinking) aggiungendo il parametro `thinking`. Sebbene questo migliorerà il ragionamento di Claude per compiti complessi, OpenAI SDK non restituirà il processo di pensiero dettagliato di Claude. Per le funzionalità complete di pensiero esteso, incluso l'accesso all'output del ragionamento passo dopo passo di Claude, utilizza l'API Claude nativa.

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

## Limiti di velocità

I limiti di velocità seguono i [limiti standard](/docs/it/api/rate-limits) di Anthropic per l'endpoint `/v1/messages`.

## Supporto dettagliato dell'API compatibile con OpenAI
### Campi di richiesta
#### Campi semplici
| Campo | Stato del supporto |
|--------|----------------|
| `model` | Utilizza nomi di modelli Claude |
| `max_tokens` | Completamente supportato |
| `max_completion_tokens` | Completamente supportato |
| `stream` | Completamente supportato |
| `stream_options` | Completamente supportato |
| `top_p` | Completamente supportato |
| `parallel_tool_calls` | Completamente supportato |
| `stop` | Tutte le sequenze di stop non-whitespace funzionano |
| `temperature` | Tra 0 e 1 (incluso). I valori maggiori di 1 sono limitati a 1. |
| `n` | Deve essere esattamente 1 |
| `logprobs` | Ignorato |
| `metadata` | Ignorato |
| `response_format` | Ignorato. Per l'output JSON, utilizza [Output Strutturati](/docs/it/build-with-claude/structured-outputs) con l'API Claude nativa |
| `prediction` | Ignorato |
| `presence_penalty` | Ignorato |
| `frequency_penalty` | Ignorato |
| `seed` | Ignorato |
| `service_tier` | Ignorato |
| `audio` | Ignorato |
| `logit_bias` | Ignorato |
| `store` | Ignorato |
| `user` | Ignorato |
| `modalities` | Ignorato |
| `top_logprobs` | Ignorato |
| `reasoning_effort` | Ignorato |

#### Campi `tools` / `functions`
<section title="Mostra campi">

<Tabs>
<Tab title="Tools">
Campi `tools[n].function`
| Campo        | Stato del supporto         |
|--------------|-----------------|
| `name`       | Completamente supportato |
| `description`| Completamente supportato |
| `parameters` | Completamente supportato |
| `strict`     | Ignorato. Utilizza [Output Strutturati](/docs/it/build-with-claude/structured-outputs) con l'API Claude nativa per la convalida dello schema rigorosa |
</Tab>
<Tab title="Functions">

Campi `functions[n]`
<Info>
OpenAI ha deprecato il campo `functions` e suggerisce di utilizzare `tools` invece.
</Info>
| Campo        | Stato del supporto         |
|--------------|-----------------|
| `name`       | Completamente supportato |
| `description`| Completamente supportato |
| `parameters` | Completamente supportato |
| `strict`     | Ignorato. Utilizza [Output Strutturati](/docs/it/build-with-claude/structured-outputs) con l'API Claude nativa per la convalida dello schema rigorosa |
</Tab>
</Tabs>

</section>

#### Campi dell'array `messages`
<section title="Mostra campi">

<Tabs>
<Tab title="Ruolo sviluppatore">
Campi per `messages[n].role == "developer"`
<Info>
I messaggi dello sviluppatore vengono sollevati all'inizio della conversazione come parte del messaggio di sistema iniziale
</Info>
| Campo | Stato del supporto |
|-------|---------|
| `content` | Completamente supportato, ma sollevato |
| `name` | Ignorato |

</Tab>
<Tab title="Ruolo sistema">
Campi per `messages[n].role == "system"`

<Info>
I messaggi di sistema vengono sollevati all'inizio della conversazione come parte del messaggio di sistema iniziale
</Info>
| Campo | Stato del supporto |
|-------|---------|
| `content` | Completamente supportato, ma sollevato |
| `name` | Ignorato |

</Tab>
<Tab title="Ruolo utente">
Campi per `messages[n].role == "user"`

| Campo | Variante | Sottocampo | Stato del supporto |
|-------|---------|-----------|----------------|
| `content` | `string` | | Completamente supportato |
| | `array`, `type == "text"` | | Completamente supportato |
| | `array`, `type == "image_url"` | `url` | Completamente supportato |
| | | `detail` | Ignorato |
| | `array`, `type == "input_audio"` | | Ignorato |
| | `array`, `type == "file"` | | Ignorato |
| `name` | | | Ignorato |

</Tab>

<Tab title="Ruolo assistente">
Campi per `messages[n].role == "assistant"`
| Campo | Variante | Stato del supporto |
|-------|---------|----------------|
| `content` | `string` | Completamente supportato |
| | `array`, `type == "text"` | Completamente supportato |
| | `array`, `type == "refusal"` | Ignorato |
| `tool_calls` | | Completamente supportato |
| `function_call` | | Completamente supportato |
| `audio` | | Ignorato |
| `refusal` | | Ignorato |

</Tab>

<Tab title="Ruolo strumento">
Campi per `messages[n].role == "tool"`
| Campo | Variante | Stato del supporto |
|-------|---------|----------------|
| `content` | `string` | Completamente supportato |
| | `array`, `type == "text"` | Completamente supportato |
| `tool_call_id` | | Completamente supportato |
| `tool_choice` | | Completamente supportato |
| `name` | | Ignorato |
</Tab>

<Tab title="Ruolo funzione">
Campi per `messages[n].role == "function"`
| Campo | Variante | Stato del supporto |
|-------|---------|----------------|
| `content` | `string` | Completamente supportato |
| | `array`, `type == "text"` | Completamente supportato |
| `tool_choice` | | Completamente supportato |
| `name` | | Ignorato |
</Tab>
</Tabs>

</section>

### Campi di risposta

| Campo | Stato del supporto |
|---------------------------|----------------|
| `id` | Completamente supportato |
| `choices[]` | Avrà sempre una lunghezza di 1 |
| `choices[].finish_reason` | Completamente supportato |
| `choices[].index` | Completamente supportato |
| `choices[].message.role` | Completamente supportato |
| `choices[].message.content` | Completamente supportato |
| `choices[].message.tool_calls` | Completamente supportato |
| `object` | Completamente supportato |
| `created` | Completamente supportato |
| `model` | Completamente supportato |
| `finish_reason` | Completamente supportato |
| `content` | Completamente supportato |
| `usage.completion_tokens` | Completamente supportato |
| `usage.prompt_tokens` | Completamente supportato |
| `usage.total_tokens` | Completamente supportato |
| `usage.completion_tokens_details` | Sempre vuoto |
| `usage.prompt_tokens_details` | Sempre vuoto |
| `choices[].message.refusal` | Sempre vuoto |
| `choices[].message.audio` | Sempre vuoto |
| `logprobs` | Sempre vuoto |
| `service_tier` | Sempre vuoto |
| `system_fingerprint` | Sempre vuoto |

### Compatibilità dei messaggi di errore

Il livello di compatibilità mantiene formati di errore coerenti con l'API OpenAI. Tuttavia, i messaggi di errore dettagliati non saranno equivalenti. Consigliamo di utilizzare i messaggi di errore solo per la registrazione e il debug.

### Compatibilità dell'intestazione

Sebbene OpenAI SDK gestisca automaticamente le intestazioni, ecco l'elenco completo delle intestazioni supportate dall'API Claude per gli sviluppatori che hanno bisogno di lavorarci direttamente.

| Intestazione | Stato del supporto |
|---------|----------------|
| `x-ratelimit-limit-requests` | Completamente supportato |
| `x-ratelimit-limit-tokens` | Completamente supportato |
| `x-ratelimit-remaining-requests` | Completamente supportato |
| `x-ratelimit-remaining-tokens` | Completamente supportato |
| `x-ratelimit-reset-requests` | Completamente supportato |
| `x-ratelimit-reset-tokens` | Completamente supportato |
| `retry-after` | Completamente supportato |
| `request-id` | Completamente supportato |
| `openai-version` | Sempre `2020-10-01` |
| `authorization` | Completamente supportato |
| `openai-processing-ms` | Sempre vuoto |