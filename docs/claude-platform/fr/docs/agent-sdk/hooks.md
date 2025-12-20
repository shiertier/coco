# Intercepter et contrôler le comportement de l'agent avec des hooks

Intercepter et personnaliser le comportement de l'agent aux points d'exécution clés avec des hooks

---

Les hooks vous permettent d'intercepter l'exécution de l'agent aux points clés pour ajouter une validation, une journalisation, des contrôles de sécurité ou une logique personnalisée. Avec les hooks, vous pouvez :

- **Bloquer les opérations dangereuses** avant leur exécution, comme les commandes shell destructrices ou l'accès non autorisé aux fichiers
- **Journaliser et auditer** chaque appel d'outil pour la conformité, le débogage ou l'analyse
- **Transformer les entrées et les sorties** pour nettoyer les données, injecter des identifiants ou rediriger les chemins de fichiers
- **Exiger une approbation humaine** pour les actions sensibles comme les écritures de base de données ou les appels API
- **Suivre le cycle de vie de la session** pour gérer l'état, nettoyer les ressources ou envoyer des notifications

Un hook comporte deux parties :

1. **La fonction de rappel** : la logique qui s'exécute lorsque le hook se déclenche
2. **La configuration du hook** : indique au SDK quel événement accrocher (comme `PreToolUse`) et quels outils faire correspondre

L'exemple suivant empêche l'agent de modifier les fichiers `.env`. Tout d'abord, définissez un rappel qui vérifie le chemin du fichier, puis transmettez-le à `query()` pour l'exécuter avant tout appel d'outil Write ou Edit :

<CodeGroup>

```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher

# Définir un rappel de hook qui reçoit les détails de l'appel d'outil
async def protect_env_files(input_data, tool_use_id, context):
    # Extraire le chemin du fichier des arguments d'entrée de l'outil
    file_path = input_data['tool_input'].get('file_path', '')
    file_name = file_path.split('/')[-1]

    # Bloquer l'opération si elle cible un fichier .env
    if file_name == '.env':
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'deny',
                'permissionDecisionReason': 'Cannot modify .env files'
            }
        }

    # Retourner un objet vide pour autoriser l'opération
    return {}

async def main():
    async for message in query(
        prompt="Update the database configuration",
        options=ClaudeAgentOptions(
            hooks={
                # Enregistrer le hook pour les événements PreToolUse
                # Le matcher filtre uniquement les appels d'outils Write et Edit
                'PreToolUse': [HookMatcher(matcher='Write|Edit', hooks=[protect_env_files])]
            }
        )
    ):
        print(message)

asyncio.run(main())
```

