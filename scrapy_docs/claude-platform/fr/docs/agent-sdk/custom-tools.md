# Outils Personnalisés

Créez et intégrez des outils personnalisés pour étendre les fonctionnalités du Claude Agent SDK

---

Les outils personnalisés vous permettent d'étendre les capacités de Claude Code avec vos propres fonctionnalités via des serveurs MCP en processus, permettant à Claude d'interagir avec des services externes, des API, ou d'effectuer des opérations spécialisées.

## Création d'Outils Personnalisés

Utilisez les fonctions d'aide `createSdkMcpServer` et `tool` pour définir des outils personnalisés type-safe :

<CodeGroup>

```typescript TypeScript
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

// Créer un serveur MCP SDK avec des outils personnalisés
const customServer = createSdkMcpServer({
  name: "my-custom-tools",
  version: "1.0.0",
  tools: [
    tool(
      "get_weather",
      "Obtenir la température actuelle d'un lieu en utilisant les coordonnées",
      {
        latitude: z.number().describe("Coordonnée de latitude"),
        longitude: z.number().describe("Coordonnée de longitude")
      },
      async (args) => {
        const response = await fetch(`https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&current=temperature_2m&temperature_unit=fahrenheit`);
        const data = await response.json();

        return {
          content: [{
            type: "text",
            text: `Température : ${data.current.temperature_2m}°F`
          }]
        };
      }
    )
  ]
});
```

```python Python
from claude_agent_sdk import tool, create_sdk_mcp_server, ClaudeSDKClient, ClaudeAgentOptions
from typing import Any
import aiohttp

# Définir un outil personnalisé en utilisant le décorateur @tool
@tool("get_weather", "Obtenir la température actuelle d'un lieu en utilisant les coordonnées", {"latitude": float, "longitude": float})
async def get_weather(args: dict[str, Any]) -> dict[str, Any]:
    # Appeler l'API météo
    async with aiohttp.ClientSession() as session:
        async with session.get(
            f"https://api.open-meteo.com/v1/forecast?latitude={args['latitude']}&longitude={args['longitude']}&current=temperature_2m&temperature_unit=fahrenheit"
        ) as response:
            data = await response.json()

    return {
        "content": [{
            "type": "text",
            "text": f"Température : {data['current']['temperature_2m']}°F"
        }]
    }

# Créer un serveur MCP SDK avec l'outil personnalisé
custom_server = create_sdk_mcp_server(
    name="my-custom-tools",
    version="1.0.0",
    tools=[get_weather]  # Passer la fonction décorée
)
```

</CodeGroup>

## Utilisation d'Outils Personnalisés

Passez le serveur personnalisé à la fonction `query` via l'option `mcpServers` comme un dictionnaire/objet.

<Note>
**Important :** Les outils MCP personnalisés nécessitent le mode d'entrée en streaming. Vous devez utiliser un générateur/itérable asynchrone pour le paramètre `prompt` - une simple chaîne ne fonctionnera pas avec les serveurs MCP.
</Note>

### Format des Noms d'Outils

Lorsque les outils MCP sont exposés à Claude, leurs noms suivent un format spécifique :
- Modèle : `mcp__{server_name}__{tool_name}`
- Exemple : Un outil nommé `get_weather` dans le serveur `my-custom-tools` devient `mcp__my-custom-tools__get_weather`

### Configuration des Outils Autorisés

Vous pouvez contrôler quels outils Claude peut utiliser via l'option `allowedTools` :

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Utiliser les outils personnalisés dans votre requête avec entrée en streaming
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "Quel temps fait-il à San Francisco ?"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // Utiliser un générateur asynchrone pour l'entrée en streaming
  options: {
    mcpServers: {
      "my-custom-tools": customServer  // Passer comme objet/dictionnaire, pas comme tableau
    },
    // Spécifier optionnellement quels outils Claude peut utiliser
    allowedTools: [
      "mcp__my-custom-tools__get_weather",  // Autoriser l'outil météo
      // Ajouter d'autres outils selon les besoins
    ],
    maxTurns: 3
  }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

