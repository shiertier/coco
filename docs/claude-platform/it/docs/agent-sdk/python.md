# Riferimento SDK Agent - Python

Riferimento API completo per l'SDK Agent Python, incluse tutte le funzioni, i tipi e le classi.

---

## Installazione

```bash
pip install claude-agent-sdk
```

## Scelta tra `query()` e `ClaudeSDKClient`

L'SDK Python fornisce due modi per interagire con Claude Code:

### Confronto rapido

| Funzionalità        | `query()`                     | `ClaudeSDKClient`                  |
| :------------------ | :---------------------------- | :--------------------------------- |
| **Sessione**        | Crea una nuova sessione ogni volta | Riutilizza la stessa sessione                |
| **Conversazione**   | Singolo scambio               | Più scambi nello stesso contesto |
| **Connessione**     | Gestita automaticamente         | Controllo manuale                     |
| **Input in streaming** | ✅ Supportato                  | ✅ Supportato                       |
| **Interruzioni**    | ❌ Non supportato              | ✅ Supportato                       |
| **Hook**            | ❌ Non supportato              | ✅ Supportato                       |
| **Strumenti personalizzati** | ❌ Non supportato              | ✅ Supportato                       |
| **Continua chat**   | ❌ Nuova sessione ogni volta      | ✅ Mantiene la conversazione          |
| **Caso d'uso**      | Attività una tantum                 | Conversazioni continue           |

### Quando usare `query()` (Nuova sessione ogni volta)

**Migliore per:**

- Domande una tantum dove non hai bisogno della cronologia della conversazione
- Attività indipendenti che non richiedono contesto da scambi precedenti
- Script di automazione semplici
- Quando vuoi un nuovo inizio ogni volta

### Quando usare `ClaudeSDKClient` (Conversazione continua)

**Migliore per:**

- **Continuare conversazioni** - Quando hai bisogno che Claude ricordi il contesto
- **Domande di follow-up** - Costruire su risposte precedenti
- **Applicazioni interattive** - Interfacce di chat, REPL
- **Logica guidata dalla risposta** - Quando l'azione successiva dipende dalla risposta di Claude
- **Controllo della sessione** - Gestire il ciclo di vita della conversazione esplicitamente

## Funzioni

### `query()`

Crea una nuova sessione per ogni interazione con Claude Code. Restituisce un iteratore asincrono che produce messaggi man mano che arrivano. Ogni chiamata a `query()` inizia da zero senza memoria di interazioni precedenti.

```python
async def query(
    *,
    prompt: str | AsyncIterable[dict[str, Any]],
    options: ClaudeAgentOptions | None = None
) -> AsyncIterator[Message]
```

#### Parametri

| Parametro | Tipo                         | Descrizione                                                                |
| :-------- | :--------------------------- | :------------------------------------------------------------------------- |
| `prompt`  | `str \| AsyncIterable[dict]` | Il prompt di input come stringa o iterabile asincrono per la modalità streaming          |
| `options` | `ClaudeAgentOptions \| None` | Oggetto di configurazione opzionale (predefinito a `ClaudeAgentOptions()` se None) |

#### Restituisce

Restituisce un `AsyncIterator[Message]` che produce messaggi dalla conversazione.

#### Esempio - Con opzioni

```python

import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions

async def main():
    options = ClaudeAgentOptions(
        system_prompt="You are an expert Python developer",
        permission_mode='acceptEdits',
        cwd="/home/user/project"
    )

    async for message in query(
        prompt="Create a Python web server",
        options=options
    ):
        print(message)


asyncio.run(main())
```

### `tool()`

Decoratore per definire strumenti MCP con sicurezza dei tipi.

```python
def tool(
    name: str,
    description: str,
    input_schema: type | dict[str, Any]
) -> Callable[[Callable[[Any], Awaitable[dict[str, Any]]]], SdkMcpTool[Any]]
```

#### Parametri

| Parametro      | Tipo                     | Descrizione                                             |
| :------------- | :----------------------- | :------------------------------------------------------ |
| `name`         | `str`                    | Identificatore univoco per lo strumento                          |
| `description`  | `str`                    | Descrizione leggibile di cosa fa lo strumento        |
| `input_schema` | `type \| dict[str, Any]` | Schema che definisce i parametri di input dello strumento (vedi sotto) |

#### Opzioni dello schema di input

1. **Mapping di tipo semplice** (consigliato):

   ```python
   {"text": str, "count": int, "enabled": bool}
   ```

2. **Formato JSON Schema** (per validazione complessa):
   ```python
   {
       "type": "object",
       "properties": {
           "text": {"type": "string"},
           "count": {"type": "integer", "minimum": 0}
       },
       "required": ["text"]
   }
   ```

#### Restituisce

Una funzione decoratore che avvolge l'implementazione dello strumento e restituisce un'istanza di `SdkMcpTool`.

#### Esempio

```python
from claude_agent_sdk import tool
from typing import Any

@tool("greet", "Greet a user", {"name": str})
async def greet(args: dict[str, Any]) -> dict[str, Any]:
    return {
        "content": [{
            "type": "text",
            "text": f"Hello, {args['name']}!"
        }]
    }
```

### `create_sdk_mcp_server()`

Crea un server MCP in-process che viene eseguito all'interno della tua applicazione Python.

```python
def create_sdk_mcp_server(
    name: str,
    version: str = "1.0.0",
    tools: list[SdkMcpTool[Any]] | None = None
) -> McpSdkServerConfig
```

#### Parametri

| Parametro | Tipo                            | Predefinito   | Descrizione                                           |
| :-------- | :------------------------------ | :-------- | :---------------------------------------------------- |
| `name`    | `str`                           | -         | Identificatore univoco per il server                      |
| `version` | `str`                           | `"1.0.0"` | Stringa della versione del server                                 |
| `tools`   | `list[SdkMcpTool[Any]] \| None` | `None`    | Elenco di funzioni strumento create con il decoratore `@tool` |

#### Restituisce

Restituisce un oggetto `McpSdkServerConfig` che può essere passato a `ClaudeAgentOptions.mcp_servers`.

#### Esempio

```python
from claude_agent_sdk import tool, create_sdk_mcp_server

@tool("add", "Add two numbers", {"a": float, "b": float})
async def add(args):
    return {
        "content": [{
            "type": "text",
            "text": f"Sum: {args['a'] + args['b']}"
        }]
    }

@tool("multiply", "Multiply two numbers", {"a": float, "b": float})
async def multiply(args):
    return {
        "content": [{
            "type": "text",
            "text": f"Product: {args['a'] * args['b']}"
        }]
    }

calculator = create_sdk_mcp_server(
    name="calculator",
    version="2.0.0",
    tools=[add, multiply]  # Pass decorated functions
)

# Use with Claude
options = ClaudeAgentOptions(
    mcp_servers={"calc": calculator},
    allowed_tools=["mcp__calc__add", "mcp__calc__multiply"]
)
```

## Classi

### `ClaudeSDKClient`

**Mantiene una sessione di conversazione su più scambi.** Questo è l'equivalente Python di come la funzione `query()` dell'SDK TypeScript funziona internamente - crea un oggetto client che può continuare conversazioni.

#### Caratteristiche principali