```typescript TypeScript
import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

// Définir un rappel de hook avec le type HookCallback
const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
  // Convertir l'entrée au type de hook spécifique pour la sécurité des types
  const preInput = input as PreToolUseHookInput;

  // Extraire le chemin du fichier des arguments d'entrée de l'outil
  const filePath = preInput.tool_input?.file_path as string;
  const fileName = filePath?.split('/').pop();

  // Bloquer l'opération si elle cible un fichier .env
  if (fileName === '.env') {
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'deny',
        permissionDecisionReason: 'Cannot modify .env files'
      }
    };
  }

  // Retourner un objet vide pour autoriser l'opération
  return {};
};

for await (const message of query({
  prompt: "Update the database configuration",
  options: {
    hooks: {
      // Enregistrer le hook pour les événements PreToolUse
      // Le matcher filtre uniquement les appels d'outils Write et Edit
      PreToolUse: [{ matcher: 'Write|Edit', hooks: [protectEnvFiles] }]
    }
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Ceci est un hook `PreToolUse`. Il s'exécute avant l'exécution de l'outil et peut bloquer ou autoriser les opérations en fonction de votre logique. Le reste de ce guide couvre tous les hooks disponibles, leurs options de configuration et les modèles pour les cas d'usage courants.

## Hooks disponibles

Le SDK fournit des hooks pour différentes étapes de l'exécution de l'agent. Certains hooks sont disponibles dans les deux SDK, tandis que d'autres sont spécifiques à TypeScript car le SDK Python ne les supporte pas.

| Événement Hook | SDK Python | SDK TypeScript | Ce qui le déclenche | Cas d'usage exemple |
|------------|------------|----------------|------------------|------------------|
| `PreToolUse` | Oui | Oui | Demande d'appel d'outil (peut bloquer ou modifier) | Bloquer les commandes shell dangereuses |
| `PostToolUse` | Oui | Oui | Résultat de l'exécution de l'outil | Journaliser tous les changements de fichiers dans la piste d'audit |
| `PostToolUseFailure` | Non | Oui | Échec de l'exécution de l'outil | Gérer ou journaliser les erreurs d'outil |
| `UserPromptSubmit` | Oui | Oui | Soumission de l'invite utilisateur | Injecter du contexte supplémentaire dans les invites |
| `Stop` | Oui | Oui | Arrêt de l'exécution de l'agent | Enregistrer l'état de la session avant la sortie |
| `SubagentStart` | Non | Oui | Initialisation du sous-agent | Suivre le lancement des tâches parallèles |
| `SubagentStop` | Oui | Oui | Achèvement du sous-agent | Agréger les résultats des tâches parallèles |
| `PreCompact` | Oui | Oui | Demande de compaction de conversation | Archiver la transcription complète avant le résumé |
| `PermissionRequest` | Non | Oui | La boîte de dialogue de permission s'afficherait | Gestion des permissions personnalisée |
| `SessionStart` | Non | Oui | Initialisation de la session | Initialiser la journalisation et la télémétrie |
| `SessionEnd` | Non | Oui | Terminaison de la session | Nettoyer les ressources temporaires |
| `Notification` | Non | Oui | Messages d'état de l'agent | Envoyer les mises à jour d'état de l'agent à Slack ou PagerDuty |

## Cas d'usage courants

Les hooks sont suffisamment flexibles pour gérer de nombreux scénarios différents. Voici certains des modèles les plus courants organisés par catégorie.

<Tabs>
  <Tab title="Sécurité">
    - Bloquer les commandes dangereuses (comme `rm -rf /`, SQL destructif)
    - Valider les chemins de fichiers avant les opérations d'écriture
    - Appliquer les listes blanches/noires pour l'utilisation des outils
  </Tab>
  <Tab title="Journalisation">
    - Créer des pistes d'audit de toutes les actions de l'agent
    - Suivre les métriques d'exécution et les performances
    - Déboguer le comportement de l'agent en développement
  </Tab>
  <Tab title="Interception d'outils">
    - Rediriger les opérations de fichiers vers des répertoires en sandbox
    - Injecter des variables d'environnement ou des identifiants
    - Transformer les entrées ou sorties des outils
  </Tab>
  <Tab title="Autorisation">
    - Implémenter le contrôle d'accès basé sur les rôles
    - Exiger une approbation humaine pour les opérations sensibles
    - Limiter le débit d'utilisation d'outils spécifiques
  </Tab>
</Tabs>

## Configurer les hooks

Pour configurer un hook pour votre agent, transmettez le hook dans le paramètre `options.hooks` lors de l'appel de `query()` :

<CodeGroup>

```python Python
async for message in query(
    prompt="Your prompt",
    options=ClaudeAgentOptions(
        hooks={
            'PreToolUse': [HookMatcher(matcher='Bash', hooks=[my_callback])]
        }
    )
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "Your prompt",
  options: {
    hooks: {
      PreToolUse: [{ matcher: 'Bash', hooks: [myCallback] }]
    }
  }
})) {
  console.log(message);
}
```

</CodeGroup>

L'option `hooks` est un dictionnaire (Python) ou un objet (TypeScript) où :
- **Les clés** sont des [noms d'événements de hook](#available-hooks) (par exemple, `'PreToolUse'`, `'PostToolUse'`, `'Stop'`)
- **Les valeurs** sont des tableaux de [matchers](#matchers), chacun contenant un motif de filtre optionnel et vos [fonctions de rappel](#callback-function-inputs)

Vos fonctions de rappel de hook reçoivent des [données d'entrée](#input-data) sur l'événement et retournent une [réponse](#callback-outputs) pour que l'agent sache s'il faut autoriser, bloquer ou modifier l'opération.

### Matchers

Utilisez les matchers pour filtrer les outils qui déclenchent vos rappels :

| Option | Type | Par défaut | Description |
|--------|------|---------|-------------|
| `matcher` | `string` | `undefined` | Motif regex pour faire correspondre les noms d'outils. Les outils intégrés incluent `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Task` et autres. Les outils MCP utilisent le motif `mcp__<server>__<action>`. |
| `hooks` | `HookCallback[]` | - | Requis. Tableau de fonctions de rappel à exécuter lorsque le motif correspond |
| `timeout` | `number` | `60` | Délai d'expiration en secondes ; augmentez pour les hooks qui effectuent des appels API externes |

Utilisez le motif `matcher` pour cibler des outils spécifiques chaque fois que possible. Un matcher avec `'Bash'` s'exécute uniquement pour les commandes Bash, tandis que l'omission du motif exécute vos rappels pour chaque appel d'outil. Notez que les matchers ne filtrent que par **nom d'outil**, pas par chemins de fichiers ou autres arguments—pour filtrer par chemin de fichier, vérifiez `tool_input.file_path` à l'intérieur de votre rappel.

Les matchers s'appliquent uniquement aux hooks basés sur les outils (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`). Pour les hooks de cycle de vie comme `Stop`, `SessionStart` et `Notification`, les matchers sont ignorés et le hook se déclenche pour tous les événements de ce type.

