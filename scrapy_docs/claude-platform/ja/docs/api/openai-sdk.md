# OpenAI SDK互換性

Anthropicは、OpenAI SDKを使用してClaude APIをテストできる互換性レイヤーを提供しています。わずかなコード変更で、Anthropicモデルの機能を迅速に評価できます。

---

<Note>
この互換性レイヤーは、主にモデル機能のテストと比較を目的としており、ほとんどのユースケースでは長期的または本番環境対応のソリューションとは見なされていません。完全に機能を保つことを意図しており、破壊的な変更は行わない予定ですが、私たちの優先事項は[Claude API](/docs/ja/api/overview)の信頼性と有効性です。

既知の互換性の制限事項の詳細については、[重要なOpenAI互換性の制限事項](#important-openai-compatibility-limitations)を参照してください。

OpenAI SDK互換性機能に関する問題が発生した場合は、[こちら](https://forms.gle/oQV4McQNiuuNbz9n8)でお知らせください。
</Note>

<Tip>
最高のエクスペリエンスとClaude APIの完全な機能セット（[PDF処理](/docs/ja/build-with-claude/pdf-support)、[引用](/docs/ja/build-with-claude/citations)、[拡張思考](/docs/ja/build-with-claude/extended-thinking)、[プロンプトキャッシング](/docs/ja/build-with-claude/prompt-caching)）へのアクセスのために、ネイティブな[Claude API](/docs/ja/api/overview)の使用をお勧めします。
</Tip>

## OpenAI SDKの使い始め

OpenAI SDK互換性機能を使用するには、以下が必要です：

1. 公式のOpenAI SDKを使用する
2. 以下を変更する
   * ベースURLをClaude APIを指すように更新する
   * APIキーを[Claude APIキー](/settings/keys)に置き換える
   * モデル名を[Claudeモデル](/docs/ja/about-claude/models/overview)を使用するように更新する
3. サポートされている機能について、以下のドキュメントを確認する

### クイックスタートの例

<CodeGroup>
    ```python Python
    from openai import OpenAI

    client = OpenAI(
        api_key="ANTHROPIC_API_KEY",  # Your Claude API key
        base_url="https://api.anthropic.com/v1/"  # the Claude API endpoint
    )

    response = client.chat.completions.create(
        model="claude-sonnet-4-5", # Anthropic model name
        messages=[
            {"role": "system", "content": "You are a helpful assistant."},
            {"role": "user", "content": "Who are you?"}
        ],
    )

    print(response.choices[0].message.content)
    ```
    
    ```typescript TypeScript
    import OpenAI from 'openai';

    const openai = new OpenAI({
        apiKey: "ANTHROPIC_API_KEY",   // Your Claude API key
        baseURL: "https://api.anthropic.com/v1/",  // Claude API endpoint
    });

    const response = await openai.chat.completions.create({
        messages: [
            { role: "user", content: "Who are you?" }
        ],
        model: "claude-sonnet-4-5", // Claude model name
    });

    console.log(response.choices[0].message.content);
    ```
</CodeGroup>

## 重要なOpenAI互換性の制限事項

#### API動作

OpenAIの使用との最も実質的な違いは以下の通りです：

* 関数呼び出しの`strict`パラメータは無視されます。つまり、ツール使用JSONは提供されたスキーマに従うことが保証されません。スキーマ準拠を保証するには、ネイティブな[Claude APIと構造化出力](/docs/ja/build-with-claude/structured-outputs)を使用してください。
* オーディオ入力はサポートされていません。単に無視され、入力から削除されます
* プロンプトキャッシングはサポートされていませんが、[Anthropic SDK](/docs/ja/api/client-sdks)ではサポートされています
* システム/開発者メッセージはホイストされ、会話の開始時に連結されます。Anthropicはシングル初期システムメッセージのみをサポートしているためです。

サポートされていないフィールドのほとんどはエラーを生成するのではなく、静かに無視されます。これらはすべて以下に記載されています。

#### 出力品質に関する考慮事項

プロンプトに多くの調整を行った場合、OpenAI用に十分に調整されている可能性があります。[Claude Consoleのプロンプト改善ツール](/dashboard)を使用することを検討してください。これは良い出発点になります。

#### システム/開発者メッセージのホイスト

OpenAI SDKへのほとんどの入力はAnthropicのAPIパラメータに直接マップされていますが、1つの明確な違いはシステム/開発者プロンプトの処理です。これら2つのプロンプトはOpenAIを通じてチャット会話全体に配置できます。Anthropicはシングル初期システムメッセージのみをサポートしているため、すべてのシステム/開発者メッセージを取得し、それらの間に単一の改行（`\n`）を挟んで連結します。この完全な文字列は、メッセージの開始時にシングルシステムメッセージとして提供されます。

#### 拡張思考サポート

`thinking`パラメータを追加することで、[拡張思考](/docs/ja/build-with-claude/extended-thinking)機能を有効にできます。これはClaudeの複雑なタスクの推論を改善しますが、OpenAI SDKはClaudeの詳細な思考プロセスを返しません。Claudeのステップバイステップの推論出力へのアクセスを含む、完全な拡張思考機能については、ネイティブなClaude APIを使用してください。

<CodeGroup>
    ```python Python
    response = client.chat.completions.create(
        model="claude-sonnet-4-5",
        messages=...,
        extra_body={
            "thinking": { "type": "enabled", "budget_tokens": 2000 }
        }
    )
    ```
    
    ```typescript TypeScript
    const response = await openai.chat.completions.create({
        messages: [
            { role: "user", content: "Who are you?" }
        ],
        model: "claude-sonnet-4-5",
        // @ts-expect-error
        thinking: { type: "enabled", budget_tokens: 2000 }
    });

    ```
</CodeGroup>

## レート制限

レート制限は、`/v1/messages`エンドポイントのAnthropicの[標準制限](/docs/ja/api/rate-limits)に従います。

## 詳細なOpenAI互換API サポート
### リクエストフィールド
#### シンプルフィールド
| フィールド | サポート状況 |
|--------|----------------|
| `model` | Claudeモデル名を使用してください |
| `max_tokens` | 完全にサポート |
| `max_completion_tokens` | 完全にサポート |
| `stream` | 完全にサポート |
| `stream_options` | 完全にサポート |
| `top_p` | 完全にサポート |
| `parallel_tool_calls` | 完全にサポート |
| `stop` | すべての非空白ストップシーケンスが機能します |
| `temperature` | 0～1（両端を含む）。1より大きい値は1で上限されます。 |
| `n` | 正確に1である必要があります |
| `logprobs` | 無視 |
| `metadata` | 無視 |
| `response_format` | 無視。JSON出力の場合は、ネイティブClaude APIで[構造化出力](/docs/ja/build-with-claude/structured-outputs)を使用してください |
| `prediction` | 無視 |
| `presence_penalty` | 無視 |
| `frequency_penalty` | 無視 |
| `seed` | 無視 |
| `service_tier` | 無視 |
| `audio` | 無視 |
| `logit_bias` | 無視 |
| `store` | 無視 |
| `user` | 無視 |
| `modalities` | 無視 |
| `top_logprobs` | 無視 |
| `reasoning_effort` | 無視 |

#### `tools` / `functions`フィールド
<section title="フィールドを表示">

<Tabs>
<Tab title="Tools">
`tools[n].function`フィールド
| フィールド        | サポート状況         |
|--------------|-----------------|
| `name`       | 完全にサポート |
| `description`| 完全にサポート |
| `parameters` | 完全にサポート |
| `strict`     | 無視。厳密なスキーマ検証については、ネイティブClaude APIで[構造化出力](/docs/ja/build-with-claude/structured-outputs)を使用してください |
</Tab>
<Tab title="Functions">

`functions[n]`フィールド
<Info>
OpenAIは`functions`フィールドを廃止し、代わりに`tools`を使用することを推奨しています。
</Info>
| フィールド        | サポート状況         |
|--------------|-----------------|
| `name`       | 完全にサポート |
| `description`| 完全にサポート |
| `parameters` | 完全にサポート |
| `strict`     | 無視。厳密なスキーマ検証については、ネイティブClaude APIで[構造化出力](/docs/ja/build-with-claude/structured-outputs)を使用してください |
</Tab>
</Tabs>

</section>

#### `messages`配列フィールド
<section title="フィールドを表示">

<Tabs>
<Tab title="Developer role">
`messages[n].role == "developer"`のフィールド
<Info>
開発者メッセージは会話の開始時に初期システムメッセージの一部としてホイストされます
</Info>
| フィールド | サポート状況 |
|-------|---------|
| `content` | 完全にサポート、ただしホイストされます |
| `name` | 無視 |

</Tab>
<Tab title="System role">
`messages[n].role == "system"`のフィールド

<Info>
システムメッセージは会話の開始時に初期システムメッセージの一部としてホイストされます
</Info>
| フィールド | サポート状況 |
|-------|---------|
| `content` | 完全にサポート、ただしホイストされます |
| `name` | 無視 |

</Tab>
<Tab title="User role">
`messages[n].role == "user"`のフィールド

| フィールド | バリアント | サブフィールド | サポート状況 |
|-------|---------|-----------|----------------|
| `content` | `string` | | 完全にサポート |
| | `array`, `type == "text"` | | 完全にサポート |
| | `array`, `type == "image_url"` | `url` | 完全にサポート |
| | | `detail` | 無視 |
| | `array`, `type == "input_audio"` | | 無視 |
| | `array`, `type == "file"` | | 無視 |
| `name` | | | 無視 |

</Tab>

<Tab title="Assistant role">
`messages[n].role == "assistant"`のフィールド
| フィールド | バリアント | サポート状況 |
|-------|---------|----------------|
| `content` | `string` | 完全にサポート |
| | `array`, `type == "text"` | 完全にサポート |
| | `array`, `type == "refusal"` | 無視 |
| `tool_calls` | | 完全にサポート |
| `function_call` | | 完全にサポート |
| `audio` | | 無視 |
| `refusal` | | 無視 |

</Tab>

<Tab title="Tool role">
`messages[n].role == "tool"`のフィールド
| フィールド | バリアント | サポート状況 |
|-------|---------|----------------|
| `content` | `string` | 完全にサポート |
| | `array`, `type == "text"` | 完全にサポート |
| `tool_call_id` | | 完全にサポート |
| `tool_choice` | | 完全にサポート |
| `name` | | 無視 |
</Tab>

<Tab title="Function role">
`messages[n].role == "function"`のフィールド
| フィールド | バリアント | サポート状況 |
|-------|---------|----------------|
| `content` | `string` | 完全にサポート |
| | `array`, `type == "text"` | 完全にサポート |
| `tool_choice` | | 完全にサポート |
| `name` | | 無視 |
</Tab>
</Tabs>

</section>

### レスポンスフィールド

| フィールド | サポート状況 |
|---------------------------|----------------|
| `id` | 完全にサポート |
| `choices[]` | 常に長さが1になります |
| `choices[].finish_reason` | 完全にサポート |
| `choices[].index` | 完全にサポート |
| `choices[].message.role` | 完全にサポート |
| `choices[].message.content` | 完全にサポート |
| `choices[].message.tool_calls` | 完全にサポート |
| `object` | 完全にサポート |
| `created` | 完全にサポート |
| `model` | 完全にサポート |
| `finish_reason` | 完全にサポート |
| `content` | 完全にサポート |
| `usage.completion_tokens` | 完全にサポート |
| `usage.prompt_tokens` | 完全にサポート |
| `usage.total_tokens` | 完全にサポート |
| `usage.completion_tokens_details` | 常に空です |
| `usage.prompt_tokens_details` | 常に空です |
| `choices[].message.refusal` | 常に空です |
| `choices[].message.audio` | 常に空です |
| `logprobs` | 常に空です |
| `service_tier` | 常に空です |
| `system_fingerprint` | 常に空です |

### エラーメッセージの互換性

互換性レイヤーはOpenAI APIと一貫したエラー形式を保持しています。ただし、詳細なエラーメッセージは同等ではありません。ロギングとデバッグにのみエラーメッセージを使用することをお勧めします。

### ヘッダー互換性

OpenAI SDKは自動的にヘッダーを管理しますが、Claude APIでサポートされているヘッダーの完全なリストは以下の通りです。これは、直接それらを操作する必要がある開発者向けです。

| ヘッダー | サポート状況 |
|---------|----------------|
| `x-ratelimit-limit-requests` | 完全にサポート |
| `x-ratelimit-limit-tokens` | 完全にサポート |
| `x-ratelimit-remaining-requests` | 完全にサポート |
| `x-ratelimit-remaining-tokens` | 完全にサポート |
| `x-ratelimit-reset-requests` | 完全にサポート |
| `x-ratelimit-reset-tokens` | 完全にサポート |
| `retry-after` | 完全にサポート |
| `request-id` | 完全にサポート |
| `openai-version` | 常に`2020-10-01` |
| `authorization` | 完全にサポート |
| `openai-processing-ms` | 常に空です |