- **Continuità della sessione**: Mantiene il contesto della conversazione su più chiamate a `query()`
- **Stessa conversazione**: Claude ricorda i messaggi precedenti nella sessione
- **Supporto per interruzioni**: Può fermare Claude durante l'esecuzione
- **Ciclo di vita esplicito**: Tu controlli quando la sessione inizia e termina
- **Flusso guidato dalla risposta**: Può reagire alle risposte e inviare follow-up
- **Strumenti personalizzati e hook**: Supporta strumenti personalizzati (creati con il decoratore `@tool`) e hook

```python
class ClaudeSDKClient:
    def __init__(self, options: ClaudeAgentOptions | None = None)
    async def connect(self, prompt: str | AsyncIterable[dict] | None = None) -> None
    async def query(self, prompt: str | AsyncIterable[dict], session_id: str = "default") -> None
    async def receive_messages(self) -> AsyncIterator[Message]
    async def receive_response(self) -> AsyncIterator[Message]
    async def interrupt(self) -> None
    async def rewind_files(self, user_message_uuid: str) -> None
    async def disconnect(self) -> None
```

#### Metodi

| Metodo                      | Descrizione                                                         |
| :-------------------------- | :------------------------------------------------------------------ |
| `__init__(options)`         | Inizializza il client con configurazione opzionale                   |
| `connect(prompt)`           | Connettiti a Claude con un prompt iniziale opzionale o flusso di messaggi |
| `query(prompt, session_id)` | Invia una nuova richiesta in modalità streaming                                |
| `receive_messages()`        | Ricevi tutti i messaggi da Claude come iteratore asincrono               |
| `receive_response()`        | Ricevi messaggi fino a e incluso un ResultMessage                |
| `interrupt()`               | Invia segnale di interruzione (funziona solo in modalità streaming)                |
| `rewind_files(user_message_uuid)` | Ripristina i file al loro stato al messaggio utente specificato. Richiede `enable_file_checkpointing=True`. Vedi [File checkpointing](/docs/it/agent-sdk/file-checkpointing) |
| `disconnect()`              | Disconnettiti da Claude                                              |

#### Supporto del context manager

Il client può essere utilizzato come context manager asincrono per la gestione automatica della connessione:

```python
async with ClaudeSDKClient() as client:
    await client.query("Hello Claude")
    async for message in client.receive_response():
        print(message)
```

> **Importante:** Quando iteri sui messaggi, evita di usare `break` per uscire anticipatamente poiché questo può causare problemi di pulizia di asyncio. Invece, lascia che l'iterazione si completi naturalmente o usa flag per tracciare quando hai trovato quello che ti serve.

#### Esempio - Continuare una conversazione

```python
import asyncio
from claude_agent_sdk import ClaudeSDKClient, AssistantMessage, TextBlock, ResultMessage

async def main():
    async with ClaudeSDKClient() as client:
        # First question
        await client.query("What's the capital of France?")

        # Process response
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

        # Follow-up question - Claude remembers the previous context
        await client.query("What's the population of that city?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

        # Another follow-up - still in the same conversation
        await client.query("What are some famous landmarks there?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

asyncio.run(main())
```

#### Esempio - Input in streaming con ClaudeSDKClient

```python
import asyncio
from claude_agent_sdk import ClaudeSDKClient

async def message_stream():
    """Generate messages dynamically."""
    yield {"type": "text", "text": "Analyze the following data:"}
    await asyncio.sleep(0.5)
    yield {"type": "text", "text": "Temperature: 25°C"}
    await asyncio.sleep(0.5)
    yield {"type": "text", "text": "Humidity: 60%"}
    await asyncio.sleep(0.5)
    yield {"type": "text", "text": "What patterns do you see?"}

async def main():
    async with ClaudeSDKClient() as client:
        # Stream input to Claude
        await client.query(message_stream())

        # Process response
        async for message in client.receive_response():
            print(message)

        # Follow-up in same session
        await client.query("Should we be concerned about these readings?")

        async for message in client.receive_response():
            print(message)

asyncio.run(main())
```

#### Esempio - Utilizzo di interruzioni

```python
import asyncio
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions

async def interruptible_task():
    options = ClaudeAgentOptions(
        allowed_tools=["Bash"],
        permission_mode="acceptEdits"
    )

    async with ClaudeSDKClient(options=options) as client:
        # Start a long-running task
        await client.query("Count from 1 to 100 slowly")

        # Let it run for a bit
        await asyncio.sleep(2)

        # Interrupt the task
        await client.interrupt()
        print("Task interrupted!")

        # Send a new command
        await client.query("Just say hello instead")

        async for message in client.receive_response():
            # Process the new response
            pass

asyncio.run(interruptible_task())
```

#### Esempio - Controllo avanzato dei permessi

```python
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions
)

async def custom_permission_handler(
    tool_name: str,
    input_data: dict,
    context: dict
):
    """Custom logic for tool permissions."""

    # Block writes to system directories
    if tool_name == "Write" and input_data.get("file_path", "").startswith("/system/"):
        return {
            "behavior": "deny",
            "message": "System directory write not allowed",
            "interrupt": True
        }

    # Redirect sensitive file operations
    if tool_name in ["Write", "Edit"] and "config" in input_data.get("file_path", ""):
        safe_path = f"./sandbox/{input_data['file_path']}"
        return {
            "behavior": "allow",
            "updatedInput": {**input_data, "file_path": safe_path}
        }

    # Allow everything else
    return {
        "behavior": "allow",
        "updatedInput": input_data
    }

async def main():
    options = ClaudeAgentOptions(
        can_use_tool=custom_permission_handler,
        allowed_tools=["Read", "Write", "Edit"]
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Update the system config file")

        async for message in client.receive_response():
            # Will use sandbox path instead
            print(message)

asyncio.run(main())
```

## Tipi

### `SdkMcpTool`

Definizione per uno strumento MCP SDK creato con il decoratore `@tool`.

```python
@dataclass
class SdkMcpTool(Generic[T]):
    name: str
    description: str
    input_schema: type[T] | dict[str, Any]
    handler: Callable[[T], Awaitable[dict[str, Any]]]
```

| Proprietà      | Tipo                                       | Descrizione                                |
| :------------- | :----------------------------------------- | :----------------------------------------- |
| `name`         | `str`                                      | Identificatore univoco per lo strumento             |
| `description`  | `str`                                      | Descrizione leggibile                 |
| `input_schema` | `type[T] \| dict[str, Any]`                | Schema per la validazione dell'input                |
| `handler`      | `Callable[[T], Awaitable[dict[str, Any]]]` | Funzione asincrona che gestisce l'esecuzione dello strumento |

### `ClaudeAgentOptions`

Dataclass di configurazione per le query di Claude Code.

```python
@dataclass
class ClaudeAgentOptions:
    allowed_tools: list[str] = field(default_factory=list)
    system_prompt: str | SystemPromptPreset | None = None
    mcp_servers: dict[str, McpServerConfig] | str | Path = field(default_factory=dict)
    permission_mode: PermissionMode | None = None
    continue_conversation: bool = False
    resume: str | None = None
    max_turns: int | None = None
    disallowed_tools: list[str] = field(default_factory=list)
    model: str | None = None
    output_format: OutputFormat | None = None
    permission_prompt_tool_name: str | None = None
    cwd: str | Path | None = None
    settings: str | None = None
    add_dirs: list[str | Path] = field(default_factory=list)
    env: dict[str, str] = field(default_factory=dict)
    extra_args: dict[str, str | None] = field(default_factory=dict)
    max_buffer_size: int | None = None
    debug_stderr: Any = sys.stderr  # Deprecated
    stderr: Callable[[str], None] | None = None
    can_use_tool: CanUseTool | None = None
    hooks: dict[HookEvent, list[HookMatcher]] | None = None
    user: str | None = None
    include_partial_messages: bool = False
    fork_session: bool = False
    agents: dict[str, AgentDefinition] | None = None
    setting_sources: list[SettingSource] | None = None
```