<Tip>
**Découvrir les noms d'outils :** Vérifiez le tableau `tools` dans le message système initial au démarrage de votre session, ou ajoutez un hook sans matcher pour journaliser tous les appels d'outils.

**Nommage des outils MCP :** Les outils MCP commencent toujours par `mcp__` suivi du nom du serveur et de l'action : `mcp__<server>__<action>`. Par exemple, si vous configurez un serveur nommé `playwright`, ses outils seront nommés `mcp__playwright__browser_screenshot`, `mcp__playwright__browser_click`, etc. Le nom du serveur provient de la clé que vous utilisez dans la configuration `mcpServers`.
</Tip>

Cet exemple utilise un matcher pour exécuter un hook uniquement pour les outils de modification de fichiers lorsque l'événement `PreToolUse` se déclenche :

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            HookMatcher(matcher='Write|Edit', hooks=[validate_file_path])
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    PreToolUse: [
      { matcher: 'Write|Edit', hooks: [validateFilePath] }
    ]
  }
};
```

</CodeGroup>

### Entrées de la fonction de rappel

Chaque rappel de hook reçoit trois arguments :

1. **Données d'entrée** (`dict` / `HookInput`) : Détails de l'événement. Voir [données d'entrée](#input-data) pour les champs
2. **ID d'utilisation d'outil** (`str | None` / `string | null`) : Corréler les événements `PreToolUse` et `PostToolUse`
3. **Contexte** (`HookContext`) : En TypeScript, contient une propriété `signal` (`AbortSignal`) pour l'annulation. Transmettez-la aux opérations asynchrones comme `fetch()` pour qu'elles s'annulent automatiquement si le hook expire. En Python, cet argument est réservé pour une utilisation future.

### Données d'entrée

Le premier argument de votre rappel de hook contient des informations sur l'événement. Les noms de champs sont identiques entre les SDK (tous deux utilisent snake_case).

**Champs communs** présents dans tous les types de hooks :

| Champ | Type | Description |
|-------|------|-------------|
| `hook_event_name` | `string` | Le type de hook (`PreToolUse`, `PostToolUse`, etc.) |
| `session_id` | `string` | Identifiant de session actuel |
| `transcript_path` | `string` | Chemin vers la transcription de conversation |
| `cwd` | `string` | Répertoire de travail actuel |

**Champs spécifiques au hook** varient selon le type de hook. Les éléments marqués <sup>TS</sup> ne sont disponibles que dans le SDK TypeScript :

| Champ | Type | Description | Hooks |
|-------|------|-------------|-------|
| `tool_name` | `string` | Nom de l'outil appelé | PreToolUse, PostToolUse, PostToolUseFailure<sup>TS</sup>, PermissionRequest<sup>TS</sup> |
| `tool_input` | `object` | Arguments passés à l'outil | PreToolUse, PostToolUse, PostToolUseFailure<sup>TS</sup>, PermissionRequest<sup>TS</sup> |
| `tool_response` | `any` | Résultat retourné par l'exécution de l'outil | PostToolUse |
| `error` | `string` | Message d'erreur de l'échec de l'exécution de l'outil | PostToolUseFailure<sup>TS</sup> |
| `is_interrupt` | `boolean` | Si l'échec a été causé par une interruption | PostToolUseFailure<sup>TS</sup> |
| `prompt` | `string` | Le texte de l'invite utilisateur | UserPromptSubmit |
| `stop_hook_active` | `boolean` | Si un hook d'arrêt est actuellement en cours de traitement | Stop, SubagentStop |
| `agent_id` | `string` | Identifiant unique du sous-agent | SubagentStart<sup>TS</sup>, SubagentStop<sup>TS</sup> |
| `agent_type` | `string` | Type/rôle du sous-agent | SubagentStart<sup>TS</sup> |
| `agent_transcript_path` | `string` | Chemin vers la transcription de conversation du sous-agent | SubagentStop<sup>TS</sup> |
| `trigger` | `string` | Ce qui a déclenché la compaction : `manual` ou `auto` | PreCompact |
| `custom_instructions` | `string` | Instructions personnalisées fournies pour la compaction | PreCompact |
| `permission_suggestions` | `array` | Mises à jour de permissions suggérées pour l'outil | PermissionRequest<sup>TS</sup> |
| `source` | `string` | Comment la session a commencé : `startup`, `resume`, `clear` ou `compact` | SessionStart<sup>TS</sup> |
| `reason` | `string` | Pourquoi la session s'est terminée : `clear`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled` ou `other` | SessionEnd<sup>TS</sup> |
| `message` | `string` | Message d'état de l'agent | Notification<sup>TS</sup> |
| `notification_type` | `string` | Type de notification : `permission_prompt`, `idle_prompt`, `auth_success` ou `elicitation_dialog` | Notification<sup>TS</sup> |
| `title` | `string` | Titre optionnel défini par l'agent | Notification<sup>TS</sup> |

