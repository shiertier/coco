# チケットルーティング

このガイドでは、Claude の高度な自然言語理解機能を活用して、顧客の意図、緊急度、優先順位、顧客プロフィールなど、様々な要因に基づいて大規模な顧客サポートチケットを分類する方法について説明します。

---

## チケットルーティングに Claude を使用するかどうかを決定する

従来の ML アプローチではなく、Claude のような LLM を分類タスクに使用すべき主な指標を以下に示します。

    <section title="利用可能なラベル付きトレーニングデータが限定されている">

        従来の ML プロセスには、大規模なラベル付きデータセットが必要です。Claude の事前学習済みモデルは、わずか数十個のラベル付き例でチケットを効果的に分類でき、データ準備の時間とコストを大幅に削減できます。
    
</section>
    <section title="分類カテゴリが時間とともに変更または進化する可能性がある">

        従来の ML アプローチが確立されると、それを変更することは面倒でデータ集約的な作業になります。一方、製品または顧客のニーズが進化するにつれて、Claude はクラス定義の変更や新しいクラスに簡単に適応でき、トレーニングデータの大規模な再ラベリングは不要です。
    
</section>
    <section title="複雑で非構造化されたテキスト入力を処理する必要がある">

        従来の ML モデルは非構造化データに苦労することが多く、広範な特徴エンジニアリングが必要です。Claude の高度な言語理解により、厳密なオントロジー構造に依存するのではなく、コンテンツとコンテキストに基づいた正確な分類が可能になります。
    
</section>
    <section title="分類ルールが意味的理解に基づいている">

        従来の ML アプローチは、多くの場合、単語の袋モデルまたは単純なパターンマッチングに依存しています。Claude は、クラスが例ではなく条件によって定義される場合、基本的なルールを理解して適用することに優れています。
    
</section>
    <section title="分類決定の解釈可能な推論が必要である">

        多くの従来の ML モデルは、意思決定プロセスについてほとんど洞察を提供しません。Claude は分類決定の人間が読める説明を提供でき、自動化システムへの信頼を構築し、必要に応じて簡単に適応できます。
    
</section>
    <section title="エッジケースと曖昧なチケットをより効果的に処理したい">

        従来の ML システムは、外れ値と曖昧な入力に苦労することが多く、それらを誤分類したり、キャッチオール カテゴリにデフォルト設定したりすることがよくあります。Claude の自然言語処理機能により、サポートチケット内のコンテキストとニュアンスをより適切に解釈でき、手動介入が必要な誤ルーティングまたは未分類のチケットの数を削減できます。
    
</section>
    <section title="別々のモデルを維持せずに多言語サポートが必要である">

        従来の ML アプローチでは、通常、サポートされている言語ごとに別々のモデルまたは広範な翻訳プロセスが必要です。Claude の多言語機能により、別々のモデルや広範な翻訳プロセスなしに、様々な言語のチケットを分類でき、グローバルな顧客ベースのサポートが合理化されます。
    
</section>

***

##  LLM サポートワークフローを構築してデプロイする

### 現在のサポートアプローチを理解する
自動化に飛び込む前に、既存のチケッティングシステムを理解することが重要です。サポートチームが現在チケットルーティングをどのように処理しているかを調査することから始めます。

以下のような質問を検討してください。
* どのような基準を使用して、どの SLA/サービス提供が適用されるかを決定していますか?
* チケットルーティングは、チケットがどのレベルのサポートまたは製品スペシャリストに送られるかを決定するために使用されていますか?
* すでに実装されている自動ルールまたはワークフローはありますか? どのような場合に失敗しますか?
* エッジケースまたは曖昧なチケットはどのように処理されていますか?
* チームはチケットをどのように優先順位付けしていますか?

人間が特定のケースをどのように処理するかについて知識が多いほど、Claude でそのタスクを実行するのに役立ちます。

### ユーザーインテントカテゴリを定義する
ユーザーインテントカテゴリの明確に定義されたリストは、Claude を使用した正確なサポートチケット分類に重要です。Claude がシステム内でチケットを効果的にルーティングする能力は、システムのカテゴリがどの程度明確に定義されているかに正比例します。

以下は、ユーザーインテントカテゴリとサブカテゴリの例です。

    <section title="技術的な問題">

        * ハードウェアの問題
        * ソフトウェアバグ
        * 互換性の問題
        * パフォーマンスの問題
    
