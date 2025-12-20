# Modification des invites système

Apprenez à personnaliser le comportement de Claude en modifiant les invites système en utilisant trois approches - styles de sortie, systemPrompt avec append, et invites système personnalisées.

---

Les invites système définissent le comportement, les capacités et le style de réponse de Claude. Le SDK Claude Agent fournit trois façons de personnaliser les invites système : utiliser des styles de sortie (configurations persistantes basées sur des fichiers), ajouter à l'invite de Claude Code, ou utiliser une invite entièrement personnalisée.

## Comprendre les invites système

Une invite système est l'ensemble d'instructions initial qui façonne le comportement de Claude tout au long d'une conversation.

<Note>
**Comportement par défaut :** Le SDK Agent utilise une **invite système vide** par défaut pour une flexibilité maximale. Pour utiliser l'invite système de Claude Code (instructions d'outils, directives de code, etc.), spécifiez `systemPrompt: { preset: "claude_code" }` en TypeScript ou `system_prompt="claude_code"` en Python.
</Note>

L'invite système de Claude Code inclut :

- Instructions d'utilisation des outils et outils disponibles
- Directives de style et de formatage du code
- Paramètres de ton et de verbosité des réponses
- Instructions de sécurité et de sûreté
- Contexte sur le répertoire de travail actuel et l'environnement

## Méthodes de modification

### Méthode 1 : Fichiers CLAUDE.md (instructions au niveau du projet)

Les fichiers CLAUDE.md fournissent un contexte et des instructions spécifiques au projet qui sont automatiquement lus par le SDK Agent lorsqu'il s'exécute dans un répertoire. Ils servent de "mémoire" persistante pour votre projet.

#### Comment CLAUDE.md fonctionne avec le SDK

**Emplacement et découverte :**

- **Niveau projet :** `CLAUDE.md` ou `.claude/CLAUDE.md` dans votre répertoire de travail
- **Niveau utilisateur :** `~/.claude/CLAUDE.md` pour des instructions globales à travers tous les projets

**IMPORTANT :** Le SDK ne lit les fichiers CLAUDE.md que lorsque vous configurez explicitement `settingSources` (TypeScript) ou `setting_sources` (Python) :

- Incluez `'project'` pour charger CLAUDE.md au niveau du projet
- Incluez `'user'` pour charger CLAUDE.md au niveau utilisateur (`~/.claude/CLAUDE.md`)

Le preset d'invite système `claude_code` ne charge PAS automatiquement CLAUDE.md - vous devez également spécifier les sources de paramètres.

**Format du contenu :**
Les fichiers CLAUDE.md utilisent du markdown simple et peuvent contenir :

- Directives et standards de codage
- Contexte spécifique au projet
- Commandes ou flux de travail courants
- Conventions d'API
- Exigences de test

#### Exemple CLAUDE.md

```markdown
# Directives du Projet

## Style de Code

- Utiliser le mode strict TypeScript
- Préférer les composants fonctionnels en React
- Toujours inclure des commentaires JSDoc pour les APIs publiques

## Tests

- Exécuter `npm test` avant de commiter
- Maintenir >80% de couverture de code
- Utiliser jest pour les tests unitaires, playwright pour E2E

## Commandes

- Build : `npm run build`
- Serveur dev : `npm run dev`
- Vérification de type : `npm run typecheck`
```

#### Utiliser CLAUDE.md avec le SDK

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// IMPORTANT : Vous devez spécifier settingSources pour charger CLAUDE.md
// Le preset claude_code seul ne charge PAS les fichiers CLAUDE.md
const messages = [];

for await (const message of query({
  prompt: "Ajouter un nouveau composant React pour les profils utilisateur",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code", // Utiliser l'invite système de Claude Code
    },
    settingSources: ["project"], // Requis pour charger CLAUDE.md du projet
  },
})) {
  messages.push(message);
}

// Maintenant Claude a accès à vos directives de projet depuis CLAUDE.md
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# IMPORTANT : Vous devez spécifier setting_sources pour charger CLAUDE.md
# Le preset claude_code seul ne charge PAS les fichiers CLAUDE.md
messages = []

async for message in query(
    prompt="Ajouter un nouveau composant React pour les profils utilisateur",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code"  # Utiliser l'invite système de Claude Code
        },
        setting_sources=["project"]  # Requis pour charger CLAUDE.md du projet
    )
):
    messages.append(message)

