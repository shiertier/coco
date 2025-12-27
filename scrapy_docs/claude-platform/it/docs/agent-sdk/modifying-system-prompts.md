# Modifica dei prompt di sistema

Impara come personalizzare il comportamento di Claude modificando i prompt di sistema utilizzando tre approcci - stili di output, systemPrompt con append e prompt di sistema personalizzati.

---

I prompt di sistema definiscono il comportamento, le capacità e lo stile di risposta di Claude. Il Claude Agent SDK fornisce tre modi per personalizzare i prompt di sistema: utilizzando stili di output (configurazioni persistenti basate su file), aggiungendo al prompt di Claude Code, o utilizzando un prompt completamente personalizzato.

## Comprendere i prompt di sistema

Un prompt di sistema è il set di istruzioni iniziali che modella come Claude si comporta durante una conversazione.

<Note>
**Comportamento predefinito:** L'Agent SDK utilizza un **prompt di sistema vuoto** per impostazione predefinita per la massima flessibilità. Per utilizzare il prompt di sistema di Claude Code (istruzioni per strumenti, linee guida per il codice, ecc.), specifica `systemPrompt: { preset: "claude_code" }` in TypeScript o `system_prompt="claude_code"` in Python.
</Note>

Il prompt di sistema di Claude Code include:

- Istruzioni per l'uso degli strumenti e strumenti disponibili
- Linee guida per lo stile e la formattazione del codice
- Impostazioni del tono di risposta e verbosità
- Istruzioni di sicurezza e protezione
- Contesto sulla directory di lavoro corrente e l'ambiente

## Metodi di modifica

### Metodo 1: File CLAUDE.md (istruzioni a livello di progetto)

I file CLAUDE.md forniscono contesto e istruzioni specifici del progetto che vengono automaticamente letti dall'Agent SDK quando viene eseguito in una directory. Servono come "memoria" persistente per il tuo progetto.

#### Come funziona CLAUDE.md con l'SDK

**Posizione e scoperta:**

- **Livello progetto:** `CLAUDE.md` o `.claude/CLAUDE.md` nella tua directory di lavoro
- **Livello utente:** `~/.claude/CLAUDE.md` per istruzioni globali in tutti i progetti

**IMPORTANTE:** L'SDK legge i file CLAUDE.md solo quando configuri esplicitamente `settingSources` (TypeScript) o `setting_sources` (Python):

- Includi `'project'` per caricare CLAUDE.md a livello di progetto
- Includi `'user'` per caricare CLAUDE.md a livello utente (`~/.claude/CLAUDE.md`)

Il preset del prompt di sistema `claude_code` NON carica automaticamente CLAUDE.md - devi anche specificare le fonti delle impostazioni.

**Formato del contenuto:**
I file CLAUDE.md utilizzano markdown semplice e possono contenere:

- Linee guida e standard di codifica
- Contesto specifico del progetto
- Comandi o flussi di lavoro comuni
- Convenzioni API
- Requisiti di test

#### Esempio CLAUDE.md

```markdown
# Linee Guida del Progetto

## Stile del Codice

- Usa la modalità strict di TypeScript
- Preferisci componenti funzionali in React
- Includi sempre commenti JSDoc per le API pubbliche

## Test

- Esegui `npm test` prima di fare commit
- Mantieni >80% di copertura del codice
- Usa jest per unit test, playwright per E2E

## Comandi

- Build: `npm run build`
- Server dev: `npm run dev`
- Controllo tipi: `npm run typecheck`
```

#### Utilizzare CLAUDE.md con l'SDK

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// IMPORTANTE: Devi specificare settingSources per caricare CLAUDE.md
// Il preset claude_code da solo NON carica i file CLAUDE.md
const messages = [];

for await (const message of query({
  prompt: "Aggiungi un nuovo componente React per i profili utente",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code", // Usa il prompt di sistema di Claude Code
    },
    settingSources: ["project"], // Richiesto per caricare CLAUDE.md dal progetto
  },
})) {
  messages.push(message);
}

// Ora Claude ha accesso alle tue linee guida del progetto da CLAUDE.md
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# IMPORTANTE: Devi specificare setting_sources per caricare CLAUDE.md
# Il preset claude_code da solo NON carica i file CLAUDE.md
messages = []

async for message in query(
    prompt="Aggiungi un nuovo componente React per i profili utente",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code"  # Usa il prompt di sistema di Claude Code
        },
        setting_sources=["project"]  # Richiesto per caricare CLAUDE.md dal progetto
    )
):
    messages.append(message)