| Proprietà                     | Tipo                                         | Predefinito              | Descrizione                                                                                                                                                                             |
| :---------------------------- | :------------------------------------------- | :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowed_tools`               | `list[str]`                                  | `[]`                 | Elenco dei nomi degli strumenti consentiti                                                                                                                                              |
| `system_prompt`               | `str \| SystemPromptPreset \| None`          | `None`               | Configurazione del prompt di sistema. Passa una stringa per un prompt personalizzato, o usa `{"type": "preset", "preset": "claude_code"}` per il prompt di sistema di Claude Code. Aggiungi `"append"` per estendere il preset |
| `mcp_servers`                 | `dict[str, McpServerConfig] \| str \| Path`  | `{}`                 | Configurazioni del server MCP o percorso del file di configurazione                                                                                                                        |
| `permission_mode`             | `PermissionMode \| None`                     | `None`               | Modalità di permesso per l'utilizzo dello strumento                                                                                                                                          |
| `continue_conversation`       | `bool`                                       | `False`              | Continua la conversazione più recente                                                                                                                                                   |
| `resume`                      | `str \| None`                                | `None`               | ID della sessione da riprendere                                                                                                                                                                    |
| `max_turns`                   | `int \| None`                                | `None`               | Numero massimo di turni della conversazione                                                                                                                                              |
| `disallowed_tools`            | `list[str]`                                  | `[]`                 | Elenco dei nomi degli strumenti non consentiti                                                                                                                                           |
| `enable_file_checkpointing`   | `bool`                                       | `False`              | Abilita il tracciamento dei cambiamenti dei file per il rewind. Vedi [File checkpointing](/docs/it/agent-sdk/file-checkpointing)                                                                              |
| `model`                       | `str \| None`                                | `None`               | Modello Claude da utilizzare                                                                                                                                                                     |
| `output_format`               | [`OutputFormat`](#outputformat) ` \| None`   | `None`               | Definisci il formato di output per i risultati dell'agente. Vedi [Structured outputs](/docs/it/agent-sdk/structured-outputs) per i dettagli                                                                    |
| `permission_prompt_tool_name` | `str \| None`                                | `None`               | Nome dello strumento MCP per i prompt di permesso                                                                                                                                                    |
| `cwd`                         | `str \| Path \| None`                        | `None`               | Directory di lavoro corrente                                                                                                                                                               |
| `settings`                    | `str \| None`                                | `None`               | Percorso del file di impostazioni                                                                                                                                                                   |
| `add_dirs`                    | `list[str \| Path]`                          | `[]`                 | Directory aggiuntive a cui Claude può accedere                                                                                                                                                |
| `env`                         | `dict[str, str]`                             | `{}`                 | Variabili di ambiente                                                                                                                                                                   |
| `extra_args`                  | `dict[str, str \| None]`                     | `{}`                 | Argomenti CLI aggiuntivi da passare direttamente alla CLI                                                                                                                                    |
| `max_buffer_size`             | `int \| None`                                | `None`               | Byte massimi durante il buffering dell'stdout della CLI                                                                                                                                 |
| `debug_stderr`                | `Any`                                        | `sys.stderr`         | _Deprecato_ - Oggetto simile a un file per l'output di debug. Usa il callback `stderr` invece                                                                                                         |
| `stderr`                      | `Callable[[str], None] \| None`              | `None`               | Funzione di callback per l'output stderr dalla CLI                                                                                                                                            |
| `can_use_tool`                | `CanUseTool \| None`                         | `None`               | Funzione di callback per il permesso dello strumento                                                                                                                                       |
| `hooks`                       | `dict[HookEvent, list[HookMatcher]] \| None` | `None`               | Configurazioni degli hook per intercettare gli eventi                                                                                                                                             |
| `user`                        | `str \| None`                                | `None`               | Identificatore dell'utente                                                                                                                                                                         |
| `include_partial_messages`    | `bool`                                       | `False`              | Includi eventi di streaming di messaggi parziali                                                                                                                                                |
| `fork_session`                | `bool`                                       | `False`              | Quando si riprende con `resume`, esegui il fork a un nuovo ID di sessione invece di continuare la sessione originale                                                                                        |
| `agents`                      | `dict[str, AgentDefinition] \| None`         | `None`               | Subagenti definiti programmaticamente                                                                                                                                                      |
| `plugins`                     | `list[SdkPluginConfig]`                      | `[]`                 | Carica plugin personalizzati da percorsi locali. Vedi [Plugins](/docs/it/agent-sdk/plugins) per i dettagli                                                                             |
| `sandbox`                     | [`SandboxSettings`](#sandboxsettings) ` \| None` | `None`              | Configura il comportamento della sandbox programmaticamente. Vedi [Sandbox settings](#sandboxsettings) per i dettagli                                        |
| `setting_sources`             | `list[SettingSource] \| None`                | `None` (nessuna impostazione) | Controlla quali impostazioni del filesystem caricare. Se omesso, nessuna impostazione viene caricata. **Nota:** Deve includere `"project"` per caricare i file CLAUDE.md                                             |

### `OutputFormat`

Configurazione per la validazione dell'output strutturato.

```python
class OutputFormat(TypedDict):
    type: Literal["json_schema"]
    schema: dict[str, Any]
```

| Campo    | Obbligatorio | Descrizione                                    |
| :------- | :------- | :--------------------------------------------- |
| `type`   | Sì      | Deve essere `"json_schema"` per la validazione JSON Schema |
| `schema` | Sì      | Definizione JSON Schema per la validazione dell'output   |

### `SystemPromptPreset`

Configurazione per l'utilizzo del prompt di sistema preset di Claude Code con aggiunte opzionali.

```python
class SystemPromptPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
    append: NotRequired[str]
```

| Campo    | Obbligatorio | Descrizione                                                   |
| :------- | :------- | :------------------------------------------------------------ |
| `type`   | Sì      | Deve essere `"preset"` per utilizzare un prompt di sistema preset              |
| `preset` | Sì      | Deve essere `"claude_code"` per utilizzare il prompt di sistema di Claude Code    |
| `append` | No       | Istruzioni aggiuntive da aggiungere al prompt di sistema preset |

### `SettingSource`

Controlla quali fonti di configurazione basate su filesystem l'SDK carica le impostazioni da.

```python
SettingSource = Literal["user", "project", "local"]
```

| Valore      | Descrizione                                  | Posizione                      |
| :---------- | :------------------------------------------- | :---------------------------- |
| `"user"`    | Impostazioni globali dell'utente                         | `~/.claude/settings.json`     |
| `"project"` | Impostazioni del progetto condivise (controllate dalla versione) | `.claude/settings.json`       |
| `"local"`   | Impostazioni del progetto locale (gitignored)          | `.claude/settings.local.json` |

#### Comportamento predefinito

Quando `setting_sources` è **omesso** o **`None`**, l'SDK **non** carica alcuna impostazione del filesystem. Questo fornisce isolamento per le applicazioni SDK.

#### Perché usare setting_sources?

**Carica tutte le impostazioni del filesystem (comportamento legacy):**

```python
# Load all settings like SDK v0.0.x did
from claude_agent_sdk import query, ClaudeAgentOptions

