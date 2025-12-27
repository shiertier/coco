# Subagenti nell'SDK

Lavorare con i subagenti nel Claude Agent SDK

---

I subagenti nel Claude Agent SDK sono IA specializzate che vengono orchestrate dall'agente principale.
Usa i subagenti per la gestione del contesto e la parallelizzazione.

Questa guida spiega come definire e utilizzare i subagenti nell'SDK usando il parametro `agents`.

## Panoramica

I subagenti possono essere definiti in due modi quando si utilizza l'SDK:

1. **Programmaticamente** - Usando il parametro `agents` nelle opzioni di `query()` (raccomandato per le applicazioni SDK)
2. **Basato su filesystem** - Posizionando file markdown con frontmatter YAML in directory designate (`.claude/agents/`)

Questa guida si concentra principalmente sull'approccio programmatico usando il parametro `agents`, che fornisce un'esperienza di sviluppo più integrata per le applicazioni SDK.

## Vantaggi dell'Uso dei Subagenti

### Gestione del Contesto
I subagenti mantengono un contesto separato dall'agente principale, prevenendo il sovraccarico di informazioni e mantenendo le interazioni focalizzate. Questo isolamento assicura che i compiti specializzati non inquinino il contesto della conversazione principale con dettagli irrilevanti.

**Esempio**: Un subagente `research-assistant` può esplorare dozzine di file e pagine di documentazione senza ingombrare la conversazione principale con tutti i risultati di ricerca intermedi - restituendo solo i risultati rilevanti.

### Parallelizzazione
Più subagenti possono funzionare contemporaneamente, accelerando drammaticamente i flussi di lavoro complessi.

**Esempio**: Durante una revisione del codice, puoi eseguire i subagenti `style-checker`, `security-scanner` e `test-coverage` simultaneamente, riducendo il tempo di revisione da minuti a secondi.

### Istruzioni e Conoscenze Specializzate
Ogni subagente può avere prompt di sistema personalizzati con competenze specifiche, migliori pratiche e vincoli.

**Esempio**: Un subagente `database-migration` può avere conoscenze dettagliate sulle migliori pratiche SQL, strategie di rollback e controlli di integrità dei dati che sarebbero rumore non necessario nelle istruzioni dell'agente principale.

### Restrizioni degli Strumenti
I subagenti possono essere limitati a strumenti specifici, riducendo il rischio di azioni non intenzionali.

**Esempio**: Un subagente `doc-reviewer` potrebbe avere accesso solo agli strumenti Read e Grep, assicurando che possa analizzare ma mai modificare accidentalmente i tuoi file di documentazione.

## Creazione di Subagenti

### Definizione Programmatica (Raccomandato)

Definisci i subagenti direttamente nel tuo codice usando il parametro `agents`:

```typescript
import { query } from '@anthropic-ai/claude-agent-sdk';

const result = query({
  prompt: "Rivedi il modulo di autenticazione per problemi di sicurezza",
  options: {
    agents: {
      'code-reviewer': {
        description: 'Specialista esperto nella revisione del codice. Usa per revisioni di qualità, sicurezza e manutenibilità.',
        prompt: `Sei uno specialista nella revisione del codice con competenze in sicurezza, prestazioni e migliori pratiche.

Quando rivedi il codice:
- Identifica vulnerabilità di sicurezza
- Controlla problemi di prestazioni
- Verifica l'aderenza agli standard di codifica
- Suggerisci miglioramenti specifici

Sii approfondito ma conciso nel tuo feedback.`,
        tools: ['Read', 'Grep', 'Glob'],
        model: 'sonnet'
      },
      'test-runner': {
        description: 'Esegue e analizza suite di test. Usa per l\'esecuzione di test e analisi della copertura.',
        prompt: `Sei uno specialista nell'esecuzione di test. Esegui test e fornisci analisi chiare dei risultati.

Concentrati su:
- Eseguire comandi di test
- Analizzare l'output dei test
- Identificare test falliti
- Suggerire correzioni per i fallimenti`,
        tools: ['Bash', 'Read', 'Grep'],
      }
    }
  }
});

for await (const message of result) {
  console.log(message);
}
```

### Configurazione AgentDefinition

| Campo | Tipo | Richiesto | Descrizione |
|:---|:---|:---|:---|
| `description` | `string` | Sì | Descrizione in linguaggio naturale di quando usare questo agente |
| `prompt` | `string` | Sì | Il prompt di sistema dell'agente che definisce il suo ruolo e comportamento |
| `tools` | `string[]` | No | Array di nomi di strumenti consentiti. Se omesso, eredita tutti gli strumenti |
| `model` | `'sonnet' \| 'opus' \| 'haiku' \| 'inherit'` | No | Override del modello per questo agente. Predefinito al modello principale se omesso |

### Definizione Basata su Filesystem (Alternativa)

Puoi anche definire i subagenti come file markdown in directory specifiche:

- **Livello progetto**: `.claude/agents/*.md` - Disponibili solo nel progetto corrente
- **Livello utente**: `~/.claude/agents/*.md` - Disponibili in tutti i progetti

Ogni subagente è un file markdown con frontmatter YAML:

```markdown
---
name: code-reviewer
description: Specialista esperto nella revisione del codice. Usa per revisioni di qualità, sicurezza e manutenibilità.
tools: Read, Grep, Glob, Bash
---

Il prompt di sistema del tuo subagente va qui. Questo definisce il ruolo del subagente,
le capacità e l'approccio per risolvere i problemi.
```

