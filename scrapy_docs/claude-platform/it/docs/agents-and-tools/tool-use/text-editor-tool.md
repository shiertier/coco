# Strumento editor di testo

Claude può utilizzare uno strumento editor di testo definito da Anthropic per visualizzare e modificare file di testo, aiutandoti a eseguire il debug, correggere e migliorare il tuo codice o altri documenti di testo.

---

Claude può utilizzare uno strumento editor di testo definito da Anthropic per visualizzare e modificare file di testo, aiutandoti a eseguire il debug, correggere e migliorare il tuo codice o altri documenti di testo. Questo consente a Claude di interagire direttamente con i tuoi file, fornendo assistenza pratica piuttosto che solo suggerire modifiche.

## Compatibilità dei modelli

| Modello | Versione dello strumento |
|-------|--------------|
| Modelli Claude 4.x | `text_editor_20250728` |
| Claude Sonnet 3.7 ([deprecato](/docs/it/about-claude/model-deprecations)) | `text_editor_20250124` |

<Warning>
Lo strumento `text_editor_20250728` per i modelli Claude 4 non include il comando `undo_edit`. Se hai bisogno di questa funzionalità, dovrai utilizzare Claude Sonnet 3.7 ([deprecato](/docs/it/about-claude/model-deprecations)).
</Warning>

<Warning>
Le versioni precedenti dello strumento non sono garantite essere retrocompatibili con i modelli più recenti. Utilizza sempre la versione dello strumento che corrisponde alla versione del tuo modello.
</Warning>

## Quando utilizzare lo strumento editor di testo

Alcuni esempi di quando utilizzare lo strumento editor di testo sono:
- **Debug del codice**: Chiedi a Claude di identificare e correggere i bug nel tuo codice, dagli errori di sintassi ai problemi di logica.
- **Refactoring del codice**: Consenti a Claude di migliorare la struttura del tuo codice, la leggibilità e le prestazioni attraverso modifiche mirate.
- **Generazione della documentazione**: Chiedi a Claude di aggiungere docstring, commenti o file README al tuo codebase.
- **Creazione di test**: Chiedi a Claude di creare unit test per il tuo codice in base alla sua comprensione dell'implementazione.

## Utilizza lo strumento editor di testo

<Tabs>
<Tab title="Claude 4">
Fornisci lo strumento editor di testo (denominato `str_replace_based_edit_tool`) a Claude utilizzando l'API Messages.

Puoi facoltativamente specificare un parametro `max_characters` per controllare il troncamento quando visualizzi file di grandi dimensioni.

<Note>
`max_characters` è compatibile solo con `text_editor_20250728` e versioni successive dello strumento editor di testo.
</Note>

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
        "type": "text_editor_20250728",
        "name": "str_replace_based_edit_tool",
        "max_characters": 10000
      }
    ],
    "messages": [
      {
        "role": "user",
        "content": "There'\''s a syntax error in my primes.py file. Can you help me fix it?"
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
            "type": "text_editor_20250728",
            "name": "str_replace_based_edit_tool",
            "max_characters": 10000
        }
    ],
    messages=[
        {
            "role": "user", 
            "content": "There's a syntax error in my primes.py file. Can you help me fix it?"
        }
    ]
)
```

```typescript TypeScript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

const response = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  tools: [
    {
      type: "text_editor_20250728",
      name: "str_replace_based_edit_tool",
      max_characters: 10000
    }
  ],
  messages: [
    {
      role: "user",
      content: "There's a syntax error in my primes.py file. Can you help me fix it?"
    }
  ]
});
```

```java Java
import com.anthropic.client.AnthropicClient;
import com.anthropic.client.okhttp.AnthropicOkHttpClient;
import com.anthropic.models.messages.Message;
import com.anthropic.models.messages.MessageCreateParams;
import com.anthropic.models.messages.Model;
import com.anthropic.models.messages.ToolStrReplaceBasedEditTool20250728;

public class TextEditorToolExample {

    public static void main(String[] args) {
        AnthropicClient client = AnthropicOkHttpClient.fromEnv();

        ToolStrReplaceBasedEditTool20250728 editorTool = ToolStrReplaceBasedEditTool20250728.builder()
                .build();

        MessageCreateParams params = MessageCreateParams.builder()
                .model(Model.CLAUDE_SONNET_4_0)
                .maxTokens(1024)
                .addTool(editorTool)
                .addUserMessage("There's a syntax error in my primes.py file. Can you help me fix it?")
                .build();

        Message message = client.messages().create(params);
    }
}
```
</CodeGroup>
</Tab>
<Tab title="Claude Sonnet 3.7">
Fornisci lo strumento editor di testo (denominato `str_replace_editor`) a Claude utilizzando l'API Messages:
<CodeGroup>

```bash Shell
curl https://api.anthropic.com/v1/messages \
  -H "content-type: application/json" \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -d '{
    "model": "claude-3-7-sonnet-20250219",
    "max_tokens": 1024,
    "tools": [
      {
        "type": "text_editor_20250124",
        "name": "str_replace_editor"
      }
    ],
    "messages": [
      {
        "role": "user",
        "content": "There'\''s a syntax error in my primes.py file. Can you help me fix it?"
      }
    ]
  }'
```

```python Python
import anthropic

client = anthropic.Anthropic()