async for message in query(
    prompt="Analyze this code",
    options=ClaudeAgentOptions(
        setting_sources=["user", "project", "local"]  # Load all settings
    )
):
    print(message)
```

**Carica solo fonti di impostazioni specifiche:**

```python
# Load only project settings, ignore user and local
async for message in query(
    prompt="Run CI checks",
    options=ClaudeAgentOptions(
        setting_sources=["project"]  # Only .claude/settings.json
    )
):
    print(message)
```

**Ambienti di test e CI:**

```python
# Ensure consistent behavior in CI by excluding local settings
async for message in query(
    prompt="Run tests",
    options=ClaudeAgentOptions(
        setting_sources=["project"],  # Only team-shared settings
        permission_mode="bypassPermissions"
    )
):
    print(message)
```

**Applicazioni solo SDK:**

```python
# Define everything programmatically (default behavior)
# No filesystem dependencies - setting_sources defaults to None
async for message in query(
    prompt="Review this PR",
    options=ClaudeAgentOptions(
        # setting_sources=None is the default, no need to specify
        agents={ /* ... */ },
        mcp_servers={ /* ... */ },
        allowed_tools=["Read", "Grep", "Glob"]
    )
):
    print(message)
```

**Caricamento delle istruzioni del progetto CLAUDE.md:**

```python
# Load project settings to include CLAUDE.md files
async for message in query(
    prompt="Add a new feature following project conventions",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code"  # Use Claude Code's system prompt
        },
        setting_sources=["project"],  # Required to load CLAUDE.md from project
        allowed_tools=["Read", "Write", "Edit"]
    )
):
    print(message)
```

#### Precedenza delle impostazioni

Quando più fonti vengono caricate, le impostazioni vengono unite con questa precedenza (da più alta a più bassa):

1. Impostazioni locali (`.claude/settings.local.json`)
2. Impostazioni del progetto (`.claude/settings.json`)
3. Impostazioni dell'utente (`~/.claude/settings.json`)

Le opzioni programmatiche (come `agents`, `allowed_tools`) sostituiscono sempre le impostazioni del filesystem.

### `AgentDefinition`

Configurazione per un subagente definito programmaticamente.

```python
@dataclass
class AgentDefinition:
    description: str
    prompt: str
    tools: list[str] | None = None
    model: Literal["sonnet", "opus", "haiku", "inherit"] | None = None
```

| Campo         | Obbligatorio | Descrizione                                                    |
| :------------ | :------- | :------------------------------------------------------------- |
| `description` | Sì      | Descrizione in linguaggio naturale di quando usare questo agente         |
| `tools`       | No       | Array di nomi degli strumenti consentiti. Se omesso, eredita tutti gli strumenti    |
| `prompt`      | Sì      | Il prompt di sistema dell'agente                                      |
| `model`       | No       | Override del modello per questo agente. Se omesso, usa il modello principale |

### `PermissionMode`

Modalità di permesso per controllare l'esecuzione dello strumento.

```python
PermissionMode = Literal[
    "default",           # Standard permission behavior
    "acceptEdits",       # Auto-accept file edits
    "plan",              # Planning mode - no execution
    "bypassPermissions"  # Bypass all permission checks (use with caution)
]
```

### `McpSdkServerConfig`

Configurazione per i server MCP SDK creati con `create_sdk_mcp_server()`.

```python
class McpSdkServerConfig(TypedDict):
    type: Literal["sdk"]
    name: str
    instance: Any  # MCP Server instance
```

### `McpServerConfig`

Tipo di unione per le configurazioni del server MCP.

```python
McpServerConfig = McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig
```

#### `McpStdioServerConfig`

```python
class McpStdioServerConfig(TypedDict):
    type: NotRequired[Literal["stdio"]]  # Optional for backwards compatibility
    command: str
    args: NotRequired[list[str]]
    env: NotRequired[dict[str, str]]
```

#### `McpSSEServerConfig`

```python
class McpSSEServerConfig(TypedDict):
    type: Literal["sse"]
    url: str
    headers: NotRequired[dict[str, str]]
```

#### `McpHttpServerConfig`

```python
class McpHttpServerConfig(TypedDict):
    type: Literal["http"]
    url: str
    headers: NotRequired[dict[str, str]]
```

### `SdkPluginConfig`

Configurazione per il caricamento dei plugin nell'SDK.

```python
class SdkPluginConfig(TypedDict):
    type: Literal["local"]
    path: str
```

| Campo | Tipo | Descrizione |
|:------|:-----|:------------|
| `type` | `Literal["local"]` | Deve essere `"local"` (attualmente sono supportati solo plugin locali) |
| `path` | `str` | Percorso assoluto o relativo alla directory del plugin |

**Esempio:**
```python
plugins=[
    {"type": "local", "path": "./my-plugin"},
    {"type": "local", "path": "/absolute/path/to/plugin"}
]
```

Per informazioni complete sulla creazione e l'utilizzo dei plugin, vedi [Plugins](/docs/it/agent-sdk/plugins).

## Tipi di messaggio

### `Message`

Tipo di unione di tutti i possibili messaggi.

```python
Message = UserMessage | AssistantMessage | SystemMessage | ResultMessage
```

### `UserMessage`

Messaggio di input dell'utente.

```python
@dataclass
class UserMessage:
    content: str | list[ContentBlock]
```

### `AssistantMessage`

Messaggio di risposta dell'assistente con blocchi di contenuto.

```python
@dataclass
class AssistantMessage:
    content: list[ContentBlock]
    model: str
```

### `SystemMessage`

Messaggio di sistema con metadati.

```python
@dataclass
class SystemMessage:
    subtype: str
    data: dict[str, Any]
```

### `ResultMessage`

Messaggio di risultato finale con informazioni su costo e utilizzo.

```python
@dataclass
class ResultMessage:
    subtype: str
    duration_ms: int
    duration_api_ms: int
    is_error: bool
    num_turns: int
    session_id: str
    total_cost_usd: float | None = None
    usage: dict[str, Any] | None = None
    result: str | None = None
```

## Tipi di blocco di contenuto

### `ContentBlock`

Tipo di unione di tutti i blocchi di contenuto.

```python
ContentBlock = TextBlock | ThinkingBlock | ToolUseBlock | ToolResultBlock
```

### `TextBlock`

Blocco di contenuto di testo.

```python
@dataclass
class TextBlock:
    text: str
```

### `ThinkingBlock`

Blocco di contenuto di pensiero (per modelli con capacità di pensiero).

```python
@dataclass
class ThinkingBlock:
    thinking: str
    signature: str
```

### `ToolUseBlock`

Blocco di richiesta di utilizzo dello strumento.

```python
@dataclass
class ToolUseBlock:
    id: str
    name: str
    input: dict[str, Any]