</section>
    <section title="アカウント管理">

        * パスワードリセット
        * アカウントアクセスの問題
        * 請求に関する問い合わせ
        * サブスクリプション変更
    
</section>
    <section title="製品情報">

        * 機能に関する問い合わせ
        * 製品互換性に関する質問
        * 価格情報
        * 利用可能性に関する問い合わせ
    
</section>
    <section title="ユーザーガイダンス">

        * ハウツー質問
        * 機能使用支援
        * ベストプラクティスアドバイス
        * トラブルシューティングガイダンス
    
</section>
    <section title="フィードバック">

        * バグレポート
        * 機能リクエスト
        * 一般的なフィードバックまたは提案
        * 苦情
    
</section>
    <section title="注文関連">

        * 注文ステータスの問い合わせ
        * 配送情報
        * 返品と交換
        * 注文の変更
    
</section>
    <section title="サービスリクエスト">

        * インストール支援
        * アップグレードリクエスト
        * メンテナンススケジューリング
        * サービスキャンセル
    
</section>
    <section title="セキュリティに関する懸念">

        * データプライバシーに関する問い合わせ
        * 疑わしいアクティビティレポート
        * セキュリティ機能支援
    
</section>
    <section title="コンプライアンスと法務">

        * 規制コンプライアンスに関する質問
        * 利用規約に関する問い合わせ
        * 法的文書リクエスト
    
</section>
    <section title="緊急サポート">

        * システムの重大な障害
        * 緊急のセキュリティ問題
        * 時間に敏感な問題
    
</section>
    <section title="トレーニングと教育">

        * 製品トレーニングリクエスト
        * ドキュメントに関する問い合わせ
        * ウェビナーまたはワークショップ情報
    
</section>
    <section title="統合と API">

        * 統合支援
        * API 使用に関する質問
        * サードパーティ互換性に関する問い合わせ
    
</section>

インテントに加えて、チケットルーティングと優先順位付けは、緊急度、顧客タイプ、SLA、言語などの他の要因の影響を受ける場合があります。自動ルーティングシステムを構築する際に、他のルーティング基準も考慮してください。

### 成功基準を確立する

サポートチームと協力して、測定可能なベンチマーク、閾値、目標を含む[明確な成功基準を定義](/docs/ja/test-and-evaluate/define-success)してください。

サポートチケットルーティングに LLM を使用する場合の標準的な基準とベンチマークを以下に示します。

    <section title="分類の一貫性">

        このメトリクスは、Claude が時間とともに同様のチケットをどの程度一貫して分類するかを評価します。ルーティングの信頼性を維持するために重要です。標準化された入力セットで定期的にモデルをテストし、一貫性率が 95% 以上を目指すことで測定します。
    
</section>
    <section title="適応速度">

        これは、Claude が新しいカテゴリまたは変化するチケットパターンにどの程度迅速に適応できるかを測定します。新しいチケットタイプを導入し、モデルがこれらの新しいカテゴリで満足のいく精度 (例: >90%) を達成するのにかかる時間を測定することでテストします。50～100 個のサンプルチケット内での適応を目指します。
    
</section>
    <section title="多言語処理">

        これは、複数の言語でチケットを正確にルーティングする Claude の能力を評価します。異なる言語全体でのルーティング精度を測定し、非主要言語での精度低下が 5～10% 以下を目指します。
    
</section>
    <section title="エッジケース処理">

        これは、異常または複雑なチケットに対する Claude のパフォーマンスを評価します。エッジケースのテストセットを作成し、ルーティング精度を測定し、これらの困難な入力に対して少なくとも 80% の精度を目指します。
    
</section>
    <section title="バイアス軽減">

        これは、異なる顧客層全体でのルーティングにおける Claude の公平性を測定します。ルーティング決定を定期的に監査して潜在的なバイアスを検出し、すべての顧客グループ全体で一貫したルーティング精度 (2～3% 以内) を目指します。
    
</section>
    <section title="プロンプト効率">

        トークン数の最小化が重要な状況では、この基準は Claude が最小限のコンテキストでどの程度パフォーマンスを発揮するかを評価します。異なる量のコンテキストを提供してルーティング精度を測定し、チケットタイトルと簡潔な説明だけで 90% 以上の精度を目指します。
    