Le code ci-dessous définit un rappel de hook qui utilise `tool_name` et `tool_input` pour journaliser les détails de chaque appel d'outil :

<CodeGroup>

```python Python
async def log_tool_calls(input_data, tool_use_id, context):
    if input_data['hook_event_name'] == 'PreToolUse':
        print(f"Tool: {input_data['tool_name']}")
        print(f"Input: {input_data['tool_input']}")
    return {}
```

```typescript TypeScript
const logToolCalls: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name === 'PreToolUse') {
    const preInput = input as PreToolUseHookInput;
    console.log(`Tool: ${preInput.tool_name}`);
    console.log(`Input:`, preInput.tool_input);
  }
  return {};
};
```

</CodeGroup>

### Sorties du rappel

Votre fonction de rappel retourne un objet qui indique au SDK comment procéder. Retournez un objet vide `{}` pour autoriser l'opération sans modifications. Pour bloquer, modifier ou ajouter du contexte à l'opération, retournez un objet avec un champ `hookSpecificOutput` contenant votre décision.

**Champs de niveau supérieur** (en dehors de `hookSpecificOutput`) :

| Champ | Type | Description |
|-------|------|-------------|
| `continue` | `boolean` | Si l'agent doit continuer après ce hook (par défaut : `true`) |
| `stopReason` | `string` | Message affiché lorsque `continue` est `false` |
| `suppressOutput` | `boolean` | Masquer stdout de la transcription (par défaut : `false`) |
| `systemMessage` | `string` | Message injecté dans la conversation pour que Claude le voie |

**Champs à l'intérieur de `hookSpecificOutput`** :

