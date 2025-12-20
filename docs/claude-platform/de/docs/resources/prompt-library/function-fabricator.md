# Funktions-Fabrikator

Erstelle Python-Funktionen basierend auf detaillierten Spezifikationen.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Deine Aufgabe ist es, Python-Funktionen basierend auf den bereitgestellten natürlichsprachlichen Anfragen zu erstellen. Die Anfragen werden die gewünschte Funktionalität der Funktion beschreiben, einschließlich der Eingabeparameter und des erwarteten Rückgabewerts. Implementiere die Funktionen gemäß den gegebenen Spezifikationen und stelle sicher, dass sie Grenzfälle behandeln, notwendige Validierungen durchführen und bewährte Praktiken für Python-Programmierung befolgen. Bitte füge angemessene Kommentare in den Code ein, um die Logik zu erklären und anderen Entwicklern beim Verständnis der Implementierung zu helfen. |
| User   | Ich möchte eine Funktion, die ein Sudoku-Puzzle lösen kann. Die Funktion sollte ein 9x9 Sudoku-Gitter als Eingabe nehmen, wobei leere Zellen durch den Wert 0 dargestellt werden. Die Funktion sollte das Puzzle mit einem Backtracking-Algorithmus lösen und das gelöste Gitter zurückgeben. Wenn das Puzzle unlösbar ist, sollte sie None zurückgeben. Die Funktion sollte auch das Eingabegitter validieren, um sicherzustellen, dass es ein gültiges Sudoku-Puzzle ist. |

## Beispielausgabe

```python Python
def solve_sudoku(grid):
   def is_valid(row, col, num):
       # Überprüfe, ob die Zahl in der aktuellen Zeile und Spalte gültig ist
       for i in range(9):
           if grid[row][i] == num or grid[i][col] == num:
               return False

       # Überprüfe, ob die Zahl im aktuellen 3x3 Untergitter gültig ist
       start_row, start_col = (row // 3) * 3, (col // 3) * 3
       for i in range(3):
           for j in range(3):
               if grid[start_row + i][start_col + j] == num:
                   return False

       return True

   def backtrack(row, col):
       # Wenn wir alle Zellen gefüllt haben, ist das Puzzle gelöst
       if row == 9:
           return True

       # Gehe zur nächsten Zeile, wenn wir das Ende der aktuellen Zeile erreicht haben
       if col == 9:
           return backtrack(row + 1, 0)

       # Wenn die aktuelle Zelle bereits gefüllt ist, gehe zur nächsten Zelle
       if grid[row][col] != 0:
           return backtrack(row, col + 1)

       # Versuche die aktuelle Zelle mit Zahlen von 1 bis 9 zu füllen
       for num in range(1, 10):
           if is_valid(row, col, num):
               grid[row][col] = num
               if backtrack(row, col + 1):
                   return True
               grid[row][col] = 0

       # Wenn keine gültige Zahl gefunden wird, gehe zurück
       return False

   # Validiere das Eingabegitter
   if not all(len(row) == 9 and all(0 <= cell <= 9 for cell in row) for row in grid):
       return None

   # Starte den Backtracking-Algorithmus
   if backtrack(0, 0):
       return grid
   else:
       return None

```

---

## API-Anfrage

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # standardmäßig os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    system="Deine Aufgabe ist es, Python-Funktionen basierend auf den bereitgestellten natürlichsprachlichen Anfragen zu erstellen. Die Anfragen werden die gewünschte Funktionalität der Funktion beschreiben, einschließlich der Eingabeparameter und des erwarteten Rückgabewerts. Implementiere die Funktionen gemäß den gegebenen Spezifikationen und stelle sicher, dass sie Grenzfälle behandeln, notwendige Validierungen durchführen und bewährte Praktiken für Python-Programmierung befolgen. Bitte füge angemessene Kommentare in den Code ein, um die Logik zu erklären und anderen Entwicklern beim Verständnis der Implementierung zu helfen.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ich möchte eine Funktion, die ein Sudoku-Puzzle lösen kann. Die Funktion sollte ein 9x9 Sudoku-Gitter als Eingabe nehmen, wobei leere Zellen durch den Wert 0 dargestellt werden. Die Funktion sollte das Puzzle mit einem Backtracking-Algorithmus lösen und das gelöste Gitter zurückgeben. Wenn das Puzzle unlösbar ist, sollte sie None zurückgeben. Die Funktion sollte auch das Eingabegitter validieren, um sicherzustellen, dass es ein gültiges Sudoku-Puzzle ist.",
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
  apiKey: "my_api_key", // standardmäßig process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "Deine Aufgabe ist es, Python-Funktionen basierend auf den bereitgestellten natürlichsprachlichen Anfragen zu erstellen. Die Anfragen werden die gewünschte Funktionalität der Funktion beschreiben, einschließlich der Eingabeparameter und des erwarteten Rückgabewerts. Implementiere die Funktionen gemäß den gegebenen Spezifikationen und stelle sicher, dass sie Grenzfälle behandeln, notwendige Validierungen durchführen und bewährte Praktiken für Python-Programmierung befolgen. Bitte füge angemessene Kommentare in den Code ein, um die Logik zu erklären und anderen Entwicklern beim Verständnis der Implementierung zu helfen.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ich möchte eine Funktion, die ein Sudoku-Puzzle lösen kann. Die Funktion sollte ein 9x9 Sudoku-Gitter als Eingabe nehmen, wobei leere Zellen durch den Wert 0 dargestellt werden. Die Funktion sollte das Puzzle mit einem Backtracking-Algorithmus lösen und das gelöste Gitter zurückgeben. Wenn das Puzzle unlösbar ist, sollte sie None zurückgeben. Die Funktion sollte auch das Eingabegitter validieren, um sicherzustellen, dass es ein gültiges Sudoku-Puzzle ist."
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

