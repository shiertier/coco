# Agent Skills dalam SDK

Perluas Claude dengan kemampuan khusus menggunakan Agent Skills dalam Claude Agent SDK

---

## Ringkasan

Agent Skills memperluas Claude dengan kemampuan khusus yang Claude secara otomatis memanggil ketika relevan. Skills dikemas sebagai file `SKILL.md` yang berisi instruksi, deskripsi, dan sumber daya pendukung opsional.

Untuk informasi komprehensif tentang Skills, termasuk manfaat, arsitektur, dan pedoman penulisan, lihat [ringkasan Agent Skills](/docs/id/agents-and-tools/agent-skills/overview).

## Cara Skills Bekerja dengan SDK

Saat menggunakan Claude Agent SDK, Skills adalah:

1. **Didefinisikan sebagai artefak sistem file**: Dibuat sebagai file `SKILL.md` di direktori tertentu (`.claude/skills/`)
2. **Dimuat dari sistem file**: Skills dimuat dari lokasi sistem file yang dikonfigurasi. Anda harus menentukan `settingSources` (TypeScript) atau `setting_sources` (Python) untuk memuat Skills dari sistem file
3. **Ditemukan secara otomatis**: Setelah pengaturan sistem file dimuat, metadata Skill ditemukan saat startup dari direktori pengguna dan proyek; konten penuh dimuat ketika dipicu
4. **Dipanggil model**: Claude secara otomatis memilih kapan menggunakannya berdasarkan konteks
5. **Diaktifkan melalui allowed_tools**: Tambahkan `"Skill"` ke `allowed_tools` Anda untuk mengaktifkan Skills

Tidak seperti subagents (yang dapat didefinisikan secara terprogram), Skills harus dibuat sebagai artefak sistem file. SDK tidak menyediakan API terprogram untuk mendaftarkan Skills.

<Note>
**Perilaku default**: Secara default, SDK tidak memuat pengaturan sistem file apa pun. Untuk menggunakan Skills, Anda harus secara eksplisit mengonfigurasi `settingSources: ['user', 'project']` (TypeScript) atau `setting_sources=["user", "project"]` (Python) dalam opsi Anda.
</Note>

## Menggunakan Skills dengan SDK

Untuk menggunakan Skills dengan SDK, Anda perlu:

1. Sertakan `"Skill"` dalam konfigurasi `allowed_tools` Anda
2. Konfigurasikan `settingSources`/`setting_sources` untuk memuat Skills dari sistem file

Setelah dikonfigurasi, Claude secara otomatis menemukan Skills dari direktori yang ditentukan dan memanggilnya ketika relevan dengan permintaan pengguna.

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

## Lokasi Skill

Skills dimuat dari direktori sistem file berdasarkan konfigurasi `settingSources`/`setting_sources` Anda:

- **Project Skills** (`.claude/skills/`): Dibagikan dengan tim Anda melalui git - dimuat ketika `setting_sources` mencakup `"project"`
- **User Skills** (`~/.claude/skills/`): Skills pribadi di semua proyek - dimuat ketika `setting_sources` mencakup `"user"`
- **Plugin Skills**: Disertakan dengan plugin Claude Code yang diinstal

## Membuat Skills

Skills didefinisikan sebagai direktori yang berisi file `SKILL.md` dengan frontmatter YAML dan konten Markdown. Bidang `description` menentukan kapan Claude memanggil Skill Anda.

**Contoh struktur direktori**:
```bash
.claude/skills/processing-pdfs/
└── SKILL.md
```

