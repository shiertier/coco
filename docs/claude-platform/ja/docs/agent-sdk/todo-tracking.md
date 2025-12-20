# Todo リスト

整理されたタスク管理のためにClaude Agent SDKを使用してtodoを追跡・表示する

---

Todo追跡は、タスクを管理し、ユーザーに進捗を表示するための構造化された方法を提供します。Claude Agent SDKには、複雑なワークフローを整理し、タスクの進行状況についてユーザーに情報を提供するのに役立つ組み込みのtodo機能が含まれています。

### Todoのライフサイクル

Todoは予測可能なライフサイクルに従います：
1. タスクが特定されたときに`pending`として**作成**される
2. 作業が開始されたときに`in_progress`に**アクティブ化**される
3. タスクが正常に完了したときに**完了**する
4. グループ内のすべてのタスクが完了したときに**削除**される

### Todoが使用される場合

SDKは以下の場合に自動的にtodoを作成します：
- 3つ以上の異なるアクションを必要とする**複雑な複数ステップのタスク**
- 複数の項目が言及されている**ユーザー提供のタスクリスト**
- 進捗追跡の恩恵を受ける**重要な操作**
- ユーザーがtodo整理を求める**明示的な要求**

## 例

### Todo変更の監視

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "React アプリのパフォーマンスを最適化し、todoで進捗を追跡する",
  options: { maxTurns: 15 }
})) {
  // Todo更新はメッセージストリームに反映されます
  if (message.type === "tool_use" && message.name === "TodoWrite") {
    const todos = message.input.todos;
    
    console.log("Todoステータス更新:");
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
    prompt="React アプリのパフォーマンスを最適化し、todoで進捗を追跡する",
    options={"max_turns": 15}
):
    # Todo更新はメッセージストリームに反映されます
    if message.get("type") == "tool_use" and message.get("name") == "TodoWrite":
        todos = message["input"]["todos"]
        
        print("Todoステータス更新:")
        for i, todo in enumerate(todos):
            status = "✅" if todo["status"] == "completed" else \
                    "🔧" if todo["status"] == "in_progress" else "❌"
            print(f"{i + 1}. {status} {todo['content']}")
```

</CodeGroup>

### リアルタイム進捗表示

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
    
    console.log(`\n進捗: ${completed}/${total} 完了`);
    console.log(`現在作業中: ${inProgress} タスク\n`);
    
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

// 使用方法
const tracker = new TodoTracker();
await tracker.trackQuery("todoを使用して完全な認証システムを構築する");
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
        
        print(f"\n進捗: {completed}/{total} 完了")
        print(f"現在作業中: {in_progress} タスク\n")
        
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

# 使用方法
tracker = TodoTracker()
await tracker.track_query("todoを使用して完全な認証システムを構築する")
```

</CodeGroup>

## 関連ドキュメント

- [TypeScript SDK リファレンス](/docs/ja/agent-sdk/typescript)
- [Python SDK リファレンス](/docs/ja/agent-sdk/python) 
- [ストリーミング vs シングルモード](/docs/ja/agent-sdk/streaming-vs-single-mode)
- [カスタムツール](/docs/ja/agent-sdk/custom-tools)