# Migrazione a Claude Agent SDK

Guida per la migrazione dei SDK TypeScript e Python di Claude Code a Claude Agent SDK

---

## Panoramica

Claude Code SDK è stato rinominato in **Claude Agent SDK** e la sua documentazione è stata riorganizzata. Questo cambiamento riflette le capacità più ampie dell'SDK per la costruzione di agenti AI oltre i semplici compiti di codifica.

## Cosa è Cambiato

| Aspetto                   | Vecchio                         | Nuovo                              |
| :----------------------- | :-------------------------- | :------------------------------- |
| **Nome Pacchetto (TS/JS)** | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk` |
| **Pacchetto Python**       | `claude-code-sdk`           | `claude-agent-sdk`               |
| **Posizione Documentazione** | Documentazione Claude Code | API Guide → Sezione Agent SDK |

<Note>
**Modifiche alla Documentazione:** La documentazione di Agent SDK è stata spostata dalla documentazione di Claude Code alla API Guide sotto una sezione dedicata [Agent SDK](/docs/it/agent-sdk/overview). La documentazione di Claude Code ora si concentra sullo strumento CLI e sulle funzionalità di automazione.
</Note>

## Passaggi di Migrazione

### Per Progetti TypeScript/JavaScript

**1. Disinstallare il vecchio pacchetto:**

```bash
npm uninstall @anthropic-ai/claude-code
```

**2. Installare il nuovo pacchetto:**

```bash
npm install @anthropic-ai/claude-agent-sdk
```

**3. Aggiornare i tuoi import:**

Cambia tutti gli import da `@anthropic-ai/claude-code` a `@anthropic-ai/claude-agent-sdk`:

```typescript
// Prima
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-code";

// Dopo
import {
  query,
  tool,
  createSdkMcpServer,
} from "@anthropic-ai/claude-agent-sdk";
```

**4. Aggiornare le dipendenze di package.json:**

Se hai il pacchetto elencato nel tuo `package.json`, aggiornalo:

```json
// Prima
{
  "dependencies": {
    "@anthropic-ai/claude-code": "^1.0.0"
  }
}

// Dopo
{
  "dependencies": {
    "@anthropic-ai/claude-agent-sdk": "^0.1.0"
  }
}
```

Questo è tutto! Non sono richiesti altri cambiamenti al codice.

### Per Progetti Python

**1. Disinstallare il vecchio pacchetto:**

```bash
pip uninstall claude-code-sdk
```

**2. Installare il nuovo pacchetto:**

```bash
pip install claude-agent-sdk
```

**3. Aggiornare i tuoi import:**

Cambia tutti gli import da `claude_code_sdk` a `claude_agent_sdk`:

```python
# Prima
from claude_code_sdk import query, ClaudeCodeOptions

# Dopo
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Aggiornare i nomi dei tipi:**

Cambia `ClaudeCodeOptions` a `ClaudeAgentOptions`:

```python
# Prima
from claude_agent_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(
    model="claude-sonnet-4-5"
)

# Dopo
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(
    model="claude-sonnet-4-5"
)
```