response = client.messages.create(
    model="claude-3-7-sonnet-20250219",
    max_tokens=1024,
    tools=[
        {
            "type": "text_editor_20250124",
            "name": "str_replace_editor"
        }
    ],
    messages=[
        {
            "role": "user", 
            "content": "There's a syntax error in my primes.py file. Can you help me fix it?"
        }
    ]
)
```

```typescript TypeScript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

const response = await anthropic.messages.create({
  model: "claude-3-7-sonnet-20250219",
  max_tokens: 1024,
  tools: [
    {
      type: "text_editor_20250124",
      name: "str_replace_editor"
    }
  ],
  messages: [
    {
      role: "user",
      content: "There's a syntax error in my primes.py file. Can you help me fix it?"
    }
  ]
});
```

```java Java
import com.anthropic.client.AnthropicClient;
import com.anthropic.client.okhttp.AnthropicOkHttpClient;
import com.anthropic.models.messages.Message;
import com.anthropic.models.messages.MessageCreateParams;
import com.anthropic.models.messages.Model;
import com.anthropic.models.messages.ToolTextEditor20250124;

public class TextEditorToolExample {

    public static void main(String[] args) {
        AnthropicClient client = AnthropicOkHttpClient.fromEnv();

        ToolTextEditor20250124 editorTool = ToolTextEditor20250124.builder()
                .build();

        MessageCreateParams params = MessageCreateParams.builder()
                .model(Model.CLAUDE_3_7_SONNET_LATEST)
                .maxTokens(1024)
                .addTool(editorTool)
                .addUserMessage("There's a syntax error in my primes.py file. Can you help me fix it?")
                .build();

        Message message = client.messages().create(params);
    }
}
```
</CodeGroup>
</Tab>
</Tabs>

Lo strumento editor di testo può essere utilizzato nel seguente modo:

<Steps>
  <Step title="Fornisci a Claude lo strumento editor di testo e un prompt dell'utente">
    - Includi lo strumento editor di testo nella tua richiesta API
    - Fornisci un prompt dell'utente che potrebbe richiedere l'esame o la modifica di file, come "Puoi correggere l'errore di sintassi nel mio codice?"
  </Step>
  <Step title="Claude utilizza lo strumento per esaminare file o directory">
    - Claude valuta cosa ha bisogno di esaminare e utilizza il comando `view` per esaminare il contenuto dei file o elencare il contenuto delle directory
    - La risposta API conterrà un blocco di contenuto `tool_use` con il comando `view`
  </Step>
  <Step title="Esegui il comando view e restituisci i risultati">
    - Estrai il percorso del file o della directory dalla richiesta di utilizzo dello strumento di Claude
    - Leggi il contenuto del file o elenca il contenuto della directory
    - Se un parametro `max_characters` è stato specificato nella configurazione dello strumento, tronca il contenuto del file a quella lunghezza
    - Restituisci i risultati a Claude continuando la conversazione con un nuovo messaggio `user` contenente un blocco di contenuto `tool_result`
  </Step>
  <Step title="Claude utilizza lo strumento per modificare i file">
    - Dopo aver esaminato il file o la directory, Claude potrebbe utilizzare un comando come `str_replace` per apportare modifiche o `insert` per aggiungere testo a un numero di riga specifico.
    - Se Claude utilizza il comando `str_replace`, Claude costruisce una richiesta di utilizzo dello strumento correttamente formattata con il testo vecchio e il testo nuovo con cui sostituirlo
  </Step>
  <Step title="Esegui la modifica e restituisci i risultati">
    - Estrai il percorso del file, il testo vecchio e il testo nuovo dalla richiesta di utilizzo dello strumento di Claude
    - Esegui la sostituzione del testo nel file
    - Restituisci i risultati a Claude
  </Step>
  <Step title="Claude fornisce la sua analisi e spiegazione">
    - Dopo aver esaminato e possibilmente modificato i file, Claude fornisce una spiegazione completa di ciò che ha trovato e quali modifiche ha apportato
  </Step>
</Steps>

### Comandi dello strumento editor di testo

Lo strumento editor di testo supporta diversi comandi per visualizzare e modificare file:

#### view

Il comando `view` consente a Claude di esaminare il contenuto di un file o elencare il contenuto di una directory. Può leggere l'intero file o un intervallo specifico di righe.

Parametri:
- `command`: Deve essere "view"
- `path`: Il percorso del file o della directory da visualizzare
- `view_range` (facoltativo): Un array di due interi che specifica i numeri di riga iniziale e finale da visualizzare. I numeri di riga sono indicizzati da 1 e -1 per la riga finale significa leggere fino alla fine del file. Questo parametro si applica solo quando si visualizzano file, non directory.

<section title="Esempi di comandi view">

```json
// Esempio per visualizzare un file
{
  "type": "tool_use",
  "id": "toolu_01A09q90qw90lq917835lq9",
  "name": "str_replace_editor",
  "input": {
    "command": "view",
    "path": "primes.py"
  }
}

