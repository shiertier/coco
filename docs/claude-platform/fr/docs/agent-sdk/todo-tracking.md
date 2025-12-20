# Listes de Tâches

Suivez et affichez les tâches à faire en utilisant le SDK Agent Claude pour une gestion organisée des tâches

---

Le suivi des tâches à faire fournit une façon structurée de gérer les tâches et d'afficher les progrès aux utilisateurs. Le SDK Agent Claude inclut une fonctionnalité de tâches à faire intégrée qui aide à organiser les flux de travail complexes et à tenir les utilisateurs informés de la progression des tâches.

### Cycle de Vie des Tâches à Faire

Les tâches à faire suivent un cycle de vie prévisible :
1. **Créées** comme `pending` lorsque les tâches sont identifiées
2. **Activées** à `in_progress` lorsque le travail commence
3. **Terminées** lorsque la tâche se termine avec succès
4. **Supprimées** lorsque toutes les tâches d'un groupe sont terminées

### Quand les Tâches à Faire Sont Utilisées

Le SDK crée automatiquement des tâches à faire pour :
- **Les tâches complexes multi-étapes** nécessitant 3 actions distinctes ou plus
- **Les listes de tâches fournies par l'utilisateur** lorsque plusieurs éléments sont mentionnés
- **Les opérations non triviales** qui bénéficient du suivi des progrès
- **Les demandes explicites** lorsque les utilisateurs demandent une organisation des tâches à faire

## Exemples

### Surveillance des Changements de Tâches à Faire

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Optimise les performances de mon app React et suit les progrès avec des tâches à faire",
  options: { maxTurns: 15 }
})) {
  // Les mises à jour des tâches à faire sont reflétées dans le flux de messages
  if (message.type === "tool_use" && message.name === "TodoWrite") {
    const todos = message.input.todos;
    
    console.log("Mise à Jour du Statut des Tâches à Faire :");
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
    prompt="Optimise les performances de mon app React et suit les progrès avec des tâches à faire",
    options={"max_turns": 15}
):
    # Les mises à jour des tâches à faire sont reflétées dans le flux de messages
    if message.get("type") == "tool_use" and message.get("name") == "TodoWrite":
        todos = message["input"]["todos"]
        
        print("Mise à Jour du Statut des Tâches à Faire :")
        for i, todo in enumerate(todos):
            status = "✅" if todo["status"] == "completed" else \
                    "🔧" if todo["status"] == "in_progress" else "❌"
            print(f"{i + 1}. {status} {todo['content']}")
```

</CodeGroup>

### Affichage des Progrès en Temps Réel

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
    
    console.log(`\nProgrès : ${completed}/${total} terminées`);
    console.log(`Travaille actuellement sur : ${inProgress} tâche(s)\n`);
    
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

// Utilisation
const tracker = new TodoTracker();
await tracker.trackQuery("Construis un système d'authentification complet avec des tâches à faire");
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
        
        print(f"\nProgrès : {completed}/{total} terminées")
        print(f"Travaille actuellement sur : {in_progress} tâche(s)\n")
        
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

# Utilisation
tracker = TodoTracker()
await tracker.track_query("Construis un système d'authentification complet avec des tâches à faire")
```

</CodeGroup>

## Documentation Connexe

- [Référence SDK TypeScript](/docs/fr/agent-sdk/typescript)
- [Référence SDK Python](/docs/fr/agent-sdk/python) 
- [Mode Streaming vs Mode Unique](/docs/fr/agent-sdk/streaming-vs-single-mode)
- [Outils Personnalisés](/docs/fr/agent-sdk/custom-tools)