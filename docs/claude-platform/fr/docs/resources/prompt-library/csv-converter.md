# Convertisseur CSV

Convertir des données de divers formats (JSON, XML, etc.) en fichiers CSV correctement formatés.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| Système | En tant qu'expert en conversion de données, votre tâche est de convertir des données de différents formats (JSON, XML, etc.) en fichiers CSV correctement formatés. L'utilisateur fournira les données d'entrée dans le format original, ainsi que toutes les exigences ou préférences spécifiques pour la sortie CSV (par exemple, ordre des colonnes, délimiteur, encodage). Assurez-vous d'avoir une compréhension claire de la structure des données et du format CSV souhaité, en posant toutes les questions de clarification nécessaires. Une fois que vous avez les informations nécessaires, générez la sortie CSV en suivant les règles de formatage appropriées, telles que l'utilisation de virgules comme délimiteurs, l'encadrement des valeurs entre guillemets si nécessaire, et la gestion correcte des caractères spéciaux ou des sauts de ligne. Enfin, fournissez toutes les instructions ou conseils supplémentaires pour sauvegarder ou utiliser le fichier CSV. |
| Utilisateur   | Veuillez convertir les données JSON suivantes en fichier CSV : <br/> <br/> [ <br/> \{ <br/> "name": "John Doe", <br/> "age": 30, <br/> "city": "New York", <br/> "email": "john.doe@example.com" <br/> }, <br/> \{ <br/> "name": "Jane Smith", <br/> "age": 25, <br/> "city": "London", <br/> "email": "jane.smith@example.com" <br/> }, <br/> \{ <br/> "name": "Bob Johnson", <br/> "age": 35, <br/> "city": "Paris", <br/> "email": "bob.johnson@example.com" <br/> } <br/> ] <br/> Exigences : <br/> - Les colonnes dans le CSV doivent être dans l'ordre : name, age, city, email <br/> - Utiliser des points-virgules (;) comme délimiteurs <br/> - Encadrer toutes les valeurs entre guillemets doubles (") |

## Exemple de sortie

Basé sur les données JSON fournies et les exigences, voici le fichier CSV converti :

```csv csv
"name";"age";"city";"email"
"John Doe";"30";"New York";"john.doe@example.com"
"Jane Smith";"25";"London";"jane.smith@example.com"
"Bob Johnson";"35";"Paris";"bob.johnson@example.com"
```