</section>
    <section title="説明可能性スコア">

        これは、ルーティング決定に対する Claude の説明の品質と関連性を評価します。人間の評価者は説明をスケール (例: 1～5) で採点でき、平均スコア 4 以上を達成することを目標とします。
    
</section>

LLM が使用されるかどうかに関係なく、有用である可能性のある一般的な成功基準を以下に示します。

    <section title="ルーティング精度">

        ルーティング精度は、チケットが初回で適切なチームまたは個人に正しく割り当てられる頻度を測定します。これは通常、正しくルーティングされたチケットの割合を総チケット数で表します。業界ベンチマークは通常 90～95% の精度を目指していますが、これはサポート構造の複雑さに基づいて異なる場合があります。
    
</section>
    <section title="割り当てまでの時間">

        このメトリクスは、チケットが送信された後、割り当てられるまでの速さを追跡します。割り当て時間が短いほど、一般的に解決が早くなり、顧客満足度が向上します。最高クラスのシステムは、多くの場合 5 分未満の平均割り当て時間を達成し、多くは瞬時のルーティング (LLM 実装で可能) を目指しています。
    
</section>
    <section title="リルーティング率">

        リルーティング率は、初期ルーティング後にチケットを再割り当てする必要がある頻度を示します。低い率は、より正確な初期ルーティングを示唆しています。リルーティング率を 10% 未満に抑えることを目指し、トップパフォーマーは 5% 以下の率を達成しています。
    
</section>
    <section title="初回接触解決率">

        これは、顧客との最初のやり取り中に解決されたチケットの割合を測定します。高い率は、効率的なルーティングとよく準備されたサポートチームを示します。業界ベンチマークは通常 70～75% の範囲ですが、トップパフォーマーは 80% 以上の率を達成しています。
    
</section>
    <section title="平均処理時間">

        平均処理時間は、チケットを開始から終了まで解決するのにかかる時間を測定します。効率的なルーティングはこの時間を大幅に削減できます。ベンチマークは業界と複雑さによって大きく異なりますが、多くの組織は非重大な問題の平均処理時間を 24 時間以下に保つことを目指しています。
    
</section>
    <section title="顧客満足度スコア">

        多くの場合、やり取り後の調査を通じて測定され、これらのスコアはサポートプロセスに対する全体的な顧客満足度を反映しています。効果的なルーティングはより高い満足度に貢献します。CSAT スコアが 90% 以上を目指し、トップパフォーマーは 95% 以上の満足度を達成することが多いです。
    
</section>
    <section title="エスカレーション率">

        これは、チケットをサポートのより高いレベルにエスカレートする必要がある頻度を測定します。低いエスカレーション率は、より正確な初期ルーティングを示すことが多いです。エスカレーション率を 20% 未満に抑えることを目指し、最高クラスのシステムは 10% 以下の率を達成しています。
    
</section>
    <section title="エージェント生産性">

        このメトリクスは、ルーティングソリューションを実装した後、エージェントが効果的に処理できるチケット数を調べます。改善されたルーティングは生産性を向上させるはずです。これを測定するには、1 日または 1 時間あたりのエージェントごとに解決されたチケット数を追跡し、新しいルーティングシステムを実装した後に 10～20% の改善を目指します。
    
</section>
    <section title="セルフサービス転換率">

        これは、ルーティングシステムに入る前にセルフサービスオプションを通じて解決された潜在的なチケットの割合を測定します。高い率は、効果的なルーティング前のトリアージを示します。転換率を 20～30% に抑えることを目指し、トップパフォーマーは 40% 以上の率を達成しています。
    
</section>
    <section title="チケットあたりのコスト">

        このメトリクスは、各サポートチケットを解決するための平均コストを計算します。効率的なルーティングは、時間とともにこのコストを削減するのに役立つはずです。ベンチマークは大きく異なりますが、多くの組織は改善されたルーティングシステムを実装した後、チケットあたりのコストを 10～15% 削減することを目指しています。
    
</section>

### 適切な Claude モデルを選択する

モデルの選択は、コスト、精度、応答時間の間のトレードオフに依存します。

多くの顧客は、`claude-haiku-4-5-20251001` がチケットルーティングに理想的なモデルであることに気付いています。これは Claude 4 ファミリーで最も高速で最もコスト効率的なモデルでありながら、優れた結果を提供しています。分類問題が深い主題専門知識または大量のインテントカテゴリと複雑な推論を必要とする場合は、[より大きな Sonnet モデル](/docs/ja/about-claude/models)を選択することができます。