# Ora Claude ha accesso alle tue linee guida del progetto da CLAUDE.md
```

</CodeGroup>

#### Quando utilizzare CLAUDE.md

**Migliore per:**

- **Contesto condiviso dal team** - Linee guida che tutti dovrebbero seguire
- **Convenzioni del progetto** - Standard di codifica, struttura dei file, pattern di denominazione
- **Comandi comuni** - Comandi di build, test, deploy specifici del tuo progetto
- **Memoria a lungo termine** - Contesto che dovrebbe persistere in tutte le sessioni
- **Istruzioni sotto controllo versione** - Commit su git così il team rimane sincronizzato

**Caratteristiche chiave:**

- ✅ Persistente in tutte le sessioni di un progetto
- ✅ Condiviso con il team tramite git
- ✅ Scoperta automatica (non sono necessarie modifiche al codice)
- ⚠️ Richiede il caricamento delle impostazioni tramite `settingSources`

### Metodo 2: Stili di output (configurazioni persistenti)

Gli stili di output sono configurazioni salvate che modificano il prompt di sistema di Claude. Sono memorizzati come file markdown e possono essere riutilizzati tra sessioni e progetti.

#### Creare uno stile di output

<CodeGroup>

```typescript TypeScript
import { writeFile, mkdir } from "fs/promises";
import { join } from "path";
import { homedir } from "os";

async function createOutputStyle(
  name: string,
  description: string,
  prompt: string
) {
  // Livello utente: ~/.claude/output-styles
  // Livello progetto: .claude/output-styles
  const outputStylesDir = join(homedir(), ".claude", "output-styles");

  await mkdir(outputStylesDir, { recursive: true });

  const content = `---
name: ${name}
description: ${description}
---

${prompt}`;

  const filePath = join(
    outputStylesDir,
    `${name.toLowerCase().replace(/\s+/g, "-")}.md`
  );
  await writeFile(filePath, content, "utf-8");
}

// Esempio: Crea uno specialista di revisione del codice
await createOutputStyle(
  "Code Reviewer",
  "Assistente di revisione del codice approfondita",
  `Sei un esperto revisore di codice.

Per ogni sottomissione di codice:
1. Controlla bug e problemi di sicurezza
2. Valuta le prestazioni
3. Suggerisci miglioramenti
4. Valuta la qualità del codice (1-10)`
);
```

```python Python
from pathlib import Path

async def create_output_style(name: str, description: str, prompt: str):
    # Livello utente: ~/.claude/output-styles
    # Livello progetto: .claude/output-styles
    output_styles_dir = Path.home() / '.claude' / 'output-styles'

    output_styles_dir.mkdir(parents=True, exist_ok=True)

    content = f"""---
name: {name}
description: {description}
---

{prompt}"""

    file_name = name.lower().replace(' ', '-') + '.md'
    file_path = output_styles_dir / file_name
    file_path.write_text(content, encoding='utf-8')

