# Agent Skills

Agent Skills sono capacità modulari che estendono la funzionalità di Claude. Ogni Skill racchiude istruzioni, metadati e risorse opzionali (script, template) che Claude utilizza automaticamente quando rilevante.

---

## Perché usare Skills

Skills sono risorse riutilizzabili basate sul filesystem che forniscono a Claude competenze specifiche di dominio: flussi di lavoro, contesto e best practice che trasformano agenti generici in specialisti. A differenza dei prompt (istruzioni a livello di conversazione per attività una tantum), Skills si caricano su richiesta ed eliminano la necessità di fornire ripetutamente la stessa guida in più conversazioni.

**Vantaggi principali**:
- **Specializza Claude**: Personalizza le capacità per attività specifiche di dominio
- **Riduci la ripetizione**: Crea una volta, usa automaticamente
- **Componi capacità**: Combina Skills per costruire flussi di lavoro complessi

<Note>
Per un approfondimento sull'architettura e le applicazioni nel mondo reale di Agent Skills, leggi il nostro blog di ingegneria: [Equipping agents for the real world with Agent Skills](https://www.anthropic.com/engineering/equipping-agents-for-the-real-world-with-agent-skills).
</Note>

## Utilizzo di Skills

Anthropic fornisce Agent Skills pre-costruite per attività comuni con documenti (PowerPoint, Excel, Word, PDF), e puoi creare le tue Skills personalizzate. Entrambe funzionano allo stesso modo. Claude le utilizza automaticamente quando rilevante per la tua richiesta.

**Agent Skills pre-costruite** sono disponibili per tutti gli utenti su claude.ai e tramite l'API Claude. Consulta la sezione [Available Skills](#available-skills) di seguito per l'elenco completo.

**Custom Skills** ti permettono di racchiudere competenze di dominio e conoscenze organizzative. Sono disponibili su tutti i prodotti di Claude: creale in Claude Code, caricale tramite l'API, o aggiungile nelle impostazioni di claude.ai.

