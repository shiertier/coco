# Strumento bash

Lo strumento bash consente a Claude di eseguire comandi shell in una sessione bash persistente, permettendo operazioni di sistema, esecuzione di script e automazione da riga di comando.

---

Lo strumento bash consente a Claude di eseguire comandi shell in una sessione bash persistente, permettendo operazioni di sistema, esecuzione di script e automazione da riga di comando.

## Panoramica

Lo strumento bash fornisce a Claude:
- Sessione bash persistente che mantiene lo stato
- Capacità di eseguire qualsiasi comando shell
- Accesso alle variabili di ambiente e alla directory di lavoro
- Capacità di concatenamento di comandi e scripting

## Compatibilità dei modelli

| Modello | Versione dello strumento |
|-------|--------------|
| Modelli Claude 4 e Sonnet 3.7 ([deprecato](/docs/it/about-claude/model-deprecations)) | `bash_20250124` |

<Warning>
Le versioni precedenti dello strumento non sono garantite essere retrocompatibili con i modelli più recenti. Utilizza sempre la versione dello strumento che corrisponde alla versione del tuo modello.
</Warning>

## Casi d'uso

- **Flussi di lavoro di sviluppo**: Esegui comandi di build, test e strumenti di sviluppo
- **Automazione del sistema**: Esegui script, gestisci file, automatizza attività
- **Elaborazione dei dati**: Elabora file, esegui script di analisi, gestisci dataset
- **Configurazione dell'ambiente**: Installa pacchetti, configura ambienti

## Avvio rapido

<CodeGroup>
```python Python
import anthropic

client = anthropic.Anthropic()

response = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    tools=[
        {
            "type": "bash_20250124",
            "name": "bash"
        }
    ],
    messages=[
        {"role": "user", "content": "List all Python files in the current directory."}
    ]
)
```

```bash Shell
curl https://api.anthropic.com/v1/messages \
  -H "content-type: application/json" \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -d '{
    "model": "claude-sonnet-4-5",
    "max_tokens": 1024,
    "tools": [
      {
        "type": "bash_20250124",
        "name": "bash"
      }
    ],
    "messages": [
      {
        "role": "user",
        "content": "List all Python files in the current directory."
      }
    ]
  }'
```
</CodeGroup>

## Come funziona

Lo strumento bash mantiene una sessione persistente:

1. Claude determina quale comando eseguire
2. Esegui il comando in una shell bash
3. Restituisci l'output (stdout e stderr) a Claude
4. Lo stato della sessione persiste tra i comandi (variabili di ambiente, directory di lavoro)

## Parametri

| Parametro | Obbligatorio | Descrizione |
|-----------|----------|-------------|
| `command` | Sì* | Il comando bash da eseguire |
| `restart` | No | Impostare su `true` per riavviare la sessione bash |

*Obbligatorio a meno che non si utilizzi `restart`

<section title="Esempio di utilizzo">

```json
// Esegui un comando
{
  "command": "ls -la *.py"
}

// Riavvia la sessione
{
  "restart": true
}
```

</section>

## Esempio: Automazione multi-step

Claude può concatenare comandi per completare attività complesse:

```python
# Richiesta dell'utente
"Install the requests library and create a simple Python script that fetches a joke from an API, then run it."

# Lo strumento di Claude utilizza:
# 1. Installa il pacchetto
{"command": "pip install requests"}

# 2. Crea lo script
{"command": "cat > fetch_joke.py << 'EOF'\nimport requests\nresponse = requests.get('https://official-joke-api.appspot.com/random_joke')\njoke = response.json()\nprint(f\"Setup: {joke['setup']}\")\nprint(f\"Punchline: {joke['punchline']}\")\nEOF"}

# 3. Esegui lo script
{"command": "python fetch_joke.py"}
```

La sessione mantiene lo stato tra i comandi, quindi i file creati nel passaggio 2 sono disponibili nel passaggio 3.

***

## Implementare lo strumento bash

