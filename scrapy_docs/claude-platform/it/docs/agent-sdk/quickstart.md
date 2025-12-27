# Guida rapida

Inizia con Python o TypeScript Agent SDK per costruire agenti AI che funzionano autonomamente

---

Usa Agent SDK per costruire un agente AI che legge il tuo codice, trova bug e li corregge, il tutto senza intervento manuale.

**Quello che farai:**
1. Configurare un progetto con Agent SDK
2. Creare un file con del codice buggy
3. Eseguire un agente che trova e corregge i bug automaticamente

## Prerequisiti

- **Node.js 18+** o **Python 3.10+**
- Un **account Anthropic** ([iscriviti qui](https://console.anthropic.com/))

## Configurazione

<Steps>
  <Step title="Installa Claude Code">
    Agent SDK utilizza Claude Code come runtime. Installalo per la tua piattaforma:

    <Tabs>
      <Tab title="macOS/Linux/WSL">
        ```bash
        curl -fsSL https://claude.ai/install.sh | bash
        ```
      </Tab>
      <Tab title="Homebrew">
        ```bash
        brew install --cask claude-code
        ```
      </Tab>
      <Tab title="npm">
        ```bash
        npm install -g @anthropic-ai/claude-code
        ```
      </Tab>
    </Tabs>

    Dopo aver installato Claude Code sulla tua macchina, esegui `claude` nel tuo terminale e segui i prompt per autenticarti. L'SDK utilizzerà questa autenticazione automaticamente.

    <Tip>
    Per ulteriori informazioni sull'installazione di Claude Code, vedi [Configurazione di Claude Code](https://docs.anthropic.com/en/docs/claude-code/setup).
    </Tip>
  </Step>

  <Step title="Crea una cartella di progetto">
    Crea una nuova directory per questa guida rapida:

    ```bash
    mkdir my-agent && cd my-agent
    ```

    Per i tuoi progetti, puoi eseguire l'SDK da qualsiasi cartella; avrà accesso ai file in quella directory e nelle sue sottodirectory per impostazione predefinita.
  </Step>

  <Step title="Installa l'SDK">
    Installa il pacchetto Agent SDK per il tuo linguaggio:

    <Tabs>
      <Tab title="TypeScript">
        ```bash
        npm install @anthropic-ai/claude-agent-sdk
        ```
      </Tab>
      <Tab title="Python (uv)">
        [uv Python package manager](https://docs.astral.sh/uv/) è un veloce gestore di pacchetti Python che gestisce gli ambienti virtuali automaticamente:
        ```bash
        uv init && uv add claude-agent-sdk
        ```
      </Tab>
      <Tab title="Python (pip)">
        Crea prima un ambiente virtuale, poi installa:
        ```bash
        python3 -m venv .venv && source .venv/bin/activate
        pip3 install claude-agent-sdk
        ```
      </Tab>
    </Tabs>
  </Step>

  <Step title="Imposta la tua chiave API">
    Se hai già autenticato Claude Code (eseguendo `claude` nel tuo terminale), l'SDK utilizza quella autenticazione automaticamente.

    Altrimenti, hai bisogno di una chiave API, che puoi ottenere dalla [Claude Console](https://console.anthropic.com/).

    Crea un file `.env` nella tua directory di progetto e archivia la chiave API lì:

    ```bash
    ANTHROPIC_API_KEY=your-api-key
    ```

    <Note>
    **Usi Amazon Bedrock, Google Vertex AI o Microsoft Azure?** Vedi le guide di configurazione per [Bedrock](https://code.claude.com/docs/en/amazon-bedrock), [Vertex AI](https://code.claude.com/docs/en/google-vertex-ai), o [Azure AI Foundry](https://code.claude.com/docs/en/azure-ai-foundry).

    Se non precedentemente approvato, Anthropic non consente ai sviluppatori di terze parti di offrire il login claude.ai o limiti di velocità per i loro prodotti, inclusi gli agenti costruiti su Claude Agent SDK. Utilizza invece i metodi di autenticazione con chiave API descritti in questo documento.
    </Note>
  </Step>
</Steps>

## Crea un file buggy

Questa guida rapida ti guida attraverso la costruzione di un agente che può trovare e correggere bug nel codice. Per prima cosa, hai bisogno di un file con alcuni bug intenzionali per l'agente da correggere. Crea `utils.py` nella directory `my-agent` e incolla il seguente codice:

```python
def calculate_average(numbers):
    total = 0
    for num in numbers:
        total += num
    return total / len(numbers)

def get_user_name(user):
    return user["name"].upper()
```

Questo codice ha due bug:
1. `calculate_average([])` si arresta in modo anomalo con divisione per zero
2. `get_user_name(None)` si arresta in modo anomalo con un TypeError

## Costruisci un agente che trova e corregge i bug

Crea `agent.py` se stai usando Python SDK, o `agent.ts` per TypeScript:

<CodeGroup>
```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage

async def main():
    # Agentic loop: streams messages as Claude works
    async for message in query(
        prompt="Review utils.py for bugs that would cause crashes. Fix any issues you find.",
        options=ClaudeAgentOptions(
            allowed_tools=["Read", "Edit", "Glob"],  # Tools Claude can use
            permission_mode="acceptEdits"            # Auto-approve file edits
        )
    ):
        # Print human-readable output
        if isinstance(message, AssistantMessage):
            for block in message.content:
                if hasattr(block, "text"):
                    print(block.text)              # Claude's reasoning
                elif hasattr(block, "name"):
                    print(f"Tool: {block.name}")   # Tool being called
        elif isinstance(message, ResultMessage):
            print(f"Done: {message.subtype}")      # Final result

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Agentic loop: streams messages as Claude works
for await (const message of query({
  prompt: "Review utils.py for bugs that would cause crashes. Fix any issues you find.",
  options: {
    allowedTools: ["Read", "Edit", "Glob"],  // Tools Claude can use
    permissionMode: "acceptEdits"            // Auto-approve file edits
  }
})) {
  // Print human-readable output
  if (message.type === "assistant" && message.message?.content) {
    for (const block of message.message.content) {
      if ("text" in block) {
        console.log(block.text);             // Claude's reasoning
      } else if ("name" in block) {
        console.log(`Tool: ${block.name}`);  // Tool being called
      }
    }
  } else if (message.type === "result") {
    console.log(`Done: ${message.subtype}`); // Final result
  }
}
```
</CodeGroup>

Questo codice ha tre parti principali:

1. **`query`**: il punto di ingresso principale che crea il loop agentico. Restituisce un iteratore asincrono, quindi usi `async for` per trasmettere i messaggi mentre Claude lavora. Vedi l'API completa nel riferimento SDK [Python](/docs/it/agent-sdk/python#query) o [TypeScript](/docs/it/agent-sdk/typescript#query).

2. **`prompt`**: quello che vuoi che Claude faccia. Claude capisce quali strumenti usare in base al compito.

3. **`options`**: configurazione per l'agente. Questo esempio usa `allowedTools` per limitare Claude a `Read`, `Edit` e `Glob`, e `permissionMode: "acceptEdits"` per approvare automaticamente i cambiamenti ai file. Altre opzioni includono `systemPrompt`, `mcpServers` e altro. Vedi tutte le opzioni per [Python](/docs/it/agent-sdk/python#claudeagentoptions) o [TypeScript](/docs/it/agent-sdk/typescript#claudeagentoptions).

Il loop `async for` continua a funzionare mentre Claude pensa, chiama strumenti, osserva i risultati e decide cosa fare dopo. Ogni iterazione produce un messaggio: il ragionamento di Claude, una chiamata a uno strumento, un risultato dello strumento o il risultato finale. L'SDK gestisce l'orchestrazione (esecuzione dello strumento, gestione del contesto, tentativi) quindi consumi semplicemente il flusso. Il loop termina quando Claude completa il compito o incontra un errore.

La gestione dei messaggi all'interno del loop filtra per l'output leggibile dall'uomo. Senza filtro, vedresti oggetti messaggio grezzi inclusa l'inizializzazione del sistema e lo stato interno, che è utile per il debug ma rumoroso altrimenti.

<Note>
Questo esempio utilizza lo streaming per mostrare i progressi in tempo reale. Se non hai bisogno di output live (ad esempio per lavori in background o pipeline CI), puoi raccogliere tutti i messaggi contemporaneamente. Vedi [Streaming vs. modalità single-turn](/docs/it/agent-sdk/streaming-vs-single-mode) per i dettagli.
</Note>

### Esegui il tuo agente

Il tuo agente è pronto. Eseguilo con il seguente comando:

<Tabs>
  <Tab title="Python">
    ```bash
    python3 agent.py
    ```
  </Tab>
  <Tab title="TypeScript">
    ```bash
    npx tsx agent.ts
    ```
  </Tab>
</Tabs>

Dopo l'esecuzione, controlla `utils.py`. Vedrai codice difensivo che gestisce elenchi vuoti e utenti nulli. Il tuo agente autonomamente:

1. **Ha letto** `utils.py` per comprendere il codice
2. **Ha analizzato** la logica e identificato i casi limite che causerebbero arresti anomali
3. **Ha modificato** il file per aggiungere una corretta gestione degli errori

Questo è ciò che rende diverso Agent SDK: Claude esegue gli strumenti direttamente invece di chiederti di implementarli.

<Note>
Se vedi "Claude Code not found", [installa Claude Code](#installa-claude-code) e riavvia il tuo terminale. Per "API key not found", [imposta la tua chiave API](#imposta-la-tua-chiave-api). Vedi la [guida completa alla risoluzione dei problemi](https://docs.anthropic.com/en/docs/claude-code/troubleshooting) per ulteriore aiuto.
</Note>

### Prova altri prompt

Ora che il tuo agente è configurato, prova alcuni prompt diversi:

- `"Add docstrings to all functions in utils.py"`
- `"Add type hints to all functions in utils.py"`
- `"Create a README.md documenting the functions in utils.py"`

### Personalizza il tuo agente

Puoi modificare il comportamento del tuo agente cambiando le opzioni. Ecco alcuni esempi:

**Aggiungi capacità di ricerca web:**

<CodeGroup>
```python Python
options=ClaudeAgentOptions(
    allowed_tools=["Read", "Edit", "Glob", "WebSearch"],
    permission_mode="acceptEdits"
)
```

```typescript TypeScript
options: {
  allowedTools: ["Read", "Edit", "Glob", "WebSearch"],
  permissionMode: "acceptEdits"
}
```
</CodeGroup>

**Dai a Claude un prompt di sistema personalizzato:**

<CodeGroup>
```python Python
options=ClaudeAgentOptions(
    allowed_tools=["Read", "Edit", "Glob"],
    permission_mode="acceptEdits",
    system_prompt="You are a senior Python developer. Always follow PEP 8 style guidelines."
)
```

```typescript TypeScript
options: {
  allowedTools: ["Read", "Edit", "Glob"],
  permissionMode: "acceptEdits",
  systemPrompt: "You are a senior Python developer. Always follow PEP 8 style guidelines."
}
```
</CodeGroup>

**Esegui comandi nel terminale:**

<CodeGroup>
```python Python
options=ClaudeAgentOptions(
    allowed_tools=["Read", "Edit", "Glob", "Bash"],
    permission_mode="acceptEdits"
)
```

```typescript TypeScript
options: {
  allowedTools: ["Read", "Edit", "Glob", "Bash"],
  permissionMode: "acceptEdits"
}
```
</CodeGroup>

Con `Bash` abilitato, prova: `"Write unit tests for utils.py, run them, and fix any failures"`

## Concetti chiave

**Strumenti** controllano cosa può fare il tuo agente:

| Strumenti | Cosa può fare l'agente |
|-------|----------------------|
| `Read`, `Glob`, `Grep` | Analisi di sola lettura |
| `Read`, `Edit`, `Glob` | Analizzare e modificare il codice |
| `Read`, `Edit`, `Bash`, `Glob`, `Grep` | Automazione completa |

**Modalità di autorizzazione** controllano quanto controllo umano desideri:

| Modalità | Comportamento | Caso d'uso |
|------|----------|----------|
| `acceptEdits` | Approva automaticamente le modifiche ai file, chiede altre azioni | Flussi di lavoro di sviluppo affidabili |
| `bypassPermissions` | Esegue senza prompt | Pipeline CI/CD, automazione |
| `default` | Richiede un callback `canUseTool` per gestire l'approvazione | Flussi di approvazione personalizzati |

L'esempio sopra utilizza la modalità `acceptEdits`, che approva automaticamente le operazioni sui file in modo che l'agente possa funzionare senza prompt interattivi. Se vuoi richiedere agli utenti l'approvazione, usa la modalità `default` e fornisci un callback [`canUseTool`](/docs/it/agent-sdk/permissions#canusetool) che raccoglie l'input dell'utente. Per un maggiore controllo, vedi [Autorizzazioni](/docs/it/agent-sdk/permissions).

## Passaggi successivi

Ora che hai creato il tuo primo agente, impara come estendere le sue capacità e adattarlo al tuo caso d'uso:

- **[Autorizzazioni](/docs/it/agent-sdk/permissions)**: controlla cosa può fare il tuo agente e quando ha bisogno di approvazione
- **[Hook](/docs/it/agent-sdk/hooks)**: esegui codice personalizzato prima o dopo le chiamate agli strumenti
- **[Sessioni](/docs/it/agent-sdk/sessions)**: costruisci agenti multi-turn che mantengono il contesto
- **[Server MCP](/docs/it/agent-sdk/mcp)**: connettiti a database, browser, API e altri sistemi esterni
- **[Hosting](/docs/it/agent-sdk/hosting)**: distribuisci agenti a Docker, cloud e CI/CD
- **[Agenti di esempio](https://github.com/anthropics/claude-agent-sdk-demos)**: vedi esempi completi: assistente email, agente di ricerca e altro