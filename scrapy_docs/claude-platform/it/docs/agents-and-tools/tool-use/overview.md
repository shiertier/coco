# Utilizzo di strumenti con Claude

Claude è in grado di interagire con strumenti e funzioni, consentendoti di estendere le capacità di Claude per eseguire una varietà più ampia di attività.

---

Claude è in grado di interagire con strumenti e funzioni, consentendoti di estendere le capacità di Claude per eseguire una varietà più ampia di attività.

<Tip>
  Impara tutto ciò che devi sapere per padroneggiare l'utilizzo di strumenti con Claude come parte dei nostri nuovi [corsi](https://anthropic.skilljar.com/)! Per favore
  continua a condividere le tue idee e suggerimenti utilizzando questo
  [modulo](https://forms.gle/BFnYc6iCkWoRzFgk7).
</Tip>

<Tip>
**Garantisci la conformità dello schema con l'utilizzo rigoroso di strumenti**

[Structured Outputs](/docs/it/build-with-claude/structured-outputs) fornisce una validazione dello schema garantita per gli input degli strumenti. Aggiungi `strict: true` alle tue definizioni di strumenti per assicurarti che le chiamate di strumenti di Claude corrispondano sempre esattamente al tuo schema, senza ulteriori mancate corrispondenze di tipo o campi mancanti.

Perfetto per agenti di produzione dove parametri di strumenti non validi causerebbero errori. [Scopri quando utilizzare l'utilizzo rigoroso di strumenti →](/docs/it/build-with-claude/structured-outputs#when-to-use-json-outputs-vs-strict-tool-use)
</Tip>

Ecco un esempio di come fornire strumenti a Claude utilizzando l'API Messages:

<CodeGroup>

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
        "name": "get_weather",
        "description": "Get the current weather in a given location",
        "input_schema": {
          "type": "object",
          "properties": {
            "location": {
              "type": "string",
              "description": "The city and state, e.g. San Francisco, CA"
            }
          },
          "required": ["location"]
        }
      }
    ],
    "messages": [
      {
        "role": "user",
        "content": "What is the weather like in San Francisco?"
      }
    ]
  }'
```

```python Python
import anthropic

client = anthropic.Anthropic()

response = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    tools=[
        {
            "name": "get_weather",
            "description": "Get the current weather in a given location",
            "input_schema": {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA",
                    }
                },
                "required": ["location"],
            },
        }
    ],
    messages=[{"role": "user", "content": "What's the weather like in San Francisco?"}],
)
print(response)
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY
});

async function main() {
  const response = await anthropic.messages.create({
    model: "claude-sonnet-4-5",
    max_tokens: 1024,
    tools: [{
      name: "get_weather",
      description: "Get the current weather in a given location",
      input_schema: {
        type: "object",
        properties: {
          location: {
            type: "string",
            description: "The city and state, e.g. San Francisco, CA"
          }
        },
        required: ["location"]
      }
    }],
    messages: [{ 
      role: "user", 
      content: "Tell me the weather in San Francisco." 
    }]
  });

  console.log(response);
}

main().catch(console.error);
```

```java Java
import java.util.List;
import java.util.Map;

import com.anthropic.client.AnthropicClient;
import com.anthropic.client.okhttp.AnthropicOkHttpClient;
import com.anthropic.core.JsonValue;
import com.anthropic.models.messages.Message;
import com.anthropic.models.messages.MessageCreateParams;
import com.anthropic.models.messages.Model;
import com.anthropic.models.messages.Tool;
import com.anthropic.models.messages.Tool.InputSchema;

public class GetWeatherExample {

