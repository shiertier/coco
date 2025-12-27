# Strumento di esecuzione del codice

Claude può analizzare dati, creare visualizzazioni, eseguire calcoli complessi, eseguire comandi di sistema, creare e modificare file ed elaborare file caricati direttamente all'interno della conversazione API.

---

Claude può analizzare dati, creare visualizzazioni, eseguire calcoli complessi, eseguire comandi di sistema, creare e modificare file ed elaborare file caricati direttamente all'interno della conversazione API.
Lo strumento di esecuzione del codice consente a Claude di eseguire comandi Bash e manipolare file, inclusa la scrittura di codice, in un ambiente sandbox sicuro.

<Note>
Lo strumento di esecuzione del codice è attualmente in beta pubblica.

Per utilizzare questa funzione, aggiungi l'[intestazione beta](/docs/it/api/beta-headers) `"code-execution-2025-08-25"` alle tue richieste API.
</Note>

## Compatibilità del modello

Lo strumento di esecuzione del codice è disponibile sui seguenti modelli:

| Modello | Versione dello strumento |
|-------|--------------|
| Claude Opus 4.5 (`claude-opus-4-5-20251101`) | `code_execution_20250825` |
| Claude Opus 4.1 (`claude-opus-4-1-20250805`) | `code_execution_20250825` |
| Claude Opus 4 (`claude-opus-4-20250514`) | `code_execution_20250825` |
| Claude Sonnet 4.5 (`claude-sonnet-4-5-20250929`) | `code_execution_20250825` |
| Claude Sonnet 4 (`claude-sonnet-4-20250514`) | `code_execution_20250825` |
| Claude Sonnet 3.7 (`claude-3-7-sonnet-20250219`) ([deprecato](/docs/it/about-claude/model-deprecations)) | `code_execution_20250825` |
| Claude Haiku 4.5 (`claude-haiku-4-5-20251001`) | `code_execution_20250825` |
| Claude Haiku 3.5 (`claude-3-5-haiku-latest`) ([deprecato](/docs/it/about-claude/model-deprecations)) | `code_execution_20250825` |