Lo strumento bash è implementato come uno strumento senza schema. Quando utilizzi questo strumento, non è necessario fornire uno schema di input come con altri strumenti; lo schema è integrato nel modello di Claude e non può essere modificato.

<Steps>
  <Step title="Configura un ambiente bash">
    Crea una sessione bash persistente con cui Claude può interagire:
    ```python
    import subprocess
    import threading
    import queue
    
    class BashSession:
        def __init__(self):
            self.process = subprocess.Popen(
                ['/bin/bash'],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                bufsize=0
            )
            self.output_queue = queue.Queue()
            self.error_queue = queue.Queue()
            self._start_readers()
    ```
  </Step>
  <Step title="Gestisci l'esecuzione dei comandi">
    Crea una funzione per eseguire comandi e acquisire l'output:
    ```python
    def execute_command(self, command):
        # Invia il comando a bash
        self.process.stdin.write(command + '\n')
        self.process.stdin.flush()
        
        # Acquisisci l'output con timeout
        output = self._read_output(timeout=10)
        return output
    ```
  </Step>
  <Step title="Elabora le chiamate degli strumenti di Claude">
    Estrai ed esegui i comandi dalle risposte di Claude:
    ```python
    for content in response.content:
        if content.type == "tool_use" and content.name == "bash":
            if content.input.get("restart"):
                bash_session.restart()
                result = "Bash session restarted"
            else:
                command = content.input.get("command")
                result = bash_session.execute_command(command)
            
            # Restituisci il risultato a Claude
            tool_result = {
                "type": "tool_result",
                "tool_use_id": content.id,
                "content": result
            }
    ```
  </Step>
  <Step title="Implementa misure di sicurezza">
    Aggiungi convalida e restrizioni:
    ```python
    def validate_command(command):
        # Blocca i comandi pericolosi
        dangerous_patterns = ['rm -rf /', 'format', ':(){:|:&};:']
        for pattern in dangerous_patterns:
            if pattern in command:
                return False, f"Command contains dangerous pattern: {pattern}"
        
        # Aggiungi ulteriore convalida secondo le necessità
        return True, None
    ```
  </Step>
</Steps>

### Gestisci gli errori

Quando implementi lo strumento bash, gestisci vari scenari di errore:

<section title="Timeout dell'esecuzione del comando">

Se un comando impiega troppo tempo per essere eseguito:

```json
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": "Error: Command timed out after 30 seconds",
      "is_error": true
    }
  ]
}
```

</section>

<section title="Comando non trovato">

Se un comando non esiste:

```json
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": "bash: nonexistentcommand: command not found",
      "is_error": true
    }
  ]
}
```

</section>

<section title="Permesso negato">

Se ci sono problemi di permessi:

```json
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": "bash: /root/sensitive-file: Permission denied",
      "is_error": true
    }
  ]
}
```

</section>

### Segui le best practice di implementazione

<section title="Utilizza timeout dei comandi">

Implementa timeout per prevenire comandi in sospeso:
```python
def execute_with_timeout(command, timeout=30):
    try:
        result = subprocess.run(
            command, 
            shell=True, 
            capture_output=True, 
            text=True, 
            timeout=timeout
        )
        return result.stdout + result.stderr
    except subprocess.TimeoutExpired:
        return f"Command timed out after {timeout} seconds"
```

</section>

<section title="Mantieni lo stato della sessione">

Mantieni la sessione bash persistente per conservare le variabili di ambiente e la directory di lavoro:
```python
# I comandi eseguiti nella stessa sessione mantengono lo stato
commands = [
    "cd /tmp",
    "echo 'Hello' > test.txt",
    "cat test.txt"  # Questo funziona perché siamo ancora in /tmp
]
```

</section>

<section title="Gestisci output di grandi dimensioni">

