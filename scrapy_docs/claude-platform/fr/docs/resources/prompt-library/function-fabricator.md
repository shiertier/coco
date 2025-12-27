# Fabricant de fonctions

Créer des fonctions Python basées sur des spécifications détaillées.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est de créer des fonctions Python basées sur les demandes en langage naturel fournies. Les demandes décriront la fonctionnalité souhaitée de la fonction, y compris les paramètres d'entrée et la valeur de retour attendue. Implémentez les fonctions selon les spécifications données, en vous assurant qu'elles gèrent les cas limites, effectuent les validations nécessaires et suivent les meilleures pratiques pour la programmation Python. Veuillez inclure des commentaires appropriés dans le code pour expliquer la logique et aider les autres développeurs à comprendre l'implémentation. |
| User   | Je veux une fonction qui peut résoudre un puzzle Sudoku. La fonction devrait prendre une grille Sudoku 9x9 en entrée, où les cellules vides sont représentées par la valeur 0. La fonction devrait résoudre le puzzle en utilisant un algorithme de retour en arrière et retourner la grille résolue. Si le puzzle est insoluble, elle devrait retourner None. La fonction devrait également valider la grille d'entrée pour s'assurer qu'elle est un puzzle Sudoku valide. |

## Exemple de sortie

```python Python
def solve_sudoku(grid):
   def is_valid(row, col, num):
       # Vérifier si le nombre est valide dans la ligne et la colonne actuelles
       for i in range(9):
           if grid[row][i] == num or grid[i][col] == num:
               return False

       # Vérifier si le nombre est valide dans la sous-grille 3x3 actuelle
       start_row, start_col = (row // 3) * 3, (col // 3) * 3
       for i in range(3):
           for j in range(3):
               if grid[start_row + i][start_col + j] == num:
                   return False

       return True

   def backtrack(row, col):
       # Si nous avons rempli toutes les cellules, le puzzle est résolu
       if row == 9:
           return True

       # Passer à la ligne suivante si nous avons atteint la fin de la ligne actuelle
       if col == 9:
           return backtrack(row + 1, 0)

       # Si la cellule actuelle est déjà remplie, passer à la cellule suivante
       if grid[row][col] != 0:
           return backtrack(row, col + 1)

       # Essayer de remplir la cellule actuelle avec des nombres de 1 à 9
       for num in range(1, 10):
           if is_valid(row, col, num):
               grid[row][col] = num
               if backtrack(row, col + 1):
                   return True
               grid[row][col] = 0

       # Si aucun nombre valide n'est trouvé, revenir en arrière
       return False

   # Valider la grille d'entrée
   if not all(len(row) == 9 and all(0 <= cell <= 9 for cell in row) for row in grid):
       return None

   # Commencer l'algorithme de retour en arrière
   if backtrack(0, 0):
       return grid
   else:
       return None

```

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
    system="Votre tâche est de créer des fonctions Python basées sur les demandes en langage naturel fournies. Les demandes décriront la fonctionnalité souhaitée de la fonction, y compris les paramètres d'entrée et la valeur de retour attendue. Implémentez les fonctions selon les spécifications données, en vous assurant qu'elles gèrent les cas limites, effectuent les validations nécessaires et suivent les meilleures pratiques pour la programmation Python. Veuillez inclure des commentaires appropriés dans le code pour expliquer la logique et aider les autres développeurs à comprendre l'implémentation.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Je veux une fonction qui peut résoudre un puzzle Sudoku. La fonction devrait prendre une grille Sudoku 9x9 en entrée, où les cellules vides sont représentées par la valeur 0. La fonction devrait résoudre le puzzle en utilisant un algorithme de retour en arrière et retourner la grille résolue. Si le puzzle est insoluble, elle devrait retourner None. La fonction devrait également valider la grille d'entrée pour s'assurer qu'elle est un puzzle Sudoku valide.",
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
  apiKey: "my_api_key", // par défaut process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "Votre tâche est de créer des fonctions Python basées sur les demandes en langage naturel fournies. Les demandes décriront la fonctionnalité souhaitée de la fonction, y compris les paramètres d'entrée et la valeur de retour attendue. Implémentez les fonctions selon les spécifications données, en vous assurant qu'elles gèrent les cas limites, effectuent les validations nécessaires et suivent les meilleures pratiques pour la programmation Python. Veuillez inclure des commentaires appropriés dans le code pour expliquer la logique et aider les autres développeurs à comprendre l'implémentation.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Je veux une fonction qui peut résoudre un puzzle Sudoku. La fonction devrait prendre une grille Sudoku 9x9 en entrée, où les cellules vides sont représentées par la valeur 0. La fonction devrait résoudre le puzzle en utilisant un algorithme de retour en arrière et retourner la grille résolue. Si le puzzle est insoluble, elle devrait retourner None. La fonction devrait également valider la grille d'entrée pour s'assurer qu'elle est un puzzle Sudoku valide."
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
system="Votre tâche est de créer des fonctions Python basées sur les demandes en langage naturel fournies. Les demandes décriront la fonctionnalité souhaitée de la fonction, y compris les paramètres d'entrée et la valeur de retour attendue. Implémentez les fonctions selon les spécifications données, en vous assurant qu'elles gèrent les cas limites, effectuent les validations nécessaires et suivent les meilleures pratiques pour la programmation Python. Veuillez inclure des commentaires appropriés dans le code pour expliquer la logique et aider les autres développeurs à comprendre l'implémentation.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Je veux une fonction qui peut résoudre un puzzle Sudoku. La fonction devrait prendre une grille Sudoku 9x9 en entrée, où les cellules vides sont représentées par la valeur 0. La fonction devrait résoudre le puzzle en utilisant un algorithme de retour en arrière et retourner la grille résolue. Si le puzzle est insoluble, elle devrait retourner None. La fonction devrait également valider la grille d'entrée pour s'assurer qu'elle est un puzzle Sudoku valide."
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

