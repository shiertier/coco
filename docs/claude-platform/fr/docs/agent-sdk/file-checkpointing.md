# Rembobiner les modifications de fichiers avec les points de contrôle

Suivre les modifications de fichiers pendant les sessions d'agent et restaurer les fichiers à n'importe quel état précédent

---

La sauvegarde des points de contrôle suit les modifications de fichiers effectuées via les outils Write, Edit et NotebookEdit pendant une session d'agent, ce qui vous permet de rembobiner les fichiers à n'importe quel état précédent. Vous voulez l'essayer ? Accédez à l'[exemple interactif](#try-it-out).

Avec les points de contrôle, vous pouvez :

- **Annuler les modifications indésirables** en restaurant les fichiers à un état connu et bon
- **Explorer des alternatives** en restaurant à un point de contrôle et en essayant une approche différente
- **Récupérer après des erreurs** lorsque l'agent effectue des modifications incorrectes

<Warning>
Seules les modifications effectuées via les outils Write, Edit et NotebookEdit sont suivies. Les modifications effectuées via des commandes Bash (comme `echo > file.txt` ou `sed -i`) ne sont pas capturées par le système de point de contrôle.
</Warning>

## Comment fonctionnent les points de contrôle

Lorsque vous activez la sauvegarde des points de contrôle de fichiers, le SDK crée des sauvegardes de fichiers avant de les modifier via les outils Write, Edit ou NotebookEdit. Les messages utilisateur dans le flux de réponse incluent un UUID de point de contrôle que vous pouvez utiliser comme point de restauration.

Le point de contrôle fonctionne avec ces outils intégrés que l'agent utilise pour modifier les fichiers :

| Outil | Description |
|------|-------------|
| Write | Crée un nouveau fichier ou remplace un fichier existant par un nouveau contenu |
| Edit | Effectue des modifications ciblées sur des parties spécifiques d'un fichier existant |
| NotebookEdit | Modifie les cellules dans les notebooks Jupyter (fichiers `.ipynb`) |

<Note>
La restauration de fichiers restaure les fichiers sur le disque à un état précédent. Elle ne rembobine pas la conversation elle-même. L'historique de la conversation et le contexte restent intacts après l'appel de `rewindFiles()` (TypeScript) ou `rewind_files()` (Python).
</Note>

Le système de point de contrôle suit :

- Les fichiers créés pendant la session
- Les fichiers modifiés pendant la session
- Le contenu original des fichiers modifiés

Lorsque vous rembobinez à un point de contrôle, les fichiers créés sont supprimés et les fichiers modifiés sont restaurés à leur contenu à ce moment-là.

## Implémenter les points de contrôle

Pour utiliser la sauvegarde des points de contrôle de fichiers, activez-la dans vos options, capturez les UUID des points de contrôle du flux de réponse, puis appelez `rewindFiles()` (TypeScript) ou `rewind_files()` (Python) lorsque vous devez restaurer.

L'exemple suivant montre le flux complet : activez les points de contrôle, capturez l'UUID du point de contrôle et l'ID de session du flux de réponse, puis reprenez la session plus tard pour rembobiner les fichiers. Chaque étape est expliquée en détail ci-dessous.

<CodeGroup>

```python Python
import asyncio
import os
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

async def main():
    # Step 1: Enable checkpointing
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",  # Auto-accept file edits without prompting
        extra_args={"replay-user-messages": None},  # Required to receive checkpoint UUIDs in the response stream
        env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
    )

    checkpoint_id = None
    session_id = None

    # Run the query and capture checkpoint UUID and session ID
    async with ClaudeSDKClient(options) as client:
        await client.query("Refactor the authentication module")

        # Step 2: Capture checkpoint UUID from the first user message
        async for message in client.receive_response():
            if isinstance(message, UserMessage) and message.uuid and not checkpoint_id:
                checkpoint_id = message.uuid
            if isinstance(message, ResultMessage) and not session_id:
                session_id = message.session_id

    # Step 3: Later, rewind by resuming the session with an empty prompt
    if checkpoint_id and session_id:
        async with ClaudeSDKClient(ClaudeAgentOptions(
            enable_file_checkpointing=True,
            resume=session_id
        )) as client:
            await client.query("")  # Empty prompt to open the connection
            async for message in client.receive_response():
                await client.rewind_files(checkpoint_id)
                break
        print(f"Rewound to checkpoint: {checkpoint_id}")

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

async function main() {
  // Step 1: Enable checkpointing
  const opts = {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,  // Auto-accept file edits without prompting
    extraArgs: { 'replay-user-messages': null },  // Required to receive checkpoint UUIDs in the response stream
    env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
  };

  const response = query({
    prompt: "Refactor the authentication module",
    options: opts
  });

  let checkpointId: string | undefined;
  let sessionId: string | undefined;

  // Step 2: Capture checkpoint UUID from the first user message
  for await (const message of response) {
    if (message.type === 'user' && message.uuid && !checkpointId) {
      checkpointId = message.uuid;
    }
    if ('session_id' in message && !sessionId) {
      sessionId = message.session_id;
    }
  }

  // Step 3: Later, rewind by resuming the session with an empty prompt
  if (checkpointId && sessionId) {
    const rewindQuery = query({
      prompt: "",  // Empty prompt to open the connection
      options: { ...opts, resume: sessionId }
    });

    for await (const msg of rewindQuery) {
      await rewindQuery.rewindFiles(checkpointId);
      break;
    }
    console.log(`Rewound to checkpoint: ${checkpointId}`);
  }
}

main();
```

</CodeGroup>

<Steps>

<Step title="Définir la variable d'environnement">

La sauvegarde des points de contrôle de fichiers nécessite la variable d'environnement `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING`. Vous pouvez la définir soit via la ligne de commande avant d'exécuter votre script, soit directement dans les options du SDK.

**Option 1 : Définir via la ligne de commande**

```bash Python
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
```

**Option 2 : Définir dans les options du SDK**

Transmettez la variable d'environnement via l'option `env` lors de la configuration du SDK :

<CodeGroup>

```python Python
import os

options = ClaudeAgentOptions(
    enable_file_checkpointing=True,
    env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
)
```

```typescript TypeScript
const opts = {
  enableFileCheckpointing: true,
  env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
};
```

</CodeGroup>

</Step>

<Step title="Activer les points de contrôle">

Configurez vos options du SDK pour activer les points de contrôle et recevoir les UUID des points de contrôle :

| Option | Python | TypeScript | Description |
|--------|--------|------------|-------------|
| Activer les points de contrôle | `enable_file_checkpointing=True` | `enableFileCheckpointing: true` | Suit les modifications de fichiers pour le rembobinage |
| Recevoir les UUID des points de contrôle | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | Requis pour obtenir les UUID des messages utilisateur dans le flux |

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    enable_file_checkpointing=True,
    permission_mode="acceptEdits",
    extra_args={"replay-user-messages": None}
)

