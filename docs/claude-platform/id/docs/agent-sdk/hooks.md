# Intercept dan kontrol perilaku agen dengan hooks

Intercept dan sesuaikan perilaku agen pada titik eksekusi kunci dengan hooks

---

Hooks memungkinkan Anda untuk menginterceptor eksekusi agen pada titik-titik kunci untuk menambahkan validasi, logging, kontrol keamanan, atau logika kustom. Dengan hooks, Anda dapat:

- **Memblokir operasi berbahaya** sebelum dieksekusi, seperti perintah shell yang merusak atau akses file yang tidak sah
- **Log dan audit** setiap pemanggilan tool untuk kepatuhan, debugging, atau analitik
- **Transformasi input dan output** untuk membersihkan data, menyuntikkan kredensial, atau mengalihkan jalur file
- **Memerlukan persetujuan manusia** untuk tindakan sensitif seperti penulisan database atau panggilan API
- **Lacak siklus hidup sesi** untuk mengelola status, membersihkan sumber daya, atau mengirim notifikasi

Sebuah hook memiliki dua bagian:

1. **Fungsi callback**: logika yang berjalan ketika hook dipicu
2. **Konfigurasi hook**: memberitahu SDK acara mana yang akan di-hook (seperti `PreToolUse`) dan tool mana yang cocok

Contoh berikut memblokir agen dari memodifikasi file `.env`. Pertama, tentukan callback yang memeriksa jalur file, kemudian teruskan ke `query()` untuk dijalankan sebelum panggilan tool Write atau Edit apa pun:

<CodeGroup>

```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher

# Define a hook callback that receives tool call details
async def protect_env_files(input_data, tool_use_id, context):
    # Extract the file path from the tool's input arguments
    file_path = input_data['tool_input'].get('file_path', '')
    file_name = file_path.split('/')[-1]

    # Block the operation if targeting a .env file
    if file_name == '.env':
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'deny',
                'permissionDecisionReason': 'Cannot modify .env files'
            }
        }

    # Return empty object to allow the operation
    return {}

async def main():
    async for message in query(
        prompt="Update the database configuration",
        options=ClaudeAgentOptions(
            hooks={
                # Register the hook for PreToolUse events
                # The matcher filters to only Write and Edit tool calls
                'PreToolUse': [HookMatcher(matcher='Write|Edit', hooks=[protect_env_files])]
            }
        )
    ):
        print(message)

asyncio.run(main())
```

