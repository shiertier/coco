# Listas de Tareas

Rastrea y muestra tareas pendientes usando el SDK de Agente Claude para una gestión organizada de tareas

---

El seguimiento de tareas pendientes proporciona una forma estructurada de gestionar tareas y mostrar el progreso a los usuarios. El SDK de Agente Claude incluye funcionalidad de tareas pendientes integrada que ayuda a organizar flujos de trabajo complejos y mantener a los usuarios informados sobre la progresión de tareas.

### Ciclo de Vida de las Tareas Pendientes

Las tareas pendientes siguen un ciclo de vida predecible:
1. **Creadas** como `pending` cuando se identifican las tareas
2. **Activadas** a `in_progress` cuando comienza el trabajo
3. **Completadas** cuando la tarea termina exitosamente
4. **Eliminadas** cuando todas las tareas en un grupo están completadas

### Cuándo se Usan las Tareas Pendientes

El SDK crea automáticamente tareas pendientes para:
- **Tareas complejas de múltiples pasos** que requieren 3 o más acciones distintas
- **Listas de tareas proporcionadas por el usuario** cuando se mencionan múltiples elementos
- **Operaciones no triviales** que se benefician del seguimiento del progreso
- **Solicitudes explícitas** cuando los usuarios piden organización de tareas pendientes

## Ejemplos

### Monitoreo de Cambios en Tareas Pendientes

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Optimiza el rendimiento de mi aplicación React y rastrea el progreso con tareas pendientes",
  options: { maxTurns: 15 }
})) {
  // Las actualizaciones de tareas pendientes se reflejan en el flujo de mensajes
  if (message.type === "tool_use" && message.name === "TodoWrite") {
    const todos = message.input.todos;
    
    console.log("Actualización de Estado de Tareas Pendientes:");
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
    prompt="Optimiza el rendimiento de mi aplicación React y rastrea el progreso con tareas pendientes",
    options={"max_turns": 15}
):
    # Las actualizaciones de tareas pendientes se reflejan en el flujo de mensajes
    if message.get("type") == "tool_use" and message.get("name") == "TodoWrite":
        todos = message["input"]["todos"]
        
        print("Actualización de Estado de Tareas Pendientes:")
        for i, todo in enumerate(todos):
            status = "✅" if todo["status"] == "completed" else \
                    "🔧" if todo["status"] == "in_progress" else "❌"
            print(f"{i + 1}. {status} {todo['content']}")
```

</CodeGroup>

### Visualización de Progreso en Tiempo Real

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
    
    console.log(`\nProgreso: ${completed}/${total} completadas`);
    console.log(`Trabajando actualmente en: ${inProgress} tarea(s)\n`);
    
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
await tracker.trackQuery("Construye un sistema de autenticación completo con tareas pendientes");
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
        
        print(f"\nProgreso: {completed}/{total} completadas")
        print(f"Trabajando actualmente en: {in_progress} tarea(s)\n")
        
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
await tracker.track_query("Construye un sistema de autenticación completo con tareas pendientes")
```

</CodeGroup>

## Documentación Relacionada

- [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript)
- [Referencia del SDK de Python](/docs/es/agent-sdk/python) 
- [Modo de Transmisión vs Modo Único](/docs/es/agent-sdk/streaming-vs-single-mode)
- [Herramientas Personalizadas](/docs/es/agent-sdk/custom-tools)