async with ClaudeSDKClient(options) as client:
    await client.query("Refactor the authentication module")
```

```typescript TypeScript
const response = query({
  prompt: "Refactor the authentication module",
  options: {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,
    extraArgs: { 'replay-user-messages': null }
  }
});
```

</CodeGroup>

</Step>

<Step title="Capturer l'UUID du point de contrôle et l'ID de session">

Avec l'option `replay-user-messages` définie (comme indiqué ci-dessus), chaque message utilisateur dans le flux de réponse a un UUID qui sert de point de contrôle.

Pour la plupart des cas d'utilisation, capturez l'UUID du premier message utilisateur (`message.uuid`) ; le rembobinage vers celui-ci restaure tous les fichiers à leur état d'origine. Pour stocker plusieurs points de contrôle et rembobiner vers des états intermédiaires, voir [Points de restauration multiples](#multiple-restore-points).

La capture de l'ID de session (`message.session_id`) est facultative ; vous n'en avez besoin que si vous souhaitez rembobiner plus tard, après la fin du flux. Si vous appelez `rewindFiles()` immédiatement tout en traitant les messages (comme l'exemple dans [Point de contrôle avant les opérations risquées](#checkpoint-before-risky-operations) le fait), vous pouvez ignorer la capture de l'ID de session.

<CodeGroup>

```python Python
checkpoint_id = None
session_id = None

async for message in client.receive_response():
    # Update checkpoint on each user message (keeps the latest)
    if isinstance(message, UserMessage) and message.uuid:
        checkpoint_id = message.uuid
    # Capture session ID from the result message
    if isinstance(message, ResultMessage):
        session_id = message.session_id
```

```typescript TypeScript
let checkpointId: string | undefined;
let sessionId: string | undefined;

