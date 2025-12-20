# Dateiveränderungen mit Checkpointing rückgängig machen

Verfolgen Sie Dateiveränderungen während Agent-Sitzungen und stellen Sie Dateien in jeden vorherigen Zustand wieder her

---

File Checkpointing verfolgt Dateiänderungen, die während einer Agent-Sitzung durch die Tools Write, Edit und NotebookEdit vorgenommen werden, und ermöglicht es Ihnen, Dateien in jeden vorherigen Zustand zurückzusetzen. Möchten Sie es ausprobieren? Springen Sie zum [interaktiven Beispiel](#try-it-out).

Mit Checkpointing können Sie:

- **Unerwünschte Änderungen rückgängig machen**, indem Sie Dateien in einen bekannten guten Zustand zurücksetzen
- **Alternativen erkunden**, indem Sie zu einem Checkpoint zurückkehren und einen anderen Ansatz versuchen
- **Von Fehlern wiederherstellen**, wenn der Agent falsche Änderungen vornimmt

<Warning>
Nur Änderungen, die durch die Tools Write, Edit und NotebookEdit vorgenommen werden, werden verfolgt. Änderungen, die durch Bash-Befehle vorgenommen werden (wie `echo > file.txt` oder `sed -i`), werden nicht vom Checkpoint-System erfasst.
</Warning>

## Wie Checkpointing funktioniert

Wenn Sie File Checkpointing aktivieren, erstellt das SDK Sicherungen von Dateien, bevor diese durch die Tools Write, Edit oder NotebookEdit geändert werden. Benutzermeldungen im Response-Stream enthalten eine Checkpoint-UUID, die Sie als Wiederherstellungspunkt verwenden können.

Checkpoint funktioniert mit diesen integrierten Tools, die der Agent zum Ändern von Dateien verwendet:

| Tool | Beschreibung |
|------|-------------|
| Write | Erstellt eine neue Datei oder überschreibt eine vorhandene Datei mit neuem Inhalt |
| Edit | Nimmt gezielte Änderungen an bestimmten Teilen einer vorhandenen Datei vor |
| NotebookEdit | Ändert Zellen in Jupyter-Notebooks (`.ipynb`-Dateien) |

<Note>
File Rewinding stellt Dateien auf der Festplatte in einen vorherigen Zustand wieder her. Es macht das Gespräch selbst nicht rückgängig. Der Gesprächsverlauf und der Kontext bleiben nach dem Aufrufen von `rewindFiles()` (TypeScript) oder `rewind_files()` (Python) intakt.
</Note>

Das Checkpoint-System verfolgt:

- Dateien, die während der Sitzung erstellt wurden
- Dateien, die während der Sitzung geändert wurden
- Den ursprünglichen Inhalt geänderter Dateien

Wenn Sie zu einem Checkpoint zurückspulen, werden erstellte Dateien gelöscht und geänderte Dateien auf ihren Inhalt an diesem Punkt zurückgesetzt.

## Checkpointing implementieren

Um File Checkpointing zu verwenden, aktivieren Sie es in Ihren Optionen, erfassen Sie Checkpoint-UUIDs aus dem Response-Stream und rufen Sie dann `rewindFiles()` (TypeScript) oder `rewind_files()` (Python) auf, wenn Sie wiederherstellen müssen.

Das folgende Beispiel zeigt den vollständigen Ablauf: Aktivieren Sie Checkpointing, erfassen Sie die Checkpoint-UUID und Session-ID aus dem Response-Stream, und setzen Sie die Sitzung später fort, um Dateien zurückzuspulen. Jeder Schritt wird unten im Detail erklärt.

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

<Step title="Umgebungsvariable setzen">

File Checkpointing erfordert die Umgebungsvariable `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING`. Sie können sie entweder über die Befehlszeile vor dem Ausführen Ihres Skripts oder direkt in den SDK-Optionen setzen.

**Option 1: Über Befehlszeile setzen**

```bash Python
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
```

**Option 2: In SDK-Optionen setzen**

Übergeben Sie die Umgebungsvariable durch die `env`-Option beim Konfigurieren des SDK:

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

<Step title="Checkpointing aktivieren">

Konfigurieren Sie Ihre SDK-Optionen, um Checkpointing zu aktivieren und Checkpoint-UUIDs zu empfangen:

| Option | Python | TypeScript | Beschreibung |
|--------|--------|------------|-------------|
| Checkpointing aktivieren | `enable_file_checkpointing=True` | `enableFileCheckpointing: true` | Verfolgt Dateiveränderungen zum Zurückspulen |
| Checkpoint-UUIDs empfangen | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | Erforderlich, um Benutzer-Nachrichten-UUIDs im Stream zu erhalten |

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

<Step title="Checkpoint-UUID und Session-ID erfassen">

Mit der oben gezeigten `replay-user-messages`-Option hat jede Benutzermeldung im Response-Stream eine UUID, die als Checkpoint dient.

Für die meisten Anwendungsfälle erfassen Sie die UUID der ersten Benutzermeldung (`message.uuid`); das Zurückspulen zu ihr stellt alle Dateien in ihren ursprünglichen Zustand wieder her. Um mehrere Checkpoints zu speichern und zu Zwischenzuständen zurückzuspulen, siehe [Mehrere Wiederherstellungspunkte](#multiple-restore-points).

Das Erfassen der Session-ID (`message.session_id`) ist optional; Sie benötigen sie nur, wenn Sie später zurückspulen möchten, nachdem der Stream abgeschlossen ist. Wenn Sie `rewindFiles()` sofort aufrufen, während Sie noch Nachrichten verarbeiten (wie das Beispiel in [Checkpoint vor riskanten Operationen](#checkpoint-before-risky-operations) zeigt), können Sie das Erfassen der Session-ID überspringen.

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

<Step title="Dateien zurückspulen">

Um nach Abschluss des Streams zurückzuspulen, setzen Sie die Sitzung mit einer leeren Eingabeaufforderung fort und rufen Sie `rewind_files()` (Python) oder `rewindFiles()` (TypeScript) mit Ihrer Checkpoint-UUID auf. Sie können auch während des Streams zurückspulen; siehe [Checkpoint vor riskanten Operationen](#checkpoint-before-risky-operations) für dieses Muster.

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

Wenn Sie die Session-ID und Checkpoint-ID erfassen, können Sie auch von der CLI aus zurückspulen:

```bash
claude --resume <session-id> --rewind-files <checkpoint-uuid>
```

</Step>

</Steps>

## Häufige Muster

Diese Muster zeigen verschiedene Möglichkeiten, Checkpoint-UUIDs je nach Ihrem Anwendungsfall zu erfassen und zu verwenden.

### Checkpoint vor riskanten Operationen

Dieses Muster behält nur die neueste Checkpoint-UUID bei und aktualisiert sie vor jeder Agent-Runde. Wenn während der Verarbeitung etwas schief geht, können Sie sofort zum letzten sicheren Zustand zurückspulen und die Schleife verlassen.

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

### Mehrere Wiederherstellungspunkte

Wenn Claude Änderungen über mehrere Runden hinweg vornimmt, möchten Sie möglicherweise zu einem bestimmten Punkt zurückspulen, anstatt ganz nach hinten zu gehen. Wenn Claude beispielsweise eine Datei in Runde eins umgestaltet und in Runde zwei Tests hinzufügt, möchten Sie möglicherweise die Umgestaltung behalten, aber die Tests rückgängig machen.

Dieses Muster speichert alle Checkpoint-UUIDs in einem Array mit Metadaten. Nach Abschluss der Sitzung können Sie zu jedem vorherigen Checkpoint zurückspulen:

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

## Ausprobieren

Dieses vollständige Beispiel erstellt eine kleine Utility-Datei, lässt den Agent Dokumentationskommentare hinzufügen, zeigt Ihnen die Änderungen und fragt, ob Sie zurückspulen möchten.

Bevor Sie beginnen, stellen Sie sicher, dass Sie das [Claude Agent SDK installiert haben](/docs/de/agent-sdk/quickstart).

<Steps>

<Step title="Testdatei erstellen">

Erstellen Sie eine neue Datei namens `utils.py` (Python) oder `utils.ts` (TypeScript) und fügen Sie den folgenden Code ein:

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

<Step title="Interaktives Beispiel ausführen">

Erstellen Sie eine neue Datei namens `try_checkpointing.py` (Python) oder `try_checkpointing.ts` (TypeScript) im selben Verzeichnis wie Ihre Utility-Datei und fügen Sie den folgenden Code ein.

Dieses Skript fordert Claude auf, Dokumentationskommentare zu Ihrer Utility-Datei hinzuzufügen, und gibt Ihnen dann die Möglichkeit, zurückzuspulen und das Original wiederherzustellen.

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

Dieses Beispiel demonstriert den vollständigen Checkpointing-Workflow:

1. **Checkpointing aktivieren**: Konfigurieren Sie das SDK mit `enable_file_checkpointing=True` und `permission_mode="acceptEdits"`, um Dateibearbeitungen automatisch zu genehmigen
2. **Checkpoint-Daten erfassen**: Während der Agent läuft, speichern Sie die UUID der ersten Benutzermeldung (Ihr Wiederherstellungspunkt) und die Session-ID
3. **Zur Rückspulung auffordern**: Nachdem der Agent fertig ist, überprüfen Sie Ihre Utility-Datei, um die Dokumentationskommentare zu sehen, und entscheiden Sie dann, ob Sie die Änderungen rückgängig machen möchten
4. **Sitzung fortsetzen und zurückspulen**: Wenn ja, setzen Sie die Sitzung mit einer leeren Eingabeaufforderung fort und rufen Sie `rewind_files()` auf, um die ursprüngliche Datei wiederherzustellen

</Step>

<Step title="Beispiel ausführen">

Setzen Sie die Umgebungsvariable und führen Sie das Skript aus dem selben Verzeichnis wie Ihre Utility-Datei aus.

<Tip>
Öffnen Sie Ihre Utility-Datei (`utils.py` oder `utils.ts`) in Ihrer IDE oder Ihrem Editor, bevor Sie das Skript ausführen. Sie sehen, wie die Datei in Echtzeit aktualisiert wird, während der Agent Dokumentationskommentare hinzufügt, und dann zum Original zurückkehrt, wenn Sie sich zum Zurückspulen entscheiden.
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

Sie sehen, dass der Agent Dokumentationskommentare hinzufügt, und dann erhalten Sie eine Eingabeaufforderung, ob Sie zurückspulen möchten. Wenn Sie ja wählen, wird die Datei in ihren ursprünglichen Zustand zurückgesetzt.

</Step>

</Steps>

## Einschränkungen

File Checkpointing hat die folgenden Einschränkungen:

| Einschränkung | Beschreibung |
|------------|-------------|
| Nur Write/Edit/NotebookEdit-Tools | Änderungen, die durch Bash-Befehle vorgenommen werden, werden nicht verfolgt |
| Gleiche Sitzung | Checkpoints sind an die Sitzung gebunden, die sie erstellt hat |
| Nur Dateiinhalt | Das Erstellen, Verschieben oder Löschen von Verzeichnissen wird durch Zurückspulen nicht rückgängig gemacht |
| Lokale Dateien | Remote- oder Netzwerkdateien werden nicht verfolgt |

## Fehlerbehebung

### Checkpointing-Optionen nicht erkannt

Wenn `enableFileCheckpointing` oder `rewindFiles()` nicht verfügbar ist, verwenden Sie möglicherweise eine ältere SDK-Version.

**Lösung**: Aktualisieren Sie auf die neueste SDK-Version:
- **Python**: `pip install --upgrade claude-agent-sdk`
- **TypeScript**: `npm install @anthropic-ai/claude-agent-sdk@latest`

### Benutzermeldungen haben keine UUIDs

Wenn `message.uuid` `undefined` oder fehlend ist, empfangen Sie keine Checkpoint-UUIDs.

**Ursache**: Die `replay-user-messages`-Option ist nicht gesetzt.

**Lösung**: Fügen Sie `extra_args={"replay-user-messages": None}` (Python) oder `extraArgs: { 'replay-user-messages': null }` (TypeScript) zu Ihren Optionen hinzu.

### Fehler „No file checkpoint found for message"

Dieser Fehler tritt auf, wenn die Checkpoint-Daten für die angegebene Benutzer-Nachrichten-UUID nicht vorhanden sind.

**Häufige Ursachen**:
- Die Umgebungsvariable `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` ist nicht gesetzt
- Die Sitzung wurde nicht ordnungsgemäß abgeschlossen, bevor versucht wurde, sie fortzusetzen und zurückzuspulen

**Lösung**: Stellen Sie sicher, dass Sie die Umgebungsvariable gesetzt haben (siehe [Umgebungsvariable setzen](#set-the-environment-variable)), und verwenden Sie dann das in den Beispielen gezeigte Muster: Erfassen Sie die UUID der ersten Benutzermeldung, schließen Sie die Sitzung vollständig ab, setzen Sie sie dann mit einer leeren Eingabeaufforderung fort und rufen Sie `rewindFiles()` einmal auf.

### Fehler „ProcessTransport is not ready for writing"

Dieser Fehler tritt auf, wenn Sie `rewindFiles()` oder `rewind_files()` aufrufen, nachdem Sie die Antwort vollständig durchlaufen haben. Die Verbindung zum CLI-Prozess wird geschlossen, wenn die Schleife abgeschlossen ist.

**Lösung**: Setzen Sie die Sitzung mit einer leeren Eingabeaufforderung fort und rufen Sie dann Rewind auf der neuen Abfrage auf:

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

## Nächste Schritte

- **[Sitzungen](/docs/de/agent-sdk/sessions)**: Erfahren Sie, wie Sie Sitzungen fortsetzen, was erforderlich ist, um nach Abschluss des Streams zurückzuspulen. Behandelt Session-IDs, Fortsetzen von Gesprächen und Session-Forking.
- **[Berechtigungen](/docs/de/agent-sdk/permissions)**: Konfigurieren Sie, welche Tools Claude verwenden kann und wie Dateiänderungen genehmigt werden. Nützlich, wenn Sie mehr Kontrolle darüber haben möchten, wann Änderungen vorgenommen werden.
- **[TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript)**: Vollständige API-Referenz einschließlich aller Optionen für `query()` und die `rewindFiles()`-Methode.
- **[Python SDK-Referenz](/docs/de/agent-sdk/python)**: Vollständige API-Referenz einschließlich aller Optionen für `ClaudeAgentOptions` und die `rewind_files()`-Methode.