# Maintenant Claude a accès à vos directives de projet depuis CLAUDE.md
```

</CodeGroup>

#### Quand utiliser CLAUDE.md

**Idéal pour :**

- **Contexte partagé en équipe** - Directives que tout le monde devrait suivre
- **Conventions de projet** - Standards de codage, structure de fichiers, modèles de nommage
- **Commandes courantes** - Commandes de build, test, déploiement spécifiques à votre projet
- **Mémoire à long terme** - Contexte qui devrait persister à travers toutes les sessions
- **Instructions versionnées** - Commiter dans git pour que l'équipe reste synchronisée

**Caractéristiques clés :**

- ✅ Persistant à travers toutes les sessions dans un projet
- ✅ Partagé avec l'équipe via git
- ✅ Découverte automatique (aucun changement de code nécessaire)
- ⚠️ Nécessite le chargement des paramètres via `settingSources`

### Méthode 2 : Styles de sortie (configurations persistantes)

Les styles de sortie sont des configurations sauvegardées qui modifient l'invite système de Claude. Ils sont stockés comme fichiers markdown et peuvent être réutilisés à travers les sessions et projets.

#### Créer un style de sortie

<CodeGroup>

```typescript TypeScript
import { writeFile, mkdir } from "fs/promises";
import { join } from "path";
import { homedir } from "os";

async function createOutputStyle(
  name: string,
  description: string,
  prompt: string
) {
  // Niveau utilisateur : ~/.claude/output-styles
  // Niveau projet : .claude/output-styles
  const outputStylesDir = join(homedir(), ".claude", "output-styles");

  await mkdir(outputStylesDir, { recursive: true });

  const content = `---
name: ${name}
description: ${description}
---

${prompt}`;

  const filePath = join(
    outputStylesDir,
    `${name.toLowerCase().replace(/\s+/g, "-")}.md`
  );
  await writeFile(filePath, content, "utf-8");
}

// Exemple : Créer un spécialiste de révision de code
await createOutputStyle(
  "Code Reviewer",
  "Assistant de révision de code approfondie",
  `Vous êtes un expert en révision de code.

Pour chaque soumission de code :
1. Vérifier les bugs et problèmes de sécurité
2. Évaluer les performances
3. Suggérer des améliorations
4. Noter la qualité du code (1-10)`
);
```

```python Python
from pathlib import Path

async def create_output_style(name: str, description: str, prompt: str):
    # Niveau utilisateur : ~/.claude/output-styles
    # Niveau projet : .claude/output-styles
    output_styles_dir = Path.home() / '.claude' / 'output-styles'

    output_styles_dir.mkdir(parents=True, exist_ok=True)

    content = f"""---
name: {name}
description: {description}
---

{prompt}"""

    file_name = name.lower().replace(' ', '-') + '.md'
    file_path = output_styles_dir / file_name
    file_path.write_text(content, encoding='utf-8')

