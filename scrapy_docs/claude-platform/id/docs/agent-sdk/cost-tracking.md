# Melacak Biaya dan Penggunaan

Memahami dan melacak penggunaan token untuk penagihan dalam Claude Agent SDK

---

# Pelacakan Biaya SDK

Claude Agent SDK menyediakan informasi penggunaan token yang detail untuk setiap interaksi dengan Claude. Panduan ini menjelaskan cara melacak biaya dengan benar dan memahami pelaporan penggunaan, terutama ketika menangani penggunaan tool paralel dan percakapan multi-langkah.

Untuk dokumentasi API lengkap, lihat [referensi TypeScript SDK](https://code.claude.com/docs/typescript-sdk-reference).

## Memahami Penggunaan Token

Ketika Claude memproses permintaan, ia melaporkan penggunaan token pada tingkat pesan. Data penggunaan ini penting untuk melacak biaya dan menagih pengguna dengan tepat.

### Konsep Kunci

1. **Langkah**: Langkah adalah pasangan permintaan/respons tunggal antara aplikasi Anda dan Claude
2. **Pesan**: Pesan individual dalam langkah (teks, penggunaan tool, hasil tool)
3. **Penggunaan**: Data konsumsi token yang dilampirkan pada pesan asisten

## Struktur Pelaporan Penggunaan

### Penggunaan Tool Tunggal vs Paralel

Ketika Claude mengeksekusi tool, pelaporan penggunaan berbeda berdasarkan apakah tool dieksekusi secara berurutan atau paralel:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Contoh: Melacak penggunaan dalam percakapan
const result = await query({
  prompt: "Analisis codebase ini dan jalankan tes",
  options: {
    onMessage: (message) => {
      if (message.type === 'assistant' && message.usage) {
        console.log(`ID Pesan: ${message.id}`);
        console.log(`Penggunaan:`, message.usage);
      }
    }
  }
});
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage
import asyncio

# Contoh: Melacak penggunaan dalam percakapan
async def track_usage():
    # Proses pesan saat tiba
    async for message in query(
        prompt="Analisis codebase ini dan jalankan tes"
    ):
        if isinstance(message, AssistantMessage) and hasattr(message, 'usage'):
            print(f"ID Pesan: {message.id}")
            print(f"Penggunaan: {message.usage}")

asyncio.run(track_usage())
```

</CodeGroup>

### Contoh Alur Pesan

Berikut cara pesan dan penggunaan dilaporkan dalam percakapan multi-langkah yang khas:

```
<!-- Langkah 1: Permintaan awal dengan penggunaan tool paralel -->
assistant (text)      { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
user (tool_result)
user (tool_result)
user (tool_result)

<!-- Langkah 2: Respons lanjutan -->
assistant (text)      { id: "msg_2", usage: { output_tokens: 98, ... } }
```

## Aturan Penggunaan Penting

### 1. ID Sama = Penggunaan Sama

**Semua pesan dengan field `id` yang sama melaporkan penggunaan yang identik**. Ketika Claude mengirim beberapa pesan dalam giliran yang sama (misalnya, teks + penggunaan tool), mereka berbagi ID pesan dan data penggunaan yang sama.

```typescript
// Semua pesan ini memiliki ID dan penggunaan yang sama
const messages = [
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } }
];

// Tagih hanya sekali per ID pesan unik
const uniqueUsage = messages[0].usage; // Sama untuk semua pesan dengan ID ini
```

### 2. Tagih Sekali Per Langkah

**Anda hanya boleh menagih pengguna sekali per langkah**, bukan untuk setiap pesan individual. Ketika Anda melihat beberapa pesan asisten dengan ID yang sama, gunakan penggunaan dari salah satu dari mereka.

### 3. Pesan Hasil Berisi Penggunaan Kumulatif

Pesan `result` akhir berisi total penggunaan kumulatif dari semua langkah dalam percakapan:

```typescript
// Hasil akhir mencakup total penggunaan
const result = await query({
  prompt: "Tugas multi-langkah",
  options: { /* ... */ }
});

