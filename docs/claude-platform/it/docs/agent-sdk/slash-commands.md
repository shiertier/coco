# Comandi Slash nell'SDK

Impara come utilizzare i comandi slash per controllare le sessioni di Claude Code attraverso l'SDK

---

I comandi slash forniscono un modo per controllare le sessioni di Claude Code con comandi speciali che iniziano con `/`. Questi comandi possono essere inviati attraverso l'SDK per eseguire azioni come cancellare la cronologia delle conversazioni, compattare i messaggi o ottenere aiuto.

## Scoprire i Comandi Slash Disponibili

L'SDK di Claude Agent fornisce informazioni sui comandi slash disponibili nel messaggio di inizializzazione del sistema. Accedi a queste informazioni quando la tua sessione inizia:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Ciao Claude",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Comandi slash disponibili:", message.slash_commands);
    // Output di esempio: ["/compact", "/clear", "/help"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="Ciao Claude",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            print("Comandi slash disponibili:", message.slash_commands)
            # Output di esempio: ["/compact", "/clear", "/help"]

asyncio.run(main())
```

</CodeGroup>

## Invio di Comandi Slash

Invia i comandi slash includendoli nella tua stringa di prompt, proprio come il testo normale:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Invia un comando slash
for await (const message of query({
  prompt: "/compact",
  options: { maxTurns: 1 }
})) {
  if (message.type === "result") {
    console.log("Comando eseguito:", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Invia un comando slash
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if message.type == "result":
            print("Comando eseguito:", message.result)

asyncio.run(main())
```

</CodeGroup>

## Comandi Slash Comuni

### `/compact` - Compatta la Cronologia delle Conversazioni