# Exemple : Créer un spécialiste de révision de code
await create_output_style(
    'Code Reviewer',
    'Assistant de révision de code approfondie',
    """Vous êtes un expert en révision de code.

Pour chaque soumission de code :
1. Vérifier les bugs et problèmes de sécurité
2. Évaluer les performances
3. Suggérer des améliorations
4. Noter la qualité du code (1-10)"""
)
```

</CodeGroup>

#### Utiliser les styles de sortie

Une fois créés, activez les styles de sortie via :

- **CLI** : `/output-style [nom-du-style]`
- **Paramètres** : `.claude/settings.local.json`
- **Créer nouveau** : `/output-style:new [description]`

**Note pour les utilisateurs du SDK :** Les styles de sortie sont chargés lorsque vous incluez `settingSources: ['user']` ou `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` ou `setting_sources=["project"]` (Python) dans vos options.

### Méthode 3 : Utiliser `systemPrompt` avec append

Vous pouvez utiliser le preset Claude Code avec une propriété `append` pour ajouter vos instructions personnalisées tout en préservant toutes les fonctionnalités intégrées.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const messages = [];

for await (const message of query({
  prompt: "Aidez-moi à écrire une fonction Python pour calculer les nombres de fibonacci",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append:
        "Toujours inclure des docstrings détaillées et des annotations de type dans le code Python.",
    },
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

messages = []

async for message in query(
    prompt="Aidez-moi à écrire une fonction Python pour calculer les nombres de fibonacci",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": "Toujours inclure des docstrings détaillées et des annotations de type dans le code Python."
        }
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

### Méthode 4 : Invites système personnalisées

Vous pouvez fournir une chaîne personnalisée comme `systemPrompt` pour remplacer entièrement le défaut par vos propres instructions.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const customPrompt = `Vous êtes un spécialiste du codage Python.
Suivez ces directives :
- Écrire du code propre et bien documenté
- Utiliser des annotations de type pour toutes les fonctions
- Inclure des docstrings complètes
- Préférer les modèles de programmation fonctionnelle quand approprié
- Toujours expliquer vos choix de code`;

const messages = [];

for await (const message of query({
  prompt: "Créer un pipeline de traitement de données",
  options: {
    systemPrompt: customPrompt,
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

custom_prompt = """Vous êtes un spécialiste du codage Python.
Suivez ces directives :
- Écrire du code propre et bien documenté
- Utiliser des annotations de type pour toutes les fonctions
- Inclure des docstrings complètes
- Préférer les modèles de programmation fonctionnelle quand approprié
- Toujours expliquer vos choix de code"""

messages = []

async for message in query(
    prompt="Créer un pipeline de traitement de données",
    options=ClaudeAgentOptions(
        system_prompt=custom_prompt
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

## Comparaison des quatre approches

| Fonctionnalité          | CLAUDE.md           | Styles de Sortie   | `systemPrompt` avec append | `systemPrompt` personnalisé |
| --- | --- | --- | --- | --- |
| **Persistance**         | Fichier par projet | Sauvegardés comme fichiers | Session seulement | Session seulement |
| **Réutilisabilité**     | Par projet         | À travers projets  | Duplication de code | Duplication de code |
| **Gestion**             | Sur système de fichiers | CLI + fichiers | Dans le code | Dans le code |
| **Outils par défaut**   | Préservés          | Préservés          | Préservés | Perdus (sauf si inclus) |
| **Sécurité intégrée**   | Maintenue          | Maintenue          | Maintenue | Doit être ajoutée |
| **Contexte environnement** | Automatique     | Automatique        | Automatique | Doit être fourni |
| **Niveau de personnalisation** | Ajouts seulement | Remplacer défaut | Ajouts seulement | Contrôle complet |
| **Contrôle de version** | Avec projet        | Oui                | Avec code | Avec code |
| **Portée**              | Spécifique au projet | Utilisateur ou projet | Session de code | Session de code |

**Note :** "Avec append" signifie utiliser `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` en TypeScript ou `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` en Python.

## Cas d'usage et meilleures pratiques

### Quand utiliser CLAUDE.md

**Idéal pour :**

- Standards et conventions de codage spécifiques au projet
- Documenter la structure et l'architecture du projet
- Lister les commandes courantes (build, test, déploiement)
- Contexte partagé en équipe qui devrait être versionné
- Instructions qui s'appliquent à toute utilisation du SDK dans un projet

**Exemples :**

- "Tous les endpoints d'API devraient utiliser les modèles async/await"
- "Exécuter `npm run lint:fix` avant de commiter"
- "Les migrations de base de données sont dans le répertoire `migrations/`"

**Important :** Pour charger les fichiers CLAUDE.md, vous devez explicitement définir `settingSources: ['project']` (TypeScript) ou `setting_sources=["project"]` (Python). Le preset d'invite système `claude_code` ne charge PAS automatiquement CLAUDE.md sans ce paramètre.

### Quand utiliser les styles de sortie

**Idéal pour :**

- Changements de comportement persistants à travers les sessions
- Configurations partagées en équipe
- Assistants spécialisés (réviseur de code, data scientist, DevOps)
- Modifications d'invite complexes qui nécessitent un versioning

**Exemples :**

- Créer un assistant dédié à l'optimisation SQL
- Construire un réviseur de code axé sur la sécurité
- Développer un assistant d'enseignement avec une pédagogie spécifique

### Quand utiliser `systemPrompt` avec append

**Idéal pour :**

- Ajouter des standards ou préférences de codage spécifiques
- Personnaliser le formatage de sortie
- Ajouter des connaissances spécifiques au domaine
- Modifier la verbosité des réponses
- Améliorer le comportement par défaut de Claude Code sans perdre les instructions d'outils

### Quand utiliser `systemPrompt` personnalisé

**Idéal pour :**

- Contrôle complet sur le comportement de Claude
- Tâches spécialisées à session unique
- Tester de nouvelles stratégies d'invite
- Situations où les outils par défaut ne sont pas nécessaires
- Construire des agents spécialisés avec un comportement unique

## Combiner les approches

Vous pouvez combiner ces méthodes pour une flexibilité maximale :

### Exemple : Style de sortie avec ajouts spécifiques à la session

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// En supposant que le style de sortie "Code Reviewer" est actif (via /output-style)
// Ajouter des domaines de focus spécifiques à la session
const messages = [];

for await (const message of query({
  prompt: "Réviser ce module d'authentification",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: `
        Pour cette révision, priorisez :
        - Conformité OAuth 2.0
        - Sécurité du stockage des tokens
        - Gestion des sessions
      `,
    },
  },
})) {
  messages.push(message);
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# En supposant que le style de sortie "Code Reviewer" est actif (via /output-style)
# Ajouter des domaines de focus spécifiques à la session
messages = []

async for message in query(
    prompt="Réviser ce module d'authentification",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": """
            Pour cette révision, priorisez :
            - Conformité OAuth 2.0
            - Sécurité du stockage des tokens
            - Gestion des sessions
            """
        }
    )
):
    messages.append(message)
```

</CodeGroup>

## Voir aussi

- [Styles de sortie](https://code.claude.com/docs/output-styles) - Documentation complète des styles de sortie
- [Guide du SDK TypeScript](/docs/fr/agent-sdk/typescript) - Guide complet d'utilisation du SDK
- [Référence du SDK TypeScript](https://code.claude.com/docs/typescript-sdk-reference) - Documentation complète de l'API
- [Guide de configuration](https://code.claude.com/docs/configuration) - Options de configuration générales