```python Python
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions
import asyncio

# Utiliser les outils personnalisés avec Claude
options = ClaudeAgentOptions(
    mcp_servers={"my-custom-tools": custom_server},
    allowed_tools=[
        "mcp__my-custom-tools__get_weather",  # Autoriser l'outil météo
        # Ajouter d'autres outils selon les besoins
    ]
)

async def main():
    async with ClaudeSDKClient(options=options) as client:
        await client.query("Quel temps fait-il à San Francisco ?")

        # Extraire et imprimer la réponse
        async for msg in client.receive_response():
            print(msg)

asyncio.run(main())
```

</CodeGroup>

### Exemple avec Plusieurs Outils

Lorsque votre serveur MCP a plusieurs outils, vous pouvez les autoriser sélectivement :

<CodeGroup>

```typescript TypeScript
const multiToolServer = createSdkMcpServer({
  name: "utilities",
  version: "1.0.0",
  tools: [
    tool("calculate", "Effectuer des calculs", { /* ... */ }, async (args) => { /* ... */ }),
    tool("translate", "Traduire du texte", { /* ... */ }, async (args) => { /* ... */ }),
    tool("search_web", "Rechercher sur le web", { /* ... */ }, async (args) => { /* ... */ })
  ]
});

// Autoriser seulement des outils spécifiques avec entrée en streaming
async function* generateMessages() {
  yield {
    type: "user" as const,
    message: {
      role: "user" as const,
      content: "Calcule 5 + 3 et traduis 'bonjour' en espagnol"
    }
  };
}

for await (const message of query({
  prompt: generateMessages(),  // Utiliser un générateur asynchrone pour l'entrée en streaming
  options: {
    mcpServers: {
      utilities: multiToolServer
    },
    allowedTools: [
      "mcp__utilities__calculate",   // Autoriser la calculatrice
      "mcp__utilities__translate",   // Autoriser le traducteur
      // "mcp__utilities__search_web" n'est PAS autorisé
    ]
  }
})) {
  // Traiter les messages
}
```

```python Python
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, tool, create_sdk_mcp_server
from typing import Any
import asyncio

# Définir plusieurs outils en utilisant le décorateur @tool
@tool("calculate", "Effectuer des calculs", {"expression": str})
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    result = eval(args["expression"])  # Utiliser un eval sécurisé en production
    return {"content": [{"type": "text", "text": f"Résultat : {result}"}]}

@tool("translate", "Traduire du texte", {"text": str, "target_lang": str})
async def translate(args: dict[str, Any]) -> dict[str, Any]:
    # Logique de traduction ici
    return {"content": [{"type": "text", "text": f"Traduit : {args['text']}"}]}

@tool("search_web", "Rechercher sur le web", {"query": str})
async def search_web(args: dict[str, Any]) -> dict[str, Any]:
    # Logique de recherche ici
    return {"content": [{"type": "text", "text": f"Résultats de recherche pour : {args['query']}"}]}

multi_tool_server = create_sdk_mcp_server(
    name="utilities",
    version="1.0.0",
    tools=[calculate, translate, search_web]  # Passer les fonctions décorées
)

# Autoriser seulement des outils spécifiques avec entrée en streaming
async def message_generator():
    yield {
        "type": "user",
        "message": {
            "role": "user",
            "content": "Calcule 5 + 3 et traduis 'bonjour' en espagnol"
        }
    }

async for message in query(
    prompt=message_generator(),  # Utiliser un générateur asynchrone pour l'entrée en streaming
    options=ClaudeAgentOptions(
        mcp_servers={"utilities": multi_tool_server},
        allowed_tools=[
            "mcp__utilities__calculate",   # Autoriser la calculatrice
            "mcp__utilities__translate",   # Autoriser le traducteur
            # "mcp__utilities__search_web" n'est PAS autorisé
        ]
    )
):
    if hasattr(message, 'result'):
        print(message.result)
```

</CodeGroup>

## Sécurité de Type avec Python

Le décorateur `@tool` prend en charge diverses approches de définition de schéma pour la sécurité de type :

<CodeGroup>

```typescript TypeScript
import { z } from "zod";

tool(
  "process_data",
  "Traiter des données structurées avec sécurité de type",
  {
    // Le schéma Zod définit à la fois la validation d'exécution et les types TypeScript
    data: z.object({
      name: z.string(),
      age: z.number().min(0).max(150),
      email: z.string().email(),
      preferences: z.array(z.string()).optional()
    }),
    format: z.enum(["json", "csv", "xml"]).default("json")
  },
  async (args) => {
    // args est entièrement typé basé sur le schéma
    // TypeScript sait : args.data.name est string, args.data.age est number, etc.
    console.log(`Traitement des données de ${args.data.name} en tant que ${args.format}`);
    
    // Votre logique de traitement ici
    return {
      content: [{
        type: "text",
        text: `Données traitées pour ${args.data.name}`
      }]
    };
  }
)
```