// Esempio per visualizzare una directory
{
  "type": "tool_use",
  "id": "toolu_02B19r91rw91mr917835mr9",
  "name": "str_replace_editor",
  "input": {
    "command": "view",
    "path": "src/"
  }
}
```

</section>

#### str_replace

Il comando `str_replace` consente a Claude di sostituire una stringa specifica in un file con una nuova stringa. Questo viene utilizzato per apportare modifiche precise.

Parametri:
- `command`: Deve essere "str_replace"
- `path`: Il percorso del file da modificare
- `old_str`: Il testo da sostituire (deve corrispondere esattamente, inclusi gli spazi bianchi e l'indentazione)
- `new_str`: Il nuovo testo da inserire al posto del testo vecchio

<section title="Esempio di comando str_replace">

```json
{
  "type": "tool_use",
  "id": "toolu_01A09q90qw90lq917835lq9",
  "name": "str_replace_editor",
  "input": {
    "command": "str_replace",
    "path": "primes.py",
    "old_str": "for num in range(2, limit + 1)",
    "new_str": "for num in range(2, limit + 1):"
  }
}
```

</section>

#### create

Il comando `create` consente a Claude di creare un nuovo file con il contenuto specificato.

Parametri:
- `command`: Deve essere "create"
- `path`: Il percorso in cui deve essere creato il nuovo file
- `file_text`: Il contenuto da scrivere nel nuovo file

<section title="Esempio di comando create">

```json
{
  "type": "tool_use",
  "id": "toolu_01A09q90qw90lq917835lq9",
  "name": "str_replace_editor",
  "input": {
    "command": "create",
    "path": "test_primes.py",
    "file_text": "import unittest\nimport primes\n\nclass TestPrimes(unittest.TestCase):\n    def test_is_prime(self):\n        self.assertTrue(primes.is_prime(2))\n        self.assertTrue(primes.is_prime(3))\n        self.assertFalse(primes.is_prime(4))\n\nif __name__ == '__main__':\n    unittest.main()"
  }
}
```

</section>

#### insert

Il comando `insert` consente a Claude di inserire testo in una posizione specifica in un file.

Parametri:
- `command`: Deve essere "insert"
- `path`: Il percorso del file da modificare
- `insert_line`: Il numero di riga dopo il quale inserire il testo (0 per l'inizio del file)
- `new_str`: Il testo da inserire

<section title="Esempio di comando insert">

```json
{
  "type": "tool_use",
  "id": "toolu_01A09q90qw90lq917835lq9",
  "name": "str_replace_editor",
  "input": {
    "command": "insert",
    "path": "primes.py",
    "insert_line": 0,
    "new_str": "\"\"\"Module for working with prime numbers.\n\nThis module provides functions to check if a number is prime\nand to generate a list of prime numbers up to a given limit.\n\"\"\"\n"
  }
}
```

</section>

#### undo_edit

Il comando `undo_edit` consente a Claude di annullare l'ultima modifica apportata a un file.

<Note>
Questo comando è disponibile solo in Claude Sonnet 3.7 ([deprecato](/docs/it/about-claude/model-deprecations)). Non è supportato nei modelli Claude 4 che utilizzano `text_editor_20250728`.
</Note>

Parametri:
- `command`: Deve essere "undo_edit"
- `path`: Il percorso del file la cui ultima modifica deve essere annullata

<section title="Esempio di comando undo_edit">

```json
{
  "type": "tool_use",
  "id": "toolu_01A09q90qw90lq917835lq9",
  "name": "str_replace_editor",
  "input": {
    "command": "undo_edit",
    "path": "primes.py"
  }
}
```

</section>

### Esempio: Correzione di un errore di sintassi con lo strumento editor di testo

<Tabs>
<Tab title="Claude 4">

Questo esempio dimostra come i modelli Claude 4 utilizzano lo strumento editor di testo per correggere un errore di sintassi in un file Python.

Innanzitutto, la tua applicazione fornisce a Claude lo strumento editor di testo e un prompt per correggere un errore di sintassi:

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
        "type": "text_editor_20250728",
        "name": "str_replace_based_edit_tool"
      }
    ],
    "messages": [
      {
        "role": "user",
        "content": "There'\''s a syntax error in my primes.py file. Can you help me fix it?"
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
            "type": "text_editor_20250728",
            "name": "str_replace_based_edit_tool"
        }
    ],
    messages=[
        {
            "role": "user", 
            "content": "There's a syntax error in my primes.py file. Can you help me fix it?"
        }
    ]
)
```

```typescript TypeScript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

const response = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  tools: [
    {
      type: "text_editor_20250728",
      name: "str_replace_based_edit_tool"
    }
  ],
  messages: [
    {
      role: "user",
      content: "There's a syntax error in my primes.py file. Can you help me fix it?"
    }
  ]
});
```

```java Java
import com.anthropic.client.AnthropicClient;
import com.anthropic.client.okhttp.AnthropicOkHttpClient;
import com.anthropic.models.messages.Message;
import com.anthropic.models.messages.MessageCreateParams;
import com.anthropic.models.messages.Model;
import com.anthropic.models.messages.ToolStrReplaceBasedEditTool20250728;

public class TextEditorToolExample {

    public static void main(String[] args) {
        AnthropicClient client = AnthropicOkHttpClient.fromEnv();

        ToolStrReplaceBasedEditTool20250728 editorTool = ToolStrReplaceBasedEditTool20250728.builder()
                .build();

        MessageCreateParams params = MessageCreateParams.builder()
                .model(Model.CLAUDE_SONNET_4_0)
                .maxTokens(1024)
                .addTool(editorTool)
                .addUserMessage("There's a syntax error in my primes.py file. Can you help me fix it?")
                .build();

        Message message = client.messages().create(params);
    }
}
```
</CodeGroup>