<Note>
**Inizia:**
- Per Agent Skills pre-costruite: Consulta il [tutorial di avvio rapido](/docs/it/agents-and-tools/agent-skills/quickstart) per iniziare a usare PowerPoint, Excel, Word e PDF skills nell'API
- Per Custom Skills: Consulta il [Agent Skills Cookbook](https://github.com/anthropics/claude-cookbooks/tree/main/skills) per imparare come creare le tue Skills
</Note>

## Come funzionano le Skills

Skills sfruttano l'ambiente VM di Claude per fornire capacità oltre ciò che è possibile con i soli prompt. Claude opera in una macchina virtuale con accesso al filesystem, permettendo alle Skills di esistere come directory contenenti istruzioni, codice eseguibile e materiali di riferimento, organizzati come una guida di onboarding che creeresti per un nuovo membro del team.

Questa architettura basata su filesystem abilita **progressive disclosure**: Claude carica le informazioni in fasi secondo le necessità, piuttosto che consumare il contesto in anticipo.

### Tre tipi di contenuto Skill, tre livelli di caricamento

Le Skills possono contenere tre tipi di contenuto, ognuno caricato in momenti diversi:

### Livello 1: Metadati (sempre caricati)

**Tipo di contenuto: Istruzioni**. Il frontmatter YAML della Skill fornisce informazioni di scoperta:

```yaml
---
name: pdf-processing
description: Extract text and tables from PDF files, fill forms, merge documents. Use when working with PDF files or when the user mentions PDFs, forms, or document extraction.
---
```

Claude carica questi metadati all'avvio e li include nel prompt di sistema. Questo approccio leggero significa che puoi installare molte Skills senza penalità di contesto; Claude sa solo che ogni Skill esiste e quando usarla.

### Livello 2: Istruzioni (caricate quando attivate)

**Tipo di contenuto: Istruzioni**. Il corpo principale di SKILL.md contiene conoscenze procedurali: flussi di lavoro, best practice e guida:

````markdown
# PDF Processing

## Quick start

Use pdfplumber to extract text from PDFs:

```python
import pdfplumber

with pdfplumber.open("document.pdf") as pdf:
    text = pdf.pages[0].extract_text()
```

For advanced form filling, see [FORMS.md](FORMS.md).
````

Quando richiedi qualcosa che corrisponde alla descrizione di una Skill, Claude legge SKILL.md dal filesystem tramite bash. Solo allora questo contenuto entra nella finestra di contesto.

### Livello 3: Risorse e codice (caricati secondo le necessità)

**Tipi di contenuto: Istruzioni, codice e risorse**. Le Skills possono raggruppare materiali aggiuntivi:

```
pdf-skill/
├── SKILL.md (main instructions)
├── FORMS.md (form-filling guide)
├── REFERENCE.md (detailed API reference)
└── scripts/
    └── fill_form.py (utility script)
```

**Istruzioni**: File markdown aggiuntivi (FORMS.md, REFERENCE.md) contenenti guida specializzata e flussi di lavoro

**Codice**: Script eseguibili (fill_form.py, validate.py) che Claude esegue tramite bash; gli script forniscono operazioni deterministiche senza consumare contesto

**Risorse**: Materiali di riferimento come schemi di database, documentazione API, template o esempi

Claude accede a questi file solo quando referenziati. Il modello filesystem significa che ogni tipo di contenuto ha diversi punti di forza: istruzioni per guida flessibile, codice per affidabilità, risorse per ricerca fattuale.

| Livello | Quando caricato | Costo token | Contenuto |
|---|---|---|---|
| **Livello 1: Metadati** | Sempre (all'avvio) | ~100 token per Skill | `name` e `description` dal frontmatter YAML |
| **Livello 2: Istruzioni** | Quando Skill è attivata | Meno di 5k token | Corpo SKILL.md con istruzioni e guida |
| **Livello 3+: Risorse** | Secondo le necessità | Effettivamente illimitato | File raggruppati eseguiti tramite bash senza caricare i contenuti nel contesto |

La progressive disclosure assicura che solo il contenuto rilevante occupi la finestra di contesto in un dato momento.

### L'architettura delle Skills

Le Skills vengono eseguite in un ambiente di esecuzione del codice dove Claude ha accesso al filesystem, comandi bash e capacità di esecuzione del codice. Pensalo così: le Skills esistono come directory su una macchina virtuale, e Claude interagisce con loro usando gli stessi comandi bash che useresti per navigare i file sul tuo computer.

![Agent Skills Architecture - showing how Skills integrate with the agent's configuration and virtual machine](/docs/images/agent-skills-architecture.png)

**Come Claude accede al contenuto Skill:**

Quando una Skill viene attivata, Claude usa bash per leggere SKILL.md dal filesystem, portando le sue istruzioni nella finestra di contesto. Se quelle istruzioni referenziano altri file (come FORMS.md o uno schema di database), Claude legge anche quei file usando comandi bash aggiuntivi. Quando le istruzioni menzionano script eseguibili, Claude li esegue tramite bash e riceve solo l'output (il codice dello script non entra mai nel contesto).

**Cosa abilita questa architettura:**

**Accesso ai file su richiesta**: Claude legge solo i file necessari per ogni attività specifica. Una Skill può includere dozzine di file di riferimento, ma se il tuo compito ha bisogno solo dello schema di vendita, Claude carica solo quel file. Il resto rimane sul filesystem consumando zero token.

**Esecuzione efficiente dello script**: Quando Claude esegue `validate_form.py`, il codice dello script non viene mai caricato nella finestra di contesto. Solo l'output dello script (come "Validation passed" o messaggi di errore specifici) consuma token. Questo rende gli script molto più efficienti che avere Claude generi codice equivalente al volo.

**Nessun limite pratico al contenuto raggruppato**: Poiché i file non consumano contesto fino a quando non vengono accessati, le Skills possono includere documentazione API completa, grandi dataset, esempi estesi, o qualsiasi materiale di riferimento di cui hai bisogno. Non c'è penalità di contesto per il contenuto raggruppato che non viene utilizzato.

Questo modello basato su filesystem è ciò che rende possibile la progressive disclosure. Claude naviga la tua Skill come faresti riferimento a sezioni specifiche di una guida di onboarding, accedendo esattamente a ciò che ogni attività richiede.

### Esempio: Caricamento di una skill di elaborazione PDF

Ecco come Claude carica e usa una skill di elaborazione PDF:

1. **Avvio**: Il prompt di sistema include: `PDF Processing - Extract text and tables from PDF files, fill forms, merge documents`
2. **Richiesta dell'utente**: "Extract the text from this PDF and summarize it"
3. **Claude invoca**: `bash: read pdf-skill/SKILL.md` → Istruzioni caricate nel contesto
4. **Claude determina**: La compilazione di moduli non è necessaria, quindi FORMS.md non viene letto
5. **Claude esegue**: Usa le istruzioni da SKILL.md per completare l'attività

![Skills loading into context window - showing the progressive loading of skill metadata and content](/docs/images/agent-skills-context-window.png)

Il diagramma mostra:
1. Stato predefinito con prompt di sistema e metadati skill pre-caricati
2. Claude attiva la skill leggendo SKILL.md tramite bash
3. Claude legge facoltativamente file raggruppati aggiuntivi come FORMS.md secondo le necessità
4. Claude procede con l'attività

Questo caricamento dinamico assicura che solo il contenuto skill rilevante occupi la finestra di contesto.

## Dove funzionano le Skills

Le Skills sono disponibili su tutti i prodotti agent di Claude:

### Claude API

L'API Claude supporta sia Agent Skills pre-costruite che Skills personalizzate. Entrambe funzionano in modo identico: specifica il `skill_id` rilevante nel parametro `container` insieme allo strumento di esecuzione del codice.

**Prerequisiti**: L'utilizzo di Skills tramite l'API richiede tre intestazioni beta:
- `code-execution-2025-08-25` - Le Skills vengono eseguite nel contenitore di esecuzione del codice
- `skills-2025-10-02` - Abilita la funzionalità Skills
- `files-api-2025-04-14` - Richiesto per caricare/scaricare file verso/dal contenitore

Usa Agent Skills pre-costruite referenziando il loro `skill_id` (ad es. `pptx`, `xlsx`), o crea e carica le tue tramite l'API Skills (endpoint `/v1/skills`). Le Custom Skills sono condivise a livello di organizzazione.

Per saperne di più, consulta [Use Skills with the Claude API](/docs/it/build-with-claude/skills-guide).

### Claude Code

[Claude Code](https://code.claude.com/docs/overview) supporta solo Custom Skills.

**Custom Skills**: Crea Skills come directory con file SKILL.md. Claude le scopre e le utilizza automaticamente.

Le Custom Skills in Claude Code sono basate su filesystem e non richiedono caricamenti API.

Per saperne di più, consulta [Use Skills in Claude Code](https://code.claude.com/docs/skills).

### Claude Agent SDK

Il [Claude Agent SDK](/docs/it/agent-sdk/overview) supporta Custom Skills attraverso la configurazione basata su filesystem.

**Custom Skills**: Crea Skills come directory con file SKILL.md in `.claude/skills/`. Abilita le Skills includendo `"Skill"` nella tua configurazione `allowed_tools`.

Le Skills nell'Agent SDK vengono quindi scoperte automaticamente quando l'SDK viene eseguito.

Per saperne di più, consulta [Agent Skills in the SDK](/docs/it/agent-sdk/skills).

### Claude.ai

[Claude.ai](https://claude.ai) supporta sia Agent Skills pre-costruite che Custom Skills.

**Agent Skills pre-costruite**: Queste Skills stanno già funzionando dietro le quinte quando crei documenti. Claude le utilizza senza richiedere alcuna configurazione.

**Custom Skills**: Carica le tue Skills come file zip tramite Settings > Features. Disponibile su piani Pro, Max, Team ed Enterprise con esecuzione del codice abilitata. Le Custom Skills sono individuali per ogni utente; non sono condivise a livello di organizzazione e non possono essere gestite centralmente dagli amministratori.

Per saperne di più sull'utilizzo di Skills in Claude.ai, consulta le seguenti risorse nel Centro assistenza Claude:
- [What are Skills?](https://support.claude.com/en/articles/12512176-what-are-skills)
- [Using Skills in Claude](https://support.claude.com/en/articles/12512180-using-skills-in-claude)
- [How to create custom Skills](https://support.claude.com/en/articles/12512198-creating-custom-skills)
- [Tech Claude your way of working using Skills](https://support.claude.com/en/articles/12580051-teach-claude-your-way-of-working-using-skills)

## Struttura della Skill

Ogni Skill richiede un file `SKILL.md` con frontmatter YAML:

```yaml
---
name: your-skill-name
description: Brief description of what this Skill does and when to use it
---

# Your Skill Name

## Instructions
[Clear, step-by-step guidance for Claude to follow]

## Examples
[Concrete examples of using this Skill]
```

**Campi obbligatori**: `name` e `description`

**Requisiti dei campi**:

`name`:
- Massimo 64 caratteri
- Deve contenere solo lettere minuscole, numeri e trattini
- Non può contenere tag XML
- Non può contenere parole riservate: "anthropic", "claude"

`description`:
- Deve essere non vuota
- Massimo 1024 caratteri
- Non può contenere tag XML

La `description` dovrebbe includere sia cosa fa la Skill che quando Claude dovrebbe usarla. Per una guida completa all'authoring, consulta la [guida alle best practice](/docs/it/agents-and-tools/agent-skills/best-practices).

## Considerazioni sulla sicurezza

Ti consigliamo vivamente di usare Skills solo da fonti affidabili: quelle che hai creato tu stesso o ottenuto da Anthropic. Le Skills forniscono a Claude nuove capacità attraverso istruzioni e codice, e mentre questo le rende potenti, significa anche che una Skill malevola può dirigere Claude a invocare strumenti o eseguire codice in modi che non corrispondono allo scopo dichiarato della Skill.

<Warning>
Se devi usare una Skill da una fonte non affidabile o sconosciuta, esercita estrema cautela e controllala accuratamente prima dell'uso. A seconda di quale accesso ha Claude durante l'esecuzione della Skill, le Skill malevolole potrebbero portare a esfiltrazione di dati, accesso non autorizzato al sistema, o altri rischi di sicurezza.
</Warning>

**Considerazioni chiave sulla sicurezza**:
- **Controlla accuratamente**: Rivedi tutti i file raggruppati nella Skill: SKILL.md, script, immagini e altre risorse. Cerca modelli insoliti come chiamate di rete inaspettate, modelli di accesso ai file, o operazioni che non corrispondono allo scopo dichiarato della Skill
- **Le fonti esterne sono rischiose**: Le Skills che recuperano dati da URL esterni pongono un rischio particolare, poiché il contenuto recuperato potrebbe contenere istruzioni malevolole. Anche le Skills affidabili possono essere compromesse se le loro dipendenze esterne cambiano nel tempo
- **Abuso di strumenti**: Le Skill malevolole possono invocare strumenti (operazioni su file, comandi bash, esecuzione di codice) in modi dannosi
- **Esposizione di dati**: Le Skills con accesso a dati sensibili potrebbero essere progettate per perdere informazioni a sistemi esterni
- **Tratta come installazione di software**: Usa Skills solo da fonti affidabili. Sii particolarmente attento quando integri Skills in sistemi di produzione con accesso a dati sensibili o operazioni critiche

## Available Skills

### Agent Skills pre-costruite

Le seguenti Agent Skills pre-costruite sono disponibili per l'uso immediato:

- **PowerPoint (pptx)**: Crea presentazioni, modifica diapositive, analizza il contenuto della presentazione
- **Excel (xlsx)**: Crea fogli di calcolo, analizza dati, genera report con grafici
- **Word (docx)**: Crea documenti, modifica contenuto, formatta testo
- **PDF (pdf)**: Genera documenti PDF formattati e report

Queste Skills sono disponibili sull'API Claude e su claude.ai. Consulta il [tutorial di avvio rapido](/docs/it/agents-and-tools/agent-skills/quickstart) per iniziare a usarle nell'API.

### Esempi di Custom Skills

Per esempi completi di Custom Skills, consulta il [Skills cookbook](https://github.com/anthropics/claude-cookbooks/tree/main/skills).

## Limitazioni e vincoli

Comprendere questi limiti ti aiuta a pianificare efficacemente il tuo deployment di Skills.

### Disponibilità cross-surface

**Le Custom Skills non si sincronizzano tra le superfici**. Le Skills caricate su una superficie non sono automaticamente disponibili su altre:

- Le Skills caricate su Claude.ai devono essere caricate separatamente sull'API
- Le Skills caricate tramite l'API non sono disponibili su Claude.ai
- Le Skills di Claude Code sono basate su filesystem e separate sia da Claude.ai che dall'API

Dovrai gestire e caricare le Skills separatamente per ogni superficie dove vuoi usarle.

### Ambito di condivisione

Le Skills hanno diversi modelli di condivisione a seconda di dove le usi:
- **Claude.ai**: Solo utente individuale; ogni membro del team deve caricare separatamente
- **Claude API**: A livello di workspace; tutti i membri del workspace possono accedere alle Skills caricate
- **Claude Code**: Personale (`~/.claude/skills/`) o basato su progetto (`.claude/skills/`)

Claude.ai attualmente non supporta la gestione centralizzata dell'amministratore o la distribuzione a livello di organizzazione delle Custom Skills.

### Vincoli dell'ambiente di runtime

Le Skills vengono eseguite nel contenitore di esecuzione del codice con questi limiti:

- **Nessun accesso di rete**: Le Skills non possono effettuare chiamate API esterne o accedere a Internet
- **Nessuna installazione di pacchetti di runtime**: Solo i pacchetti pre-installati sono disponibili. Non puoi installare nuovi pacchetti durante l'esecuzione.
- **Solo dipendenze pre-configurate**: Consulta la [documentazione dello strumento di esecuzione del codice](/docs/it/agents-and-tools/tool-use/code-execution-tool) per l'elenco dei pacchetti disponibili

Pianifica le tue Skills per funzionare all'interno di questi vincoli.

## Passaggi successivi

<CardGroup cols={2}>
  <Card
    title="Inizia con Agent Skills"
    icon="graduation-cap"
    href="/docs/it/agents-and-tools/agent-skills/quickstart"
  >
    Crea la tua prima Skill
  </Card>
  <Card
    title="Guida API"
    icon="code"
    href="/docs/it/build-with-claude/skills-guide"
  >
    Usa Skills con l'API Claude
  </Card>
  <Card
    title="Usa Skills in Claude Code"
    icon="terminal"
    href="https://code.claude.com/docs/skills"
  >
    Crea e gestisci Custom Skills in Claude Code
  </Card>
  <Card
    title="Usa Skills in Agent SDK"
    icon="cube"
    href="/docs/it/agent-sdk/skills"
  >
    Usa Skills in modo programmatico in TypeScript e Python
  </Card>
  <Card
    title="Best practice di authoring"
    icon="lightbulb"
    href="/docs/it/agents-and-tools/agent-skills/best-practices"
  >
    Scrivi Skills che Claude può usare efficacemente
  </Card>
</CardGroup>