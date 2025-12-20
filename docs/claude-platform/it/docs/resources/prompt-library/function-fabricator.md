# Fabbricatore di funzioni

Crea funzioni Python basate su specifiche dettagliate.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è creare funzioni Python basate sulle richieste in linguaggio naturale fornite. Le richieste descriveranno la funzionalità desiderata della funzione, inclusi i parametri di input e il valore di ritorno atteso. Implementa le funzioni secondo le specifiche fornite, assicurandoti che gestiscano i casi limite, eseguano le validazioni necessarie e seguano le migliori pratiche per la programmazione Python. Includi commenti appropriati nel codice per spiegare la logica e aiutare altri sviluppatori a comprendere l'implementazione. |
| User   | Voglio una funzione che possa risolvere un puzzle Sudoku. La funzione dovrebbe prendere una griglia Sudoku 9x9 come input, dove le celle vuote sono rappresentate dal valore 0. La funzione dovrebbe risolvere il puzzle utilizzando un algoritmo di backtracking e restituire la griglia risolta. Se il puzzle è irrisolvibile, dovrebbe restituire None. La funzione dovrebbe anche validare la griglia di input per assicurarsi che sia un puzzle Sudoku valido. |

## Output di Esempio

```python Python
def solve_sudoku(grid):
   def is_valid(row, col, num):
       # Controlla se il numero è valido nella riga e colonna correnti
       for i in range(9):
           if grid[row][i] == num or grid[i][col] == num:
               return False

       # Controlla se il numero è valido nella sottograiglia 3x3 corrente
       start_row, start_col = (row // 3) * 3, (col // 3) * 3
       for i in range(3):
           for j in range(3):
               if grid[start_row + i][start_col + j] == num:
                   return False

       return True

   def backtrack(row, col):
       # Se abbiamo riempito tutte le celle, il puzzle è risolto
       if row == 9:
           return True

       # Passa alla riga successiva se abbiamo raggiunto la fine della riga corrente
       if col == 9:
           return backtrack(row + 1, 0)

       # Se la cella corrente è già riempita, passa alla cella successiva
       if grid[row][col] != 0:
           return backtrack(row, col + 1)

       # Prova a riempire la cella corrente con numeri da 1 a 9
       for num in range(1, 10):
           if is_valid(row, col, num):
               grid[row][col] = num
               if backtrack(row, col + 1):
                   return True
               grid[row][col] = 0

       # Se non viene trovato nessun numero valido, torna indietro
       return False

   # Valida la griglia di input
   if not all(len(row) == 9 and all(0 <= cell <= 9 for cell in row) for row in grid):
       return None

   # Inizia l'algoritmo di backtracking
   if backtrack(0, 0):
       return grid
   else:
       return None

```

---

## Richiesta API

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    system="Il tuo compito è creare funzioni Python basate sulle richieste in linguaggio naturale fornite. Le richieste descriveranno la funzionalità desiderata della funzione, inclusi i parametri di input e il valore di ritorno atteso. Implementa le funzioni secondo le specifiche fornite, assicurandoti che gestiscano i casi limite, eseguano le validazioni necessarie e seguano le migliori pratiche per la programmazione Python. Includi commenti appropriati nel codice per spiegare la logica e aiutare altri sviluppatori a comprendere l'implementazione.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Voglio una funzione che possa risolvere un puzzle Sudoku. La funzione dovrebbe prendere una griglia Sudoku 9x9 come input, dove le celle vuote sono rappresentate dal valore 0. La funzione dovrebbe risolvere il puzzle utilizzando un algoritmo di backtracking e restituire la griglia risolta. Se il puzzle è irrisolvibile, dovrebbe restituire None. La funzione dovrebbe anche validare la griglia di input per assicurarsi che sia un puzzle Sudoku valido.",
                }
            ],
        }
    ],
)
print(message.content)