```python Python
from typing import Any

# Mappage de type simple - recommandé pour la plupart des cas
@tool(
    "process_data",
    "Traiter des données structurées avec sécurité de type",
    {
        "name": str,
        "age": int,
        "email": str,
        "preferences": list  # Les paramètres optionnels peuvent être gérés dans la fonction
    }
)
async def process_data(args: dict[str, Any]) -> dict[str, Any]:
    # Accéder aux arguments avec des indices de type pour le support IDE
    name = args["name"]
    age = args["age"]
    email = args["email"]
    preferences = args.get("preferences", [])
    
    print(f"Traitement des données de {name} (âge : {age})")
    
    return {
        "content": [{
            "type": "text",
            "text": f"Données traitées pour {name}"
        }]
    }

# Pour des schémas plus complexes, vous pouvez utiliser le format JSON Schema
@tool(
    "advanced_process",
    "Traiter des données avec validation avancée",
    {
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "integer", "minimum": 0, "maximum": 150},
            "email": {"type": "string", "format": "email"},
            "format": {"type": "string", "enum": ["json", "csv", "xml"], "default": "json"}
        },
        "required": ["name", "age", "email"]
    }
)
async def advanced_process(args: dict[str, Any]) -> dict[str, Any]:
    # Traiter avec validation de schéma avancée
    return {
        "content": [{
            "type": "text",
            "text": f"Traitement avancé pour {args['name']}"
        }]
    }
```

</CodeGroup>

## Gestion des Erreurs

Gérez les erreurs avec élégance pour fournir un retour significatif :

<CodeGroup>

```typescript TypeScript
tool(
  "fetch_data",
  "Récupérer des données depuis une API",
  {
    endpoint: z.string().url().describe("URL du point de terminaison API")
  },
  async (args) => {
    try {
      const response = await fetch(args.endpoint);
      
      if (!response.ok) {
        return {
          content: [{
            type: "text",
            text: `Erreur API : ${response.status} ${response.statusText}`
          }]
        };
      }
      
      const data = await response.json();
      return {
        content: [{
          type: "text",
          text: JSON.stringify(data, null, 2)
        }]
      };
    } catch (error) {
      return {
        content: [{
          type: "text",
          text: `Échec de récupération des données : ${error.message}`
        }]
      };
    }
  }
)
```

```python Python
import json
import aiohttp
from typing import Any

@tool(
    "fetch_data",
    "Récupérer des données depuis une API",
    {"endpoint": str}  # Schéma simple
)
async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
    try:
        async with aiohttp.ClientSession() as session:
            async with session.get(args["endpoint"]) as response:
                if response.status != 200:
                    return {
                        "content": [{
                            "type": "text",
                            "text": f"Erreur API : {response.status} {response.reason}"
                        }]
                    }
                
                data = await response.json()
                return {
                    "content": [{
                        "type": "text",
                        "text": json.dumps(data, indent=2)
                    }]
                }
    except Exception as e:
        return {
            "content": [{
                "type": "text",
                "text": f"Échec de récupération des données : {str(e)}"
            }]
        }
```

</CodeGroup>

## Exemples d'Outils

### Outil de Requête de Base de Données

<CodeGroup>

```typescript TypeScript
const databaseServer = createSdkMcpServer({
  name: "database-tools",
  version: "1.0.0",
  tools: [
    tool(
      "query_database",
      "Exécuter une requête de base de données",
      {
        query: z.string().describe("Requête SQL à exécuter"),
        params: z.array(z.any()).optional().describe("Paramètres de requête")
      },
      async (args) => {
        const results = await db.query(args.query, args.params || []);
        return {
          content: [{
            type: "text",
            text: `Trouvé ${results.length} lignes :\n${JSON.stringify(results, null, 2)}`
          }]
        };
      }
    )
  ]
});
```