Claude utilizzerà prima lo strumento editor di testo per visualizzare il file:

```json
{
  "id": "msg_01XAbCDeFgHiJkLmNoPQrStU",
  "model": "claude-sonnet-4-5",
  "stop_reason": "tool_use",
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "I'll help you fix the syntax error in your primes.py file. First, let me take a look at the file to identify the issue."
    },
    {
      "type": "tool_use",
      "id": "toolu_01AbCdEfGhIjKlMnOpQrStU",
      "name": "str_replace_based_edit_tool",
      "input": {
        "command": "view",
        "path": "primes.py"
      }
    }
  ]
}
```

La tua applicazione dovrebbe quindi leggere il file e restituire il suo contenuto a Claude:

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
        "type": "text_editor_20250728",
        "name": "str_replace_based_edit_tool"
      }
    ],
    "messages": [
      {
        "role": "user",
        "content": "There'\''s a syntax error in my primes.py file. Can you help me fix it?"
      },
      {
            "role": "assistant",
            "content": [
                {
                    "type": "text",
                    "text": "I'\''ll help you fix the syntax error in your primes.py file. First, let me take a look at the file to identify the issue."
                },
                {
                    "type": "tool_use",
                    "id": "toolu_01AbCdEfGhIjKlMnOpQrStU",
                    "name": "str_replace_based_edit_tool",
                    "input": {
                        "command": "view",
                        "path": "primes.py"
                    }
                }
            ]
        },
        {
            "role": "user",
            "content": [
                {
                    "type": "tool_result",
                    "tool_use_id": "toolu_01AbCdEfGhIjKlMnOpQrStU",
                    "content": "1: def is_prime(n):\n2:     \"\"\"Check if a number is prime.\"\"\"\n3:     if n <= 1:\n4:         return False\n5:     if n <= 3:\n6:         return True\n7:     if n % 2 == 0 or n % 3 == 0:\n8:         return False\n9:     i = 5\n10:     while i * i <= n:\n11:         if n % i == 0 or n % (i + 2) == 0:\n12:             return False\n13:         i += 6\n14:     return True\n15: \n16: def get_primes(limit):\n17:     \"\"\"Generate a list of prime numbers up to the given limit.\"\"\"\n18:     primes = []\n19:     for num in range(2, limit + 1)\n20:         if is_prime(num):\n21:             primes.append(num)\n22:     return primes\n23: \n24: def main():\n25:     \"\"\"Main function to demonstrate prime number generation.\"\"\"\n26:     limit = 100\n27:     prime_list = get_primes(limit)\n28:     print(f\"Prime numbers up to {limit}:\")\n29:     print(prime_list)\n30:     print(f\"Found {len(prime_list)} prime numbers.\")\n31: \n32: if __name__ == \"__main__\":\n33:     main()"
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
            "type": "text_editor_20250728",
            "name": "str_replace_based_edit_tool"
        }
    ],
    messages=[
        {
            "role": "user", 
            "content": "There's a syntax error in my primes.py file. Can you help me fix it?"
        },
        {
            "role": "assistant",
            "content": [
                {
                    "type": "text",
                    "text": "I'll help you fix the syntax error in your primes.py file. First, let me take a look at the file to identify the issue."
                },
                {
                    "type": "tool_use",
                    "id": "toolu_01AbCdEfGhIjKlMnOpQrStU",
                    "name": "str_replace_based_edit_tool",
                    "input": {
                        "command": "view",
                        "path": "primes.py"
                    }
                }
            ]
        },
        {
            "role": "user",
            "content": [
                {
                    "type": "tool_result",
                    "tool_use_id": "toolu_01AbCdEfGhIjKlMnOpQrStU",
                    "content": "1: def is_prime(n):\n2:     \"\"\"Check if a number is prime.\"\"\"\n3:     if n <= 1:\n4:         return False\n5:     if n <= 3:\n6:         return True\n7:     if n % 2 == 0 or n % 3 == 0:\n8:         return False\n9:     i = 5\n10:     while i * i <= n:\n11:         if n % i == 0 or n % (i + 2) == 0:\n12:             return False\n13:         i += 6\n14:     return True\n15: \n16: def get_primes(limit):\n17:     \"\"\"Generate a list of prime numbers up to the given limit.\"\"\"\n18:     primes = []\n19:     for num in range(2, limit + 1)\n20:         if is_prime(num):\n21:             primes.append(num)\n22:     return primes\n23: \n24: def main():\n25:     \"\"\"Main function to demonstrate prime number generation.\"\"\"\n26:     limit = 100\n27:     prime_list = get_primes(limit)\n28:     print(f\"Prime numbers up to {limit}:\")\n29:     print(prime_list)\n30:     print(f\"Found {len(prime_list)} prime numbers.\")\n31: \n32: if __name__ == \"__main__\":\n33:     main()"
                }
            ]
        }
    ]
)
```

```typescript TypeScript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

const response = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  tools: [
    {
      type: "text_editor_20250728",
      name: "str_replace_based_edit_tool"
    }
  ],
  messages: [
    {
      role: "user",
      content: "There's a syntax error in my primes.py file. Can you help me fix it?"
    },
    {
      role: "assistant",
      content: [
          {
              type: "text",
              text: "I'll help you fix the syntax error in your primes.py file. First, let me take a look at the file to identify the issue."
          },
          {
              type: "tool_use",
              id: "toolu_01AbCdEfGhIjKlMnOpQrStU",
              name: "str_replace_based_edit_tool",
              input: {
                  command: "view",
                  path: "primes.py"
              }
          }
      ]
    },
    {
      role: "user",
      content: [
          {
              type: "tool_result",
              tool_use_id: "toolu_01AbCdEfGhIjKlMnOpQrStU",
              content: "1: def is_prime(n):\n2:     \"\"\"Check if a number is prime.\"\"\"\n3:     if n <= 1:\n4:         return False\n5:     if n <= 3:\n6:         return True\n7:     if n % 2 == 0 or n % 3 == 0:\n8:         return False\n9:     i = 5\n10:     while i * i <= n:\n11:         if n % i == 0 or n % (i + 2) == 0:\n12:             return False\n13:         i += 6\n14:     return True\n15: \n16: def get_primes(limit):\n17:     \"\"\"Generate a list of prime numbers up to the given limit.\"\"\"\n18:     primes = []\n19:     for num in range(2, limit + 1)\n20:         if is_prime(num):\n21:             primes.append(num)\n22:     return primes\n23: \n24: def main():\n25:     \"\"\"Main function to demonstrate prime number generation.\"\"\"\n26:     limit = 100\n27:     prime_list = get_primes(limit)\n28:     print(f\"Prime numbers up to {limit}:\")\n29:     print(prime_list)\n30:     print(f\"Found {len(prime_list)} prime numbers.\")\n31: \n32: if __name__ == \"__main__\":\n33:     main()"
          }
      ]
    }
  ]
});
```

```java Java
import com.anthropic.client.AnthropicClient;
import com.anthropic.client.okhttp.AnthropicOkHttpClient;
import com.anthropic.models.messages.Message;
import com.anthropic.models.messages.MessageCreateParams;
import com.anthropic.models.messages.Model;
import com.anthropic.models.messages.ToolStrReplaceBasedEditTool20250728;

