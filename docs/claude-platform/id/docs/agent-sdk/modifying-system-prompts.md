# Memodifikasi system prompt

Pelajari cara menyesuaikan perilaku Claude dengan memodifikasi system prompt menggunakan tiga pendekatan - output styles, systemPrompt dengan append, dan custom system prompt.

---

System prompt mendefinisikan perilaku, kemampuan, dan gaya respons Claude. Claude Agent SDK menyediakan tiga cara untuk menyesuaikan system prompt: menggunakan output styles (konfigurasi persisten berbasis file), menambahkan ke prompt Claude Code, atau menggunakan prompt yang sepenuhnya kustom.

## Memahami system prompt

System prompt adalah set instruksi awal yang membentuk bagaimana Claude berperilaku sepanjang percakapan.

<Note>
**Perilaku default:** Agent SDK menggunakan **system prompt kosong** secara default untuk fleksibilitas maksimum. Untuk menggunakan system prompt Claude Code (instruksi tool, panduan kode, dll.), tentukan `systemPrompt: { preset: "claude_code" }` di TypeScript atau `system_prompt="claude_code"` di Python.
</Note>

System prompt Claude Code mencakup:

- Instruksi penggunaan tool dan tool yang tersedia
- Panduan gaya dan format kode
- Pengaturan nada respons dan verbositas
- Instruksi keamanan dan keselamatan
- Konteks tentang direktori kerja saat ini dan lingkungan

## Metode modifikasi

### Metode 1: File CLAUDE.md (instruksi tingkat proyek)

File CLAUDE.md menyediakan konteks dan instruksi spesifik proyek yang secara otomatis dibaca oleh Agent SDK ketika berjalan di sebuah direktori. Mereka berfungsi sebagai "memori" persisten untuk proyek Anda.

#### Cara CLAUDE.md bekerja dengan SDK

**Lokasi dan penemuan:**

- **Tingkat proyek:** `CLAUDE.md` atau `.claude/CLAUDE.md` di direktori kerja Anda
- **Tingkat pengguna:** `~/.claude/CLAUDE.md` untuk instruksi global di semua proyek

**PENTING:** SDK hanya membaca file CLAUDE.md ketika Anda secara eksplisit mengkonfigurasi `settingSources` (TypeScript) atau `setting_sources` (Python):

- Sertakan `'project'` untuk memuat CLAUDE.md tingkat proyek
- Sertakan `'user'` untuk memuat CLAUDE.md tingkat pengguna (`~/.claude/CLAUDE.md`)

Preset system prompt `claude_code` TIDAK secara otomatis memuat CLAUDE.md - Anda juga harus menentukan setting sources.

**Format konten:**
File CLAUDE.md menggunakan markdown biasa dan dapat berisi:

- Panduan dan standar coding
- Konteks spesifik proyek
- Perintah atau workflow umum
- Konvensi API
- Persyaratan testing

#### Contoh CLAUDE.md

```markdown
# Project Guidelines

## Code Style

- Use TypeScript strict mode
- Prefer functional components in React
- Always include JSDoc comments for public APIs

## Testing

- Run `npm test` before committing
- Maintain >80% code coverage
- Use jest for unit tests, playwright for E2E

## Commands

- Build: `npm run build`
- Dev server: `npm run dev`
- Type check: `npm run typecheck`
```

#### Menggunakan CLAUDE.md dengan SDK

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// PENTING: Anda harus menentukan settingSources untuk memuat CLAUDE.md
// Preset claude_code saja TIDAK memuat file CLAUDE.md
const messages = [];

for await (const message of query({
  prompt: "Add a new React component for user profiles",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code", // Gunakan system prompt Claude Code
    },
    settingSources: ["project"], // Diperlukan untuk memuat CLAUDE.md dari proyek
  },
})) {
  messages.push(message);
}

// Sekarang Claude memiliki akses ke panduan proyek Anda dari CLAUDE.md
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# PENTING: Anda harus menentukan setting_sources untuk memuat CLAUDE.md
# Preset claude_code saja TIDAK memuat file CLAUDE.md
messages = []

async for message in query(
    prompt="Add a new React component for user profiles",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code"  # Gunakan system prompt Claude Code
        },
        setting_sources=["project"]  # Diperlukan untuk memuat CLAUDE.md dari proyek
    )
):
    messages.append(message)

