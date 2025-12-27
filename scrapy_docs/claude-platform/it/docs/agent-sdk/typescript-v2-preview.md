# TypeScript SDK V2 interface (preview)

Anteprima dell'SDK Agent TypeScript V2 semplificato, con pattern send/receive basati su sessione per conversazioni multi-turno.

---

<Warning>
L'interfaccia V2 è un'**anteprima instabile**. Le API potrebbero cambiare in base al feedback prima di diventare stabili. Alcune funzionalità come il session forking sono disponibili solo nell'[SDK V1](/docs/it/agent-sdk/typescript).
</Warning>

L'SDK Agent TypeScript V2 di Claude rimuove la necessità di generatori asincroni e coordinamento yield. Questo rende le conversazioni multi-turno più semplici—invece di gestire lo stato del generatore tra i turni, ogni turno è un ciclo separato `send()`/`receive()`. La superficie API si riduce a tre concetti:

- `createSession()` / `resumeSession()`: Avvia o continua una conversazione
- `session.send()`: Invia un messaggio
- `session.receive()`: Ottieni la risposta

## Installation

L'interfaccia V2 è inclusa nel pacchetto SDK esistente:

```bash
npm install @anthropic-ai/claude-agent-sdk
```

## Quick start

### One-shot prompt

Per semplici query single-turn dove non hai bisogno di mantenere una sessione, usa `unstable_v2_prompt()`. Questo esempio invia una domanda di matematica e registra la risposta:

```typescript
import { unstable_v2_prompt } from '@anthropic-ai/claude-agent-sdk'

const result = await unstable_v2_prompt('What is 2 + 2?', {
  model: 'claude-sonnet-4-5-20250929'
})
console.log(result.result)
```

<details>
<summary>Vedi la stessa operazione in V1</summary>

```typescript
import { query } from '@anthropic-ai/claude-agent-sdk'

const q = query({
  prompt: 'What is 2 + 2?',
  options: { model: 'claude-sonnet-4-5-20250929' }
})

for await (const msg of q) {
  if (msg.type === 'result') {
    console.log(msg.result)
  }
}
```

</details>

### Basic session

Per interazioni oltre un singolo prompt, crea una sessione. V2 separa l'invio e la ricezione in passaggi distinti:
- `send()` invia il tuo messaggio
- `receive()` trasmette la risposta

Questa separazione esplicita rende più facile aggiungere logica tra i turni (come elaborare le risposte prima di inviare follow-up).

L'esempio seguente crea una sessione, invia "Hello!" a Claude e stampa la risposta di testo. Usa [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management) (TypeScript 5.2+) per chiudere automaticamente la sessione quando il blocco esce. Puoi anche chiamare `session.close()` manualmente.

```typescript
import { unstable_v2_createSession } from '@anthropic-ai/claude-agent-sdk'

await using session = unstable_v2_createSession({
  model: 'claude-sonnet-4-5-20250929'
})

await session.send('Hello!')
for await (const msg of session.receive()) {
  // Filter for assistant messages to get human-readable output
  if (msg.type === 'assistant') {
    const text = msg.message.content
      .filter(block => block.type === 'text')
      .map(block => block.text)
      .join('')
    console.log(text)
  }
}
```

<details>
<summary>Vedi la stessa operazione in V1</summary>

In V1, sia l'input che l'output fluiscono attraverso un singolo generatore asincrono. Per un prompt di base questo sembra simile, ma aggiungere logica multi-turno richiede di ristrutturare per usare un generatore di input.

```typescript
import { query } from '@anthropic-ai/claude-agent-sdk'

const q = query({
  prompt: 'Hello!',
  options: { model: 'claude-sonnet-4-5-20250929' }
})

for await (const msg of q) {
  if (msg.type === 'assistant') {
    const text = msg.message.content
      .filter(block => block.type === 'text')
      .map(block => block.text)
      .join('')
    console.log(text)
  }
}
```

</details>

### Multi-turn conversation

Le sessioni mantengono il contesto attraverso più scambi. Per continuare una conversazione, chiama `send()` di nuovo sulla stessa sessione. Claude ricorda i turni precedenti.

Questo esempio pone una domanda di matematica, quindi pone un follow-up che fa riferimento alla risposta precedente:

```typescript
import { unstable_v2_createSession } from '@anthropic-ai/claude-agent-sdk'

await using session = unstable_v2_createSession({
  model: 'claude-sonnet-4-5-20250929'
})

// Turn 1
await session.send('What is 5 + 3?')
for await (const msg of session.receive()) {
  // Filter for assistant messages to get human-readable output
  if (msg.type === 'assistant') {
    const text = msg.message.content
      .filter(block => block.type === 'text')
      .map(block => block.text)
      .join('')
    console.log(text)
  }
}

// Turn 2
await session.send('Multiply that by 2')
for await (const msg of session.receive()) {
  if (msg.type === 'assistant') {
    const text = msg.message.content
      .filter(block => block.type === 'text')
      .map(block => block.text)
      .join('')
    console.log(text)
  }
}
```

<details>
<summary>Vedi la stessa operazione in V1</summary>

```typescript
import { query } from '@anthropic-ai/claude-agent-sdk'

// Must create an async iterable to feed messages
async function* createInputStream() {
  yield {
    type: 'user',
    session_id: '',
    message: { role: 'user', content: [{ type: 'text', text: 'What is 5 + 3?' }] },
    parent_tool_use_id: null
  }
  // Must coordinate when to yield next message
  yield {
    type: 'user',
    session_id: '',
    message: { role: 'user', content: [{ type: 'text', text: 'Multiply by 2' }] },
    parent_tool_use_id: null
  }
}

const q = query({
  prompt: createInputStream(),
  options: { model: 'claude-sonnet-4-5-20250929' }
})

for await (const msg of q) {
  if (msg.type === 'assistant') {
    const text = msg.message.content
      .filter(block => block.type === 'text')
      .map(block => block.text)
      .join('')
    console.log(text)
  }
}
```

