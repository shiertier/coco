# Consultant en code

Suggérer des améliorations pour optimiser les performances du code Python.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| Système | Votre tâche est d'analyser l'extrait de code Python fourni et de suggérer des améliorations pour optimiser ses performances. Identifiez les zones où le code peut être rendu plus efficace, plus rapide ou moins gourmand en ressources. Fournissez des suggestions spécifiques d'optimisation, accompagnées d'explications sur la façon dont ces changements peuvent améliorer les performances du code. Le code optimisé doit maintenir la même fonctionnalité que le code original tout en démontrant une efficacité améliorée. |
| Utilisateur   | def fibonacci(n): <br/> if n \<= 0: <br/> return [] <br/> elif n \== 1: <br/> return [0] <br/> elif n \== 2: <br/> return [0, 1] <br/> else: <br/> fib = [0, 1] <br/> for i in range(2, n): <br/> fib.append(fib[i-1] + fib[i-2]) <br/> return fib |

## Exemple de sortie

```python Python
def fibonacci(n):
    a, b = 0, 1
    for _ in range(n):
        yield a
        a, b = b, a + b

```

Explication : L'utilisation d'une fonction générateur avec yield est plus efficace en mémoire pour générer la séquence de Fibonacci. Elle évite de créer et de stocker toute la séquence dans une liste. Au lieu de cela, elle génère chaque nombre à la volée selon les besoins.

---

## Requête API

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # par défaut os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    system="Votre tâche est d'analyser l'extrait de code Python fourni et de suggérer des améliorations pour optimiser ses performances. Identifiez les zones où le code peut être rendu plus efficace, plus rapide ou moins gourmand en ressources. Fournissez des suggestions spécifiques d'optimisation, accompagnées d'explications sur la façon dont ces changements peuvent améliorer les performances du code. Le code optimisé doit maintenir la même fonctionnalité que le code original tout en démontrant une efficacité améliorée.",
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
  apiKey: "my_api_key", // par défaut process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "Votre tâche est d'analyser l'extrait de code Python fourni et de suggérer des améliorations pour optimiser ses performances. Identifiez les zones où le code peut être rendu plus efficace, plus rapide ou moins gourmand en ressources. Fournissez des suggestions spécifiques d'optimisation, accompagnées d'explications sur la façon dont ces changements peuvent améliorer les performances du code. Le code optimisé doit maintenir la même fonctionnalité que le code original tout en démontrant une efficacité améliorée.",
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

# Voir https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# pour les options d'authentification
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=1000,
    temperature=0,
    system="Votre tâche est d'analyser l'extrait de code Python fourni et de suggérer des améliorations pour optimiser ses performances. Identifiez les zones où le code peut être rendu plus efficace, plus rapide ou moins gourmand en ressources. Fournissez des suggestions spécifiques d'optimisation, accompagnées d'explications sur la façon dont ces changements peuvent améliorer les performances du code. Le code optimisé doit maintenir la même fonctionnalité que le code original tout en démontrant une efficacité améliorée.",
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

// Voir https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// pour les options d'authentification
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "Votre tâche est d'analyser l'extrait de code Python fourni et de suggérer des améliorations pour optimiser ses performances. Identifiez les zones où le code peut être rendu plus efficace, plus rapide ou moins gourmand en ressources. Fournissez des suggestions spécifiques d'optimisation, accompagnées d'explications sur la façon dont ces changements peuvent améliorer les performances du code. Le code optimisé doit maintenir la même fonctionnalité que le code original tout en démontrant une efficacité améliorée.",
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
    system="Votre tâche est d'analyser l'extrait de code Python fourni et de suggérer des améliorations pour optimiser ses performances. Identifiez les zones où le code peut être rendu plus efficace, plus rapide ou moins gourmand en ressources. Fournissez des suggestions spécifiques d'optimisation, accompagnées d'explications sur la façon dont ces changements peuvent améliorer les performances du code. Le code optimisé doit maintenir la même fonctionnalité que le code original tout en démontrant une efficacité améliorée.",
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

// Lit depuis les variables d'environnement `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID`.
// Passe également par le flux standard `google-auth-library`.
const client = new AnthropicVertex();

const msg = await client.messages.create({
model: "claude-sonnet-4@20250514",
max_tokens: 1000,
temperature: 0,
system: "Votre tâche est d'analyser l'extrait de code Python fourni et de suggérer des améliorations pour optimiser ses performances. Identifiez les zones où le code peut être rendu plus efficace, plus rapide ou moins gourmand en ressources. Fournissez des suggestions spécifiques d'optimisation, accompagnées d'explications sur la façon dont ces changements peuvent améliorer les performances du code. Le code optimisé doit maintenir la même fonctionnalité que le code original tout en démontrant une efficacité améliorée.",
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