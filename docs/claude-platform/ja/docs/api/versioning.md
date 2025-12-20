# バージョン

APIリクエストを行う際は、`anthropic-version`リクエストヘッダーを送信する必要があります。例：`anthropic-version: 2023-06-01`。[クライアントSDK](/docs/ja/api/client-sdks)を使用している場合、これは自動的に処理されます。

---

任意のAPIバージョンについて、以下を保持します：

* 既存の入力パラメータ
* 既存の出力パラメータ

ただし、以下を行う場合があります：

* 追加のオプション入力の追加
* 出力への追加値の追加
* 特定のエラータイプの条件の変更
* 列挙型のような出力値への新しいバリアントの追加（例：ストリーミングイベントタイプ）

一般的に、このリファレンスで文書化されているとおりにAPIを使用している場合、使用方法が破綻することはありません。

## バージョン履歴

可能な限り最新のAPIバージョンを使用することを常に推奨します。以前のバージョンは非推奨とみなされ、新規ユーザーには利用できない場合があります。

* `2023-06-01`  
   * [ストリーミング](/docs/ja/api/streaming)サーバー送信イベント（SSE）の新しい形式：  
         * 補完は増分的です。例：`" Hello"`, `" my"`, `" name"`, `" is"`, `" Claude." `の代わりに`" Hello"`, `" Hello my"`, `" Hello my name"`, `" Hello my name is"`, `" Hello my name is Claude."`。  
         * すべてのイベントは[データのみのイベント](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#data-only%5Fmessages)ではなく、[名前付きイベント](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#named%5Fevents)です。  
         * 不要な`data: [DONE]`イベントを削除。  
   * レスポンスの従来の`exception`と`truncated`値を削除。
* `2023-01-01`: 初回リリース。