public class TextEditorToolExample {

    public static void main(String[] args) {
        AnthropicClient client = AnthropicOkHttpClient.fromEnv();

        ToolStrReplaceBasedEditTool20250728 editorTool = ToolStrReplaceBasedEditTool20250728.builder()
            .build();

        MessageCreateParams params = MessageCreateParams.builder()
            .model(Model.CLAUDE_SONNET_4_0)
            .maxTokens(1024)
            .addTool(editorTool)
            .addUserMessage("There's a syntax error in my primes.py file. Can you help me fix it?")
            .build();

        Message message = client.messages().create(params);
        System.out.println(message);
    }
}
```
</CodeGroup>

<Tip>
**Numeri di riga**

Nell'esempio precedente, il risultato dello strumento `view` include il contenuto del file con i numeri di riga anteposti a ogni riga (ad esempio, "1: def is_prime(n):"). I numeri di riga non sono obbligatori, ma sono essenziali per utilizzare con successo il parametro `view_range` per esaminare sezioni specifiche di file e il parametro `insert_line` per aggiungere contenuto in posizioni precise.
</Tip>

Claude identificherà l'errore di sintassi e utilizzerà il comando `str_replace` per correggerlo:

```json
{
  "id": "msg_01VwXyZAbCdEfGhIjKlMnO",
  "model": "claude-sonnet-4-5",
  "stop_reason": "tool_use",
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "I found the syntax error in your primes.py file. In the `get_primes` function, there is a missing colon (:) at the end of the for loop line. Let me fix that for you."
    },
    {
      "type": "tool_use",
      "id": "toolu_01PqRsTuVwXyZAbCdEfGh",
      "name": "str_replace_based_edit_tool",
      "input": {
        "command": "str_replace",
        "path": "primes.py",
        "old_str": "    for num in range(2, limit + 1)",
        "new_str": "    for num in range(2, limit + 1):"
      }
    }
  ]
}
```

La tua applicazione dovrebbe quindi apportare la modifica e restituire il risultato:

<CodeGroup>
```python Python
response = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    tools=[
        {
            "type": "text_editor_20250728",
            "name": "str_replace_based_edit_tool"
        }
    ],
    messages=[
        # Previous messages...
        {
            "role": "assistant",
            "content": [
                {
                    "type": "text",
                    "text": "I found the syntax error in your primes.py file. In the `get_primes` function, there is a missing colon (:) at the end of the for loop line. Let me fix that for you."
                },
                {
                    "type": "tool_use",
                    "id": "toolu_01PqRsTuVwXyZAbCdEfGh",
                    "name": "str_replace_based_edit_tool",
                    "input": {
                        "command": "str_replace",
                        "path": "primes.py",
                        "old_str": "    for num in range(2, limit + 1)",
                        "new_str": "    for num in range(2, limit + 1):"
                    }
                }
            ]
        },
        {
            "role": "user",
            "content": [
                {
                    "type": "tool_result",
                    "tool_use_id": "toolu_01PqRsTuVwXyZAbCdEfGh",
                    "content": "Successfully replaced text at exactly one location."
                }
            ]
        }
    ]
)
```

```typescript TypeScript
const response = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  tools: [
    {
      type: "text_editor_20250728",
      name: "str_replace_based_edit_tool"
    }
  ],
  messages: [
    // Previous messages...
    {
      role: "assistant",
      content: [
        {
          type: "text",
          text: "I found the syntax error in your primes.py file. In the `get_primes` function, there is a missing colon (:) at the end of the for loop line. Let me fix that for you."
        },
        {
          type: "tool_use",
          id: "toolu_01PqRsTuVwXyZAbCdEfGh",
          name: "str_replace_based_edit_tool",
          input: {
            command: "str_replace",
            path: "primes.py",
            old_str: "    for num in range(2, limit + 1)",
            new_str: "    for num in range(2, limit + 1):"
          }
        }
      ]
    },
    {
      role: "user",
      content: [
        {
          type: "tool_result",
          tool_use_id: "toolu_01PqRsTuVwXyZAbCdEfGh",
          content: "Successfully replaced text at exactly one location."
        }
      ]
    }
  ]
});
```

```java Java
import java.util.List;
import java.util.Map;

