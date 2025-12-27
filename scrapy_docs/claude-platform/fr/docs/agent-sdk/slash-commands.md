# Commandes Slash dans le SDK

Apprenez à utiliser les commandes slash pour contrôler les sessions Claude Code via le SDK

---

Les commandes slash fournissent un moyen de contrôler les sessions Claude Code avec des commandes spéciales qui commencent par `/`. Ces commandes peuvent être envoyées via le SDK pour effectuer des actions comme effacer l'historique de conversation, compacter les messages, ou obtenir de l'aide.

## Découvrir les Commandes Slash Disponibles

Le SDK Claude Agent fournit des informations sur les commandes slash disponibles dans le message d'initialisation du système. Accédez à ces informations lorsque votre session démarre :

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Bonjour Claude",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Commandes slash disponibles :", message.slash_commands);
    // Exemple de sortie : ["/compact", "/clear", "/help"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="Bonjour Claude",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            print("Commandes slash disponibles :", message.slash_commands)
            # Exemple de sortie : ["/compact", "/clear", "/help"]

asyncio.run(main())
```

</CodeGroup>

## Envoyer des Commandes Slash

Envoyez des commandes slash en les incluant dans votre chaîne de prompt, comme du texte normal :

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Envoyer une commande slash
for await (const message of query({
  prompt: "/compact",
  options: { maxTurns: 1 }
})) {
  if (message.type === "result") {
    console.log("Commande exécutée :", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Envoyer une commande slash
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if message.type == "result":
            print("Commande exécutée :", message.result)

asyncio.run(main())
```

</CodeGroup>

## Commandes Slash Courantes

### `/compact` - Compacter l'Historique de Conversation

