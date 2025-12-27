# コストと使用量の追跡

Claude Agent SDKでの請求のためのトークン使用量の理解と追跡

---

# SDKコスト追跡

Claude Agent SDKは、Claudeとの各インタラクションについて詳細なトークン使用量情報を提供します。このガイドでは、特に並列ツール使用や複数ステップの会話を扱う際に、コストを適切に追跡し、使用量レポートを理解する方法について説明します。

完全なAPIドキュメントについては、[TypeScript SDKリファレンス](https://code.claude.com/docs/typescript-sdk-reference)を参照してください。

## トークン使用量の理解

Claudeがリクエストを処理する際、メッセージレベルでトークン使用量を報告します。この使用量データは、コストを追跡し、ユーザーに適切に請求するために不可欠です。

### 主要概念

1. **ステップ**: アプリケーションとClaude間の単一のリクエスト/レスポンスペア
2. **メッセージ**: ステップ内の個別メッセージ（テキスト、ツール使用、ツール結果）
3. **使用量**: アシスタントメッセージに添付されるトークン消費データ

## 使用量レポート構造

### 単一対並列ツール使用

Claudeがツールを実行する際、ツールが順次実行されるか並列実行されるかに基づいて、使用量レポートが異なります：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 例: 会話での使用量追跡
const result = await query({
  prompt: "このコードベースを分析してテストを実行してください",
  options: {
    onMessage: (message) => {
      if (message.type === 'assistant' && message.usage) {
        console.log(`メッセージID: ${message.id}`);
        console.log(`使用量:`, message.usage);
      }
    }
  }
});
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage
import asyncio

# 例: 会話での使用量追跡
async def track_usage():
    # メッセージが到着するたびに処理
    async for message in query(
        prompt="このコードベースを分析してテストを実行してください"
    ):
        if isinstance(message, AssistantMessage) and hasattr(message, 'usage'):
            print(f"メッセージID: {message.id}")
            print(f"使用量: {message.usage}")

asyncio.run(track_usage())
```

</CodeGroup>

### メッセージフローの例

典型的な複数ステップ会話でのメッセージと使用量の報告方法は以下の通りです：

```
<!-- ステップ1: 並列ツール使用を伴う初期リクエスト -->
assistant (text)      { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
user (tool_result)
user (tool_result)
user (tool_result)

<!-- ステップ2: フォローアップレスポンス -->
assistant (text)      { id: "msg_2", usage: { output_tokens: 98, ... } }
```

## 重要な使用量ルール

### 1. 同じID = 同じ使用量

**同じ`id`フィールドを持つすべてのメッセージは同一の使用量を報告します**。Claudeが同じターンで複数のメッセージを送信する場合（例：テキスト + ツール使用）、それらは同じメッセージIDと使用量データを共有します。

```typescript
// これらのメッセージはすべて同じIDと使用量を持ちます
const messages = [
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } }
];

// 一意のメッセージIDごとに一度だけ課金
const uniqueUsage = messages[0].usage; // このIDを持つすべてのメッセージで同じ
```

### 2. ステップごとに一度課金

**ユーザーにはステップごとに一度だけ課金すべきです**、個別のメッセージごとではありません。同じIDを持つ複数のアシスタントメッセージを見た場合、そのうちの任意の一つから使用量を使用してください。

### 3. 結果メッセージには累積使用量が含まれる

最終的な`result`メッセージには、会話のすべてのステップからの累積使用量の合計が含まれます：

```typescript
// 最終結果には合計使用量が含まれます
const result = await query({
  prompt: "複数ステップのタスク",
  options: { /* ... */ }
});