```

### `ToolResultBlock`

Blocco di risultato dell'esecuzione dello strumento.

```python
@dataclass
class ToolResultBlock:
    tool_use_id: str
    content: str | list[dict[str, Any]] | None = None
    is_error: bool | None = None
```

## Tipi di errore

### `ClaudeSDKError`

Classe di eccezione base per tutti gli errori dell'SDK.

```python
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

### `CLINotFoundError`

Generato quando Claude Code CLI non è installato o non trovato.

```python
class CLINotFoundError(CLIConnectionError):
    def __init__(self, message: str = "Claude Code not found", cli_path: str | None = None):
        """
        Args:
            message: Error message (default: "Claude Code not found")
            cli_path: Optional path to the CLI that was not found
        """
```

### `CLIConnectionError`

Generato quando la connessione a Claude Code non riesce.

```python
class CLIConnectionError(ClaudeSDKError):
    """Failed to connect to Claude Code."""
```

### `ProcessError`

Generato quando il processo Claude Code non riesce.

```python
class ProcessError(ClaudeSDKError):
    def __init__(self, message: str, exit_code: int | None = None, stderr: str | None = None):
        self.exit_code = exit_code
        self.stderr = stderr
```

### `CLIJSONDecodeError`

Generato quando l'analisi JSON non riesce.

```python
class CLIJSONDecodeError(ClaudeSDKError):
    def __init__(self, line: str, original_error: Exception):
        """
        Args:
            line: The line that failed to parse
            original_error: The original JSON decode exception
        """
        self.line = line
        self.original_error = original_error
```

## Tipi di hook

Per una guida completa sull'utilizzo degli hook con esempi e modelli comuni, consulta la [guida degli hook](/docs/it/agent-sdk/hooks).

### `HookEvent`

Tipi di evento hook supportati. Si noti che a causa delle limitazioni di configurazione, Python SDK non supporta gli hook SessionStart, SessionEnd e Notification.

```python
HookEvent = Literal[
    "PreToolUse",      # Called before tool execution
    "PostToolUse",     # Called after tool execution
    "UserPromptSubmit", # Called when user submits a prompt
    "Stop",            # Called when stopping execution
    "SubagentStop",    # Called when a subagent stops
    "PreCompact"       # Called before message compaction
]
```

### `HookCallback`

Definizione del tipo per le funzioni di callback degli hook.

```python
HookCallback = Callable[
    [dict[str, Any], str | None, HookContext],
    Awaitable[dict[str, Any]]
]
```

Parametri:

- `input_data`: Dati di input specifici dell'hook (vedi [guida degli hook](/docs/it/agent-sdk/hooks#input-data))
- `tool_use_id`: Identificatore facoltativo di utilizzo dello strumento (per gli hook correlati agli strumenti)
- `context`: Contesto dell'hook con informazioni aggiuntive

Restituisce un dizionario che può contenere:

- `decision`: `"block"` per bloccare l'azione
- `systemMessage`: Messaggio di sistema da aggiungere alla trascrizione
- `hookSpecificOutput`: Dati di output specifici dell'hook

### `HookContext`

Informazioni di contesto passate ai callback degli hook.

```python
@dataclass
class HookContext:
    signal: Any | None = None  # Future: abort signal support
```

### `HookMatcher`

Configurazione per l'abbinamento degli hook a eventi o strumenti specifici.

```python
@dataclass
class HookMatcher:
    matcher: str | None = None        # Tool name or pattern to match (e.g., "Bash", "Write|Edit")
    hooks: list[HookCallback] = field(default_factory=list)  # List of callbacks to execute
    timeout: float | None = None        # Timeout in seconds for all hooks in this matcher (default: 60)
```

### Esempio di utilizzo degli hook

Questo esempio registra due hook: uno che blocca i comandi bash pericolosi come `rm -rf /`, e un altro che registra tutti gli utilizzi degli strumenti per il controllo. L'hook di sicurezza viene eseguito solo sui comandi Bash (tramite `matcher`), mentre l'hook di registrazione viene eseguito su tutti gli strumenti.

```python
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, HookContext
from typing import Any

async def validate_bash_command(
    input_data: dict[str, Any],
    tool_use_id: str | None,
    context: HookContext
) -> dict[str, Any]:
    """Validate and potentially block dangerous bash commands."""
    if input_data['tool_name'] == 'Bash':
        command = input_data['tool_input'].get('command', '')
        if 'rm -rf /' in command:
            return {
                'hookSpecificOutput': {
                    'hookEventName': 'PreToolUse',
                    'permissionDecision': 'deny',
                    'permissionDecisionReason': 'Dangerous command blocked'
                }
            }
    return {}

async def log_tool_use(
    input_data: dict[str, Any],
    tool_use_id: str | None,
    context: HookContext
) -> dict[str, Any]:
    """Log all tool usage for auditing."""
    print(f"Tool used: {input_data.get('tool_name')}")
    return {}

options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            HookMatcher(matcher='Bash', hooks=[validate_bash_command], timeout=120),  # 2 min for validation
            HookMatcher(hooks=[log_tool_use])  # Applies to all tools (default 60s timeout)
        ],
        'PostToolUse': [
            HookMatcher(hooks=[log_tool_use])
        ]
    }
)

async for message in query(
    prompt="Analyze this codebase",
    options=options
):
    print(message)
```

## Tipi di input/output dello strumento

Documentazione degli schemi di input/output per tutti gli strumenti Claude Code integrati. Sebbene Python SDK non esporti questi come tipi, rappresentano la struttura degli input e output degli strumenti nei messaggi.

### Task

**Nome dello strumento:** `Task`

**Input:**

```python
{
    "description": str,      # A short (3-5 word) description of the task
    "prompt": str,           # The task for the agent to perform
    "subagent_type": str     # The type of specialized agent to use
}
```

**Output:**

```python
{
    "result": str,                    # Final result from the subagent
    "usage": dict | None,             # Token usage statistics
    "total_cost_usd": float | None,  # Total cost in USD
    "duration_ms": int | None         # Execution duration in milliseconds
}
```

### Bash

**Nome dello strumento:** `Bash`

**Input:**

```python
{
    "command": str,                  # The command to execute
    "timeout": int | None,           # Optional timeout in milliseconds (max 600000)
    "description": str | None,       # Clear, concise description (5-10 words)
    "run_in_background": bool | None # Set to true to run in background
}
```

**Output:**

```python
{
    "output": str,              # Combined stdout and stderr output
    "exitCode": int,            # Exit code of the command
    "killed": bool | None,      # Whether command was killed due to timeout
    "shellId": str | None       # Shell ID for background processes
}
```

### Edit

**Nome dello strumento:** `Edit`

**Input:**

```python
{
    "file_path": str,           # The absolute path to the file to modify
    "old_string": str,          # The text to replace
    "new_string": str,          # The text to replace it with
    "replace_all": bool | None  # Replace all occurrences (default False)
}
```

**Output:**

```python
{
    "message": str,      # Confirmation message
    "replacements": int, # Number of replacements made
    "file_path": str     # File path that was edited
}
```

### Read

**Nome dello strumento:** `Read`

**Input:**

```python
{
    "file_path": str,       # The absolute path to the file to read
    "offset": int | None,   # The line number to start reading from
    "limit": int | None     # The number of lines to read
}
```

**Output (file di testo):**