</details>

### Session resume

Se hai un ID di sessione da un'interazione precedente, puoi riprendere la sessione in seguito. Questo è utile per flussi di lavoro di lunga durata o quando hai bisogno di persistere conversazioni attraverso riavvii dell'applicazione.

Questo esempio crea una sessione, memorizza il suo ID, la chiude, quindi riprende la conversazione:

```typescript
import {
  unstable_v2_createSession,
  unstable_v2_resumeSession,
  type SDKMessage
} from '@anthropic-ai/claude-agent-sdk'

// Helper to extract text from assistant messages
function getAssistantText(msg: SDKMessage): string | null {
  if (msg.type !== 'assistant') return null
  return msg.message.content
    .filter(block => block.type === 'text')
    .map(block => block.text)
    .join('')
}

// Create initial session and have a conversation
const session = unstable_v2_createSession({
  model: 'claude-sonnet-4-5-20250929'
})

await session.send('Remember this number: 42')

// Get the session ID from any received message
let sessionId: string | undefined
for await (const msg of session.receive()) {
  sessionId = msg.session_id
  const text = getAssistantText(msg)
  if (text) console.log('Initial response:', text)
}

console.log('Session ID:', sessionId)
session.close()

// Later: resume the session using the stored ID
await using resumedSession = unstable_v2_resumeSession(sessionId!, {
  model: 'claude-sonnet-4-5-20250929'
})

await resumedSession.send('What number did I ask you to remember?')
for await (const msg of resumedSession.receive()) {
  const text = getAssistantText(msg)
  if (text) console.log('Resumed response:', text)
}
```

<details>
<summary>Vedi la stessa operazione in V1</summary>

```typescript
import { query } from '@anthropic-ai/claude-agent-sdk'

// Create initial session
const initialQuery = query({
  prompt: 'Remember this number: 42',
  options: { model: 'claude-sonnet-4-5-20250929' }
})

// Get session ID from any message
let sessionId: string | undefined
for await (const msg of initialQuery) {
  sessionId = msg.session_id
  if (msg.type === 'assistant') {
    const text = msg.message.content
      .filter(block => block.type === 'text')
      .map(block => block.text)
      .join('')
    console.log('Initial response:', text)
  }
}

console.log('Session ID:', sessionId)

// Later: resume the session
const resumedQuery = query({
  prompt: 'What number did I ask you to remember?',
  options: {
    model: 'claude-sonnet-4-5-20250929',
    resume: sessionId
  }
})

for await (const msg of resumedQuery) {
  if (msg.type === 'assistant') {
    const text = msg.message.content
      .filter(block => block.type === 'text')
      .map(block => block.text)
      .join('')
    console.log('Resumed response:', text)
  }
}
```

</details>

### Cleanup

Le sessioni possono essere chiuse manualmente o automaticamente usando [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management), una funzionalità di TypeScript 5.2+ per la pulizia automatica delle risorse. Se stai usando una versione più vecchia di TypeScript o riscontri problemi di compatibilità, usa invece la pulizia manuale.

**Pulizia automatica (TypeScript 5.2+):**

```typescript
import { unstable_v2_createSession } from '@anthropic-ai/claude-agent-sdk'

await using session = unstable_v2_createSession({
  model: 'claude-sonnet-4-5-20250929'
})
// Session closes automatically when the block exits
```

**Pulizia manuale:**

```typescript
import { unstable_v2_createSession } from '@anthropic-ai/claude-agent-sdk'

const session = unstable_v2_createSession({
  model: 'claude-sonnet-4-5-20250929'
})
// ... use the session ...
session.close()
```

## API reference

### `unstable_v2_createSession()`

Crea una nuova sessione per conversazioni multi-turno.

```typescript
function unstable_v2_createSession(options: {
  model: string;
  // Additional options supported
}): Session
```

### `unstable_v2_resumeSession()`

Riprende una sessione esistente per ID.

```typescript
function unstable_v2_resumeSession(
  sessionId: string,
  options: {
    model: string;
    // Additional options supported
  }
): Session
```

### `unstable_v2_prompt()`

Funzione di convenienza one-shot per query single-turn.

```typescript
function unstable_v2_prompt(
  prompt: string,
  options: {
    model: string;
    // Additional options supported
  }
): Promise<Result>
```

### Session interface

```typescript
interface Session {
  send(message: string): Promise<void>;
  receive(): AsyncGenerator<SDKMessage>;
  close(): void;
}
```

## Feature availability

Non tutte le funzionalità di V1 sono ancora disponibili in V2. Le seguenti richiedono l'uso dell'[SDK V1](/docs/it/agent-sdk/typescript):

- Session forking (opzione `forkSession`)
- Alcuni pattern di input streaming avanzati

## Feedback

Condividi il tuo feedback sull'interfaccia V2 prima che diventi stabile. Segnala problemi e suggerimenti tramite [GitHub Issues](https://github.com/anthropics/claude-code/issues).

## See also

- [TypeScript SDK reference (V1)](/docs/it/agent-sdk/typescript) - Documentazione completa dell'SDK V1
- [SDK overview](/docs/it/agent-sdk/overview) - Concetti generali dell'SDK
- [V2 examples on GitHub](https://github.com/anthropics/claude-agent-sdk-demos/tree/main/hello-world-v2) - Esempi di codice funzionanti