console.log("合計使用量:", result.usage);
console.log("合計コスト:", result.usage.total_cost_usd);
```

## 実装: コスト追跡システム

コスト追跡システムの実装の完全な例は以下の通りです：

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
    // 使用量を持つアシスタントメッセージのみを処理
    if (message.type !== 'assistant' || !message.usage) {
      return;
    }
    
    // このメッセージIDを既に処理している場合はスキップ
    if (this.processedMessageIds.has(message.id)) {
      return;
    }
    
    // 処理済みとしてマークし、使用量を記録
    this.processedMessageIds.add(message.id);
    this.stepUsages.push({
      messageId: message.id,
      timestamp: new Date().toISOString(),
      usage: message.usage,
      costUSD: this.calculateCost(message.usage)
    });
  }
  
  private calculateCost(usage: any): number {
    // ここで価格計算を実装
    // これは簡略化された例です
    const inputCost = usage.input_tokens * 0.00003;
    const outputCost = usage.output_tokens * 0.00015;
    const cacheReadCost = (usage.cache_read_input_tokens || 0) * 0.0000075;
    
    return inputCost + outputCost + cacheReadCost;
  }
}

// 使用方法
const tracker = new CostTracker();
const { result, stepUsages, totalCost } = await tracker.trackConversation(
  "このコードを分析してリファクタリングしてください"
);

console.log(`処理されたステップ: ${stepUsages.length}`);
console.log(`合計コスト: $${totalCost.toFixed(4)}`);
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

        # メッセージが到着するたびに処理
        async for message in query(prompt=prompt):
            self.process_message(message)

            # 最終結果メッセージをキャプチャ
            if isinstance(message, ResultMessage):
                result = message

        return {
            "result": result,
            "step_usages": self.step_usages,
            "total_cost": result.total_cost_usd if result else 0
        }

    def process_message(self, message):
        # 使用量を持つアシスタントメッセージのみを処理
        if not isinstance(message, AssistantMessage) or not hasattr(message, 'usage'):
            return

        # このメッセージIDを既に処理している場合はスキップ
        message_id = getattr(message, 'id', None)
        if not message_id or message_id in self.processed_message_ids:
            return

        # 処理済みとしてマークし、使用量を記録
        self.processed_message_ids.add(message_id)
        self.step_usages.append({
            "message_id": message_id,
            "timestamp": datetime.now().isoformat(),
            "usage": message.usage,
            "cost_usd": self.calculate_cost(message.usage)
        })

    def calculate_cost(self, usage):
        # 価格計算を実装
        input_cost = usage.get("input_tokens", 0) * 0.00003
        output_cost = usage.get("output_tokens", 0) * 0.00015
        cache_read_cost = usage.get("cache_read_input_tokens", 0) * 0.0000075

        return input_cost + output_cost + cache_read_cost

# 使用方法
async def main():
    tracker = CostTracker()
    result = await tracker.track_conversation("このコードを分析してリファクタリングしてください")

    print(f"処理されたステップ: {len(result['step_usages'])}")
    print(f"合計コスト: ${result['total_cost']:.4f}")

asyncio.run(main())
```

</CodeGroup>

## エッジケースの処理

### 出力トークンの不一致

まれに、同じIDを持つメッセージで異なる`output_tokens`値を観察する場合があります。これが発生した場合：

1. **最高値を使用** - グループ内の最後のメッセージが通常正確な合計を含んでいます
2. **合計コストと照合** - 結果メッセージの`total_cost_usd`が権威あるものです
3. **不一致を報告** - [Claude Code GitHubリポジトリ](https://github.com/anthropics/claude-code/issues)で問題を報告してください

### キャッシュトークンの追跡

プロンプトキャッシュを使用する場合、これらのトークンタイプを別々に追跡してください：

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

## ベストプラクティス

1. **重複排除にメッセージIDを使用**: 二重課金を避けるため、常に処理済みメッセージIDを追跡する
2. **結果メッセージを監視**: 最終結果には権威ある累積使用量が含まれる
3. **ログを実装**: 監査とデバッグのためにすべての使用量データをログに記録する
4. **失敗を適切に処理**: 会話が失敗しても部分的な使用量を追跡する
5. **ストリーミングを考慮**: ストリーミングレスポンスでは、メッセージが到着するたびに使用量を蓄積する

## 使用量フィールドリファレンス

各使用量オブジェクトには以下が含まれます：

- `input_tokens`: 処理されたベース入力トークン
- `output_tokens`: レスポンスで生成されたトークン
- `cache_creation_input_tokens`: キャッシュエントリの作成に使用されたトークン
- `cache_read_input_tokens`: キャッシュから読み取られたトークン
- `service_tier`: 使用されたサービス階層（例："standard"）
- `total_cost_usd`: USD単位の合計コスト（結果メッセージのみ）

## 例: 請求ダッシュボードの構築

請求ダッシュボード用の使用量データを集約する方法は以下の通りです：

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
    
    // ユーザー合計を更新
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

## 関連ドキュメント

- [TypeScript SDKリファレンス](https://code.claude.com/docs/typescript-sdk-reference) - 完全なAPIドキュメント
- [SDK概要](/docs/ja/agent-sdk/overview) - SDKの開始方法
- [SDK権限](/docs/ja/agent-sdk/permissions) - ツール権限の管理