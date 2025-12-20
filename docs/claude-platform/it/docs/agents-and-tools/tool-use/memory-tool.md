# Strumento di memoria

Lo strumento di memoria consente a Claude di archiviare e recuperare informazioni tra conversazioni attraverso una directory di file di memoria.

---

Lo strumento di memoria consente a Claude di archiviare e recuperare informazioni tra conversazioni attraverso una directory di file di memoria. Claude può creare, leggere, aggiornare ed eliminare file che persistono tra le sessioni, permettendogli di costruire conoscenza nel tempo senza mantenere tutto nella finestra di contesto.

Lo strumento di memoria funziona lato client: tu controlli dove e come i dati vengono archiviati attraverso la tua infrastruttura.

<Note>
Lo strumento di memoria è attualmente in beta. Per abilitarlo, utilizza l'intestazione beta `context-management-2025-06-27` nelle tue richieste API.

Ti invitiamo a contattarci tramite il nostro [modulo di feedback](https://forms.gle/YXC2EKGMhjN1c4L88) per condividere il tuo feedback su questa funzione.
</Note>

## Casi d'uso

- Mantenere il contesto del progetto tra più esecuzioni di agenti
- Imparare dalle interazioni passate, dalle decisioni e dal feedback
- Costruire basi di conoscenza nel tempo
- Abilitare l'apprendimento tra conversazioni dove Claude migliora nei flussi di lavoro ricorrenti

## Come funziona

Quando abilitato, Claude controlla automaticamente la sua directory di memoria prima di iniziare i compiti. Claude può creare, leggere, aggiornare ed eliminare file nella directory `/memories` per archiviare ciò che impara mentre lavora, quindi fare riferimento a quei ricordi nelle conversazioni future per gestire compiti simili più efficacemente o riprendere da dove si era fermato.

Poiché questo è uno strumento lato client, Claude effettua chiamate di strumento per eseguire operazioni di memoria, e la tua applicazione esegue quelle operazioni localmente. Questo ti dà il controllo completo su dove e come la memoria viene archiviata. Per la sicurezza, dovresti limitare tutte le operazioni di memoria alla directory `/memories`.

### Esempio: Come funzionano le chiamate dello strumento di memoria

Quando chiedi a Claude di aiutarti con un compito, Claude controlla automaticamente la sua directory di memoria per primo. Ecco come appare una tipica interazione:

**1. Richiesta dell'utente:**
```
"Aiutami a rispondere a questo ticket di servizio clienti."
```

**2. Claude controlla la directory di memoria:**
```
"Ti aiuterò a rispondere al ticket di servizio clienti. Lasciami controllare la mia memoria per qualsiasi contesto precedente."
```

Claude chiama lo strumento di memoria:
```json
{
  "type": "tool_use",
  "id": "toolu_01C4D5E6F7G8H9I0J1K2L3M4",
  "name": "memory",
  "input": {
    "command": "view",
    "path": "/memories"
  }
}
```

**3. La tua applicazione restituisce il contenuto della directory:**
```json
{
  "type": "tool_result",
  "tool_use_id": "toolu_01C4D5E6F7G8H9I0J1K2L3M4",
  "content": "Ecco i file e le directory fino a 2 livelli di profondità in /memories, escludendo elementi nascosti e node_modules:\n4.0K\t/memories\n1.5K\t/memories/customer_service_guidelines.xml\n2.0K\t/memories/refund_policies.xml"
}
```

**4. Claude legge i file rilevanti:**
```json
{
  "type": "tool_use",
  "id": "toolu_01D5E6F7G8H9I0J1K2L3M4N5",
  "name": "memory",
  "input": {
    "command": "view",
    "path": "/memories/customer_service_guidelines.xml"
  }
}
```

**5. La tua applicazione restituisce il contenuto del file:**
```json
{
  "type": "tool_result",
  "tool_use_id": "toolu_01D5E6F7G8H9I0J1K2L3M4N5",
  "content": "Ecco il contenuto di /memories/customer_service_guidelines.xml con numeri di riga:\n     1\t<guidelines>\n     2\t<addressing_customers>\n     3\t- Rivolgersi sempre ai clienti per nome\n     4\t- Utilizzare un linguaggio empatico\n..."
}
```

**6. Claude utilizza la memoria per aiutare:**
```
"In base alle tue linee guida per il servizio clienti, posso aiutarti a formulare una risposta. Per favore, condividi i dettagli del ticket..."
```

## Modelli supportati

Lo strumento di memoria è disponibile su:

- Claude Sonnet 4.5 (`claude-sonnet-4-5-20250929`)
- Claude Sonnet 4 (`claude-sonnet-4-20250514`)
- Claude Haiku 4.5 (`claude-haiku-4-5-20251001`)
- Claude Opus 4.5 (`claude-opus-4-5-20251101`)
- Claude Opus 4.1 (`claude-opus-4-1-20250805`)
- Claude Opus 4 (`claude-opus-4-20250514`)

## Iniziare

Per utilizzare lo strumento di memoria:

1. Includi l'intestazione beta `context-management-2025-06-27` nelle tue richieste API
2. Aggiungi lo strumento di memoria alla tua richiesta
3. Implementa gestori lato client per le operazioni di memoria

<Note>
Per gestire le operazioni dello strumento di memoria nella tua applicazione, devi implementare gestori per ogni comando di memoria. I nostri SDK forniscono helper dello strumento di memoria che gestiscono l'interfaccia dello strumento: puoi sottoclassare `BetaAbstractMemoryTool` (Python) o utilizzare `betaMemoryTool` (TypeScript) per implementare il tuo backend di memoria (basato su file, database, archiviazione cloud, file crittografati, ecc.).

Per esempi funzionanti, vedi:
- Python: [examples/memory/basic.py](https://github.com/anthropics/anthropic-sdk-python/blob/main/examples/memory/basic.py)
- TypeScript: [examples/tools-helpers-memory.ts](https://github.com/anthropics/anthropic-sdk-typescript/blob/main/examples/tools-helpers-memory.ts)
</Note>

## Utilizzo di base

<CodeGroup>

```bash cURL
curl https://api.anthropic.com/v1/messages \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "content-type: application/json" \
    --header "anthropic-beta: context-management-2025-06-27" \
    --data '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 2048,
        "messages": [
            {
                "role": "user",
                "content": "I'\''m working on a Python web scraper that keeps crashing with a timeout error. Here'\''s the problematic function:\n\n```python\ndef fetch_page(url, retries=3):\n    for i in range(retries):\n        try:\n            response = requests.get(url, timeout=5)\n            return response.text\n        except requests.exceptions.Timeout:\n            if i == retries - 1:\n                raise\n            time.sleep(1)\n```\n\nPlease help me debug this."
            }
        ],
        "tools": [{
            "type": "memory_20250818",
            "name": "memory"
        }]
    }'
```

```python Python
import anthropic

client = anthropic.Anthropic()

message = client.beta.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=2048,
    messages=[
        {
            "role": "user",
            "content": "I'm working on a Python web scraper that keeps crashing with a timeout error. Here's the problematic function:\n\n```python\ndef fetch_page(url, retries=3):\n    for i in range(retries):\n        try:\n            response = requests.get(url, timeout=5)\n            return response.text\n        except requests.exceptions.Timeout:\n            if i == retries - 1:\n                raise\n            time.sleep(1)\n```\n\nPlease help me debug this."
        }
    ],
    tools=[{
        "type": "memory_20250818",
        "name": "memory"
    }],
    betas=["context-management-2025-06-27"]
)
```

```typescript TypeScript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY,
});

const message = await anthropic.beta.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 2048,
  messages: [
    {
      role: "user",
      content: "I'm working on a Python web scraper that keeps crashing with a timeout error. Here's the problematic function:\n\n```python\ndef fetch_page(url, retries=3):\n    for i in range(retries):\n        try:\n            response = requests.get(url, timeout=5)\n            return response.text\n        except requests.exceptions.Timeout:\n            if i == retries - 1:\n                raise\n            time.sleep(1)\n```\n\nPlease help me debug this."
    }
  ],
  tools: [{
    type: "memory_20250818",
    name: "memory"
  }],
  betas: ["context-management-2025-06-27"]
});
```

</CodeGroup>

## Comandi dello strumento

La tua implementazione lato client deve gestire questi comandi dello strumento di memoria. Mentre queste specifiche descrivono i comportamenti consigliati con cui Claude ha più familiarità, puoi modificare la tua implementazione e restituire stringhe come necessario per il tuo caso d'uso.

### view
Mostra il contenuto della directory o il contenuto del file con intervalli di riga opzionali:

```json
{
  "command": "view",
  "path": "/memories",
  "view_range": [1, 10]  // Opzionale: visualizza righe specifiche
}
```

#### Valori di ritorno

**Per le directory:** Restituisci un elenco che mostra file e directory con le loro dimensioni:
```
Ecco i file e le directory fino a 2 livelli di profondità in {path}, escludendo elementi nascosti e node_modules:
{size}    {path}
{size}    {path}/{filename1}
{size}    {path}/{filename2}
```

- Elenca file fino a 2 livelli di profondità
- Mostra dimensioni leggibili dall'uomo (ad es. `5.5K`, `1.2M`)
- Esclude elementi nascosti (file che iniziano con `.`) e `node_modules`
- Utilizza il carattere di tabulazione tra dimensione e percorso

**Per i file:** Restituisci il contenuto del file con un'intestazione e numeri di riga:
```
Ecco il contenuto di {path} con numeri di riga:
{line_numbers}{tab}{content}
```

Formattazione del numero di riga:
- **Larghezza**: 6 caratteri, allineati a destra con riempimento di spazi
- **Separatore**: Carattere di tabulazione tra numero di riga e contenuto
- **Indicizzazione**: 1-indicizzato (la prima riga è la riga 1)
- **Limite di riga**: I file con più di 999.999 righe dovrebbero restituire un errore: `"File {path} exceeds maximum line limit of 999,999 lines."`

**Esempio di output:**
```
Ecco il contenuto di /memories/notes.txt con numeri di riga:
     1	Hello World
     2	This is line two
    10	Line ten
   100	Line one hundred
```

#### Gestione degli errori

- **File/directory non esiste**: `"The path {path} does not exist. Please provide a valid path."`

### create
Crea un nuovo file:

```json
{
  "command": "create",
  "path": "/memories/notes.txt",
  "file_text": "Meeting notes:\n- Discussed project timeline\n- Next steps defined\n"
}
```

#### Valori di ritorno

- **Successo**: `"File created successfully at: {path}"`

#### Gestione degli errori

- **Il file esiste già**: `"Error: File {path} already exists"`

### str_replace
Sostituisci il testo in un file:

```json
{
  "command": "str_replace",
  "path": "/memories/preferences.txt",
  "old_str": "Favorite color: blue",
  "new_str": "Favorite color: green"
}
```

#### Valori di ritorno

- **Successo**: `"The memory file has been edited."` seguito da uno snippet del file modificato con numeri di riga

#### Gestione degli errori

- **Il file non esiste**: `"Error: The path {path} does not exist. Please provide a valid path."`
- **Testo non trovato**: ``"No replacement was performed, old_str `{old_str}` did not appear verbatim in {path}."``
- **Testo duplicato**: Quando `old_str` appare più volte, restituisci: ``"No replacement was performed. Multiple occurrences of old_str `{old_str}` in lines: {line_numbers}. Please ensure it is unique"``

#### Gestione della directory

Se il percorso è una directory, restituisci un errore "file non esiste".

### insert
Inserisci il testo in una riga specifica:

```json
{
  "command": "insert",
  "path": "/memories/todo.txt",
  "insert_line": 2,
  "insert_text": "- Review memory tool documentation\n"
}
```

#### Valori di ritorno

- **Successo**: `"The file {path} has been edited."`

#### Gestione degli errori

- **Il file non esiste**: `"Error: The path {path} does not exist"`
- **Numero di riga non valido**: ``"Error: Invalid `insert_line` parameter: {insert_line}. It should be within the range of lines of the file: [0, {n_lines}]"``

#### Gestione della directory

Se il percorso è una directory, restituisci un errore "file non esiste".

### delete
Elimina un file o una directory:

```json
{
  "command": "delete",
  "path": "/memories/old_file.txt"
}
```

#### Valori di ritorno

- **Successo**: `"Successfully deleted {path}"`

#### Gestione degli errori

- **File/directory non esiste**: `"Error: The path {path} does not exist"`

#### Gestione della directory

Elimina la directory e tutto il suo contenuto ricorsivamente.

### rename
Rinomina o sposta un file/directory:

```json
{
  "command": "rename",
  "old_path": "/memories/draft.txt",
  "new_path": "/memories/final.txt"
}
```

#### Valori di ritorno

- **Successo**: `"Successfully renamed {old_path} to {new_path}"`

#### Gestione degli errori

- **L'origine non esiste**: `"Error: The path {old_path} does not exist"`
- **La destinazione esiste già**: Restituisci un errore (non sovrascrivere): `"Error: The destination {new_path} already exists"`

#### Gestione della directory

Rinomina la directory.

## Guida al prompt

Includiamo automaticamente questa istruzione nel prompt di sistema quando lo strumento di memoria è incluso:

```
IMPORTANT: ALWAYS VIEW YOUR MEMORY DIRECTORY BEFORE DOING ANYTHING ELSE.
MEMORY PROTOCOL:
1. Use the `view` command of your `memory` tool to check for earlier progress.
2. ... (work on the task) ...
     - As you make progress, record status / progress / thoughts etc in your memory.
ASSUME INTERRUPTION: Your context window might be reset at any moment, so you risk losing any progress that is not recorded in your memory directory.
```

Se osservi che Claude crea file di memoria disordinati, puoi includere questa istruzione:

> Nota: quando modifichi la tua cartella di memoria, cerca sempre di mantenere il suo contenuto aggiornato, coerente e organizzato. Puoi rinominare o eliminare file che non sono più rilevanti. Non creare nuovi file se non necessario.

Puoi anche guidare ciò che Claude scrive in memoria, ad es. "Scrivi solo informazioni rilevanti a \<topic\> nel tuo sistema di memoria."

## Considerazioni sulla sicurezza

Ecco importanti considerazioni sulla sicurezza quando implementi il tuo archivio di memoria:

### Informazioni sensibili
Claude di solito rifiuta di scrivere informazioni sensibili nei file di memoria. Tuttavia, potresti voler implementare una convalida più rigorosa che rimuova potenziali informazioni sensibili.

### Dimensione dell'archiviazione dei file
Considera il tracciamento delle dimensioni dei file di memoria e la prevenzione della crescita eccessiva dei file. Considera l'aggiunta di un numero massimo di caratteri che il comando di lettura della memoria può restituire, e lascia che Claude pagini attraverso i contenuti.

### Scadenza della memoria
Considera la cancellazione periodica dei file di memoria che non sono stati accessibili per un tempo prolungato.

### Protezione dall'attraversamento del percorso

<Warning>
Gli input di percorso dannosi potrebbero tentare di accedere a file al di fuori della directory `/memories`. La tua implementazione **DEVE** convalidare tutti i percorsi per prevenire attacchi di attraversamento di directory.
</Warning>

Considera queste misure di sicurezza:

- Convalida che tutti i percorsi inizino con `/memories`
- Risolvi i percorsi nella loro forma canonica e verifica che rimangano all'interno della directory di memoria
- Rifiuta i percorsi contenenti sequenze come `../`, `..\\`, o altri modelli di attraversamento
- Stai attento alle sequenze di attraversamento codificate in URL (`%2e%2e%2f`)
- Utilizza le utility di sicurezza del percorso integrate nel tuo linguaggio (ad es. `pathlib.Path.resolve()` e `relative_to()` di Python)

## Gestione degli errori

Lo strumento di memoria utilizza modelli di gestione degli errori simili a quelli dello [strumento editor di testo](/docs/it/agents-and-tools/tool-use/text-editor-tool#handle-errors). Vedi le sezioni dei singoli comandi dello strumento sopra per messaggi di errore dettagliati e comportamenti. Gli errori comuni includono file non trovato, errori di permesso, percorsi non validi e corrispondenze di testo duplicate.

## Utilizzo con Context Editing

Lo strumento di memoria può essere combinato con [context editing](/docs/it/build-with-claude/context-editing), che cancella automaticamente i vecchi risultati degli strumenti quando il contesto della conversazione cresce oltre una soglia configurata. Questa combinazione abilita flussi di lavoro agentivi di lunga durata che altrimenti supererebbero i limiti di contesto.

### Come funzionano insieme

Quando context editing è abilitato e la tua conversazione si avvicina alla soglia di cancellazione, Claude riceve automaticamente una notifica di avviso. Questo spinge Claude a preservare qualsiasi informazione importante dai risultati degli strumenti nei file di memoria prima che quei risultati vengano cancellati dalla finestra di contesto.

Dopo che i risultati degli strumenti vengono cancellati, Claude può recuperare le informazioni archiviate dai file di memoria ogni volta che necessario, trattando effettivamente la memoria come un'estensione del suo contesto di lavoro. Questo consente a Claude di:

- Continuare flussi di lavoro complessi e multi-step senza perdere informazioni critiche
- Fare riferimento al lavoro passato e alle decisioni anche dopo che i risultati degli strumenti sono stati rimossi
- Mantenere un contesto coerente tra conversazioni che supererebbero i limiti di contesto tipici
- Costruire una base di conoscenza nel tempo mantenendo la finestra di contesto attiva gestibile

### Esempio di flusso di lavoro

Considera un progetto di refactoring del codice con molte operazioni su file:

1. Claude effettua numerose modifiche ai file, generando molti risultati degli strumenti
2. Man mano che il contesto cresce e si avvicina alla tua soglia, Claude riceve un avviso
3. Claude riassume le modifiche apportate finora in un file di memoria (ad es. `/memories/refactoring_progress.xml`)
4. Context editing cancella automaticamente i risultati degli strumenti più vecchi
5. Claude continua a lavorare, facendo riferimento al file di memoria quando ha bisogno di ricordare quali modifiche erano già state completate
6. Il flusso di lavoro può continuare indefinitamente, con Claude che gestisce sia il contesto attivo che la memoria persistente

### Configurazione

Per utilizzare entrambe le funzioni insieme:

<CodeGroup>

```python Python
response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=4096,
    messages=[...],
    tools=[
        {
            "type": "memory_20250818",
            "name": "memory"
        },
        # Your other tools
    ],
    betas=["context-management-2025-06-27"],
    context_management={
        "edits": [
            {
                "type": "clear_tool_uses_20250919",
                "trigger": {
                    "type": "input_tokens",
                    "value": 100000
                },
                "keep": {
                    "type": "tool_uses",
                    "value": 3
                }
            }
        ]
    }
)
```

```typescript TypeScript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY,
});

const response = await anthropic.beta.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 4096,
  messages: [...],
  tools: [
    {
      type: "memory_20250818",
      name: "memory"
    },
    // Your other tools
  ],
  betas: ["context-management-2025-06-27"],
  context_management: {
    edits: [
      {
        type: "clear_tool_uses_20250919",
        trigger: {
          type: "input_tokens",
          value: 100000
        },
        keep: {
          type: "tool_uses",
          value: 3
        }
      }
    ]
  }
});
```

</CodeGroup>

Puoi anche escludere le chiamate dello strumento di memoria dall'essere cancellate per assicurare che Claude abbia sempre accesso alle operazioni di memoria recenti:

<CodeGroup>

```python Python
context_management={
    "edits": [
        {
            "type": "clear_tool_uses_20250919",
            "exclude_tools": ["memory"]
        }
    ]
}
```

```typescript TypeScript
context_management: {
  edits: [
    {
      type: "clear_tool_uses_20250919",
      exclude_tools: ["memory"]
    }
  ]
}
```

</CodeGroup>