console.log("Total penggunaan:", result.usage);
console.log("Total biaya:", result.usage.total_cost_usd);
```

## Implementasi: Sistem Pelacakan Biaya

Berikut contoh lengkap implementasi sistem pelacakan biaya:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

class CostTracker {
  private processedMessageIds = new Set<string>();
  private stepUsages: Array<any> = [];
  
  async trackConversation(prompt: string) {
    const result = await query({
      prompt,
      options: {
        onMessage: (message) => {
          this.processMessage(message);
        }
      }
    });
    
    return {
      result,
      stepUsages: this.stepUsages,
      totalCost: result.usage?.total_cost_usd || 0
    };
  }
  
  private processMessage(message: any) {
    // Hanya proses pesan asisten dengan penggunaan
    if (message.type !== 'assistant' || !message.usage) {
      return;
    }
    
    // Lewati jika sudah memproses ID pesan ini
    if (this.processedMessageIds.has(message.id)) {
      return;
    }
    
    // Tandai sebagai diproses dan catat penggunaan
    this.processedMessageIds.add(message.id);
    this.stepUsages.push({
      messageId: message.id,
      timestamp: new Date().toISOString(),
      usage: message.usage,
      costUSD: this.calculateCost(message.usage)
    });
  }
  
  private calculateCost(usage: any): number {
    // Implementasikan perhitungan harga Anda di sini
    // Ini adalah contoh yang disederhanakan
    const inputCost = usage.input_tokens * 0.00003;
    const outputCost = usage.output_tokens * 0.00015;
    const cacheReadCost = (usage.cache_read_input_tokens || 0) * 0.0000075;
    
    return inputCost + outputCost + cacheReadCost;
  }
}

// Penggunaan
const tracker = new CostTracker();
const { result, stepUsages, totalCost } = await tracker.trackConversation(
  "Analisis dan refaktor kode ini"
);

console.log(`Langkah diproses: ${stepUsages.length}`);
console.log(`Total biaya: $${totalCost.toFixed(4)}`);
```

```python Python
from claude_agent_sdk import query, AssistantMessage, ResultMessage
from datetime import datetime
import asyncio

class CostTracker:
    def __init__(self):
        self.processed_message_ids = set()
        self.step_usages = []

    async def track_conversation(self, prompt):
        result = None

        # Proses pesan saat tiba
        async for message in query(prompt=prompt):
            self.process_message(message)

            # Tangkap pesan hasil akhir
            if isinstance(message, ResultMessage):
                result = message

        return {
            "result": result,
            "step_usages": self.step_usages,
            "total_cost": result.total_cost_usd if result else 0
        }

    def process_message(self, message):
        # Hanya proses pesan asisten dengan penggunaan
        if not isinstance(message, AssistantMessage) or not hasattr(message, 'usage'):
            return

        # Lewati jika sudah memproses ID pesan ini
        message_id = getattr(message, 'id', None)
        if not message_id or message_id in self.processed_message_ids:
            return

        # Tandai sebagai diproses dan catat penggunaan
        self.processed_message_ids.add(message_id)
        self.step_usages.append({
            "message_id": message_id,
            "timestamp": datetime.now().isoformat(),
            "usage": message.usage,
            "cost_usd": self.calculate_cost(message.usage)
        })

    def calculate_cost(self, usage):
        # Implementasikan perhitungan harga Anda
        input_cost = usage.get("input_tokens", 0) * 0.00003
        output_cost = usage.get("output_tokens", 0) * 0.00015
        cache_read_cost = usage.get("cache_read_input_tokens", 0) * 0.0000075

        return input_cost + output_cost + cache_read_cost

# Penggunaan
async def main():
    tracker = CostTracker()
    result = await tracker.track_conversation("Analisis dan refaktor kode ini")

    print(f"Langkah diproses: {len(result['step_usages'])}")
    print(f"Total biaya: ${result['total_cost']:.4f}")

asyncio.run(main())
```

