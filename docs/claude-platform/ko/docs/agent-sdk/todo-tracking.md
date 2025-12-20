# 할 일 목록

체계적인 작업 관리를 위해 Claude Agent SDK를 사용하여 할 일을 추적하고 표시합니다

---

할 일 추적은 작업을 관리하고 사용자에게 진행 상황을 표시하는 구조화된 방법을 제공합니다. Claude Agent SDK에는 복잡한 워크플로우를 구성하고 사용자에게 작업 진행 상황을 알려주는 데 도움이 되는 내장된 할 일 기능이 포함되어 있습니다.

### 할 일 생명주기

할 일은 예측 가능한 생명주기를 따릅니다:
1. 작업이 식별되면 `pending`으로 **생성됨**
2. 작업이 시작되면 `in_progress`로 **활성화됨**
3. 작업이 성공적으로 완료되면 **완료됨**
4. 그룹의 모든 작업이 완료되면 **제거됨**

### 할 일이 사용되는 경우

SDK는 다음과 같은 경우에 자동으로 할 일을 생성합니다:
- **복잡한 다단계 작업**에서 3개 이상의 별개 작업이 필요한 경우
- **사용자 제공 작업 목록**에서 여러 항목이 언급된 경우
- **중요한 작업**에서 진행 상황 추적이 도움이 되는 경우
- **명시적 요청**에서 사용자가 할 일 구성을 요청하는 경우

## 예제

### 할 일 변경 사항 모니터링

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "React 앱 성능을 최적화하고 할 일로 진행 상황을 추적하세요",
  options: { maxTurns: 15 }
})) {
  // 할 일 업데이트는 메시지 스트림에 반영됩니다
  if (message.type === "tool_use" && message.name === "TodoWrite") {
    const todos = message.input.todos;
    
    console.log("할 일 상태 업데이트:");
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
    prompt="React 앱 성능을 최적화하고 할 일로 진행 상황을 추적하세요",
    options={"max_turns": 15}
):
    # 할 일 업데이트는 메시지 스트림에 반영됩니다
    if message.get("type") == "tool_use" and message.get("name") == "TodoWrite":
        todos = message["input"]["todos"]
        
        print("할 일 상태 업데이트:")
        for i, todo in enumerate(todos):
            status = "✅" if todo["status"] == "completed" else \
                    "🔧" if todo["status"] == "in_progress" else "❌"
            print(f"{i + 1}. {status} {todo['content']}")
```

</CodeGroup>

### 실시간 진행 상황 표시

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
    
    console.log(`\n진행 상황: ${completed}/${total} 완료`);
    console.log(`현재 작업 중: ${inProgress}개 작업\n`);
    
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

// 사용법
const tracker = new TodoTracker();
await tracker.trackQuery("할 일과 함께 완전한 인증 시스템을 구축하세요");
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
        
        print(f"\n진행 상황: {completed}/{total} 완료")
        print(f"현재 작업 중: {in_progress}개 작업\n")
        
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

# 사용법
tracker = TodoTracker()
await tracker.track_query("할 일과 함께 완전한 인증 시스템을 구축하세요")
```

</CodeGroup>

## 관련 문서

- [TypeScript SDK 참조](/docs/ko/agent-sdk/typescript)
- [Python SDK 참조](/docs/ko/agent-sdk/python) 
- [스트리밍 vs 단일 모드](/docs/ko/agent-sdk/streaming-vs-single-mode)
- [사용자 정의 도구](/docs/ko/agent-sdk/custom-tools)