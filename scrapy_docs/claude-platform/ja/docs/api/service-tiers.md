# サービスティア

異なるサービスティアにより、アプリケーションのニーズに基づいて、可用性、パフォーマンス、および予測可能なコストのバランスを取ることができます。

---

3つのサービスティアを提供しています：
- **Priority Tier（優先度ティア）:** 本番環境にデプロイされたワークフローに最適で、時間、可用性、および予測可能な価格設定が重要な場合に使用します
- **Standard（標準）:** パイロットと日常的なユースケースのスケーリングの両方のデフォルトティア
- **Batch（バッチ）:** 非同期ワークフローに最適で、待機できるか、通常の容量外で実行されることから利益を得られる場合に使用します

## Standard Tier（標準ティア）

標準ティアは、すべてのAPIリクエストのデフォルトサービスティアです。このティアのリクエストは、他のすべてのリクエストと一緒に優先順位付けされ、ベストエフォート型の可用性を観察します。

## Priority Tier（優先度ティア）

このティアのリクエストは、Anthropicへの他のすべてのリクエストよりも優先されます。この優先順位付けにより、ピーク時でも["サーバーオーバーロード"エラー](/docs/ja/api/errors#http-errors)を最小化できます。

詳細については、[Priority Tierの開始](#get-started-with-priority-tier)を参照してください。

## リクエストがティアに割り当てられる方法

リクエストを処理する際、Anthropicは以下のシナリオでリクエストをPriority Tierに割り当てることを決定します：
- 組織が十分なPriority Tierの容量を持っている **input** トークン/分
- 組織が十分なPriority Tierの容量を持っている **output** トークン/分

Anthropicは、Priority Tierの容量に対する使用量を以下のようにカウントします：

**入力トークン**
- キャッシュ読み取りは、キャッシュから読み取られたトークンあたり0.1トークン
- キャッシュ書き込みは、5分のTTLでキャッシュに書き込まれたトークンあたり1.25トークン
- キャッシュ書き込みは、1時間のTTLでキャッシュに書き込まれたトークンあたり2.00トークン
- [長いコンテキスト](/docs/ja/build-with-claude/context-windows)（>200k入力トークン）リクエストの場合、入力トークンはトークンあたり2トークン
- その他のすべての入力トークンはトークンあたり1トークン

**出力トークン**
- [長いコンテキスト](/docs/ja/build-with-claude/context-windows)（>200k入力トークン）リクエストの場合、出力トークンはトークンあたり1.5トークン
- その他のすべての出力トークンはトークンあたり1トークン

それ以外の場合、リクエストは標準ティアで進行します。

<Note>
Priority Tierに割り当てられたリクエストは、Priority Tierの容量と通常のレート制限の両方から引き出されます。
リクエストを処理するとレート制限を超える場合、リクエストは拒否されます。
</Note>

## サービスティアの使用

`service_tier`パラメータを設定することで、リクエストに使用できるサービスティアを制御できます：

```python
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello, Claude!"}],
    service_tier="auto"  # 利用可能な場合はPriority Tierを自動的に使用し、そうでない場合は標準にフォールバック
)
```

`service_tier`パラメータは以下の値を受け入れます：

- `"auto"`（デフォルト）- Priority Tierの容量が利用可能な場合はそれを使用し、利用できない場合は他の容量にフォールバック
- `"standard_only"` - 標準ティアの容量のみを使用します。Priority Tierの容量を使用したくない場合に便利です

レスポンスの`usage`オブジェクトには、リクエストに割り当てられたサービスティアも含まれます：

```json
{
  "usage": {
    "input_tokens": 410,
    "cache_creation_input_tokens": 0,
    "cache_read_input_tokens": 0,
    "output_tokens": 585,
    "service_tier": "priority"
  }
}
```
これにより、リクエストに割り当てられたサービスティアを判断できます。

Priority Tierコミットメント付きのモデルで`service_tier="auto"`をリクエストする場合、これらのレスポンスヘッダーは洞察を提供します：
```
anthropic-priority-input-tokens-limit: 10000
anthropic-priority-input-tokens-remaining: 9618
anthropic-priority-input-tokens-reset: 2025-01-12T23:11:59Z
anthropic-priority-output-tokens-limit: 10000
anthropic-priority-output-tokens-remaining: 6000
anthropic-priority-output-tokens-reset: 2025-01-12T23:12:21Z
```
これらのヘッダーの存在を使用して、リクエストが制限を超えていた場合でも、Priority Tierの対象であったかどうかを検出できます。

## Priority Tierの開始

以下に関心がある場合は、Priority Tierの容量にコミットすることを検討してください：
- **より高い可用性**: 優先度の高い計算リソースで99.5%のアップタイムを目標
- **コスト管理**: 予測可能な支出と長期コミットメントの割引
- **柔軟なオーバーフロー**: コミットされた容量を超えた場合、自動的に標準ティアにフォールバック

Priority Tierへのコミットには、以下を決定することが含まれます：
- 1分あたりの入力トークン数
- 1分あたりの出力トークン数
- コミットメント期間（1、3、6、または12か月）
- 特定のモデルバージョン

<Note>
購入する入力トークンと出力トークンの比率が重要です。Priority Tierの容量を実際のトラフィックパターンに合わせてサイズ設定することで、購入したトークンの利用率を最大化できます。
</Note>

### サポートされているモデル

Priority Tierは以下でサポートされています：

- Claude Opus 4.5
- Claude Sonnet 4.5
- Claude Haiku 4.5
- Claude Opus 4.1
- Claude Opus 4
- Claude Sonnet 4
- Claude Sonnet 3.7 ([非推奨](/docs/ja/about-claude/model-deprecations))
- Claude Haiku 3.5 ([非推奨](/docs/ja/about-claude/model-deprecations))

モデルの詳細については、[モデル概要ページ](/docs/ja/about-claude/models/overview)を確認してください。

### Priority Tierへのアクセス方法

Priority Tierの使用を開始するには：

1. [営業に連絡](https://claude.com/contact-sales/priority-tier)してプロビジョニングを完了します
2. （オプション）APIリクエストを更新して、`service_tier`パラメータを`auto`に設定します
3. レスポンスヘッダーとClaude Consoleを通じて使用状況を監視します