### 強力なプロンプトを構築する

チケットルーティングは分類タスクの一種です。Claude はサポートチケットのコンテンツを分析し、問題タイプ、緊急度、必要な専門知識、またはその他の関連要因に基づいて、事前定義されたカテゴリに分類します。

チケット分類プロンプトを作成しましょう。初期プロンプトには、ユーザーリクエストのコンテンツが含まれ、推論とインテントの両方を返す必要があります。

<Tip>
[Claude Console](/login) の[プロンプトジェネレータ](/docs/ja/prompt-generator)を試して、Claude に最初のドラフトを作成させてください。
</Tip>

チケットルーティング分類プロンプトの例を以下に示します。
```python
def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. Your task is to analyze customer support requests and output the appropriate classification intent for each request, along with your reasoning. 

        Here is the customer support request you need to classify:

        <request>{ticket_contents}</request>

        Please carefully analyze the above request to determine the customer's core intent and needs. Consider what the customer is asking for has concerns about.

        First, write out your reasoning and analysis of how to classify this request inside <reasoning> tags.

        Then, output the appropriate classification label for the request inside a <intent> tag. The valid intents are:
        <intents>
        <intent>Support, Feedback, Complaint</intent>
        <intent>Order Tracking</intent>
        <intent>Refund/Exchange</intent>
        </intents>

        A request may have ONLY ONE applicable intent. Only include the intent that is most applicable to the request.

        As an example, consider the following request:
        <request>Hello! I had high-speed fiber internet installed on Saturday and my installer, Kevin, was absolutely fantastic! Where can I send my positive review? Thanks for your help!</request>

        Here is an example of how your output should be formatted (for the above example request):
        <reasoning>The user seeks information in order to leave positive feedback.</reasoning>
        <intent>Support, Feedback, Complaint</intent>

        Here are a few more examples:
        <examples>
        <example 2>
        Example 2 Input:
        <request>I wanted to write and personally thank you for the compassion you showed towards my family during my father's funeral this past weekend. Your staff was so considerate and helpful throughout this whole process; it really took a load off our shoulders. The visitation brochures were beautiful. We'll never forget the kindness you showed us and we are so appreciative of how smoothly the proceedings went. Thank you, again, Amarantha Hill on behalf of the Hill Family.</request>

        Example 2 Output:
        <reasoning>User leaves a positive review of their experience.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 2>
        <example 3>

        ...

        </example 8>
        <example 9>
        Example 9 Input:
        <request>Your website keeps sending ad-popups that block the entire screen. It took me twenty minutes just to finally find the phone number to call and complain. How can I possibly access my account information with all of these popups? Can you access my account for me, since your website is broken? I need to know what the address is on file.</request>

        Example 9 Output:
        <reasoning>The user requests help accessing their web account information.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 9>

        Remember to always include your classification reasoning before your actual intent output. The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
```

このプロンプトの主要なコンポーネントを分解してみましょう。
* Python f-string を使用してプロンプトテンプレートを作成し、`ticket_contents` を `<request>` タグに挿入できるようにしています。
* Claude に分類システムとしての明確に定義された役割を与え、チケットコンテンツを慎重に分析して顧客の中核的な意図とニーズを決定します。
* Claude に適切な出力フォーマットについて指示し、この場合、推論と分析を `<reasoning>` タグ内に、適切な分類ラベルを `<intent>` タグ内に提供します。
* 有効なインテントカテゴリを指定します。「Support, Feedback, Complaint」、「Order Tracking」、「Refund/Exchange」です。
* 出力がどのようにフォーマットされるべきかを示すために、いくつかの例 (少数ショットプロンプティングとも呼ばれます) を含めます。これにより、精度と一貫性が向上します。

Claude の応答を様々な XML タグセクションに分割したい理由は、正規表現を使用して出力から推論とインテントを個別に抽出できるようにするためです。これにより、チケットルーティングワークフロー内で対象となる次のステップを作成できます。例えば、インテントのみを使用してチケットをどの人にルーティングするかを決定します。

### プロンプトをデプロイする

プロンプトがテスト本番環境でどの程度機能するかを知ることは難しく、[評価を実行](/docs/ja/test-and-evaluate/develop-tests)する必要があります。

