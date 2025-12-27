# Agent Skills nell'SDK

Estendi Claude con capacità specializzate utilizzando Agent Skills nell'SDK di Claude Agent

---

## Panoramica

Agent Skills estendono Claude con capacità specializzate che Claude richiama autonomamente quando rilevante. Le Skills sono confezionate come file `SKILL.md` contenenti istruzioni, descrizioni e risorse di supporto opzionali.

Per informazioni complete su Skills, inclusi vantaggi, architettura e linee guida di authoring, consulta la [panoramica di Agent Skills](/docs/it/agents-and-tools/agent-skills/overview).

## Come funzionano le Skills con l'SDK

Quando si utilizza l'SDK di Claude Agent, le Skills sono:

1. **Definite come artefatti del filesystem**: Create come file `SKILL.md` in directory specifiche (`.claude/skills/`)
2. **Caricate dal filesystem**: Le Skills vengono caricate da posizioni del filesystem configurate. Devi specificare `settingSources` (TypeScript) o `setting_sources` (Python) per caricare le Skills dal filesystem
3. **Scoperte automaticamente**: Una volta caricate le impostazioni del filesystem, i metadati delle Skill vengono scoperti all'avvio dalle directory utente e progetto; il contenuto completo viene caricato quando attivato
4. **Richiamate dal modello**: Claude sceglie autonomamente quando utilizzarle in base al contesto
5. **Abilitate tramite allowed_tools**: Aggiungi `"Skill"` al tuo `allowed_tools` per abilitare le Skills

A differenza dei subagenti (che possono essere definiti programmaticamente), le Skills devono essere create come artefatti del filesystem. L'SDK non fornisce un'API programmatica per registrare le Skills.

<Note>
**Comportamento predefinito**: Per impostazione predefinita, l'SDK non carica alcuna impostazione del filesystem. Per utilizzare le Skills, devi configurare esplicitamente `settingSources: ['user', 'project']` (TypeScript) o `setting_sources=["user", "project"]` (Python) nelle tue opzioni.
</Note>

## Utilizzo delle Skills con l'SDK

Per utilizzare le Skills con l'SDK, devi:

1. Includere `"Skill"` nella tua configurazione `allowed_tools`
2. Configurare `settingSources`/`setting_sources` per caricare le Skills dal filesystem

Una volta configurato, Claude scopre automaticamente le Skills dalle directory specificate e le richiama quando rilevante per la richiesta dell'utente.

<CodeGroup>

```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions

async def main():
    options = ClaudeAgentOptions(
        cwd="/path/to/project",  # Project with .claude/skills/
        setting_sources=["user", "project"],  # Load Skills from filesystem
        allowed_tools=["Skill", "Read", "Write", "Bash"]  # Enable Skill tool
    )

    async for message in query(
        prompt="Help me process this PDF document",
        options=options
    ):
        print(message)

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me process this PDF document",
  options: {
    cwd: "/path/to/project",  // Project with .claude/skills/
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Write", "Bash"]  // Enable Skill tool
  }
})) {
  console.log(message);
}
```

</CodeGroup>

## Posizioni delle Skills

Le Skills vengono caricate dalle directory del filesystem in base alla tua configurazione `settingSources`/`setting_sources`:

- **Project Skills** (`.claude/skills/`): Condivise con il tuo team tramite git - caricate quando `setting_sources` include `"project"`
- **User Skills** (`~/.claude/skills/`): Skills personali su tutti i progetti - caricate quando `setting_sources` include `"user"`
- **Plugin Skills**: Fornite con i plugin Claude Code installati

## Creazione di Skills

Le Skills sono definite come directory contenenti un file `SKILL.md` con frontmatter YAML e contenuto Markdown. Il campo `description` determina quando Claude richiama la tua Skill.

**Struttura di directory di esempio**:
```bash
.claude/skills/processing-pdfs/
└── SKILL.md
```

Per una guida completa sulla creazione di Skills, inclusa la struttura di SKILL.md, Skills multi-file ed esempi, consulta:
- [Agent Skills in Claude Code](https://code.claude.com/docs/skills): Guida completa con esempi
- [Agent Skills Best Practices](/docs/it/agents-and-tools/agent-skills/best-practices): Linee guida di authoring e convenzioni di denominazione

## Restrizioni degli strumenti

<Note>
Il campo frontmatter `allowed-tools` in SKILL.md è supportato solo quando si utilizza direttamente Claude Code CLI. **Non si applica quando si utilizzano le Skills tramite l'SDK**.

Quando si utilizza l'SDK, controlla l'accesso agli strumenti tramite l'opzione principale `allowedTools` nella configurazione della tua query.
</Note>

Per limitare gli strumenti per le Skills nelle applicazioni SDK, utilizza l'opzione `allowedTools`:

<Note>
Le istruzioni di importazione dal primo esempio sono assunte nei seguenti frammenti di codice.
</Note>

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill", "Read", "Grep", "Glob"]  # Restricted toolset
)