</CodeGroup>

## Menangani Kasus Tepi

### Perbedaan Token Output

Dalam kasus langka, Anda mungkin mengamati nilai `output_tokens` yang berbeda untuk pesan dengan ID yang sama. Ketika ini terjadi:

1. **Gunakan nilai tertinggi** - Pesan terakhir dalam grup biasanya berisi total yang akurat
2. **Verifikasi terhadap total biaya** - `total_cost_usd` dalam pesan hasil adalah otoritatif
3. **Laporkan inkonsistensi** - Ajukan masalah di [repositori GitHub Claude Code](https://github.com/anthropics/claude-code/issues)

### Pelacakan Token Cache

Ketika menggunakan prompt caching, lacak jenis token ini secara terpisah:

```typescript
interface CacheUsage {
  cache_creation_input_tokens: number;
  cache_read_input_tokens: number;
  cache_creation: {
    ephemeral_5m_input_tokens: number;
    ephemeral_1h_input_tokens: number;
  };
}
```

## Praktik Terbaik

1. **Gunakan ID Pesan untuk Deduplikasi**: Selalu lacak ID pesan yang diproses untuk menghindari penagihan ganda
2. **Pantau Pesan Hasil**: Hasil akhir berisi penggunaan kumulatif yang otoritatif
3. **Implementasikan Logging**: Catat semua data penggunaan untuk audit dan debugging
4. **Tangani Kegagalan dengan Baik**: Lacak penggunaan parsial bahkan jika percakapan gagal
5. **Pertimbangkan Streaming**: Untuk respons streaming, akumulasi penggunaan saat pesan tiba

## Referensi Field Penggunaan

Setiap objek penggunaan berisi:

- `input_tokens`: Token input dasar yang diproses
- `output_tokens`: Token yang dihasilkan dalam respons
- `cache_creation_input_tokens`: Token yang digunakan untuk membuat entri cache
- `cache_read_input_tokens`: Token yang dibaca dari cache
- `service_tier`: Tingkat layanan yang digunakan (misalnya, "standard")
- `total_cost_usd`: Total biaya dalam USD (hanya dalam pesan hasil)

## Contoh: Membangun Dashboard Penagihan

Berikut cara mengagregasi data penggunaan untuk dashboard penagihan:

```typescript
class BillingAggregator {
  private userUsage = new Map<string, {
    totalTokens: number;
    totalCost: number;
    conversations: number;
  }>();
  
  async processUserRequest(userId: string, prompt: string) {
    const tracker = new CostTracker();
    const { result, stepUsages, totalCost } = await tracker.trackConversation(prompt);
    
    // Perbarui total pengguna
    const current = this.userUsage.get(userId) || {
      totalTokens: 0,
      totalCost: 0,
      conversations: 0
    };
    
    const totalTokens = stepUsages.reduce((sum, step) => 
      sum + step.usage.input_tokens + step.usage.output_tokens, 0
    );
    
    this.userUsage.set(userId, {
      totalTokens: current.totalTokens + totalTokens,
      totalCost: current.totalCost + totalCost,
      conversations: current.conversations + 1
    });
    
    return result;
  }
  
  getUserBilling(userId: string) {
    return this.userUsage.get(userId) || {
      totalTokens: 0,
      totalCost: 0,
      conversations: 0
    };
  }
}
```

## Dokumentasi Terkait

- [Referensi TypeScript SDK](https://code.claude.com/docs/typescript-sdk-reference) - Dokumentasi API lengkap
- [Ikhtisar SDK](/docs/id/agent-sdk/overview) - Memulai dengan SDK
- [Izin SDK](/docs/id/agent-sdk/permissions) - Mengelola izin tool