<Note>
La versione attuale `code_execution_20250825` supporta comandi Bash e operazioni su file. È disponibile anche una versione legacy `code_execution_20250522` (solo Python). Vedi [Aggiorna alla versione più recente dello strumento](#upgrade-to-latest-tool-version) per i dettagli della migrazione.
</Note>

<Warning>
Le versioni precedenti dello strumento non sono garantite come compatibili all'indietro con i modelli più recenti. Utilizza sempre la versione dello strumento che corrisponde alla versione del tuo modello.
</Warning>

## Avvio rapido

Ecco un semplice esempio che chiede a Claude di eseguire un calcolo:

<CodeGroup>
```bash Shell
curl https://api.anthropic.com/v1/messages \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "anthropic-beta: code-execution-2025-08-25" \
    --header "content-type: application/json" \
    --data '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 4096,
        "messages": [
            {
                "role": "user",
                "content": "Calculate the mean and standard deviation of [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"
            }
        ],
        "tools": [{
            "type": "code_execution_20250825",
            "name": "code_execution"
        }]
    }'
```

```python Python
import anthropic

client = anthropic.Anthropic()

response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    betas=["code-execution-2025-08-25"],
    max_tokens=4096,
    messages=[{
        "role": "user",
        "content": "Calculate the mean and standard deviation of [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"
    }],
    tools=[{
        "type": "code_execution_20250825",
        "name": "code_execution"
    }]
)

print(response)
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

async function main() {
  const response = await anthropic.beta.messages.create({
    model: "claude-sonnet-4-5",
    betas: ["code-execution-2025-08-25"],
    max_tokens: 4096,
    messages: [
      {
        role: "user",
        content: "Calculate the mean and standard deviation of [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"
      }
    ],
    tools: [{
      type: "code_execution_20250825",
      name: "code_execution"
    }]
  });

  console.log(response);
}

main().catch(console.error);
```
</CodeGroup>

## Come funziona l'esecuzione del codice

Quando aggiungi lo strumento di esecuzione del codice alla tua richiesta API:

1. Claude valuta se l'esecuzione del codice aiuterebbe a rispondere alla tua domanda
2. Lo strumento fornisce automaticamente a Claude le seguenti capacità:
   - **Comandi Bash**: Esegui comandi shell per operazioni di sistema e gestione dei pacchetti
   - **Operazioni su file**: Crea, visualizza e modifica file direttamente, inclusa la scrittura di codice
3. Claude può utilizzare qualsiasi combinazione di queste capacità in una singola richiesta
4. Tutte le operazioni vengono eseguite in un ambiente sandbox sicuro
5. Claude fornisce risultati con eventuali grafici generati, calcoli o analisi

## Come utilizzare lo strumento

### Esegui comandi Bash

Chiedi a Claude di controllare le informazioni di sistema e installare pacchetti:

<CodeGroup>
```bash Shell
curl https://api.anthropic.com/v1/messages \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "anthropic-beta: code-execution-2025-08-25" \
    --header "content-type: application/json" \
    --data '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 4096,
        "messages": [{
            "role": "user",
            "content": "Check the Python version and list installed packages"
        }],
        "tools": [{
            "type": "code_execution_20250825",
            "name": "code_execution"
        }]
    }'
```

```python Python
response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    betas=["code-execution-2025-08-25"],
    max_tokens=4096,
    messages=[{
        "role": "user",
        "content": "Check the Python version and list installed packages"
    }],
    tools=[{
        "type": "code_execution_20250825",
        "name": "code_execution"
    }]
)
```

```typescript TypeScript
const response = await anthropic.beta.messages.create({
  model: "claude-sonnet-4-5",
  betas: ["code-execution-2025-08-25"],
  max_tokens: 4096,
  messages: [{
    role: "user",
    content: "Check the Python version and list installed packages"
  }],
  tools: [{
    type: "code_execution_20250825",
    name: "code_execution"
  }]
});
```
</CodeGroup>

### Crea e modifica file direttamente

Claude può creare, visualizzare e modificare file direttamente nella sandbox utilizzando le capacità di manipolazione dei file:

<CodeGroup>
```bash Shell
curl https://api.anthropic.com/v1/messages \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "anthropic-beta: code-execution-2025-08-25" \
    --header "content-type: application/json" \
    --data '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 4096,
        "messages": [{
            "role": "user",
            "content": "Create a config.yaml file with database settings, then update the port from 5432 to 3306"
        }],
        "tools": [{
            "type": "code_execution_20250825",
            "name": "code_execution"
        }]
    }'
```

```python Python
response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    betas=["code-execution-2025-08-25"],
    max_tokens=4096,
    messages=[{
        "role": "user",
        "content": "Create a config.yaml file with database settings, then update the port from 5432 to 3306"
    }],
    tools=[{
        "type": "code_execution_20250825",
        "name": "code_execution"
    }]
)
```

```typescript TypeScript
const response = await anthropic.beta.messages.create({
  model: "claude-sonnet-4-5",
  betas: ["code-execution-2025-08-25"],
  max_tokens: 4096,
  messages: [{
    role: "user",
    content: "Create a config.yaml file with database settings, then update the port from 5432 to 3306"
  }],
  tools: [{
    type: "code_execution_20250825",
    name: "code_execution"
  }]
});
```
</CodeGroup>

### Carica e analizza i tuoi file

Per analizzare i tuoi file di dati (CSV, Excel, immagini, ecc.), caricali tramite l'API Files e fai riferimento ad essi nella tua richiesta:

<Note>
L'utilizzo dell'API Files con Code Execution richiede due intestazioni beta: `"anthropic-beta": "code-execution-2025-08-25,files-api-2025-04-14"`
</Note>

L'ambiente Python può elaborare vari tipi di file caricati tramite l'API Files, inclusi:

- CSV
- Excel (.xlsx, .xls)
- JSON
- XML
- Immagini (JPEG, PNG, GIF, WebP)
- File di testo (.txt, .md, .py, ecc)

#### Carica e analizza file

1. **Carica il tuo file** utilizzando l'[API Files](/docs/it/build-with-claude/files)
2. **Fai riferimento al file** nel tuo messaggio utilizzando un blocco di contenuto `container_upload`
3. **Includi lo strumento di esecuzione del codice** nella tua richiesta API

<CodeGroup>
```bash Shell
# First, upload a file
curl https://api.anthropic.com/v1/files \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "anthropic-beta: files-api-2025-04-14" \
    --form 'file=@"data.csv"' \

# Then use the file_id with code execution
curl https://api.anthropic.com/v1/messages \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "anthropic-beta: code-execution-2025-08-25,files-api-2025-04-14" \
    --header "content-type: application/json" \
    --data '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 4096,
        "messages": [{
            "role": "user",
            "content": [
                {"type": "text", "text": "Analyze this CSV data"},
                {"type": "container_upload", "file_id": "file_abc123"}
            ]
        }],
        "tools": [{
            "type": "code_execution_20250825",
            "name": "code_execution"
        }]
    }'
```

```python Python
import anthropic

client = anthropic.Anthropic()

# Upload a file
file_object = client.beta.files.upload(
    file=open("data.csv", "rb"),
)

# Use the file_id with code execution
response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    betas=["code-execution-2025-08-25", "files-api-2025-04-14"],
    max_tokens=4096,
    messages=[{
        "role": "user",
        "content": [
            {"type": "text", "text": "Analyze this CSV data"},
            {"type": "container_upload", "file_id": file_object.id}
        ]
    }],
    tools=[{
        "type": "code_execution_20250825",
        "name": "code_execution"
    }]
)
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';
import { createReadStream } from 'fs';

const anthropic = new Anthropic();

async function main() {
  // Upload a file
  const fileObject = await anthropic.beta.files.create({
    file: createReadStream("data.csv"),
  });

  // Use the file_id with code execution
  const response = await anthropic.beta.messages.create({
    model: "claude-sonnet-4-5",
    betas: ["code-execution-2025-08-25", "files-api-2025-04-14"],
    max_tokens: 4096,
    messages: [{
      role: "user",
      content: [
        { type: "text", text: "Analyze this CSV data" },
        { type: "container_upload", file_id: fileObject.id }
      ]
    }],
    tools: [{
      type: "code_execution_20250825",
      name: "code_execution"
    }]
  });

  console.log(response);
}

main().catch(console.error);
```
</CodeGroup>

#### Recupera file generati

Quando Claude crea file durante l'esecuzione del codice, puoi recuperare questi file utilizzando l'API Files:

<CodeGroup>
```python Python
from anthropic import Anthropic

# Initialize the client
client = Anthropic()

# Request code execution that creates files
response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    betas=["code-execution-2025-08-25", "files-api-2025-04-14"],
    max_tokens=4096,
    messages=[{
        "role": "user",
        "content": "Create a matplotlib visualization and save it as output.png"
    }],
    tools=[{
        "type": "code_execution_20250825",
        "name": "code_execution"
    }]
)

# Extract file IDs from the response
def extract_file_ids(response):
    file_ids = []
    for item in response.content:
        if item.type == 'bash_code_execution_tool_result':
            content_item = item.content
            if content_item.type == 'bash_code_execution_result':
                for file in content_item.content:
                    if hasattr(file, 'file_id'):
                        file_ids.append(file.file_id)
    return file_ids

# Download the created files
for file_id in extract_file_ids(response):
    file_metadata = client.beta.files.retrieve_metadata(file_id)
    file_content = client.beta.files.download(file_id)
    file_content.write_to_file(file_metadata.filename)
    print(f"Downloaded: {file_metadata.filename}")
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';
import { writeFileSync } from 'fs';

// Initialize the client
const anthropic = new Anthropic();

async function main() {
  // Request code execution that creates files
  const response = await anthropic.beta.messages.create({
    model: "claude-sonnet-4-5",
    betas: ["code-execution-2025-08-25", "files-api-2025-04-14"],
    max_tokens: 4096,
    messages: [{
      role: "user",
      content: "Create a matplotlib visualization and save it as output.png"
    }],
    tools: [{
      type: "code_execution_20250825",
      name: "code_execution"
    }]
  });

  // Extract file IDs from the response
  function extractFileIds(response: any): string[] {
    const fileIds: string[] = [];
    for (const item of response.content) {
      if (item.type === 'bash_code_execution_tool_result') {
        const contentItem = item.content;
        if (contentItem.type === 'bash_code_execution_result' && contentItem.content) {
          for (const file of contentItem.content) {
            fileIds.push(file.file_id);
          }
        }
      }
    }
    return fileIds;
  }

  // Download the created files
  const fileIds = extractFileIds(response);
  for (const fileId of fileIds) {
    const fileMetadata = await anthropic.beta.files.retrieveMetadata(fileId);
    const fileContent = await anthropic.beta.files.download(fileId);

    // Convert ReadableStream to Buffer and save
    const chunks: Uint8Array[] = [];
    for await (const chunk of fileContent) {
      chunks.push(chunk);
    }
    const buffer = Buffer.concat(chunks);
    writeFileSync(fileMetadata.filename, buffer);
    console.log(`Downloaded: ${fileMetadata.filename}`);
  }
}

main().catch(console.error);
```
</CodeGroup>

### Combina operazioni

Un flusso di lavoro complesso utilizzando tutte le capacità:

<CodeGroup>
```bash Shell
# First, upload a file
curl https://api.anthropic.com/v1/files \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "anthropic-beta: files-api-2025-04-14" \
    --form 'file=@"data.csv"' \
    > file_response.json

# Extract file_id (using jq)
FILE_ID=$(jq -r '.id' file_response.json)

# Then use it with code execution
curl https://api.anthropic.com/v1/messages \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "anthropic-beta: code-execution-2025-08-25,files-api-2025-04-14" \
    --header "content-type: application/json" \
    --data '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 4096,
        "messages": [{
            "role": "user",
            "content": [
                {
                    "type": "text", 
                    "text": "Analyze this CSV data: create a summary report, save visualizations, and create a README with the findings"
                },
                {
                    "type": "container_upload", 
                    "file_id": "'$FILE_ID'"
                }
            ]
        }],
        "tools": [{
            "type": "code_execution_20250825",
            "name": "code_execution"
        }]
    }'
```

```python Python
# Upload a file
file_object = client.beta.files.upload(
    file=open("data.csv", "rb"),
)

# Use it with code execution
response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    betas=["code-execution-2025-08-25", "files-api-2025-04-14"],
    max_tokens=4096,
    messages=[{
        "role": "user",
        "content": [
            {"type": "text", "text": "Analyze this CSV data: create a summary report, save visualizations, and create a README with the findings"},
            {"type": "container_upload", "file_id": file_object.id}
        ]
    }],
    tools=[{
        "type": "code_execution_20250825",
        "name": "code_execution"
    }]
)

# Claude might:
# 1. Use bash to check file size and preview data
# 2. Use text_editor to write Python code to analyze the CSV and create visualizations
# 3. Use bash to run the Python code
# 4. Use text_editor to create a README.md with findings
# 5. Use bash to organize files into a report directory
```

```typescript TypeScript
// Upload a file
const fileObject = await anthropic.beta.files.create({
  file: createReadStream("data.csv"),
});

// Use it with code execution
const response = await anthropic.beta.messages.create({
  model: "claude-sonnet-4-5",
  betas: ["code-execution-2025-08-25", "files-api-2025-04-14"],
  max_tokens: 4096,
  messages: [{
    role: "user",
    content: [
      {type: "text", text: "Analyze this CSV data: create a summary report, save visualizations, and create a README with the findings"},
      {type: "container_upload", file_id: fileObject.id}
    ]
  }],
  tools: [{
    type: "code_execution_20250825",
    name: "code_execution"
  }]
});

// Claude might:
// 1. Use bash to check file size and preview data
// 2. Use text_editor to write Python code to analyze the CSV and create visualizations
// 3. Use bash to run the Python code
// 4. Use text_editor to create a README.md with findings
// 5. Use bash to organize files into a report directory
```
</CodeGroup>

## Definizione dello strumento

Lo strumento di esecuzione del codice non richiede parametri aggiuntivi:

```json JSON
{
  "type": "code_execution_20250825",
  "name": "code_execution"
}
```

Quando questo strumento viene fornito, Claude ottiene automaticamente accesso a due sotto-strumenti:
- `bash_code_execution`: Esegui comandi shell
- `text_editor_code_execution`: Visualizza, crea e modifica file, inclusa la scrittura di codice

## Formato della risposta

Lo strumento di esecuzione del codice può restituire due tipi di risultati a seconda dell'operazione:

### Risposta del comando Bash

```json
{
  "type": "server_tool_use",
  "id": "srvtoolu_01B3C4D5E6F7G8H9I0J1K2L3",
  "name": "bash_code_execution",
  "input": {
    "command": "ls -la | head -5"
  }
},
{
  "type": "bash_code_execution_tool_result",
  "tool_use_id": "srvtoolu_01B3C4D5E6F7G8H9I0J1K2L3",
  "content": {
    "type": "bash_code_execution_result",
    "stdout": "total 24\ndrwxr-xr-x 2 user user 4096 Jan 1 12:00 .\ndrwxr-xr-x 3 user user 4096 Jan 1 11:00 ..\n-rw-r--r-- 1 user user  220 Jan 1 12:00 data.csv\n-rw-r--r-- 1 user user  180 Jan 1 12:00 config.json",
    "stderr": "",
    "return_code": 0
  }
}
```

### Risposte delle operazioni su file

**Visualizza file:**
```json
{
  "type": "server_tool_use",
  "id": "srvtoolu_01C4D5E6F7G8H9I0J1K2L3M4",
  "name": "text_editor_code_execution",
  "input": {
    "command": "view",
    "path": "config.json"
  }
},
{
  "type": "text_editor_code_execution_tool_result",
  "tool_use_id": "srvtoolu_01C4D5E6F7G8H9I0J1K2L3M4",
  "content": {
    "type": "text_editor_code_execution_result",
    "file_type": "text",
    "content": "{\n  \"setting\": \"value\",\n  \"debug\": true\n}",
    "numLines": 4,
    "startLine": 1,
    "totalLines": 4
  }
}
```

**Crea file:**
```json
{
  "type": "server_tool_use",
  "id": "srvtoolu_01D5E6F7G8H9I0J1K2L3M4N5",
  "name": "text_editor_code_execution",
  "input": {
    "command": "create",
    "path": "new_file.txt",
    "file_text": "Hello, World!"
  }
},
{
  "type": "text_editor_code_execution_tool_result",
  "tool_use_id": "srvtoolu_01D5E6F7G8H9I0J1K2L3M4N5",
  "content": {
    "type": "text_editor_code_execution_result",
    "is_file_update": false
  }
}
```

**Modifica file (str_replace):**
```json
{
  "type": "server_tool_use",
  "id": "srvtoolu_01E6F7G8H9I0J1K2L3M4N5O6",
  "name": "text_editor_code_execution",
  "input": {
    "command": "str_replace",
    "path": "config.json",
    "old_str": "\"debug\": true",
    "new_str": "\"debug\": false"
  }
},
{
  "type": "text_editor_code_execution_tool_result",
  "tool_use_id": "srvtoolu_01E6F7G8H9I0J1K2L3M4N5O6",
  "content": {
    "type": "text_editor_code_execution_result",
    "oldStart": 3,
    "oldLines": 1,
    "newStart": 3,
    "newLines": 1,
    "lines": ["-  \"debug\": true", "+  \"debug\": false"]
  }
}
```

### Risultati

Tutti i risultati dell'esecuzione includono:
- `stdout`: Output dall'esecuzione riuscita
- `stderr`: Messaggi di errore se l'esecuzione fallisce
- `return_code`: 0 per il successo, non zero per il fallimento

Campi aggiuntivi per le operazioni su file:
- **Visualizza**: `file_type`, `content`, `numLines`, `startLine`, `totalLines`
- **Crea**: `is_file_update` (se il file esisteva già)
- **Modifica**: `oldStart`, `oldLines`, `newStart`, `newLines`, `lines` (formato diff)

### Errori

Ogni tipo di strumento può restituire errori specifici:

**Errori comuni (tutti gli strumenti):**
```json
{
  "type": "bash_code_execution_tool_result",
  "tool_use_id": "srvtoolu_01VfmxgZ46TiHbmXgy928hQR",
  "content": {
    "type": "bash_code_execution_tool_result_error",
    "error_code": "unavailable"
  }
}
```

**Codici di errore per tipo di strumento:**

| Strumento | Codice di errore | Descrizione |
|------|-----------|-------------|
| Tutti gli strumenti | `unavailable` | Lo strumento è temporaneamente non disponibile |
| Tutti gli strumenti | `execution_time_exceeded` | L'esecuzione ha superato il limite di tempo massimo |
| Tutti gli strumenti | `container_expired` | Il contenitore è scaduto e non è più disponibile |
| Tutti gli strumenti | `invalid_tool_input` | Parametri non validi forniti allo strumento |
| Tutti gli strumenti | `too_many_requests` | Limite di velocità superato per l'utilizzo dello strumento |
| text_editor | `file_not_found` | Il file non esiste (per operazioni di visualizzazione/modifica) |
| text_editor | `string_not_found` | `old_str` non trovato nel file (per str_replace) |

#### Motivo di arresto `pause_turn`

La risposta può includere un motivo di arresto `pause_turn`, che indica che l'API ha messo in pausa un turno di lunga durata. Puoi fornire la risposta così com'è in una richiesta successiva per consentire a Claude di continuare il suo turno, oppure modificare il contenuto se desideri interrompere la conversazione.

## Contenitori

Lo strumento di esecuzione del codice viene eseguito in un ambiente containerizzato sicuro progettato specificamente per l'esecuzione del codice, con un focus maggiore su Python.

### Ambiente di runtime
- **Versione Python**: 3.11.12
- **Sistema operativo**: Contenitore basato su Linux
- **Architettura**: x86_64 (AMD64)

### Limiti di risorse
- **Memoria**: 5GiB RAM
- **Spazio su disco**: 5GiB di archiviazione dell'area di lavoro
- **CPU**: 1 CPU

### Rete e sicurezza
- **Accesso a Internet**: Completamente disabilitato per motivi di sicurezza
- **Connessioni esterne**: Nessuna richiesta di rete in uscita consentita
- **Isolamento sandbox**: Isolamento completo dal sistema host e da altri contenitori
- **Accesso ai file**: Limitato solo alla directory dell'area di lavoro
- **Ambito dell'area di lavoro**: Come [Files](/docs/it/build-with-claude/files), i contenitori sono limitati all'area di lavoro della chiave API
- **Scadenza**: I contenitori scadono 30 giorni dopo la creazione

### Librerie preinstallate
L'ambiente Python in sandbox include queste librerie comunemente utilizzate:
- **Data Science**: pandas, numpy, scipy, scikit-learn, statsmodels
- **Visualizzazione**: matplotlib, seaborn
- **Elaborazione file**: pyarrow, openpyxl, xlsxwriter, xlrd, pillow, python-pptx, python-docx, pypdf, pdfplumber, pypdfium2, pdf2image, pdfkit, tabula-py, reportlab[pycairo], Img2pdf
- **Matematica e calcolo**: sympy, mpmath
- **Utilità**: tqdm, python-dateutil, pytz, joblib, unzip, unrar, 7zip, bc, rg (ripgrep), fd, sqlite

## Riutilizzo del contenitore

Puoi riutilizzare un contenitore esistente in più richieste API fornendo l'ID del contenitore da una risposta precedente.
Questo ti consente di mantenere i file creati tra le richieste.

### Esempio

<CodeGroup>
```python Python
import os
from anthropic import Anthropic

# Initialize the client
client = Anthropic(
    api_key=os.getenv("ANTHROPIC_API_KEY")
)

# First request: Create a file with a random number
response1 = client.beta.messages.create(
    model="claude-sonnet-4-5",
    betas=["code-execution-2025-08-25"],
    max_tokens=4096,
    messages=[{
        "role": "user",
        "content": "Write a file with a random number and save it to '/tmp/number.txt'"
    }],
    tools=[{
        "type": "code_execution_20250825",
        "name": "code_execution"
    }]
)

# Extract the container ID from the first response
container_id = response1.container.id

# Second request: Reuse the container to read the file
response2 = client.beta.messages.create(
    container=container_id,  # Reuse the same container
    model="claude-sonnet-4-5",
    betas=["code-execution-2025-08-25"],
    max_tokens=4096,
    messages=[{
        "role": "user",
        "content": "Read the number from '/tmp/number.txt' and calculate its square"
    }],
    tools=[{
        "type": "code_execution_20250825",
        "name": "code_execution"
    }]
)
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

async function main() {
  // First request: Create a file with a random number
  const response1 = await anthropic.beta.messages.create({
    model: "claude-sonnet-4-5",
    betas: ["code-execution-2025-08-25"],
    max_tokens: 4096,
    messages: [{
      role: "user",
      content: "Write a file with a random number and save it to '/tmp/number.txt'"
    }],
    tools: [{
      type: "code_execution_20250825",
      name: "code_execution"
    }]
  });

  // Extract the container ID from the first response
  const containerId = response1.container.id;

  // Second request: Reuse the container to read the file
  const response2 = await anthropic.beta.messages.create({
    container: containerId,  // Reuse the same container
    model: "claude-sonnet-4-5",
    betas: ["code-execution-2025-08-25"],
    max_tokens: 4096,
    messages: [{
      role: "user",
      content: "Read the number from '/tmp/number.txt' and calculate its square"
    }],
    tools: [{
      type: "code_execution_20250825",
      name: "code_execution"
    }]
  });

  console.log(response2.content);
}

main().catch(console.error);
```

```bash Shell
# First request: Create a file with a random number
curl https://api.anthropic.com/v1/messages \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "anthropic-beta: code-execution-2025-08-25" \
    --header "content-type: application/json" \
    --data '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 4096,
        "messages": [{
            "role": "user",
            "content": "Write a file with a random number and save it to \"/tmp/number.txt\""
        }],
        "tools": [{
            "type": "code_execution_20250825",
            "name": "code_execution"
        }]
    }' > response1.json

# Extract container ID from the response (using jq)
CONTAINER_ID=$(jq -r '.container.id' response1.json)

# Second request: Reuse the container to read the file
curl https://api.anthropic.com/v1/messages \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "anthropic-beta: code-execution-2025-08-25" \
    --header "content-type: application/json" \
    --data '{
        "container": "'$CONTAINER_ID'",
        "model": "claude-sonnet-4-5",
        "max_tokens": 4096,
        "messages": [{
            "role": "user",
            "content": "Read the number from \"/tmp/number.txt\" and calculate its square"
        }],
        "tools": [{
            "type": "code_execution_20250825",
            "name": "code_execution"
        }]
    }'
```
</CodeGroup>

## Streaming

Con lo streaming abilitato, riceverai gli eventi di esecuzione del codice mentre si verificano:

```javascript
event: content_block_start
data: {"type": "content_block_start", "index": 1, "content_block": {"type": "server_tool_use", "id": "srvtoolu_xyz789", "name": "code_execution"}}

// Code execution streamed
event: content_block_delta
data: {"type": "content_block_delta", "index": 1, "delta": {"type": "input_json_delta", "partial_json": "{\"code\":\"import pandas as pd\\ndf = pd.read_csv('data.csv')\\nprint(df.head())\"}"}}

// Pause while code executes

// Execution results streamed
event: content_block_start
data: {"type": "content_block_start", "index": 2, "content_block": {"type": "code_execution_tool_result", "tool_use_id": "srvtoolu_xyz789", "content": {"stdout": "   A  B  C\n0  1  2  3\n1  4  5  6", "stderr": ""}}}
```

## Richieste batch

Puoi includere lo strumento di esecuzione del codice nell'[API Messages Batches](/docs/it/build-with-claude/batch-processing). Le chiamate dello strumento di esecuzione del codice tramite l'API Messages Batches hanno lo stesso prezzo di quelle nelle richieste API Messages regolari.

## Utilizzo e prezzi

Code execution tool usage is tracked separately from token usage. Execution time has a minimum of 5 minutes.
If files are included in the request, execution time is billed even if the tool is not used due to files being preloaded onto the container.

Each organization receives 1,550 free hours of usage with the code execution tool per month. Additional usage beyond the first 1,550 hours is billed at $0.05 per hour, per container.

## Aggiorna alla versione più recente dello strumento

Aggiornando a `code-execution-2025-08-25`, ottieni accesso alla manipolazione dei file e alle capacità Bash, incluso il codice in più linguaggi. Non c'è differenza di prezzo.

### Cosa è cambiato

| Componente | Legacy | Attuale |
|-----------|------------------|----------------------------|
| Intestazione beta | `code-execution-2025-05-22` | `code-execution-2025-08-25` |
| Tipo di strumento | `code_execution_20250522` | `code_execution_20250825` |
| Capacità | Solo Python | Comandi Bash, operazioni su file |
| Tipi di risposta | `code_execution_result` | `bash_code_execution_result`, `text_editor_code_execution_result` |

### Compatibilità all'indietro

- Tutto il codice Python di esecuzione esistente continua a funzionare esattamente come prima
- Nessun cambiamento richiesto ai flussi di lavoro esistenti solo Python

### Passaggi di aggiornamento

Per eseguire l'aggiornamento, devi apportare le seguenti modifiche alle tue richieste API:

1. **Aggiorna l'intestazione beta**:
   ```diff
   - "anthropic-beta": "code-execution-2025-05-22"
   + "anthropic-beta": "code-execution-2025-08-25"
   ```

2. **Aggiorna il tipo di strumento**:
   ```diff
   - "type": "code_execution_20250522"
   + "type": "code_execution_20250825"
   ```

3. **Rivedi la gestione della risposta** (se analizzi le risposte a livello di programmazione):
   - I blocchi precedenti per le risposte di esecuzione Python non verranno più inviati
   - Invece, verranno inviati nuovi tipi di risposta per operazioni Bash e file (vedi sezione Formato della risposta)

## Chiamata dello strumento a livello di programmazione

Lo strumento di esecuzione del codice alimenta la [chiamata dello strumento a livello di programmazione](/docs/it/agents-and-tools/tool-use/programmatic-tool-calling), che consente a Claude di scrivere codice che chiama i tuoi strumenti personalizzati a livello di programmazione all'interno del contenitore di esecuzione. Ciò consente flussi di lavoro multi-strumento efficienti, filtraggio dei dati prima di raggiungere il contesto di Claude e logica condizionale complessa.

<CodeGroup>
```python Python
# Enable programmatic calling for your tools
response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    betas=["advanced-tool-use-2025-11-20"],
    max_tokens=4096,
    messages=[{
        "role": "user",
        "content": "Get weather for 5 cities and find the warmest"
    }],
    tools=[
        {
            "type": "code_execution_20250825",
            "name": "code_execution"
        },
        {
            "name": "get_weather",
            "description": "Get weather for a city",
            "input_schema": {...},
            "allowed_callers": ["code_execution_20250825"]  # Enable programmatic calling
        }
    ]
)
```
</CodeGroup>

Scopri di più nella [documentazione sulla chiamata dello strumento a livello di programmazione](/docs/it/agents-and-tools/tool-use/programmatic-tool-calling).

## Utilizzo dell'esecuzione del codice con Agent Skills

Lo strumento di esecuzione del codice consente a Claude di utilizzare [Agent Skills](/docs/it/agents-and-tools/agent-skills/overview). Le Skills sono capacità modulari costituite da istruzioni, script e risorse che estendono la funzionalità di Claude.

Scopri di più nella [documentazione di Agent Skills](/docs/it/agents-and-tools/agent-skills/overview) e nella [guida all'API di Agent Skills](/docs/it/build-with-claude/skills-guide).