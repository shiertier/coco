# Code-Berater

Schlagen Sie Verbesserungen zur Optimierung der Python-Code-Performance vor.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Ihre Aufgabe ist es, den bereitgestellten Python-Code-Ausschnitt zu analysieren und Verbesserungen zur Optimierung seiner Performance vorzuschlagen. Identifizieren Sie Bereiche, in denen der Code effizienter, schneller oder weniger ressourcenintensiv gemacht werden kann. Geben Sie spezifische Optimierungsvorschläge zusammen mit Erklärungen darüber, wie diese Änderungen die Performance des Codes verbessern können. Der optimierte Code sollte die gleiche Funktionalität wie der ursprüngliche Code beibehalten und dabei verbesserte Effizienz demonstrieren. |
| User   | def fibonacci(n): <br/> if n \<= 0: <br/> return [] <br/> elif n \== 1: <br/> return [0] <br/> elif n \== 2: <br/> return [0, 1] <br/> else: <br/> fib = [0, 1] <br/> for i in range(2, n): <br/> fib.append(fib[i-1] + fib[i-2]) <br/> return fib |

## Beispielausgabe

```python Python
def fibonacci(n):
    a, b = 0, 1
    for _ in range(n):
        yield a
        a, b = b, a + b

```

Erklärung: Die Verwendung einer Generator-Funktion mit yield ist speichereffizienter für die Generierung der Fibonacci-Sequenz. Sie vermeidet das Erstellen und Speichern der gesamten Sequenz in einer Liste. Stattdessen generiert sie jede Zahl on-the-fly nach Bedarf.

---

## API-Anfrage

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
    system="Ihre Aufgabe ist es, den bereitgestellten Python-Code-Ausschnitt zu analysieren und Verbesserungen zur Optimierung seiner Performance vorzuschlagen. Identifizieren Sie Bereiche, in denen der Code effizienter, schneller oder weniger ressourcenintensiv gemacht werden kann. Geben Sie spezifische Optimierungsvorschläge zusammen mit Erklärungen darüber, wie diese Änderungen die Performance des Codes verbessern können. Der optimierte Code sollte die gleiche Funktionalität wie der ursprüngliche Code beibehalten und dabei verbesserte Effizienz demonstrieren.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "def fibonacci(n):\n if n <= 0:\n return []\n elif n == 1:\n return [0]\n elif n == 2:\n return [0, 1]\n else:\n fib = [0, 1]\n for i in range(2, n):\n fib.append(fib[i-1] + fib[i-2])\n return fib",
                }
            ],
        }
    ],
)
print(message.content)


````
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
  system: "Ihre Aufgabe ist es, den bereitgestellten Python-Code-Ausschnitt zu analysieren und Verbesserungen zur Optimierung seiner Performance vorzuschlagen. Identifizieren Sie Bereiche, in denen der Code effizienter, schneller oder weniger ressourcenintensiv gemacht werden kann. Geben Sie spezifische Optimierungsvorschläge zusammen mit Erklärungen darüber, wie diese Änderungen die Performance des Codes verbessern können. Der optimierte Code sollte die gleiche Funktionalität wie der ursprüngliche Code beibehalten und dabei verbesserte Effizienz demonstrieren.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "def fibonacci(n):\n    if n <= 0:\n        return []\n    elif n == 1:\n        return [0]\n    elif n == 2:\n        return [0, 1]\n    else:\n        fib = [0, 1]\n        for i in range(2, n):\n            fib.append(fib[i-1] + fib[i-2])\n    return fib"
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

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=1000,
    temperature=0,
    system="Ihre Aufgabe ist es, den bereitgestellten Python-Code-Ausschnitt zu analysieren und Verbesserungen zur Optimierung seiner Performance vorzuschlagen. Identifizieren Sie Bereiche, in denen der Code effizienter, schneller oder weniger ressourcenintensiv gemacht werden kann. Geben Sie spezifische Optimierungsvorschläge zusammen mit Erklärungen darüber, wie diese Änderungen die Performance des Codes verbessern können. Der optimierte Code sollte die gleiche Funktionalität wie der ursprüngliche Code beibehalten und dabei verbesserte Effizienz demonstrieren.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "def fibonacci(n):\n    if n <= 0:\n        return []\n    elif n == 1:\n        return [0]\n    elif n == 2:\n        return [0, 1]\n    else:\n        fib = [0, 1]\n        for i in range(2, n):\n            fib.append(fib[i-1] + fib[i-2])\n    return fib"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "Ihre Aufgabe ist es, den bereitgestellten Python-Code-Ausschnitt zu analysieren und Verbesserungen zur Optimierung seiner Performance vorzuschlagen. Identifizieren Sie Bereiche, in denen der Code effizienter, schneller oder weniger ressourcenintensiv gemacht werden kann. Geben Sie spezifische Optimierungsvorschläge zusammen mit Erklärungen darüber, wie diese Änderungen die Performance des Codes verbessern können. Der optimierte Code sollte die gleiche Funktionalität wie der ursprüngliche Code beibehalten und dabei verbesserte Effizienz demonstrieren.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "def fibonacci(n):\n    if n <= 0:\n        return []\n    elif n == 1:\n        return [0]\n    elif n == 2:\n        return [0, 1]\n    else:\n        fib = [0, 1]\n        for i in range(2, n):\n            fib.append(fib[i-1] + fib[i-2])\n    return fib"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=1000,
    temperature=0,
    system="Ihre Aufgabe ist es, den bereitgestellten Python-Code-Ausschnitt zu analysieren und Verbesserungen zur Optimierung seiner Performance vorzuschlagen. Identifizieren Sie Bereiche, in denen der Code effizienter, schneller oder weniger ressourcenintensiv gemacht werden kann. Geben Sie spezifische Optimierungsvorschläge zusammen mit Erklärungen darüber, wie diese Änderungen die Performance des Codes verbessern können. Der optimierte Code sollte die gleiche Funktionalität wie der ursprüngliche Code beibehalten und dabei verbesserte Effizienz demonstrieren.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "def fibonacci(n):\n    if n <= 0:\n        return []\n    elif n == 1:\n        return [0]\n    elif n == 2:\n        return [0, 1]\n    else:\n        fib = [0, 1]\n        for i in range(2, n):\n            fib.append(fib[i-1] + fib[i-2])\n    return fib"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="Vertex AI TypeScript">
```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
model: "claude-sonnet-4@20250514",
max_tokens: 1000,
temperature: 0,
system: "Ihre Aufgabe ist es, den bereitgestellten Python-Code-Ausschnitt zu analysieren und Verbesserungen zur Optimierung seiner Performance vorzuschlagen. Identifizieren Sie Bereiche, in denen der Code effizienter, schneller oder weniger ressourcenintensiv gemacht werden kann. Geben Sie spezifische Optimierungsvorschläge zusammen mit Erklärungen darüber, wie diese Änderungen die Performance des Codes verbessern können. Der optimierte Code sollte die gleiche Funktionalität wie der ursprüngliche Code beibehalten und dabei verbesserte Effizienz demonstrieren.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "def fibonacci(n):\n if n <= 0:\n return []\n elif n == 1:\n return [0]\n elif n == 2:\n return [0, 1]\n else:\n fib = [0, 1]\n for i in range(2, n):\n fib.append(fib[i-1] + fib[i-2])\n return fib"
}
]
}
]
});
console.log(msg);

```
</Tab>
</Tabs>
```