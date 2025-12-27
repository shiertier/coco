# Liste Todo

Traccia e visualizza i todo utilizzando il Claude Agent SDK per una gestione organizzata delle attività

---

Il tracciamento dei todo fornisce un modo strutturato per gestire le attività e mostrare i progressi agli utenti. Il Claude Agent SDK include funzionalità todo integrate che aiutano a organizzare flussi di lavoro complessi e mantenere gli utenti informati sulla progressione delle attività.

### Ciclo di Vita dei Todo

I todo seguono un ciclo di vita prevedibile:
1. **Creati** come `pending` quando le attività vengono identificate
2. **Attivati** a `in_progress` quando inizia il lavoro
3. **Completati** quando l'attività finisce con successo
4. **Rimossi** quando tutte le attività in un gruppo sono completate

### Quando Vengono Utilizzati i Todo

L'SDK crea automaticamente todo per:
- **Attività complesse multi-step** che richiedono 3 o più azioni distinte
- **Liste di attività fornite dall'utente** quando vengono menzionati più elementi
- **Operazioni non banali** che beneficiano del tracciamento dei progressi
- **Richieste esplicite** quando gli utenti chiedono l'organizzazione dei todo

## Esempi

### Monitoraggio dei Cambiamenti dei Todo

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Ottimizza le prestazioni della mia app React e traccia i progressi con i todo",
  options: { maxTurns: 15 }
})) {
  // Gli aggiornamenti dei todo si riflettono nel flusso dei messaggi
  if (message.type === "tool_use" && message.name === "TodoWrite") {
    const todos = message.input.todos;
    
    console.log("Aggiornamento Stato Todo:");
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
    prompt="Ottimizza le prestazioni della mia app React e traccia i progressi con i todo",
    options={"max_turns": 15}
):
    # Gli aggiornamenti dei todo si riflettono nel flusso dei messaggi
    if message.get("type") == "tool_use" and message.get("name") == "TodoWrite":
        todos = message["input"]["todos"]
        
        print("Aggiornamento Stato Todo:")
        for i, todo in enumerate(todos):
            status = "✅" if todo["status"] == "completed" else \
                    "🔧" if todo["status"] == "in_progress" else "❌"
            print(f"{i + 1}. {status} {todo['content']}")
```

</CodeGroup>

### Visualizzazione dei Progressi in Tempo Reale

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
    
    console.log(`\nProgressi: ${completed}/${total} completati`);
    console.log(`Attualmente lavorando su: ${inProgress} attività\n`);
    
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

// Utilizzo
const tracker = new TodoTracker();
await tracker.trackQuery("Costruisci un sistema di autenticazione completo con i todo");
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
        
        print(f"\nProgressi: {completed}/{total} completati")
        print(f"Attualmente lavorando su: {in_progress} attività\n")
        
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

# Utilizzo
tracker = TodoTracker()
await tracker.track_query("Costruisci un sistema di autenticazione completo con i todo")
```

</CodeGroup>

## Documentazione Correlata

- [Riferimento SDK TypeScript](/docs/it/agent-sdk/typescript)
- [Riferimento SDK Python](/docs/it/agent-sdk/python) 
- [Modalità Streaming vs Singola](/docs/it/agent-sdk/streaming-vs-single-mode)
- [Strumenti Personalizzati](/docs/it/agent-sdk/custom-tools)