```typescript TypeScript
import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

// Define a hook callback with the HookCallback type
const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
  // Cast input to the specific hook type for type safety
  const preInput = input as PreToolUseHookInput;

  // Extract the file path from the tool's input arguments
  const filePath = preInput.tool_input?.file_path as string;
  const fileName = filePath?.split('/').pop();

  // Block the operation if targeting a .env file
  if (fileName === '.env') {
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'deny',
        permissionDecisionReason: 'Cannot modify .env files'
      }
    };
  }

  // Return empty object to allow the operation
  return {};
};

for await (const message of query({
  prompt: "Update the database configuration",
  options: {
    hooks: {
      // Register the hook for PreToolUse events
      // The matcher filters to only Write and Edit tool calls
      PreToolUse: [{ matcher: 'Write|Edit', hooks: [protectEnvFiles] }]
    }
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Ini adalah hook `PreToolUse`. Hook ini berjalan sebelum tool dieksekusi dan dapat memblokir atau mengizinkan operasi berdasarkan logika Anda. Sisa panduan ini mencakup semua hook yang tersedia, opsi konfigurasi mereka, dan pola untuk kasus penggunaan umum.

## Hook yang tersedia

SDK menyediakan hooks untuk tahap berbeda dari eksekusi agen. Beberapa hooks tersedia di kedua SDK, sementara yang lain hanya TypeScript karena Python SDK tidak mendukungnya.

| Hook Event | Python SDK | TypeScript SDK | Apa yang memicunya | Contoh kasus penggunaan |
|------------|------------|----------------|------------------|------------------------|
| `PreToolUse` | Ya | Ya | Permintaan pemanggilan tool (dapat memblokir atau memodifikasi) | Blokir perintah shell berbahaya |
| `PostToolUse` | Ya | Ya | Hasil eksekusi tool | Log semua perubahan file ke jejak audit |
| `PostToolUseFailure` | Tidak | Ya | Kegagalan eksekusi tool | Tangani atau log kesalahan tool |
| `UserPromptSubmit` | Ya | Ya | Pengajuan prompt pengguna | Suntikkan konteks tambahan ke dalam prompt |
| `Stop` | Ya | Ya | Penghentian eksekusi agen | Simpan status sesi sebelum keluar |
| `SubagentStart` | Tidak | Ya | Inisialisasi subagen | Lacak pemijahan tugas paralel |
| `SubagentStop` | Ya | Ya | Penyelesaian subagen | Agregasi hasil dari tugas paralel |
| `PreCompact` | Ya | Ya | Permintaan pemadatan percakapan | Arsipkan transkrip lengkap sebelum merangkum |
| `PermissionRequest` | Tidak | Ya | Dialog izin akan ditampilkan | Penanganan izin kustom |
| `SessionStart` | Tidak | Ya | Inisialisasi sesi | Inisialisasi logging dan telemetri |
| `SessionEnd` | Tidak | Ya | Penghentian sesi | Bersihkan sumber daya sementara |
| `Notification` | Tidak | Ya | Pesan status agen | Kirim pembaruan status agen ke Slack atau PagerDuty |

## Kasus penggunaan umum

Hooks cukup fleksibel untuk menangani banyak skenario berbeda. Berikut adalah beberapa pola paling umum yang diorganisir berdasarkan kategori.

<Tabs>
  <Tab title="Keamanan">
    - Blokir perintah berbahaya (seperti `rm -rf /`, SQL yang merusak)
    - Validasi jalur file sebelum operasi penulisan
    - Terapkan allowlist/blocklist untuk penggunaan tool
  </Tab>
  <Tab title="Logging">
    - Buat jejak audit dari semua tindakan agen
    - Lacak metrik eksekusi dan kinerja
    - Debug perilaku agen dalam pengembangan
  </Tab>
  <Tab title="Interception tool">
    - Alihkan operasi file ke direktori sandbox
    - Suntikkan variabel lingkungan atau kredensial
    - Transformasi input atau output tool
  </Tab>
  <Tab title="Otorisasi">
    - Implementasikan kontrol akses berbasis peran
    - Memerlukan persetujuan manusia untuk operasi sensitif
    - Batasi tingkat penggunaan tool tertentu
  </Tab>
</Tabs>

## Konfigurasi hooks

Untuk mengonfigurasi hook untuk agen Anda, teruskan hook dalam parameter `options.hooks` saat memanggil `query()`:

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

Opsi `hooks` adalah kamus (Python) atau objek (TypeScript) di mana:
- **Kunci** adalah [nama acara hook](#available-hooks) (misalnya, `'PreToolUse'`, `'PostToolUse'`, `'Stop'`)
- **Nilai** adalah array dari [matcher](#matchers), masing-masing berisi pola filter opsional dan [fungsi callback](#callback-function-inputs) Anda

Fungsi callback hook Anda menerima [data input](#input-data) tentang acara dan mengembalikan [respons](#callback-outputs) sehingga agen tahu untuk mengizinkan, memblokir, atau memodifikasi operasi.

### Matchers

Gunakan matcher untuk memfilter tool mana yang memicu callback Anda:

| Opsi | Tipe | Default | Deskripsi |
|--------|------|---------|-------------|
| `matcher` | `string` | `undefined` | Pola regex untuk mencocokkan nama tool. Tool bawaan termasuk `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Task`, dan lainnya. Tool MCP menggunakan pola `mcp__<server>__<action>`. |
| `hooks` | `HookCallback[]` | - | Diperlukan. Array fungsi callback untuk dieksekusi ketika pola cocok |
| `timeout` | `number` | `60` | Timeout dalam detik; tingkatkan untuk hooks yang melakukan panggilan API eksternal |

Gunakan pola `matcher` untuk menargetkan tool tertentu kapan pun memungkinkan. Matcher dengan `'Bash'` hanya berjalan untuk perintah Bash, sementara menghilangkan pola menjalankan callback Anda untuk setiap pemanggilan tool. Perhatikan bahwa matcher hanya memfilter berdasarkan **nama tool**, bukan jalur file atau argumen lainnya—untuk memfilter berdasarkan jalur file, periksa `tool_input.file_path` di dalam callback Anda.

Matcher hanya berlaku untuk hook berbasis tool (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`). Untuk hook siklus hidup seperti `Stop`, `SessionStart`, dan `Notification`, matcher diabaikan dan hook dipicu untuk semua acara dari tipe itu.