| Champ | Type | Hooks | Description |
|-------|------|-------|-------------|
| `hookEventName` | `string` | Tous | Requis. Utilisez `input.hook_event_name` pour faire correspondre l'événement actuel |
| `permissionDecision` | `'allow'` \| `'deny'` \| `'ask'` | PreToolUse | Contrôle si l'outil s'exécute |
| `permissionDecisionReason` | `string` | PreToolUse | Explication affichée à Claude pour la décision |
| `updatedInput` | `object` | PreToolUse | Entrée d'outil modifiée (nécessite `permissionDecision: 'allow'`) |
| `additionalContext` | `string` | PostToolUse, UserPromptSubmit, SessionStart<sup>TS</sup>, SubagentStart<sup>TS</sup> | Contexte ajouté à la conversation |

Cet exemple bloque les opérations d'écriture dans le répertoire `/etc` tout en injectant un message système pour rappeler à Claude les pratiques sûres de manipulation de fichiers :

<CodeGroup>

```python Python
async def block_etc_writes(input_data, tool_use_id, context):
    file_path = input_data['tool_input'].get('file_path', '')

    if file_path.startswith('/etc'):
        return {
            # Champ de niveau supérieur : injecter des conseils dans la conversation
            'systemMessage': 'Remember: system directories like /etc are protected.',
            # hookSpecificOutput : bloquer l'opération
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'deny',
                'permissionDecisionReason': 'Writing to /etc is not allowed'
            }
        }
    return {}
```

```typescript TypeScript
const blockEtcWrites: HookCallback = async (input, toolUseID, { signal }) => {
  const filePath = (input as PreToolUseHookInput).tool_input?.file_path as string;

  if (filePath?.startsWith('/etc')) {
    return {
      // Champ de niveau supérieur : injecter des conseils dans la conversation
      systemMessage: 'Remember: system directories like /etc are protected.',
      // hookSpecificOutput : bloquer l'opération
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'deny',
        permissionDecisionReason: 'Writing to /etc is not allowed'
      }
    };
  }
  return {};
};
```

</CodeGroup>

#### Flux de décision de permission

Lorsque plusieurs hooks ou règles de permission s'appliquent, le SDK les évalue dans cet ordre :

1. **Les règles Deny** sont vérifiées en premier (toute correspondance = refus immédiat).
2. **Les règles Ask** sont vérifiées en second.
3. **Les règles Allow** sont vérifiées en troisième.
4. **Par défaut Ask** si rien ne correspond.

Si un hook retourne `deny`, l'opération est bloquée—les autres hooks retournant `allow` ne l'annuleront pas.

#### Bloquer un outil

Retournez une décision de refus pour empêcher l'exécution de l'outil :

<CodeGroup>

```python Python
async def block_dangerous_commands(input_data, tool_use_id, context):
    if input_data['hook_event_name'] != 'PreToolUse':
        return {}

    command = input_data['tool_input'].get('command', '')

    if 'rm -rf /' in command:
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'deny',
                'permissionDecisionReason': 'Dangerous command blocked: rm -rf /'
            }
        }
    return {}
```

```typescript TypeScript
const blockDangerousCommands: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name !== 'PreToolUse') return {};

  const command = (input as PreToolUseHookInput).tool_input.command as string;

  if (command?.includes('rm -rf /')) {
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'deny',
        permissionDecisionReason: 'Dangerous command blocked: rm -rf /'
      }
    };
  }
  return {};
};
```

</CodeGroup>

#### Modifier l'entrée de l'outil

Retournez une entrée mise à jour pour modifier ce que l'outil reçoit :

<CodeGroup>

```python Python
async def redirect_to_sandbox(input_data, tool_use_id, context):
    if input_data['hook_event_name'] != 'PreToolUse':
        return {}

    if input_data['tool_name'] == 'Write':
        original_path = input_data['tool_input'].get('file_path', '')
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'allow',
                'updatedInput': {
                    **input_data['tool_input'],
                    'file_path': f'/sandbox{original_path}'
                }
            }
        }
    return {}
```

```typescript TypeScript
const redirectToSandbox: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name !== 'PreToolUse') return {};

  const preInput = input as PreToolUseHookInput;
  if (preInput.tool_name === 'Write') {
    const originalPath = preInput.tool_input.file_path as string;
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'allow',
        updatedInput: {
          ...preInput.tool_input,
          file_path: `/sandbox${originalPath}`
        }
      }
    };
  }
  return {};
};
```

