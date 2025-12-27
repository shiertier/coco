# Daftar Todo

Lacak dan tampilkan todo menggunakan Claude Agent SDK untuk manajemen tugas yang terorganisir

---

Pelacakan todo menyediakan cara terstruktur untuk mengelola tugas dan menampilkan kemajuan kepada pengguna. Claude Agent SDK menyertakan fungsionalitas todo bawaan yang membantu mengorganisir alur kerja yang kompleks dan menjaga pengguna tetap terinformasi tentang perkembangan tugas.

### Siklus Hidup Todo

Todo mengikuti siklus hidup yang dapat diprediksi:
1. **Dibuat** sebagai `pending` ketika tugas diidentifikasi
2. **Diaktifkan** menjadi `in_progress` ketika pekerjaan dimulai
3. **Diselesaikan** ketika tugas berhasil selesai
4. **Dihapus** ketika semua tugas dalam grup telah selesai

### Kapan Todo Digunakan

SDK secara otomatis membuat todo untuk:
- **Tugas multi-langkah yang kompleks** yang memerlukan 3 atau lebih tindakan berbeda
- **Daftar tugas yang disediakan pengguna** ketika beberapa item disebutkan
- **Operasi non-trivial** yang mendapat manfaat dari pelacakan kemajuan
- **Permintaan eksplisit** ketika pengguna meminta organisasi todo

## Contoh

### Memantau Perubahan Todo

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Optimalkan performa aplikasi React saya dan lacak kemajuan dengan todo",
  options: { maxTurns: 15 }
})) {
  // Pembaruan todo tercermin dalam aliran pesan
  if (message.type === "tool_use" && message.name === "TodoWrite") {
    const todos = message.input.todos;
    
    console.log("Pembaruan Status Todo:");
    todos.forEach((todo, index) => {
      const status = todo.status === "completed" ? "✅" : 
                    todo.status === "in_progress" ? "🔧" : "❌";
      console.log(`${index + 1}. ${status} ${todo.content}`);
    });
  }
}
```

```python Python
from claude_agent_sdk import query

async for message in query(
    prompt="Optimalkan performa aplikasi React saya dan lacak kemajuan dengan todo",
    options={"max_turns": 15}
):
    # Pembaruan todo tercermin dalam aliran pesan
    if message.get("type") == "tool_use" and message.get("name") == "TodoWrite":
        todos = message["input"]["todos"]
        
        print("Pembaruan Status Todo:")
        for i, todo in enumerate(todos):
            status = "✅" if todo["status"] == "completed" else \
                    "🔧" if todo["status"] == "in_progress" else "❌"
            print(f"{i + 1}. {status} {todo['content']}")
```

</CodeGroup>

### Tampilan Kemajuan Real-time

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

class TodoTracker {
  private todos: any[] = [];
  
  displayProgress() {
    if (this.todos.length === 0) return;
    
    const completed = this.todos.filter(t => t.status === "completed").length;
    const inProgress = this.todos.filter(t => t.status === "in_progress").length;
    const total = this.todos.length;
    
    console.log(`\nKemajuan: ${completed}/${total} selesai`);
    console.log(`Sedang mengerjakan: ${inProgress} tugas\n`);
    
    this.todos.forEach((todo, index) => {
      const icon = todo.status === "completed" ? "✅" : 
                  todo.status === "in_progress" ? "🔧" : "❌";
      const text = todo.status === "in_progress" ? todo.activeForm : todo.content;
      console.log(`${index + 1}. ${icon} ${text}`);
    });
  }
  
  async trackQuery(prompt: string) {
    for await (const message of query({
      prompt,
      options: { maxTurns: 20 }
    })) {
      if (message.type === "tool_use" && message.name === "TodoWrite") {
        this.todos = message.input.todos;
        this.displayProgress();
      }
    }
  }
}

// Penggunaan
const tracker = new TodoTracker();
await tracker.trackQuery("Bangun sistem autentikasi lengkap dengan todo");
```

```python Python
from claude_agent_sdk import query
from typing import List, Dict

class TodoTracker:
    def __init__(self):
        self.todos: List[Dict] = []
    
    def display_progress(self):
        if not self.todos:
            return
        
        completed = len([t for t in self.todos if t["status"] == "completed"])
        in_progress = len([t for t in self.todos if t["status"] == "in_progress"])
        total = len(self.todos)
        
        print(f"\nKemajuan: {completed}/{total} selesai")
        print(f"Sedang mengerjakan: {in_progress} tugas\n")
        
        for i, todo in enumerate(self.todos):
            icon = "✅" if todo["status"] == "completed" else \
                  "🔧" if todo["status"] == "in_progress" else "❌"
            text = todo["activeForm"] if todo["status"] == "in_progress" else todo["content"]
            print(f"{i + 1}. {icon} {text}")
    
    async def track_query(self, prompt: str):
        async for message in query(
            prompt=prompt,
            options={"max_turns": 20}
        ):
            if message.get("type") == "tool_use" and message.get("name") == "TodoWrite":
                self.todos = message["input"]["todos"]
                self.display_progress()

# Penggunaan
tracker = TodoTracker()
await tracker.track_query("Bangun sistem autentikasi lengkap dengan todo")
```

</CodeGroup>

## Dokumentasi Terkait

- [Referensi TypeScript SDK](/docs/id/agent-sdk/typescript)
- [Referensi Python SDK](/docs/id/agent-sdk/python) 
- [Streaming vs Single Mode](/docs/id/agent-sdk/streaming-vs-single-mode)
- [Custom Tools](/docs/id/agent-sdk/custom-tools)