**5. Rivedi i [cambiamenti critici](#breaking-changes)**

Apporta i cambiamenti al codice necessari per completare la migrazione.

## Cambiamenti critici

<Warning>
Per migliorare l'isolamento e la configurazione esplicita, Claude Agent SDK v0.1.0 introduce cambiamenti critici per gli utenti che migrano da Claude Code SDK. Rivedi attentamente questa sezione prima di eseguire la migrazione.
</Warning>

### Python: ClaudeCodeOptions rinominato a ClaudeAgentOptions

**Cosa è cambiato:** Il tipo SDK Python `ClaudeCodeOptions` è stato rinominato a `ClaudeAgentOptions`.

**Migrazione:**

```python
# PRIMA (v0.0.x)
from claude_agent_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(
    model="claude-sonnet-4-5",
    permission_mode="acceptEdits"
)

# DOPO (v0.1.0)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(
    model="claude-sonnet-4-5",
    permission_mode="acceptEdits"
)
```

**Perché è cambiato:** Il nome del tipo ora corrisponde al branding "Claude Agent SDK" e fornisce coerenza nelle convenzioni di denominazione dell'SDK.

### Il prompt di sistema non è più predefinito

**Cosa è cambiato:** L'SDK non utilizza più il prompt di sistema di Claude Code per impostazione predefinita.

**Migrazione:**

<CodeGroup>

```typescript TypeScript
// PRIMA (v0.0.x) - Utilizzava il prompt di sistema di Claude Code per impostazione predefinita
const result = query({ prompt: "Hello" });

// DOPO (v0.1.0) - Utilizza un prompt di sistema vuoto per impostazione predefinita
// Per ottenere il comportamento precedente, richiedi esplicitamente il preset di Claude Code:
const result = query({
  prompt: "Hello",
  options: {
    systemPrompt: { type: "preset", preset: "claude_code" }
  }
});

// Oppure usa un prompt di sistema personalizzato:
const result = query({
  prompt: "Hello",
  options: {
    systemPrompt: "You are a helpful coding assistant"
  }
});
```

```python Python
# PRIMA (v0.0.x) - Utilizzava il prompt di sistema di Claude Code per impostazione predefinita
async for message in query(prompt="Hello"):
    print(message)

# DOPO (v0.1.0) - Utilizza un prompt di sistema vuoto per impostazione predefinita
# Per ottenere il comportamento precedente, richiedi esplicitamente il preset di Claude Code:
from claude_agent_sdk import query, ClaudeAgentOptions

async for message in query(
    prompt="Hello",
    options=ClaudeAgentOptions(
        system_prompt={"type": "preset", "preset": "claude_code"}  # Usa il preset
    )
):
    print(message)

# Oppure usa un prompt di sistema personalizzato:
async for message in query(
    prompt="Hello",
    options=ClaudeAgentOptions(
        system_prompt="You are a helpful coding assistant"
    )
):
    print(message)
```

</CodeGroup>

**Perché è cambiato:** Fornisce un migliore controllo e isolamento per le applicazioni SDK. Ora puoi costruire agenti con comportamento personalizzato senza ereditare le istruzioni focalizzate sulla CLI di Claude Code.

### Le Fonti di Impostazioni Non Vengono Più Caricate per Impostazione Predefinita

**Cosa è cambiato:** L'SDK non legge più le impostazioni del filesystem (CLAUDE.md, settings.json, comandi slash, ecc.) per impostazione predefinita.

**Migrazione:**

<CodeGroup>

```typescript TypeScript
// PRIMA (v0.0.x) - Caricava tutte le impostazioni automaticamente
const result = query({ prompt: "Hello" });
// Leggerebbe da:
// - ~/.claude/settings.json (utente)
// - .claude/settings.json (progetto)
// - .claude/settings.local.json (locale)
// - File CLAUDE.md
// - Comandi slash personalizzati

// DOPO (v0.1.0) - Nessuna impostazione caricata per impostazione predefinita
// Per ottenere il comportamento precedente:
const result = query({
  prompt: "Hello",
  options: {
    settingSources: ["user", "project", "local"]
  }
});

// Oppure carica solo fonti specifiche:
const result = query({
  prompt: "Hello",
  options: {
    settingSources: ["project"]  // Solo impostazioni di progetto
  }
});
```

```python Python
# PRIMA (v0.0.x) - Caricava tutte le impostazioni automaticamente
async for message in query(prompt="Hello"):
    print(message)
# Leggerebbe da:
# - ~/.claude/settings.json (utente)
# - .claude/settings.json (progetto)
# - .claude/settings.local.json (locale)
# - File CLAUDE.md
# - Comandi slash personalizzati

# DOPO (v0.1.0) - Nessuna impostazione caricata per impostazione predefinita
# Per ottenere il comportamento precedente:
from claude_agent_sdk import query, ClaudeAgentOptions

async for message in query(
    prompt="Hello",
    options=ClaudeAgentOptions(
        setting_sources=["user", "project", "local"]
    )
):
    print(message)

# Oppure carica solo fonti specifiche:
async for message in query(
    prompt="Hello",
    options=ClaudeAgentOptions(
        setting_sources=["project"]  # Solo impostazioni di progetto
    )
):
    print(message)
```

</CodeGroup>

**Perché è cambiato:** Garantisce che le applicazioni SDK abbiano un comportamento prevedibile indipendente dalle configurazioni del filesystem locale. Questo è particolarmente importante per:
- **Ambienti CI/CD** - Comportamento coerente senza personalizzazioni locali
- **Applicazioni distribuite** - Nessuna dipendenza dalle impostazioni del filesystem
- **Test** - Ambienti di test isolati
- **Sistemi multi-tenant** - Prevenire la perdita di impostazioni tra gli utenti

<Note>
**Compatibilità all'indietro:** Se la tua applicazione si affidava alle impostazioni del filesystem (comandi slash personalizzati, istruzioni CLAUDE.md, ecc.), aggiungi `settingSources: ['user', 'project', 'local']` alle tue opzioni.
</Note>

## Perché il Cambio di Nome?

Claude Code SDK è stato originariamente progettato per compiti di codifica, ma si è evoluto in un framework potente per la costruzione di tutti i tipi di agenti AI. Il nuovo nome "Claude Agent SDK" riflette meglio le sue capacità:

- Costruzione di agenti aziendali (assistenti legali, consulenti finanziari, supporto clienti)
- Creazione di agenti di codifica specializzati (bot SRE, revisori di sicurezza, agenti di revisione del codice)
- Sviluppo di agenti personalizzati per qualsiasi dominio con uso di strumenti, integrazione MCP e altro

## Ottenere Aiuto

Se riscontri problemi durante la migrazione:

**Per TypeScript/JavaScript:**

1. Verifica che tutti gli import siano aggiornati per utilizzare `@anthropic-ai/claude-agent-sdk`
2. Verifica che il tuo package.json abbia il nuovo nome del pacchetto
3. Esegui `npm install` per assicurarti che le dipendenze siano aggiornate

**Per Python:**

1. Verifica che tutti gli import siano aggiornati per utilizzare `claude_agent_sdk`
2. Verifica che il tuo requirements.txt o pyproject.toml abbia il nuovo nome del pacchetto
3. Esegui `pip install claude-agent-sdk` per assicurarti che il pacchetto sia installato

## Passaggi Successivi

- Esplora la [Panoramica di Agent SDK](/docs/it/agent-sdk/overview) per scoprire le funzionalità disponibili
- Consulta il [Riferimento SDK TypeScript](/docs/it/agent-sdk/typescript) per la documentazione dettagliata dell'API
- Rivedi il [Riferimento SDK Python](/docs/it/agent-sdk/python) per la documentazione specifica di Python
- Scopri di più su [Strumenti Personalizzati](/docs/it/agent-sdk/custom-tools) e [Integrazione MCP](/docs/it/agent-sdk/mcp)