import com.anthropic.client.AnthropicClient;
import com.anthropic.client.okhttp.AnthropicOkHttpClient;
import com.anthropic.core.JsonValue;
import com.anthropic.models.messages.ContentBlockParam;
import com.anthropic.models.messages.Message;
import com.anthropic.models.messages.MessageCreateParams;
import com.anthropic.models.messages.MessageParam;
import com.anthropic.models.messages.Model;
import com.anthropic.models.messages.TextBlockParam;
import com.anthropic.models.messages.ToolResultBlockParam;
import com.anthropic.models.messages.ToolStrReplaceBasedEditTool20250728;
import com.anthropic.models.messages.ToolUseBlockParam;

public class TextEditorConversationExample {

    public static void main(String[] args) {
        AnthropicClient client = AnthropicOkHttpClient.fromEnv();

        MessageCreateParams params = MessageCreateParams.builder()
                .model(Model.CLAUDE_SONNET_4_0)
                .maxTokens(1024)
                .addTool(ToolStrReplaceBasedEditTool20250728.builder()
                        .build())
                // Previous messages would go here
                .addAssistantMessageOfBlockParams(
                        List.of(
                                ContentBlockParam.ofText(TextBlockParam.builder()
                                        .text("I found the syntax error in your primes.py file. In the `get_primes` function, there is a missing colon (:) at the end of the for loop line. Let me fix that for you.")
                                        .build()),
                                ContentBlockParam.ofToolUse(ToolUseBlockParam.builder()
                                        .id("toolu_01PqRsTuVwXyZAbCdEfGh")
                                        .name("str_replace_based_edit_tool")
                                        .input(JsonValue.from(Map.of(
                                                "command", "str_replace",
                                                "path", "primes.py",
                                                "old_str", "    for num in range(2, limit + 1)",
                                                "new_str", "    for num in range(2, limit + 1):"
                                        )))
                                        .build()
                                )
                        )
                )
                .addUserMessageOfBlockParams(List.of(
                        ContentBlockParam.ofToolResult(ToolResultBlockParam.builder()
                                .toolUseId("toolu_01PqRsTuVwXyZAbCdEfGh")
                                .content("Successfully replaced text at exactly one location.")
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

Infine, Claude fornirà una spiegazione completa della correzione:

```json
{
  "id": "msg_01IjKlMnOpQrStUvWxYzAb",
  "model": "claude-sonnet-4-5",
  "stop_reason": "end_turn",
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "I've fixed the syntax error in your primes.py file. The issue was in the `get_primes` function at line 19. There was a missing colon (:) at the end of the for loop line.\n\nHere's what I changed:\n\nFrom:\n```python\nfor num in range(2, limit + 1)\n```\n\nTo:\n```python\nfor num in range(2, limit + 1):\n```\n\nIn Python, a colon is required at the end of statements that start a new code block, such as for loops, if statements, function definitions, etc. Without the colon, Python raises a syntax error because it doesn't know where the code block begins.\n\nYour code should now run correctly. The function will properly generate a list of prime numbers up to the specified limit."
    }
  ]
}
```
</Tab>

<Tab title="Claude Sonnet 3.7">
Questo esempio dimostra come Claude Sonnet 3.7 utilizza lo strumento editor di testo per correggere un errore di sintassi in un file Python.

Innanzitutto, la tua applicazione fornisce a Claude lo strumento editor di testo e un prompt per correggere un errore di sintassi:

```python
import anthropic

client = anthropic.Anthropic()

response = client.messages.create(
    model="claude-3-7-sonnet-20250219",
    max_tokens=1024,
    tools=[
        {
            "type": "text_editor_20250124",
            "name": "str_replace_editor"
        }
    ],
    messages=[
        {
            "role": "user", 
            "content": "There's a syntax error in my primes.py file. Can you help me fix it?"
        }
    ]
)

print(response)
```

<Note>
Gli esempi di Claude Sonnet 3.7 seguono lo stesso formato degli esempi di Claude 4 precedenti, utilizzando le stesse chiamate di strumento e risposte ma con il tipo di strumento `text_editor_20250124` e il nome `str_replace_editor`.
</Note>
</Tab>
</Tabs>

***

## Implementare lo strumento editor di testo

Lo strumento editor di testo è implementato come uno strumento senza schema. Quando utilizzi questo strumento, non è necessario fornire uno schema di input come con altri strumenti; lo schema è integrato nel modello di Claude e non può essere modificato.

Il tipo di strumento dipende dalla versione del modello:
- **Claude 4**: `type: "text_editor_20250728"`
- **Claude Sonnet 3.7**: `type: "text_editor_20250124"`

<Steps>
  <Step title="Inizializza l'implementazione dell'editor">
    Crea funzioni helper per gestire le operazioni sui file come la lettura, la scrittura e la modifica dei file. Considera l'implementazione della funzionalità di backup per recuperare da errori.
  </Step>
  <Step title="Gestisci le chiamate dello strumento editor">
    Crea una funzione che elabora le chiamate dello strumento da Claude in base al tipo di comando:
    ```python
    def handle_editor_tool(tool_call, model_version):
        input_params = tool_call.input
        command = input_params.get('command', '')
        file_path = input_params.get('path', '')
        
        if command == 'view':
            # Read and return file contents
            pass
        elif command == 'str_replace':
            # Replace text in file
            pass
        elif command == 'create':
            # Create new file
            pass
        elif command == 'insert':
            # Insert text at location
            pass
        elif command == 'undo_edit':
            # Check if it's a Claude 4 model
            if 'str_replace_based_edit_tool' in model_version:
                return {"error": "undo_edit command is not supported in Claude 4"}
            # Restore from backup for Claude 3.7
            pass
    ```
  </Step>
  <Step title="Implementa misure di sicurezza">
    Aggiungi controlli di validazione e sicurezza:
    - Valida i percorsi dei file per prevenire l'attraversamento di directory
    - Crea backup prima di apportare modifiche
    - Gestisci gli errori con eleganza
    - Implementa controlli delle autorizzazioni
  </Step>
  <Step title="Elabora le risposte di Claude">
    Estrai e gestisci le chiamate dello strumento dalle risposte di Claude:
    ```python
    # Process tool use in Claude's response
    for content in response.content:
        if content.type == "tool_use":
            # Execute the tool based on command
            result = handle_editor_tool(content)
            
            # Return result to Claude
            tool_result = {
                "type": "tool_result",
                "tool_use_id": content.id,
                "content": result
            }
    ```
  </Step>
</Steps>

<Warning>
Quando implementi lo strumento editor di testo, tieni presente:

1. **Sicurezza**: Lo strumento ha accesso al tuo file system locale, quindi implementa misure di sicurezza appropriate.
2. **Backup**: Crea sempre backup prima di consentire modifiche a file importanti.
3. **Validazione**: Valida tutti gli input per prevenire modifiche indesiderate.
4. **Corrispondenza univoca**: Assicurati che le sostituzioni corrispondano esattamente a una sola posizione per evitare modifiche indesiderate.
</Warning>

### Gestisci gli errori

Quando utilizzi lo strumento editor di testo, possono verificarsi vari errori. Ecco una guida su come gestirli:

<section title="File non trovato">

Se Claude tenta di visualizzare o modificare un file che non esiste, restituisci un messaggio di errore appropriato in `tool_result`:

```json
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": "Error: File not found",
      "is_error": true
    }
  ]
}
```

</section>

<section title="Corrispondenze multiple per la sostituzione">

Se il comando `str_replace` di Claude corrisponde a più posizioni nel file, restituisci un messaggio di errore appropriato:

```json
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": "Error: Found 3 matches for replacement text. Please provide more context to make a unique match.",
      "is_error": true
    }
  ]
}
```

</section>

<section title="Nessuna corrispondenza per la sostituzione">

Se il comando `str_replace` di Claude non corrisponde a nessun testo nel file, restituisci un messaggio di errore appropriato:

```json
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": "Error: No match found for replacement. Please check your text and try again.",
      "is_error": true
    }
  ]
}
```

</section>

<section title="Errori di autorizzazione">

Se ci sono problemi di autorizzazione con la creazione, la lettura o la modifica di file, restituisci un messaggio di errore appropriato:

```json
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": "Error: Permission denied. Cannot write to file.",
      "is_error": true
    }
  ]
}
```

</section>

### Segui le migliori pratiche di implementazione

<section title="Fornisci un contesto chiaro">

Quando chiedi a Claude di correggere o modificare il codice, sii specifico su quali file devono essere esaminati o quali problemi devono essere affrontati. Un contesto chiaro aiuta Claude a identificare i file giusti e a fare modifiche appropriate.

**Prompt meno utile**: "Puoi correggere il mio codice?"

**Prompt migliore**: "C'è un errore di sintassi nel mio file primes.py che impedisce l'esecuzione. Puoi correggerlo?"

</section>

<section title="Sii esplicito sui percorsi dei file">

Specifica chiaramente i percorsi dei file quando necessario, soprattutto se stai lavorando con più file o file in directory diverse.

**Prompt meno utile**: "Rivedi il mio file helper"

**Prompt migliore**: "Puoi controllare il mio file utils/helpers.py per eventuali problemi di prestazioni?"

</section>

<section title="Crea backup prima di modificare">

Implementa un sistema di backup nella tua applicazione che crea copie dei file prima di consentire a Claude di modificarli, soprattutto per codice importante o di produzione.

```python
def backup_file(file_path):
    """Create a backup of a file before editing."""
    backup_path = f"{file_path}.backup"
    if os.path.exists(file_path):
        with open(file_path, 'r') as src, open(backup_path, 'w') as dst:
            dst.write(src.read())
```

</section>

<section title="Gestisci con attenzione la sostituzione di testo univoco">

Il comando `str_replace` richiede una corrispondenza esatta per il testo da sostituire. La tua applicazione dovrebbe assicurare che ci sia esattamente una corrispondenza per il testo vecchio o fornire messaggi di errore appropriati.
```python
def safe_replace(file_path, old_text, new_text):
    """Replace text only if there's exactly one match."""
    with open(file_path, 'r') as f:
        content = f.read()
    
    count = content.count(old_text)
    if count == 0:
        return "Error: No match found"
    elif count > 1:
        return f"Error: Found {count} matches"
    else:
        new_content = content.replace(old_text, new_text)
        with open(file_path, 'w') as f:
            f.write(new_content)
        return "Successfully replaced text"
```

</section>

<section title="Verifica le modifiche">

Dopo che Claude apporta modifiche a un file, verifica le modifiche eseguendo test o controllando che il codice funzioni ancora come previsto.
```python
def verify_changes(file_path):
    """Run tests or checks after making changes."""
    try:
        # For Python files, check for syntax errors
        if file_path.endswith('.py'):
            import ast
            with open(file_path, 'r') as f:
                ast.parse(f.read())
            return "Syntax check passed"
    except Exception as e:
        return f"Verification failed: {str(e)}"
```

</section>

---

## Prezzi e utilizzo dei token

The text editor tool uses the same pricing structure as other tools used with Claude. It follows the standard input and output token pricing based on the Claude model you're using.

In addition to the base tokens, the following additional input tokens are needed for the text editor tool:

| Tool | Additional input tokens |
| ----------------------------------------- | --------------------------------------- |
| `text_editor_20250429` (Claude 4.x) | 700 tokens |
| `text_editor_20250124` (Claude Sonnet 3.7 ([deprecated](/docs/en/about-claude/model-deprecations))) | 700 tokens |

Per informazioni più dettagliate sui prezzi degli strumenti, vedi [Prezzi dell'utilizzo degli strumenti](/docs/it/agents-and-tools/tool-use/overview#pricing).

## Integra lo strumento editor di testo con altri strumenti

Lo strumento editor di testo può essere utilizzato insieme ad altri strumenti Claude. Quando combini gli strumenti, assicurati di:
- Abbinare la versione dello strumento al modello che stai utilizzando
- Tenere conto dell'utilizzo aggiuntivo dei token per tutti gli strumenti inclusi nella tua richiesta

## Registro delle modifiche

| Data | Versione | Modifiche |
| ---- | ------- | ------- |
| 28 luglio 2025 | `text_editor_20250728` | Rilascio di uno strumento editor di testo aggiornato che corregge alcuni problemi e aggiunge un parametro `max_characters` opzionale. È altrimenti identico a `text_editor_20250429`. |
| 29 aprile 2025 | `text_editor_20250429` | Rilascio dello strumento editor di testo per Claude 4. Questa versione rimuove il comando `undo_edit` ma mantiene tutte le altre funzionalità. Il nome dello strumento è stato aggiornato per riflettere la sua architettura basata su str_replace. |
| 13 marzo 2025 | `text_editor_20250124` | Introduzione della documentazione dello strumento editor di testo autonomo. Questa versione è ottimizzata per Claude Sonnet 3.7 ma ha funzionalità identiche alla versione precedente. |
| 22 ottobre 2024 | `text_editor_20241022` | Rilascio iniziale dello strumento editor di testo con Claude Sonnet 3.5 ([ritirato](/docs/it/about-claude/model-deprecations)). Fornisce funzionalità per visualizzare, creare e modificare file attraverso i comandi `view`, `create`, `str_replace`, `insert` e `undo_edit`. |

## Passaggi successivi

Ecco alcune idee su come utilizzare lo strumento editor di testo in modi più convenienti e potenti:

- **Integra con il tuo flusso di lavoro di sviluppo**: Integra lo strumento editor di testo nei tuoi strumenti di sviluppo o IDE
- **Crea un sistema di revisione del codice**: Fai in modo che Claude riveda il tuo codice e apporta miglioramenti
- **Crea un assistente di debug**: Crea un sistema in cui Claude può aiutarti a diagnosticare e risolvere i problemi nel tuo codice
- **Implementa la conversione del formato di file**: Consenti a Claude di aiutarti a convertire file da un formato a un altro
- **Automatizza la documentazione**: Configura flussi di lavoro affinché Claude documenti automaticamente il tuo codice

Mentre costruisci applicazioni con lo strumento editor di testo, siamo entusiasti di vedere come sfruttare le capacità di Claude per migliorare il tuo flusso di lavoro di sviluppo e la tua produttività.

<CardGroup cols={3}>
  <Card
    title="Panoramica dell'utilizzo degli strumenti"
    icon="wrench"
    href="/docs/it/agents-and-tools/tool-use/overview"
  >
    Scopri come implementare flussi di lavoro degli strumenti da utilizzare con Claude.
  </Card>

  <Card
    title="Strumento Bash"
    icon="terminal"
    href="/docs/it/agents-and-tools/tool-use/bash-tool"
  >
    Esegui comandi shell con Claude.
  </Card>
</CardGroup>