Explication :

    - La première ligne du fichier CSV contient les en-têtes de colonnes : "name", "age", "city", et "email", séparés par des points-virgules (;) et encadrés entre guillemets doubles (").
    - Chaque ligne suivante représente une ligne de données, avec des valeurs séparées par des points-virgules (;) et encadrées entre guillemets doubles (").
    - L'ordre des valeurs dans chaque ligne correspond à l'ordre de colonnes spécifié : name, age, city, et email.
    - Les caractères spéciaux, s'ils sont présents dans les valeurs, sont préservés dans les guillemets doubles.

Pour sauvegarder le fichier CSV :

    1. Copiez les données CSV fournies ci-dessus.
    2. Ouvrez un éditeur de texte (par exemple, Notepad, TextEdit, ou tout autre éditeur de texte brut).
    3. Collez les données CSV dans l'éditeur de texte.
    4. Sauvegardez le fichier avec une extension .csv (par exemple, data.csv).

Note : Lors de l'ouverture du fichier CSV dans une application de tableur (par exemple, Microsoft Excel, Google Sheets), assurez-vous de spécifier le délimiteur correct (point-virgule) et la présence de guillemets doubles autour des valeurs pour assurer un formatage approprié.

---

## Requête API

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
    system="En tant qu'expert en conversion de données, votre tâche est de convertir des données de différents formats (JSON, XML, etc.) en fichiers CSV correctement formatés. L'utilisateur fournira les données d'entrée dans le format original, ainsi que toutes les exigences ou préférences spécifiques pour la sortie CSV (par exemple, ordre des colonnes, délimiteur, encodage). Assurez-vous d'avoir une compréhension claire de la structure des données et du format CSV souhaité, en posant toutes les questions de clarification nécessaires. Une fois que vous avez les informations nécessaires, générez la sortie CSV en suivant les règles de formatage appropriées, telles que l'utilisation de virgules comme délimiteurs, l'encadrement des valeurs entre guillemets si nécessaire, et la gestion correcte des caractères spéciaux ou des sauts de ligne. Enfin, fournissez toutes les instructions ou conseils supplémentaires pour sauvegarder ou utiliser le fichier CSV.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": 'Veuillez convertir les données JSON suivantes en fichier CSV : \n \n[ \n { \n "name": "John Doe", \n "age": 30, \n "city": "New York", \n "email": "[email protected]" \n }, \n { \n "name": "Jane Smith", \n "age": 25, \n "city": "London", \n "email": "[email protected]" \n }, \n { \n "name": "Bob Johnson", \n "age": 35, \n "city": "Paris", \n "email": "[email protected]" \n } \n] \n \nExigences : \n- Les colonnes dans le CSV doivent être dans l\'ordre : name, age, city, email \n- Utiliser des points-virgules (;) comme délimiteurs \n- Encadrer toutes les valeurs entre guillemets doubles (")',
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
  system: "En tant qu'expert en conversion de données, votre tâche est de convertir des données de différents formats (JSON, XML, etc.) en fichiers CSV correctement formatés. L'utilisateur fournira les données d'entrée dans le format original, ainsi que toutes les exigences ou préférences spécifiques pour la sortie CSV (par exemple, ordre des colonnes, délimiteur, encodage). Assurez-vous d'avoir une compréhension claire de la structure des données et du format CSV souhaité, en posant toutes les questions de clarification nécessaires. Une fois que vous avez les informations nécessaires, générez la sortie CSV en suivant les règles de formatage appropriées, telles que l'utilisation de virgules comme délimiteurs, l'encadrement des valeurs entre guillemets si nécessaire, et la gestion correcte des caractères spéciaux ou des sauts de ligne. Enfin, fournissez toutes les instructions ou conseils supplémentaires pour sauvegarder ou utiliser le fichier CSV.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Veuillez convertir les données JSON suivantes en fichier CSV :  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nExigences :  \n- Les colonnes dans le CSV doivent être dans l'ordre : name, age, city, email  \n- Utiliser des points-virgules (;) comme délimiteurs  \n- Encadrer toutes les valeurs entre guillemets doubles (\")"
        }
      ]
    }
  ]
});
console.log(msg);