# Esempio: Crea uno specialista di revisione del codice
await create_output_style(
    'Code Reviewer',
    'Assistente di revisione del codice approfondita',
    """Sei un esperto revisore di codice.

Per ogni sottomissione di codice:
1. Controlla bug e problemi di sicurezza
2. Valuta le prestazioni
3. Suggerisci miglioramenti
4. Valuta la qualità del codice (1-10)"""
)
```

</CodeGroup>

#### Utilizzare gli stili di output

Una volta creati, attiva gli stili di output tramite:

- **CLI**: `/output-style [nome-stile]`
- **Impostazioni**: `.claude/settings.local.json`
- **Crea nuovo**: `/output-style:new [descrizione]`

**Nota per gli utenti SDK:** Gli stili di output vengono caricati quando includi `settingSources: ['user']` o `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` o `setting_sources=["project"]` (Python) nelle tue opzioni.

### Metodo 3: Utilizzare `systemPrompt` con append

Puoi utilizzare il preset Claude Code con una proprietà `append` per aggiungere le tue istruzioni personalizzate preservando tutte le funzionalità integrate.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const messages = [];

for await (const message of query({
  prompt: "Aiutami a scrivere una funzione Python per calcolare i numeri di fibonacci",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append:
        "Includi sempre docstring dettagliate e type hints nel codice Python.",
    },
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

messages = []

async for message in query(
    prompt="Aiutami a scrivere una funzione Python per calcolare i numeri di fibonacci",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": "Includi sempre docstring dettagliate e type hints nel codice Python."
        }
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

### Metodo 4: Prompt di sistema personalizzati

Puoi fornire una stringa personalizzata come `systemPrompt` per sostituire completamente il default con le tue istruzioni.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const customPrompt = `Sei uno specialista di codifica Python.
Segui queste linee guida:
- Scrivi codice pulito e ben documentato
- Usa type hints per tutte le funzioni
- Includi docstring complete
- Preferisci pattern di programmazione funzionale quando appropriato
- Spiega sempre le tue scelte di codice`;

const messages = [];

for await (const message of query({
  prompt: "Crea una pipeline di elaborazione dati",
  options: {
    systemPrompt: customPrompt,
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

custom_prompt = """Sei uno specialista di codifica Python.
Segui queste linee guida:
- Scrivi codice pulito e ben documentato
- Usa type hints per tutte le funzioni
- Includi docstring complete
- Preferisci pattern di programmazione funzionale quando appropriato
- Spiega sempre le tue scelte di codice"""

messages = []

async for message in query(
    prompt="Crea una pipeline di elaborazione dati",
    options=ClaudeAgentOptions(
        system_prompt=custom_prompt
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

## Confronto di tutti e quattro gli approcci

| Caratteristica          | CLAUDE.md           | Stili di Output    | `systemPrompt` con append | `systemPrompt` Personalizzato |
| --- | --- | --- | --- | --- |
| **Persistenza**         | File per progetto  | Salvati come file  | Solo sessione               | Solo sessione              |
| **Riutilizzabilità**    | Per progetto       | Tra progetti       | Duplicazione codice         | Duplicazione codice        |
| **Gestione**            | Su filesystem      | CLI + file         | Nel codice                  | Nel codice                 |
| **Strumenti predefiniti** | Preservati        | Preservati         | Preservati                  | Persi (a meno che inclusi) |
| **Sicurezza integrata** | Mantenuta          | Mantenuta          | Mantenuta                   | Deve essere aggiunta       |
| **Contesto ambiente**   | Automatico         | Automatico         | Automatico                  | Deve essere fornito        |
| **Livello personalizzazione** | Solo aggiunte | Sostituisce default | Solo aggiunte              | Controllo completo         |
| **Controllo versione**  | Con progetto       | Sì                 | Con codice                  | Con codice                 |
| **Ambito**              | Specifico progetto | Utente o progetto  | Sessione codice             | Sessione codice            |

**Nota:** "Con append" significa utilizzare `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` in TypeScript o `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` in Python.

## Casi d'uso e migliori pratiche

### Quando utilizzare CLAUDE.md

**Migliore per:**

- Standard e convenzioni di codifica specifici del progetto
- Documentazione della struttura e architettura del progetto
- Elenco di comandi comuni (build, test, deploy)
- Contesto condiviso dal team che dovrebbe essere sotto controllo versione
- Istruzioni che si applicano a tutto l'uso dell'SDK in un progetto

**Esempi:**

- "Tutti gli endpoint API dovrebbero utilizzare pattern async/await"
- "Esegui `npm run lint:fix` prima di fare commit"
- "Le migrazioni del database sono nella directory `migrations/`"

**Importante:** Per caricare i file CLAUDE.md, devi impostare esplicitamente `settingSources: ['project']` (TypeScript) o `setting_sources=["project"]` (Python). Il preset del prompt di sistema `claude_code` NON carica automaticamente CLAUDE.md senza questa impostazione.

### Quando utilizzare gli stili di output

**Migliore per:**

- Modifiche comportamentali persistenti tra sessioni
- Configurazioni condivise dal team
- Assistenti specializzati (revisore di codice, data scientist, DevOps)
- Modifiche complesse del prompt che necessitano di versioning

**Esempi:**

- Creare un assistente dedicato all'ottimizzazione SQL
- Costruire un revisore di codice focalizzato sulla sicurezza
- Sviluppare un assistente didattico con pedagogia specifica

### Quando utilizzare `systemPrompt` con append

**Migliore per:**

- Aggiungere standard o preferenze di codifica specifici
- Personalizzare la formattazione dell'output
- Aggiungere conoscenza specifica del dominio
- Modificare la verbosità della risposta
- Migliorare il comportamento predefinito di Claude Code senza perdere le istruzioni degli strumenti

### Quando utilizzare `systemPrompt` personalizzato

**Migliore per:**

- Controllo completo sul comportamento di Claude
- Compiti specializzati di singola sessione
- Testare nuove strategie di prompt
- Situazioni in cui gli strumenti predefiniti non sono necessari
- Costruire agenti specializzati con comportamento unico

## Combinare gli approcci

Puoi combinare questi metodi per la massima flessibilità:

### Esempio: Stile di output con aggiunte specifiche della sessione

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Assumendo che lo stile di output "Code Reviewer" sia attivo (tramite /output-style)
// Aggiungi aree di focus specifiche della sessione
const messages = [];

for await (const message of query({
  prompt: "Rivedi questo modulo di autenticazione",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: `
        Per questa revisione, dai priorità a:
        - Conformità OAuth 2.0
        - Sicurezza dell'archiviazione token
        - Gestione delle sessioni
      `,
    },
  },
})) {
  messages.push(message);
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# Assumendo che lo stile di output "Code Reviewer" sia attivo (tramite /output-style)
# Aggiungi aree di focus specifiche della sessione
messages = []

async for message in query(
    prompt="Rivedi questo modulo di autenticazione",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": """
            Per questa revisione, dai priorità a:
            - Conformità OAuth 2.0
            - Sicurezza dell'archiviazione token
            - Gestione delle sessioni
            """
        }
    )
):
    messages.append(message)
```

</CodeGroup>

## Vedi anche

- [Stili di output](https://code.claude.com/docs/output-styles) - Documentazione completa degli stili di output
- [Guida TypeScript SDK](/docs/it/agent-sdk/typescript) - Guida completa all'uso dell'SDK
- [Riferimento TypeScript SDK](https://code.claude.com/docs/typescript-sdk-reference) - Documentazione completa dell'API
- [Guida alla configurazione](https://code.claude.com/docs/configuration) - Opzioni di configurazione generali