for await (const message of response) {
  // Update checkpoint on each user message (keeps the latest)
  if (message.type === 'user' && message.uuid) {
    checkpointId = message.uuid;
  }
  // Capture session ID from any message that has it
  if ('session_id' in message) {
    sessionId = message.session_id;
  }
}
```

</CodeGroup>

</Step>

<Step title="Rembobiner les fichiers">

Pour rembobiner après la fin du flux, reprenez la session avec une invite vide et appelez `rewind_files()` (Python) ou `rewindFiles()` (TypeScript) avec votre UUID de point de contrôle. Vous pouvez également rembobiner pendant le flux ; voir [Point de contrôle avant les opérations risquées](#checkpoint-before-risky-operations) pour ce modèle.

<CodeGroup>

```python Python
async with ClaudeSDKClient(ClaudeAgentOptions(
    enable_file_checkpointing=True,
    resume=session_id
)) as client:
    await client.query("")  # Empty prompt to open the connection
    async for message in client.receive_response():
        await client.rewind_files(checkpoint_id)
        break
```

```typescript TypeScript
const rewindQuery = query({
  prompt: "",  // Empty prompt to open the connection
  options: { ...opts, resume: sessionId }
});

for await (const msg of rewindQuery) {
  await rewindQuery.rewindFiles(checkpointId);
  break;
}
```

</CodeGroup>

Si vous capturez l'ID de session et l'UUID du point de contrôle, vous pouvez également rembobiner depuis la CLI :

```bash
claude --resume <session-id> --rewind-files <checkpoint-uuid>
```

</Step>

</Steps>

## Modèles courants

Ces modèles montrent différentes façons de capturer et d'utiliser les UUID des points de contrôle en fonction de votre cas d'utilisation.

### Point de contrôle avant les opérations risquées

Ce modèle conserve uniquement l'UUID du point de contrôle le plus récent, le mettant à jour avant chaque tour d'agent. Si quelque chose se passe mal pendant le traitement, vous pouvez immédiatement rembobiner vers le dernier état sûr et sortir de la boucle.

<CodeGroup>

```python Python
import asyncio
import os
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage

async def main():
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",
        extra_args={"replay-user-messages": None},
        env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
    )

    safe_checkpoint = None

    async with ClaudeSDKClient(options) as client:
        await client.query("Refactor the authentication module")

        async for message in client.receive_response():
            # Update checkpoint before each agent turn starts
            # This overwrites the previous checkpoint. Only keep the latest
            if isinstance(message, UserMessage) and message.uuid:
                safe_checkpoint = message.uuid

            # Decide when to revert based on your own logic
            # For example: error detection, validation failure, or user input
            if your_revert_condition and safe_checkpoint:
                await client.rewind_files(safe_checkpoint)
                # Exit the loop after rewinding, files are restored
                break

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

async function main() {
  const response = query({
    prompt: "Refactor the authentication module",
    options: {
      enableFileCheckpointing: true,
      permissionMode: "acceptEdits" as const,
      extraArgs: { 'replay-user-messages': null },
      env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
    }
  });

  let safeCheckpoint: string | undefined;

  for await (const message of response) {
    // Update checkpoint before each agent turn starts
    // This overwrites the previous checkpoint. Only keep the latest
    if (message.type === 'user' && message.uuid) {
      safeCheckpoint = message.uuid;
    }

    // Decide when to revert based on your own logic
    // For example: error detection, validation failure, or user input
    if (yourRevertCondition && safeCheckpoint) {
      await response.rewindFiles(safeCheckpoint);
      // Exit the loop after rewinding, files are restored
      break;
    }
  }
}

main();
```

</CodeGroup>

### Points de restauration multiples

Si Claude effectue des modifications sur plusieurs tours, vous pourriez vouloir rembobiner à un point spécifique plutôt que complètement. Par exemple, si Claude refactorise un fichier au tour un et ajoute des tests au tour deux, vous pourriez vouloir conserver la refactorisation mais annuler les tests.

Ce modèle stocke tous les UUID des points de contrôle dans un tableau avec des métadonnées. Après la fin de la session, vous pouvez rembobiner à n'importe quel point de contrôle précédent :

<CodeGroup>

```python Python
import asyncio
import os
from dataclasses import dataclass
from datetime import datetime
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

# Store checkpoint metadata for better tracking
@dataclass
class Checkpoint:
    id: str
    description: str
    timestamp: datetime