</CodeGroup>

<Note>
Lors de l'utilisation de `updatedInput`, vous devez également inclure `permissionDecision`. Retournez toujours un nouvel objet plutôt que de muter le `tool_input` original.
</Note>

#### Ajouter un message système

Injecter du contexte dans la conversation :

<CodeGroup>

```python Python
async def add_security_reminder(input_data, tool_use_id, context):
    return {
        'systemMessage': 'Remember to follow security best practices.'
    }
```

```typescript TypeScript
const addSecurityReminder: HookCallback = async (input, toolUseID, { signal }) => {
  return {
    systemMessage: 'Remember to follow security best practices.'
  };
};
```

</CodeGroup>

#### Approuver automatiquement des outils spécifiques

Contourner les invites de permission pour les outils de confiance. C'est utile lorsque vous souhaitez que certaines opérations s'exécutent sans confirmation de l'utilisateur :

<CodeGroup>

```python Python
async def auto_approve_read_only(input_data, tool_use_id, context):
    if input_data['hook_event_name'] != 'PreToolUse':
        return {}

    read_only_tools = ['Read', 'Glob', 'Grep', 'LS']
    if input_data['tool_name'] in read_only_tools:
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'allow',
                'permissionDecisionReason': 'Read-only tool auto-approved'
            }
        }
    return {}
```

```typescript TypeScript
const autoApproveReadOnly: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name !== 'PreToolUse') return {};

  const preInput = input as PreToolUseHookInput;
  const readOnlyTools = ['Read', 'Glob', 'Grep', 'LS'];
  if (readOnlyTools.includes(preInput.tool_name)) {
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'allow',
        permissionDecisionReason: 'Read-only tool auto-approved'
      }
    };
  }
  return {};
};
```

</CodeGroup>

<Note>
Le champ `permissionDecision` accepte trois valeurs : `'allow'` (approbation automatique), `'deny'` (bloquer) ou `'ask'` (demander une confirmation).
</Note>

## Gérer les scénarios avancés

Ces modèles vous aident à construire des systèmes de hooks plus sophistiqués pour les cas d'usage complexes.

### Chaîner plusieurs hooks

Les hooks s'exécutent dans l'ordre dans lequel ils apparaissent dans le tableau. Gardez chaque hook concentré sur une seule responsabilité et chaînez plusieurs hooks pour une logique complexe. Cet exemple exécute les quatre hooks pour chaque appel d'outil (aucun matcher spécifié) :

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            HookMatcher(hooks=[rate_limiter]),        # Premier : vérifier les limites de débit
            HookMatcher(hooks=[authorization_check]), # Deuxième : vérifier les permissions
            HookMatcher(hooks=[input_sanitizer]),     # Troisième : nettoyer les entrées
            HookMatcher(hooks=[audit_logger])         # Dernier : journaliser l'action
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    'PreToolUse': [
      { hooks: [rateLimiter] },        // Premier : vérifier les limites de débit
      { hooks: [authorizationCheck] }, // Deuxième : vérifier les permissions
      { hooks: [inputSanitizer] },     // Troisième : nettoyer les entrées
      { hooks: [auditLogger] }         // Dernier : journaliser l'action
    ]
  }
};
```

</CodeGroup>

### Matchers d'outils spécifiques avec regex

Utilisez des motifs regex pour faire correspondre plusieurs outils :

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            # Faire correspondre les outils de modification de fichiers
            HookMatcher(matcher='Write|Edit|Delete', hooks=[file_security_hook]),

            # Faire correspondre tous les outils MCP
            HookMatcher(matcher='^mcp__', hooks=[mcp_audit_hook]),

            # Faire correspondre tout (pas de matcher)
            HookMatcher(hooks=[global_logger])
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    'PreToolUse': [
      // Faire correspondre les outils de modification de fichiers
      { matcher: 'Write|Edit|Delete', hooks: [fileSecurityHook] },

      // Faire correspondre tous les outils MCP
      { matcher: '^mcp__', hooks: [mcpAuditHook] },

      // Faire correspondre tout (pas de matcher)
      { hooks: [globalLogger] }
    ]
  }
};
```