デプロイメント構造を構築しましょう。Claude への呼び出しをラップするメソッドシグネチャを定義することから始めます。すでに書き始めたメソッドを取得します。これは `ticket_contents` を入力として持ち、`reasoning` と `intent` のタプルを出力として返すようにします。従来の ML を使用した既存の自動化がある場合は、代わりにそのメソッドシグネチャに従う必要があります。

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ... The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
    # Send the prompt to the API to classify the support request.
    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
        stream=False,
    )
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

    return reasoning, intent
```

このコードは以下を実行します。
* Anthropic ライブラリをインポートし、API キーを使用してクライアントインスタンスを作成します。
* `ticket_contents` 文字列を取得する `classify_support_request` 関数を定義します。
* `classification_prompt` を使用して `ticket_contents` を Claude に送信します。
* レスポンスから抽出された `reasoning` と `intent` を返します。

推論とインテントテキスト全体が生成されるのを待ってから解析する必要があるため、`stream=False` (デフォルト) を設定します。

***

## プロンプトを評価する

プロンプティングは、本番環境に対応するために、テストと最適化が必要なことが多いです。ソリューションの準備状況を判断するには、以前に確立した成功基準と閾値に基づいてパフォーマンスを評価します。

評価を実行するには、実行するテストケースが必要です。このガイドの残りの部分では、すでに[テストケースを開発](/docs/ja/test-and-evaluate/develop-tests)していることを前提としています。

### 評価関数を構築する

このガイドの例の評価は、3 つの主要なメトリクスに沿って Claude のパフォーマンスを測定します。
* 精度
* 分類あたりのコスト

あなたにとって重要な要因に応じて、他の軸で Claude を評価する必要があるかもしれません。

これを評価するために、まず書いたスクリプトを変更し、予測されたインテントと実際のインテントを比較し、正しい予測の割合を計算する関数を追加する必要があります。また、コスト計算と時間測定機能を追加する必要があります。

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(request, actual_intent):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ...The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """

    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
    )
    usage = message.usage  # Get the usage statistics for the API call for how many input and output tokens were used.
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

      # Check if the model's prediction is correct.
    correct = actual_intent.strip() == intent.strip()

    # Return the reasoning, intent, correct, and usage.
    return reasoning, intent, correct, usage
```

加えた編集を分解してみましょう。
* テストケースから `actual_intent` を `classify_support_request` メソッドに追加し、Claude のインテント分類が黄金のインテント分類と一致するかどうかを評価するための比較を設定しました。
* API 呼び出しの使用統計を抽出して、使用されたインプットトークンとアウトプットトークンに基づいてコストを計算しました。

### 評価を実行する

適切な評価には、何が良い結果であるかを判断するための明確な閾値とベンチマークが必要です。上記のスクリプトは、精度、応答時間、分類あたりのコストの実行時値を提供しますが、明確に確立された閾値が必要です。例えば:
* **精度:** 95% (100 個のテストのうち)
* **分類あたりのコスト:** 現在のルーティング方法から平均 50% 削減 (100 個のテスト全体)

これらの閾値を設定することで、大規模で公平な経験主義を使用して、どの方法があなたに最適であるか、そしてあなたの要件に合わせるためにどのような変更が必要かを迅速かつ簡単に判断できます。

***

## パフォーマンスを改善する

複雑なシナリオでは、標準的な[プロンプトエンジニアリング技術](/docs/ja/build-with-claude/prompt-engineering/overview)と[ガードレール実装戦略](/docs/ja/test-and-evaluate/strengthen-guardrails/reduce-hallucinations)を超えてパフォーマンスを改善するための追加戦略を検討することが役立つ場合があります。一般的なシナリオを以下に示します。

### 20 以上のインテントカテゴリがある場合は、分類学的階層を使用する

クラスの数が増えるにつれて、必要な例の数も増加し、プロンプトが扱いにくくなる可能性があります。別の方法として、分類器の混合を使用して階層的分類システムを実装することを検討できます。
1. インテントを分類学的ツリー構造に整理します。
2. ツリーのすべてのレベルで一連の分類器を作成し、カスケードルーティングアプローチを有効にします。

例えば、チケットを「技術的な問題」、「請求に関する質問」、「一般的な問い合わせ」に大まかに分類する最上位の分類器がある場合があります。これらのカテゴリのそれぞれは、分類をさらに絞り込むための独自のサブ分類器を持つことができます。