<Tip>
**Menemukan nama tool:** Periksa array `tools` dalam pesan sistem awal ketika sesi Anda dimulai, atau tambahkan hook tanpa matcher untuk mencatat semua pemanggilan tool.

**Penamaan tool MCP:** Tool MCP selalu dimulai dengan `mcp__` diikuti oleh nama server dan tindakan: `mcp__<server>__<action>`. Misalnya, jika Anda mengonfigurasi server bernama `playwright`, toolnya akan dinamai `mcp__playwright__browser_screenshot`, `mcp__playwright__browser_click`, dll. Nama server berasal dari kunci yang Anda gunakan dalam konfigurasi `mcpServers`.
</Tip>

Contoh ini menggunakan matcher untuk menjalankan hook hanya untuk tool yang memodifikasi file ketika acara `PreToolUse` dipicu:

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

### Input fungsi callback

Setiap callback hook menerima tiga argumen:

1. **Data input** (`dict` / `HookInput`): Detail acara. Lihat [data input](#input-data) untuk bidang
2. **ID penggunaan tool** (`str | None` / `string | null`): Korelasikan acara `PreToolUse` dan `PostToolUse`
3. **Konteks** (`HookContext`): Di TypeScript, berisi properti `signal` (`AbortSignal`) untuk pembatalan. Teruskan ini ke operasi async seperti `fetch()` sehingga mereka secara otomatis membatalkan jika hook kedaluwarsa. Di Python, argumen ini dicadangkan untuk penggunaan di masa depan.

### Data input

Argumen pertama untuk callback hook Anda berisi informasi tentang acara. Nama bidang identik di seluruh SDK (keduanya menggunakan snake_case).

**Bidang umum** yang ada di semua tipe hook:

| Bidang | Tipe | Deskripsi |
|-------|------|-------------|
| `hook_event_name` | `string` | Tipe hook (`PreToolUse`, `PostToolUse`, dll.) |
| `session_id` | `string` | Pengidentifikasi sesi saat ini |
| `transcript_path` | `string` | Jalur ke transkrip percakapan |
| `cwd` | `string` | Direktori kerja saat ini |

**Bidang spesifik hook** bervariasi menurut tipe hook. Item yang ditandai <sup>TS</sup> hanya tersedia di TypeScript SDK:

| Bidang | Tipe | Deskripsi | Hooks |
|-------|------|-------------|-------|
| `tool_name` | `string` | Nama tool yang dipanggil | PreToolUse, PostToolUse, PostToolUseFailure<sup>TS</sup>, PermissionRequest<sup>TS</sup> |
| `tool_input` | `object` | Argumen yang dilewatkan ke tool | PreToolUse, PostToolUse, PostToolUseFailure<sup>TS</sup>, PermissionRequest<sup>TS</sup> |
| `tool_response` | `any` | Hasil yang dikembalikan dari eksekusi tool | PostToolUse |
| `error` | `string` | Pesan kesalahan dari kegagalan eksekusi tool | PostToolUseFailure<sup>TS</sup> |
| `is_interrupt` | `boolean` | Apakah kegagalan disebabkan oleh gangguan | PostToolUseFailure<sup>TS</sup> |
| `prompt` | `string` | Teks prompt pengguna | UserPromptSubmit |
| `stop_hook_active` | `boolean` | Apakah hook stop sedang diproses | Stop, SubagentStop |
| `agent_id` | `string` | Pengidentifikasi unik untuk subagen | SubagentStart<sup>TS</sup>, SubagentStop<sup>TS</sup> |
| `agent_type` | `string` | Tipe/peran subagen | SubagentStart<sup>TS</sup> |
| `agent_transcript_path` | `string` | Jalur ke transkrip percakapan subagen | SubagentStop<sup>TS</sup> |
| `trigger` | `string` | Apa yang memicu pemadatan: `manual` atau `auto` | PreCompact |
| `custom_instructions` | `string` | Instruksi kustom yang disediakan untuk pemadatan | PreCompact |
| `permission_suggestions` | `array` | Pembaruan izin yang disarankan untuk tool | PermissionRequest<sup>TS</sup> |
| `source` | `string` | Bagaimana sesi dimulai: `startup`, `resume`, `clear`, atau `compact` | SessionStart<sup>TS</sup> |
| `reason` | `string` | Mengapa sesi berakhir: `clear`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled`, atau `other` | SessionEnd<sup>TS</sup> |
| `message` | `string` | Pesan status dari agen | Notification<sup>TS</sup> |
| `notification_type` | `string` | Tipe notifikasi: `permission_prompt`, `idle_prompt`, `auth_success`, atau `elicitation_dialog` | Notification<sup>TS</sup> |
| `title` | `string` | Judul opsional yang ditetapkan oleh agen | Notification<sup>TS</sup> |

Kode di bawah mendefinisikan callback hook yang menggunakan `tool_name` dan `tool_input` untuk mencatat detail tentang setiap pemanggilan tool:

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

### Output callback

Fungsi callback Anda mengembalikan objek yang memberitahu SDK cara melanjutkan. Kembalikan objek kosong `{}` untuk mengizinkan operasi tanpa perubahan. Untuk memblokir, memodifikasi, atau menambahkan konteks ke operasi, kembalikan objek dengan bidang `hookSpecificOutput` yang berisi keputusan Anda.

**Bidang tingkat atas** (di luar `hookSpecificOutput`):

| Bidang | Tipe | Deskripsi |
|-------|------|-------------|
| `continue` | `boolean` | Apakah agen harus melanjutkan setelah hook ini (default: `true`) |
| `stopReason` | `string` | Pesan yang ditampilkan ketika `continue` adalah `false` |
| `suppressOutput` | `boolean` | Sembunyikan stdout dari transkrip (default: `false`) |
| `systemMessage` | `string` | Pesan yang disuntikkan ke dalam percakapan untuk Claude lihat |

**Bidang di dalam `hookSpecificOutput`**:

| Bidang | Tipe | Hooks | Deskripsi |
|-------|------|-------|-------------|
| `hookEventName` | `string` | Semua | Diperlukan. Gunakan `input.hook_event_name` untuk mencocokkan acara saat ini |
| `permissionDecision` | `'allow'` \| `'deny'` \| `'ask'` | PreToolUse | Mengontrol apakah tool dieksekusi |
| `permissionDecisionReason` | `string` | PreToolUse | Penjelasan yang ditampilkan kepada Claude untuk keputusan |
| `updatedInput` | `object` | PreToolUse | Input tool yang dimodifikasi (memerlukan `permissionDecision: 'allow'`) |
| `additionalContext` | `string` | PostToolUse, UserPromptSubmit, SessionStart<sup>TS</sup>, SubagentStart<sup>TS</sup> | Konteks yang ditambahkan ke percakapan |

Contoh ini memblokir operasi penulisan ke direktori `/etc` sambil menyuntikkan pesan sistem untuk mengingatkan Claude tentang praktik file yang aman:

<CodeGroup>

```python Python
async def block_etc_writes(input_data, tool_use_id, context):
    file_path = input_data['tool_input'].get('file_path', '')

    if file_path.startswith('/etc'):
        return {
            # Top-level field: inject guidance into the conversation
            'systemMessage': 'Remember: system directories like /etc are protected.',
            # hookSpecificOutput: block the operation
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
      // Top-level field: inject guidance into the conversation
      systemMessage: 'Remember: system directories like /etc are protected.',
      // hookSpecificOutput: block the operation
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

#### Alur keputusan izin

Ketika beberapa hook atau aturan izin berlaku, SDK mengevaluasinya dalam urutan ini:

1. Aturan **Deny** diperiksa terlebih dahulu (kecocokan apa pun = penolakan segera).
2. Aturan **Ask** diperiksa kedua.
3. Aturan **Allow** diperiksa ketiga.
4. **Default ke Ask** jika tidak ada yang cocok.

Jika hook apa pun mengembalikan `deny`, operasi diblokir—hook lain yang mengembalikan `allow` tidak akan menimpanya.

#### Blokir tool

Kembalikan keputusan deny untuk mencegah eksekusi tool:

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

#### Modifikasi input tool

Kembalikan input yang diperbarui untuk mengubah apa yang diterima tool:

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
Saat menggunakan `updatedInput`, Anda juga harus menyertakan `permissionDecision`. Selalu kembalikan objek baru daripada mutasi `tool_input` asli.
</Note>

#### Tambahkan pesan sistem

Suntikkan konteks ke dalam percakapan:

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

#### Persetujuan otomatis untuk tool tertentu

Lewati prompt izin untuk tool terpercaya. Ini berguna ketika Anda ingin operasi tertentu berjalan tanpa konfirmasi pengguna:

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
Bidang `permissionDecision` menerima tiga nilai: `'allow'` (persetujuan otomatis), `'deny'` (blokir), atau `'ask'` (minta konfirmasi).
</Note>

## Tangani skenario lanjutan

Pola-pola ini membantu Anda membangun sistem hook yang lebih canggih untuk kasus penggunaan yang kompleks.

### Chaining multiple hooks

Hooks dieksekusi dalam urutan kemunculannya di array. Jaga setiap hook tetap fokus pada tanggung jawab tunggal dan rantai beberapa hook untuk logika kompleks. Contoh ini menjalankan keempat hook untuk setiap pemanggilan tool (tidak ada matcher yang ditentukan):

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            HookMatcher(hooks=[rate_limiter]),        # First: check rate limits
            HookMatcher(hooks=[authorization_check]), # Second: verify permissions
            HookMatcher(hooks=[input_sanitizer]),     # Third: sanitize inputs
            HookMatcher(hooks=[audit_logger])         # Last: log the action
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    'PreToolUse': [
      { hooks: [rateLimiter] },        // First: check rate limits
      { hooks: [authorizationCheck] }, // Second: verify permissions
      { hooks: [inputSanitizer] },     // Third: sanitize inputs
      { hooks: [auditLogger] }         // Last: log the action
    ]
  }
};
```

</CodeGroup>

### Matcher spesifik tool dengan regex

Gunakan pola regex untuk mencocokkan beberapa tool:

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            # Match file modification tools
            HookMatcher(matcher='Write|Edit|Delete', hooks=[file_security_hook]),

            # Match all MCP tools
            HookMatcher(matcher='^mcp__', hooks=[mcp_audit_hook]),

            # Match everything (no matcher)
            HookMatcher(hooks=[global_logger])
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    'PreToolUse': [
      // Match file modification tools
      { matcher: 'Write|Edit|Delete', hooks: [fileSecurityHook] },

      // Match all MCP tools
      { matcher: '^mcp__', hooks: [mcpAuditHook] },

      // Match everything (no matcher)
      { hooks: [globalLogger] }
    ]
  }
};
```