Tronca output molto grandi per evitare problemi di limite di token:
```python
def truncate_output(output, max_lines=100):
    lines = output.split('\n')
    if len(lines) > max_lines:
        truncated = '\n'.join(lines[:max_lines])
        return f"{truncated}\n\n... Output truncated ({len(lines)} total lines) ..."
    return output
```

</section>

<section title="Registra tutti i comandi">

Mantieni un registro di controllo dei comandi eseguiti:
```python
import logging

def log_command(command, output, user_id):
    logging.info(f"User {user_id} executed: {command}")
    logging.info(f"Output: {output[:200]}...")  # Registra i primi 200 caratteri
```

</section>

<section title="Sanitizza gli output">

Rimuovi informazioni sensibili dagli output dei comandi:
```python
def sanitize_output(output):
    # Rimuovi potenziali segreti o credenziali
    import re
    # Esempio: Rimuovi le credenziali AWS
    output = re.sub(r'aws_access_key_id\s*=\s*\S+', 'aws_access_key_id=***', output)
    output = re.sub(r'aws_secret_access_key\s*=\s*\S+', 'aws_secret_access_key=***', output)
    return output
```

</section>

## Sicurezza

<Warning>
Lo strumento bash fornisce accesso diretto al sistema. Implementa queste misure di sicurezza essenziali:
- Esecuzione in ambienti isolati (Docker/VM)
- Implementazione del filtraggio dei comandi e delle liste di autorizzazione
- Impostazione dei limiti di risorse (CPU, memoria, disco)
- Registrazione di tutti i comandi eseguiti
</Warning>

### Raccomandazioni chiave
- Utilizza `ulimit` per impostare vincoli di risorse
- Filtra i comandi pericolosi (`sudo`, `rm -rf`, ecc.)
- Esegui con permessi utente minimi
- Monitora e registra tutta l'esecuzione dei comandi

## Prezzi

The bash tool adds **245 input tokens** to your API calls.

Additional tokens are consumed by:
- Command outputs (stdout/stderr)
- Error messages
- Large file contents

Vedi [prezzi dell'utilizzo degli strumenti](/docs/it/agents-and-tools/tool-use/overview#pricing) per i dettagli completi sui prezzi.

## Modelli comuni

### Flussi di lavoro di sviluppo
- Esecuzione dei test: `pytest && coverage report`
- Compilazione dei progetti: `npm install && npm run build`
- Operazioni Git: `git status && git add . && git commit -m "message"`

### Operazioni su file
- Elaborazione dei dati: `wc -l *.csv && ls -lh *.csv`
- Ricerca nei file: `find . -name "*.py" | xargs grep "pattern"`
- Creazione di backup: `tar -czf backup.tar.gz ./data`

### Attività di sistema
- Verifica delle risorse: `df -h && free -m`
- Gestione dei processi: `ps aux | grep python`
- Configurazione dell'ambiente: `export PATH=$PATH:/new/path && echo $PATH`

## Limitazioni

- **Nessun comando interattivo**: Non può gestire `vim`, `less` o prompt di password
- **Nessuna applicazione GUI**: Solo riga di comando
- **Ambito della sessione**: Persiste all'interno della conversazione, perso tra le chiamate API
- **Limiti di output**: Gli output di grandi dimensioni possono essere troncati
- **Nessuno streaming**: I risultati vengono restituiti dopo il completamento

## Combinazione con altri strumenti

Lo strumento bash è più potente quando combinato con l'[editor di testo](/docs/it/agents-and-tools/tool-use/text-editor-tool) e altri strumenti.

## Passaggi successivi

<CardGroup cols={2}>
  <Card
    title="Panoramica dell'utilizzo degli strumenti"
    icon="tool"
    href="/docs/it/agents-and-tools/tool-use/overview"
  >
    Scopri l'utilizzo degli strumenti con Claude
  </Card>

  <Card
    title="Strumento editor di testo"
    icon="file"
    href="/docs/it/agents-and-tools/tool-use/text-editor-tool"
  >
    Visualizza e modifica file di testo con Claude
  </Card>
</CardGroup>