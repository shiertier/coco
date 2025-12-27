# Panoramica dell'Agent SDK

Costruisci agenti AI di produzione con Claude Code come libreria

---

<Note>
L'SDK di Claude Code è stato rinominato in Claude Agent SDK. Se stai migrando dal vecchio SDK, consulta la [Guida alla migrazione](/docs/it/agent-sdk/migration-guide).
</Note>

Costruisci agenti AI che leggono autonomamente i file, eseguono comandi, cercano sul web, modificano il codice e molto altro. L'Agent SDK ti offre gli stessi strumenti, il ciclo dell'agente e la gestione del contesto che alimentano Claude Code, programmabili in Python e TypeScript.

<CodeGroup>
```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions

async def main():
    async for message in query(
        prompt="Find and fix the bug in auth.py",
        options=ClaudeAgentOptions(allowed_tools=["Read", "Edit", "Bash"])
    ):
        print(message)  # Claude reads the file, finds the bug, edits it

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Find and fix the bug in auth.py",
  options: { allowedTools: ["Read", "Edit", "Bash"] }
})) {
  console.log(message);  // Claude reads the file, finds the bug, edits it
}
```
</CodeGroup>

L'Agent SDK include strumenti integrati per leggere file, eseguire comandi e modificare il codice, quindi il tuo agente può iniziare a lavorare immediatamente senza che tu implementi l'esecuzione degli strumenti. Tuffati nella guida rapida o esplora agenti reali costruiti con l'SDK:

<CardGroup cols={2}>
  <Card title="Guida rapida" icon="play" href="/docs/it/agent-sdk/quickstart">
    Costruisci un agente di correzione dei bug in pochi minuti
  </Card>
  <Card title="Agenti di esempio" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    Assistente email, agente di ricerca e altro ancora
  </Card>
</CardGroup>

## Capacità

Tutto ciò che rende potente Claude Code è disponibile nell'SDK:

<Tabs>
  <Tab title="Strumenti integrati">
    Il tuo agente può leggere file, eseguire comandi e cercare basi di codice subito. Gli strumenti chiave includono:

    | Strumento | Cosa fa |
    |------|--------------|
    | **Read** | Leggi qualsiasi file nella directory di lavoro |
    | **Write** | Crea nuovi file |
    | **Edit** | Apporta modifiche precise ai file esistenti |
    | **Bash** | Esegui comandi di terminale, script, operazioni git |
    | **Glob** | Trova file per pattern (`**/*.ts`, `src/**/*.py`) |
    | **Grep** | Cerca il contenuto dei file con regex |
    | **WebSearch** | Cerca sul web per informazioni attuali |
    | **WebFetch** | Recupera e analizza il contenuto delle pagine web |

    Questo esempio crea un agente che cerca commenti TODO nella tua base di codice:

    <CodeGroup>
    ```python Python
    import asyncio
    from claude_agent_sdk import query, ClaudeAgentOptions

    async def main():
        async for message in query(
            prompt="Find all TODO comments and create a summary",
            options=ClaudeAgentOptions(allowed_tools=["Read", "Glob", "Grep"])
        ):
            print(message)

    asyncio.run(main())
    ```

    ```typescript TypeScript
    import { query } from "@anthropic-ai/claude-agent-sdk";

    for await (const message of query({
      prompt: "Find all TODO comments and create a summary",
      options: { allowedTools: ["Read", "Glob", "Grep"] }
    })) {
      console.log(message);
    }
    ```
    </CodeGroup>

  </Tab>
  <Tab title="Hook">
    Esegui codice personalizzato in punti chiave del ciclo di vita dell'agente. Gli hook possono eseguire comandi shell o script personalizzati per convalidare, registrare, bloccare o trasformare il comportamento dell'agente.

    **Hook disponibili:** `PreToolUse`, `PostToolUse`, `Stop`, `SessionStart`, `SessionEnd`, `UserPromptSubmit` e altri.

    Questo esempio registra tutte le modifiche ai file in un file di audit:

    <CodeGroup>
    ```python Python
    import asyncio
    from claude_agent_sdk import query, ClaudeAgentOptions

    async def main():
        async for message in query(
            prompt="Suggest improvements to utils.py",
            options=ClaudeAgentOptions(
                permission_mode="acceptEdits",
                hooks={
                    "PostToolUse": [{
                        "matcher": "Edit|Write",
                        "hooks": [{"type": "command", "command": "echo \"$(date): file modified\" >> ./audit.log"}]
                    }]
                }
            )
        ):
            print(message)

    asyncio.run(main())
    ```

    ```typescript TypeScript
    import { query } from "@anthropic-ai/claude-agent-sdk";

    for await (const message of query({
      prompt: "Suggest improvements to utils.py",
      options: {
        permissionMode: "acceptEdits",
        hooks: {
          PostToolUse: [{
            matcher: "Edit|Write",
            hooks: [{ type: "command", command: "echo \"$(date): file modified\" >> ./audit.log" }]
          }]
        }
      }
    })) {
      console.log(message);
    }
    ```
    </CodeGroup>

    [Scopri di più sugli hook →](/docs/it/agent-sdk/hooks)
  </Tab>
  <Tab title="Subagenti">
    Genera agenti specializzati per gestire sottoattività mirate. Il tuo agente principale delega il lavoro e i subagenti riferiscono i risultati.

    Abilita lo strumento `Task` per permettere a Claude di generare subagenti quando decide che un'attività è abbastanza complessa da beneficiare della delega. Claude determina automaticamente quando utilizzare i subagenti in base alla complessità dell'attività.

    <CodeGroup>
    ```python Python
    import asyncio
    from claude_agent_sdk import query, ClaudeAgentOptions

    async def main():
        async for message in query(
            prompt="Analyze this codebase for security vulnerabilities",
            options=ClaudeAgentOptions(allowed_tools=["Read", "Glob", "Grep", "Task"])
        ):
            print(message)

    asyncio.run(main())
    ```

    ```typescript TypeScript
    import { query } from "@anthropic-ai/claude-agent-sdk";

    for await (const message of query({
      prompt: "Analyze this codebase for security vulnerabilities",
      options: {
        allowedTools: ["Read", "Glob", "Grep", "Task"]
      }
    })) {
      console.log(message);
    }
    ```
    </CodeGroup>

    Puoi anche definire tipi di agenti personalizzati con l'opzione `agents` per modelli di delega più specializzati.

    [Scopri di più sui subagenti →](/docs/it/agent-sdk/subagents)
  </Tab>
  <Tab title="MCP">
    Connettiti a sistemi esterni tramite il Model Context Protocol: database, browser, API e [centinaia di altri](https://github.com/modelcontextprotocol/servers).

    Questo esempio connette il [server Playwright MCP](https://github.com/microsoft/playwright-mcp) per dare al tuo agente capacità di automazione del browser:

    <CodeGroup>
    ```python Python
    import asyncio
    from claude_agent_sdk import query, ClaudeAgentOptions

    async def main():
        async for message in query(
            prompt="Open example.com and describe what you see",
            options=ClaudeAgentOptions(
                mcp_servers={
                    "playwright": {"command": "npx", "args": ["@playwright/mcp@latest"]}
                }
            )
        ):
            print(message)

    asyncio.run(main())
    ```

    ```typescript TypeScript
    import { query } from "@anthropic-ai/claude-agent-sdk";

    for await (const message of query({
      prompt: "Open example.com and describe what you see",
      options: {
        mcpServers: {
          playwright: { command: "npx", args: ["@playwright/mcp@latest"] }
        }
      }
    })) {
      console.log(message);
    }
    ```
    </CodeGroup>

    [Scopri di più su MCP →](/docs/it/agent-sdk/mcp)
  </Tab>
  <Tab title="Permessi">
    Controlla esattamente quali strumenti il tuo agente può utilizzare. Consenti operazioni sicure, blocca quelle pericolose o richiedi approvazione per azioni sensibili.

    Questo esempio crea un agente di sola lettura che può analizzare ma non modificare il codice:

    <CodeGroup>
    ```python Python
    import asyncio
    from claude_agent_sdk import query, ClaudeAgentOptions

    async def main():
        async for message in query(
            prompt="Review this code for best practices",
            options=ClaudeAgentOptions(
                allowed_tools=["Read", "Glob", "Grep"],
                permission_mode="bypassPermissions"
            )
        ):
            print(message)

    asyncio.run(main())
    ```

    ```typescript TypeScript
    import { query } from "@anthropic-ai/claude-agent-sdk";

    for await (const message of query({
      prompt: "Review this code for best practices",
      options: {
        allowedTools: ["Read", "Glob", "Grep"],
        permissionMode: "bypassPermissions"
      }
    })) {
      console.log(message);
    }
    ```
    </CodeGroup>

    [Scopri di più sui permessi →](/docs/it/agent-sdk/permissions)
  </Tab>
  <Tab title="Sessioni">
    Mantieni il contesto su più scambi. Claude ricorda i file letti, l'analisi eseguita e la cronologia della conversazione. Riprendi le sessioni in seguito o dividile per esplorare approcci diversi.

    Questo esempio acquisisce l'ID della sessione dalla prima query, quindi riprende per continuare con il contesto completo:

    <CodeGroup>
    ```python Python
    import asyncio
    from claude_agent_sdk import query, ClaudeAgentOptions

    async def main():
        session_id = None

        # First query: capture the session ID
        async for message in query(
            prompt="Read the authentication module",
            options=ClaudeAgentOptions(allowed_tools=["Read", "Glob"])
        ):
            if hasattr(message, 'subtype') and message.subtype == 'init':
                session_id = message.data.get('session_id')

        # Resume with full context from the first query
        async for message in query(
            prompt="Now find all places that call it",  # "it" = auth module
            options=ClaudeAgentOptions(resume=session_id)
        ):
            pass

    asyncio.run(main())
    ```

    ```typescript TypeScript
    import { query } from "@anthropic-ai/claude-agent-sdk";

    let sessionId: string | undefined;

    // First query: capture the session ID
    for await (const message of query({
      prompt: "Read the authentication module",
      options: { allowedTools: ["Read", "Glob"] }
    })) {
      if (message.type === "system" && message.subtype === "init") {
        sessionId = message.session_id;
      }
    }

    // Resume with full context from the first query
    for await (const message of query({
      prompt: "Now find all places that call it",  // "it" = auth module
      options: { resume: sessionId }
    })) {
      // Process messages...
    }
    ```
    </CodeGroup>

    [Scopri di più sulle sessioni →](/docs/it/agent-sdk/sessions)
  </Tab>
</Tabs>

### Funzionalità di Claude Code

L'SDK supporta anche la configurazione basata su filesystem di Claude Code. Per utilizzare queste funzionalità, imposta `setting_sources=["project"]` (Python) o `settingSources: ['project']` (TypeScript) nelle tue opzioni.

| Funzionalità | Descrizione | Posizione |
|---------|-------------|----------|
| [Skills](/docs/it/agent-sdk/skills) | Capacità specializzate definite in Markdown | `.claude/skills/SKILL.md` |
| [Slash commands](/docs/it/agent-sdk/slash-commands) | Comandi personalizzati per attività comuni | `.claude/commands/*.md` |
| [Memory](/docs/it/agent-sdk/modifying-system-prompts) | Contesto del progetto e istruzioni | `CLAUDE.md` o `.claude/CLAUDE.md` |
| [Plugins](/docs/it/agent-sdk/plugins) | Estendi con comandi personalizzati, agenti e server MCP | Programmatico tramite opzione `plugins` |

## Inizia

<Steps>
  <Step title="Installa Claude Code">
    L'SDK utilizza Claude Code come runtime:

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

    Consulta [Configurazione di Claude Code](https://docs.anthropic.com/en/docs/claude-code/setup) per Windows e altre opzioni.
  </Step>
  <Step title="Installa l'SDK">
    <Tabs>
      <Tab title="TypeScript">
        ```bash
        npm install @anthropic-ai/claude-agent-sdk
        ```
      </Tab>
      <Tab title="Python">
        ```bash
        pip install claude-agent-sdk
        ```
      </Tab>
    </Tabs>
  </Step>
  <Step title="Imposta la tua chiave API">
    ```bash
    export ANTHROPIC_API_KEY=your-api-key
    ```
    Ottieni la tua chiave dalla [Console](https://console.anthropic.com/).

    L'SDK supporta anche l'autenticazione tramite provider API di terze parti:

    - **Amazon Bedrock**: Imposta la variabile di ambiente `CLAUDE_CODE_USE_BEDROCK=1` e configura le credenziali AWS
    - **Google Vertex AI**: Imposta la variabile di ambiente `CLAUDE_CODE_USE_VERTEX=1` e configura le credenziali di Google Cloud
    - **Microsoft Foundry**: Imposta la variabile di ambiente `CLAUDE_CODE_USE_FOUNDRY=1` e configura le credenziali di Azure

    <Note>
    Se non precedentemente approvato, non consentiamo agli sviluppatori di terze parti di offrire il login di Claude.ai o limiti di velocità per i loro prodotti, inclusi gli agenti costruiti su Claude Agent SDK. Utilizza invece i metodi di autenticazione con chiave API descritti in questo documento.
    </Note>
  </Step>
  <Step title="Esegui il tuo primo agente">
    Questo esempio crea un agente che elenca i file nella tua directory corrente utilizzando strumenti integrati.

    <CodeGroup>
    ```python Python
    import asyncio
    from claude_agent_sdk import query, ClaudeAgentOptions

    async def main():
        async for message in query(
            prompt="What files are in this directory?",
            options=ClaudeAgentOptions(allowed_tools=["Bash", "Glob"])
        ):
            print(message)

    asyncio.run(main())
    ```

    ```typescript TypeScript
    import { query } from "@anthropic-ai/claude-agent-sdk";

    for await (const message of query({
      prompt: "What files are in this directory?",
      options: { allowedTools: ["Bash", "Glob"] },
    })) {
      console.log(message);
    }
    ```
    </CodeGroup>
  </Step>
</Steps>

**Pronto a costruire?** Segui la [Guida rapida](/docs/it/agent-sdk/quickstart) per creare un agente che trova e corregge i bug in pochi minuti.

## Confronta l'Agent SDK con altri strumenti Claude

La piattaforma Claude offre più modi per costruire con Claude. Ecco come si inserisce l'Agent SDK:

<Tabs>
  <Tab title="Agent SDK vs Client SDK">
    L'[Anthropic Client SDK](/docs/it/api/client-sdks) ti offre accesso diretto all'API: invii prompt e implementi tu stesso l'esecuzione degli strumenti. L'**Agent SDK** ti offre Claude con esecuzione degli strumenti integrata.

    Con il Client SDK, implementi un ciclo di strumenti. Con l'Agent SDK, Claude lo gestisce:

    <CodeGroup>
    ```python Python
    # Client SDK: You implement the tool loop
    response = client.messages.create(...)
    while response.stop_reason == "tool_use":
        result = your_tool_executor(response.tool_use)
        response = client.messages.create(tool_result=result, ...)

    # Agent SDK: Claude handles tools autonomously
    async for message in query(prompt="Fix the bug in auth.py"):
        print(message)
    ```

    ```typescript TypeScript
    // Client SDK: You implement the tool loop
    let response = await client.messages.create({...});
    while (response.stop_reason === "tool_use") {
      const result = yourToolExecutor(response.tool_use);
      response = await client.messages.create({ tool_result: result, ... });
    }

    // Agent SDK: Claude handles tools autonomously
    for await (const message of query({ prompt: "Fix the bug in auth.py" })) {
      console.log(message);
    }
    ```
    </CodeGroup>
  </Tab>
  <Tab title="Agent SDK vs Claude Code CLI">
    Stesse capacità, interfaccia diversa:

    | Caso d'uso | Scelta migliore |
    |----------|-------------|
    | Sviluppo interattivo | CLI |
    | Pipeline CI/CD | SDK |
    | Applicazioni personalizzate | SDK |
    | Attività una tantum | CLI |
    | Automazione di produzione | SDK |

    Molti team utilizzano entrambi: CLI per lo sviluppo quotidiano, SDK per la produzione. I flussi di lavoro si traducono direttamente tra loro.
  </Tab>
</Tabs>

## Changelog

Visualizza il changelog completo per gli aggiornamenti dell'SDK, le correzioni di bug e le nuove funzionalità:

- **TypeScript SDK**: [Visualizza CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
- **Python SDK**: [Visualizza CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

## Segnalazione di bug

Se riscontri bug o problemi con l'Agent SDK:

- **TypeScript SDK**: [Segnala i problemi su GitHub](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
- **Python SDK**: [Segnala i problemi su GitHub](https://github.com/anthropics/claude-agent-sdk-python/issues)

## Linee guida di branding

Per i partner che integrano Claude Agent SDK, l'uso del branding Claude è facoltativo. Quando fai riferimento a Claude nel tuo prodotto:

**Consentito:**
- "Claude Agent" (preferito per i menu a discesa)
- "Claude" (quando già all'interno di un menu etichettato "Agents")
- "{YourAgentName} Powered by Claude" (se hai un nome di agente esistente)

**Non consentito:**
- "Claude Code" o "Claude Code Agent"
- Arte ASCII con marchio Claude Code o elementi visivi che imitano Claude Code

Il tuo prodotto dovrebbe mantenere il proprio branding e non sembrare Claude Code o alcun prodotto Anthropic. Per domande sulla conformità del branding, contatta il nostro [team di vendita](https://www.anthropic.com/contact-sales).

## Licenza e termini

L'uso di Claude Agent SDK è disciplinato dai [Termini di servizio commerciali di Anthropic](https://www.anthropic.com/legal/commercial-terms), incluso quando lo utilizzi per alimentare prodotti e servizi che metti a disposizione dei tuoi clienti e utenti finali, tranne nella misura in cui un componente o una dipendenza specifica è coperta da una licenza diversa come indicato nel file LICENSE di quel componente.

## Passaggi successivi

<CardGroup cols={2}>
  <Card title="Guida rapida" icon="play" href="/docs/it/agent-sdk/quickstart">
    Costruisci un agente che trova e corregge i bug in pochi minuti
  </Card>
  <Card title="Agenti di esempio" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    Assistente email, agente di ricerca e altro ancora
  </Card>
  <Card title="TypeScript SDK" icon="code" href="/docs/it/agent-sdk/typescript">
    Riferimento API completo di TypeScript ed esempi
  </Card>
  <Card title="Python SDK" icon="code" href="/docs/it/agent-sdk/python">
    Riferimento API completo di Python ed esempi
  </Card>
</CardGroup>