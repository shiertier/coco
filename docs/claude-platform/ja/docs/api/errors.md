# エラー

---

## HTTPエラー

私たちのAPIは予測可能なHTTPエラーコード形式に従います：

* 400 - `invalid_request_error`: リクエストの形式または内容に問題がありました。以下にリストされていない他の4XXステータスコードについても、このエラータイプを使用する場合があります。
* 401 - `authentication_error`: APIキーに問題があります。
* 403 - `permission_error`: APIキーに指定されたリソースを使用する権限がありません。
* 404 - `not_found_error`: 要求されたリソースが見つかりませんでした。
* 413 - `request_too_large`: リクエストが許可される最大バイト数を超えています。標準APIエンドポイントの最大リクエストサイズは32MBです。
* 429 - `rate_limit_error`: アカウントがレート制限に達しました。
* 500 - `api_error`: Anthropicのシステム内部で予期しないエラーが発生しました。
* 529 - `overloaded_error`: APIが一時的に過負荷状態です。

  <Warning>
  529エラーは、すべてのユーザーにわたってAPIが高いトラフィックを経験している場合に発生する可能性があります。
  
  まれなケースですが、組織の使用量が急激に増加した場合、APIの加速制限により429エラーが表示される可能性があります。加速制限に達することを避けるために、トラフィックを段階的に増加させ、一貫した使用パターンを維持してください。
  </Warning>

SSE経由で[ストリーミング](/docs/ja/build-with-claude/streaming)レスポンスを受信する際、200レスポンスを返した後にエラーが発生する可能性があり、その場合エラーハンドリングはこれらの標準的なメカニズムに従いません。

## リクエストサイズ制限

APIは最適なパフォーマンスを確保するためにリクエストサイズ制限を実施しています：

| エンドポイントタイプ | 最大リクエストサイズ |
|:---|:---|
| Messages API | 32 MB |
| Token Counting API | 32 MB |
| [Batch API](/docs/ja/build-with-claude/batch-processing) | 256 MB |
| [Files API](/docs/ja/build-with-claude/files) | 500 MB |

これらの制限を超えると、413 `request_too_large`エラーを受信します。エラーは、リクエストがAPIサーバーに到達する前にCloudflareから返されます。

## エラーの形状

エラーは常にJSONとして返され、常に`type`と`message`の値を含むトップレベルの`error`オブジェクトがあります。レスポンスには、より簡単な追跡とデバッグのための`request_id`フィールドも含まれています。例えば：

```json JSON
{
  "type": "error",
  "error": {
    "type": "not_found_error",
    "message": "The requested resource could not be found."
  },
  "request_id": "req_011CSHoEeqs5C35K2UUqR7Fy"
}
```

私たちの[バージョニング](/docs/ja/api/versioning)ポリシーに従って、これらのオブジェクト内の値を拡張する場合があり、`type`の値が時間の経過とともに増加する可能性があります。

## リクエストID

すべてのAPIレスポンスには一意の`request-id`ヘッダーが含まれています。このヘッダーには`req_018EeWyXxfu5pfWkrYcMdjWG`などの値が含まれています。特定のリクエストについてサポートに連絡する際は、問題を迅速に解決するためにこのIDを含めてください。

私たちの公式SDKは、`request-id`ヘッダーの値を含む、トップレベルレスポンスオブジェクトのプロパティとしてこの値を提供します：

<CodeGroup>
  ```python Python
  import anthropic

  client = anthropic.Anthropic()

  message = client.messages.create(
      model="claude-sonnet-4-5",
      max_tokens=1024,
      messages=[
          {"role": "user", "content": "Hello, Claude"}
      ]
  )
  print(f"Request ID: {message._request_id}")
  ```

  ```typescript TypeScript
  import Anthropic from '@anthropic-ai/sdk';

  const client = new Anthropic();

  const message = await client.messages.create({
    model: 'claude-sonnet-4-5',
    max_tokens: 1024,
    messages: [
      {"role": "user", "content": "Hello, Claude"}
    ]
  });
  console.log('Request ID:', message._request_id);
  ```
</CodeGroup>

## 長時間のリクエスト

<Warning>
 特に10分を超える長時間実行されるリクエストについては、[ストリーミングMessages API](/docs/ja/build-with-claude/streaming)または[Message Batches API](/docs/ja/api/creating-message-batches)の使用を強く推奨します。
</Warning>

私たちの[ストリーミングMessages API](/docs/ja/build-with-claude/streaming)または[Message Batches API](/docs/ja/api/creating-message-batches)を使用せずに大きな`max_tokens`値を設定することは推奨しません：

- 一部のネットワークは可変期間後にアイドル接続を切断する場合があり、これによりAnthropicからレスポンスを受信することなくリクエストが失敗またはタイムアウトする可能性があります。
- ネットワークの信頼性は異なります；私たちの[Message Batches API](/docs/ja/api/creating-message-batches)は、中断されないネットワーク接続を必要とするのではなく、結果をポーリングできるようにすることで、ネットワーク問題のリスクを管理するのに役立ちます。

直接的なAPI統合を構築している場合、[TCPソケットキープアライブ](https://tldp.org/HOWTO/TCP-Keepalive-HOWTO/programming.html)を設定することで、一部のネットワークでのアイドル接続タイムアウトの影響を軽減できることを認識しておく必要があります。

私たちの[SDK](/docs/ja/api/client-sdks)は、非ストリーミングMessages APIリクエストが10分のタイムアウトを超えることが予想されないことを検証し、TCPキープアライブのソケットオプションも設定します。