async def main():
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",
        extra_args={"replay-user-messages": None},
        env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
    )

    checkpoints = []
    session_id = None

    async with ClaudeSDKClient(options) as client:
        await client.query("Refactor the authentication module")

        async for message in client.receive_response():
            if isinstance(message, UserMessage) and message.uuid:
                checkpoints.append(Checkpoint(
                    id=message.uuid,
                    description=f"After turn {len(checkpoints) + 1}",
                    timestamp=datetime.now()
                ))
            if isinstance(message, ResultMessage) and not session_id:
                session_id = message.session_id

    # Later: rewind to any checkpoint by resuming the session
    if checkpoints and session_id:
        target = checkpoints[0]  # Pick any checkpoint
        async with ClaudeSDKClient(ClaudeAgentOptions(
            enable_file_checkpointing=True,
            resume=session_id
        )) as client:
            await client.query("")  # Empty prompt to open the connection
            async for message in client.receive_response():
                await client.rewind_files(target.id)
                break
        print(f"Rewound to: {target.description}")

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Store checkpoint metadata for better tracking
interface Checkpoint {
  id: string;
  description: string;
  timestamp: Date;
}

async function main() {
  const opts = {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,
    extraArgs: { 'replay-user-messages': null },
    env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
  };

  const response = query({
    prompt: "Refactor the authentication module",
    options: opts
  });

  const checkpoints: Checkpoint[] = [];
  let sessionId: string | undefined;

  for await (const message of response) {
    if (message.type === 'user' && message.uuid) {
      checkpoints.push({
        id: message.uuid,
        description: `After turn ${checkpoints.length + 1}`,
        timestamp: new Date()
      });
    }
    if ('session_id' in message && !sessionId) {
      sessionId = message.session_id;
    }
  }

  // Later: rewind to any checkpoint by resuming the session
  if (checkpoints.length > 0 && sessionId) {
    const target = checkpoints[0];  // Pick any checkpoint
    const rewindQuery = query({
      prompt: "",  // Empty prompt to open the connection
      options: { ...opts, resume: sessionId }
    });

    for await (const msg of rewindQuery) {
      await rewindQuery.rewindFiles(target.id);
      break;
    }
    console.log(`Rewound to: ${target.description}`);
  }
}

main();
```

</CodeGroup>

## Essayez-le

Cet exemple complet crée un petit fichier utilitaire, demande à l'agent d'ajouter des commentaires de documentation, vous montre les modifications, puis vous demande si vous souhaitez rembobiner.

Avant de commencer, assurez-vous d'avoir [Claude Agent SDK installé](/docs/fr/agent-sdk/quickstart).

<Steps>

<Step title="Créer un fichier de test">

Créez un nouveau fichier appelé `utils.py` (Python) ou `utils.ts` (TypeScript) et collez le code suivant :

<CodeGroup>

```python utils.py
def add(a, b):
    return a + b

def subtract(a, b):
    return a - b

def multiply(a, b):
    return a * b

def divide(a, b):
    if b == 0:
        raise ValueError("Cannot divide by zero")
    return a / b
```

```typescript utils.ts
export function add(a: number, b: number): number {
  return a + b;
}

export function subtract(a: number, b: number): number {
  return a - b;
}

export function multiply(a: number, b: number): number {
  return a * b;
}

export function divide(a: number, b: number): number {
  if (b === 0) {
    throw new Error("Cannot divide by zero");
  }
  return a / b;
}
```

</CodeGroup>

</Step>

<Step title="Exécuter l'exemple interactif">

Créez un nouveau fichier appelé `try_checkpointing.py` (Python) ou `try_checkpointing.ts` (TypeScript) dans le même répertoire que votre fichier utilitaire, et collez le code suivant.

Ce script demande à Claude d'ajouter des commentaires de documentation à votre fichier utilitaire, puis vous donne la possibilité de rembobiner et de restaurer l'original.

<CodeGroup>

```python try_checkpointing.py
import asyncio
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