```
</Tab>
<Tab title="TypeScript">

```typescript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic({
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "Il tuo compito è creare funzioni Python basate sulle richieste in linguaggio naturale fornite. Le richieste descriveranno la funzionalità desiderata della funzione, inclusi i parametri di input e il valore di ritorno atteso. Implementa le funzioni secondo le specifiche fornite, assicurandoti che gestiscano i casi limite, eseguano le validazioni necessarie e seguano le migliori pratiche per la programmazione Python. Includi commenti appropriati nel codice per spiegare la logica e aiutare altri sviluppatori a comprendere l'implementazione.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Voglio una funzione che possa risolvere un puzzle Sudoku. La funzione dovrebbe prendere una griglia Sudoku 9x9 come input, dove le celle vuote sono rappresentate dal valore 0. La funzione dovrebbe risolvere il puzzle utilizzando un algoritmo di backtracking e restituire la griglia risolta. Se il puzzle è irrisolvibile, dovrebbe restituire None. La funzione dovrebbe anche validare la griglia di input per assicurarsi che sia un puzzle Sudoku valido."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">
```python
from anthropic import AnthropicBedrock

# Vedi https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# per le opzioni di autenticazione

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=1000,
temperature=0,
system="Il tuo compito è creare funzioni Python basate sulle richieste in linguaggio naturale fornite. Le richieste descriveranno la funzionalità desiderata della funzione, inclusi i parametri di input e il valore di ritorno atteso. Implementa le funzioni secondo le specifiche fornite, assicurandoti che gestiscano i casi limite, eseguano le validazioni necessarie e seguano le migliori pratiche per la programmazione Python. Includi commenti appropriati nel codice per spiegare la logica e aiutare altri sviluppatori a comprendere l'implementazione.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Voglio una funzione che possa risolvere un puzzle Sudoku. La funzione dovrebbe prendere una griglia Sudoku 9x9 come input, dove le celle vuote sono rappresentate dal valore 0. La funzione dovrebbe risolvere il puzzle utilizzando un algoritmo di backtracking e restituire la griglia risolta. Se il puzzle è irrisolvibile, dovrebbe restituire None. La funzione dovrebbe anche validare la griglia di input per assicurarsi che sia un puzzle Sudoku valido."
}
]
}
]
)
print(message.content)

````
</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// Vedi https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// per le opzioni di autenticazione
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "Il tuo compito è creare funzioni Python basate sulle richieste in linguaggio naturale fornite. Le richieste descriveranno la funzionalità desiderata della funzione, inclusi i parametri di input e il valore di ritorno atteso. Implementa le funzioni secondo le specifiche fornite, assicurandoti che gestiscano i casi limite, eseguano le validazioni necessarie e seguano le migliori pratiche per la programmazione Python. Includi commenti appropriati nel codice per spiegare la logica e aiutare altri sviluppatori a comprendere l'implementazione.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Voglio una funzione che possa risolvere un puzzle Sudoku. La funzione dovrebbe prendere una griglia Sudoku 9x9 come input, dove le celle vuote sono rappresentate dal valore 0. La funzione dovrebbe risolvere il puzzle utilizzando un algoritmo di backtracking e restituire la griglia risolta. Se il puzzle è irrisolvibile, dovrebbe restituire None. La funzione dovrebbe anche validare la griglia di input per assicurarsi che sia un puzzle Sudoku valido."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="Vertex AI Python">
```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
model="claude-sonnet-4@20250514",
max_tokens=1000,
temperature=0,
system="Il tuo compito è creare funzioni Python basate sulle richieste in linguaggio naturale fornite. Le richieste descriveranno la funzionalità desiderata della funzione, inclusi i parametri di input e il valore di ritorno atteso. Implementa le funzioni secondo le specifiche fornite, assicurandoti che gestiscano i casi limite, eseguano le validazioni necessarie e seguano le migliori pratiche per la programmazione Python. Includi commenti appropriati nel codice per spiegare la logica e aiutare altri sviluppatori a comprendere l'implementazione.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Voglio una funzione che possa risolvere un puzzle Sudoku. La funzione dovrebbe prendere una griglia Sudoku 9x9 come input, dove le celle vuote sono rappresentate dal valore 0. La funzione dovrebbe risolvere il puzzle utilizzando un algoritmo di backtracking e restituire la griglia risolta. Se il puzzle è irrisolvibile, dovrebbe restituire None. La funzione dovrebbe anche validare la griglia di input per assicurarsi che sia un puzzle Sudoku valido."
}
]
}
]
)
print(message.content)

````
</Tab>
<Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Legge dalle variabili d'ambiente `CLOUD_ML_REGION` e `ANTHROPIC_VERTEX_PROJECT_ID`.
// Inoltre segue il flusso standard di `google-auth-library`.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 0,
  system: "Il tuo compito è creare funzioni Python basate sulle richieste in linguaggio naturale fornite. Le richieste descriveranno la funzionalità desiderata della funzione, inclusi i parametri di input e il valore di ritorno atteso. Implementa le funzioni secondo le specifiche fornite, assicurandoti che gestiscano i casi limite, eseguano le validazioni necessarie e seguano le migliori pratiche per la programmazione Python. Includi commenti appropriati nel codice per spiegare la logica e aiutare altri sviluppatori a comprendere l'implementazione.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Voglio una funzione che possa risolvere un puzzle Sudoku. La funzione dovrebbe prendere una griglia Sudoku 9x9 come input, dove le celle vuote sono rappresentate dal valore 0. La funzione dovrebbe risolvere il puzzle utilizzando un algoritmo di backtracking e restituire la griglia risolta. Se il puzzle è irrisolvibile, dovrebbe restituire None. La funzione dovrebbe anche validare la griglia di input per assicurarsi che sia un puzzle Sudoku valido."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>