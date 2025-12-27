# コンテンツモデレーション

コンテンツモデレーションは、デジタルアプリケーションにおいて安全で敬意に満ちた生産的な環境を維持するための重要な側面です。このガイドでは、デジタルアプリケーション内でコンテンツをモデレートするためにClaudeをどのように使用できるかについて説明します。

---

> Claudeを使用したコンテンツモデレーション実装の例については、[コンテンツモデレーションクックブック](https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fmoderation%5Ffilter.ipynb)をご覧ください。

<Tip>このガイドは、アプリケーション内でユーザー生成コンテンツをモデレートすることに焦点を当てています。Claudeとのやり取りをモデレートするためのガイダンスをお探しの場合は、[ガードレールガイド](/docs/ja/test-and-evaluate/strengthen-guardrails/reduce-hallucinations)をご参照ください。</Tip>

## Claudeで構築する前に

### コンテンツモデレーションにClaudeを使用するかどうかを決定する

コンテンツモデレーションに従来のMLやルールベースのアプローチではなく、ClaudeのようなLLMを使用すべき主要な指標をいくつか示します：

<section title="コスト効率的で迅速な実装を求めている">
従来のML手法には、大幅なエンジニアリングリソース、ML専門知識、インフラストラクチャコストが必要です。人間によるモデレーションシステムはさらに高いコストがかかります。Claudeを使用すれば、わずかな時間とコストで洗練されたモデレーションシステムを稼働させることができます。
</section>
<section title="意味理解と迅速な判断の両方を求めている">
bag-of-wordsモデルや単純なパターンマッチングなどの従来のMLアプローチは、コンテンツのトーン、意図、文脈を理解するのに苦労することがよくあります。人間によるモデレーションシステムは意味理解に優れていますが、コンテンツのレビューに時間が必要です。Claudeは意味理解と迅速なモデレーション判断の提供能力を組み合わせることで、このギャップを埋めます。
</section>
<section title="一貫したポリシー決定が必要">
高度な推論能力を活用することで、Claudeは複雑なモデレーションガイドラインを統一的に解釈し適用できます。この一貫性により、すべてのコンテンツの公平な扱いが保証され、ユーザーの信頼を損なう可能性のある一貫性のない偏ったモデレーション決定のリスクが軽減されます。
</section>
<section title="モデレーションポリシーが時間とともに変更または進化する可能性が高い">
従来のMLアプローチが確立されると、それを変更することは労力集約的でデータ集約的な作業になります。一方、製品や顧客のニーズが進化するにつれて、Claudeは大量のトレーニングデータの再ラベル付けなしに、モデレーションポリシーの変更や追加に簡単に適応できます。
</section>
<section title="モデレーション決定に対する解釈可能な推論が必要">
モデレーション決定の背後にある明確な説明をユーザーや規制当局に提供したい場合、Claudeは詳細で一貫した正当化を生成できます。この透明性は、コンテンツモデレーション実践における信頼の構築と説明責任の確保にとって重要です。
</section>
<section title="別々のモデルを維持することなく多言語サポートが必要">
従来のMLアプローチでは、通常、サポートする各言語に対して別々のモデルまたは広範な翻訳プロセスが必要です。人間によるモデレーションでは、サポートする各言語に堪能な労働力を雇用する必要があります。Claudeの多言語機能により、別々のモデルや広範な翻訳プロセスを必要とせずに、さまざまな言語でチケットを分類でき、グローバルな顧客ベースのモデレーションを合理化します。
</section>
<section title="マルチモーダルサポートが必要">
Claudeのマルチモーダル機能により、テキストと画像の両方でコンテンツを分析し解釈できます。これにより、異なるメディアタイプを一緒に評価する必要がある環境での包括的なコンテンツモデレーションのための多用途ツールとなります。
</section>

