# Connettore MCP

Connetti a server MCP remoti direttamente dall'API Messages senza un client MCP separato

---

Il connettore Model Context Protocol (MCP) di Claude ti consente di connetterti a server MCP remoti direttamente dall'API Messages senza un client MCP separato.

<Note>
  **Versione attuale**: Questa funzione richiede l'intestazione beta: `"anthropic-beta": "mcp-client-2025-11-20"`

  La versione precedente (`mcp-client-2025-04-04`) è deprecata. Consulta la [documentazione della versione deprecata](#deprecated-version-mcp-client-2025-04-04) di seguito.
</Note>

## Caratteristiche principali

- **Integrazione API diretta**: Connettiti a server MCP senza implementare un client MCP
- **Supporto per le chiamate di strumenti**: Accedi agli strumenti MCP tramite l'API Messages
- **Configurazione flessibile degli strumenti**: Abilita tutti gli strumenti, consenti strumenti specifici o blocca strumenti indesiderati
- **Configurazione per strumento**: Configura singoli strumenti con impostazioni personalizzate
- **Autenticazione OAuth**: Supporto per token Bearer OAuth per server autenticati
- **Server multipli**: Connettiti a più server MCP in una singola richiesta

## Limitazioni

- Del set di funzionalità della [specifica MCP](https://modelcontextprotocol.io/introduction#explore-mcp), sono attualmente supportate solo le [chiamate di strumenti](https://modelcontextprotocol.io/docs/concepts/tools).
- Il server deve essere esposto pubblicamente tramite HTTP (supporta sia i trasporti Streamable HTTP che SSE). I server STDIO locali non possono essere connessi direttamente.
- Il connettore MCP non è attualmente supportato su Amazon Bedrock e Google Vertex.

## Utilizzo del connettore MCP nell'API Messages

Il connettore MCP utilizza due componenti:

1. **Definizione del server MCP** (array `mcp_servers`): Definisce i dettagli della connessione al server (URL, autenticazione)
2. **Set di strumenti MCP** (array `tools`): Configura quali strumenti abilitare e come configurarli

### Esempio di base

Questo esempio abilita tutti gli strumenti da un server MCP con configurazione predefinita:

<CodeGroup>
```bash Shell
curl https://api.anthropic.com/v1/messages \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "anthropic-beta: mcp-client-2025-11-20" \
  -d '{
    "model": "claude-sonnet-4-5",
    "max_tokens": 1000,
    "messages": [{"role": "user", "content": "What tools do you have available?"}],
    "mcp_servers": [
      {
        "type": "url",
        "url": "https://example-server.modelcontextprotocol.io/sse",
        "name": "example-mcp",
        "authorization_token": "YOUR_TOKEN"
      }
    ],
    "tools": [
      {
        "type": "mcp_toolset",
        "mcp_server_name": "example-mcp"
      }
    ]
  }'
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

const response = await anthropic.beta.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  messages: [
    {
      role: "user",
      content: "What tools do you have available?",
    },
  ],
  mcp_servers: [
    {
      type: "url",
      url: "https://example-server.modelcontextprotocol.io/sse",
      name: "example-mcp",
      authorization_token: "YOUR_TOKEN",
    },
  ],
  tools: [
    {
      type: "mcp_toolset",
      mcp_server_name: "example-mcp",
    },
  ],
  betas: ["mcp-client-2025-11-20"],
});
```

```python Python
import anthropic

client = anthropic.Anthropic()

response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    messages=[{
        "role": "user",
        "content": "What tools do you have available?"
    }],
    mcp_servers=[{
        "type": "url",
        "url": "https://mcp.example.com/sse",
        "name": "example-mcp",
        "authorization_token": "YOUR_TOKEN"
    }],
    tools=[{
        "type": "mcp_toolset",
        "mcp_server_name": "example-mcp"
    }],
    betas=["mcp-client-2025-11-20"]
)
```
</CodeGroup>

## Configurazione del server MCP

Ogni server MCP nell'array `mcp_servers` definisce i dettagli della connessione:

```json
{
  "type": "url",
  "url": "https://example-server.modelcontextprotocol.io/sse",
  "name": "example-mcp",
  "authorization_token": "YOUR_TOKEN"
}
```

### Descrizioni dei campi

| Proprietà | Tipo | Obbligatorio | Descrizione |
|----------|------|----------|-------------|
| `type` | string | Sì | Attualmente è supportato solo "url" |
| `url` | string | Sì | L'URL del server MCP. Deve iniziare con https:// |
| `name` | string | Sì | Un identificatore univoco per questo server MCP. Deve essere referenziato da esattamente un MCPToolset nell'array `tools`. |
| `authorization_token` | string | No | Token di autorizzazione OAuth se richiesto dal server MCP. Consulta la [specifica MCP](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization). |

## Configurazione del set di strumenti MCP

MCPToolset si trova nell'array `tools` e configura quali strumenti dal server MCP sono abilitati e come devono essere configurati.

### Struttura di base

```json
{
  "type": "mcp_toolset",
  "mcp_server_name": "example-mcp",
  "default_config": {
    "enabled": true,
    "defer_loading": false
  },
  "configs": {
    "specific_tool_name": {
      "enabled": true,
      "defer_loading": true
    }
  }
}
```

### Descrizioni dei campi

| Proprietà | Tipo | Obbligatorio | Descrizione |
|----------|------|----------|-------------|
| `type` | string | Sì | Deve essere "mcp_toolset" |
| `mcp_server_name` | string | Sì | Deve corrispondere a un nome di server definito nell'array `mcp_servers` |
| `default_config` | object | No | Configurazione predefinita applicata a tutti gli strumenti in questo set. Le configurazioni di singoli strumenti in `configs` sostituiranno questi valori predefiniti. |
| `configs` | object | No | Sostituzioni di configurazione per singolo strumento. Le chiavi sono nomi di strumenti, i valori sono oggetti di configurazione. |
| `cache_control` | object | No | Configurazione del punto di interruzione della cache per questo set di strumenti |

### Opzioni di configurazione dello strumento

Ogni strumento (sia configurato in `default_config` che in `configs`) supporta i seguenti campi:

| Proprietà | Tipo | Predefinito | Descrizione |
|----------|------|---------|-------------|
| `enabled` | boolean | `true` | Se questo strumento è abilitato |
| `defer_loading` | boolean | `false` | Se true, la descrizione dello strumento non viene inviata al modello inizialmente. Utilizzato con [Tool Search Tool](/docs/it/agents-and-tools/tool-search-tool). |

### Unione della configurazione

I valori di configurazione si uniscono con questa precedenza (da più alta a più bassa):

1. Impostazioni specifiche dello strumento in `configs`
2. `default_config` a livello di set
3. Valori predefiniti del sistema

Esempio:

```json
{
  "type": "mcp_toolset",
  "mcp_server_name": "google-calendar-mcp",
  "default_config": {
    "defer_loading": true
  },
  "configs": {
    "search_events": {
      "enabled": false
    }
  }
}
```

Risultati in:
- `search_events`: `enabled: false` (da configs), `defer_loading: true` (da default_config)
- Tutti gli altri strumenti: `enabled: true` (valore predefinito del sistema), `defer_loading: true` (da default_config)

## Modelli di configurazione comuni

### Abilita tutti gli strumenti con configurazione predefinita

Il modello più semplice - abilita tutti gli strumenti da un server:

```json
{
  "type": "mcp_toolset",
  "mcp_server_name": "google-calendar-mcp",
}
```

### Allowlist - Abilita solo strumenti specifici

Imposta `enabled: false` come predefinito, quindi abilita esplicitamente strumenti specifici:

```json
{
  "type": "mcp_toolset",
  "mcp_server_name": "google-calendar-mcp",
  "default_config": {
    "enabled": false
  },
  "configs": {
    "search_events": {
      "enabled": true
    },
    "create_event": {
      "enabled": true
    }
  }
}
```

### Denylist - Disabilita strumenti specifici

Abilita tutti gli strumenti per impostazione predefinita, quindi disabilita esplicitamente gli strumenti indesiderati:

```json
{
  "type": "mcp_toolset",
  "mcp_server_name": "google-calendar-mcp",
  "configs": {
    "delete_all_events": {
      "enabled": false
    },
    "share_calendar_publicly": {
      "enabled": false
    }
  }
}
```

### Misto - Allowlist con configurazione per singolo strumento

Combina allowlisting con configurazione personalizzata per ogni strumento:

```json
{
  "type": "mcp_toolset",
  "mcp_server_name": "google-calendar-mcp",
  "default_config": {
    "enabled": false,
    "defer_loading": true
  },
  "configs": {
    "search_events": {
      "enabled": true,
      "defer_loading": false
    },
    "list_events": {
      "enabled": true
    }
  }
}
```

In questo esempio:
- `search_events` è abilitato con `defer_loading: false`
- `list_events` è abilitato con `defer_loading: true` (ereditato da default_config)
- Tutti gli altri strumenti sono disabilitati

## Regole di convalida

L'API applica queste regole di convalida:

- **Il server deve esistere**: Il `mcp_server_name` in un MCPToolset deve corrispondere a un server definito nell'array `mcp_servers`
- **Il server deve essere utilizzato**: Ogni server MCP definito in `mcp_servers` deve essere referenziato da esattamente un MCPToolset
- **Toolset univoco per server**: Ogni server MCP può essere referenziato solo da un MCPToolset
- **Nomi di strumenti sconosciuti**: Se un nome di strumento in `configs` non esiste sul server MCP, viene registrato un avviso di backend ma non viene restituito alcun errore (i server MCP possono avere disponibilità di strumenti dinamica)

## Tipi di contenuto della risposta

Quando Claude utilizza strumenti MCP, la risposta includerà due nuovi tipi di blocco di contenuto:

### Blocco di utilizzo dello strumento MCP

```json
{
  "type": "mcp_tool_use",
  "id": "mcptoolu_014Q35RayjACSWkSj4X2yov1",
  "name": "echo",
  "server_name": "example-mcp",
  "input": { "param1": "value1", "param2": "value2" }
}
```

### Blocco di risultato dello strumento MCP

```json
{
  "type": "mcp_tool_result",
  "tool_use_id": "mcptoolu_014Q35RayjACSWkSj4X2yov1",
  "is_error": false,
  "content": [
    {
      "type": "text",
      "text": "Hello"
    }
  ]
}
```

## Server MCP multipli

Puoi connetterti a più server MCP includendo più definizioni di server in `mcp_servers` e un MCPToolset corrispondente per ciascuno nell'array `tools`:

```json
{
  "model": "claude-sonnet-4-5",
  "max_tokens": 1000,
  "messages": [
    {
      "role": "user",
      "content": "Use tools from both mcp-server-1 and mcp-server-2 to complete this task"
    }
  ],
  "mcp_servers": [
    {
      "type": "url",
      "url": "https://mcp.example1.com/sse",
      "name": "mcp-server-1",
      "authorization_token": "TOKEN1"
    },
    {
      "type": "url",
      "url": "https://mcp.example2.com/sse",
      "name": "mcp-server-2",
      "authorization_token": "TOKEN2"
    }
  ],
  "tools": [
    {
      "type": "mcp_toolset",
      "mcp_server_name": "mcp-server-1"
    },
    {
      "type": "mcp_toolset",
      "mcp_server_name": "mcp-server-2",
      "default_config": {
        "defer_loading": true
      }
    }
  ]
}
```

## Autenticazione

Per i server MCP che richiedono l'autenticazione OAuth, dovrai ottenere un token di accesso. Il beta del connettore MCP supporta il passaggio di un parametro `authorization_token` nella definizione del server MCP.
I consumatori di API devono gestire il flusso OAuth e ottenere il token di accesso prima di effettuare la chiamata API, nonché aggiornare il token secondo necessità.

### Ottenere un token di accesso per i test

L'ispettore MCP può guidarti attraverso il processo di ottenimento di un token di accesso a scopo di test.

1. Esegui l'ispettore con il seguente comando. Hai bisogno di Node.js installato sulla tua macchina.

   ```bash
   npx @modelcontextprotocol/inspector
   ```

2. Nella barra laterale a sinistra, per "Tipo di trasporto", seleziona "SSE" o "Streamable HTTP".
3. Inserisci l'URL del server MCP.
4. Nell'area a destra, fai clic sul pulsante "Open Auth Settings" dopo "Need to configure authentication?".
5. Fai clic su "Quick OAuth Flow" e autorizza nella schermata OAuth.
6. Segui i passaggi nella sezione "OAuth Flow Progress" dell'ispettore e fai clic su "Continue" finché non raggiungi "Authentication complete".
7. Copia il valore di `access_token`.
8. Incollalo nel campo `authorization_token` nella configurazione del tuo server MCP.

### Utilizzo del token di accesso

Una volta ottenuto un token di accesso utilizzando uno dei flussi OAuth sopra, puoi utilizzarlo nella configurazione del tuo server MCP:

```json
{
  "mcp_servers": [
    {
      "type": "url",
      "url": "https://example-server.modelcontextprotocol.io/sse",
      "name": "authenticated-server",
      "authorization_token": "YOUR_ACCESS_TOKEN_HERE"
    }
  ]
}
```

Per spiegazioni dettagliate del flusso OAuth, consulta la [sezione Autorizzazione](https://modelcontextprotocol.io/docs/concepts/authentication) nella specifica MCP.

## Guida alla migrazione

Se stai utilizzando l'intestazione beta deprecata `mcp-client-2025-04-04`, segui questa guida per migrare alla nuova versione.

### Modifiche principali

1. **Nuova intestazione beta**: Cambia da `mcp-client-2025-04-04` a `mcp-client-2025-11-20`
2. **Configurazione dello strumento spostata**: La configurazione dello strumento ora si trova nell'array `tools` come oggetti MCPToolset, non nella definizione del server MCP
3. **Configurazione più flessibile**: Il nuovo modello supporta allowlisting, denylisting e configurazione per singolo strumento

### Passaggi di migrazione

**Prima (deprecato):**

```json
{
  "model": "claude-sonnet-4-5",
  "max_tokens": 1000,
  "messages": [...],
  "mcp_servers": [
    {
      "type": "url",
      "url": "https://mcp.example.com/sse",
      "name": "example-mcp",
      "authorization_token": "YOUR_TOKEN",
      "tool_configuration": {
        "enabled": true,
        "allowed_tools": ["tool1", "tool2"]
      }
    }
  ]
}
```

**Dopo (attuale):**

```json
{
  "model": "claude-sonnet-4-5",
  "max_tokens": 1000,
  "messages": [...],
  "mcp_servers": [
    {
      "type": "url",
      "url": "https://mcp.example.com/sse",
      "name": "example-mcp",
      "authorization_token": "YOUR_TOKEN"
    }
  ],
  "tools": [
    {
      "type": "mcp_toolset",
      "mcp_server_name": "example-mcp",
      "default_config": {
        "enabled": false
      },
      "configs": {
        "tool1": {
          "enabled": true
        },
        "tool2": {
          "enabled": true
        }
      }
    }
  ]
}
```

### Modelli di migrazione comuni

| Modello precedente | Nuovo modello |
|-------------|-------------|
| Nessun `tool_configuration` (tutti gli strumenti abilitati) | MCPToolset senza `default_config` o `configs` |
| `tool_configuration.enabled: false` | MCPToolset con `default_config.enabled: false` |
| `tool_configuration.allowed_tools: [...]` | MCPToolset con `default_config.enabled: false` e strumenti specifici abilitati in `configs` |

## Versione deprecata: mcp-client-2025-04-04

<Note type="warning">
  Questa versione è deprecata. Esegui la migrazione a `mcp-client-2025-11-20` utilizzando la [guida alla migrazione](#migration-guide) sopra.
</Note>

La versione precedente del connettore MCP includeva la configurazione dello strumento direttamente nella definizione del server MCP:

```json
{
  "mcp_servers": [
    {
      "type": "url",
      "url": "https://example-server.modelcontextprotocol.io/sse",
      "name": "example-mcp",
      "authorization_token": "YOUR_TOKEN",
      "tool_configuration": {
        "enabled": true,
        "allowed_tools": ["example_tool_1", "example_tool_2"]
      }
    }
  ]
}
```

### Descrizioni dei campi deprecati

| Proprietà | Tipo | Descrizione |
|----------|------|-------------|
| `tool_configuration` | object | **Deprecato**: Utilizza MCPToolset nell'array `tools` |
| `tool_configuration.enabled` | boolean | **Deprecato**: Utilizza `default_config.enabled` in MCPToolset |
| `tool_configuration.allowed_tools` | array | **Deprecato**: Utilizza il modello allowlist con `configs` in MCPToolset |