```python
{
    "content": str,         # File contents with line numbers
    "total_lines": int,     # Total number of lines in file
    "lines_returned": int   # Lines actually returned
}
```

**Output (immagini):**

```python
{
    "image": str,       # Base64 encoded image data
    "mime_type": str,   # Image MIME type
    "file_size": int    # File size in bytes
}
```

### Write

**Nome dello strumento:** `Write`

**Input:**

```python
{
    "file_path": str,  # The absolute path to the file to write
    "content": str     # The content to write to the file
}
```

**Output:**

```python
{
    "message": str,        # Success message
    "bytes_written": int,  # Number of bytes written
    "file_path": str       # File path that was written
}
```

### Glob

**Nome dello strumento:** `Glob`

**Input:**

```python
{
    "pattern": str,       # The glob pattern to match files against
    "path": str | None    # The directory to search in (defaults to cwd)
}
```

**Output:**

```python
{
    "matches": list[str],  # Array of matching file paths
    "count": int,          # Number of matches found
    "search_path": str     # Search directory used
}
```

### Grep

**Nome dello strumento:** `Grep`

**Input:**

```python
{
    "pattern": str,                    # The regular expression pattern
    "path": str | None,                # File or directory to search in
    "glob": str | None,                # Glob pattern to filter files
    "type": str | None,                # File type to search
    "output_mode": str | None,         # "content", "files_with_matches", or "count"
    "-i": bool | None,                 # Case insensitive search
    "-n": bool | None,                 # Show line numbers
    "-B": int | None,                  # Lines to show before each match
    "-A": int | None,                  # Lines to show after each match
    "-C": int | None,                  # Lines to show before and after
    "head_limit": int | None,          # Limit output to first N lines/entries
    "multiline": bool | None           # Enable multiline mode
}
```

**Output (modalità content):**

```python
{
    "matches": [
        {
            "file": str,
            "line_number": int | None,
            "line": str,
            "before_context": list[str] | None,
            "after_context": list[str] | None
        }
    ],
    "total_matches": int
}
```

**Output (modalità files_with_matches):**

```python
{
    "files": list[str],  # Files containing matches
    "count": int         # Number of files with matches
}
```

### NotebookEdit

**Nome dello strumento:** `NotebookEdit`

**Input:**

```python
{
    "notebook_path": str,                     # Absolute path to the Jupyter notebook
    "cell_id": str | None,                    # The ID of the cell to edit
    "new_source": str,                        # The new source for the cell
    "cell_type": "code" | "markdown" | None,  # The type of the cell
    "edit_mode": "replace" | "insert" | "delete" | None  # Edit operation type
}
```

**Output:**

```python
{
    "message": str,                              # Success message
    "edit_type": "replaced" | "inserted" | "deleted",  # Type of edit performed
    "cell_id": str | None,                       # Cell ID that was affected
    "total_cells": int                           # Total cells in notebook after edit
}
```

### WebFetch

**Nome dello strumento:** `WebFetch`

**Input:**

```python
{
    "url": str,     # The URL to fetch content from
    "prompt": str   # The prompt to run on the fetched content
}
```

**Output:**

```python
{
    "response": str,           # AI model's response to the prompt
    "url": str,                # URL that was fetched
    "final_url": str | None,   # Final URL after redirects
    "status_code": int | None  # HTTP status code
}
```

### WebSearch

**Nome dello strumento:** `WebSearch`

**Input:**

```python
{
    "query": str,                        # The search query to use
    "allowed_domains": list[str] | None, # Only include results from these domains
    "blocked_domains": list[str] | None  # Never include results from these domains
}
```

**Output:**

```python
{
    "results": [
        {
            "title": str,
            "url": str,
            "snippet": str,
            "metadata": dict | None
        }
    ],
    "total_results": int,
    "query": str
}
```

### TodoWrite

**Nome dello strumento:** `TodoWrite`

**Input:**

```python
{
    "todos": [
        {
            "content": str,                              # The task description
            "status": "pending" | "in_progress" | "completed",  # Task status
            "activeForm": str                            # Active form of the description
        }
    ]
}
```

**Output:**

```python
{
    "message": str,  # Success message
    "stats": {
        "total": int,
        "pending": int,
        "in_progress": int,
        "completed": int
    }
}
```

### BashOutput

**Nome dello strumento:** `BashOutput`

**Input:**

```python
{
    "bash_id": str,       # The ID of the background shell
    "filter": str | None  # Optional regex to filter output lines
}
```

**Output:**

```python
{
    "output": str,                                      # New output since last check
    "status": "running" | "completed" | "failed",       # Current shell status
    "exitCode": int | None                              # Exit code when completed
}
```

### KillBash

**Nome dello strumento:** `KillBash`

**Input:**

```python
{
    "shell_id": str  # The ID of the background shell to kill
}
```

**Output:**

```python
{
    "message": str,  # Success message
    "shell_id": str  # ID of the killed shell
}
```

### ExitPlanMode

**Nome dello strumento:** `ExitPlanMode`

**Input:**

```python
{
    "plan": str  # The plan to run by the user for approval
}
```

**Output:**

```python
{
    "message": str,          # Confirmation message
    "approved": bool | None  # Whether user approved the plan
}
```

### ListMcpResources

**Nome dello strumento:** `ListMcpResources`

**Input:**

```python
{
    "server": str | None  # Optional server name to filter resources by
}
```

**Output:**

```python
{
    "resources": [
        {
            "uri": str,
            "name": str,
            "description": str | None,
            "mimeType": str | None,
            "server": str
        }
    ],
    "total": int
}
```

### ReadMcpResource

**Nome dello strumento:** `ReadMcpResource`

**Input:**

```python
{
    "server": str,  # The MCP server name
    "uri": str      # The resource URI to read
}
```

**Output:**

```python
{
    "contents": [
        {
            "uri": str,
            "mimeType": str | None,
            "text": str | None,
            "blob": str | None
        }
    ],
    "server": str
}
```

## Funzionalità avanzate con ClaudeSDKClient

### Creazione di un'interfaccia di conversazione continua

```python
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, AssistantMessage, TextBlock
import asyncio

class ConversationSession:
    """Maintains a single conversation session with Claude."""

    def __init__(self, options: ClaudeAgentOptions = None):
        self.client = ClaudeSDKClient(options)
        self.turn_count = 0

    async def start(self):
        await self.client.connect()
        print("Starting conversation session. Claude will remember context.")
        print("Commands: 'exit' to quit, 'interrupt' to stop current task, 'new' for new session")

        while True:
            user_input = input(f"\n[Turn {self.turn_count + 1}] You: ")

            if user_input.lower() == 'exit':
                break
            elif user_input.lower() == 'interrupt':
                await self.client.interrupt()
                print("Task interrupted!")
                continue
            elif user_input.lower() == 'new':
                # Disconnect and reconnect for a fresh session
                await self.client.disconnect()
                await self.client.connect()
                self.turn_count = 0
                print("Started new conversation session (previous context cleared)")
                continue

            # Send message - Claude remembers all previous messages in this session
            await self.client.query(user_input)
            self.turn_count += 1

            # Process response
            print(f"[Turn {self.turn_count}] Claude: ", end="")
            async for message in self.client.receive_response():
                if isinstance(message, AssistantMessage):
                    for block in message.content:
                        if isinstance(block, TextBlock):
                            print(block.text, end="")
            print()  # New line after response

        await self.client.disconnect()
        print(f"Conversation ended after {self.turn_count} turns.")

async def main():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Bash"],
        permission_mode="acceptEdits"
    )
    session = ConversationSession(options)
    await session.start()

# Example conversation:
# Turn 1 - You: "Create a file called hello.py"
# Turn 1 - Claude: "I'll create a hello.py file for you..."
# Turn 2 - You: "What's in that file?"
# Turn 2 - Claude: "The hello.py file I just created contains..." (remembers!)
# Turn 3 - You: "Add a main function to it"
# Turn 3 - Claude: "I'll add a main function to hello.py..." (knows which file!)

asyncio.run(main())
```

