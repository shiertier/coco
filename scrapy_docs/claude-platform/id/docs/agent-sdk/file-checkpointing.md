# Membatalkan perubahan file dengan checkpointing

Lacak perubahan file selama sesi agen dan pulihkan file ke status sebelumnya

---

File checkpointing melacak modifikasi file yang dilakukan melalui alat Write, Edit, dan NotebookEdit selama sesi agen, memungkinkan Anda untuk membatalkan file ke status sebelumnya. Ingin mencobanya? Lompat ke [contoh interaktif](#try-it-out).

Dengan checkpointing, Anda dapat:

- **Membatalkan perubahan yang tidak diinginkan** dengan memulihkan file ke status yang diketahui baik
- **Menjelajahi alternatif** dengan memulihkan ke checkpoint dan mencoba pendekatan berbeda
- **Pulih dari kesalahan** ketika agen membuat modifikasi yang salah

<Warning>
Hanya perubahan yang dilakukan melalui alat Write, Edit, dan NotebookEdit yang dilacak. Perubahan yang dilakukan melalui perintah Bash (seperti `echo > file.txt` atau `sed -i`) tidak ditangkap oleh sistem checkpoint.
</Warning>

## Cara kerja checkpointing

Ketika Anda mengaktifkan file checkpointing, SDK membuat cadangan file sebelum memodifikasinya melalui alat Write, Edit, atau NotebookEdit. Pesan pengguna dalam aliran respons mencakup UUID checkpoint yang dapat Anda gunakan sebagai titik pemulihan.

Checkpoint bekerja dengan alat bawaan ini yang digunakan agen untuk memodifikasi file:

| Alat | Deskripsi |
|------|-----------|
| Write | Membuat file baru atau menimpa file yang ada dengan konten baru |
| Edit | Membuat pengeditan yang ditargetkan ke bagian tertentu dari file yang ada |
| NotebookEdit | Memodifikasi sel dalam notebook Jupyter (file `.ipynb`) |

<Note>
Pembatalan file memulihkan file di disk ke status sebelumnya. Ini tidak membatalkan percakapan itu sendiri. Riwayat percakapan dan konteks tetap utuh setelah memanggil `rewindFiles()` (TypeScript) atau `rewind_files()` (Python).
</Note>

Sistem checkpoint melacak:

- File yang dibuat selama sesi
- File yang dimodifikasi selama sesi
- Konten asli file yang dimodifikasi

Ketika Anda membatalkan ke checkpoint, file yang dibuat dihapus dan file yang dimodifikasi dipulihkan ke konten mereka pada titik itu.

## Implementasikan checkpointing

Untuk menggunakan file checkpointing, aktifkan dalam opsi Anda, tangkap UUID checkpoint dari aliran respons, kemudian panggil `rewindFiles()` (TypeScript) atau `rewind_files()` (Python) ketika Anda perlu memulihkan.

Contoh berikut menunjukkan alur lengkap: aktifkan checkpointing, tangkap UUID checkpoint dan ID sesi dari aliran respons, kemudian lanjutkan sesi nanti untuk membatalkan file. Setiap langkah dijelaskan secara detail di bawah.

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

<Step title="Atur variabel lingkungan">

File checkpointing memerlukan variabel lingkungan `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING`. Anda dapat mengaturnya melalui baris perintah sebelum menjalankan skrip Anda, atau langsung dalam opsi SDK.

**Opsi 1: Atur melalui baris perintah**

```bash Python
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
```

**Opsi 2: Atur dalam opsi SDK**

Lewatkan variabel lingkungan melalui opsi `env` saat mengonfigurasi SDK:

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

<Step title="Aktifkan checkpointing">

Konfigurasikan opsi SDK Anda untuk mengaktifkan checkpointing dan menerima UUID checkpoint:

| Opsi | Python | TypeScript | Deskripsi |
|--------|--------|------------|-----------|
| Aktifkan checkpointing | `enable_file_checkpointing=True` | `enableFileCheckpointing: true` | Melacak perubahan file untuk pembatalan |
| Terima UUID checkpoint | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | Diperlukan untuk mendapatkan UUID pesan pengguna dalam aliran |

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

<Step title="Tangkap UUID checkpoint dan ID sesi">

Dengan opsi `replay-user-messages` yang diatur (ditunjukkan di atas), setiap pesan pengguna dalam aliran respons memiliki UUID yang berfungsi sebagai checkpoint.

Untuk sebagian besar kasus penggunaan, tangkap UUID pesan pengguna pertama (`message.uuid`); pembatalan ke sana memulihkan semua file ke status asli mereka. Untuk menyimpan beberapa checkpoint dan membatalkan ke status perantara, lihat [Beberapa titik pemulihan](#multiple-restore-points).

Menangkap ID sesi (`message.session_id`) bersifat opsional; Anda hanya membutuhkannya jika Anda ingin membatalkan nanti, setelah aliran selesai. Jika Anda memanggil `rewindFiles()` segera saat masih memproses pesan (seperti yang dilakukan contoh di [Checkpoint sebelum operasi berisiko](#checkpoint-before-risky-operations)), Anda dapat melewatkan penangkapan ID sesi.

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

<Step title="Batalkan file">

Untuk membatalkan setelah aliran selesai, lanjutkan sesi dengan prompt kosong dan panggil `rewind_files()` (Python) atau `rewindFiles()` (TypeScript) dengan UUID checkpoint Anda. Anda juga dapat membatalkan selama aliran; lihat [Checkpoint sebelum operasi berisiko](#checkpoint-before-risky-operations) untuk pola itu.

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

Jika Anda menangkap ID sesi dan ID checkpoint, Anda juga dapat membatalkan dari CLI:

```bash
claude --resume <session-id> --rewind-files <checkpoint-uuid>
```

</Step>

</Steps>

## Pola umum

Pola ini menunjukkan cara berbeda untuk menangkap dan menggunakan UUID checkpoint tergantung pada kasus penggunaan Anda.

### Checkpoint sebelum operasi berisiko

Pola ini menyimpan hanya UUID checkpoint paling baru, memperbarui sebelum setiap putaran agen. Jika ada yang salah selama pemrosesan, Anda dapat segera membatalkan ke status terakhir yang aman dan keluar dari loop.

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

### Beberapa titik pemulihan

Jika Claude membuat perubahan di beberapa putaran, Anda mungkin ingin membatalkan ke titik tertentu daripada semuanya. Misalnya, jika Claude merefaktor file di putaran satu dan menambahkan tes di putaran dua, Anda mungkin ingin menyimpan refaktor tetapi membatalkan tes.

Pola ini menyimpan semua UUID checkpoint dalam array dengan metadata. Setelah sesi selesai, Anda dapat membatalkan ke checkpoint sebelumnya:

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

## Coba sekarang

Contoh lengkap ini membuat file utilitas kecil, membuat agen menambahkan komentar dokumentasi, menunjukkan perubahan kepada Anda, kemudian menanyakan apakah Anda ingin membatalkan.

Sebelum Anda mulai, pastikan Anda telah [menginstal Claude Agent SDK](/docs/id/agent-sdk/quickstart).

<Steps>

<Step title="Buat file uji">

Buat file baru bernama `utils.py` (Python) atau `utils.ts` (TypeScript) dan tempel kode berikut:

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

<Step title="Jalankan contoh interaktif">

Buat file baru bernama `try_checkpointing.py` (Python) atau `try_checkpointing.ts` (TypeScript) di direktori yang sama dengan file utilitas Anda, dan tempel kode berikut.

Skrip ini meminta Claude untuk menambahkan komentar doc ke file utilitas Anda, kemudian memberi Anda opsi untuk membatalkan dan memulihkan yang asli.

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

Contoh ini mendemonstrasikan alur kerja checkpointing lengkap:

1. **Aktifkan checkpointing**: konfigurasikan SDK dengan `enable_file_checkpointing=True` dan `permission_mode="acceptEdits"` untuk menyetujui pengeditan file secara otomatis
2. **Tangkap data checkpoint**: saat agen berjalan, simpan UUID pesan pengguna pertama (titik pemulihan Anda) dan ID sesi
3. **Minta pembatalan**: setelah agen selesai, periksa file utilitas Anda untuk melihat komentar doc, kemudian putuskan apakah Anda ingin membatalkan perubahan
4. **Lanjutkan dan batalkan**: jika ya, lanjutkan sesi dengan prompt kosong dan panggil `rewind_files()` untuk memulihkan file asli

</Step>

<Step title="Jalankan contoh">

Atur variabel lingkungan dan jalankan skrip dari direktori yang sama dengan file utilitas Anda.

<Tip>
Buka file utilitas Anda (`utils.py` atau `utils.ts`) di IDE atau editor Anda sebelum menjalankan skrip. Anda akan melihat file diperbarui secara real-time saat agen menambahkan komentar doc, kemudian kembali ke asli ketika Anda memilih untuk membatalkan.
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

Anda akan melihat agen menambahkan komentar doc, kemudian prompt menanyakan apakah Anda ingin membatalkan. Jika Anda memilih ya, file dipulihkan ke status aslinya.

</Step>

</Steps>

## Keterbatasan

File checkpointing memiliki keterbatasan berikut:

| Keterbatasan | Deskripsi |
|------------|-----------|
| Alat Write/Edit/NotebookEdit saja | Perubahan yang dilakukan melalui perintah Bash tidak dilacak |
| Sesi yang sama | Checkpoint terikat pada sesi yang membuatnya |
| Konten file saja | Membuat, memindahkan, atau menghapus direktori tidak dibatalkan oleh pembatalan |
| File lokal | File jarak jauh atau jaringan tidak dilacak |

## Pemecahan masalah

### Opsi checkpointing tidak dikenali

Jika `enableFileCheckpointing` atau `rewindFiles()` tidak tersedia, Anda mungkin menggunakan versi SDK yang lebih lama.

**Solusi**: Perbarui ke versi SDK terbaru:
- **Python**: `pip install --upgrade claude-agent-sdk`
- **TypeScript**: `npm install @anthropic-ai/claude-agent-sdk@latest`

### Pesan pengguna tidak memiliki UUID

Jika `message.uuid` adalah `undefined` atau hilang, Anda tidak menerima UUID checkpoint.

**Penyebab**: Opsi `replay-user-messages` tidak diatur.

**Solusi**: Tambahkan `extra_args={"replay-user-messages": None}` (Python) atau `extraArgs: { 'replay-user-messages': null }` (TypeScript) ke opsi Anda.

### Kesalahan "No file checkpoint found for message"

Kesalahan ini terjadi ketika data checkpoint tidak ada untuk UUID pesan pengguna yang ditentukan.

**Penyebab umum**:
- Variabel lingkungan `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` tidak diatur
- Sesi tidak diselesaikan dengan benar sebelum mencoba melanjutkan dan membatalkan

**Solusi**: Pastikan Anda telah mengatur variabel lingkungan (lihat [Atur variabel lingkungan](#set-the-environment-variable)), kemudian gunakan pola yang ditunjukkan dalam contoh: tangkap UUID pesan pengguna pertama, selesaikan sesi sepenuhnya, kemudian lanjutkan dengan prompt kosong dan panggil `rewindFiles()` sekali.

### Kesalahan "ProcessTransport is not ready for writing"

Kesalahan ini terjadi ketika Anda memanggil `rewindFiles()` atau `rewind_files()` setelah Anda selesai melakukan iterasi melalui respons. Koneksi ke proses CLI ditutup ketika loop selesai.

**Solusi**: Lanjutkan sesi dengan prompt kosong, kemudian batalkan pada kueri baru:

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

## Langkah berikutnya

- **[Sesi](/docs/id/agent-sdk/sessions)**: pelajari cara melanjutkan sesi, yang diperlukan untuk pembatalan setelah aliran selesai. Mencakup ID sesi, melanjutkan percakapan, dan forking sesi.
- **[Izin](/docs/id/agent-sdk/permissions)**: konfigurasikan alat mana yang dapat digunakan Claude dan bagaimana modifikasi file disetujui. Berguna jika Anda menginginkan kontrol lebih besar atas kapan pengeditan terjadi.
- **[Referensi SDK TypeScript](/docs/id/agent-sdk/typescript)**: referensi API lengkap termasuk semua opsi untuk `query()` dan metode `rewindFiles()`.
- **[Referensi SDK Python](/docs/id/agent-sdk/python)**: referensi API lengkap termasuk semua opsi untuk `ClaudeAgentOptions` dan metode `rewind_files()`.