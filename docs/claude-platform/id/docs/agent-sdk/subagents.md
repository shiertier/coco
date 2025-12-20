# Subagen dalam SDK

Bekerja dengan subagen dalam Claude Agent SDK

---

Subagen dalam Claude Agent SDK adalah AI khusus yang diatur oleh agen utama.
Gunakan subagen untuk manajemen konteks dan paralelisasi.

Panduan ini menjelaskan cara mendefinisikan dan menggunakan subagen dalam SDK menggunakan parameter `agents`.

## Gambaran Umum

Subagen dapat didefinisikan dengan dua cara saat menggunakan SDK:

1. **Secara Programatis** - Menggunakan parameter `agents` dalam opsi `query()` Anda (direkomendasikan untuk aplikasi SDK)
2. **Berbasis Sistem File** - Menempatkan file markdown dengan frontmatter YAML di direktori yang ditentukan (`.claude/agents/`)

Panduan ini terutama berfokus pada pendekatan programatis menggunakan parameter `agents`, yang memberikan pengalaman pengembangan yang lebih terintegrasi untuk aplikasi SDK.

## Manfaat Menggunakan Subagen

### Manajemen Konteks
Subagen mempertahankan konteks terpisah dari agen utama, mencegah kelebihan informasi dan menjaga interaksi tetap fokus. Isolasi ini memastikan bahwa tugas-tugas khusus tidak mencemari konteks percakapan utama dengan detail yang tidak relevan.

**Contoh**: Subagen `research-assistant` dapat menjelajahi puluhan file dan halaman dokumentasi tanpa mengacaukan percakapan utama dengan semua hasil pencarian perantara - hanya mengembalikan temuan yang relevan.

### Paralelisasi
Beberapa subagen dapat berjalan secara bersamaan, secara dramatis mempercepat alur kerja yang kompleks.

**Contoh**: Selama tinjauan kode, Anda dapat menjalankan subagen `style-checker`, `security-scanner`, dan `test-coverage` secara bersamaan, mengurangi waktu tinjauan dari menit menjadi detik.

### Instruksi dan Pengetahuan Khusus
Setiap subagen dapat memiliki prompt sistem yang disesuaikan dengan keahlian, praktik terbaik, dan batasan tertentu.

**Contoh**: Subagen `database-migration` dapat memiliki pengetahuan detail tentang praktik terbaik SQL, strategi rollback, dan pemeriksaan integritas data yang akan menjadi noise yang tidak perlu dalam instruksi agen utama.

### Pembatasan Alat
Subagen dapat dibatasi pada alat tertentu, mengurangi risiko tindakan yang tidak diinginkan.

**Contoh**: Subagen `doc-reviewer` mungkin hanya memiliki akses ke alat Read dan Grep, memastikan ia dapat menganalisis tetapi tidak pernah secara tidak sengaja memodifikasi file dokumentasi Anda.

## Membuat Subagen

### Definisi Programatis (Direkomendasikan)

Definisikan subagen langsung dalam kode Anda menggunakan parameter `agents`:

```typescript
import { query } from '@anthropic-ai/claude-agent-sdk';

const result = query({
  prompt: "Tinjau modul autentikasi untuk masalah keamanan",
  options: {
    agents: {
      'code-reviewer': {
        description: 'Spesialis tinjauan kode ahli. Gunakan untuk tinjauan kualitas, keamanan, dan maintainability.',
        prompt: `Anda adalah spesialis tinjauan kode dengan keahlian dalam keamanan, performa, dan praktik terbaik.

Saat meninjau kode:
- Identifikasi kerentanan keamanan
- Periksa masalah performa
- Verifikasi kepatuhan terhadap standar coding
- Sarankan perbaikan spesifik

Jadilah menyeluruh tetapi ringkas dalam umpan balik Anda.`,
        tools: ['Read', 'Grep', 'Glob'],
        model: 'sonnet'
      },
      'test-runner': {
        description: 'Menjalankan dan menganalisis test suite. Gunakan untuk eksekusi tes dan analisis cakupan.',
        prompt: `Anda adalah spesialis eksekusi tes. Jalankan tes dan berikan analisis yang jelas tentang hasilnya.

Fokus pada:
- Menjalankan perintah tes
- Menganalisis output tes
- Mengidentifikasi tes yang gagal
- Menyarankan perbaikan untuk kegagalan`,
        tools: ['Bash', 'Read', 'Grep'],
      }
    }
  }
});

for await (const message of result) {
  console.log(message);
}
```

### Konfigurasi AgentDefinition

| Field | Type | Required | Description |
|:---|:---|:---|:---|
| `description` | `string` | Ya | Deskripsi bahasa alami tentang kapan menggunakan agen ini |
| `prompt` | `string` | Ya | Prompt sistem agen yang mendefinisikan peran dan perilakunya |
| `tools` | `string[]` | Tidak | Array nama alat yang diizinkan. Jika dihilangkan, mewarisi semua alat |
| `model` | `'sonnet' \| 'opus' \| 'haiku' \| 'inherit'` | Tidak | Override model untuk agen ini. Default ke model utama jika dihilangkan |

### Definisi Berbasis Sistem File (Alternatif)

Anda juga dapat mendefinisikan subagen sebagai file markdown di direktori tertentu:

- **Level proyek**: `.claude/agents/*.md` - Tersedia hanya dalam proyek saat ini
- **Level pengguna**: `~/.claude/agents/*.md` - Tersedia di semua proyek

Setiap subagen adalah file markdown dengan frontmatter YAML:

```markdown
---
name: code-reviewer
description: Spesialis tinjauan kode ahli. Gunakan untuk tinjauan kualitas, keamanan, dan maintainability.
tools: Read, Grep, Glob, Bash
---

Prompt sistem subagen Anda ada di sini. Ini mendefinisikan peran subagen,
kemampuan, dan pendekatan untuk memecahkan masalah.
```

**Catatan:** Agen yang didefinisikan secara programatis (melalui parameter `agents`) mengambil prioritas atas agen berbasis sistem file dengan nama yang sama.

## Bagaimana SDK Menggunakan Subagen

Saat menggunakan Claude Agent SDK, subagen dapat didefinisikan secara programatis atau dimuat dari sistem file. Claude akan:

1. **Memuat agen programatis** dari parameter `agents` dalam opsi Anda
2. **Auto-deteksi agen sistem file** dari direktori `.claude/agents/` (jika tidak ditimpa)
3. **Memanggil mereka secara otomatis** berdasarkan pencocokan tugas dan `description` agen
4. **Menggunakan prompt khusus mereka** dan pembatasan alat
5. **Mempertahankan konteks terpisah** untuk setiap pemanggilan subagen

Agen yang didefinisikan secara programatis (melalui parameter `agents`) mengambil prioritas atas agen berbasis sistem file dengan nama yang sama.

## Contoh Subagen

Untuk contoh komprehensif subagen termasuk peninjau kode, pelari tes, debugger, dan auditor keamanan, lihat [panduan Subagen utama](https://code.claude.com/docs/sub-agents#example-subagents). Panduan ini mencakup konfigurasi detail dan praktik terbaik untuk membuat subagen yang efektif.

## Pola Integrasi SDK

### Pemanggilan Otomatis

SDK akan secara otomatis memanggil subagen yang sesuai berdasarkan konteks tugas. Pastikan field `description` agen Anda dengan jelas menunjukkan kapan harus digunakan:

```typescript
const result = query({
  prompt: "Optimalkan query database di lapisan API",
  options: {
    agents: {
      'performance-optimizer': {
        description: 'Gunakan PROAKTIF ketika perubahan kode mungkin berdampak pada performa. HARUS DIGUNAKAN untuk tugas optimisasi.',
        prompt: 'Anda adalah spesialis optimisasi performa...',
        tools: ['Read', 'Edit', 'Bash', 'Grep'],
        model: 'sonnet'
      }
    }
  }
});
```

### Pemanggilan Eksplisit

Pengguna dapat meminta subagen tertentu dalam prompt mereka:

```typescript
const result = query({
  prompt: "Gunakan agen code-reviewer untuk memeriksa modul autentikasi",
  options: {
    agents: {
      'code-reviewer': {
        description: 'Spesialis tinjauan kode ahli',
        prompt: 'Anda adalah peninjau kode yang berfokus pada keamanan...',
        tools: ['Read', 'Grep', 'Glob']
      }
    }
  }
});
```

### Konfigurasi Agen Dinamis

Anda dapat mengkonfigurasi agen secara dinamis berdasarkan kebutuhan aplikasi Anda:

```typescript
import { query, type AgentDefinition } from '@anthropic-ai/claude-agent-sdk';

function createSecurityAgent(securityLevel: 'basic' | 'strict'): AgentDefinition {
  return {
    description: 'Peninjau kode keamanan',
    prompt: `Anda adalah peninjau keamanan ${securityLevel === 'strict' ? 'ketat' : 'seimbang'}...`,
    tools: ['Read', 'Grep', 'Glob'],
    model: securityLevel === 'strict' ? 'opus' : 'sonnet'
  };
}

const result = query({
  prompt: "Tinjau PR ini untuk masalah keamanan",
  options: {
    agents: {
      'security-reviewer': createSecurityAgent('strict')
    }
  }
});
```

## Pembatasan Alat

Subagen dapat memiliki akses alat yang dibatasi melalui field `tools`:

- **Hilangkan field** - Agen mewarisi semua alat yang tersedia (default)
- **Tentukan alat** - Agen hanya dapat menggunakan alat yang terdaftar

Contoh agen analisis read-only:

```typescript
const result = query({
  prompt: "Analisis arsitektur dari codebase ini",
  options: {
    agents: {
      'code-analyzer': {
        description: 'Analisis kode statis dan tinjauan arsitektur',
        prompt: `Anda adalah analis arsitektur kode. Analisis struktur kode,
identifikasi pola, dan sarankan perbaikan tanpa membuat perubahan.`,
        tools: ['Read', 'Grep', 'Glob']  // Tidak ada izin tulis atau eksekusi
      }
    }
  }
});
```

### Kombinasi Alat Umum

**Agen read-only** (analisis, tinjauan):
```typescript
tools: ['Read', 'Grep', 'Glob']
```

**Agen eksekusi tes**:
```typescript
tools: ['Bash', 'Read', 'Grep']
```

**Agen modifikasi kode**:
```typescript
tools: ['Read', 'Edit', 'Write', 'Grep', 'Glob']
```

## Dokumentasi Terkait

- [Panduan Subagen Utama](https://code.claude.com/docs/sub-agents) - Dokumentasi subagen komprehensif
- [Gambaran Umum SDK](/docs/id/agent-sdk/overview) - Gambaran umum Claude Agent SDK
- [Pengaturan](https://code.claude.com/docs/settings) - Referensi file konfigurasi
- [Perintah Slash](https://code.claude.com/docs/slash-commands) - Pembuatan perintah kustom