### Utilizzo degli hook per la modifica del comportamento

```python
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    HookMatcher,
    HookContext
)
import asyncio
from typing import Any

async def pre_tool_logger(
    input_data: dict[str, Any],
    tool_use_id: str | None,
    context: HookContext
) -> dict[str, Any]:
    """Log all tool usage before execution."""
    tool_name = input_data.get('tool_name', 'unknown')
    print(f"[PRE-TOOL] About to use: {tool_name}")

    # You can modify or block the tool execution here
    if tool_name == "Bash" and "rm -rf" in str(input_data.get('tool_input', {})):
        return {
            'hookSpecificOutput': {
                'hookEventName': 'PreToolUse',
                'permissionDecision': 'deny',
                'permissionDecisionReason': 'Dangerous command blocked'
            }
        }
    return {}

async def post_tool_logger(
    input_data: dict[str, Any],
    tool_use_id: str | None,
    context: HookContext
) -> dict[str, Any]:
    """Log results after tool execution."""
    tool_name = input_data.get('tool_name', 'unknown')
    print(f"[POST-TOOL] Completed: {tool_name}")
    return {}

async def user_prompt_modifier(
    input_data: dict[str, Any],
    tool_use_id: str | None,
    context: HookContext
) -> dict[str, Any]:
    """Add context to user prompts."""
    original_prompt = input_data.get('prompt', '')

    # Add timestamp to all prompts
    from datetime import datetime
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    return {
        'hookSpecificOutput': {
            'hookEventName': 'UserPromptSubmit',
            'updatedPrompt': f"[{timestamp}] {original_prompt}"
        }
    }

async def main():
    options = ClaudeAgentOptions(
        hooks={
            'PreToolUse': [
                HookMatcher(hooks=[pre_tool_logger]),
                HookMatcher(matcher='Bash', hooks=[pre_tool_logger])
            ],
            'PostToolUse': [
                HookMatcher(hooks=[post_tool_logger])
            ],
            'UserPromptSubmit': [
                HookMatcher(hooks=[user_prompt_modifier])
            ]
        },
        allowed_tools=["Read", "Write", "Bash"]
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query("List files in current directory")

        async for message in client.receive_response():
            # Hooks will automatically log tool usage
            pass

asyncio.run(main())
```

### Monitoraggio della progressione in tempo reale

```python
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    ToolUseBlock,
    ToolResultBlock,
    TextBlock
)
import asyncio

async def monitor_progress():
    options = ClaudeAgentOptions(
        allowed_tools=["Write", "Bash"],
        permission_mode="acceptEdits"
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query(
            "Create 5 Python files with different sorting algorithms"
        )

        # Monitor progress in real-time
        files_created = []
        async for message in client.receive_messages():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, ToolUseBlock):
                        if block.name == "Write":
                            file_path = block.input.get("file_path", "")
                            print(f"🔨 Creating: {file_path}")
                    elif isinstance(block, ToolResultBlock):
                        print(f"✅ Completed tool execution")
                    elif isinstance(block, TextBlock):
                        print(f"💭 Claude says: {block.text[:100]}...")

            # Check if we've received the final result
            if hasattr(message, 'subtype') and message.subtype in ['success', 'error']:
                print(f"\n🎯 Task completed!")
                break

asyncio.run(monitor_progress())
```

## Utilizzo di esempio

### Operazioni di file di base (utilizzando query)

```python
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock
import asyncio

async def create_project():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Bash"],
        permission_mode='acceptEdits',
        cwd="/home/user/project"
    )

    async for message in query(
        prompt="Create a Python project structure with setup.py",
        options=options
    ):
        if isinstance(message, AssistantMessage):
            for block in message.content:
                if isinstance(block, ToolUseBlock):
                    print(f"Using tool: {block.name}")

asyncio.run(create_project())
```

### Gestione degli errori

```python
from claude_agent_sdk import (
    query,
    CLINotFoundError,
    ProcessError,
    CLIJSONDecodeError
)

try:
    async for message in query(prompt="Hello"):
        print(message)
except CLINotFoundError:
    print("Please install Claude Code: npm install -g @anthropic-ai/claude-code")
except ProcessError as e:
    print(f"Process failed with exit code: {e.exit_code}")
except CLIJSONDecodeError as e:
    print(f"Failed to parse response: {e}")
```

### Modalità streaming con client

```python
from claude_agent_sdk import ClaudeSDKClient
import asyncio

async def interactive_session():
    async with ClaudeSDKClient() as client:
        # Send initial message
        await client.query("What's the weather like?")

        # Process responses
        async for msg in client.receive_response():
            print(msg)

        # Send follow-up
        await client.query("Tell me more about that")

        # Process follow-up response
        async for msg in client.receive_response():
            print(msg)

asyncio.run(interactive_session())
```

### Utilizzo di strumenti personalizzati con ClaudeSDKClient

```python
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    tool,
    create_sdk_mcp_server,
    AssistantMessage,
    TextBlock
)
import asyncio
from typing import Any

# Define custom tools with @tool decorator
@tool("calculate", "Perform mathematical calculations", {"expression": str})
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    try:
        result = eval(args["expression"], {"__builtins__": {}})
        return {
            "content": [{
                "type": "text",
                "text": f"Result: {result}"
            }]
        }
    except Exception as e:
        return {
            "content": [{
                "type": "text",
                "text": f"Error: {str(e)}"
            }],
            "is_error": True
        }

@tool("get_time", "Get current time", {})
async def get_time(args: dict[str, Any]) -> dict[str, Any]:
    from datetime import datetime
    current_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    return {
        "content": [{
            "type": "text",
            "text": f"Current time: {current_time}"
        }]
    }

async def main():
    # Create SDK MCP server with custom tools
    my_server = create_sdk_mcp_server(
        name="utilities",
        version="1.0.0",
        tools=[calculate, get_time]
    )

    # Configure options with the server
    options = ClaudeAgentOptions(
        mcp_servers={"utils": my_server},
        allowed_tools=[
            "mcp__utils__calculate",
            "mcp__utils__get_time"
        ]
    )

    # Use ClaudeSDKClient for interactive tool usage
    async with ClaudeSDKClient(options=options) as client:
        await client.query("What's 123 * 456?")

        # Process calculation response
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Calculation: {block.text}")

        # Follow up with time query
        await client.query("What time is it now?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Time: {block.text}")

asyncio.run(main())
```

## Configurazione della sandbox

### `SandboxSettings`

Configurazione per il comportamento della sandbox. Utilizzare questa opzione per abilitare il sandboxing dei comandi e configurare le restrizioni di rete a livello di programmazione.

