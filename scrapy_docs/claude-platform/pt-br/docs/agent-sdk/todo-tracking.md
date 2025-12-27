# Listas de Tarefas

Rastreie e exiba tarefas usando o Claude Agent SDK para gerenciamento organizado de tarefas

---

O rastreamento de tarefas fornece uma maneira estruturada de gerenciar tarefas e exibir o progresso aos usuários. O Claude Agent SDK inclui funcionalidade de tarefas integrada que ajuda a organizar fluxos de trabalho complexos e manter os usuários informados sobre a progressão das tarefas.

### Ciclo de Vida das Tarefas

As tarefas seguem um ciclo de vida previsível:
1. **Criadas** como `pending` quando as tarefas são identificadas
2. **Ativadas** para `in_progress` quando o trabalho começa
3. **Concluídas** quando a tarefa termina com sucesso
4. **Removidas** quando todas as tarefas em um grupo são concluídas

### Quando as Tarefas São Usadas

O SDK cria automaticamente tarefas para:
- **Tarefas complexas de múltiplas etapas** que requerem 3 ou mais ações distintas
- **Listas de tarefas fornecidas pelo usuário** quando múltiplos itens são mencionados
- **Operações não triviais** que se beneficiam do rastreamento de progresso
- **Solicitações explícitas** quando os usuários pedem organização de tarefas

## Exemplos

### Monitorando Mudanças nas Tarefas

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Otimize o desempenho do meu app React e rastreie o progresso com tarefas",
  options: { maxTurns: 15 }
})) {
  // Atualizações de tarefas são refletidas no fluxo de mensagens
  if (message.type === "tool_use" && message.name === "TodoWrite") {
    const todos = message.input.todos;
    
    console.log("Atualização de Status das Tarefas:");
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
    prompt="Otimize o desempenho do meu app React e rastreie o progresso com tarefas",
    options={"max_turns": 15}
):
    # Atualizações de tarefas são refletidas no fluxo de mensagens
    if message.get("type") == "tool_use" and message.get("name") == "TodoWrite":
        todos = message["input"]["todos"]
        
        print("Atualização de Status das Tarefas:")
        for i, todo in enumerate(todos):
            status = "✅" if todo["status"] == "completed" else \
                    "🔧" if todo["status"] == "in_progress" else "❌"
            print(f"{i + 1}. {status} {todo['content']}")
```

</CodeGroup>

### Exibição de Progresso em Tempo Real

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
    
    console.log(`\nProgresso: ${completed}/${total} concluídas`);
    console.log(`Atualmente trabalhando em: ${inProgress} tarefa(s)\n`);
    
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

// Uso
const tracker = new TodoTracker();
await tracker.trackQuery("Construa um sistema de autenticação completo com tarefas");
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
        
        print(f"\nProgresso: {completed}/{total} concluídas")
        print(f"Atualmente trabalhando em: {in_progress} tarefa(s)\n")
        
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

# Uso
tracker = TodoTracker()
await tracker.track_query("Construa um sistema de autenticação completo com tarefas")
```

</CodeGroup>

## Documentação Relacionada

- [Referência do SDK TypeScript](/docs/pt-BR/agent-sdk/typescript)
- [Referência do SDK Python](/docs/pt-BR/agent-sdk/python) 
- [Modo Streaming vs Único](/docs/pt-BR/agent-sdk/streaming-vs-single-mode)
- [Ferramentas Personalizadas](/docs/pt-BR/agent-sdk/custom-tools)