</CodeGroup>

<Note>
Les matchers ne correspondent qu'aux **noms d'outils**, pas aux chemins de fichiers ou autres arguments. Pour filtrer par chemin de fichier, vérifiez `tool_input.file_path` à l'intérieur de votre rappel de hook.
</Note>

### Suivi de l'activité des sous-agents

Utilisez les hooks `SubagentStop` pour surveiller l'achèvement des sous-agents. Le `tool_use_id` aide à corréler les appels de l'agent parent avec leurs sous-agents :

<CodeGroup>

```python Python
async def subagent_tracker(input_data, tool_use_id, context):
    if input_data['hook_event_name'] == 'SubagentStop':
        print(f"[SUBAGENT] Completed")
        print(f"  Tool use ID: {tool_use_id}")
        print(f"  Stop hook active: {input_data.get('stop_hook_active')}")
    return {}

options = ClaudeAgentOptions(
    hooks={
        'SubagentStop': [HookMatcher(hooks=[subagent_tracker])]
    }
)
```

```typescript TypeScript
const subagentTracker: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name === 'SubagentStop') {
    console.log(`[SUBAGENT] Completed`);
    console.log(`  Tool use ID: ${toolUseID}`);
    console.log(`  Stop hook active: ${input.stop_hook_active}`);
  }
  return {};
};

const options = {
  hooks: {
    SubagentStop: [{ hooks: [subagentTracker] }]
  }
};
```

</CodeGroup>

### Opérations asynchrones dans les hooks

Les hooks peuvent effectuer des opérations asynchrones comme les requêtes HTTP. Gérez les erreurs correctement en capturant les exceptions au lieu de les lever. En TypeScript, transmettez le `signal` à `fetch()` pour que la requête s'annule si le hook expire :

<CodeGroup>

```python Python
import aiohttp
from datetime import datetime

async def webhook_notifier(input_data, tool_use_id, context):
    if input_data['hook_event_name'] != 'PostToolUse':
        return {}

    try:
        async with aiohttp.ClientSession() as session:
            await session.post(
                'https://api.example.com/webhook',
                json={
                    'tool': input_data['tool_name'],
                    'timestamp': datetime.now().isoformat()
                }
            )
    except Exception as e:
        print(f'Webhook request failed: {e}')

    return {}
```

```typescript TypeScript
const webhookNotifier: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name !== 'PostToolUse') return {};

  try {
    // Transmettre le signal pour une annulation appropriée
    await fetch('https://api.example.com/webhook', {
      method: 'POST',
      body: JSON.stringify({
        tool: (input as PostToolUseHookInput).tool_name,
        timestamp: new Date().toISOString()
      }),
      signal
    });
  } catch (error) {
    if (error instanceof Error && error.name === 'AbortError') {
      console.log('Webhook request cancelled');
    }
  }

  return {};
};
```

</CodeGroup>

### Envoyer des notifications (TypeScript uniquement)

Utilisez les hooks `Notification` pour recevoir les mises à jour d'état de l'agent et les transférer vers des services externes comme Slack ou des tableaux de bord de surveillance :

```typescript TypeScript
import { query, HookCallback, NotificationHookInput } from "@anthropic-ai/claude-agent-sdk";

const notificationHandler: HookCallback = async (input, toolUseID, { signal }) => {
  const notification = input as NotificationHookInput;

  await fetch('https://hooks.slack.com/services/YOUR/WEBHOOK/URL', {
    method: 'POST',
    body: JSON.stringify({
      text: `Agent status: ${notification.message}`
    }),
    signal
  });

  return {};
};

for await (const message of query({
  prompt: "Analyze this codebase",
  options: {
    hooks: {
      Notification: [{ hooks: [notificationHandler] }]
    }
  }
})) {
  console.log(message);
}
```

## Corriger les problèmes courants

Cette section couvre les problèmes courants et comment les résoudre.

### Le hook ne se déclenche pas