<Note>Anthropicは、すべてのClaudeモデルを正直で有用で無害になるように訓練しています。これにより、使用されるプロンプトに関係なく、Claudeが特に危険とみなされるコンテンツ（[利用規約](https://www.anthropic.com/legal/aup)に沿って）をモデレートする可能性があります。たとえば、ユーザーが露骨な性的コンテンツを投稿できるようにしたいアダルトウェブサイトでは、プロンプトで露骨な性的コンテンツをモデレートしないよう指定しても、Claudeが依然として露骨なコンテンツをモデレーションが必要としてフラグを立てる可能性があります。モデレーションソリューションを構築する前に、利用規約を事前に確認することをお勧めします。</Note>

### モデレートするコンテンツの例を生成する
コンテンツモデレーションソリューションを開発する前に、まずフラグを立てるべきコンテンツとフラグを立てるべきでないコンテンツの例を作成します。コンテンツモデレーションシステムが効果的に処理するのが困難な可能性のあるエッジケースや困難なシナリオを含めるようにしてください。その後、例を確認して、明確に定義されたモデレーションカテゴリのリストを作成します。
たとえば、ソーシャルメディアプラットフォームによって生成される例には、以下が含まれる可能性があります：

```python
allowed_user_comments = [
    'This movie was great, I really enjoyed it. The main actor really killed it!',
    'I hate Mondays.',
    'It is a great time to invest in gold!'
]

disallowed_user_comments = [
    'Delete this post now or you better hide. I am coming after you and your family.',
    'Stay away from the 5G cellphones!! They are using 5G to control you.',
    'Congratulations! You have won a $1,000 gift card. Click here to claim your prize!'
]

# Sample user comments to test the content moderation
user_comments = allowed_user_comments + disallowed_user_comments

# List of categories considered unsafe for content moderation
unsafe_categories = [
    'Child Exploitation',
    'Conspiracy Theories',
    'Hate',
    'Indiscriminate Weapons', 
    'Intellectual Property',
    'Non-Violent Crimes', 
    'Privacy',
    'Self-Harm',
    'Sex Crimes',
    'Sexual Content',
    'Specialized Advice',
    'Violent Crimes'
]
```

これらの例を効果的にモデレートするには、言語の微妙な理解が必要です。コメント「This movie was great, I really enjoyed it. The main actor really killed it!」では、コンテンツモデレーションシステムは「killed it」が比喩であり、実際の暴力の兆候ではないことを認識する必要があります。逆に、暴力の明示的な言及がないにもかかわらず、コメント「Delete this post now or you better hide. I am coming after you and your family.」はコンテンツモデレーションシステムによってフラグを立てられるべきです。

`unsafe_categories`リストは、特定のニーズに合わせてカスタマイズできます。たとえば、未成年者がウェブサイトでコンテンツを作成することを防ぎたい場合は、リストに「Underage Posting」を追加できます。

___

## Claudeを使用してコンテンツをモデレートする方法

### 適切なClaudeモデルを選択する
モデルを選択する際は、データのサイズを考慮することが重要です。コストが懸念事項である場合、Claude Haiku 3のような小さなモデルは、コスト効率性により優れた選択です。以下は、月に10億件の投稿を受信するソーシャルメディアプラットフォームのテキストをモデレートするコストの見積もりです：

* **コンテンツサイズ**
    * 月間投稿数：10億件
    * 投稿あたりの文字数：100文字
    * 総文字数：1000億文字

* **推定トークン**
    * 入力トークン：286億トークン（3.5文字あたり1トークンと仮定）
    * フラグが立てられるメッセージの割合：3%
    * フラグが立てられたメッセージあたりの出力トークン：50
    * 総出力トークン：15億トークン

* **Claude Haiku 3の推定コスト**
    * 入力トークンコスト：2,860 MTok * $0.25/MTok = $715
    * 出力トークンコスト：1,500 MTok * $1.25/MTok = $1,875
    * 月間コスト：$715 + $1,875 = $2,590

* **Claude Sonnet 4.5の推定コスト**
    * 入力トークンコスト：2,860 MTok * $3.00/MTok = $8,580
    * 出力トークンコスト：1,500 MTok * $15.00/MTok = $22,500
    * 月間コスト：$8,580 + $22,500 = $31,080

<Tip>実際のコストはこれらの見積もりと異なる場合があります。これらの見積もりは、[バッチ処理](#consider-batch-processing)のセクションで強調されているプロンプトに基づいています。出力トークンは、レスポンスから`explanation`フィールドを削除することでさらに削減できます。</Tip>  

### 強力なプロンプトを構築する

コンテンツモデレーションにClaudeを使用するために、Claudeはアプリケーションのモデレーション要件を理解する必要があります。モデレーションニーズを定義できるプロンプトを書くことから始めましょう：

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def moderate_message(message, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Construct the prompt for Claude, including the message and unsafe categories
    assessment_prompt = f"""
    Determine whether the following message warrants moderation, 
    based on the unsafe categories outlined below.
    
    Message:
    <message>{message}</message>

    Unsafe Categories:
    <categories>
    {unsafe_category_str}
    </categories>

    Respond with ONLY a JSON object, using the format below:
    {{
    "violation": <Boolean field denoting whether the message should be moderated>,
    "categories": [Comma-separated list of violated categories],
    "explanation": [Optional. Only include if there is a violation.]
    }}"""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the violation status from the assessment
    contains_violation = assessment['violation']
    
    # If there's a violation, get the categories and explanation; otherwise, use empty defaults
    violated_categories = assessment.get('categories', []) if contains_violation else []
    explanation = assessment.get('explanation') if contains_violation else None
    
    return contains_violation, violated_categories, explanation

# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    violation, violated_categories, explanation = moderate_message(comment, unsafe_categories)
    
    if violation:
        print(f"Violated Categories: {', '.join(violated_categories)}")
        print(f"Explanation: {explanation}")
    else:
        print("No issues detected.")
```

この例では、`moderate_message`関数には、安全でないコンテンツカテゴリと評価したいメッセージを含む評価プロンプトが含まれています。プロンプトは、定義した安全でないカテゴリに基づいて、メッセージをモデレートすべきかどうかをClaudeに評価するよう求めています。

その後、モデルの評価が解析され、違反があるかどうかが判断されます。違反がある場合、Claudeは違反したカテゴリのリストと、メッセージが安全でない理由の説明も返します。

### プロンプトを評価する

コンテンツモデレーションは分類問題です。したがって、[分類クックブック](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/classification/guide.ipynb)で概説されている同じ技術を使用して、コンテンツモデレーションシステムの精度を判断できます。

追加の考慮事項の一つは、コンテンツモデレーションを二項分類問題として扱う代わりに、さまざまなリスクレベルを表す複数のカテゴリを作成することです。複数のリスクレベルを作成することで、モデレーションの積極性を調整できます。たとえば、高リスクとみなされるユーザークエリを自動的にブロックし、中リスククエリが多いユーザーを人間によるレビューのためにフラグを立てることができます。

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def assess_risk_level(message, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Construct the prompt for Claude, including the message, unsafe categories, and risk level definitions
    assessment_prompt = f"""
    Assess the risk level of the following message warrants moderation, 
    based on the unsafe categories listed below.
    
Message:
<message>{message}</message>

Unsafe Categories:
<categories>
{unsafe_category_str}
</categories>

Assign a risk level based on your confidence that the user's message should be moderated 
based on the defined unsafe categories:

0 - No risk
1 - Low risk
2 - Medium risk
3 - High risk

Respond with ONLY a JSON object, using the format below:
{{
  "risk_level": <Numerical field denoting the risk level>,
  "categories": [Comma-separated list of violated categories],
  "explanation": <Optional. Only include if risk level is greater than 0>
}}"""

    # Send the request to Claude for risk assessment
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the risk level, violated categories, and explanation from the assessment
    risk_level = assessment["risk_level"]
    violated_categories = assessment["categories"]
    explanation = assessment.get("explanation")
    
    return risk_level, violated_categories, explanation

# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    risk_level, violated_categories, explanation = assess_risk_level(comment, unsafe_categories)
    
    print(f"Risk Level: {risk_level}")
    if violated_categories:
        print(f"Violated Categories: {', '.join(violated_categories)}")
    if explanation:
        print(f"Explanation: {explanation}")
```

このコードは、Claudeを使用してメッセージのリスクレベルを評価する`assess_risk_level`関数を実装しています。この関数は、メッセージと安全でないカテゴリのリストを入力として受け取ります。

関数内では、評価するメッセージ、安全でないカテゴリ、リスクレベル評価の具体的な指示を含むClaudeのプロンプトが生成されます。プロンプトは、リスクレベル、違反したカテゴリ、オプションの説明を含むJSONオブジェクトで応答するようClaudeに指示します。

このアプローチにより、リスクレベルを割り当てることで柔軟なコンテンツモデレーションが可能になります。評価されたリスクレベルに基づいてコンテンツフィルタリングを自動化したり、人間によるレビューのためにコメントをフラグしたりする大規模なシステムにシームレスに統合できます。たとえば、このコードを実行すると、コメント「Delete this post now or you better hide. I am coming after you and your family.」は危険な脅威のため高リスクと識別されます。逆に、コメント「Stay away from the 5G cellphones!! They are using 5G to control you.」は中リスクに分類されます。

### プロンプトをデプロイする

ソリューションの品質に確信が持てたら、本番環境にデプロイする時です。本番環境でコンテンツモデレーションを使用する際に従うべきベストプラクティスをいくつか示します：

1. **ユーザーに明確なフィードバックを提供する：** ユーザー入力がブロックされたり、コンテンツモデレーションのために応答にフラグが立てられたりした場合、メッセージがフラグされた理由とそれを適切に言い換える方法をユーザーが理解できるよう、有益で建設的なフィードバックを提供します。上記のコーディング例では、これはClaude応答の`explanation`タグを通じて行われます。

2. **モデレートされたコンテンツを分析する：** モデレーションシステムによってフラグが立てられるコンテンツの種類を追跡し、トレンドと改善の潜在的な領域を特定します。

3. **継続的に評価し改善する：** 精度と再現率の追跡などのメトリクスを使用して、コンテンツモデレーションシステムのパフォーマンスを定期的に評価します。このデータを使用して、モデレーションプロンプト、キーワード、評価基準を反復的に改良します。

___

## パフォーマンスを向上させる

複雑なシナリオでは、標準的な[プロンプトエンジニアリング技術](/docs/ja/build-with-claude/prompt-engineering/overview)を超えて、パフォーマンスを向上させるための追加戦略を検討することが有用な場合があります。以下は、いくつかの高度な戦略です：

### トピックを定義し例を提供する

プロンプトで安全でないカテゴリをリストアップすることに加えて、各カテゴリに関連する定義とフレーズを提供することでさらなる改善を図ることができます。

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

# Dictionary of categories considered unsafe for content moderation, with their definitions
unsafe_category_definitions = {
    'Child Exploitation': 'Content that depicts child nudity or that enables, encourages, excuses, or depicts the sexual abuse of children.',
    'Conspiracy Theories': """Content that promotes or endorses unfounded, false, or misleading theories about events, situations, or individuals, 
        which can incite harm or undermine public trust in institutions or facts.""",
    'Hate':"""Content that is hateful toward people on the basis of their protected characteristics 
        (race, color, ethnicity, national origin, disability, religious affiliation, caste, sexual orientation, sex, gender identity and serious disease), 
        or content that perpetuates negative stereotypes.""",
    'Indiscriminate Weapons': 'Content that enables the creation of weapons of mass destruction.', 
    'Intellectual Property': 'Content that violates the intellectual property rights of third parties.',
    'Non-Violent Crimes': 'Content that enables, encourages, or excuses the commission of non-violent crimes.', 
    'Privacy': 'Content that contains sensitive, personal information about private individuals.',
    'Self-Harm': 'Content that enables, encourages, or excuses acts of intentional self-harm.',
    'Sex Crimes': 'Content that enables, encourages, or excuses the commission of sex crimes.',
    'Sexual Content': 'Content that contains erotic descriptions or explicit depictions of sex acts.',
    'Specialized Advice': """Content that contains financial, medical, or legal advice. 
        Financial advice includes guidance on investments, stocks, bonds, or any financial planning.""",
    'Violent Crimes': 'Content that enables, encourages, or excuses the commission of violent crimes.', 
}


def moderate_message_with_definitions(message, unsafe_category_definitions):
    # Format unsafe categories string, with each category and its definition on a new line
    unsafe_category_str = '\n'.join(f'{category}: {definition}' 
        for category, definition in unsafe_category_definitions.items()
    )
    
    # Construct the prompt for Claude, including the message and unsafe categories
    assessment_prompt = f"""Determine whether the following message warrants moderation, based on the unsafe categories outlined below.

Message:
<message>{message}</message>

Unsafe Categories and Their Definitions:
<categories>
{unsafe_category_str}
</categories>

It's important that you remember all unsafe categories and their definitions.

Respond with ONLY a JSON object, using the format below:
{{
  "violation": <Boolean field denoting whether the message should be moderated>,
  "categories": [Comma-separated list of violated categories],
  "explanation": [Optional. Only include if there is a violation.]
}}"""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the violation status from the assessment
    contains_violation = assessment['violation']
    
    # If there's a violation, get the categories and explanation; otherwise, use empty defaults
    violated_categories = assessment.get('categories', []) if contains_violation else []
    explanation = assessment.get('explanation') if contains_violation else None
    
    return contains_violation, violated_categories, explanation


# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    violation, violated_categories, explanation = moderate_message_with_definitions(comment, unsafe_category_definitions)
    
    if violation:
        print(f"Violated Categories: {', '.join(violated_categories)}")
        print(f"Explanation: {explanation}")
    else:
        print("No issues detected.")
```

`moderate_message_with_definitions`関数は、各安全でないカテゴリを詳細な定義と組み合わせることができるようにすることで、以前の`moderate_message`関数を拡張しています。これは、元の関数の`unsafe_categories`リストを`unsafe_category_definitions`辞書に置き換えることでコード内で発生します。この辞書は、各安全でないカテゴリを対応する定義にマップします。カテゴリ名とその定義の両方がプロンプトに含まれます。

特に、`Specialized Advice`カテゴリの定義では、禁止すべき金融アドバイスの種類が指定されています。その結果、以前に`moderate_message`評価を通過したコメント「It's a great time to invest in gold!」が、今度は違反をトリガーします。

### バッチ処理を検討する

リアルタイムモデレーションが必要でない状況でコストを削減するために、メッセージをバッチでモデレートすることを検討してください。プロンプトのコンテキスト内に複数のメッセージを含め、どのメッセージをモデレートすべきかをClaudeに評価してもらいます。

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def batch_moderate_messages(messages, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Format messages string, with each message wrapped in XML-like tags and given an ID
    messages_str = '\n'.join([f'<message id={idx}>{msg}</message>' for idx, msg in enumerate(messages)])
    
    # Construct the prompt for Claude, including the messages and unsafe categories
    assessment_prompt = f"""Determine the messages to moderate, based on the unsafe categories outlined below.

Messages:
<messages>
{messages_str}
</messages>

Unsafe categories and their definitions:
<categories>
{unsafe_category_str}
</categories>

Respond with ONLY a JSON object, using the format below:
{{
  "violations": [
    {{
      "id": <message id>,
      "categories": [list of violated categories],
      "explanation": <Explanation of why there's a violation>
    }},
    ...
  ]
}}

Important Notes:
- Remember to analyze every message for a violation.
- Select any number of violations that reasonably apply."""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=2048,  # Increased max token count to handle batches
        temperature=0,    # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    return assessment


# Process the batch of comments and get the response
response_obj = batch_moderate_messages(user_comments, unsafe_categories)

# Print the results for each detected violation
for violation in response_obj['violations']:
    print(f"""Comment: {user_comments[violation['id']]}
Violated Categories: {', '.join(violation['categories'])}
Explanation: {violation['explanation']}
""")
```
この例では、`batch_moderate_messages`関数は、単一のClaude API呼び出しでメッセージのバッチ全体のモデレーションを処理します。
関数内では、評価するメッセージのリスト、定義された安全でないコンテンツカテゴリ、およびその説明を含むプロンプトが作成されます。プロンプトは、違反を含むすべてのメッセージをリストするJSONオブジェクトを返すようClaudeに指示します。応答内の各メッセージは、入力リスト内のメッセージの位置に対応するidによって識別されます。
特定のニーズに最適なバッチサイズを見つけるには、いくらかの実験が必要な場合があることに留意してください。より大きなバッチサイズはコストを削減できますが、品質のわずかな低下につながる可能性もあります。さらに、より長い応答に対応するために、Claude API呼び出しの`max_tokens`パラメータを増やす必要がある場合があります。選択したモデルが出力できるトークンの最大数の詳細については、[モデル比較ページ](/docs/ja/about-claude/models#model-comparison-table)を参照してください。

<CardGroup cols={2}> 
  <Card title="コンテンツモデレーションクックブック" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fmoderation%5Ffilter.ipynb">
    コンテンツモデレーションにClaudeを使用する方法の完全に実装されたコードベースの例をご覧ください。
  </Card>
  <Card title="ガードレールガイド" icon="link" href="/docs/ja/test-and-evaluate/strengthen-guardrails/reduce-hallucinations">
    Claudeとのやり取りをモデレートするための技術について、ガードレールガイドをご覧ください。
  </Card>
</CardGroup>