![](/docs/images/ticket-hierarchy.png)

* **長所 - より大きなニュアンスと精度:** 各親パスに対して異なるプロンプトを作成でき、より対象を絞った文脈固有の分類が可能になります。これにより、精度が向上し、顧客リクエストのより微妙な処理が可能になります。

* **短所 - レイテンシの増加:** 複数の分類器はレイテンシの増加につながる可能性があり、最速のモデルである Haiku でこのアプローチを実装することをお勧めします。

### ベクトルデータベースと類似性検索検索を使用して、非常に可変的なチケットを処理する

例を提供することが最も効果的なパフォーマンス改善方法ですが、サポートリクエストが非常に可変的な場合、単一のプロンプトに十分な例を含めるのは難しい場合があります。

このシナリオでは、ベクトルデータベースを使用して例のデータセットから類似性検索を実行し、特定のクエリに対して最も関連性の高い例を取得できます。

[分類レシピ](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb)で詳しく説明されているこのアプローチは、パフォーマンスを 71% の精度から 93% の精度に改善することが示されています。

### 予想されるエッジケースを具体的に考慮する

Claude がチケットを誤分類する可能性があるシナリオを以下に示します (あなたの状況に固有の他のシナリオがある場合があります)。これらのシナリオでは、Claude がエッジケースをどのように処理するかについて、プロンプトで明示的な指示または例を提供することを検討してください。

    <section title="顧客は暗黙的なリクエストを行う">

        顧客は多くの場合、ニーズを間接的に表現します。例えば、「2 週間以上前から荷物を待っています」は、注文ステータスの間接的なリクエストかもしれません。
        * **解決策:** Claude に、これらの種類のリクエストの実際の顧客例と、基本的な意図が何であるかを提供します。特に微妙なチケットインテントの分類根拠を含めると、Claude が他のチケットにロジックをより適切に一般化できるため、さらに良い結果が得られます。
    
</section>
    <section title="Claude は感情をインテントより優先する">

        顧客が不満を表現する場合、Claude は基本的な問題を解決するのではなく、感情に対処することを優先する場合があります。
        * **解決策:** Claude に、顧客の感情を優先するかどうかについての指示を提供します。「すべての顧客の感情を無視してください。顧客のリクエストの意図と顧客が求めている可能性のある情報の分析のみに焦点を当ててください」のようなシンプルなものでもかまいません。
    
</section>
    <section title="複数の問題が問題の優先順位付けの混乱を引き起こす">

        顧客が単一のやり取りで複数の問題を提示する場合、Claude は主な懸念事項を特定するのに困難を感じる場合があります。
        * **解決策:** インテントの優先順位付けを明確にして、Claude がより適切に抽出されたインテントをランク付けし、主な懸念事項を特定できるようにします。
    
</section>

***

## Claude をより大きなサポートワークフローに統合する

適切な統合には、Claude ベースのチケットルーティングスクリプトがより大きなチケットルーティングシステムのアーキテクチャにどのように適合するかについて、いくつかの決定を行う必要があります。これを行う方法は 2 つあります。
* **プッシュベース:** 使用しているサポートチケットシステム (例: Zendesk) は、ルーティングサービスに webhook イベントを送信することでコードをトリガーし、その後、インテントを分類してルーティングします。
    * このアプローチはより web スケーラブルですが、公開エンドポイントを公開する必要があります。
* **プルベース:** コードは指定されたスケジュールに基づいて最新のチケットをプルし、プル時にルーティングします。
    * このアプローチは実装が簡単ですが、プル頻度が高すぎるとサポートチケットシステムへの不要な呼び出しが発生する可能性があり、プル頻度が低すぎるとやや遅い可能性があります。

これらのアプローチのいずれかについて、スクリプトをサービスにラップする必要があります。アプローチの選択は、サポートチケッティングシステムが提供する API に依存します。

***

<CardGroup cols={2}>
    <Card title="分類クックブック" icon="link" href="https://github.com/anthropics/anthropic-cookbook/tree/main/capabilities/classification">
        より多くのサンプルコードと詳細な評価ガイダンスについては、分類クックブックをご覧ください。
    </Card>
    <Card title="Claude Console" icon="link" href="/dashboard">
        Claude Console でワークフローの構築と評価を開始します。
    </Card>
</CardGroup>