- Vérifiez que le nom de l'événement du hook est correct et sensible à la casse (`PreToolUse`, pas `preToolUse`)
- Vérifiez que votre motif de matcher correspond exactement au nom de l'outil
- Assurez-vous que le hook se trouve sous le type d'événement correct dans `options.hooks`
- Pour les hooks `SubagentStop`, `Stop`, `SessionStart`, `SessionEnd` et `Notification`, les matchers sont ignorés. Ces hooks se déclenchent pour tous les événements de ce type.
- Les hooks peuvent ne pas se déclencher lorsque l'agent atteint la limite [`max_turns`](/docs/fr/agent-sdk/python#configuration-options) car la session se termine avant que les hooks puissent s'exécuter

### Le matcher ne filtre pas comme prévu

Les matchers ne correspondent qu'aux **noms d'outils**, pas aux chemins de fichiers ou autres arguments. Pour filtrer par chemin de fichier, vérifiez `tool_input.file_path` à l'intérieur de votre hook :

```typescript
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const filePath = preInput.tool_input?.file_path as string;
  if (!filePath?.endsWith('.md')) return {};  // Ignorer les fichiers non-markdown
  // Traiter les fichiers markdown...
};
```

### Expiration du hook

- Augmentez la valeur `timeout` dans la configuration `HookMatcher`
- Utilisez le `AbortSignal` du troisième argument de rappel pour gérer l'annulation correctement en TypeScript

### Outil bloqué de manière inattendue

- Vérifiez tous les hooks `PreToolUse` pour les retours `permissionDecision: 'deny'`
- Ajoutez une journalisation à vos hooks pour voir quel `permissionDecisionReason` ils retournent
- Vérifiez que les motifs de matcher ne sont pas trop larges (un matcher vide correspond à tous les outils)

### Entrée modifiée non appliquée

- Assurez-vous que `updatedInput` se trouve à l'intérieur de `hookSpecificOutput`, pas au niveau supérieur :

  ```typescript
  return {
    hookSpecificOutput: {
      hookEventName: input.hook_event_name,
      permissionDecision: 'allow',
      updatedInput: { command: 'new command' }
    }
  };
  ```

- Vous devez également retourner `permissionDecision: 'allow'` pour que la modification d'entrée prenne effet
- Incluez `hookEventName` dans `hookSpecificOutput` pour identifier le type de hook pour lequel la sortie est destinée

### Hooks de session non disponibles

Les hooks `SessionStart`, `SessionEnd` et `Notification` ne sont disponibles que dans le SDK TypeScript. Le SDK Python ne supporte pas ces événements en raison de limitations de configuration.

### Les invites de permission des sous-agents se multiplient

Lors du lancement de plusieurs sous-agents, chacun peut demander des permissions séparément. Les sous-agents n'héritent pas automatiquement des permissions de l'agent parent. Pour éviter les invites répétées, utilisez les hooks `PreToolUse` pour approuver automatiquement des outils spécifiques, ou configurez des règles de permission qui s'appliquent aux sessions des sous-agents.

### Boucles de hooks récursives avec des sous-agents

Un hook `UserPromptSubmit` qui lance des sous-agents peut créer des boucles infinies si ces sous-agents déclenchent le même hook. Pour éviter cela :

- Vérifiez un indicateur de sous-agent dans l'entrée du hook avant de lancer
- Utilisez le champ `parent_tool_use_id` pour détecter si vous êtes déjà dans un contexte de sous-agent
- Limitez les hooks pour s'exécuter uniquement pour la session de l'agent de niveau supérieur

### systemMessage n'apparaît pas dans la sortie

Le champ `systemMessage` ajoute du contexte à la conversation que le modèle voit, mais il peut ne pas apparaître dans tous les modes de sortie du SDK. Si vous avez besoin de faire apparaître les décisions des hooks à votre application, journalisez-les séparément ou utilisez un canal de sortie dédié.

## En savoir plus

- [Permissions](/docs/fr/agent-sdk/permissions) : contrôler ce que votre agent peut faire
- [Outils personnalisés](/docs/fr/agent-sdk/custom-tools) : construire des outils pour étendre les capacités de l'agent
- [Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript)
- [Référence du SDK Python](/docs/fr/agent-sdk/python)