# Sekarang Claude memiliki akses ke panduan proyek Anda dari CLAUDE.md
```

</CodeGroup>

#### Kapan menggunakan CLAUDE.md

**Terbaik untuk:**

- **Konteks yang dibagikan tim** - Panduan yang harus diikuti semua orang
- **Konvensi proyek** - Standar coding, struktur file, pola penamaan
- **Perintah umum** - Perintah build, test, deploy spesifik untuk proyek Anda
- **Memori jangka panjang** - Konteks yang harus bertahan di semua sesi
- **Instruksi yang dikontrol versi** - Commit ke git agar tim tetap sinkron

**Karakteristik kunci:**

- ✅ Persisten di semua sesi dalam proyek
- ✅ Dibagikan dengan tim melalui git
- ✅ Penemuan otomatis (tidak perlu perubahan kode)
- ⚠️ Memerlukan loading settings melalui `settingSources`

### Metode 2: Output styles (konfigurasi persisten)

Output styles adalah konfigurasi tersimpan yang memodifikasi system prompt Claude. Mereka disimpan sebagai file markdown dan dapat digunakan kembali di berbagai sesi dan proyek.

#### Membuat output style

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
  // Tingkat pengguna: ~/.claude/output-styles
  // Tingkat proyek: .claude/output-styles
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

// Contoh: Buat spesialis code review
await createOutputStyle(
  "Code Reviewer",
  "Thorough code review assistant",
  `You are an expert code reviewer.

For every code submission:
1. Check for bugs and security issues
2. Evaluate performance
3. Suggest improvements
4. Rate code quality (1-10)`
);
```

```python Python
from pathlib import Path

async def create_output_style(name: str, description: str, prompt: str):
    # Tingkat pengguna: ~/.claude/output-styles
    # Tingkat proyek: .claude/output-styles
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

# Contoh: Buat spesialis code review
await create_output_style(
    'Code Reviewer',
    'Thorough code review assistant',
    """You are an expert code reviewer.

For every code submission:
1. Check for bugs and security issues
2. Evaluate performance
3. Suggest improvements
4. Rate code quality (1-10)"""
)
```

</CodeGroup>

#### Menggunakan output styles

Setelah dibuat, aktifkan output styles melalui:

- **CLI**: `/output-style [style-name]`
- **Settings**: `.claude/settings.local.json`
- **Buat baru**: `/output-style:new [description]`

**Catatan untuk pengguna SDK:** Output styles dimuat ketika Anda menyertakan `settingSources: ['user']` atau `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` atau `setting_sources=["project"]` (Python) dalam opsi Anda.

### Metode 3: Menggunakan `systemPrompt` dengan append

Anda dapat menggunakan preset Claude Code dengan properti `append` untuk menambahkan instruksi kustom Anda sambil mempertahankan semua fungsionalitas bawaan.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const messages = [];