Il comando `/compact` riduce la dimensione della cronologia delle tue conversazioni riassumendo i messaggi più vecchi preservando il contesto importante:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "/compact",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "compact_boundary") {
    console.log("Compattazione completata");
    console.log("Token pre-compattazione:", message.compact_metadata.pre_tokens);
    console.log("Trigger:", message.compact_metadata.trigger);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if (message.type == "system" and 
            message.subtype == "compact_boundary"):
            print("Compattazione completata")
            print("Token pre-compattazione:", 
                  message.compact_metadata.pre_tokens)
            print("Trigger:", message.compact_metadata.trigger)

asyncio.run(main())
```

</CodeGroup>

### `/clear` - Cancella Conversazione

Il comando `/clear` inizia una conversazione fresca cancellando tutta la cronologia precedente:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Cancella conversazione e inizia da capo
for await (const message of query({
  prompt: "/clear",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Conversazione cancellata, nuova sessione iniziata");
    console.log("ID Sessione:", message.session_id);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Cancella conversazione e inizia da capo
    async for message in query(
        prompt="/clear",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            print("Conversazione cancellata, nuova sessione iniziata")
            print("ID Sessione:", message.session_id)

asyncio.run(main())
```

</CodeGroup>

## Creazione di Comandi Slash Personalizzati

Oltre a utilizzare i comandi slash integrati, puoi creare i tuoi comandi personalizzati che sono disponibili attraverso l'SDK. I comandi personalizzati sono definiti come file markdown in directory specifiche, simile a come sono configurati i subagenti.

### Posizioni dei File

I comandi slash personalizzati sono memorizzati in directory designate basate sul loro ambito:

- **Comandi di progetto**: `.claude/commands/` - Disponibili solo nel progetto corrente
- **Comandi personali**: `~/.claude/commands/` - Disponibili in tutti i tuoi progetti

### Formato del File

Ogni comando personalizzato è un file markdown dove:
- Il nome del file (senza estensione `.md`) diventa il nome del comando
- Il contenuto del file definisce cosa fa il comando
- Il frontmatter YAML opzionale fornisce la configurazione

#### Esempio Base

Crea `.claude/commands/refactor.md`:

```markdown
Refactorizza il codice selezionato per migliorare la leggibilità e la manutenibilità.
Concentrati sui principi del codice pulito e le migliori pratiche.
```

Questo crea il comando `/refactor` che puoi utilizzare attraverso l'SDK.

#### Con Frontmatter

Crea `.claude/commands/security-check.md`:

```markdown
---
allowed-tools: Read, Grep, Glob
description: Esegui scansione vulnerabilità di sicurezza
model: claude-3-5-sonnet-20241022
---

Analizza la base di codice per vulnerabilità di sicurezza incluse:
- Rischi di SQL injection
- Vulnerabilità XSS
- Credenziali esposte
- Configurazioni insicure
```

### Utilizzo di Comandi Personalizzati nell'SDK

Una volta definiti nel filesystem, i comandi personalizzati sono automaticamente disponibili attraverso l'SDK:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Usa un comando personalizzato
for await (const message of query({
  prompt: "/refactor src/auth/login.ts",
  options: { maxTurns: 3 }
})) {
  if (message.type === "assistant") {
    console.log("Suggerimenti di refactoring:", message.message);
  }
}

// I comandi personalizzati appaiono nella lista slash_commands
for await (const message of query({
  prompt: "Ciao",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // Includerà sia comandi integrati che personalizzati
    console.log("Comandi disponibili:", message.slash_commands);
    // Esempio: ["/compact", "/clear", "/help", "/refactor", "/security-check"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Usa un comando personalizzato
    async for message in query(
        prompt="/refactor src/auth/login.py",
        options={"max_turns": 3}
    ):
        if message.type == "assistant":
            print("Suggerimenti di refactoring:", message.message)
    
    # I comandi personalizzati appaiono nella lista slash_commands
    async for message in query(
        prompt="Ciao",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            # Includerà sia comandi integrati che personalizzati
            print("Comandi disponibili:", message.slash_commands)
            # Esempio: ["/compact", "/clear", "/help", "/refactor", "/security-check"]

asyncio.run(main())
```

</CodeGroup>

### Funzionalità Avanzate

#### Argomenti e Segnaposto

I comandi personalizzati supportano argomenti dinamici utilizzando segnaposto:

Crea `.claude/commands/fix-issue.md`:

```markdown
---
argument-hint: [numero-issue] [priorità]
description: Risolvi un issue GitHub
---

Risolvi issue #$1 con priorità $2.
Controlla la descrizione dell'issue e implementa le modifiche necessarie.
```

Usa nell'SDK:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Passa argomenti al comando personalizzato
for await (const message of query({
  prompt: "/fix-issue 123 alta",
  options: { maxTurns: 5 }
})) {
  // Il comando processerà con $1="123" e $2="alta"
  if (message.type === "result") {
    console.log("Issue risolto:", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Passa argomenti al comando personalizzato
    async for message in query(
        prompt="/fix-issue 123 alta",
        options={"max_turns": 5}
    ):
        # Il comando processerà con $1="123" e $2="alta"
        if message.type == "result":
            print("Issue risolto:", message.result)

asyncio.run(main())
```

</CodeGroup>

#### Esecuzione di Comandi Bash

I comandi personalizzati possono eseguire comandi bash e includere il loro output:

Crea `.claude/commands/git-commit.md`:

```markdown
---
allowed-tools: Bash(git add:*), Bash(git status:*), Bash(git commit:*)
description: Crea un commit git
---

## Contesto

- Stato corrente: !`git status`
- Diff corrente: !`git diff HEAD`

## Compito

Crea un commit git con messaggio appropriato basato sui cambiamenti.
```

#### Riferimenti ai File

Includi contenuti di file utilizzando il prefisso `@`:

Crea `.claude/commands/review-config.md`:

```markdown
---
description: Rivedi file di configurazione
---

Rivedi i seguenti file di configurazione per problemi:
- Configurazione package: @package.json
- Configurazione TypeScript: @tsconfig.json
- Configurazione ambiente: @.env

Controlla per problemi di sicurezza, dipendenze obsolete e configurazioni errate.
```

### Organizzazione con Namespace

Organizza i comandi in sottodirectory per una migliore struttura:

```bash
.claude/commands/
├── frontend/
│   ├── component.md      # Crea /component (project:frontend)
│   └── style-check.md     # Crea /style-check (project:frontend)
├── backend/
│   ├── api-test.md        # Crea /api-test (project:backend)
│   └── db-migrate.md      # Crea /db-migrate (project:backend)
└── review.md              # Crea /review (project)
```

La sottodirectory appare nella descrizione del comando ma non influisce sul nome del comando stesso.

### Esempi Pratici

#### Comando di Code Review

Crea `.claude/commands/code-review.md`:

```markdown
---
allowed-tools: Read, Grep, Glob, Bash(git diff:*)
description: Code review completo
---

## File Modificati
!`git diff --name-only HEAD~1`

## Modifiche Dettagliate
!`git diff HEAD~1`

## Checklist di Review

Rivedi le modifiche sopra per:
1. Qualità del codice e leggibilità
2. Vulnerabilità di sicurezza
3. Implicazioni di performance
4. Copertura dei test
5. Completezza della documentazione

Fornisci feedback specifico e attuabile organizzato per priorità.
```

#### Comando Test Runner

Crea `.claude/commands/test.md`:

```markdown
---
allowed-tools: Bash, Read, Edit
argument-hint: [pattern-test]
description: Esegui test con pattern opzionale
---

Esegui test che corrispondono al pattern: $ARGUMENTS

1. Rileva il framework di test (Jest, pytest, ecc.)
2. Esegui test con il pattern fornito
3. Se i test falliscono, analizza e correggili
4. Ri-esegui per verificare le correzioni
```

Usa questi comandi attraverso l'SDK:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Esegui code review
for await (const message of query({
  prompt: "/code-review",
  options: { maxTurns: 3 }
})) {
  // Processa feedback di review
}

// Esegui test specifici
for await (const message of query({
  prompt: "/test auth",
  options: { maxTurns: 5 }
})) {
  // Gestisci risultati dei test
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Esegui code review
    async for message in query(
        prompt="/code-review",
        options={"max_turns": 3}
    ):
        # Processa feedback di review
        pass
    
    # Esegui test specifici
    async for message in query(
        prompt="/test auth",
        options={"max_turns": 5}
    ):
        # Gestisci risultati dei test
        pass

asyncio.run(main())
```

</CodeGroup>

## Vedi Anche

- [Comandi Slash](https://code.claude.com/docs/slash-commands) - Documentazione completa dei comandi slash
- [Subagenti nell'SDK](/docs/it/agent-sdk/subagents) - Configurazione basata su filesystem simile per i subagenti
- [Riferimento SDK TypeScript](https://code.claude.com/docs/typescript-sdk-reference) - Documentazione API completa
- [Panoramica SDK](/docs/it/agent-sdk/overview) - Concetti generali dell'SDK
- [Riferimento CLI](https://code.claude.com/docs/cli-reference) - Interfaccia a riga di comando