// Voir https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// pour les options d'authentification
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "Votre tâche est de créer des fonctions Python basées sur les demandes en langage naturel fournies. Les demandes décriront la fonctionnalité souhaitée de la fonction, y compris les paramètres d'entrée et la valeur de retour attendue. Implémentez les fonctions selon les spécifications données, en vous assurant qu'elles gèrent les cas limites, effectuent les validations nécessaires et suivent les meilleures pratiques pour la programmation Python. Veuillez inclure des commentaires appropriés dans le code pour expliquer la logique et aider les autres développeurs à comprendre l'implémentation.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Je veux une fonction qui peut résoudre un puzzle Sudoku. La fonction devrait prendre une grille Sudoku 9x9 en entrée, où les cellules vides sont représentées par la valeur 0. La fonction devrait résoudre le puzzle en utilisant un algorithme de retour en arrière et retourner la grille résolue. Si le puzzle est insoluble, elle devrait retourner None. La fonction devrait également valider la grille d'entrée pour s'assurer qu'elle est un puzzle Sudoku valide."
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
system="Votre tâche est de créer des fonctions Python basées sur les demandes en langage naturel fournies. Les demandes décriront la fonctionnalité souhaitée de la fonction, y compris les paramètres d'entrée et la valeur de retour attendue. Implémentez les fonctions selon les spécifications données, en vous assurant qu'elles gèrent les cas limites, effectuent les validations nécessaires et suivent les meilleures pratiques pour la programmation Python. Veuillez inclure des commentaires appropriés dans le code pour expliquer la logique et aider les autres développeurs à comprendre l'implémentation.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Je veux une fonction qui peut résoudre un puzzle Sudoku. La fonction devrait prendre une grille Sudoku 9x9 en entrée, où les cellules vides sont représentées par la valeur 0. La fonction devrait résoudre le puzzle en utilisant un algorithme de retour en arrière et retourner la grille résolue. Si le puzzle est insoluble, elle devrait retourner None. La fonction devrait également valider la grille d'entrée pour s'assurer qu'elle est un puzzle Sudoku valide."
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

// Lit à partir des variables d'environnement `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID`.
// Passe également par le flux standard `google-auth-library`.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 0,
  system: "Votre tâche est de créer des fonctions Python basées sur les demandes en langage naturel fournies. Les demandes décriront la fonctionnalité souhaitée de la fonction, y compris les paramètres d'entrée et la valeur de retour attendue. Implémentez les fonctions selon les spécifications données, en vous assurant qu'elles gèrent les cas limites, effectuent les validations nécessaires et suivent les meilleures pratiques pour la programmation Python. Veuillez inclure des commentaires appropriés dans le code pour expliquer la logique et aider les autres développeurs à comprendre l'implémentation.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Je veux une fonction qui peut résoudre un puzzle Sudoku. La fonction devrait prendre une grille Sudoku 9x9 en entrée, où les cellules vides sont représentées par la valeur 0. La fonction devrait résoudre le puzzle en utilisant un algorithme de retour en arrière et retourner la grille résolue. Si le puzzle est insoluble, elle devrait retourner None. La fonction devrait également valider la grille d'entrée pour s'assurer qu'elle est un puzzle Sudoku valide."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>