    public static void main(String[] args) {
        AnthropicClient client = AnthropicOkHttpClient.fromEnv();

        InputSchema schema = InputSchema.builder()
                .properties(JsonValue.from(Map.of(
                        "location",
                        Map.of(
                                "type", "string",
                                "description", "The city and state, e.g. San Francisco, CA"))))
                .putAdditionalProperty("required", JsonValue.from(List.of("location")))
                .build();

        MessageCreateParams params = MessageCreateParams.builder()
                .model(Model.CLAUDE_OPUS_4_0)
                .maxTokens(1024)
                .addTool(Tool.builder()
                        .name("get_weather")
                        .description("Get the current weather in a given location")
                        .inputSchema(schema)
                        .build())
                .addUserMessage("What's the weather like in San Francisco?")
                .build();

        Message message = client.messages().create(params);
        System.out.println(message);
    }
}
```

</CodeGroup>

---

## Come funziona l'utilizzo di strumenti

Claude supporta due tipi di strumenti:

1. **Strumenti client**: Strumenti che si eseguono sui tuoi sistemi, che includono:
   - Strumenti personalizzati definiti dall'utente che crei e implementi
   - Strumenti definiti da Anthropic come [computer use](/docs/it/agents-and-tools/tool-use/computer-use-tool) e [text editor](/docs/it/agents-and-tools/tool-use/text-editor-tool) che richiedono l'implementazione del client

2. **Strumenti server**: Strumenti che si eseguono sui server di Anthropic, come gli strumenti [web search](/docs/it/agents-and-tools/tool-use/web-search-tool) e [web fetch](/docs/it/agents-and-tools/tool-use/web-fetch-tool). Questi strumenti devono essere specificati nella richiesta API ma non richiedono implementazione da parte tua.

<Note>
Gli strumenti definiti da Anthropic utilizzano tipi con versione (ad es. `web_search_20250305`, `text_editor_20250124`) per garantire la compatibilità tra le versioni del modello.
</Note>

### Strumenti client
Integra gli strumenti client con Claude in questi passaggi:

<Steps>
  <Step title="Fornisci a Claude gli strumenti e un prompt dell'utente">
    - Definisci gli strumenti client con nomi, descrizioni e schemi di input nella tua richiesta API.
    - Includi un prompt dell'utente che potrebbe richiedere questi strumenti, ad es. "Qual è il meteo a San Francisco?"
  </Step>
  <Step title="Claude decide di utilizzare uno strumento">
    - Claude valuta se uno qualsiasi degli strumenti può aiutare con la query dell'utente.
    - Se sì, Claude costruisce una richiesta di utilizzo dello strumento correttamente formattata.
    - Per gli strumenti client, la risposta API ha un `stop_reason` di `tool_use`, segnalando l'intenzione di Claude.
  </Step>
  <Step title="Esegui lo strumento e restituisci i risultati">
    - Estrai il nome dello strumento e l'input dalla richiesta di Claude
    - Esegui il codice dello strumento sul tuo sistema
    - Restituisci i risultati in un nuovo messaggio `user` contenente un blocco di contenuto `tool_result`
  </Step>
  <Step title="Claude utilizza il risultato dello strumento per formulare una risposta">
    - Claude analizza i risultati dello strumento per elaborare la sua risposta finale al prompt originale dell'utente.
  </Step>
</Steps>
Nota: i passaggi 3 e 4 sono facoltativi. Per alcuni flussi di lavoro, la richiesta di utilizzo dello strumento di Claude (passaggio 2) potrebbe essere tutto ciò di cui hai bisogno, senza inviare i risultati di nuovo a Claude.

### Strumenti server

Gli strumenti server seguono un flusso di lavoro diverso:

<Steps>
  <Step title="Fornisci a Claude gli strumenti e un prompt dell'utente">
    - Gli strumenti server, come [web search](/docs/it/agents-and-tools/tool-use/web-search-tool) e [web fetch](/docs/it/agents-and-tools/tool-use/web-fetch-tool), hanno i loro parametri.
    - Includi un prompt dell'utente che potrebbe richiedere questi strumenti, ad es. "Cerca le ultime notizie sull'IA" o "Analizza il contenuto a questo URL."
  </Step>
  <Step title="Claude esegue lo strumento server">
    - Claude valuta se uno strumento server può aiutare con la query dell'utente.
    - Se sì, Claude esegue lo strumento e i risultati vengono automaticamente incorporati nella risposta di Claude.
  </Step>
  <Step title="Claude utilizza il risultato dello strumento server per formulare una risposta">
    - Claude analizza i risultati dello strumento server per elaborare la sua risposta finale al prompt originale dell'utente.
    - Non è necessaria alcuna interazione aggiuntiva dell'utente per l'esecuzione dello strumento server.
  </Step>
</Steps>

---

## Utilizzo di strumenti MCP con Claude

Se stai costruendo un'applicazione che utilizza il [Model Context Protocol (MCP)](https://modelcontextprotocol.io), puoi utilizzare gli strumenti dai server MCP direttamente con l'API Messages di Claude. Le definizioni degli strumenti MCP utilizzano un formato di schema simile al formato degli strumenti di Claude. Devi solo rinominare `inputSchema` in `input_schema`.

<Tip>
**Non vuoi costruire il tuo client MCP?** Utilizza il [connettore MCP](/docs/it/agents-and-tools/mcp-connector) per connetterti direttamente ai server MCP remoti dall'API Messages senza implementare un client.
</Tip>

### Conversione degli strumenti MCP al formato Claude

Quando costruisci un client MCP e chiami `list_tools()` su un server MCP, riceverai definizioni di strumenti con un campo `inputSchema`. Per utilizzare questi strumenti con Claude, convertili al formato di Claude:

<CodeGroup>
```python Python
from mcp import ClientSession

async def get_claude_tools(mcp_session: ClientSession):
    """Convert MCP tools to Claude's tool format."""
    mcp_tools = await mcp_session.list_tools()

    claude_tools = []
    for tool in mcp_tools.tools:
        claude_tools.append({
            "name": tool.name,
            "description": tool.description or "",
            "input_schema": tool.inputSchema  # Rename inputSchema to input_schema
        })

    return claude_tools
```

```typescript TypeScript
import { Client } from "@modelcontextprotocol/sdk/client/index.js";

