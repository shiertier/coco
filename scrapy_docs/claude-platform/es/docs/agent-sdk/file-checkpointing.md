# Revertir cambios de archivos con puntos de control

Rastrear cambios de archivos durante sesiones de agente y restaurar archivos a cualquier estado anterior

---

El punto de control de archivos rastrea las modificaciones de archivos realizadas a través de las herramientas Write, Edit y NotebookEdit durante una sesión de agente, permitiéndote revertir archivos a cualquier estado anterior. ¿Quieres probarlo? Salta al [ejemplo interactivo](#try-it-out).

Con los puntos de control, puedes:

- **Deshacer cambios no deseados** restaurando archivos a un estado conocido como bueno
- **Explorar alternativas** restaurando a un punto de control e intentando un enfoque diferente
- **Recuperarte de errores** cuando el agente realiza modificaciones incorrectas

<Warning>
Solo se rastrean los cambios realizados a través de las herramientas Write, Edit y NotebookEdit. Los cambios realizados a través de comandos Bash (como `echo > file.txt` o `sed -i`) no son capturados por el sistema de puntos de control.
</Warning>

## Cómo funcionan los puntos de control

Cuando habilitas el punto de control de archivos, el SDK crea copias de seguridad de archivos antes de modificarlos a través de las herramientas Write, Edit o NotebookEdit. Los mensajes del usuario en el flujo de respuesta incluyen un UUID de punto de control que puedes usar como punto de restauración.

El punto de control funciona con estas herramientas integradas que el agente usa para modificar archivos:

| Herramienta | Descripción |
|------|-------------|
| Write | Crea un nuevo archivo o sobrescribe un archivo existente con contenido nuevo |
| Edit | Realiza ediciones dirigidas a partes específicas de un archivo existente |
| NotebookEdit | Modifica celdas en cuadernos Jupyter (archivos `.ipynb`) |

<Note>
La reversión de archivos restaura los archivos en disco a un estado anterior. No revierte la conversación en sí. El historial de conversación y el contexto permanecen intactos después de llamar a `rewindFiles()` (TypeScript) o `rewind_files()` (Python).
</Note>

El sistema de puntos de control rastrea:

- Archivos creados durante la sesión
- Archivos modificados durante la sesión
- El contenido original de los archivos modificados

Cuando reviertes a un punto de control, los archivos creados se eliminan y los archivos modificados se restauran a su contenido en ese momento.

## Implementar puntos de control

Para usar el punto de control de archivos, habilítalo en tus opciones, captura los UUID de puntos de control del flujo de respuesta, luego llama a `rewindFiles()` (TypeScript) o `rewind_files()` (Python) cuando necesites restaurar.

El siguiente ejemplo muestra el flujo completo: habilita los puntos de control, captura el UUID del punto de control y el ID de sesión del flujo de respuesta, luego reanuda la sesión más tarde para revertir archivos. Cada paso se explica en detalle a continuación.

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

<Step title="Establecer la variable de entorno">

El punto de control de archivos requiere la variable de entorno `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING`. Puedes establecerla a través de la línea de comandos antes de ejecutar tu script, o directamente en las opciones del SDK.

**Opción 1: Establecer a través de la línea de comandos**

```bash Python
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
```

**Opción 2: Establecer en las opciones del SDK**

Pasa la variable de entorno a través de la opción `env` al configurar el SDK:

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

<Step title="Habilitar puntos de control">

Configura tus opciones del SDK para habilitar los puntos de control y recibir UUID de puntos de control:

| Opción | Python | TypeScript | Descripción |
|--------|--------|------------|-------------|
| Habilitar puntos de control | `enable_file_checkpointing=True` | `enableFileCheckpointing: true` | Rastrea cambios de archivos para reversión |
| Recibir UUID de puntos de control | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | Requerido para obtener UUID de mensajes de usuario en el flujo |

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

<Step title="Capturar UUID de punto de control e ID de sesión">

Con la opción `replay-user-messages` establecida (mostrada arriba), cada mensaje de usuario en el flujo de respuesta tiene un UUID que sirve como punto de control.

Para la mayoría de los casos de uso, captura el UUID del primer mensaje de usuario (`message.uuid`); revertir a él restaura todos los archivos a su estado original. Para almacenar múltiples puntos de control y revertir a estados intermedios, consulta [Múltiples puntos de restauración](#multiple-restore-points).

Capturar el ID de sesión (`message.session_id`) es opcional; solo lo necesitas si deseas revertir más tarde, después de que se complete el flujo. Si estás llamando a `rewindFiles()` inmediatamente mientras aún procesas mensajes (como lo hace el ejemplo en [Punto de control antes de operaciones arriesgadas](#checkpoint-before-risky-operations)), puedes omitir la captura del ID de sesión.

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

<Step title="Revertir archivos">

Para revertir después de que se complete el flujo, reanuda la sesión con un mensaje vacío y llama a `rewind_files()` (Python) o `rewindFiles()` (TypeScript) con tu UUID de punto de control. También puedes revertir durante el flujo; consulta [Punto de control antes de operaciones arriesgadas](#checkpoint-before-risky-operations) para ese patrón.

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

Si capturaste el ID de sesión y el ID de punto de control, también puedes revertir desde la CLI:

```bash
claude --resume <session-id> --rewind-files <checkpoint-uuid>
```

</Step>

</Steps>

## Patrones comunes

Estos patrones muestran diferentes formas de capturar y usar UUID de puntos de control dependiendo de tu caso de uso.

### Punto de control antes de operaciones arriesgadas

Este patrón mantiene solo el UUID de punto de control más reciente, actualizándolo antes de cada turno del agente. Si algo sale mal durante el procesamiento, puedes revertir inmediatamente al último estado seguro y salir del bucle.

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

### Múltiples puntos de restauración

Si Claude realiza cambios en múltiples turnos, es posible que desees revertir a un punto específico en lugar de todo el camino de regreso. Por ejemplo, si Claude refactoriza un archivo en el turno uno y agrega pruebas en el turno dos, es posible que desees mantener la refactorización pero deshacer las pruebas.

Este patrón almacena todos los UUID de puntos de control en una matriz con metadatos. Después de que se complete la sesión, puedes revertir a cualquier punto de control anterior:

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

## Pruébalo

Este ejemplo completo crea un pequeño archivo de utilidad, hace que el agente agregue comentarios de documentación, te muestra los cambios, luego pregunta si deseas revertir.

Antes de comenzar, asegúrate de tener el [Claude Agent SDK instalado](/docs/es/agent-sdk/quickstart).

<Steps>

<Step title="Crear un archivo de prueba">

Crea un nuevo archivo llamado `utils.py` (Python) o `utils.ts` (TypeScript) y pega el siguiente código:

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

<Step title="Ejecutar el ejemplo interactivo">

Crea un nuevo archivo llamado `try_checkpointing.py` (Python) o `try_checkpointing.ts` (TypeScript) en el mismo directorio que tu archivo de utilidad, y pega el siguiente código.

Este script le pide a Claude que agregue comentarios de documentación a tu archivo de utilidad, luego te da la opción de revertir y restaurar el original.

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

Este ejemplo demuestra el flujo de trabajo completo de puntos de control:

1. **Habilitar puntos de control**: configura el SDK con `enable_file_checkpointing=True` y `permission_mode="acceptEdits"` para aprobar automáticamente ediciones de archivos
2. **Capturar datos de punto de control**: mientras se ejecuta el agente, almacena el UUID del primer mensaje de usuario (tu punto de restauración) y el ID de sesión
3. **Solicitar reversión**: después de que el agente termine, verifica tu archivo de utilidad para ver los comentarios de documentación, luego decide si deseas deshacer los cambios
4. **Reanudar y revertir**: si es sí, reanuda la sesión con un mensaje vacío y llama a `rewind_files()` para restaurar el archivo original

</Step>

<Step title="Ejecutar el ejemplo">

Establece la variable de entorno y ejecuta el script desde el mismo directorio que tu archivo de utilidad.

<Tip>
Abre tu archivo de utilidad (`utils.py` o `utils.ts`) en tu IDE o editor antes de ejecutar el script. Verás que el archivo se actualiza en tiempo real mientras el agente agrega comentarios de documentación, luego se revierte al original cuando elijas revertir.
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

Verás que el agente agrega comentarios de documentación, luego un mensaje preguntando si deseas revertir. Si eliges sí, el archivo se restaura a su estado original.

</Step>

</Steps>

## Limitaciones

El punto de control de archivos tiene las siguientes limitaciones:

| Limitación | Descripción |
|------------|-------------|
| Solo herramientas Write/Edit/NotebookEdit | Los cambios realizados a través de comandos Bash no se rastrean |
| Misma sesión | Los puntos de control están vinculados a la sesión que los creó |
| Solo contenido de archivo | La creación, movimiento o eliminación de directorios no se deshace al revertir |
| Archivos locales | Los archivos remotos o de red no se rastrean |

## Solución de problemas

### Las opciones de puntos de control no se reconocen

Si `enableFileCheckpointing` o `rewindFiles()` no están disponibles, es posible que estés en una versión anterior del SDK.

**Solución**: Actualiza a la última versión del SDK:
- **Python**: `pip install --upgrade claude-agent-sdk`
- **TypeScript**: `npm install @anthropic-ai/claude-agent-sdk@latest`

### Los mensajes de usuario no tienen UUID

Si `message.uuid` es `undefined` o está faltando, no estás recibiendo UUID de puntos de control.

**Causa**: La opción `replay-user-messages` no está establecida.

**Solución**: Agrega `extra_args={"replay-user-messages": None}` (Python) o `extraArgs: { 'replay-user-messages': null }` (TypeScript) a tus opciones.

### Error "No file checkpoint found for message"

Este error ocurre cuando los datos del punto de control no existen para el UUID de mensaje de usuario especificado.

**Causas comunes**:
- La variable de entorno `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` no está establecida
- La sesión no se completó correctamente antes de intentar reanudar y revertir

**Solución**: Asegúrate de haber establecido la variable de entorno (consulta [Establecer la variable de entorno](#set-the-environment-variable)), luego usa el patrón mostrado en los ejemplos: captura el UUID del primer mensaje de usuario, completa la sesión completamente, luego reanuda con un mensaje vacío y llama a `rewindFiles()` una vez.

### Error "ProcessTransport is not ready for writing"

Este error ocurre cuando llamas a `rewindFiles()` o `rewind_files()` después de haber terminado de iterar a través de la respuesta. La conexión al proceso de CLI se cierra cuando se completa el bucle.

**Solución**: Reanuda la sesión con un mensaje vacío, luego revertir en la nueva consulta:

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

## Próximos pasos

- **[Sesiones](/docs/es/agent-sdk/sessions)**: aprende cómo reanudar sesiones, que es necesario para revertir después de que se complete el flujo. Cubre ID de sesión, reanudación de conversaciones y bifurcación de sesiones.
- **[Permisos](/docs/es/agent-sdk/permissions)**: configura qué herramientas puede usar Claude y cómo se aprueban las modificaciones de archivos. Útil si deseas más control sobre cuándo ocurren las ediciones.
- **[Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript)**: referencia completa de la API incluyendo todas las opciones para `query()` y el método `rewindFiles()`.
- **[Referencia del SDK de Python](/docs/es/agent-sdk/python)**: referencia completa de la API incluyendo todas las opciones para `ClaudeAgentOptions` y el método `rewind_files()`.