```python
class SandboxSettings(TypedDict, total=False):
    enabled: bool
    autoAllowBashIfSandboxed: bool
    excludedCommands: list[str]
    allowUnsandboxedCommands: bool
    network: SandboxNetworkConfig
    ignoreViolations: SandboxIgnoreViolations
    enableWeakerNestedSandbox: bool
```

| Proprietà | Tipo | Predefinito | Descrizione |
| :------- | :--- | :------ | :---------- |
| `enabled` | `bool` | `False` | Abilita la modalità sandbox per l'esecuzione dei comandi |
| `autoAllowBashIfSandboxed` | `bool` | `False` | Approva automaticamente i comandi bash quando la sandbox è abilitata |
| `excludedCommands` | `list[str]` | `[]` | Comandi che bypassano sempre le restrizioni della sandbox (ad es. `["docker"]`). Questi vengono eseguiti senza sandbox automaticamente senza il coinvolgimento del modello |
| `allowUnsandboxedCommands` | `bool` | `False` | Consenti al modello di richiedere l'esecuzione di comandi al di fuori della sandbox. Quando `True`, il modello può impostare `dangerouslyDisableSandbox` nell'input dello strumento, che ricade nel [sistema di autorizzazioni](#permissions-fallback-for-unsandboxed-commands) |
| `network` | [`SandboxNetworkConfig`](#sandboxnetworkconfig) | `None` | Configurazione della sandbox specifica della rete |
| `ignoreViolations` | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None` | Configura quali violazioni della sandbox ignorare |
| `enableWeakerNestedSandbox` | `bool` | `False` | Abilita una sandbox nidificata più debole per la compatibilità |

<Note>
**Le restrizioni di accesso al filesystem e alla rete** NON sono configurate tramite le impostazioni della sandbox. Invece, sono derivate dalle [regole di autorizzazione](https://code.claude.com/docs/en/settings#permission-settings):

- **Restrizioni di lettura del filesystem**: Regole di negazione della lettura
- **Restrizioni di scrittura del filesystem**: Regole di consentimento/negazione della modifica
- **Restrizioni di rete**: Regole di consentimento/negazione di WebFetch

Utilizzare le impostazioni della sandbox per il sandboxing dell'esecuzione dei comandi e le regole di autorizzazione per il controllo dell'accesso al filesystem e alla rete.
</Note>

#### Esempio di utilizzo

```python
from claude_agent_sdk import query, ClaudeAgentOptions, SandboxSettings

sandbox_settings: SandboxSettings = {
    "enabled": True,
    "autoAllowBashIfSandboxed": True,
    "excludedCommands": ["docker"],
    "network": {
        "allowLocalBinding": True,
        "allowUnixSockets": ["/var/run/docker.sock"]
    }
}

async for message in query(
    prompt="Build and test my project",
    options=ClaudeAgentOptions(sandbox=sandbox_settings)
):
    print(message)
```

### `SandboxNetworkConfig`

Configurazione specifica della rete per la modalità sandbox.

```python
class SandboxNetworkConfig(TypedDict, total=False):
    allowLocalBinding: bool
    allowUnixSockets: list[str]
    allowAllUnixSockets: bool
    httpProxyPort: int
    socksProxyPort: int
```

| Proprietà | Tipo | Predefinito | Descrizione |
| :------- | :--- | :------ | :---------- |
| `allowLocalBinding` | `bool` | `False` | Consenti ai processi di associarsi alle porte locali (ad es. per i server di sviluppo) |
| `allowUnixSockets` | `list[str]` | `[]` | Percorsi dei socket Unix a cui i processi possono accedere (ad es. socket Docker) |
| `allowAllUnixSockets` | `bool` | `False` | Consenti l'accesso a tutti i socket Unix |
| `httpProxyPort` | `int` | `None` | Porta del proxy HTTP per le richieste di rete |
| `socksProxyPort` | `int` | `None` | Porta del proxy SOCKS per le richieste di rete |

### `SandboxIgnoreViolations`

Configurazione per ignorare violazioni specifiche della sandbox.

```python
class SandboxIgnoreViolations(TypedDict, total=False):
    file: list[str]
    network: list[str]
```

| Proprietà | Tipo | Predefinito | Descrizione |
| :------- | :--- | :------ | :---------- |
| `file` | `list[str]` | `[]` | Modelli di percorso file per ignorare le violazioni |
| `network` | `list[str]` | `[]` | Modelli di rete per ignorare le violazioni |

### Fallback delle autorizzazioni per i comandi senza sandbox

Quando `allowUnsandboxedCommands` è abilitato, il modello può richiedere di eseguire comandi al di fuori della sandbox impostando `dangerouslyDisableSandbox: True` nell'input dello strumento. Queste richieste ricadono nel sistema di autorizzazioni esistente, il che significa che il tuo gestore `can_use_tool` verrà invocato, permettendoti di implementare una logica di autorizzazione personalizzata.

<Note>
**`excludedCommands` vs `allowUnsandboxedCommands`:**
- `excludedCommands`: Un elenco statico di comandi che bypassano sempre la sandbox automaticamente (ad es. `["docker"]`). Il modello non ha alcun controllo su questo.
- `allowUnsandboxedCommands`: Consente al modello di decidere in fase di esecuzione se richiedere l'esecuzione senza sandbox impostando `dangerouslyDisableSandbox: True` nell'input dello strumento.
</Note>

```python
from claude_agent_sdk import query, ClaudeAgentOptions

async def can_use_tool(tool: str, input: dict) -> bool:
    # Check if the model is requesting to bypass the sandbox
    if tool == "Bash" and input.get("dangerouslyDisableSandbox"):
        # The model wants to run this command outside the sandbox
        print(f"Unsandboxed command requested: {input.get('command')}")

        # Return True to allow, False to deny
        return is_command_authorized(input.get("command"))
    return True

async def main():
    async for message in query(
        prompt="Deploy my application",
        options=ClaudeAgentOptions(
            sandbox={
                "enabled": True,
                "allowUnsandboxedCommands": True  # Model can request unsandboxed execution
            },
            permission_mode="default",
            can_use_tool=can_use_tool
        )
    ):
        print(message)
```

Questo modello ti consente di:

- **Controllare le richieste del modello**: Registra quando il modello richiede l'esecuzione senza sandbox
- **Implementare elenchi di autorizzazione**: Consenti solo a comandi specifici di essere eseguiti senza sandbox
- **Aggiungere flussi di lavoro di approvazione**: Richiedi un'autorizzazione esplicita per le operazioni privilegiate

<Warning>
I comandi in esecuzione con `dangerouslyDisableSandbox: True` hanno accesso completo al sistema. Assicurati che il tuo gestore `can_use_tool` convalidi queste richieste con attenzione.
</Warning>

## Vedi anche

- [Guida Python SDK](/docs/it/agent-sdk/python) - Tutorial ed esempi
- [Panoramica dell'SDK](/docs/it/agent-sdk/overview) - Concetti generali dell'SDK
- [Riferimento TypeScript SDK](/docs/it/agent-sdk/typescript) - Documentazione TypeScript SDK
- [Riferimento CLI](https://code.claude.com/docs/en/cli-reference) - Interfaccia della riga di comando
- [Flussi di lavoro comuni](https://code.claude.com/docs/en/common-workflows) - Guide passo dopo passo