async function getClaudeTools(mcpClient: Client) {
  // Convert MCP tools to Claude's tool format
  const mcpTools = await mcpClient.listTools();

  return mcpTools.tools.map((tool) => ({
    name: tool.name,
    description: tool.description ?? "",
    input_schema: tool.inputSchema, // Rename inputSchema to input_schema
  }));
}
```
</CodeGroup>

Quindi passa questi strumenti convertiti a Claude:

<CodeGroup>
```python Python
import anthropic

client = anthropic.Anthropic()
claude_tools = await get_claude_tools(mcp_session)

response = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    tools=claude_tools,
    messages=[{"role": "user", "content": "What tools do you have available?"}]
)
```

```typescript TypeScript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic();
const claudeTools = await getClaudeTools(mcpClient);

const response = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  tools: claudeTools,
  messages: [{ role: "user", content: "What tools do you have available?" }],
});
```
</CodeGroup>

Quando Claude risponde con un blocco `tool_use`, esegui lo strumento sul tuo server MCP utilizzando `call_tool()` e restituisci il risultato a Claude in un blocco `tool_result`.

Per una guida completa alla costruzione di client MCP, vedi [Build an MCP client](https://modelcontextprotocol.io/docs/develop/build-client).

---

## Esempi di utilizzo degli strumenti

Ecco alcuni esempi di codice che dimostrano vari modelli e tecniche di utilizzo degli strumenti. Per brevità, gli strumenti sono semplici e le descrizioni degli strumenti sono più brevi di quanto sarebbe ideale per garantire le migliori prestazioni.

<section title="Esempio di singolo strumento">

<CodeGroup>
    ```bash Shell
    curl https://api.anthropic.com/v1/messages \
         --header "x-api-key: $ANTHROPIC_API_KEY" \
         --header "anthropic-version: 2023-06-01" \
         --header "content-type: application/json" \
         --data \
    '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 1024,
        "tools": [{
            "name": "get_weather",
            "description": "Get the current weather in a given location",
            "input_schema": {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"],
                        "description": "The unit of temperature, either \"celsius\" or \"fahrenheit\""
                    }
                },
                "required": ["location"]
            }
        }],
        "messages": [{"role": "user", "content": "What is the weather like in San Francisco?"}]
    }'
    ```

    ```python Python
    import anthropic
    client = anthropic.Anthropic()

    response = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=1024,
        tools=[
            {
                "name": "get_weather",
                "description": "Get the current weather in a given location",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA"
                        },
                        "unit": {
                            "type": "string",
                            "enum": ["celsius", "fahrenheit"],
                            "description": "The unit of temperature, either \"celsius\" or \"fahrenheit\""
                        }
                    },
                    "required": ["location"]
                }
            }
        ],
        messages=[{"role": "user", "content": "What is the weather like in San Francisco?"}]
    )

    print(response)
    ```
    
    ```java Java
    import java.util.List;
    import java.util.Map;

    import com.anthropic.client.AnthropicClient;
    import com.anthropic.client.okhttp.AnthropicOkHttpClient;
    import com.anthropic.core.JsonValue;
    import com.anthropic.models.messages.Message;
    import com.anthropic.models.messages.MessageCreateParams;
    import com.anthropic.models.messages.Model;
    import com.anthropic.models.messages.Tool;
    import com.anthropic.models.messages.Tool.InputSchema;

    public class WeatherToolExample {

        public static void main(String[] args) {
            AnthropicClient client = AnthropicOkHttpClient.fromEnv();

            InputSchema schema = InputSchema.builder()
                    .properties(JsonValue.from(Map.of(
                            "location", Map.of(
                                    "type", "string",
                                    "description", "The city and state, e.g. San Francisco, CA"
                            ),
                            "unit", Map.of(
                                    "type", "string",
                                    "enum", List.of("celsius", "fahrenheit"),
                                    "description", "The unit of temperature, either \"celsius\" or \"fahrenheit\""
                            )
                    )))
                    .putAdditionalProperty("required", JsonValue.from(List.of("location")))
                    .build();

            MessageCreateParams params = MessageCreateParams.builder()
                    .model(Model.CLAUDE_OPUS_4_0)
                    .maxTokens(1024)
                    .addTool(Tool.builder()
                            .name("get_weather")
                            .description("Get the current weather in a given location")
                            .inputSchema(schema)
                            .build())
                    .addUserMessage("What is the weather like in San Francisco?")
                    .build();

            Message message = client.messages().create(params);
            System.out.println(message);
        }
    }
    ```

</CodeGroup>

Claude restituirà una risposta simile a:

```json JSON
{
  "id": "msg_01Aq9w938a90dw8q",
  "model": "claude-sonnet-4-5",
  "stop_reason": "tool_use",
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "I'll check the current weather in San Francisco for you."
    },
    {
      "type": "tool_use",
      "id": "toolu_01A09q90qw90lq917835lq9",
      "name": "get_weather",
      "input": {"location": "San Francisco, CA", "unit": "celsius"}
    }
  ]
}
```

Dovresti quindi eseguire la funzione `get_weather` con l'input fornito e restituire il risultato in un nuovo messaggio `user`:

<CodeGroup>
    ```bash Shell
    curl https://api.anthropic.com/v1/messages \
         --header "x-api-key: $ANTHROPIC_API_KEY" \
         --header "anthropic-version: 2023-06-01" \
         --header "content-type: application/json" \
         --data \
    '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 1024,
        "tools": [
            {
                "name": "get_weather",
                "description": "Get the current weather in a given location",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA"
                        },
                        "unit": {
                            "type": "string",
                            "enum": ["celsius", "fahrenheit"],
                            "description": "The unit of temperature, either \"celsius\" or \"fahrenheit\""
                        }
                    },
                    "required": ["location"]
                }
            }
        ],
        "messages": [
            {
                "role": "user",
                "content": "What is the weather like in San Francisco?"
            },
            {
                "role": "assistant",
                "content": [
                    {
                        "type": "text",
                        "text": "I'll check the current weather in San Francisco for you."
                    },
                    {
                        "type": "tool_use",
                        "id": "toolu_01A09q90qw90lq917835lq9",
                        "name": "get_weather",
                        "input": {
                            "location": "San Francisco, CA",
                            "unit": "celsius"
                        }
                    }
                ]
            },
            {
                "role": "user",
                "content": [
                    {
                        "type": "tool_result",
                        "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
                        "content": "15 degrees"
                    }
                ]
            }
        ]
    }'
    ```

    ```python Python
    response = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=1024,
        tools=[
            {
                "name": "get_weather",
                "description": "Get the current weather in a given location",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA"
                        },
                        "unit": {
                            "type": "string",
                            "enum": ["celsius", "fahrenheit"],
                            "description": "The unit of temperature, either 'celsius' or 'fahrenheit'"
                        }
                    },
                    "required": ["location"]
                }
            }
        ],
        messages=[
            {
                "role": "user",
                "content": "What's the weather like in San Francisco?"
            },
            {
                "role": "assistant",
                "content": [
                    {
                        "type": "text",
                        "text": "I'll check the current weather in San Francisco for you."
                    },
                    {
                        "type": "tool_use",
                        "id": "toolu_01A09q90qw90lq917835lq9",
                        "name": "get_weather",
                        "input": {"location": "San Francisco, CA", "unit": "celsius"}
                    }
                ]
            },
            {
                "role": "user",
                "content": [
                    {
                        "type": "tool_result",
                        "tool_use_id": "toolu_01A09q90qw90lq917835lq9", # from the API response
                        "content": "65 degrees" # from running your tool
                    }
                ]
            }
        ]
    )

    print(response)
    ```
    
   ```java Java
    import java.util.List;
    import java.util.Map;

    import com.anthropic.client.AnthropicClient;
    import com.anthropic.client.okhttp.AnthropicOkHttpClient;
    import com.anthropic.core.JsonValue;
    import com.anthropic.models.messages.*;
    import com.anthropic.models.messages.Tool.InputSchema;

    public class ToolConversationExample {

        public static void main(String[] args) {
            AnthropicClient client = AnthropicOkHttpClient.fromEnv();

            InputSchema schema = InputSchema.builder()
                    .properties(JsonValue.from(Map.of(
                            "location", Map.of(
                                    "type", "string",
                                    "description", "The city and state, e.g. San Francisco, CA"
                            ),
                            "unit", Map.of(
                                    "type", "string",
                                    "enum", List.of("celsius", "fahrenheit"),
                                    "description", "The unit of temperature, either \"celsius\" or \"fahrenheit\""
                            )
                    )))
                    .putAdditionalProperty("required", JsonValue.from(List.of("location")))
                    .build();

            MessageCreateParams params = MessageCreateParams.builder()
                    .model(Model.CLAUDE_OPUS_4_0)
                    .maxTokens(1024)
                    .addTool(Tool.builder()
                            .name("get_weather")
                            .description("Get the current weather in a given location")
                            .inputSchema(schema)
                            .build())
                    .addUserMessage("What is the weather like in San Francisco?")
                    .addAssistantMessageOfBlockParams(
                            List.of(
                                    ContentBlockParam.ofText(
                                            TextBlockParam.builder()
                                                    .text("I'll check the current weather in San Francisco for you.")
                                                    .build()
                                    ),
                                    ContentBlockParam.ofToolUse(
                                            ToolUseBlockParam.builder()
                                                    .id("toolu_01A09q90qw90lq917835lq9")
                                                    .name("get_weather")
                                                    .input(JsonValue.from(Map.of(
                                                            "location", "San Francisco, CA",
                                                            "unit", "celsius"
                                                    )))
                                                    .build()
                                    )
                            )
                    )
                    .addUserMessageOfBlockParams(List.of(
                            ContentBlockParam.ofToolResult(
                                    ToolResultBlockParam.builder()
                                            .toolUseId("toolu_01A09q90qw90lq917835lq9")
                                            .content("15 degrees")
                                            .build()
                            )
                    ))
                    .build();

            Message message = client.messages().create(params);
            System.out.println(message);
        }
    }
   ```

</CodeGroup>

Questo stamperà la risposta finale di Claude, incorporando i dati meteorologici:

```json JSON
{
  "id": "msg_01Aq9w938a90dw8q",
  "model": "claude-sonnet-4-5",
  "stop_reason": "stop_sequence",
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "The current weather in San Francisco is 15 degrees Celsius (59 degrees Fahrenheit). It's a cool day in the city by the bay!"
    }
  ]
}
```

</section>
<section title="Utilizzo parallelo degli strumenti">

Claude può chiamare più strumenti in parallelo all'interno di una singola risposta, il che è utile per attività che richiedono più operazioni indipendenti. Quando si utilizzano strumenti paralleli, tutti i blocchi `tool_use` sono inclusi in un singolo messaggio dell'assistente e tutti i blocchi `tool_result` corrispondenti devono essere forniti nel messaggio utente successivo.

<Note>
**Importante**: I risultati degli strumenti devono essere formattati correttamente per evitare errori API e garantire che Claude continui a utilizzare strumenti paralleli. Consulta la nostra [guida all'implementazione](/docs/it/agents-and-tools/tool-use/implement-tool-use#parallel-tool-use) per i requisiti di formattazione dettagliati e gli esempi di codice completi.
</Note>

Per esempi completi, script di test e best practice per l'implementazione di chiamate di strumenti paralleli, consulta la [sezione sull'utilizzo parallelo degli strumenti](/docs/it/agents-and-tools/tool-use/implement-tool-use#parallel-tool-use) nella nostra guida all'implementazione.

</section>
<section title="Esempio di più strumenti">

Puoi fornire a Claude più strumenti tra cui scegliere in una singola richiesta. Ecco un esempio con sia uno strumento `get_weather` che uno `get_time`, insieme a una query dell'utente che chiede entrambi.

<CodeGroup>
    ```bash Shell
    curl https://api.anthropic.com/v1/messages \
         --header "x-api-key: $ANTHROPIC_API_KEY" \
         --header "anthropic-version: 2023-06-01" \
         --header "content-type: application/json" \
         --data \
    '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 1024,
        "tools": [{
            "name": "get_weather",
            "description": "Get the current weather in a given location",
            "input_schema": {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"],
                        "description": "The unit of temperature, either 'celsius' or 'fahrenheit'"
                    }
                },
                "required": ["location"]
            }
        },
        {
            "name": "get_time",
            "description": "Get the current time in a given time zone",
            "input_schema": {
                "type": "object",
                "properties": {
                    "timezone": {
                        "type": "string",
                        "description": "The IANA time zone name, e.g. America/Los_Angeles"
                    }
                },
                "required": ["timezone"]
            }
        }],
        "messages": [{
            "role": "user",
            "content": "What is the weather like right now in New York? Also what time is it there?"
        }]
    }'
    ```

    ```python Python
    import anthropic
    client = anthropic.Anthropic()

    response = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=1024,
        tools=[
            {
                "name": "get_weather",
                "description": "Get the current weather in a given location",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA"
                        },
                        "unit": {
                            "type": "string",
                            "enum": ["celsius", "fahrenheit"],
                            "description": "The unit of temperature, either 'celsius' or 'fahrenheit'"
                        }
                    },
                    "required": ["location"]
                }
            },
            {
                "name": "get_time",
                "description": "Get the current time in a given time zone",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "timezone": {
                            "type": "string",
                            "description": "The IANA time zone name, e.g. America/Los_Angeles"
                        }
                    },
                    "required": ["timezone"]
                }
            }
        ],
        messages=[
            {
                "role": "user",
                "content": "What is the weather like right now in New York? Also what time is it there?"
            }
        ]
    )
    print(response)
    ```
    
    ```java Java
    import java.util.List;
    import java.util.Map;

    import com.anthropic.client.AnthropicClient;
    import com.anthropic.client.okhttp.AnthropicOkHttpClient;
    import com.anthropic.core.JsonValue;
    import com.anthropic.models.messages.Message;
    import com.anthropic.models.messages.MessageCreateParams;
    import com.anthropic.models.messages.Model;
    import com.anthropic.models.messages.Tool;
    import com.anthropic.models.messages.Tool.InputSchema;

    public class MultipleToolsExample {

        public static void main(String[] args) {
            AnthropicClient client = AnthropicOkHttpClient.fromEnv();

            // Weather tool schema
            InputSchema weatherSchema = InputSchema.builder()
                    .properties(JsonValue.from(Map.of(
                            "location", Map.of(
                                    "type", "string",
                                    "description", "The city and state, e.g. San Francisco, CA"
                            ),
                            "unit", Map.of(
                                    "type", "string",
                                    "enum", List.of("celsius", "fahrenheit"),
                                    "description", "The unit of temperature, either \"celsius\" or \"fahrenheit\""
                            )
                    )))
                    .putAdditionalProperty("required", JsonValue.from(List.of("location")))
                    .build();

            // Time tool schema
            InputSchema timeSchema = InputSchema.builder()
                    .properties(JsonValue.from(Map.of(
                            "timezone", Map.of(
                                    "type", "string",
                                    "description", "The IANA time zone name, e.g. America/Los_Angeles"
                            )
                    )))
                    .putAdditionalProperty("required", JsonValue.from(List.of("timezone")))
                    .build();

            MessageCreateParams params = MessageCreateParams.builder()
                    .model(Model.CLAUDE_OPUS_4_0)
                    .maxTokens(1024)
                    .addTool(Tool.builder()
                            .name("get_weather")
                            .description("Get the current weather in a given location")
                            .inputSchema(weatherSchema)
                            .build())
                    .addTool(Tool.builder()
                            .name("get_time")
                            .description("Get the current time in a given time zone")
                            .inputSchema(timeSchema)
                            .build())
                    .addUserMessage("What is the weather like right now in New York? Also what time is it there?")
                    .build();

            Message message = client.messages().create(params);
            System.out.println(message);
        }
    }
    ```

</CodeGroup>

In questo caso, Claude potrebbe:
- Utilizzare gli strumenti in sequenza (uno alla volta) — chiamando prima `get_weather`, poi `get_time` dopo aver ricevuto il risultato meteorologico
- Utilizzare chiamate di strumenti paralleli — producendo più blocchi `tool_use` in una singola risposta quando le operazioni sono indipendenti

Quando Claude effettua chiamate di strumenti paralleli, devi restituire tutti i risultati degli strumenti in un singolo messaggio `user`, con ogni risultato nel suo proprio blocco `tool_result`.

</section>
<section title="Informazioni mancanti">

Se il prompt dell'utente non include informazioni sufficienti per compilare tutti i parametri richiesti per uno strumento, Claude Opus è molto più probabile che riconosca che un parametro è mancante e lo chieda. Claude Sonnet potrebbe chiedere, soprattutto quando gli viene richiesto di pensare prima di produrre una richiesta di strumento. Ma potrebbe anche fare del suo meglio per dedurre un valore ragionevole.

Ad esempio, utilizzando lo strumento `get_weather` sopra, se chiedi a Claude "Qual è il meteo?" senza specificare una posizione, Claude, in particolare Claude Sonnet, potrebbe fare una supposizione sui parametri degli strumenti:

```json JSON
{
  "type": "tool_use",
  "id": "toolu_01A09q90qw90lq917835lq9",
  "name": "get_weather",
  "input": {"location": "New York, NY", "unit": "fahrenheit"}
}
```

Questo comportamento non è garantito, soprattutto per prompt più ambigui e per modelli meno intelligenti. Se Claude Opus non ha abbastanza contesto per compilare i parametri richiesti, è molto più probabile che risponda con una domanda di chiarimento invece di effettuare una chiamata di strumento.

</section>
<section title="Strumenti sequenziali">

Alcuni compiti potrebbero richiedere la chiamata di più strumenti in sequenza, utilizzando l'output di uno strumento come input per un altro. In tal caso, Claude chiamerà uno strumento alla volta. Se gli viene richiesto di chiamare gli strumenti tutti contemporaneamente, Claude probabilmente indovinerà i parametri per gli strumenti più a valle se dipendono dai risultati degli strumenti più a monte.

Ecco un esempio di utilizzo di uno strumento `get_location` per ottenere la posizione dell'utente, quindi passare quella posizione allo strumento `get_weather`:

<CodeGroup>
    ```bash Shell
    curl https://api.anthropic.com/v1/messages \
         --header "x-api-key: $ANTHROPIC_API_KEY" \
         --header "anthropic-version: 2023-06-01" \
         --header "content-type: application/json" \
         --data \
    '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 1024,
        "tools": [
            {
                "name": "get_location",
                "description": "Get the current user location based on their IP address. This tool has no parameters or arguments.",
                "input_schema": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "get_weather",
                "description": "Get the current weather in a given location",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA"
                        },
                        "unit": {
                            "type": "string",
                            "enum": ["celsius", "fahrenheit"],
                            "description": "The unit of temperature, either 'celsius' or 'fahrenheit'"
                        }
                    },
                    "required": ["location"]
                }
            }
        ],
        "messages": [{
            "role": "user",
            "content": "What is the weather like where I am?"
        }]
    }'
    ```

    ```python Python
    response = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=1024,
        tools=[
            {
                "name": "get_location",
                "description": "Get the current user location based on their IP address. This tool has no parameters or arguments.",
                "input_schema": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "get_weather",
                "description": "Get the current weather in a given location",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA"
                        },
                        "unit": {
                            "type": "string",
                            "enum": ["celsius", "fahrenheit"],
                            "description": "The unit of temperature, either 'celsius' or 'fahrenheit'"
                        }
                    },
                    "required": ["location"]
                }
            }
        ],
        messages=[{
       		  "role": "user",
        	  "content": "What's the weather like where I am?"
        }]
    )
    ```
    
    ```java Java
    import java.util.List;
    import java.util.Map;

    import com.anthropic.client.AnthropicClient;
    import com.anthropic.client.okhttp.AnthropicOkHttpClient;
    import com.anthropic.core.JsonValue;
    import com.anthropic.models.messages.Message;
    import com.anthropic.models.messages.MessageCreateParams;
    import com.anthropic.models.messages.Model;
    import com.anthropic.models.messages.Tool;
    import com.anthropic.models.messages.Tool.InputSchema;

    public class EmptySchemaToolExample {

        public static void main(String[] args) {
            AnthropicClient client = AnthropicOkHttpClient.fromEnv();

            // Empty schema for location tool
            InputSchema locationSchema = InputSchema.builder()
                    .properties(JsonValue.from(Map.of()))
                    .build();

            // Weather tool schema
            InputSchema weatherSchema = InputSchema.builder()
                    .properties(JsonValue.from(Map.of(
                            "location", Map.of(
                                    "type", "string",
                                    "description", "The city and state, e.g. San Francisco, CA"
                            ),
                            "unit", Map.of(
                                    "type", "string",
                                    "enum", List.of("celsius", "fahrenheit"),
                                    "description", "The unit of temperature, either \"celsius\" or \"fahrenheit\""
                            )
                    )))
                    .putAdditionalProperty("required", JsonValue.from(List.of("location")))
                    .build();

            MessageCreateParams params = MessageCreateParams.builder()
                    .model(Model.CLAUDE_OPUS_4_0)
                    .maxTokens(1024)
                    .addTool(Tool.builder()
                            .name("get_location")
                            .description("Get the current user location based on their IP address. This tool has no parameters or arguments.")
                            .inputSchema(locationSchema)
                            .build())
                    .addTool(Tool.builder()
                            .name("get_weather")
                            .description("Get the current weather in a given location")
                            .inputSchema(weatherSchema)
                            .build())
                    .addUserMessage("What is the weather like where I am?")
                    .build();

            Message message = client.messages().create(params);
            System.out.println(message);
        }
    }
    ```

</CodeGroup>

In questo caso, Claude chiamerebbe prima lo strumento `get_location` per ottenere la posizione dell'utente. Dopo aver restituito la posizione in un `tool_result`, Claude chiamerebbe quindi `get_weather` con quella posizione per ottenere la risposta finale.

La conversazione completa potrebbe assomigliare a:

| Ruolo      | Contenuto                                                                                                                                                                                                                                 |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Utente      | Qual è il meteo dove mi trovo?                                                                                                                                                                                                                     |
| Assistente | Troverò prima la tua posizione attuale, poi controllerò il meteo lì. \[Utilizzo dello strumento per get_location\] |
| Utente      | \[Risultato dello strumento per get_location con id corrispondente e risultato di San Francisco, CA\]                                                                                                                                                       |
| Assistente | \[Utilizzo dello strumento per get_weather con il seguente input\]\{ "location": "San Francisco, CA", "unit": "fahrenheit" }                                                                                                                         |
| Utente      | \[Risultato dello strumento per get_weather con id corrispondente e risultato di "59°F (15°C), per lo più nuvoloso"\]                                                                                                                                             |
| Assistente | In base alla tua posizione attuale a San Francisco, CA, il meteo in questo momento è 59°F (15°C) e per lo più nuvoloso. È una giornata abbastanza fresca e nuvolosa in città. Potresti voler portare una giacca leggera se stai per uscire.           |

Questo esempio dimostra come Claude può concatenare più chiamate di strumenti per rispondere a una domanda che richiede la raccolta di dati da diverse fonti. I passaggi chiave sono:

1. Claude si rende conto che ha bisogno della posizione dell'utente per rispondere alla domanda sul meteo, quindi chiama lo strumento `get_location`.
2. L'utente (cioè il codice client) esegue la funzione `get_location` effettiva e restituisce il risultato "San Francisco, CA" in un blocco `tool_result`.
3. Con la posizione ora nota, Claude procede a chiamare lo strumento `get_weather`, passando "San Francisco, CA" come parametro `location` (così come un parametro `unit` indovinato, poiché `unit` non è un parametro obbligatorio).
4. L'utente esegue di nuovo la funzione `get_weather` effettiva con gli argomenti forniti e restituisce i dati meteorologici in un altro blocco `tool_result`.
5. Infine, Claude incorpora i dati meteorologici in una risposta in linguaggio naturale alla domanda originale.

</section>
<section title="Utilizzo degli strumenti con catena di pensiero">

Per impostazione predefinita, Claude Opus è incoraggiato a pensare prima di rispondere a una query di utilizzo dello strumento per determinare al meglio se uno strumento è necessario, quale strumento utilizzare e i parametri appropriati. Claude Sonnet e Claude Haiku sono incoraggiati a cercare di utilizzare gli strumenti il più possibile e sono più propensi a chiamare uno strumento non necessario o a dedurre parametri mancanti. Per incoraggiare Sonnet o Haiku a valutare meglio la query dell'utente prima di effettuare chiamate di strumenti, è possibile utilizzare il seguente prompt:

Prompt della catena di pensiero

`Rispondi alla richiesta dell'utente utilizzando gli strumenti rilevanti (se disponibili). Prima di chiamare uno strumento, fai un'analisi. Innanzitutto, pensa a quale dei strumenti forniti è lo strumento rilevante per rispondere alla richiesta dell'utente. In secondo luogo, esamina ogni parametro obbligatorio dello strumento rilevante e determina se l'utente ha fornito direttamente o fornito informazioni sufficienti per dedurre un valore. Quando decidi se il parametro può essere dedotto, considera attentamente tutto il contesto per vedere se supporta un valore specifico. Se tutti i parametri richiesti sono presenti o possono essere ragionevolmente dedotti, procedi con la chiamata dello strumento. MA, se uno dei valori per un parametro obbligatorio è mancante, NON invocare la funzione (nemmeno con riempitivi per i parametri mancanti) e invece, chiedi all'utente di fornire i parametri mancanti. NON chiedere ulteriori informazioni sui parametri facoltativi se non forniti.
`

</section>

---

## Prezzi

Tool use requests are priced based on:
1. The total number of input tokens sent to the model (including in the `tools` parameter)
2. The number of output tokens generated
3. For server-side tools, additional usage-based pricing (e.g., web search charges per search performed)

Client-side tools are priced the same as any other Claude API request, while server-side tools may incur additional charges based on their specific usage.

The additional tokens from tool use come from:

- The `tools` parameter in API requests (tool names, descriptions, and schemas)
- `tool_use` content blocks in API requests and responses
- `tool_result` content blocks in API requests

When you use `tools`, we also automatically include a special system prompt for the model which enables tool use. The number of tool use tokens required for each model are listed below (excluding the additional tokens listed above). Note that the table assumes at least 1 tool is provided. If no `tools` are provided, then a tool choice of `none` uses 0 additional system prompt tokens.

| Model                    | Tool choice                                          | Tool use system prompt token count          |
|--------------------------|------------------------------------------------------|---------------------------------------------|
| Claude Opus 4.5            | `auto`, `none`<hr />`any`, `tool`   | 346 tokens<hr />313 tokens |
| Claude Opus 4.1            | `auto`, `none`<hr />`any`, `tool`   | 346 tokens<hr />313 tokens |
| Claude Opus 4            | `auto`, `none`<hr />`any`, `tool`   | 346 tokens<hr />313 tokens |
| Claude Sonnet 4.5          | `auto`, `none`<hr />`any`, `tool`   | 346 tokens<hr />313 tokens |
| Claude Sonnet 4          | `auto`, `none`<hr />`any`, `tool`   | 346 tokens<hr />313 tokens |
| Claude Sonnet 3.7 ([deprecated](/docs/en/about-claude/model-deprecations))        | `auto`, `none`<hr />`any`, `tool`   | 346 tokens<hr />313 tokens |
| Claude Haiku 4.5         | `auto`, `none`<hr />`any`, `tool`   | 346 tokens<hr />313 tokens |
| Claude Haiku 3.5         | `auto`, `none`<hr />`any`, `tool`   | 264 tokens<hr />340 tokens |
| Claude Opus 3 ([deprecated](/docs/en/about-claude/model-deprecations))            | `auto`, `none`<hr />`any`, `tool`   | 530 tokens<hr />281 tokens |
| Claude Sonnet 3          | `auto`, `none`<hr />`any`, `tool`   | 159 tokens<hr />235 tokens |
| Claude Haiku 3           | `auto`, `none`<hr />`any`, `tool`   | 264 tokens<hr />340 tokens |

These token counts are added to your normal input and output tokens to calculate the total cost of a request.

Consulta la nostra [tabella di confronto dei modelli](/docs/it/about-claude/models/overview#model-comparison-table) per i prezzi attuali per modello.

Quando invii un prompt con tool use, proprio come qualsiasi altra richiesta API, la risposta restituirà sia i conteggi dei token di input che di output come parte delle metriche `usage` segnalate.

---

## Passaggi Successivi

Esplora il nostro repository di esempi di codice tool use pronti per l'implementazione nei nostri cookbook:

<CardGroup cols={3}>
  <Card
    title="Calculator Tool"
    icon="calculator"
    href="https://github.com/anthropics/anthropic-cookbook/blob/main/tool_use/calculator_tool.ipynb"
  >
    Scopri come integrare uno strumento calcolatrice semplice con Claude per calcoli numerici precisi.
  </Card>

{" "}
<Card
  title="Customer Service Agent"
  icon="headset"
  href="https://github.com/anthropics/anthropic-cookbook/blob/main/tool_use/customer_service_agent.ipynb"
>
  Crea un bot di servizio clienti reattivo che sfrutta gli strumenti client per
  migliorare il supporto.
</Card>

  <Card
    title="JSON Extractor"
    icon="code-brackets"
    href="https://github.com/anthropics/anthropic-cookbook/blob/main/tool_use/extracting_structured_json.ipynb"
  >
    Scopri come Claude e tool use possono estrarre dati strutturati da testo non strutturato.
  </Card>
</CardGroup>