async def main():
    # Configure the SDK with checkpointing enabled
    # - enable_file_checkpointing: Track file changes for rewinding
    # - permission_mode: Auto-accept file edits without prompting
    # - extra_args: Required to receive user message UUIDs in the stream
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",
        extra_args={"replay-user-messages": None}
    )

    checkpoint_id = None  # Store the user message UUID for rewinding
    session_id = None     # Store the session ID for resuming

    print("Running agent to add doc comments to utils.py...\n")

    # Run the agent and capture checkpoint data from the response stream
    async with ClaudeSDKClient(options) as client:
        await client.query("Add doc comments to utils.py")

        async for message in client.receive_response():
            # Capture the first user message UUID - this is our restore point
            if isinstance(message, UserMessage) and message.uuid and not checkpoint_id:
                checkpoint_id = message.uuid
            # Capture the session ID so we can resume later
            if isinstance(message, ResultMessage):
                session_id = message.session_id

    print("Done! Open utils.py to see the added doc comments.\n")

    # Ask the user if they want to rewind the changes
    if checkpoint_id and session_id:
        response = input("Rewind to remove the doc comments? (y/n): ")

        if response.lower() == "y":
            # Resume the session with an empty prompt, then rewind
            async with ClaudeSDKClient(ClaudeAgentOptions(
                enable_file_checkpointing=True,
                resume=session_id
            )) as client:
                await client.query("")  # Empty prompt opens the connection
                async for message in client.receive_response():
                    await client.rewind_files(checkpoint_id)  # Restore files
                    break

            print("\n✓ File restored! Open utils.py to verify the doc comments are gone.")
        else:
            print("\nKept the modified file.")

asyncio.run(main())
```

```typescript try_checkpointing.ts
import { query } from "@anthropic-ai/claude-agent-sdk";
import * as readline from "readline";

async function main() {
  // Configure the SDK with checkpointing enabled
  // - enableFileCheckpointing: Track file changes for rewinding
  // - permissionMode: Auto-accept file edits without prompting
  // - extraArgs: Required to receive user message UUIDs in the stream
  const opts = {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,
    extraArgs: { 'replay-user-messages': null }
  };

  let sessionId: string | undefined;    // Store the session ID for resuming
  let checkpointId: string | undefined; // Store the user message UUID for rewinding

  console.log("Running agent to add doc comments to utils.ts...\n");

  // Run the agent and capture checkpoint data from the response stream
  const response = query({
    prompt: "Add doc comments to utils.ts",
    options: opts
  });

  for await (const message of response) {
    // Capture the first user message UUID - this is our restore point
    if (message.type === "user" && message.uuid && !checkpointId) {
      checkpointId = message.uuid;
    }
    // Capture the session ID so we can resume later
    if ("session_id" in message) {
      sessionId = message.session_id;
    }
  }

  console.log("Done! Open utils.ts to see the added doc comments.\n");

  // Ask the user if they want to rewind the changes
  if (checkpointId && sessionId) {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });

    const answer = await new Promise<string>((resolve) => {
      rl.question("Rewind to remove the doc comments? (y/n): ", resolve);
    });
    rl.close();

    if (answer.toLowerCase() === "y") {
      // Resume the session with an empty prompt, then rewind
      const rewindQuery = query({
        prompt: "",  // Empty prompt opens the connection
        options: { ...opts, resume: sessionId }
      });

      for await (const msg of rewindQuery) {
        await rewindQuery.rewindFiles(checkpointId);  // Restore files
        break;
      }

      console.log("\n✓ File restored! Open utils.ts to verify the doc comments are gone.");
    } else {
      console.log("\nKept the modified file.");
    }
  }
}