```python Python
from typing import Any
import json

@tool(
    "query_database",
    "Exécuter une requête de base de données",
    {"query": str, "params": list}  # Schéma simple avec type list
)
async def query_database(args: dict[str, Any]) -> dict[str, Any]:
    results = await db.query(args["query"], args.get("params", []))
    return {
        "content": [{
            "type": "text",
            "text": f"Trouvé {len(results)} lignes :\n{json.dumps(results, indent=2)}"
        }]
    }

database_server = create_sdk_mcp_server(
    name="database-tools",
    version="1.0.0",
    tools=[query_database]  # Passer la fonction décorée
)
```

</CodeGroup>

### Outil de Passerelle API

<CodeGroup>

```typescript TypeScript
const apiGatewayServer = createSdkMcpServer({
  name: "api-gateway",
  version: "1.0.0",
  tools: [
    tool(
      "api_request",
      "Effectuer des requêtes API authentifiées vers des services externes",
      {
        service: z.enum(["stripe", "github", "openai", "slack"]).describe("Service à appeler"),
        endpoint: z.string().describe("Chemin du point de terminaison API"),
        method: z.enum(["GET", "POST", "PUT", "DELETE"]).describe("Méthode HTTP"),
        body: z.record(z.any()).optional().describe("Corps de la requête"),
        query: z.record(z.string()).optional().describe("Paramètres de requête")
      },
      async (args) => {
        const config = {
          stripe: { baseUrl: "https://api.stripe.com/v1", key: process.env.STRIPE_KEY },
          github: { baseUrl: "https://api.github.com", key: process.env.GITHUB_TOKEN },
          openai: { baseUrl: "https://api.openai.com/v1", key: process.env.OPENAI_KEY },
          slack: { baseUrl: "https://slack.com/api", key: process.env.SLACK_TOKEN }
        };
        
        const { baseUrl, key } = config[args.service];
        const url = new URL(`${baseUrl}${args.endpoint}`);
        
        if (args.query) {
          Object.entries(args.query).forEach(([k, v]) => url.searchParams.set(k, v));
        }
        
        const response = await fetch(url, {
          method: args.method,
          headers: { Authorization: `Bearer ${key}`, "Content-Type": "application/json" },
          body: args.body ? JSON.stringify(args.body) : undefined
        });
        
        const data = await response.json();
        return {
          content: [{
            type: "text",
            text: JSON.stringify(data, null, 2)
          }]
        };
      }
    )
  ]
});
```

```python Python
import os
import json
import aiohttp
from typing import Any

# Pour des schémas complexes avec énumérations, utiliser le format JSON Schema
@tool(
    "api_request",
    "Effectuer des requêtes API authentifiées vers des services externes",
    {
        "type": "object",
        "properties": {
            "service": {"type": "string", "enum": ["stripe", "github", "openai", "slack"]},
            "endpoint": {"type": "string"},
            "method": {"type": "string", "enum": ["GET", "POST", "PUT", "DELETE"]},
            "body": {"type": "object"},
            "query": {"type": "object"}
        },
        "required": ["service", "endpoint", "method"]
    }
)
async def api_request(args: dict[str, Any]) -> dict[str, Any]:
    config = {
        "stripe": {"base_url": "https://api.stripe.com/v1", "key": os.environ["STRIPE_KEY"]},
        "github": {"base_url": "https://api.github.com", "key": os.environ["GITHUB_TOKEN"]},
        "openai": {"base_url": "https://api.openai.com/v1", "key": os.environ["OPENAI_KEY"]},
        "slack": {"base_url": "https://slack.com/api", "key": os.environ["SLACK_TOKEN"]}
    }
    
    service_config = config[args["service"]]
    url = f"{service_config['base_url']}{args['endpoint']}"
    
    if args.get("query"):
        params = "&".join([f"{k}={v}" for k, v in args["query"].items()])
        url += f"?{params}"
    
    headers = {"Authorization": f"Bearer {service_config['key']}", "Content-Type": "application/json"}
    
    async with aiohttp.ClientSession() as session:
        async with session.request(
            args["method"], url, headers=headers, json=args.get("body")
        ) as response:
            data = await response.json()
            return {
                "content": [{
                    "type": "text",
                    "text": json.dumps(data, indent=2)
                }]
            }

api_gateway_server = create_sdk_mcp_server(
    name="api-gateway",
    version="1.0.0",
    tools=[api_request]  # Passer la fonction décorée
)
```