**Nota:** Gli agenti definiti programmaticamente (tramite il parametro `agents`) hanno precedenza sugli agenti basati su filesystem con lo stesso nome.

## Come l'SDK Usa i Subagenti

Quando si utilizza il Claude Agent SDK, i subagenti possono essere definiti programmaticamente o caricati dal filesystem. Claude:

1. **Carica gli agenti programmatici** dal parametro `agents` nelle tue opzioni
2. **Auto-rileva gli agenti del filesystem** dalle directory `.claude/agents/` (se non sovrascritti)
3. **Li invoca automaticamente** basandosi sulla corrispondenza dei compiti e sulla `description` dell'agente
4. **Usa i loro prompt specializzati** e le restrizioni degli strumenti
5. **Mantiene contesti separati** per ogni invocazione di subagente

Gli agenti definiti programmaticamente (tramite il parametro `agents`) hanno precedenza sugli agenti basati su filesystem con lo stesso nome.

## Subagenti di Esempio

Per esempi completi di subagenti inclusi revisori di codice, esecutori di test, debugger e auditor di sicurezza, vedi la [guida principale sui Subagenti](https://code.claude.com/docs/sub-agents#example-subagents). La guida include configurazioni dettagliate e migliori pratiche per creare subagenti efficaci.

## Pattern di Integrazione SDK

### Invocazione Automatica

L'SDK invocherà automaticamente i subagenti appropriati basandosi sul contesto del compito. Assicurati che il campo `description` del tuo agente indichi chiaramente quando dovrebbe essere usato:

```typescript
const result = query({
  prompt: "Ottimizza le query del database nel livello API",
  options: {
    agents: {
      'performance-optimizer': {
        description: 'Usa PROATTIVAMENTE quando le modifiche al codice potrebbero impattare le prestazioni. DEVE ESSERE USATO per compiti di ottimizzazione.',
        prompt: 'Sei uno specialista nell\'ottimizzazione delle prestazioni...',
        tools: ['Read', 'Edit', 'Bash', 'Grep'],
        model: 'sonnet'
      }
    }
  }
});
```

### Invocazione Esplicita

Gli utenti possono richiedere subagenti specifici nei loro prompt:

```typescript
const result = query({
  prompt: "Usa l'agente code-reviewer per controllare il modulo di autenticazione",
  options: {
    agents: {
      'code-reviewer': {
        description: 'Specialista esperto nella revisione del codice',
        prompt: 'Sei un revisore di codice focalizzato sulla sicurezza...',
        tools: ['Read', 'Grep', 'Glob']
      }
    }
  }
});
```

### Configurazione Dinamica degli Agenti

Puoi configurare dinamicamente gli agenti basandoti sulle esigenze della tua applicazione:

```typescript
import { query, type AgentDefinition } from '@anthropic-ai/claude-agent-sdk';

function createSecurityAgent(securityLevel: 'basic' | 'strict'): AgentDefinition {
  return {
    description: 'Revisore di codice per la sicurezza',
    prompt: `Sei un revisore di sicurezza ${securityLevel === 'strict' ? 'rigoroso' : 'bilanciato'}...`,
    tools: ['Read', 'Grep', 'Glob'],
    model: securityLevel === 'strict' ? 'opus' : 'sonnet'
  };
}

const result = query({
  prompt: "Rivedi questa PR per problemi di sicurezza",
  options: {
    agents: {
      'security-reviewer': createSecurityAgent('strict')
    }
  }
});
```

## Restrizioni degli Strumenti

I subagenti possono avere accesso limitato agli strumenti tramite il campo `tools`:

- **Ometti il campo** - L'agente eredita tutti gli strumenti disponibili (predefinito)
- **Specifica strumenti** - L'agente può usare solo gli strumenti elencati

Esempio di un agente di analisi in sola lettura:

```typescript
const result = query({
  prompt: "Analizza l'architettura di questa base di codice",
  options: {
    agents: {
      'code-analyzer': {
        description: 'Analisi statica del codice e revisione dell\'architettura',
        prompt: `Sei un analista dell'architettura del codice. Analizza la struttura del codice,
identifica pattern e suggerisci miglioramenti senza apportare modifiche.`,
        tools: ['Read', 'Grep', 'Glob']  // Nessun permesso di scrittura o esecuzione
      }
    }
  }
});
```

### Combinazioni di Strumenti Comuni

**Agenti in sola lettura** (analisi, revisione):
```typescript
tools: ['Read', 'Grep', 'Glob']
```

**Agenti di esecuzione test**:
```typescript
tools: ['Bash', 'Read', 'Grep']
```

**Agenti di modifica codice**:
```typescript
tools: ['Read', 'Edit', 'Write', 'Grep', 'Glob']
```

## Documentazione Correlata

- [Guida Principale sui Subagenti](https://code.claude.com/docs/sub-agents) - Documentazione completa sui subagenti
- [Panoramica SDK](/docs/it/agent-sdk/overview) - Panoramica del Claude Agent SDK
- [Impostazioni](https://code.claude.com/docs/settings) - Riferimento del file di configurazione
- [Comandi Slash](https://code.claude.com/docs/slash-commands) - Creazione di comandi personalizzati