main();
```

</CodeGroup>

Cet exemple démontre le flux complet de point de contrôle :

1. **Activer les points de contrôle** : configurez le SDK avec `enable_file_checkpointing=True` et `permission_mode="acceptEdits"` pour approuver automatiquement les modifications de fichiers
2. **Capturer les données du point de contrôle** : à mesure que l'agent s'exécute, stockez l'UUID du premier message utilisateur (votre point de restauration) et l'ID de session
3. **Demander le rembobinage** : après la fin de l'agent, vérifiez votre fichier utilitaire pour voir les commentaires de documentation, puis décidez si vous souhaitez annuler les modifications
4. **Reprendre et rembobiner** : si oui, reprenez la session avec une invite vide et appelez `rewind_files()` pour restaurer le fichier d'origine

</Step>

<Step title="Exécuter l'exemple">

Définissez la variable d'environnement et exécutez le script à partir du même répertoire que votre fichier utilitaire.

<Tip>
Ouvrez votre fichier utilitaire (`utils.py` ou `utils.ts`) dans votre IDE ou éditeur avant d'exécuter le script. Vous verrez le fichier se mettre à jour en temps réel à mesure que l'agent ajoute des commentaires de documentation, puis revenir à l'original lorsque vous choisissez de rembobiner.
</Tip>

<CodeGroup>

```bash Python
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
python try_checkpointing.py
```

```bash TypeScript
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
npx tsx try_checkpointing.ts
```

</CodeGroup>

Vous verrez l'agent ajouter des commentaires de documentation, puis une invite vous demandant si vous souhaitez rembobiner. Si vous choisissez oui, le fichier est restauré à son état d'origine.

</Step>

</Steps>

## Limitations

La sauvegarde des points de contrôle de fichiers a les limitations suivantes :

| Limitation | Description |
|------------|-------------|
| Outils Write/Edit/NotebookEdit uniquement | Les modifications effectuées via des commandes Bash ne sont pas suivies |
| Même session | Les points de contrôle sont liés à la session qui les a créés |
| Contenu de fichier uniquement | La création, le déplacement ou la suppression de répertoires ne sont pas annulés par le rembobinage |
| Fichiers locaux | Les fichiers distants ou réseau ne sont pas suivis |

## Dépannage

### Les options de point de contrôle ne sont pas reconnues

Si `enableFileCheckpointing` ou `rewindFiles()` n'est pas disponible, vous utilisez peut-être une version plus ancienne du SDK.

**Solution** : Mettez à jour vers la dernière version du SDK :
- **Python** : `pip install --upgrade claude-agent-sdk`
- **TypeScript** : `npm install @anthropic-ai/claude-agent-sdk@latest`

### Les messages utilisateur n'ont pas d'UUID

Si `message.uuid` est `undefined` ou manquant, vous ne recevez pas les UUID des points de contrôle.

**Cause** : L'option `replay-user-messages` n'est pas définie.

**Solution** : Ajoutez `extra_args={"replay-user-messages": None}` (Python) ou `extraArgs: { 'replay-user-messages': null }` (TypeScript) à vos options.

### Erreur « No file checkpoint found for message »

Cette erreur se produit lorsque les données du point de contrôle n'existent pas pour l'UUID du message utilisateur spécifié.

**Causes courantes** :
- La variable d'environnement `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` n'est pas définie
- La session n'a pas été correctement complétée avant de tenter de reprendre et de rembobiner

**Solution** : Assurez-vous d'avoir défini la variable d'environnement (voir [Définir la variable d'environnement](#set-the-environment-variable)), puis utilisez le modèle montré dans les exemples : capturez l'UUID du premier message utilisateur, complétez la session entièrement, puis reprenez avec une invite vide et appelez `rewindFiles()` une seule fois.

### Erreur « ProcessTransport is not ready for writing »

Cette erreur se produit lorsque vous appelez `rewindFiles()` ou `rewind_files()` après avoir terminé l'itération dans la réponse. La connexion au processus CLI se ferme lorsque la boucle se termine.

**Solution** : Reprenez la session avec une invite vide, puis rembobinez sur la nouvelle requête :

<CodeGroup>

```python Python
# Resume session with empty prompt, then rewind
async with ClaudeSDKClient(ClaudeAgentOptions(
    enable_file_checkpointing=True,
    resume=session_id
)) as client:
    await client.query("")
    async for message in client.receive_response():
        await client.rewind_files(checkpoint_id)
        break
```

```typescript TypeScript
// Resume session with empty prompt, then rewind
const rewindQuery = query({
  prompt: "",
  options: { ...opts, resume: sessionId }
});

for await (const msg of rewindQuery) {
  await rewindQuery.rewindFiles(checkpointId);
  break;
}
```

</CodeGroup>

## Étapes suivantes

- **[Sessions](/docs/fr/agent-sdk/sessions)** : apprenez comment reprendre les sessions, ce qui est requis pour rembobiner après la fin du flux. Couvre les ID de session, la reprise des conversations et la bifurcation de session.
- **[Permissions](/docs/fr/agent-sdk/permissions)** : configurez les outils que Claude peut utiliser et comment les modifications de fichiers sont approuvées. Utile si vous souhaitez plus de contrôle sur le moment où les modifications se produisent.
- **[Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript)** : référence API complète incluant toutes les options pour `query()` et la méthode `rewindFiles()`.
- **[Référence du SDK Python](/docs/fr/agent-sdk/python)** : référence API complète incluant toutes les options pour `ClaudeAgentOptions` et la méthode `rewind_files()`.