La commande `/compact` réduit la taille de votre historique de conversation en résumant les messages plus anciens tout en préservant le contexte important :

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "/compact",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "compact_boundary") {
    console.log("Compactage terminé");
    console.log("Jetons pré-compactage :", message.compact_metadata.pre_tokens);
    console.log("Déclencheur :", message.compact_metadata.trigger);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if (message.type == "system" and 
            message.subtype == "compact_boundary"):
            print("Compactage terminé")
            print("Jetons pré-compactage :", 
                  message.compact_metadata.pre_tokens)
            print("Déclencheur :", message.compact_metadata.trigger)

asyncio.run(main())
```

</CodeGroup>

### `/clear` - Effacer la Conversation

La commande `/clear` démarre une nouvelle conversation en effaçant tout l'historique précédent :

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Effacer la conversation et recommencer à zéro
for await (const message of query({
  prompt: "/clear",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Conversation effacée, nouvelle session démarrée");
    console.log("ID de session :", message.session_id);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Effacer la conversation et recommencer à zéro
    async for message in query(
        prompt="/clear",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            print("Conversation effacée, nouvelle session démarrée")
            print("ID de session :", message.session_id)

asyncio.run(main())
```

</CodeGroup>

## Créer des Commandes Slash Personnalisées

En plus d'utiliser les commandes slash intégrées, vous pouvez créer vos propres commandes personnalisées qui sont disponibles via le SDK. Les commandes personnalisées sont définies comme des fichiers markdown dans des répertoires spécifiques, de manière similaire à la configuration des sous-agents.

### Emplacements des Fichiers

Les commandes slash personnalisées sont stockées dans des répertoires désignés selon leur portée :

- **Commandes de projet** : `.claude/commands/` - Disponibles uniquement dans le projet actuel
- **Commandes personnelles** : `~/.claude/commands/` - Disponibles dans tous vos projets

### Format de Fichier

Chaque commande personnalisée est un fichier markdown où :
- Le nom du fichier (sans l'extension `.md`) devient le nom de la commande
- Le contenu du fichier définit ce que fait la commande
- Le frontmatter YAML optionnel fournit la configuration

#### Exemple de Base

Créez `.claude/commands/refactor.md` :

```markdown
Refactorisez le code sélectionné pour améliorer la lisibilité et la maintenabilité.
Concentrez-vous sur les principes de code propre et les meilleures pratiques.
```

Cela crée la commande `/refactor` que vous pouvez utiliser via le SDK.

#### Avec Frontmatter

Créez `.claude/commands/security-check.md` :

```markdown
---
allowed-tools: Read, Grep, Glob
description: Exécuter un scan de vulnérabilités de sécurité
model: claude-3-5-sonnet-20241022
---

Analysez la base de code pour les vulnérabilités de sécurité incluant :
- Risques d'injection SQL
- Vulnérabilités XSS
- Identifiants exposés
- Configurations non sécurisées
```

### Utiliser les Commandes Personnalisées dans le SDK

Une fois définies dans le système de fichiers, les commandes personnalisées sont automatiquement disponibles via le SDK :

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Utiliser une commande personnalisée
for await (const message of query({
  prompt: "/refactor src/auth/login.ts",
  options: { maxTurns: 3 }
})) {
  if (message.type === "assistant") {
    console.log("Suggestions de refactorisation :", message.message);
  }
}

// Les commandes personnalisées apparaissent dans la liste slash_commands
for await (const message of query({
  prompt: "Bonjour",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // Inclura à la fois les commandes intégrées et personnalisées
    console.log("Commandes disponibles :", message.slash_commands);
    // Exemple : ["/compact", "/clear", "/help", "/refactor", "/security-check"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Utiliser une commande personnalisée
    async for message in query(
        prompt="/refactor src/auth/login.py",
        options={"max_turns": 3}
    ):
        if message.type == "assistant":
            print("Suggestions de refactorisation :", message.message)
    
    # Les commandes personnalisées apparaissent dans la liste slash_commands
    async for message in query(
        prompt="Bonjour",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            # Inclura à la fois les commandes intégrées et personnalisées
            print("Commandes disponibles :", message.slash_commands)
            # Exemple : ["/compact", "/clear", "/help", "/refactor", "/security-check"]

asyncio.run(main())
```

</CodeGroup>

### Fonctionnalités Avancées

#### Arguments et Espaces Réservés

Les commandes personnalisées supportent les arguments dynamiques en utilisant des espaces réservés :

Créez `.claude/commands/fix-issue.md` :

```markdown
---
argument-hint: [numéro-issue] [priorité]
description: Corriger une issue GitHub
---

Corrigez l'issue #$1 avec la priorité $2.
Vérifiez la description de l'issue et implémentez les changements nécessaires.
```

Utilisez dans le SDK :

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Passer des arguments à une commande personnalisée
for await (const message of query({
  prompt: "/fix-issue 123 high",
  options: { maxTurns: 5 }
})) {
  // La commande traitera avec $1="123" et $2="high"
  if (message.type === "result") {
    console.log("Issue corrigée :", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Passer des arguments à une commande personnalisée
    async for message in query(
        prompt="/fix-issue 123 high",
        options={"max_turns": 5}
    ):
        # La commande traitera avec $1="123" et $2="high"
        if message.type == "result":
            print("Issue corrigée :", message.result)

asyncio.run(main())
```

</CodeGroup>

#### Exécution de Commandes Bash

Les commandes personnalisées peuvent exécuter des commandes bash et inclure leur sortie :

Créez `.claude/commands/git-commit.md` :

```markdown
---
allowed-tools: Bash(git add:*), Bash(git status:*), Bash(git commit:*)
description: Créer un commit git
---

## Contexte

- Statut actuel : !`git status`
- Diff actuel : !`git diff HEAD`

## Tâche

Créez un commit git avec un message approprié basé sur les changements.
```

#### Références de Fichiers

Incluez le contenu des fichiers en utilisant le préfixe `@` :

Créez `.claude/commands/review-config.md` :

```markdown
---
description: Réviser les fichiers de configuration
---

Révisez les fichiers de configuration suivants pour les problèmes :
- Configuration du package : @package.json
- Configuration TypeScript : @tsconfig.json
- Configuration d'environnement : @.env

Vérifiez les problèmes de sécurité, les dépendances obsolètes et les mauvaises configurations.
```

### Organisation avec Espaces de Noms

Organisez les commandes dans des sous-répertoires pour une meilleure structure :

```bash
.claude/commands/
├── frontend/
│   ├── component.md      # Crée /component (projet:frontend)
│   └── style-check.md     # Crée /style-check (projet:frontend)
├── backend/
│   ├── api-test.md        # Crée /api-test (projet:backend)
│   └── db-migrate.md      # Crée /db-migrate (projet:backend)
└── review.md              # Crée /review (projet)
```

Le sous-répertoire apparaît dans la description de la commande mais n'affecte pas le nom de la commande lui-même.

### Exemples Pratiques

#### Commande de Révision de Code

Créez `.claude/commands/code-review.md` :

```markdown
---
allowed-tools: Read, Grep, Glob, Bash(git diff:*)
description: Révision de code complète
---

## Fichiers Modifiés
!`git diff --name-only HEAD~1`

## Changements Détaillés
!`git diff HEAD~1`

## Liste de Vérification de Révision

Révisez les changements ci-dessus pour :
1. Qualité et lisibilité du code
2. Vulnérabilités de sécurité
3. Implications de performance
4. Couverture de tests
5. Complétude de la documentation

Fournissez des commentaires spécifiques et exploitables organisés par priorité.
```

#### Commande d'Exécution de Tests

Créez `.claude/commands/test.md` :

```markdown
---
allowed-tools: Bash, Read, Edit
argument-hint: [motif-test]
description: Exécuter des tests avec motif optionnel
---

Exécutez les tests correspondant au motif : $ARGUMENTS

1. Détectez le framework de test (Jest, pytest, etc.)
2. Exécutez les tests avec le motif fourni
3. Si les tests échouent, analysez et corrigez-les
4. Ré-exécutez pour vérifier les corrections
```

Utilisez ces commandes via le SDK :

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Exécuter une révision de code
for await (const message of query({
  prompt: "/code-review",
  options: { maxTurns: 3 }
})) {
  // Traiter les commentaires de révision
}

// Exécuter des tests spécifiques
for await (const message of query({
  prompt: "/test auth",
  options: { maxTurns: 5 }
})) {
  // Gérer les résultats de test
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Exécuter une révision de code
    async for message in query(
        prompt="/code-review",
        options={"max_turns": 3}
    ):
        # Traiter les commentaires de révision
        pass
    
    # Exécuter des tests spécifiques
    async for message in query(
        prompt="/test auth",
        options={"max_turns": 5}
    ):
        # Gérer les résultats de test
        pass

asyncio.run(main())
```

</CodeGroup>

## Voir Aussi

- [Commandes Slash](https://code.claude.com/docs/slash-commands) - Documentation complète des commandes slash
- [Sous-agents dans le SDK](/docs/fr/agent-sdk/subagents) - Configuration similaire basée sur le système de fichiers pour les sous-agents
- [Référence du SDK TypeScript](https://code.claude.com/docs/typescript-sdk-reference) - Documentation complète de l'API
- [Aperçu du SDK](/docs/fr/agent-sdk/overview) - Concepts généraux du SDK
- [Référence CLI](https://code.claude.com/docs/cli-reference) - Interface en ligne de commande