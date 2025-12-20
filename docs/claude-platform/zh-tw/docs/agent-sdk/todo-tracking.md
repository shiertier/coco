# 待辦事項清單

使用 Claude Agent SDK 追蹤和顯示待辦事項，實現有組織的任務管理

---

待辦事項追蹤提供了一種結構化的方式來管理任務並向用戶顯示進度。Claude Agent SDK 包含內建的待辦事項功能，有助於組織複雜的工作流程並讓用戶了解任務進展。

### 待辦事項生命週期

待辦事項遵循可預測的生命週期：
1. **建立**為 `pending` 狀態，當任務被識別時
2. **啟動**為 `in_progress` 狀態，當工作開始時
3. **完成**當任務成功完成時
4. **移除**當群組中的所有任務都完成時

### 何時使用待辦事項

SDK 會自動為以下情況建立待辦事項：
- **複雜的多步驟任務**需要 3 個或更多不同的操作
- **用戶提供的任務清單**當提到多個項目時
- **非平凡的操作**受益於進度追蹤
- **明確請求**當用戶要求待辦事項組織時

## 範例

### 監控待辦事項變更

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "優化我的 React 應用程式效能並使用待辦事項追蹤進度",
  options: { maxTurns: 15 }
})) {
  // 待辦事項更新會反映在訊息串流中
  if (message.type === "tool_use" && message.name === "TodoWrite") {
    const todos = message.input.todos;
    
    console.log("待辦事項狀態更新：");
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
    prompt="優化我的 React 應用程式效能並使用待辦事項追蹤進度",
    options={"max_turns": 15}
):
    # 待辦事項更新會反映在訊息串流中
    if message.get("type") == "tool_use" and message.get("name") == "TodoWrite":
        todos = message["input"]["todos"]
        
        print("待辦事項狀態更新：")
        for i, todo in enumerate(todos):
            status = "✅" if todo["status"] == "completed" else \
                    "🔧" if todo["status"] == "in_progress" else "❌"
            print(f"{i + 1}. {status} {todo['content']}")
```

</CodeGroup>

### 即時進度顯示

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
    
    console.log(`\n進度：${completed}/${total} 已完成`);
    console.log(`目前正在處理：${inProgress} 個任務\n`);
    
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

// 使用方式
const tracker = new TodoTracker();
await tracker.trackQuery("建立完整的身份驗證系統並使用待辦事項");
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
        
        print(f"\n進度：{completed}/{total} 已完成")
        print(f"目前正在處理：{in_progress} 個任務\n")
        
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

# 使用方式
tracker = TodoTracker()
await tracker.track_query("建立完整的身份驗證系統並使用待辦事項")
```

</CodeGroup>

## 相關文件

- [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript)
- [Python SDK 參考](/docs/zh-TW/agent-sdk/python) 
- [串流模式與單次模式](/docs/zh-TW/agent-sdk/streaming-vs-single-mode)
- [自訂工具](/docs/zh-TW/agent-sdk/custom-tools)