async for message in query(
    prompt="Analyze the codebase structure",
    options=options
):
    print(message)
```

```typescript TypeScript
// Skills can only use Read, Grep, and Glob tools
for await (const message of query({
  prompt: "Analyze the codebase structure",
  options: {
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Grep", "Glob"]  // Restricted toolset
  }
})) {
  console.log(message);
}
```

</CodeGroup>

## Scoperta delle Skills disponibili

Per vedere quali Skills sono disponibili nella tua applicazione SDK, chiedi semplicemente a Claude:

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill"]
)

async for message in query(
    prompt="What Skills are available?",
    options=options
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "What Skills are available?",
  options: {
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill"]
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Claude elencherà le Skills disponibili in base alla tua directory di lavoro corrente e ai plugin installati.

## Test delle Skills

Testa le Skills ponendo domande che corrispondono alle loro descrizioni:

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    cwd="/path/to/project",
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill", "Read", "Bash"]
)

async for message in query(
    prompt="Extract text from invoice.pdf",
    options=options
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "Extract text from invoice.pdf",
  options: {
    cwd: "/path/to/project",
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Bash"]
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Claude richiama automaticamente la Skill rilevante se la descrizione corrisponde alla tua richiesta.

## Risoluzione dei problemi

### Skills non trovate

**Controlla la configurazione di settingSources**: Le Skills vengono caricate solo quando configuri esplicitamente `settingSources`/`setting_sources`. Questo è il problema più comune:

<CodeGroup>

```python Python
# Wrong - Skills won't be loaded
options = ClaudeAgentOptions(
    allowed_tools=["Skill"]
)

# Correct - Skills will be loaded
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// Wrong - Skills won't be loaded
const options = {
  allowedTools: ["Skill"]
};

// Correct - Skills will be loaded
const options = {
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

Per ulteriori dettagli su `settingSources`/`setting_sources`, consulta il [riferimento SDK TypeScript](/docs/it/agent-sdk/typescript#settingsource) o il [riferimento SDK Python](/docs/it/agent-sdk/python#settingsource).

**Controlla la directory di lavoro**: L'SDK carica le Skills relative all'opzione `cwd`. Assicurati che punti a una directory contenente `.claude/skills/`:

<CodeGroup>

```python Python
# Ensure your cwd points to the directory containing .claude/skills/
options = ClaudeAgentOptions(
    cwd="/path/to/project",  # Must contain .claude/skills/
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// Ensure your cwd points to the directory containing .claude/skills/
const options = {
  cwd: "/path/to/project",  // Must contain .claude/skills/
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

Consulta la sezione "Utilizzo delle Skills con l'SDK" sopra per il modello completo.

**Verifica la posizione del filesystem**:
```bash
# Check project Skills
ls .claude/skills/*/SKILL.md

# Check personal Skills
ls ~/.claude/skills/*/SKILL.md
```

### Skill non utilizzata

**Controlla che lo strumento Skill sia abilitato**: Conferma che `"Skill"` sia nel tuo `allowedTools`.

**Controlla la descrizione**: Assicurati che sia specifica e includa parole chiave rilevanti. Consulta [Agent Skills Best Practices](/docs/it/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) per una guida sulla scrittura di descrizioni efficaci.

### Risoluzione dei problemi aggiuntiva

Per la risoluzione generale dei problemi delle Skills (sintassi YAML, debug, ecc.), consulta la [sezione di risoluzione dei problemi delle Skills di Claude Code](https://code.claude.com/docs/skills#troubleshooting).

## Documentazione correlata

### Guide sulle Skills
- [Agent Skills in Claude Code](https://code.claude.com/docs/skills): Guida completa sulle Skills con creazione, esempi e risoluzione dei problemi
- [Agent Skills Overview](/docs/it/agents-and-tools/agent-skills/overview): Panoramica concettuale, vantaggi e architettura
- [Agent Skills Best Practices](/docs/it/agents-and-tools/agent-skills/best-practices): Linee guida di authoring per Skills efficaci
- [Agent Skills Cookbook](https://github.com/anthropics/claude-cookbooks/tree/main/skills): Skills di esempio e template

### Risorse SDK
- [Subagents in the SDK](/docs/it/agent-sdk/subagents): Agenti basati su filesystem simili con opzioni programmatiche
- [Slash Commands in the SDK](/docs/it/agent-sdk/slash-commands): Comandi richiamati dall'utente
- [SDK Overview](/docs/it/agent-sdk/overview): Concetti generali dell'SDK
- [TypeScript SDK Reference](/docs/it/agent-sdk/typescript): Documentazione API completa
- [Python SDK Reference](/docs/it/agent-sdk/python): Documentazione API completa