</CodeGroup>

<Note>
Matcher hanya mencocokkan **nama tool**, bukan jalur file atau argumen lainnya. Untuk memfilter berdasarkan jalur file, periksa `tool_input.file_path` di dalam callback hook Anda.
</Note>

### Melacak aktivitas subagen

Gunakan hook `SubagentStop` untuk memantau penyelesaian subagen. `tool_use_id` membantu menghubungkan panggilan agen induk dengan subagen mereka:

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

### Operasi async dalam hooks

Hooks dapat melakukan operasi async seperti permintaan HTTP. Tangani kesalahan dengan baik dengan menangkap pengecualian daripada melemparnya. Di TypeScript, teruskan `signal` ke `fetch()` sehingga permintaan dibatalkan jika hook kedaluwarsa:

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
    // Pass signal for proper cancellation
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

### Mengirim notifikasi (hanya TypeScript)

Gunakan hook `Notification` untuk menerima pembaruan status dari agen dan meneruskannya ke layanan eksternal seperti Slack atau dashboard pemantauan:

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

## Perbaiki masalah umum

Bagian ini mencakup masalah umum dan cara menyelesaikannya.

### Hook tidak dipicu

- Verifikasi nama acara hook benar dan peka huruf besar-kecil (`PreToolUse`, bukan `preToolUse`)
- Periksa bahwa pola matcher Anda cocok dengan nama tool dengan tepat
- Pastikan hook berada di bawah tipe acara yang benar dalam `options.hooks`
- Untuk hook `SubagentStop`, `Stop`, `SessionStart`, `SessionEnd`, dan `Notification`, matcher diabaikan. Hook ini dipicu untuk semua acara dari tipe itu.
- Hook mungkin tidak dipicu ketika agen mencapai batas [`max_turns`](/docs/id/agent-sdk/python#configuration-options) karena sesi berakhir sebelum hook dapat dieksekusi

### Matcher tidak memfilter seperti yang diharapkan

Matcher hanya mencocokkan **nama tool**, bukan jalur file atau argumen lainnya. Untuk memfilter berdasarkan jalur file, periksa `tool_input.file_path` di dalam hook Anda:

```typescript
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const filePath = preInput.tool_input?.file_path as string;
  if (!filePath?.endsWith('.md')) return {};  // Skip non-markdown files
  // Process markdown files...
};
```

### Hook timeout

- Tingkatkan nilai `timeout` dalam konfigurasi `HookMatcher`
- Gunakan `AbortSignal` dari argumen callback ketiga untuk menangani pembatalan dengan baik di TypeScript

### Tool diblokir secara tidak terduga

- Periksa semua hook `PreToolUse` untuk pengembalian `permissionDecision: 'deny'`
- Tambahkan logging ke hook Anda untuk melihat apa `permissionDecisionReason` yang mereka kembalikan
- Verifikasi pola matcher tidak terlalu luas (matcher kosong cocok dengan semua tool)

### Input yang dimodifikasi tidak diterapkan

- Pastikan `updatedInput` berada di dalam `hookSpecificOutput`, bukan di tingkat atas:

  ```typescript
  return {
    hookSpecificOutput: {
      hookEventName: input.hook_event_name,
      permissionDecision: 'allow',
      updatedInput: { command: 'new command' }
    }
  };
  ```

- Anda juga harus mengembalikan `permissionDecision: 'allow'` agar modifikasi input berlaku
- Sertakan `hookEventName` dalam `hookSpecificOutput` untuk mengidentifikasi tipe hook mana output tersebut

### Hook sesi tidak tersedia

Hook `SessionStart`, `SessionEnd`, dan `Notification` hanya tersedia di TypeScript SDK. Python SDK tidak mendukung acara ini karena keterbatasan setup.

### Prompt izin subagen berlipat ganda

Saat memijahkan beberapa subagen, masing-masing mungkin meminta izin secara terpisah. Subagen tidak secara otomatis mewarisi izin agen induk. Untuk menghindari prompt berulang, gunakan hook `PreToolUse` untuk persetujuan otomatis tool tertentu, atau konfigurasi aturan izin yang berlaku untuk sesi subagen.

### Loop hook rekursif dengan subagen

Hook `UserPromptSubmit` yang memijahkan subagen dapat membuat loop tak terbatas jika subagen tersebut memicu hook yang sama. Untuk mencegah ini:

- Periksa indikator subagen dalam input hook sebelum memijahkan
- Gunakan bidang `parent_tool_use_id` untuk mendeteksi jika Anda sudah dalam konteks subagen
- Batasi hook hanya berjalan untuk sesi agen tingkat atas

### systemMessage tidak muncul dalam output

Bidang `systemMessage` menambahkan konteks ke percakapan yang model lihat, tetapi mungkin tidak muncul di semua mode output SDK. Jika Anda perlu menampilkan keputusan hook ke aplikasi Anda, log mereka secara terpisah atau gunakan saluran output khusus.

## Pelajari lebih lanjut

- [Permissions](/docs/id/agent-sdk/permissions): kontrol apa yang dapat dilakukan agen Anda
- [Custom Tools](/docs/id/agent-sdk/custom-tools): bangun tool untuk memperluas kemampuan agen
- [TypeScript SDK Reference](/docs/id/agent-sdk/typescript)
- [Python SDK Reference](/docs/id/agent-sdk/python)