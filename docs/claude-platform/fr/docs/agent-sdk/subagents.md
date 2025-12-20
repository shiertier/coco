# Sous-agents dans le SDK

Travailler avec les sous-agents dans le SDK Claude Agent

---

Les sous-agents dans le SDK Claude Agent sont des IA spécialisées qui sont orchestrées par l'agent principal.
Utilisez les sous-agents pour la gestion du contexte et la parallélisation.

Ce guide explique comment définir et utiliser les sous-agents dans le SDK en utilisant le paramètre `agents`.

## Aperçu

Les sous-agents peuvent être définis de deux façons lors de l'utilisation du SDK :

1. **Par programmation** - En utilisant le paramètre `agents` dans vos options `query()` (recommandé pour les applications SDK)
2. **Basé sur le système de fichiers** - En plaçant des fichiers markdown avec frontmatter YAML dans des répertoires désignés (`.claude/agents/`)

Ce guide se concentre principalement sur l'approche programmatique utilisant le paramètre `agents`, qui fournit une expérience de développement plus intégrée pour les applications SDK.

## Avantages de l'utilisation des sous-agents

### Gestion du contexte
Les sous-agents maintiennent un contexte séparé de l'agent principal, empêchant la surcharge d'informations et gardant les interactions focalisées. Cette isolation garantit que les tâches spécialisées ne polluent pas le contexte de conversation principal avec des détails non pertinents.

**Exemple** : Un sous-agent `research-assistant` peut explorer des dizaines de fichiers et de pages de documentation sans encombrer la conversation principale avec tous les résultats de recherche intermédiaires - ne retournant que les découvertes pertinentes.

### Parallélisation
Plusieurs sous-agents peuvent fonctionner simultanément, accélérant considérablement les flux de travail complexes.

**Exemple** : Lors d'une revue de code, vous pouvez exécuter les sous-agents `style-checker`, `security-scanner`, et `test-coverage` simultanément, réduisant le temps de revue de minutes à secondes.

### Instructions et connaissances spécialisées
Chaque sous-agent peut avoir des invites système personnalisées avec une expertise spécifique, des meilleures pratiques et des contraintes.

**Exemple** : Un sous-agent `database-migration` peut avoir des connaissances détaillées sur les meilleures pratiques SQL, les stratégies de rollback et les vérifications d'intégrité des données qui seraient du bruit inutile dans les instructions de l'agent principal.

### Restrictions d'outils
Les sous-agents peuvent être limités à des outils spécifiques, réduisant le risque d'actions non intentionnelles.

**Exemple** : Un sous-agent `doc-reviewer` pourrait n'avoir accès qu'aux outils Read et Grep, garantissant qu'il peut analyser mais ne jamais modifier accidentellement vos fichiers de documentation.

## Création de sous-agents

### Définition programmatique (Recommandée)

Définissez les sous-agents directement dans votre code en utilisant le paramètre `agents` :

```typescript
import { query } from '@anthropic-ai/claude-agent-sdk';

const result = query({
  prompt: "Examinez le module d'authentification pour les problèmes de sécurité",
  options: {
    agents: {
      'code-reviewer': {
        description: 'Spécialiste expert en revue de code. Utilisez pour les revues de qualité, sécurité et maintenabilité.',
        prompt: `Vous êtes un spécialiste de la revue de code avec une expertise en sécurité, performance et meilleures pratiques.

Lors de la revue de code :
- Identifiez les vulnérabilités de sécurité
- Vérifiez les problèmes de performance
- Vérifiez l'adhérence aux standards de codage
- Suggérez des améliorations spécifiques

Soyez minutieux mais concis dans vos commentaires.`,
        tools: ['Read', 'Grep', 'Glob'],
        model: 'sonnet'
      },
      'test-runner': {
        description: 'Exécute et analyse les suites de tests. Utilisez pour l\'exécution de tests et l\'analyse de couverture.',
        prompt: `Vous êtes un spécialiste de l'exécution de tests. Exécutez les tests et fournissez une analyse claire des résultats.

Concentrez-vous sur :
- L'exécution des commandes de test
- L'analyse de la sortie des tests
- L'identification des tests qui échouent
- La suggestion de corrections pour les échecs`,
        tools: ['Bash', 'Read', 'Grep'],
      }
    }
  }
});

for await (const message of result) {
  console.log(message);
}
```

### Configuration AgentDefinition

| Champ | Type | Requis | Description |
|:---|:---|:---|:---|
| `description` | `string` | Oui | Description en langage naturel de quand utiliser cet agent |
| `prompt` | `string` | Oui | L'invite système de l'agent définissant son rôle et comportement |
| `tools` | `string[]` | Non | Tableau des noms d'outils autorisés. Si omis, hérite de tous les outils |
| `model` | `'sonnet' \| 'opus' \| 'haiku' \| 'inherit'` | Non | Remplacement de modèle pour cet agent. Par défaut au modèle principal si omis |

### Définition basée sur le système de fichiers (Alternative)

Vous pouvez également définir les sous-agents comme des fichiers markdown dans des répertoires spécifiques :

- **Niveau projet** : `.claude/agents/*.md` - Disponible uniquement dans le projet actuel
- **Niveau utilisateur** : `~/.claude/agents/*.md` - Disponible dans tous les projets

Chaque sous-agent est un fichier markdown avec frontmatter YAML :

