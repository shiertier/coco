# Come implementare l'uso degli strumenti

Guida completa all'implementazione dell'uso degli strumenti con Claude, inclusi best practice, esempi e il tool runner.

---

## Scelta di un modello

Consigliamo di utilizzare il modello Claude Sonnet (4.5) o Claude Opus (4.1) più recente per strumenti complessi e query ambigue; gestiscono meglio più strumenti e cercano chiarimenti quando necessario.

Utilizza i modelli Claude Haiku per strumenti semplici, ma tieni presente che potrebbero dedurre parametri mancanti.

<Tip>
Se utilizzi Claude con l'uso degli strumenti e il pensiero esteso, consulta la nostra guida [qui](/docs/it/build-with-claude/extended-thinking) per ulteriori informazioni.
</Tip>

## Specifica degli strumenti client

Gli strumenti client (sia definiti da Anthropic che definiti dall'utente) sono specificati nel parametro `tools` di primo livello della richiesta API. Ogni definizione di strumento include:

| Parametro      | Descrizione                                                                                         |
| :------------- | :-------------------------------------------------------------------------------------------------- |
| `name`         | Il nome dello strumento. Deve corrispondere all'espressione regolare `^[a-zA-Z0-9_-]{1,64}$`.                                 |
| `description`  | Una descrizione dettagliata in testo semplice di cosa fa lo strumento, quando deve essere utilizzato e come si comporta. |
| `input_schema` | Un oggetto [JSON Schema](https://json-schema.org/) che definisce i parametri previsti per lo strumento.     |
| `input_examples` | (Facoltativo, beta) Un array di oggetti di input di esempio per aiutare Claude a capire come utilizzare lo strumento. Vedi [Fornire esempi di uso degli strumenti](#providing-tool-use-examples). |

<section title="Esempio di definizione di strumento semplice">

```json JSON
{
  "name": "get_weather",
  "description": "Get the current weather in a given location",
  "input_schema": {
    "type": "object",
    "properties": {
      "location": {
        "type": "string",
        "description": "The city and state, e.g. San Francisco, CA"
      },
      "unit": {
        "type": "string",
        "enum": ["celsius", "fahrenheit"],
        "description": "The unit of temperature, either 'celsius' or 'fahrenheit'"
      }
    },
    "required": ["location"]
  }
}
```

Questo strumento, denominato `get_weather`, prevede un oggetto di input con una stringa `location` obbligatoria e una stringa `unit` facoltativa che deve essere "celsius" o "fahrenheit".

</section>

### Prompt di sistema per l'uso degli strumenti

Quando chiami l'API Claude con il parametro `tools`, costruiamo un prompt di sistema speciale dalle definizioni degli strumenti, dalla configurazione degli strumenti e da qualsiasi prompt di sistema specificato dall'utente. Il prompt costruito è progettato per istruire il modello a utilizzare gli strumenti specificati e fornire il contesto necessario affinché lo strumento funzioni correttamente:

```
In this environment you have access to a set of tools you can use to answer the user's question.
{{ FORMATTING INSTRUCTIONS }}
String and scalar parameters should be specified as is, while lists and objects should use JSON format. Note that spaces for string values are not stripped. The output is not expected to be valid XML and is parsed with regular expressions.
Here are the functions available in JSONSchema format:
{{ TOOL DEFINITIONS IN JSON SCHEMA }}
{{ USER SYSTEM PROMPT }}
{{ TOOL CONFIGURATION }}
```

### Best practice per le definizioni degli strumenti

Per ottenere le migliori prestazioni da Claude quando utilizzi gli strumenti, segui queste linee guida:

- **Fornisci descrizioni estremamente dettagliate.** Questo è di gran lunga il fattore più importante nelle prestazioni degli strumenti. Le tue descrizioni dovrebbero spiegare ogni dettaglio dello strumento, incluso:
  - Cosa fa lo strumento
  - Quando deve essere utilizzato (e quando non deve)
  - Cosa significa ogni parametro e come influisce sul comportamento dello strumento
  - Eventuali avvertenze o limitazioni importanti, come quali informazioni lo strumento non restituisce se il nome dello strumento non è chiaro. Più contesto puoi fornire a Claude sui tuoi strumenti, meglio sarà in grado di decidere quando e come utilizzarli. Mira ad almeno 3-4 frasi per descrizione dello strumento, di più se lo strumento è complesso.
- **Dai priorità alle descrizioni, ma considera l'utilizzo di `input_examples` per strumenti complessi.** Le descrizioni chiare sono più importanti, ma per strumenti con input complessi, oggetti annidati o parametri sensibili al formato, puoi utilizzare il campo `input_examples` (beta) per fornire esempi convalidati dallo schema. Vedi [Fornire esempi di uso degli strumenti](#providing-tool-use-examples) per i dettagli.

<section title="Esempio di una buona descrizione dello strumento">

```json JSON
{
  "name": "get_stock_price",
  "description": "Retrieves the current stock price for a given ticker symbol. The ticker symbol must be a valid symbol for a publicly traded company on a major US stock exchange like NYSE or NASDAQ. The tool will return the latest trade price in USD. It should be used when the user asks about the current or most recent price of a specific stock. It will not provide any other information about the stock or company.",
  "input_schema": {
    "type": "object",
    "properties": {
      "ticker": {
        "type": "string",
        "description": "The stock ticker symbol, e.g. AAPL for Apple Inc."
      }
    },
    "required": ["ticker"]
  }
}
```

</section>

<section title="Esempio di descrizione dello strumento scadente">

```json JSON
{
  "name": "get_stock_price",
  "description": "Gets the stock price for a ticker.",
  "input_schema": {
    "type": "object",
    "properties": {
      "ticker": {
        "type": "string"
      }
    },
    "required": ["ticker"]
  }
}
```

</section>

La buona descrizione spiega chiaramente cosa fa lo strumento, quando utilizzarlo, quali dati restituisce e cosa significa il parametro `ticker`. La descrizione scadente è troppo breve e lascia Claude con molte domande aperte sul comportamento e l'utilizzo dello strumento.

## Fornire esempi di uso degli strumenti

Puoi fornire esempi concreti di input di strumenti validi per aiutare Claude a capire come utilizzare i tuoi strumenti in modo più efficace. Questo è particolarmente utile per strumenti complessi con oggetti annidati, parametri facoltativi o input sensibili al formato.

<Info>
Gli esempi di uso degli strumenti sono una funzione beta. Includi l'appropriato [header beta](/docs/it/api/beta-headers) per il tuo provider:

| Provider | Header beta | Modelli supportati |
|----------|-------------|------------------|
| Claude API,<br/>Microsoft Foundry | `advanced-tool-use-2025-11-20` | Tutti i modelli |
| Vertex AI,<br/>Amazon Bedrock | `tool-examples-2025-10-29` | Solo Claude Opus 4.5 |
</Info>

### Utilizzo di base

Aggiungi un campo `input_examples` facoltativo alla definizione dello strumento con un array di oggetti di input di esempio. Ogni esempio deve essere valido secondo l'`input_schema` dello strumento:

<CodeGroup>
```python Python
import anthropic

client = anthropic.Anthropic()

response = client.messages.create(
    model="claude-sonnet-4-5-20250929",
    max_tokens=1024,
    betas=["advanced-tool-use-2025-11-20"],
    tools=[
        {
            "name": "get_weather",
            "description": "Get the current weather in a given location",
            "input_schema": {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"],
                        "description": "The unit of temperature"
                    }
                },
                "required": ["location"]
            },
            "input_examples": [
                {
                    "location": "San Francisco, CA",
                    "unit": "fahrenheit"
                },
                {
                    "location": "Tokyo, Japan",
                    "unit": "celsius"
                },
                {
                    "location": "New York, NY"  # 'unit' is optional
                }
            ]
        }
    ],
    messages=[
        {"role": "user", "content": "What's the weather like in San Francisco?"}
    ]
)
```

```typescript TypeScript
import Anthropic from "@anthropic-ai/sdk";

const client = new Anthropic();

const response = await client.messages.create({
  model: "claude-sonnet-4-5-20250929",
  max_tokens: 1024,
  betas: ["advanced-tool-use-2025-11-20"],
  tools: [
    {
      name: "get_weather",
      description: "Get the current weather in a given location",
      input_schema: {
        type: "object",
        properties: {
          location: {
            type: "string",
            description: "The city and state, e.g. San Francisco, CA",
          },
          unit: {
            type: "string",
            enum: ["celsius", "fahrenheit"],
            description: "The unit of temperature",
          },
        },
        required: ["location"],
      },
      input_examples: [
        {
          location: "San Francisco, CA",
          unit: "fahrenheit",
        },
        {
          location: "Tokyo, Japan",
          unit: "celsius",
        },
        {
          location: "New York, NY",
          // Demonstrates that 'unit' is optional
        },
      ],
    },
  ],
  messages: [{ role: "user", content: "What's the weather like in San Francisco?" }],
});
```
</CodeGroup>

Gli esempi sono inclusi nel prompt insieme al tuo schema dello strumento, mostrando a Claude modelli concreti per chiamate di strumenti ben formate. Questo aiuta Claude a capire quando includere parametri facoltativi, quali formati utilizzare e come strutturare input complessi.

### Requisiti e limitazioni

- **Convalida dello schema** - Ogni esempio deve essere valido secondo l'`input_schema` dello strumento. Gli esempi non validi restituiscono un errore 400
- **Non supportato per strumenti lato server** - Solo gli strumenti definiti dall'utente possono avere esempi di input
- **Costo dei token** - Gli esempi si aggiungono ai token di prompt: ~20-50 token per esempi semplici, ~100-200 token per oggetti annidati complessi

## Tool runner (beta)

Il tool runner fornisce una soluzione pronta all'uso per l'esecuzione di strumenti con Claude. Invece di gestire manualmente le chiamate di strumenti, i risultati degli strumenti e la gestione della conversazione, il tool runner automaticamente:

- Esegue gli strumenti quando Claude li chiama
- Gestisce il ciclo di richiesta/risposta
- Gestisce lo stato della conversazione
- Fornisce sicurezza dei tipi e convalida

Consigliamo di utilizzare il tool runner per la maggior parte delle implementazioni di uso degli strumenti.

<Note>
Il tool runner è attualmente in beta ed è disponibile negli SDK [Python](https://github.com/anthropics/anthropic-sdk-python/blob/main/tools.md), [TypeScript](https://github.com/anthropics/anthropic-sdk-typescript/blob/main/helpers.md#tool-helpers) e [Ruby](https://github.com/anthropics/anthropic-sdk-ruby/blob/main/helpers.md#3-auto-looping-tool-runner-beta).
</Note>

<Tip>
**Gestione automatica del contesto con compattazione**

Il tool runner supporta la [compattazione](/docs/it/build-with-claude/context-editing#client-side-compaction-sdk) automatica, che genera riepiloghi quando l'utilizzo dei token supera una soglia. Questo consente ai compiti agentivi di lunga durata di continuare oltre i limiti della finestra di contesto.
</Tip>

<Tabs>
<Tab title="Python">

### Utilizzo di base

Utilizza il decoratore `@beta_tool` per definire gli strumenti e `client.beta.messages.tool_runner()` per eseguirli.

<Note>
Se stai utilizzando il client asincrono, sostituisci `@beta_tool` con `@beta_async_tool` e definisci la funzione con `async def`.
</Note>

```python
import anthropic
import json
from anthropic import beta_tool

# Initialize client
client = anthropic.Anthropic()

# Define tools using the decorator
@beta_tool
def get_weather(location: str, unit: str = "fahrenheit") -> str:
    """Get the current weather in a given location.

    Args:
        location: The city and state, e.g. San Francisco, CA
        unit: Temperature unit, either 'celsius' or 'fahrenheit'
    """
    # In a full implementation, you'd call a weather API here
    return json.dumps({"temperature": "20°C", "condition": "Sunny"})

@beta_tool
def calculate_sum(a: int, b: int) -> str:
    """Add two numbers together.

    Args:
        a: First number
        b: Second number
    """
    return str(a + b)

# Use the tool runner
runner = client.beta.messages.tool_runner(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    tools=[get_weather, calculate_sum],
    messages=[
        {"role": "user", "content": "What's the weather like in Paris? Also, what's 15 + 27?"}
    ]
)
for message in runner:
    print(message.content[0].text)
```

La funzione decorata deve restituire un blocco di contenuto o un array di blocchi di contenuto, inclusi blocchi di testo, immagini o documenti. Questo consente agli strumenti di restituire risposte ricche e multimodali. Le stringhe restituite verranno convertite in un blocco di contenuto di testo.
Se desideri restituire un oggetto JSON strutturato a Claude, codificalo in una stringa JSON prima di restituirlo. I numeri, i booleani o altri primitivi non stringa devono anche essere convertiti in stringhe.

Il decoratore `@beta_tool` ispezionerà gli argomenti della funzione e la docstring per estrarre una rappresentazione dello schema json della funzione data, nell'esempio sopra `calculate_sum` verrà trasformato in:

```json
{
  "name": "calculate_sum",
  "description": "Adds two integers together.",
  "input_schema": {
    "additionalProperties": false,
    "properties": {
      "left": {
        "description": "The first integer to add.",
        "title": "Left",
        "type": "integer"
      },
      "right": {
        "description": "The second integer to add.",
        "title": "Right",
        "type": "integer"
      }
    },
    "required": ["left", "right"],
    "type": "object"
  }
}
```

### Iterazione sul tool runner

Il tool runner restituito da `tool_runner()` è un iterabile, che puoi iterare con un ciclo `for`. Questo è spesso indicato come un "tool call loop".
Ogni iterazione del ciclo produce un messaggio restituito da Claude.

Dopo che il tuo codice ha avuto la possibilità di elaborare il messaggio corrente all'interno del ciclo, il tool runner controllerà il messaggio per vedere se Claude ha richiesto un uso dello strumento. Se è così, chiamerà lo strumento e invierà automaticamente il risultato dello strumento a Claude, quindi produrrà il messaggio successivo da Claude per iniziare l'iterazione successiva del tuo ciclo.

Puoi terminare il ciclo in qualsiasi iterazione con una semplice istruzione `break`. Il tool runner continuerà il ciclo fino a quando Claude non restituisce un messaggio senza un uso dello strumento.

Se non ti interessa dei messaggi intermedi, invece di utilizzare un ciclo, puoi chiamare il metodo `until_done()`, che restituirà il messaggio finale da Claude:

```python
runner = client.beta.messages.tool_runner(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    tools=[get_weather, calculate_sum],
    messages=[
        {"role": "user", "content": "What's the weather like in Paris? Also, what's 15 + 27?"}
    ]
)
final_message = runner.until_done()
print(final_message.content[0].text)
```

### Utilizzo avanzato

All'interno del ciclo, hai la possibilità di personalizzare completamente la prossima richiesta del tool runner all'API Messages.
Il metodo `runner.generate_tool_call_response()` chiamerà lo strumento (se Claude ha attivato un uso dello strumento) e ti darà accesso al risultato dello strumento che verrà inviato all'API Messages.
I metodi `runner.set_messages_params()` e `runner.append_messages()` ti consentono di modificare i parametri per la prossima richiesta dell'API Messages.

```python
runner = client.beta.messages.tool_runner(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    tools=[get_weather],
    messages=[{"role": "user", "content": "What's the weather in San Francisco?"}]
)
for message in runner:
    # Get the tool response that will be sent
    tool_response = runner.generate_tool_call_response()

    # Customize the next request
    runner.set_messages_params(lambda params: {
        **params,
        "max_tokens": 2048  # Increase tokens for next request
    })

    # Or add additional messages
    runner.append_messages(
        {"role": "user", "content": "Please be concise in your response."}
    )
```

### Streaming

Quando abiliti lo streaming con `stream=True`, ogni valore emesso dal tool runner è un `BetaMessageStream` come restituito da `anthropic.messages.stream()`. Il `BetaMessageStream` è esso stesso un iterabile che produce eventi di streaming dall'API Messages.

Puoi utilizzare `message_stream.get_final_message()` per lasciare che l'SDK esegua l'accumulo degli eventi di streaming nel messaggio finale per te.

```python
runner = client.beta.messages.tool_runner(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    tools=[calculate_sum],
    messages=[{"role": "user", "content": "What is 15 + 27?"}],
    stream=True
)

# When streaming, the runner returns BetaMessageStream
for message_stream in runner:
    for event in message_stream:
        print('event:', event)
    print('message:', message_stream.get_final_message())

print(runner.until_done())
```

</Tab>
<Tab title="TypeScript (Zod)">

### Utilizzo di base

Utilizza `betaZodTool()` per definizioni di strumenti type-safe con convalida Zod (richiede Zod 3.25.0 o superiore).

```typescript
import { Anthropic } from '@anthropic-ai/sdk';
import { betaZodTool, betaTool } from '@anthropic-ai/sdk/helpers/beta/zod';
import { z } from 'zod';

const anthropic = new Anthropic();

// Using betaZodTool (requires Zod 3.25.0+)
const getWeatherTool = betaZodTool({
  name: 'get_weather',
  description: 'Get the current weather in a given location',
  inputSchema: z.object({
    location: z.string().describe('The city and state, e.g. San Francisco, CA'),
    unit: z.enum(['celsius', 'fahrenheit']).default('fahrenheit')
      .describe('Temperature unit')
  }),
  run: async (input) => {
    // In a full implementation, you'd call a weather API here
    return JSON.stringify({temperature: '20°C', condition: 'Sunny'});
  }
});

// Use the tool runner
const runner = anthropic.beta.messages.toolRunner({
  model: 'claude-sonnet-4-5',
  max_tokens: 1024,
  tools: [getWeatherTool],
  messages: [
    {
      role: 'user',
      content: "What's the weather like in Paris?"
    }
  ]
});

// Process messages as they come in
for await (const message of runner) {
  console.log(message.content[0].text);
}
```

La funzione `run` deve restituire un blocco di contenuto o un array di blocchi di contenuto, inclusi blocchi di testo, immagini o documenti. Questo consente agli strumenti di restituire risposte ricche e multimodali. Le stringhe restituite verranno convertite in un blocco di contenuto di testo.
Se desideri restituire un oggetto JSON strutturato a Claude, stringificalo in una stringa JSON prima di restituirlo. I numeri, i booleani o altri primitivi non stringa devono anche essere convertiti in stringhe.

### Iterazione sul tool runner

Il tool runner restituito da `toolRunner()` è un iterabile asincrono, che puoi iterare con un ciclo `for await ... of`. Questo è spesso indicato come un "tool call loop".
Ogni iterazione del ciclo produce un messaggio restituito da Claude.

Dopo che il tuo codice ha avuto la possibilità di elaborare il messaggio corrente all'interno del ciclo, il tool runner controllerà il messaggio per vedere se Claude ha richiesto un uso dello strumento. Se è così, chiamerà lo strumento e invierà automaticamente il risultato dello strumento a Claude, quindi produrrà il messaggio successivo da Claude per iniziare l'iterazione successiva del tuo ciclo.

Puoi terminare il ciclo in qualsiasi iterazione con una semplice istruzione `break`. Il tool runner continuerà il ciclo fino a quando Claude non restituisce un messaggio senza un uso dello strumento.

Se non ti interessa dei messaggi intermedi, invece di utilizzare un ciclo, puoi semplicemente `await` il tool runner, che restituirà il messaggio finale da Claude.

### Utilizzo avanzato

All'interno del ciclo, hai la possibilità di personalizzare completamente la prossima richiesta del tool runner all'API Messages.
Il metodo `runner.generateToolResponse()` chiamerà lo strumento (se Claude ha attivato un uso dello strumento) e ti darà accesso al risultato dello strumento che verrà inviato all'API Messages.
I metodi `runner.setMessagesParams()` e `runner.pushMessages()` ti consentono di modificare i parametri per la prossima richiesta dell'API Messages. I parametri correnti sono disponibili sotto `runner.params`.

```typescript
const runner = anthropic.beta.messages.toolRunner({
  model: 'claude-sonnet-4-5',
  max_tokens: 1024,
  tools: [getWeatherTool],
  messages: [
    { role: 'user', content: "What's the weather in San Francisco?" }
  ]
});

for await (const message of runner) {
  // Get the tool response that will be sent
  const toolResponse = await runner.generateToolResponse();

  // Customize the next request
  runner.setMessagesParams(params => ({
    ...params,
    max_tokens: 2048  // Increase tokens for next request
  }));

  // Or add additional messages
  runner.pushMessages(
    { role: 'user', content: 'Please be concise in your response.' }
  );
}
```

### Streaming

Quando abiliti lo streaming con `stream: true`, ogni valore emesso dal tool runner è un `MessageStream` come restituito da `anthropic.messages.stream()`. Il `MessageStream` è esso stesso un iterabile asincrono che produce eventi di streaming dall'API Messages.

Puoi utilizzare `messageStream.finalMessage()` per lasciare che l'SDK esegua l'accumulo degli eventi di streaming nel messaggio finale per te.

```typescript
const runner = anthropic.beta.messages.toolRunner({
  model: 'claude-sonnet-4-5-20250929',
  max_tokens: 1000,
  messages: [{ role: 'user', content: 'What is the weather in San Francisco?' }],
  tools: [calculatorTool],
  stream: true,
});

// When streaming, the runner returns BetaMessageStream
for await (const messageStream of runner) {
  for await (const event of messageStream) {
    console.log('event:', event);
  }
  console.log('message:', await messageStream.finalMessage());
}

console.log(await runner);
```

</Tab>
<Tab title="TypeScript (JSON Schema)">

### Utilizzo di base

Utilizza `betaTool()` per definizioni di strumenti type-safe basate su schemi JSON. TypeScript e il tuo editor saranno consapevoli del tipo del parametro `input` per l'autocompletamento.

<Note>
L'input generato da Claude non verrà convalidato in fase di esecuzione. Esegui la convalida all'interno della funzione `run` se necessario.
</Note>

```typescript
import { Anthropic } from '@anthropic-ai/sdk';
import { betaZodTool, betaTool } from '@anthropic-ai/sdk/helpers/beta/json-schema';
import { z } from 'zod';

const anthropic = new Anthropic();

// Using betaTool with JSON schema (no Zod required)
const calculateSumTool = betaTool({
  name: 'calculate_sum',
  description: 'Add two numbers together',
  inputSchema: {
    type: 'object',
    properties: {
      a: { type: 'number', description: 'First number' },
      b: { type: 'number', description: 'Second number' }
    },
    required: ['a', 'b']
  },
  run: async (input) => {
    return String(input.a + input.b);
  }
});

// Use the tool runner
const runner = anthropic.beta.messages.toolRunner({
  model: 'claude-sonnet-4-5',
  max_tokens: 1024,
  tools: [getWeatherTool, calculateSumTool],
  messages: [
    {
      role: 'user',
      content: "What's 15 + 27?"
    }
  ]
});

// Process messages as they come in
for await (const message of runner) {
  console.log(message.content[0].text);
}
```

La funzione `run` deve restituire qualsiasi blocco di contenuto o array di blocchi di contenuto, inclusi blocchi di testo, immagine o documento. Questo consente agli strumenti di restituire risposte ricche e multimodali. Le stringhe restituite verranno convertite in un blocco di contenuto di testo.
Se desideri restituire un oggetto JSON strutturato a Claude, codificalo in una stringa JSON prima di restituirlo. I numeri, i booleani o altri primitivi non stringa devono anche essere convertiti in stringhe.

### Iterazione sul tool runner

Il tool runner restituito da `toolRunner()` è un iterabile asincrono, che puoi iterare con un ciclo `for await ... of`. Questo è spesso indicato come un "tool call loop".
Ogni iterazione del ciclo produce un messaggio restituito da Claude.

Dopo che il tuo codice ha avuto la possibilità di elaborare il messaggio corrente all'interno del ciclo, il tool runner controllerà il messaggio per vedere se Claude ha richiesto un uso dello strumento. Se è così, chiamerà lo strumento e invierà automaticamente il risultato dello strumento a Claude, quindi produrrà il messaggio successivo da Claude per iniziare l'iterazione successiva del tuo ciclo.

Puoi terminare il ciclo in qualsiasi iterazione con una semplice istruzione `break`. Il tool runner continuerà il ciclo fino a quando Claude non restituisce un messaggio senza un uso dello strumento.

Se non ti interessa dei messaggi intermedi, invece di utilizzare un ciclo, puoi semplicemente `await` il tool runner, che restituirà il messaggio finale da Claude.

### Utilizzo avanzato

All'interno del ciclo, hai la possibilità di personalizzare completamente la prossima richiesta del tool runner all'API Messages.
Il metodo `runner.generateToolResponse()` chiamerà lo strumento (se Claude ha attivato un uso dello strumento) e ti darà accesso al risultato dello strumento che verrà inviato all'API Messages.
I metodi `runner.setMessagesParams()` e `runner.pushMessages()` ti consentono di modificare i parametri per la prossima richiesta dell'API Messages. I parametri correnti sono disponibili sotto `runner.params`.

```typescript
const runner = anthropic.beta.messages.toolRunner({
  model: 'claude-sonnet-4-5',
  max_tokens: 1024,
  tools: [getWeatherTool],
  messages: [
    { role: 'user', content: "What's the weather in San Francisco?" }
  ]
});

for await (const message of runner) {
  // Get the tool response that will be sent
  const toolResponse = await runner.generateToolResponse();

  // Customize the next request
  runner.setMessagesParams(params => ({
    ...params,
    max_tokens: 2048  // Increase tokens for next request
  }));

  // Or add additional messages
  runner.pushMessages(
    { role: 'user', content: 'Please be concise in your response.' }
  );
}
```

### Streaming

Quando abiliti lo streaming con `stream: true`, ogni valore emesso dal tool runner è un `MessageStream` come restituito da `anthropic.messages.stream()`. Il `MessageStream` è esso stesso un iterabile asincrono che produce eventi di streaming dall'API Messages.

Puoi utilizzare `messageStream.finalMessage()` per lasciare che l'SDK esegua l'accumulo degli eventi di streaming nel messaggio finale per te.

```typescript
const runner = anthropic.beta.messages.toolRunner({
  model: 'claude-sonnet-4-5-20250929',
  max_tokens: 1000,
  messages: [{ role: 'user', content: 'What is the weather in San Francisco?' }],
  tools: [calculatorTool],
  stream: true,
});

// When streaming, the runner returns BetaMessageStream
for await (const messageStream of runner) {
  for await (const event of messageStream) {
    console.log('event:', event);
  }
  console.log('message:', await messageStream.finalMessage());
}

console.log(await runner);
```

</Tab>
<Tab title="Ruby">

### Utilizzo di base

Definisci gli strumenti utilizzando `Anthropic::BaseTool` con uno schema di input, quindi utilizza `client.beta.messages.tool_runner` per eseguirli.

```ruby
require "anthropic"

# Initialize client
client = Anthropic::Client.new

# Define input schema
class GetWeatherInput < Anthropic::BaseModel
  required :location, String, doc: "The city and state, e.g. San Francisco, CA"
  optional :unit, Anthropic::InputSchema::EnumOf["celsius", "fahrenheit"],
           doc: "Temperature unit"
end

# Define tool
class GetWeather < Anthropic::BaseTool
  doc "Get the current weather in a given location"
  input_schema GetWeatherInput

  def call(input)
    # In a full implementation, you'd call a weather API here
    JSON.generate({temperature: "20°C", condition: "Sunny"})
  end
end

class CalculateSumInput < Anthropic::BaseModel
  required :a, Integer, doc: "First number"
  required :b, Integer, doc: "Second number"
end

class CalculateSum < Anthropic::BaseTool
  doc "Add two numbers together"
  input_schema CalculateSumInput

  def call(input)
    (input.a + input.b).to_s
  end
end

# Use the tool runner
runner = client.beta.messages.tool_runner(
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  tools: [GetWeather.new, CalculateSum.new],
  messages: [
    {role: "user", content: "What's the weather like in Paris? Also, what's 15 + 27?"}
  ]
)

runner.each_message do |message|
  message.content.each do |block|
    puts block.text if block.respond_to?(:text)
  end
end
```

Il metodo `call` deve restituire una stringa o un array di blocchi di contenuto. Se desideri restituire un oggetto JSON strutturato a Claude, codificalo in una stringa JSON prima di restituirlo.

La classe `Anthropic::BaseTool` utilizza il metodo `doc` per la descrizione dello strumento e `input_schema` per definire i parametri previsti. L'SDK convertirà automaticamente questo nel formato dello schema JSON appropriato.

### Iterazione sul tool runner

Il tool runner fornisce un metodo `each_message` che produce ogni messaggio mentre la conversazione progredisce. Questo è spesso indicato come un "tool call loop".

Dopo che il tuo codice ha avuto la possibilità di elaborare il messaggio corrente, il tool runner controllerà se Claude ha richiesto un uso dello strumento. Se è così, chiamerà lo strumento e invierà automaticamente il risultato dello strumento a Claude, quindi produrrà il messaggio successivo.

Se non ti interessa dei messaggi intermedi, puoi utilizzare il metodo `run_until_finished` per ottenere tutti i messaggi in una volta:

```ruby
runner = client.beta.messages.tool_runner(
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  tools: [GetWeather.new, CalculateSum.new],
  messages: [
    {role: "user", content: "What's the weather like in Paris? Also, what's 15 + 27?"}
  ]
)

all_messages = runner.run_until_finished
all_messages.each { |msg| puts msg.content }
```

### Utilizzo avanzato

Il tool runner fornisce diversi metodi per personalizzare il comportamento:

- `#next_message` - Avanza manualmente nella conversazione un messaggio alla volta
- `#feed_messages` - Inietta messaggi aggiuntivi a metà conversazione
- `#params` - Accedi o modifica i parametri di richiesta correnti

```ruby
runner = client.beta.messages.tool_runner(
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  tools: [GetWeather.new],
  messages: [{role: "user", content: "What's the weather in San Francisco?"}]
)

# Manual step-by-step control
message = runner.next_message
puts message.content

# Inject follow-up messages
runner.feed_messages([
  {role: "user", content: "Also check Boston"}
])

# Access current parameters
puts runner.params
```

### Streaming

Quando utilizzi lo streaming, itera con `each_streaming` per ricevere eventi in tempo reale:

```ruby
runner = client.beta.messages.tool_runner(
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  tools: [CalculateSum.new],
  messages: [{role: "user", content: "What is 15 + 27?"}]
)

runner.each_streaming do |event|
  case event
  when Anthropic::Streaming::TextEvent
    print event.text
  when Anthropic::Streaming::ToolUseEvent
    puts "\nTool called: #{event.tool_name}"
  end
end
```

</Tab>
</Tabs>

<Note>
Il tool runner dell'SDK è in beta. Il resto di questo documento copre l'implementazione manuale degli strumenti.
</Note>

## Controllo dell'output di Claude

### Forzare l'uso degli strumenti

In alcuni casi, potresti voler che Claude utilizzi uno strumento specifico per rispondere alla domanda dell'utente, anche se Claude pensa di poter fornire una risposta senza utilizzare uno strumento. Puoi farlo specificando lo strumento nel campo `tool_choice` come segue:

```
tool_choice = {"type": "tool", "name": "get_weather"}
```

Quando si lavora con il parametro tool_choice, abbiamo quattro opzioni possibili:

- `auto` consente a Claude di decidere se chiamare uno qualsiasi degli strumenti forniti o meno. Questo è il valore predefinito quando vengono forniti `tools`.
- `any` dice a Claude che deve utilizzare uno degli strumenti forniti, ma non forza uno strumento particolare.
- `tool` ci consente di forzare Claude a utilizzare sempre uno strumento particolare.
- `none` impedisce a Claude di utilizzare qualsiasi strumento. Questo è il valore predefinito quando non vengono forniti `tools`.

<Note>
Quando si utilizza la [memorizzazione nella cache dei prompt](/docs/it/build-with-claude/prompt-caching#what-invalidates-the-cache), le modifiche al parametro `tool_choice` invalideranno i blocchi di messaggio memorizzati nella cache. Le definizioni degli strumenti e i prompt di sistema rimangono memorizzati nella cache, ma il contenuto del messaggio deve essere rielaborato.
</Note>

Questo diagramma illustra come funziona ogni opzione:

<Frame>
  ![Image](/docs/images/tool_choice.png)
</Frame>

Nota che quando hai `tool_choice` come `any` o `tool`, precompileremo il messaggio dell'assistente per forzare l'uso di uno strumento. Ciò significa che i modelli non emetteranno una risposta in linguaggio naturale o una spiegazione prima dei blocchi di contenuto `tool_use`, anche se esplicitamente richiesto di farlo.

<Note>
Quando si utilizza il [pensiero esteso](/docs/it/build-with-claude/extended-thinking) con l'uso degli strumenti, `tool_choice: {"type": "any"}` e `tool_choice: {"type": "tool", "name": "..."}` non sono supportati e risulteranno in un errore. Solo `tool_choice: {"type": "auto"}` (il valore predefinito) e `tool_choice: {"type": "none"}` sono compatibili con il pensiero esteso.
</Note>

I nostri test hanno dimostrato che questo non dovrebbe ridurre le prestazioni. Se desideri che il modello fornisca contesto in linguaggio naturale o spiegazioni mentre richiedi comunque che il modello utilizzi uno strumento specifico, puoi utilizzare `{"type": "auto"}` per `tool_choice` (il valore predefinito) e aggiungere istruzioni esplicite in un messaggio `user`. Ad esempio: `What's the weather like in London? Use the get_weather tool in your response.`

<Tip>
**Chiamate di strumenti garantite con strumenti rigorosi**

Combina `tool_choice: {"type": "any"}` con l'[uso rigoroso degli strumenti](/docs/it/build-with-claude/structured-outputs) per garantire sia che uno dei tuoi strumenti verrà chiamato CHE gli input dello strumento seguiranno rigorosamente il tuo schema. Imposta `strict: true` sulle definizioni dei tuoi strumenti per abilitare la convalida dello schema.
</Tip>

### Output JSON

Gli strumenti non devono necessariamente essere funzioni client — puoi utilizzare gli strumenti ogni volta che desideri che il modello restituisca un output JSON che segua uno schema fornito. Ad esempio, potresti utilizzare uno strumento `record_summary` con uno schema particolare. Vedi [Uso degli strumenti con Claude](/docs/it/agents-and-tools/tool-use/overview) per un esempio di lavoro completo.

### Risposte del modello con strumenti

Quando si utilizzano gli strumenti, Claude spesso commenterà quello che sta facendo o risponderà naturalmente all'utente prima di invocare gli strumenti.

Ad esempio, dato il prompt "Qual è il meteo a San Francisco in questo momento e che ora è lì?", Claude potrebbe rispondere con:

```json JSON
{
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "Ti aiuterò a controllare il meteo attuale e l'ora a San Francisco."
    },
    {
      "type": "tool_use",
      "id": "toolu_01A09q90qw90lq917835lq9",
      "name": "get_weather",
      "input": {"location": "San Francisco, CA"}
    }
  ]
}
```

Questo stile di risposta naturale aiuta gli utenti a capire cosa sta facendo Claude e crea un'interazione più conversazionale. Puoi guidare lo stile e il contenuto di queste risposte attraverso i tuoi prompt di sistema e fornendo `<examples>` nei tuoi prompt.

È importante notare che Claude può utilizzare varie formulazioni e approcci quando spiega le sue azioni. Il tuo codice dovrebbe trattare queste risposte come qualsiasi altro testo generato dall'assistente e non fare affidamento su convenzioni di formattazione specifiche.

### Utilizzo parallelo degli strumenti

Per impostazione predefinita, Claude può utilizzare più strumenti per rispondere a una query dell'utente. Puoi disabilitare questo comportamento da:

- Impostando `disable_parallel_tool_use=true` quando il tipo di tool_choice è `auto`, il che garantisce che Claude utilizzi **al massimo uno** strumento
- Impostando `disable_parallel_tool_use=true` quando il tipo di tool_choice è `any` o `tool`, il che garantisce che Claude utilizzi **esattamente uno** strumento

<section title="Esempio completo di utilizzo parallelo degli strumenti">

<Note>
**Più semplice con Tool runner**: L'esempio seguente mostra la gestione manuale degli strumenti paralleli. Per la maggior parte dei casi d'uso, [tool runner](#tool-runner-beta) gestisce automaticamente l'esecuzione parallela degli strumenti con molto meno codice.
</Note>

Ecco un esempio completo che mostra come formattare correttamente le chiamate parallele degli strumenti nella cronologia dei messaggi:

<CodeGroup>
```python Python
import anthropic

client = anthropic.Anthropic()

# Definisci gli strumenti
tools = [
    {
        "name": "get_weather",
        "description": "Ottieni il meteo attuale in una determinata posizione",
        "input_schema": {
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "La città e lo stato, ad es. San Francisco, CA"
                }
            },
            "required": ["location"]
        }
    },
    {
        "name": "get_time",
        "description": "Ottieni l'ora attuale in un determinato fuso orario",
        "input_schema": {
            "type": "object",
            "properties": {
                "timezone": {
                    "type": "string",
                    "description": "Il fuso orario, ad es. America/New_York"
                }
            },
            "required": ["timezone"]
        }
    }
]

# Richiesta iniziale
response = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    tools=tools,
    messages=[
        {
            "role": "user",
            "content": "Qual è il meteo a SF e NYC, e che ora è lì?"
        }
    ]
)

# Risposta di Claude con chiamate parallele degli strumenti
print("Claude vuole usare gli strumenti:", response.stop_reason == "tool_use")
print("Numero di chiamate agli strumenti:", len([c for c in response.content if c.type == "tool_use"]))

# Costruisci la conversazione con i risultati degli strumenti
messages = [
    {
        "role": "user",
        "content": "Qual è il meteo a SF e NYC, e che ora è lì?"
    },
    {
        "role": "assistant",
        "content": response.content  # Contiene più blocchi tool_use
    },
    {
        "role": "user",
        "content": [
            {
                "type": "tool_result",
                "tool_use_id": "toolu_01",  # Deve corrispondere all'ID da tool_use
                "content": "San Francisco: 68°F, parzialmente nuvoloso"
            },
            {
                "type": "tool_result",
                "tool_use_id": "toolu_02",
                "content": "New York: 45°F, cielo sereno"
            },
            {
                "type": "tool_result",
                "tool_use_id": "toolu_03",
                "content": "Ora a San Francisco: 14:30 PST"
            },
            {
                "type": "tool_result",
                "tool_use_id": "toolu_04",
                "content": "Ora a New York: 17:30 EST"
            }
        ]
    }
]

# Ottieni la risposta finale
final_response = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    tools=tools,
    messages=messages
)

print(final_response.content[0].text)
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

// Definisci gli strumenti
const tools = [
  {
    name: "get_weather",
    description: "Ottieni il meteo attuale in una determinata posizione",
    input_schema: {
      type: "object",
      properties: {
        location: {
          type: "string",
          description: "La città e lo stato, ad es. San Francisco, CA"
        }
      },
      required: ["location"]
    }
  },
  {
    name: "get_time",
    description: "Ottieni l'ora attuale in un determinato fuso orario",
    input_schema: {
      type: "object",
      properties: {
        timezone: {
          type: "string",
          description: "Il fuso orario, ad es. America/New_York"
        }
      },
      required: ["timezone"]
    }
  }
];

// Richiesta iniziale
const response = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  tools: tools,
  messages: [
    {
      role: "user",
      content: "Qual è il meteo a SF e NYC, e che ora è lì?"
    }
  ]
});

// Costruisci la conversazione con i risultati degli strumenti
const messages = [
  {
    role: "user",
    content: "Qual è il meteo a SF e NYC, e che ora è lì?"
  },
  {
    role: "assistant",
    content: response.content  // Contiene più blocchi tool_use
  },
  {
    role: "user",
    content: [
      {
        type: "tool_result",
        tool_use_id: "toolu_01",  // Deve corrispondere all'ID da tool_use
        content: "San Francisco: 68°F, parzialmente nuvoloso"
      },
      {
        type: "tool_result",
        tool_use_id: "toolu_02",
        content: "New York: 45°F, cielo sereno"
      },
      {
        type: "tool_result",
        tool_use_id: "toolu_03",
        content: "Ora a San Francisco: 14:30 PST"
      },
      {
        type: "tool_result",
        tool_use_id: "toolu_04",
        content: "Ora a New York: 17:30 EST"
      }
    ]
  }
];

// Ottieni la risposta finale
const finalResponse = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  tools: tools,
  messages: messages
});

console.log(finalResponse.content[0].text);
```
</CodeGroup>

Il messaggio dell'assistente con chiamate parallele degli strumenti avrebbe questo aspetto:

```json
{
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "Controllerò il meteo e l'ora sia per San Francisco che per New York City."
    },
    {
      "type": "tool_use",
      "id": "toolu_01",
      "name": "get_weather",
      "input": {"location": "San Francisco, CA"}
    },
    {
      "type": "tool_use",
      "id": "toolu_02",
      "name": "get_weather",
      "input": {"location": "New York, NY"}
    },
    {
      "type": "tool_use",
      "id": "toolu_03",
      "name": "get_time",
      "input": {"timezone": "America/Los_Angeles"}
    },
    {
      "type": "tool_use",
      "id": "toolu_04",
      "name": "get_time",
      "input": {"timezone": "America/New_York"}
    }
  ]
}
```

</section>
<section title="Script di test completo per gli strumenti paralleli">

Ecco uno script completo e eseguibile per testare e verificare che le chiamate parallele degli strumenti funzionino correttamente:

<CodeGroup>
```python Python
#!/usr/bin/env python3
"""Script di test per verificare le chiamate parallele degli strumenti con l'API Claude"""

import os
from anthropic import Anthropic

# Inizializza il client
client = Anthropic(api_key=os.environ.get("ANTHROPIC_API_KEY"))

# Definisci gli strumenti
tools = [
    {
        "name": "get_weather",
        "description": "Ottieni il meteo attuale in una determinata posizione",
        "input_schema": {
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "La città e lo stato, ad es. San Francisco, CA"
                }
            },
            "required": ["location"]
        }
    },
    {
        "name": "get_time",
        "description": "Ottieni l'ora attuale in un determinato fuso orario",
        "input_schema": {
            "type": "object",
            "properties": {
                "timezone": {
                    "type": "string",
                    "description": "Il fuso orario, ad es. America/New_York"
                }
            },
            "required": ["timezone"]
        }
    }
]

# Conversazione di test con chiamate parallele degli strumenti
messages = [
    {
        "role": "user",
        "content": "Qual è il meteo a SF e NYC, e che ora è lì?"
    }
]

# Effettua la richiesta iniziale
print("Richiesta di chiamate parallele degli strumenti...")
response = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=messages,
    tools=tools
)

# Verifica le chiamate parallele degli strumenti
tool_uses = [block for block in response.content if block.type == "tool_use"]
print(f"\n✓ Claude ha effettuato {len(tool_uses)} chiamate agli strumenti")

if len(tool_uses) > 1:
    print("✓ Rilevate chiamate parallele degli strumenti!")
    for tool in tool_uses:
        print(f"  - {tool.name}: {tool.input}")
else:
    print("✗ Nessuna chiamata parallela degli strumenti rilevata")

# Simula l'esecuzione dello strumento e formatta i risultati correttamente
tool_results = []
for tool_use in tool_uses:
    if tool_use.name == "get_weather":
        if "San Francisco" in str(tool_use.input):
            result = "San Francisco: 68°F, parzialmente nuvoloso"
        else:
            result = "New York: 45°F, cielo sereno"
    else:  # get_time
        if "Los_Angeles" in str(tool_use.input):
            result = "14:30 PST"
        else:
            result = "17:30 EST"

    tool_results.append({
        "type": "tool_result",
        "tool_use_id": tool_use.id,
        "content": result
    })

# Continua la conversazione con i risultati degli strumenti
messages.extend([
    {"role": "assistant", "content": response.content},
    {"role": "user", "content": tool_results}  # Tutti i risultati in un messaggio!
])

# Ottieni la risposta finale
print("\nOttenimento della risposta finale...")
final_response = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=messages,
    tools=tools
)

print(f"\nRisposta di Claude:\n{final_response.content[0].text}")

# Verifica la formattazione
print("\n--- Verifica ---")
print(f"✓ Risultati degli strumenti inviati in un singolo messaggio utente: {len(tool_results)} risultati")
print("✓ Nessun testo prima dei risultati degli strumenti nell'array di contenuto")
print("✓ Conversazione formattata correttamente per l'utilizzo parallelo futuro degli strumenti")
```

```typescript TypeScript
#!/usr/bin/env node
// Script di test per verificare le chiamate parallele degli strumenti con l'API Claude

import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY
});

// Definisci gli strumenti
const tools = [
  {
    name: "get_weather",
    description: "Ottieni il meteo attuale in una determinata posizione",
    input_schema: {
      type: "object",
      properties: {
        location: {
          type: "string",
          description: "La città e lo stato, ad es. San Francisco, CA"
        }
      },
      required: ["location"]
    }
  },
  {
    name: "get_time",
    description: "Ottieni l'ora attuale in un determinato fuso orario",
    input_schema: {
      type: "object",
      properties: {
        timezone: {
          type: "string",
          description: "Il fuso orario, ad es. America/New_York"
        }
      },
      required: ["timezone"]
    }
  }
];

async function testParallelTools() {
  // Effettua la richiesta iniziale
  console.log("Richiesta di chiamate parallele degli strumenti...");
  const response = await anthropic.messages.create({
    model: "claude-sonnet-4-5",
    max_tokens: 1024,
    messages: [{
      role: "user",
      content: "Qual è il meteo a SF e NYC, e che ora è lì?"
    }],
    tools: tools
  });

  // Verifica le chiamate parallele degli strumenti
  const toolUses = response.content.filter(block => block.type === "tool_use");
  console.log(`\n✓ Claude ha effettuato ${toolUses.length} chiamate agli strumenti`);

  if (toolUses.length > 1) {
    console.log("✓ Rilevate chiamate parallele degli strumenti!");
    toolUses.forEach(tool => {
      console.log(`  - ${tool.name}: ${JSON.stringify(tool.input)}`);
    });
  } else {
    console.log("✗ Nessuna chiamata parallela degli strumenti rilevata");
  }

  // Simula l'esecuzione dello strumento e formatta i risultati correttamente
  const toolResults = toolUses.map(toolUse => {
    let result;
    if (toolUse.name === "get_weather") {
      result = toolUse.input.location.includes("San Francisco")
        ? "San Francisco: 68°F, parzialmente nuvoloso"
        : "New York: 45°F, cielo sereno";
    } else {
      result = toolUse.input.timezone.includes("Los_Angeles")
        ? "14:30 PST"
        : "17:30 EST";
    }

    return {
      type: "tool_result",
      tool_use_id: toolUse.id,
      content: result
    };
  });

  // Ottieni la risposta finale con formattazione corretta
  console.log("\nOttenimento della risposta finale...");
  const finalResponse = await anthropic.messages.create({
    model: "claude-sonnet-4-5",
    max_tokens: 1024,
    messages: [
      { role: "user", content: "Qual è il meteo a SF e NYC, e che ora è lì?" },
      { role: "assistant", content: response.content },
      { role: "user", content: toolResults }  // Tutti i risultati in un messaggio!
    ],
    tools: tools
  });

  console.log(`\nRisposta di Claude:\n${finalResponse.content[0].text}`);

  // Verifica la formattazione
  console.log("\n--- Verifica ---");
  console.log(`✓ Risultati degli strumenti inviati in un singolo messaggio utente: ${toolResults.length} risultati`);
  console.log("✓ Nessun testo prima dei risultati degli strumenti nell'array di contenuto");
  console.log("✓ Conversazione formattata correttamente per l'utilizzo parallelo futuro degli strumenti");
}

testParallelTools().catch(console.error);
```
</CodeGroup>

Questo script dimostra:
- Come formattare correttamente le chiamate parallele degli strumenti e i risultati
- Come verificare che le chiamate parallele vengono effettuate
- La struttura corretta del messaggio che incoraggia l'utilizzo parallelo futuro degli strumenti
- Gli errori comuni da evitare (come il testo prima dei risultati degli strumenti)

Esegui questo script per testare la tua implementazione e assicurarti che Claude stia effettuando le chiamate parallele degli strumenti in modo efficace.

</section>

#### Massimizzare l'utilizzo parallelo degli strumenti

Sebbene i modelli Claude 4 abbiano eccellenti capacità di utilizzo parallelo degli strumenti per impostazione predefinita, puoi aumentare la probabilità di esecuzione parallela degli strumenti su tutti i modelli con prompt mirati:

<section title="Prompt di sistema per l'utilizzo parallelo degli strumenti">

Per i modelli Claude 4 (Opus 4 e Sonnet 4), aggiungi questo al tuo prompt di sistema:
```text
Per la massima efficienza, ogni volta che devi eseguire più operazioni indipendenti, invoca tutti gli strumenti rilevanti simultaneamente piuttosto che sequenzialmente.
```

Per un utilizzo parallelo degli strumenti ancora più forte (consigliato se quello predefinito non è sufficiente), usa:
```text
<use_parallel_tool_calls>
Per la massima efficienza, ogni volta che esegui più operazioni indipendenti, invoca tutti gli strumenti rilevanti simultaneamente piuttosto che sequenzialmente. Dai priorità alle chiamate parallele degli strumenti ogni volta che è possibile. Ad esempio, quando leggi 3 file, esegui 3 chiamate agli strumenti in parallelo per leggere tutti e 3 i file nel contesto contemporaneamente. Quando esegui più comandi di sola lettura come `ls` o `list_dir`, esegui sempre tutti i comandi in parallelo. Sbaglia nel senso di massimizzare le chiamate parallele degli strumenti piuttosto che eseguire troppi strumenti sequenzialmente.
</use_parallel_tool_calls>
```

</section>
<section title="Prompt dei messaggi utente">

Puoi anche incoraggiare l'utilizzo parallelo degli strumenti all'interno di messaggi utente specifici:

```python
# Invece di:
"Qual è il meteo a Parigi? Controlla anche Londra."

# Usa:
"Controlla il meteo a Parigi e Londra simultaneamente."

# O sii esplicito:
"Per favore, usa le chiamate parallele degli strumenti per ottenere il meteo per Parigi, Londra e Tokyo contemporaneamente."
```

</section>

<Warning>
**Utilizzo parallelo degli strumenti con Claude Sonnet 3.7**

Claude Sonnet 3.7 potrebbe essere meno propenso a effettuare chiamate parallele degli strumenti in una risposta, anche quando non hai impostato `disable_parallel_tool_use`. Ti consigliamo di [eseguire l'upgrade ai modelli Claude 4](/docs/it/about-claude/models/migrating-to-claude-4), che hanno utilizzo degli strumenti efficiente in termini di token e miglioramento delle chiamate parallele degli strumenti.

Se stai ancora utilizzando Claude Sonnet 3.7, puoi abilitare l'[intestazione beta](/docs/it/api/beta-headers) `token-efficient-tools-2025-02-19`, che aiuta a incoraggiare Claude a utilizzare gli strumenti in parallelo. Puoi anche introdurre uno "strumento batch" che può fungere da meta-strumento per avvolgere le invocazioni ad altri strumenti simultaneamente.

Vedi [questo esempio](https://github.com/anthropics/anthropic-cookbook/blob/main/tool_use/parallel_tools_claude_3_7_sonnet.ipynb) nel nostro cookbook per come utilizzare questa soluzione alternativa.

</Warning>

## Gestione dei blocchi di contenuto tool use e tool result

<Note>
**Più semplice con Tool runner**: La gestione manuale degli strumenti descritta in questa sezione è gestita automaticamente da [tool runner](#tool-runner-beta). Usa questa sezione quando hai bisogno di un controllo personalizzato sull'esecuzione degli strumenti.
</Note>

La risposta di Claude differisce a seconda che utilizzi uno strumento client o server.

### Gestione dei risultati dagli strumenti client

La risposta avrà un `stop_reason` di `tool_use` e uno o più blocchi di contenuto `tool_use` che includono:

- `id`: Un identificatore univoco per questo particolare blocco tool_use. Questo verrà utilizzato per abbinare i risultati dello strumento in seguito.
- `name`: Il nome dello strumento utilizzato.
- `input`: Un oggetto contenente l'input passato allo strumento, conforme a `input_schema` dello strumento.

<section title="Esempio di risposta API con un blocco di contenuto `tool_use`">

```json JSON
{
  "id": "msg_01Aq9w938a90dw8q",
  "model": "claude-sonnet-4-5",
  "stop_reason": "tool_use",
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "Controllerò il meteo attuale a San Francisco per te."
    },
    {
      "type": "tool_use",
      "id": "toolu_01A09q90qw90lq917835lq9",
      "name": "get_weather",
      "input": {"location": "San Francisco, CA", "unit": "celsius"}
    }
  ]
}
```

</section>

Quando ricevi una risposta tool use per uno strumento client, dovresti:

1. Estrarre `name`, `id` e `input` dal blocco `tool_use`.
2. Eseguire lo strumento effettivo nel tuo codebase corrispondente a quel nome di strumento, passando l'`input` dello strumento.
3. Continuare la conversazione inviando un nuovo messaggio con il `role` di `user` e un blocco `content` contenente il tipo `tool_result` e le seguenti informazioni:
   - `tool_use_id`: L'`id` della richiesta tool use per cui questo è un risultato.
   - `content`: Il risultato dello strumento, come stringa (ad es. `"content": "15 gradi"`), un elenco di blocchi di contenuto annidati (ad es. `"content": [{"type": "text", "text": "15 gradi"}]`), o un elenco di blocchi di documento (ad es. `"content": ["type": "document", "source": {"type": "text", "media_type": "text/plain", "data": "15 gradi"}]`). Questi blocchi di contenuto possono utilizzare i tipi `text`, `image` o `document`.
   - `is_error` (facoltativo): Impostare su `true` se l'esecuzione dello strumento ha generato un errore.

<Note>
**Requisiti di formattazione importanti**:
- I blocchi di risultato dello strumento devono seguire immediatamente i loro corrispondenti blocchi tool use nella cronologia dei messaggi. Non puoi includere alcun messaggio tra il messaggio tool use dell'assistente e il messaggio tool result dell'utente.
- Nel messaggio utente contenente i risultati dello strumento, i blocchi tool_result devono venire PRIMA nell'array di contenuto. Qualsiasi testo deve venire DOPO tutti i risultati dello strumento.

Ad esempio, questo causerà un errore 400:
```json
{"role": "user", "content": [
  {"type": "text", "text": "Ecco i risultati:"},  // ❌ Testo prima di tool_result
  {"type": "tool_result", "tool_use_id": "toolu_01", ...}
]}
```

Questo è corretto:
```json
{"role": "user", "content": [
  {"type": "tool_result", "tool_use_id": "toolu_01", ...},
  {"type": "text", "text": "Cosa dovrei fare dopo?"}  // ✅ Testo dopo tool_result
]}
```

Se ricevi un errore come "tool_use ids were found without tool_result blocks immediately after", verifica che i tuoi risultati dello strumento siano formattati correttamente.
</Note>

<section title="Esempio di risultato dello strumento riuscito">

```json JSON
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": "15 gradi"
    }
  ]
}
```

</section>

<section title="Esempio di risultato dello strumento con immagini">

```json JSON
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": [
        {"type": "text", "text": "15 gradi"},
        {
          "type": "image",
          "source": {
            "type": "base64",
            "media_type": "image/jpeg",
            "data": "/9j/4AAQSkZJRg...",
          }
        }
      ]
    }
  ]
}
```

</section>
<section title="Esempio di risultato dello strumento vuoto">

```json JSON
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
    }
  ]
}
```

</section>

<section title="Esempio di risultato dello strumento con documenti">

```json JSON
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": [
        {"type": "text", "text": "Il meteo è"},
        {
          "type": "document",
          "source": {
            "type": "text",
            "media_type": "text/plain",
            "data": "15 gradi"
          }
        }
      ]
    }
  ]
}
```

</section>

Dopo aver ricevuto il risultato dello strumento, Claude utilizzerà quell'informazione per continuare a generare una risposta al prompt originale dell'utente.

### Gestione dei risultati dagli strumenti server

Claude esegue lo strumento internamente e incorpora i risultati direttamente nella sua risposta senza richiedere ulteriore interazione dell'utente.

<Tip>
  **Differenze da altre API**

A differenza delle API che separano l'utilizzo dello strumento o utilizzano ruoli speciali come `tool` o `function`, l'API Claude integra gli strumenti direttamente nella struttura dei messaggi `user` e `assistant`.

I messaggi contengono array di blocchi `text`, `image`, `tool_use` e `tool_result`. I messaggi `user` includono contenuto client e `tool_result`, mentre i messaggi `assistant` contengono contenuto generato dall'IA e `tool_use`.

</Tip>

### Gestione del motivo di arresto `max_tokens`

Se la [risposta di Claude viene interrotta a causa del raggiungimento del limite `max_tokens`](/docs/it/build-with-claude/handling-stop-reasons#max-tokens) e la risposta troncata contiene un blocco tool use incompleto, dovrai riprovare la richiesta con un valore `max_tokens` più alto per ottenere il tool use completo.

<CodeGroup>
```python Python
# Verifica se la risposta è stata troncata durante l'utilizzo dello strumento
if response.stop_reason == "max_tokens":
    # Verifica se l'ultimo blocco di contenuto è un tool_use incompleto
    last_block = response.content[-1]
    if last_block.type == "tool_use":
        # Invia la richiesta con max_tokens più alto
        response = client.messages.create(
            model="claude-sonnet-4-5",
            max_tokens=4096,  # Limite aumentato
            messages=messages,
            tools=tools
        )
```

```typescript TypeScript
// Verifica se la risposta è stata troncata durante l'utilizzo dello strumento
if (response.stop_reason === "max_tokens") {
  // Verifica se l'ultimo blocco di contenuto è un tool_use incompleto
  const lastBlock = response.content[response.content.length - 1];
  if (lastBlock.type === "tool_use") {
    // Invia la richiesta con max_tokens più alto
    response = await anthropic.messages.create({
      model: "claude-sonnet-4-5",
      max_tokens: 4096, // Limite aumentato
      messages: messages,
      tools: tools
    });
  }
}
```
</CodeGroup>

#### Gestione del motivo di arresto `pause_turn`

Quando si utilizzano strumenti server come la ricerca web, l'API può restituire un motivo di arresto `pause_turn`, indicando che l'API ha messo in pausa un turno di lunga durata.

Ecco come gestire il motivo di arresto `pause_turn`:

<CodeGroup>
```python Python
import anthropic

client = anthropic.Anthropic()

# Richiesta iniziale con ricerca web
response = client.messages.create(
    model="claude-3-7-sonnet-latest",
    max_tokens=1024,
    messages=[
        {
            "role": "user",
            "content": "Cerca informazioni complete sui progressi del calcolo quantistico nel 2025"
        }
    ],
    tools=[{
        "type": "web_search_20250305",
        "name": "web_search",
        "max_uses": 10
    }]
)

# Verifica se la risposta ha il motivo di arresto pause_turn
if response.stop_reason == "pause_turn":
    # Continua la conversazione con il contenuto messo in pausa
    messages = [
        {"role": "user", "content": "Cerca informazioni complete sui progressi del calcolo quantistico nel 2025"},
        {"role": "assistant", "content": response.content}
    ]

    # Invia la richiesta di continuazione
    continuation = client.messages.create(
        model="claude-3-7-sonnet-latest",
        max_tokens=1024,
        messages=messages,
        tools=[{
            "type": "web_search_20250305",
            "name": "web_search",
            "max_uses": 10
        }]
    )

    print(continuation)
else:
    print(response)
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

// Richiesta iniziale con ricerca web
const response = await anthropic.messages.create({
  model: "claude-3-7-sonnet-latest",
  max_tokens: 1024,
  messages: [
    {
      role: "user",
      content: "Cerca informazioni complete sui progressi del calcolo quantistico nel 2025"
    }
  ],
  tools: [{
    type: "web_search_20250305",
    name: "web_search",
    max_uses: 10
  }]
});

// Verifica se la risposta ha il motivo di arresto pause_turn
if (response.stop_reason === "pause_turn") {
  // Continua la conversazione con il contenuto messo in pausa
  const messages = [
    { role: "user", content: "Cerca informazioni complete sui progressi del calcolo quantistico nel 2025" },
    { role: "assistant", content: response.content }
  ];

  // Invia la richiesta di continuazione
  const continuation = await anthropic.messages.create({
    model: "claude-3-7-sonnet-latest",
    max_tokens: 1024,
    messages: messages,
    tools: [{
      type: "web_search_20250305",
      name: "web_search",
      max_uses: 10
    }]
  });

  console.log(continuation);
} else {
  console.log(response);
}
```
</CodeGroup>

Quando gestisci `pause_turn`:
- **Continua la conversazione**: Passa la risposta messa in pausa così com'è in una richiesta successiva per permettere a Claude di continuare il suo turno
- **Modifica se necessario**: Puoi facoltativamente modificare il contenuto prima di continuare se vuoi interrompere o reindirizzare la conversazione
- **Preserva lo stato dello strumento**: Includi gli stessi strumenti nella richiesta di continuazione per mantenere la funzionalità

## Risoluzione dei problemi degli errori

<Note>
**Gestione degli errori integrata**: [Tool runner](#tool-runner-beta) fornisce la gestione automatica degli errori per la maggior parte degli scenari comuni. Questa sezione copre la gestione manuale degli errori per i casi d'uso avanzati.
</Note>

Ci sono alcuni diversi tipi di errori che possono verificarsi quando si utilizzano gli strumenti con Claude:

<section title="Errore di esecuzione dello strumento">

Se lo strumento stesso genera un errore durante l'esecuzione (ad es. un errore di rete durante il recupero dei dati meteorologici), puoi restituire il messaggio di errore nel `content` insieme a `"is_error": true`:

```json JSON
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": "ConnectionError: il servizio meteorologico API non è disponibile (HTTP 500)",
      "is_error": true
    }
  ]
}
```

Claude incorporerà quindi questo errore nella sua risposta all'utente, ad es. "Mi dispiace, non sono riuscito a recuperare il meteo attuale perché il servizio meteorologico API non è disponibile. Per favore, riprova più tardi."

</section>
<section title="Nome dello strumento non valido">

Se il tentativo di Claude di utilizzare uno strumento non è valido (ad es. parametri obbligatori mancanti), di solito significa che non c'era abbastanza informazione per Claude per utilizzare lo strumento correttamente. La tua migliore opzione durante lo sviluppo è riprovare la richiesta con valori `description` più dettagliati nelle tue definizioni degli strumenti.

Tuttavia, puoi anche continuare la conversazione con un `tool_result` che indica l'errore, e Claude proverà a utilizzare lo strumento di nuovo con le informazioni mancanti compilate:

```json JSON
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": "Errore: parametro 'location' obbligatorio mancante",
      "is_error": true
    }
  ]
}
```

Se una richiesta di strumento non è valida o mancano parametri, Claude riproverà 2-3 volte con correzioni prima di scusarsi con l'utente.

<Tip>
Per eliminare completamente le chiamate agli strumenti non valide, usa [utilizzo rigoroso degli strumenti](/docs/it/build-with-claude/structured-outputs) con `strict: true` sulle tue definizioni degli strumenti. Questo garantisce che gli input degli strumenti corrisponderanno sempre esattamente al tuo schema, prevenendo parametri mancanti e mancate corrispondenze di tipo.
</Tip>

</section>
<section title="Tag \<search_quality_reflection>">

Per impedire a Claude di riflettere sulla qualità dei risultati della ricerca con tag \<search_quality_reflection>, aggiungi "Non riflettere sulla qualità dei risultati della ricerca restituiti nella tua risposta" al tuo prompt.

</section>
<section title="Errori degli strumenti server">

Quando gli strumenti server incontrano errori (ad es., problemi di rete con Web Search), Claude gestisce questi errori in modo trasparente e tenta di fornire una risposta alternativa o una spiegazione all'utente. A differenza degli strumenti client, non devi gestire i risultati `is_error` per gli strumenti server.

Per la ricerca web in particolare, i possibili codici di errore includono:
- `too_many_requests`: Limite di velocità superato
- `invalid_input`: Parametro di query di ricerca non valido
- `max_uses_exceeded`: Utilizzi massimi dello strumento di ricerca web superati
- `query_too_long`: La query supera la lunghezza massima
- `unavailable`: Si è verificato un errore interno

</section>
<section title="Le chiamate parallele degli strumenti non funzionano">

Se Claude non sta effettuando le chiamate parallele degli strumenti quando previsto, controlla questi problemi comuni:

**1. Formattazione non corretta dei risultati dello strumento**

Il problema più comune è formattare in modo non corretto i risultati dello strumento nella cronologia della conversazione. Questo "insegna" a Claude di evitare le chiamate parallele.

Specificamente per l'utilizzo parallelo degli strumenti:
- ❌ **Sbagliato**: Invio di messaggi utente separati per ogni risultato dello strumento
- ✅ **Corretto**: Tutti i risultati dello strumento devono essere in un singolo messaggio utente

```json
// ❌ Questo riduce l'utilizzo parallelo degli strumenti
[
  {"role": "assistant", "content": [tool_use_1, tool_use_2]},
  {"role": "user", "content": [tool_result_1]},
  {"role": "user", "content": [tool_result_2]}  // Messaggio separato
]

// ✅ Questo mantiene l'utilizzo parallelo degli strumenti
[
  {"role": "assistant", "content": [tool_use_1, tool_use_2]},
  {"role": "user", "content": [tool_result_1, tool_result_2]}  // Messaggio singolo
]
```

Vedi i [requisiti di formattazione generali sopra](#handling-tool-use-and-tool-result-content-blocks) per altre regole di formattazione.

**2. Prompt debole**

Il prompt predefinito potrebbe non essere sufficiente. Usa un linguaggio più forte:

```text
<use_parallel_tool_calls>
Per la massima efficienza, ogni volta che esegui più operazioni indipendenti,
invoca tutti gli strumenti rilevanti simultaneamente piuttosto che sequenzialmente.
Dai priorità alle chiamate parallele degli strumenti ogni volta che è possibile.
</use_parallel_tool_calls>
```

**3. Misurazione dell'utilizzo parallelo degli strumenti**

Per verificare che le chiamate parallele degli strumenti funzionino:

```python
# Calcola gli strumenti medi per messaggio di chiamata dello strumento
tool_call_messages = [msg for msg in messages if any(
    block.type == "tool_use" for block in msg.content
)]
total_tool_calls = sum(
    len([b for b in msg.content if b.type == "tool_use"])
    for msg in tool_call_messages
)
avg_tools_per_message = total_tool_calls / len(tool_call_messages)
print(f"Strumenti medi per messaggio: {avg_tools_per_message}")
# Dovrebbe essere > 1.0 se le chiamate parallele funzionano
```

**4. Comportamento specifico del modello**

- Claude Opus 4.5, Opus 4.1 e Sonnet 4: Eccellono nell'utilizzo parallelo degli strumenti con prompt minimo
- Claude Sonnet 3.7: Potrebbe aver bisogno di prompt più forti o dell'[intestazione beta](/docs/it/api/beta-headers) `token-efficient-tools-2025-02-19`. Considera di [eseguire l'upgrade a Claude 4](/docs/it/about-claude/models/migrating-to-claude-4).
- Claude Haiku: Meno propenso a utilizzare gli strumenti in parallelo senza prompt esplicito

</section>