</CodeGroup>

### Outil Calculatrice

<CodeGroup>

```typescript TypeScript
const calculatorServer = createSdkMcpServer({
  name: "calculator",
  version: "1.0.0",
  tools: [
    tool(
      "calculate",
      "Effectuer des calculs mathématiques",
      {
        expression: z.string().describe("Expression mathématique à évaluer"),
        precision: z.number().optional().default(2).describe("Précision décimale")
      },
      async (args) => {
        try {
          // Utiliser une bibliothèque d'évaluation mathématique sécurisée en production
          const result = eval(args.expression); // Exemple seulement !
          const formatted = Number(result).toFixed(args.precision);
          
          return {
            content: [{
              type: "text",
              text: `${args.expression} = ${formatted}`
            }]
          };
        } catch (error) {
          return {
            content: [{
              type: "text",
              text: `Erreur : Expression invalide - ${error.message}`
            }]
          };
        }
      }
    ),
    tool(
      "compound_interest",
      "Calculer les intérêts composés pour un investissement",
      {
        principal: z.number().positive().describe("Montant d'investissement initial"),
        rate: z.number().describe("Taux d'intérêt annuel (en décimal, ex : 0,05 pour 5%)"),
        time: z.number().positive().describe("Période d'investissement en années"),
        n: z.number().positive().default(12).describe("Fréquence de composition par an")
      },
      async (args) => {
        const amount = args.principal * Math.pow(1 + args.rate / args.n, args.n * args.time);
        const interest = amount - args.principal;
        
        return {
          content: [{
            type: "text",
            text: `Analyse d'Investissement :\n` +
                  `Principal : ${args.principal.toFixed(2)}$\n` +
                  `Taux : ${(args.rate * 100).toFixed(2)}%\n` +
                  `Durée : ${args.time} années\n` +
                  `Composition : ${args.n} fois par an\n\n` +
                  `Montant Final : ${amount.toFixed(2)}$\n` +
                  `Intérêts Gagnés : ${interest.toFixed(2)}$\n` +
                  `Rendement : ${((interest / args.principal) * 100).toFixed(2)}%`
          }]
        };
      }
    )
  ]
});
```

```python Python
import math
from typing import Any

@tool(
    "calculate",
    "Effectuer des calculs mathématiques",
    {"expression": str, "precision": int}  # Schéma simple
)
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    try:
        # Utiliser une bibliothèque d'évaluation mathématique sécurisée en production
        result = eval(args["expression"], {"__builtins__": {}})
        precision = args.get("precision", 2)
        formatted = round(result, precision)
        
        return {
            "content": [{
                "type": "text",
                "text": f"{args['expression']} = {formatted}"
            }]
        }
    except Exception as e:
        return {
            "content": [{
                "type": "text",
                "text": f"Erreur : Expression invalide - {str(e)}"
            }]
        }

@tool(
    "compound_interest",
    "Calculer les intérêts composés pour un investissement",
    {"principal": float, "rate": float, "time": float, "n": int}
)
async def compound_interest(args: dict[str, Any]) -> dict[str, Any]:
    principal = args["principal"]
    rate = args["rate"]
    time = args["time"]
    n = args.get("n", 12)
    
    amount = principal * (1 + rate / n) ** (n * time)
    interest = amount - principal
    
    return {
        "content": [{
            "type": "text",
            "text": f"""Analyse d'Investissement :
Principal : {principal:.2f}$
Taux : {rate * 100:.2f}%
Durée : {time} années
Composition : {n} fois par an

Montant Final : {amount:.2f}$
Intérêts Gagnés : {interest:.2f}$
Rendement : {(interest / principal) * 100:.2f}%"""
        }]
    }

calculator_server = create_sdk_mcp_server(
    name="calculator",
    version="1.0.0",
    tools=[calculate, compound_interest]  # Passer les fonctions décorées
)
```

</CodeGroup>

## Documentation Connexe

- [Référence SDK TypeScript](/docs/fr/agent-sdk/typescript)
- [Référence SDK Python](/docs/fr/agent-sdk/python)
- [Documentation MCP](https://modelcontextprotocol.io)
- [Aperçu du SDK](/docs/fr/agent-sdk/overview)