# Siehe https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# für Authentifizierungsoptionen

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=1000,
temperature=0,
system="Deine Aufgabe ist es, Python-Funktionen basierend auf den bereitgestellten natürlichsprachlichen Anfragen zu erstellen. Die Anfragen werden die gewünschte Funktionalität der Funktion beschreiben, einschließlich der Eingabeparameter und des erwarteten Rückgabewerts. Implementiere die Funktionen gemäß den gegebenen Spezifikationen und stelle sicher, dass sie Grenzfälle behandeln, notwendige Validierungen durchführen und bewährte Praktiken für Python-Programmierung befolgen. Bitte füge angemessene Kommentare in den Code ein, um die Logik zu erklären und anderen Entwicklern beim Verständnis der Implementierung zu helfen.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Ich möchte eine Funktion, die ein Sudoku-Puzzle lösen kann. Die Funktion sollte ein 9x9 Sudoku-Gitter als Eingabe nehmen, wobei leere Zellen durch den Wert 0 dargestellt werden. Die Funktion sollte das Puzzle mit einem Backtracking-Algorithmus lösen und das gelöste Gitter zurückgeben. Wenn das Puzzle unlösbar ist, sollte sie None zurückgeben. Die Funktion sollte auch das Eingabegitter validieren, um sicherzustellen, dass es ein gültiges Sudoku-Puzzle ist."
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

// Siehe https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// für Authentifizierungsoptionen
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "Deine Aufgabe ist es, Python-Funktionen basierend auf den bereitgestellten natürlichsprachlichen Anfragen zu erstellen. Die Anfragen werden die gewünschte Funktionalität der Funktion beschreiben, einschließlich der Eingabeparameter und des erwarteten Rückgabewerts. Implementiere die Funktionen gemäß den gegebenen Spezifikationen und stelle sicher, dass sie Grenzfälle behandeln, notwendige Validierungen durchführen und bewährte Praktiken für Python-Programmierung befolgen. Bitte füge angemessene Kommentare in den Code ein, um die Logik zu erklären und anderen Entwicklern beim Verständnis der Implementierung zu helfen.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ich möchte eine Funktion, die ein Sudoku-Puzzle lösen kann. Die Funktion sollte ein 9x9 Sudoku-Gitter als Eingabe nehmen, wobei leere Zellen durch den Wert 0 dargestellt werden. Die Funktion sollte das Puzzle mit einem Backtracking-Algorithmus lösen und das gelöste Gitter zurückgeben. Wenn das Puzzle unlösbar ist, sollte sie None zurückgeben. Die Funktion sollte auch das Eingabegitter validieren, um sicherzustellen, dass es ein gültiges Sudoku-Puzzle ist."
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
system="Deine Aufgabe ist es, Python-Funktionen basierend auf den bereitgestellten natürlichsprachlichen Anfragen zu erstellen. Die Anfragen werden die gewünschte Funktionalität der Funktion beschreiben, einschließlich der Eingabeparameter und des erwarteten Rückgabewerts. Implementiere die Funktionen gemäß den gegebenen Spezifikationen und stelle sicher, dass sie Grenzfälle behandeln, notwendige Validierungen durchführen und bewährte Praktiken für Python-Programmierung befolgen. Bitte füge angemessene Kommentare in den Code ein, um die Logik zu erklären und anderen Entwicklern beim Verständnis der Implementierung zu helfen.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Ich möchte eine Funktion, die ein Sudoku-Puzzle lösen kann. Die Funktion sollte ein 9x9 Sudoku-Gitter als Eingabe nehmen, wobei leere Zellen durch den Wert 0 dargestellt werden. Die Funktion sollte das Puzzle mit einem Backtracking-Algorithmus lösen und das gelöste Gitter zurückgeben. Wenn das Puzzle unlösbar ist, sollte sie None zurückgeben. Die Funktion sollte auch das Eingabegitter validieren, um sicherzustellen, dass es ein gültiges Sudoku-Puzzle ist."
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

// Liest aus den Umgebungsvariablen `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID`.
// Durchläuft zusätzlich den Standard-`google-auth-library`-Ablauf.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 0,
  system: "Deine Aufgabe ist es, Python-Funktionen basierend auf den bereitgestellten natürlichsprachlichen Anfragen zu erstellen. Die Anfragen werden die gewünschte Funktionalität der Funktion beschreiben, einschließlich der Eingabeparameter und des erwarteten Rückgabewerts. Implementiere die Funktionen gemäß den gegebenen Spezifikationen und stelle sicher, dass sie Grenzfälle behandeln, notwendige Validierungen durchführen und bewährte Praktiken für Python-Programmierung befolgen. Bitte füge angemessene Kommentare in den Code ein, um die Logik zu erklären und anderen Entwicklern beim Verständnis der Implementierung zu helfen.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ich möchte eine Funktion, die ein Sudoku-Puzzle lösen kann. Die Funktion sollte ein 9x9 Sudoku-Gitter als Eingabe nehmen, wobei leere Zellen durch den Wert 0 dargestellt werden. Die Funktion sollte das Puzzle mit einem Backtracking-Algorithmus lösen und das gelöste Gitter zurückgeben. Wenn das Puzzle unlösbar ist, sollte sie None zurückgeben. Die Funktion sollte auch das Eingabegitter validieren, um sicherzustellen, dass es ein gültiges Sudoku-Puzzle ist."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>