Untuk panduan lengkap tentang membuat Skills, termasuk struktur SKILL.md, Skills multi-file, dan contoh, lihat:
- [Agent Skills dalam Claude Code](https://code.claude.com/docs/skills): Panduan lengkap dengan contoh
- [Agent Skills Best Practices](/docs/id/agents-and-tools/agent-skills/best-practices): Pedoman penulisan dan konvensi penamaan

## Pembatasan Alat

<Note>
Bidang frontmatter `allowed-tools` dalam SKILL.md hanya didukung saat menggunakan Claude Code CLI secara langsung. **Ini tidak berlaku saat menggunakan Skills melalui SDK**.

Saat menggunakan SDK, kontrol akses alat melalui opsi `allowedTools` utama dalam konfigurasi kueri Anda.
</Note>

Untuk membatasi alat untuk Skills dalam aplikasi SDK, gunakan opsi `allowedTools`:

<Note>
Pernyataan impor dari contoh pertama diasumsikan dalam cuplikan kode berikut.
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

## Menemukan Skills yang Tersedia

Untuk melihat Skills mana yang tersedia dalam aplikasi SDK Anda, cukup tanyakan kepada Claude:

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

Claude akan mencantumkan Skills yang tersedia berdasarkan direktori kerja saat ini dan plugin yang diinstal.

## Menguji Skills

Uji Skills dengan mengajukan pertanyaan yang cocok dengan deskripsi mereka:

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

Claude secara otomatis memanggil Skill yang relevan jika deskripsi cocok dengan permintaan Anda.

## Pemecahan Masalah

### Skills Tidak Ditemukan

**Periksa konfigurasi settingSources**: Skills hanya dimuat ketika Anda secara eksplisit mengonfigurasi `settingSources`/`setting_sources`. Ini adalah masalah paling umum:

<CodeGroup>

```python Python
# Wrong - Skills won't be loaded
options = ClaudeAgentOptions(
    allowed_tools=["Skill"]
)

# Correct - Skills will be loaded
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// Wrong - Skills won't be loaded
const options = {
  allowedTools: ["Skill"]
};

// Correct - Skills will be loaded
const options = {
  settingSources: ["user", "project"],  # Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

Untuk detail lebih lanjut tentang `settingSources`/`setting_sources`, lihat [referensi SDK TypeScript](/docs/id/agent-sdk/typescript#settingsource) atau [referensi SDK Python](/docs/id/agent-sdk/python#settingsource).

**Periksa direktori kerja**: SDK memuat Skills relatif terhadap opsi `cwd`. Pastikan itu menunjuk ke direktori yang berisi `.claude/skills/`:

<CodeGroup>

```python Python
# Ensure your cwd points to the directory containing .claude/skills/
options = ClaudeAgentOptions(
    cwd="/path/to/project",  # Must contain .claude/skills/
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// Ensure your cwd points to the directory containing .claude/skills/
const options = {
  cwd: "/path/to/project",  // Must contain .claude/skills/
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

Lihat bagian "Menggunakan Skills dengan SDK" di atas untuk pola lengkapnya.

**Verifikasi lokasi sistem file**:
```bash
# Check project Skills
ls .claude/skills/*/SKILL.md

# Check personal Skills
ls ~/.claude/skills/*/SKILL.md
```

### Skill Tidak Digunakan

**Periksa alat Skill diaktifkan**: Konfirmasi `"Skill"` ada dalam `allowedTools` Anda.

**Periksa deskripsi**: Pastikan itu spesifik dan mencakup kata kunci yang relevan. Lihat [Agent Skills Best Practices](/docs/id/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) untuk panduan tentang menulis deskripsi yang efektif.

### Pemecahan Masalah Tambahan

Untuk pemecahan masalah Skills umum (sintaks YAML, debugging, dll.), lihat [bagian pemecahan masalah Claude Code Skills](https://code.claude.com/docs/skills#troubleshooting).

## Dokumentasi Terkait

### Panduan Skills
- [Agent Skills dalam Claude Code](https://code.claude.com/docs/skills): Panduan Skills lengkap dengan pembuatan, contoh, dan pemecahan masalah
- [Ringkasan Agent Skills](/docs/id/agents-and-tools/agent-skills/overview): Ringkasan konseptual, manfaat, dan arsitektur
- [Agent Skills Best Practices](/docs/id/agents-and-tools/agent-skills/best-practices): Pedoman penulisan untuk Skills yang efektif
- [Agent Skills Cookbook](https://github.com/anthropics/claude-cookbooks/tree/main/skills): Contoh Skills dan template

### Sumber Daya SDK
- [Subagents dalam SDK](/docs/id/agent-sdk/subagents): Agen berbasis sistem file serupa dengan opsi terprogram
- [Slash Commands dalam SDK](/docs/id/agent-sdk/slash-commands): Perintah yang dipanggil pengguna
- [Ringkasan SDK](/docs/id/agent-sdk/overview): Konsep SDK umum
- [Referensi SDK TypeScript](/docs/id/agent-sdk/typescript): Dokumentasi API lengkap
- [Referensi SDK Python](/docs/id/agent-sdk/python): Dokumentasi API lengkap