for await (const message of query({
  prompt: "Help me write a Python function to calculate fibonacci numbers",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append:
        "Always include detailed docstrings and type hints in Python code.",
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
    prompt="Help me write a Python function to calculate fibonacci numbers",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": "Always include detailed docstrings and type hints in Python code."
        }
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

### Metode 4: Custom system prompt

Anda dapat menyediakan string kustom sebagai `systemPrompt` untuk mengganti default sepenuhnya dengan instruksi Anda sendiri.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const customPrompt = `You are a Python coding specialist.
Follow these guidelines:
- Write clean, well-documented code
- Use type hints for all functions
- Include comprehensive docstrings
- Prefer functional programming patterns when appropriate
- Always explain your code choices`;

const messages = [];

for await (const message of query({
  prompt: "Create a data processing pipeline",
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

custom_prompt = """You are a Python coding specialist.
Follow these guidelines:
- Write clean, well-documented code
- Use type hints for all functions
- Include comprehensive docstrings
- Prefer functional programming patterns when appropriate
- Always explain your code choices"""

messages = []

async for message in query(
    prompt="Create a data processing pipeline",
    options=ClaudeAgentOptions(
        system_prompt=custom_prompt
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

## Perbandingan keempat pendekatan

| Fitur                   | CLAUDE.md           | Output Styles      | `systemPrompt` dengan append | Custom `systemPrompt`     |
| --- | --- | --- | --- | --- |
| **Persistensi**         | File per-proyek | Disimpan sebagai file  | Hanya sesi            | Hanya sesi           |
| **Reusabilitas**        | Per-proyek      | Lintas proyek | Duplikasi kode        | Duplikasi kode       |
| **Manajemen**           | Di filesystem    | CLI + file     | Dalam kode                 | Dalam kode                |
| **Tool default**        | Dipertahankan        | Dipertahankan       | Dipertahankan               | Hilang (kecuali disertakan) |
| **Keamanan bawaan**     | Dipertahankan       | Dipertahankan      | Dipertahankan              | Harus ditambahkan          |
| **Konteks lingkungan** | Otomatis        | Otomatis       | Otomatis               | Harus disediakan       |
| **Level kustomisasi** | Hanya penambahan   | Ganti default | Hanya penambahan          | Kontrol penuh       |
| **Kontrol versi**     | Dengan proyek     | Ya             | Dengan kode               | Dengan kode              |
| **Cakupan**               | Spesifik proyek | Pengguna atau proyek | Sesi kode            | Sesi kode           |

**Catatan:** "Dengan append" berarti menggunakan `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` di TypeScript atau `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` di Python.

## Kasus penggunaan dan praktik terbaik

### Kapan menggunakan CLAUDE.md

**Terbaik untuk:**

- Standar dan konvensi coding spesifik proyek
- Mendokumentasikan struktur dan arsitektur proyek
- Mencantumkan perintah umum (build, test, deploy)
- Konteks yang dibagikan tim yang harus dikontrol versi
- Instruksi yang berlaku untuk semua penggunaan SDK dalam proyek

**Contoh:**

- "Semua endpoint API harus menggunakan pola async/await"
- "Jalankan `npm run lint:fix` sebelum commit"
- "Migrasi database ada di direktori `migrations/`"

**Penting:** Untuk memuat file CLAUDE.md, Anda harus secara eksplisit mengatur `settingSources: ['project']` (TypeScript) atau `setting_sources=["project"]` (Python). Preset system prompt `claude_code` TIDAK secara otomatis memuat CLAUDE.md tanpa pengaturan ini.

### Kapan menggunakan output styles

**Terbaik untuk:**

- Perubahan perilaku persisten di berbagai sesi
- Konfigurasi yang dibagikan tim
- Asisten khusus (code reviewer, data scientist, DevOps)
- Modifikasi prompt kompleks yang memerlukan versioning

**Contoh:**

- Membuat asisten optimisasi SQL khusus
- Membangun code reviewer yang fokus pada keamanan
- Mengembangkan asisten pengajaran dengan pedagogi spesifik

### Kapan menggunakan `systemPrompt` dengan append

**Terbaik untuk:**

- Menambahkan standar atau preferensi coding spesifik
- Menyesuaikan format output
- Menambahkan pengetahuan domain-spesifik
- Memodifikasi verbositas respons
- Meningkatkan perilaku default Claude Code tanpa kehilangan instruksi tool

### Kapan menggunakan custom `systemPrompt`

**Terbaik untuk:**

- Kontrol penuh atas perilaku Claude
- Tugas khusus sesi tunggal
- Menguji strategi prompt baru
- Situasi di mana tool default tidak diperlukan
- Membangun agen khusus dengan perilaku unik

## Menggabungkan pendekatan

Anda dapat menggabungkan metode ini untuk fleksibilitas maksimum:

### Contoh: Output style dengan penambahan spesifik sesi

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Dengan asumsi output style "Code Reviewer" aktif (melalui /output-style)
// Tambahkan area fokus spesifik sesi
const messages = [];

for await (const message of query({
  prompt: "Review this authentication module",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: `
        For this review, prioritize:
        - OAuth 2.0 compliance
        - Token storage security
        - Session management
      `,
    },
  },
})) {
  messages.push(message);
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# Dengan asumsi output style "Code Reviewer" aktif (melalui /output-style)
# Tambahkan area fokus spesifik sesi
messages = []

async for message in query(
    prompt="Review this authentication module",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": """
            For this review, prioritize:
            - OAuth 2.0 compliance
            - Token storage security
            - Session management
            """
        }
    )
):
    messages.append(message)
```

</CodeGroup>

## Lihat juga

- [Output styles](https://code.claude.com/docs/output-styles) - Dokumentasi lengkap output styles
- [Panduan TypeScript SDK](/docs/id/agent-sdk/typescript) - Panduan penggunaan SDK lengkap
- [Referensi TypeScript SDK](https://code.claude.com/docs/typescript-sdk-reference) - Dokumentasi API lengkap
- [Panduan konfigurasi](https://code.claude.com/docs/configuration) - Opsi konfigurasi umum