```markdown
---
name: code-reviewer
description: Spécialiste expert en revue de code. Utilisez pour les revues de qualité, sécurité et maintenabilité.
tools: Read, Grep, Glob, Bash
---

L'invite système de votre sous-agent va ici. Ceci définit le rôle du sous-agent,
ses capacités et son approche pour résoudre les problèmes.
```

**Note :** Les agents définis par programmation (via le paramètre `agents`) ont la priorité sur les agents basés sur le système de fichiers avec le même nom.

## Comment le SDK utilise les sous-agents

Lors de l'utilisation du SDK Claude Agent, les sous-agents peuvent être définis par programmation ou chargés depuis le système de fichiers. Claude va :

1. **Charger les agents programmatiques** depuis le paramètre `agents` dans vos options
2. **Auto-détecter les agents du système de fichiers** depuis les répertoires `.claude/agents/` (si non remplacés)
3. **Les invoquer automatiquement** basé sur la correspondance des tâches et la `description` de l'agent
4. **Utiliser leurs invites spécialisées** et restrictions d'outils
5. **Maintenir un contexte séparé** pour chaque invocation de sous-agent

Les agents définis par programmation (via le paramètre `agents`) ont la priorité sur les agents basés sur le système de fichiers avec le même nom.

## Exemples de sous-agents

Pour des exemples complets de sous-agents incluant des réviseurs de code, des exécuteurs de tests, des débogueurs et des auditeurs de sécurité, voir le [guide principal des Sous-agents](https://code.claude.com/docs/sub-agents#example-subagents). Le guide inclut des configurations détaillées et des meilleures pratiques pour créer des sous-agents efficaces.

## Modèles d'intégration SDK

### Invocation automatique

Le SDK invoquera automatiquement les sous-agents appropriés basés sur le contexte de la tâche. Assurez-vous que le champ `description` de votre agent indique clairement quand il devrait être utilisé :

```typescript
const result = query({
  prompt: "Optimisez les requêtes de base de données dans la couche API",
  options: {
    agents: {
      'performance-optimizer': {
        description: 'Utilisez PROACTIVEMENT quand les changements de code pourraient impacter la performance. DOIT ÊTRE UTILISÉ pour les tâches d\'optimisation.',
        prompt: 'Vous êtes un spécialiste de l\'optimisation de performance...',
        tools: ['Read', 'Edit', 'Bash', 'Grep'],
        model: 'sonnet'
      }
    }
  }
});
```

### Invocation explicite

Les utilisateurs peuvent demander des sous-agents spécifiques dans leurs invites :

```typescript
const result = query({
  prompt: "Utilisez l'agent code-reviewer pour vérifier le module d'authentification",
  options: {
    agents: {
      'code-reviewer': {
        description: 'Spécialiste expert en revue de code',
        prompt: 'Vous êtes un réviseur de code axé sur la sécurité...',
        tools: ['Read', 'Grep', 'Glob']
      }
    }
  }
});
```

### Configuration d'agent dynamique

Vous pouvez configurer dynamiquement les agents basés sur les besoins de votre application :

```typescript
import { query, type AgentDefinition } from '@anthropic-ai/claude-agent-sdk';

function createSecurityAgent(securityLevel: 'basic' | 'strict'): AgentDefinition {
  return {
    description: 'Réviseur de code de sécurité',
    prompt: `Vous êtes un réviseur de sécurité ${securityLevel === 'strict' ? 'strict' : 'équilibré'}...`,
    tools: ['Read', 'Grep', 'Glob'],
    model: securityLevel === 'strict' ? 'opus' : 'sonnet'
  };
}

const result = query({
  prompt: "Examinez cette PR pour les problèmes de sécurité",
  options: {
    agents: {
      'security-reviewer': createSecurityAgent('strict')
    }
  }
});
```

## Restrictions d'outils

Les sous-agents peuvent avoir un accès restreint aux outils via le champ `tools` :

- **Omettre le champ** - L'agent hérite de tous les outils disponibles (par défaut)
- **Spécifier les outils** - L'agent ne peut utiliser que les outils listés

Exemple d'un agent d'analyse en lecture seule :

```typescript
const result = query({
  prompt: "Analysez l'architecture de cette base de code",
  options: {
    agents: {
      'code-analyzer': {
        description: 'Analyse statique de code et revue d\'architecture',
        prompt: `Vous êtes un analyste d'architecture de code. Analysez la structure du code,
identifiez les modèles et suggérez des améliorations sans faire de changements.`,
        tools: ['Read', 'Grep', 'Glob']  // Pas de permissions d'écriture ou d'exécution
      }
    }
  }
});
```

### Combinaisons d'outils communes

**Agents en lecture seule** (analyse, revue) :
```typescript
tools: ['Read', 'Grep', 'Glob']
```

**Agents d'exécution de tests** :
```typescript
tools: ['Bash', 'Read', 'Grep']
```

**Agents de modification de code** :
```typescript
tools: ['Read', 'Edit', 'Write', 'Grep', 'Glob']
```

## Documentation connexe

- [Guide principal des Sous-agents](https://code.claude.com/docs/sub-agents) - Documentation complète des sous-agents
- [Aperçu du SDK](/docs/fr/agent-sdk/overview) - Aperçu du SDK Claude Agent
- [Paramètres](https://code.claude.com/docs/settings) - Référence du fichier de configuration
- [Commandes Slash](https://code.claude.com/docs/slash-commands) - Création de commandes personnalisées