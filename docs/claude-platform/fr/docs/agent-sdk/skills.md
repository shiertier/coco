# Compétences d'agent dans le SDK

Étendez Claude avec des capacités spécialisées en utilisant les compétences d'agent dans le SDK Claude Agent

---

## Aperçu

Les compétences d'agent étendent Claude avec des capacités spécialisées que Claude invoque de manière autonome lorsque c'est pertinent. Les compétences sont empaquetées sous forme de fichiers `SKILL.md` contenant des instructions, des descriptions et des ressources de support optionnelles.

Pour des informations complètes sur les compétences, y compris les avantages, l'architecture et les directives de création, consultez l'[aperçu des compétences d'agent](/docs/fr/agents-and-tools/agent-skills/overview).

## Comment les compétences fonctionnent avec le SDK

Lors de l'utilisation du SDK Claude Agent, les compétences sont :

1. **Définies comme des artefacts du système de fichiers** : Créées sous forme de fichiers `SKILL.md` dans des répertoires spécifiques (`.claude/skills/`)
2. **Chargées à partir du système de fichiers** : Les compétences sont chargées à partir des emplacements du système de fichiers configurés. Vous devez spécifier `settingSources` (TypeScript) ou `setting_sources` (Python) pour charger les compétences à partir du système de fichiers
3. **Découvertes automatiquement** : Une fois les paramètres du système de fichiers chargés, les métadonnées des compétences sont découvertes au démarrage à partir des répertoires utilisateur et projet ; le contenu complet est chargé lorsqu'il est déclenché
4. **Invoquées par le modèle** : Claude choisit de manière autonome quand les utiliser en fonction du contexte
5. **Activées via allowed_tools** : Ajoutez `"Skill"` à votre `allowed_tools` pour activer les compétences

Contrairement aux sous-agents (qui peuvent être définis par programmation), les compétences doivent être créées comme des artefacts du système de fichiers. Le SDK ne fournit pas d'API programmatique pour enregistrer les compétences.

<Note>
**Comportement par défaut** : Par défaut, le SDK ne charge aucun paramètre du système de fichiers. Pour utiliser les compétences, vous devez explicitement configurer `settingSources: ['user', 'project']` (TypeScript) ou `setting_sources=["user", "project"]` (Python) dans vos options.
</Note>

## Utilisation des compétences avec le SDK

Pour utiliser les compétences avec le SDK, vous devez :

1. Inclure `"Skill"` dans votre configuration `allowed_tools`
2. Configurer `settingSources`/`setting_sources` pour charger les compétences à partir du système de fichiers

Une fois configuré, Claude découvre automatiquement les compétences à partir des répertoires spécifiés et les invoque lorsqu'elles sont pertinentes pour la demande de l'utilisateur.

<CodeGroup>

```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions

async def main():
    options = ClaudeAgentOptions(
        cwd="/path/to/project",  # Project with .claude/skills/
        setting_sources=["user", "project"],  # Load Skills from filesystem
        allowed_tools=["Skill", "Read", "Write", "Bash"]  # Enable Skill tool
    )

    async for message in query(
        prompt="Help me process this PDF document",
        options=options
    ):
        print(message)

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me process this PDF document",
  options: {
    cwd: "/path/to/project",  // Project with .claude/skills/
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Write", "Bash"]  // Enable Skill tool
  }
})) {
  console.log(message);
}
```

</CodeGroup>

## Emplacements des compétences

Les compétences sont chargées à partir des répertoires du système de fichiers en fonction de votre configuration `settingSources`/`setting_sources` :

- **Compétences de projet** (`.claude/skills/`) : Partagées avec votre équipe via git - chargées lorsque `setting_sources` inclut `"project"`
- **Compétences utilisateur** (`~/.claude/skills/`) : Compétences personnelles dans tous les projets - chargées lorsque `setting_sources` inclut `"user"`
- **Compétences de plugin** : Fournies avec les plugins Claude Code installés

## Création de compétences

Les compétences sont définies comme des répertoires contenant un fichier `SKILL.md` avec un préambule YAML et du contenu Markdown. Le champ `description` détermine quand Claude invoque votre compétence.

**Exemple de structure de répertoire** :
```bash
.claude/skills/processing-pdfs/
└── SKILL.md
```

Pour des conseils complets sur la création de compétences, y compris la structure SKILL.md, les compétences multi-fichiers et les exemples, consultez :
- [Compétences d'agent dans Claude Code](https://code.claude.com/docs/skills) : Guide complet avec des exemples
- [Meilleures pratiques des compétences d'agent](/docs/fr/agents-and-tools/agent-skills/best-practices) : Directives de création et conventions de nommage

## Restrictions d'outils

<Note>
Le champ de préambule `allowed-tools` dans SKILL.md n'est pris en charge que lors de l'utilisation directe de Claude Code CLI. **Il ne s'applique pas lors de l'utilisation des compétences via le SDK**.

Lors de l'utilisation du SDK, contrôlez l'accès aux outils via l'option principale `allowedTools` dans votre configuration de requête.
</Note>

Pour restreindre les outils pour les compétences dans les applications SDK, utilisez l'option `allowedTools` :

<Note>
Les déclarations d'importation du premier exemple sont supposées être dans les extraits de code suivants.
</Note>

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill", "Read", "Grep", "Glob"]  # Restricted toolset
)

async for message in query(
    prompt="Analyze the codebase structure",
    options=options
):
    print(message)
```

```typescript TypeScript
// Skills can only use Read, Grep, and Glob tools
for await (const message of query({
  prompt: "Analyze the codebase structure",
  options: {
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Grep", "Glob"]  // Restricted toolset
  }
})) {
  console.log(message);
}
```

</CodeGroup>

## Découverte des compétences disponibles

Pour voir quelles compétences sont disponibles dans votre application SDK, demandez simplement à Claude :

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill"]
)

async for message in query(
    prompt="What Skills are available?",
    options=options
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "What Skills are available?",
  options: {
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill"]
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Claude listera les compétences disponibles en fonction de votre répertoire de travail actuel et des plugins installés.

## Test des compétences

Testez les compétences en posant des questions qui correspondent à leurs descriptions :

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    cwd="/path/to/project",
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill", "Read", "Bash"]
)

async for message in query(
    prompt="Extract text from invoice.pdf",
    options=options
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "Extract text from invoice.pdf",
  options: {
    cwd: "/path/to/project",
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Bash"]
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Claude invoque automatiquement la compétence pertinente si la description correspond à votre demande.

## Dépannage

### Compétences non trouvées

**Vérifiez la configuration settingSources** : Les compétences ne sont chargées que lorsque vous configurez explicitement `settingSources`/`setting_sources`. C'est le problème le plus courant :

<CodeGroup>

```python Python
# Incorrect - Les compétences ne seront pas chargées
options = ClaudeAgentOptions(
    allowed_tools=["Skill"]
)

# Correct - Les compétences seront chargées
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// Incorrect - Les compétences ne seront pas chargées
const options = {
  allowedTools: ["Skill"]
};

// Correct - Les compétences seront chargées
const options = {
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

Pour plus de détails sur `settingSources`/`setting_sources`, consultez la [référence du SDK TypeScript](/docs/fr/agent-sdk/typescript#settingsource) ou la [référence du SDK Python](/docs/fr/agent-sdk/python#settingsource).

**Vérifiez le répertoire de travail** : Le SDK charge les compétences par rapport à l'option `cwd`. Assurez-vous qu'elle pointe vers un répertoire contenant `.claude/skills/` :

<CodeGroup>

```python Python
# Assurez-vous que votre cwd pointe vers le répertoire contenant .claude/skills/
options = ClaudeAgentOptions(
    cwd="/path/to/project",  # Must contain .claude/skills/
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// Assurez-vous que votre cwd pointe vers le répertoire contenant .claude/skills/
const options = {
  cwd: "/path/to/project",  // Must contain .claude/skills/
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

Consultez la section « Utilisation des compétences avec le SDK » ci-dessus pour le modèle complet.

**Vérifiez l'emplacement du système de fichiers** :
```bash
# Vérifiez les compétences du projet
ls .claude/skills/*/SKILL.md

# Vérifiez les compétences personnelles
ls ~/.claude/skills/*/SKILL.md
```

### La compétence n'est pas utilisée

**Vérifiez que l'outil Skill est activé** : Confirmez que `"Skill"` est dans votre `allowedTools`.

**Vérifiez la description** : Assurez-vous qu'elle est spécifique et inclut les mots-clés pertinents. Consultez [Meilleures pratiques des compétences d'agent](/docs/fr/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) pour des conseils sur la rédaction de descriptions efficaces.

### Dépannage supplémentaire

Pour le dépannage général des compétences (syntaxe YAML, débogage, etc.), consultez la [section dépannage des compétences Claude Code](https://code.claude.com/docs/skills#troubleshooting).

## Documentation connexe

### Guides des compétences
- [Compétences d'agent dans Claude Code](https://code.claude.com/docs/skills) : Guide complet des compétences avec création, exemples et dépannage
- [Aperçu des compétences d'agent](/docs/fr/agents-and-tools/agent-skills/overview) : Aperçu conceptuel, avantages et architecture
- [Meilleures pratiques des compétences d'agent](/docs/fr/agents-and-tools/agent-skills/best-practices) : Directives de création pour des compétences efficaces
- [Livre de recettes des compétences d'agent](https://github.com/anthropics/claude-cookbooks/tree/main/skills) : Exemples de compétences et modèles

### Ressources du SDK
- [Sous-agents dans le SDK](/docs/fr/agent-sdk/subagents) : Agents basés sur le système de fichiers similaires avec options programmatiques
- [Commandes slash dans le SDK](/docs/fr/agent-sdk/slash-commands) : Commandes invoquées par l'utilisateur
- [Aperçu du SDK](/docs/fr/agent-sdk/overview) : Concepts généraux du SDK
- [Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript) : Documentation complète de l'API
- [Référence du SDK Python](/docs/fr/agent-sdk/python) : Documentation complète de l'API