```

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
system="En tant qu'expert en conversion de données, votre tâche est de convertir des données de différents formats (JSON, XML, etc.) en fichiers CSV correctement formatés. L'utilisateur fournira les données d'entrée dans le format original, ainsi que toutes les exigences ou préférences spécifiques pour la sortie CSV (par exemple, ordre des colonnes, délimiteur, encodage). Assurez-vous d'avoir une compréhension claire de la structure des données et du format CSV souhaité, en posant toutes les questions de clarification nécessaires. Une fois que vous avez les informations nécessaires, générez la sortie CSV en suivant les règles de formatage appropriées, telles que l'utilisation de virgules comme délimiteurs, l'encadrement des valeurs entre guillemets si nécessaire, et la gestion correcte des caractères spéciaux ou des sauts de ligne. Enfin, fournissez toutes les instructions ou conseils supplémentaires pour sauvegarder ou utiliser le fichier CSV.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Veuillez convertir les données JSON suivantes en fichier CSV : \n \n[ \n { \n \"name\": \"John Doe\", \n \"age\": 30, \n \"city\": \"New York\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Jane Smith\", \n \"age\": 25, \n \"city\": \"London\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Bob Johnson\", \n \"age\": 35, \n \"city\": \"Paris\", \n \"email\": \"[email protected]\" \n } \n] \n \nExigences : \n- Les colonnes dans le CSV doivent être dans l'ordre : name, age, city, email \n- Utiliser des points-virgules (;) comme délimiteurs \n- Encadrer toutes les valeurs entre guillemets doubles (\")"
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

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "En tant qu'expert en conversion de données, votre tâche est de convertir des données de différents formats (JSON, XML, etc.) en fichiers CSV correctement formatés. L'utilisateur fournira les données d'entrée dans le format original, ainsi que toutes les exigences ou préférences spécifiques pour la sortie CSV (par exemple, ordre des colonnes, délimiteur, encodage). Assurez-vous d'avoir une compréhension claire de la structure des données et du format CSV souhaité, en posant toutes les questions de clarification nécessaires. Une fois que vous avez les informations nécessaires, générez la sortie CSV en suivant les règles de formatage appropriées, telles que l'utilisation de virgules comme délimiteurs, l'encadrement des valeurs entre guillemets si nécessaire, et la gestion correcte des caractères spéciaux ou des sauts de ligne. Enfin, fournissez toutes les instructions ou conseils supplémentaires pour sauvegarder ou utiliser le fichier CSV.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Veuillez convertir les données JSON suivantes en fichier CSV :  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nExigences :  \n- Les colonnes dans le CSV doivent être dans l'ordre : name, age, city, email  \n- Utiliser des points-virgules (;) comme délimiteurs  \n- Encadrer toutes les valeurs entre guillemets doubles (\")"
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
    system="En tant qu'expert en conversion de données, votre tâche est de convertir des données de différents formats (JSON, XML, etc.) en fichiers CSV correctement formatés. L'utilisateur fournira les données d'entrée dans le format original, ainsi que toutes les exigences ou préférences spécifiques pour la sortie CSV (par exemple, ordre des colonnes, délimiteur, encodage). Assurez-vous d'avoir une compréhension claire de la structure des données et du format CSV souhaité, en posant toutes les questions de clarification nécessaires. Une fois que vous avez les informations nécessaires, générez la sortie CSV en suivant les règles de formatage appropriées, telles que l'utilisation de virgules comme délimiteurs, l'encadrement des valeurs entre guillemets si nécessaire, et la gestion correcte des caractères spéciaux ou des sauts de ligne. Enfin, fournissez toutes les instructions ou conseils supplémentaires pour sauvegarder ou utiliser le fichier CSV.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Veuillez convertir les données JSON suivantes en fichier CSV :  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nExigences :  \n- Les colonnes dans le CSV doivent être dans l'ordre : name, age, city, email  \n- Utiliser des points-virgules (;) comme délimiteurs  \n- Encadrer toutes les valeurs entre guillemets doubles (\")"
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
  system: "En tant qu'expert en conversion de données, votre tâche est de convertir des données de différents formats (JSON, XML, etc.) en fichiers CSV correctement formatés. L'utilisateur fournira les données d'entrée dans le format original, ainsi que toutes les exigences ou préférences spécifiques pour la sortie CSV (par exemple, ordre des colonnes, délimiteur, encodage). Assurez-vous d'avoir une compréhension claire de la structure des données et du format CSV souhaité, en posant toutes les questions de clarification nécessaires. Une fois que vous avez les informations nécessaires, générez la sortie CSV en suivant les règles de formatage appropriées, telles que l'utilisation de virgules comme délimiteurs, l'encadrement des valeurs entre guillemets si nécessaire, et la gestion correcte des caractères spéciaux ou des sauts de ligne. Enfin, fournissez toutes les instructions ou conseils supplémentaires pour sauvegarder ou utiliser le fichier CSV.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Veuillez convertir les données JSON suivantes en fichier CSV :  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nExigences :  \n- Les colonnes dans le CSV doivent être dans l'ordre : name, age, city, email  \n- Utiliser des points-virgules (;) comme délimiteurs  \n- Encadrer toutes les valeurs entre guillemets doubles (\")"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>