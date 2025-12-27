# 法的文書の要約

このガイドでは、Claudeの高度な自然言語処理機能を活用して、法的文書を効率的に要約し、重要な情報を抽出し、法的調査を迅速化する方法について説明します。Claudeを使用することで、契約書、訴訟準備、規制業務のレビューを合理化し、時間を節約し、法的プロセスの正確性を確保できます。

---

> 当社の[要約クックブック](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb)にアクセスして、Claudeを使用した法的文書要約の実装例を確認してください。

## Claudeで構築する前に

### 法的文書の要約にClaudeを使用するかどうかを決定する

Claudeのようなエンタープライズ向けLLMを使用して法的文書を要約すべき主な指標を以下に示します：

<section title="大量の文書を効率的かつ手頃な価格でレビューしたい">
大規模な文書レビューは、手動で行う場合、時間がかかり、費用がかかる可能性があります。Claudeは、大量の法的文書を迅速に処理および要約でき、文書レビューに関連する時間と費用を大幅に削減できます。この機能は、デューデリジェンス、契約分析、訴訟上の開示など、効率が重要なタスクに特に価値があります。
</section>
<section title="重要なメタデータの自動抽出が必要">
Claudeは、関係者、日付、契約条件、特定の条項など、法的文書から重要なメタデータを効率的に抽出および分類できます。この自動抽出により、情報を整理し、大規模な文書セットの検索、分析、管理を容易にできます。これは、契約管理、コンプライアンスチェック、または法的情報の検索可能なデータベースの作成に特に役立ちます。
</section>
<section title="明確で簡潔で標準化された要約を生成したい">
Claudeは、事前に決定された形式に従う構造化された要約を生成でき、法的専門家がさまざまな文書の重要なポイントを素早く把握しやすくなります。これらの標準化された要約は、読みやすさを向上させ、文書間の比較を容易にし、特に複雑な法的言語や技術用語を扱う場合に、全体的な理解を向上させることができます。
</section>
<section title="要約の正確な引用が必要">
法的要約を作成する場合、信頼性と法的基準への準拠を確保するために、適切な帰属と引用が重要です。Claudeは、参照されるすべての法的ポイントに対して正確な引用を含めるようにプロンプトを指定でき、法的専門家が要約された情報をレビューおよび検証しやすくなります。
</section>
<section title="法的調査プロセスを合理化および迅速化したい">
Claudeは、判例法、法令、法的解説の大量を迅速に分析することで、法的調査を支援できます。関連する先例を特定し、重要な法的原則を抽出し、複雑な法的議論を要約できます。この機能により、調査プロセスを大幅に加速でき、法的専門家がより高度な分析と戦略開発に集中できるようになります。
</section>

### 要約で抽出する詳細を決定する
特定の文書に対して、単一の正しい要約は存在しません。明確な指示がないと、Claudeが含める詳細を決定するのが難しい場合があります。最適な結果を得るには、要約に含める特定の情報を特定してください。

たとえば、サブリース契約を要約する場合、次の重要なポイントを抽出したいかもしれません：

```python
details_to_extract = [
    'Parties involved (sublessor, sublessee, and original lessor)',
    'Property details (address, description, and permitted use)', 
    'Term and rent (start date, end date, monthly rent, and security deposit)',
    'Responsibilities (utilities, maintenance, and repairs)',
    'Consent and notices (landlord\'s consent, and notice requirements)',
    'Special provisions (furniture, parking, and subletting restrictions)'
]
```

### 成功基準を確立する

要約の品質を評価することは、悪名高く困難なタスクです。他の多くの自然言語処理タスクとは異なり、要約の評価には、明確で客観的なメトリクスが不足していることがよくあります。このプロセスは非常に主観的であり、異なる読者が要約のさまざまな側面を重視します。Claudeが法的要約をどの程度うまく実行しているかを評価する際に検討したい基準を以下に示します。

<section title="事実の正確性">
要約は、文書内の事実、法的概念、および重要なポイントを正確に表現する必要があります。
</section>
<section title="法的精度">
用語および法令、判例法、または規制への参照は、正確であり、法的基準に準拠している必要があります。
</section>
<section title="簡潔性">
要約は、重要な詳細を失わずに、法的文書を本質的なポイントに凝縮する必要があります。
</section>
<section title="一貫性">
複数の文書を要約する場合、LLMは各要約に対して一貫した構造とアプローチを維持する必要があります。
</section>
<section title="読みやすさ">
テキストは明確で理解しやすい必要があります。対象者が法的専門家でない場合、要約には対象者を混乱させる可能性のある法的用語を含めるべきではありません。
</section>
<section title="バイアスと公平性">
要約は、法的議論と立場の不偏で公平な描写を提示する必要があります。
</section>

成功基準の確立に関する詳細については、当社の[ガイド](/docs/ja/test-and-evaluate/define-success)を参照してください。

---

## Claudeを使用して法的文書を要約する方法

### 適切なClaudeモデルを選択する

モデルの精度は、法的文書を要約する際に非常に重要です。Claude Sonnet 4.5は、高い精度が必要なこのようなユースケースに最適な選択肢です。文書のサイズと数が大きく、コストが懸念事項になり始めた場合は、Claude Haiku 4.5のような小さいモデルを試すこともできます。

これらのコストを見積もるのに役立つように、以下はSonnetとHaikuの両方を使用して1,000のサブリース契約を要約するコストの比較です：

* **コンテンツサイズ**
    * 契約数：1,000
    * 契約あたりの文字数：300,000
    * 総文字数：300M

* **推定トークン**
    * 入力トークン：86M（1トークンあたり3.5文字と仮定）
    * 要約あたりの出力トークン：350
    * 総出力トークン：350,000
 
* **Claude Sonnet 4.5推定コスト**
    * 入力トークンコスト：86 MTok * \$3.00/MTok = \$258
    * 出力トークンコスト：0.35 MTok * \$15.00/MTok = \$5.25
    * 総コスト：\$258.00 + \$5.25 = \$263.25

* **Claude Haiku 3推定コスト**
    * 入力トークンコスト：86 MTok * \$0.25/MTok = \$21.50
    * 出力トークンコスト：0.35 MTok * \$1.25/MTok = \$0.44
    * 総コスト：\$21.50 + \$0.44 = \$21.96

<Tip>実際のコストはこれらの見積もりと異なる場合があります。これらの見積もりは、[プロンプト](#build-a-strong-prompt)セクションで強調されている例に基づいています。</Tip>

### 文書をClaudeが処理できる形式に変換する

文書の要約を開始する前に、データを準備する必要があります。これには、PDFからテキストを抽出し、テキストをクリーンアップし、Claudeで処理できる状態にすることが含まれます。

サンプルPDFでこのプロセスのデモンストレーションを以下に示します：

```python
from io import BytesIO
import re

import pypdf
import requests

def get_llm_text(pdf_file):
    reader = pypdf.PdfReader(pdf_file)
    text = "\n".join([page.extract_text() for page in reader.pages])

    # Remove extra whitespace
    text = re.sub(r'\s+', ' ', text) 

    # Remove page numbers
    text = re.sub(r'\n\s*\d+\s*\n', '\n', text) 

    return text


# Create the full URL from the GitHub repository
url = "https://raw.githubusercontent.com/anthropics/anthropic-cookbook/main/skills/summarization/data/Sample Sublease Agreement.pdf"
url = url.replace(" ", "%20")

# Download the PDF file into memory
response = requests.get(url)

# Load the PDF from memory
pdf_file = BytesIO(response.content)

document_text = get_llm_text(pdf_file) 
print(document_text[:50000]) 
```

この例では、まず[要約クックブック](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/data/Sample%20Sublease%20Agreement.pdf)で使用されるサンプルサブリース契約のPDFをダウンロードします。この契約は、[sec.govウェブサイト](https://www.sec.gov/Archives/edgar/data/1045425/000119312507044370/dex1032.htm)から入手可能な公開サブリース契約から取得されました。

pypdfライブラリを使用してPDFの内容を抽出し、テキストに変換します。その後、テキストデータは、余分な空白とページ番号を削除することでクリーンアップされます。

### 強力なプロンプトを構築する

Claudeはさまざまな要約スタイルに適応できます。プロンプトの詳細を変更して、Claudeをより詳細または簡潔にしたり、より多くまたはより少ない技術用語を含めたり、手元のコンテキストのより高いまたはより低いレベルの要約を提供したりするようにガイドできます。

サブリース契約を分析する際に、生成された要約が一貫した構造に従うようにするプロンプトを作成する方法の例を以下に示します：

```python
import anthropic

# Initialize the Anthropic client
client = anthropic.Anthropic()

def summarize_document(text, details_to_extract, model="claude-sonnet-4-5", max_tokens=1000):

    # Format the details to extract to be placed within the prompt's context
    details_to_extract_str = '\n'.join(details_to_extract)
    
    # Prompt the model to summarize the sublease agreement
    prompt = f"""Summarize the following sublease agreement. Focus on these key aspects:

    {details_to_extract_str}

    Provide the summary in bullet points nested within the XML header for each section. For example:

    <parties involved>
    - Sublessor: [Name]
    // Add more details as needed
    </parties involved>
    
    If any information is not explicitly stated in the document, note it as "Not specified". Do not preamble.

    Sublease agreement text:
    {text}
    """

    response = client.messages.create(
        model=model,
        max_tokens=max_tokens,
        system="You are a legal analyst specializing in real estate law, known for highly accurate and detailed summaries of sublease agreements.",
        messages=[
            {"role": "user", "content": prompt},
            {"role": "assistant", "content": "Here is the summary of the sublease agreement: <summary>"}
        ],
        stop_sequences=["</summary>"]
    )

    return response.content[0].text

sublease_summary = summarize_document(document_text, details_to_extract)
print(sublease_summary)
```

このコードは、Claudeを使用してサブリース契約の内容を要約する`summarize_document`関数を実装しています。この関数は、テキスト文字列と抽出する詳細のリストを入力として受け入れます。この例では、前のコードスニペットで定義された`document_text`および`details_to_extract`変数を使用して関数を呼び出します。

関数内では、要約する文書、抽出する詳細、および文書を要約するための特定の指示を含む、Claudeのプロンプトが生成されます。プロンプトは、Claudeに、XMLヘッダー内にネストされた各詳細の要約で応答するよう指示します。

要約の各セクションをタグ内に出力することにしたため、各セクションは後処理ステップとして簡単に解析できます。このアプローチにより、ユースケースに適応できる構造化された要約が可能になり、各要約が同じパターンに従うようになります。

### プロンプトを評価する

プロンプトは、本番環境で使用できるようにするために、テストと最適化が必要なことがよくあります。ソリューションの準備状況を判断するには、定義した成功基準に基づいて、定量的および定性的な方法を組み合わせた体系的なプロセスを使用して、要約の品質を評価してください。定義した成功基準に基づいた[強力な経験的評価](/docs/ja/test-and-evaluate/develop-tests#building-evals-and-test-cases)を作成することで、プロンプトを最適化できます。経験的評価に含めたいメトリクスの例を以下に示します：

<section title="ROUGEスコア">
これは、生成された要約と専門家が作成した参照要約との間のオーバーラップを測定します。このメトリクスは主に再現率に焦点を当てており、コンテンツカバレッジを評価するのに役立ちます。
</section>
<section title="BLEUスコア">
もともと機械翻訳用に開発されたこのメトリクスは、要約タスクに適応させることができます。BLEUスコアは、生成された要約と参照要約の間のn-gramマッチの精度を測定します。スコアが高いほど、生成された要約に参照要約と同様のフレーズと用語が含まれていることを示します。
</section>
<section title="文脈的埋め込み類似性">
このメトリクスには、生成された要約と参照要約の両方のベクトル表現（埋め込み）を作成することが含まれます。その後、これらの埋め込み間の類似性が計算され、多くの場合、コサイン類似性を使用します。類似性スコアが高いほど、生成された要約が参照要約の意味的意味とコンテキストをキャプチャしていることを示します。正確な表現は異なる場合があります。
</section>
<section title="LLMベースのグレーディング">
この方法には、Claudeなどのエンタープライズ向けLLMを使用して、スコアリングルーブリックに対して生成された要約の品質を評価することが含まれます。ルーブリックは特定のニーズに合わせてカスタマイズでき、精度、完全性、一貫性などの重要な要因を評価できます。LLMベースのグレーディングの実装に関するガイダンスについては、これらの[ヒント](/docs/ja/test-and-evaluate/develop-tests#tips-for-llm-based-grading)を参照してください。
</section>
<section title="人間による評価">
参照要約の作成に加えて、法的専門家は生成された要約の品質も評価できます。これは大規模では費用がかかり、時間がかかりますが、本番環境に展開する前に、いくつかの要約に対してサニティチェックとして実行されることがよくあります。
</section>

### プロンプトを展開する

本番環境にソリューションを展開する際に留意すべき追加の考慮事項を以下に示します。

1. **責任を確保する：** 要約のエラーの法的影響を理解してください。これは、組織またはクライアントに対する法的責任につながる可能性があります。要約がAIによって生成されており、法的専門家によってレビューされるべきであることを明確にする免責事項または法的通知を提供してください。

2. **多様な文書タイプを処理する：** このガイドでは、PDFからテキストを抽出する方法について説明しました。実際には、文書はさまざまな形式（PDF、Word文書、テキストファイルなど）で提供される場合があります。データ抽出パイプラインが、受け取ることが予想されるすべてのファイル形式を変換できることを確認してください。

3. **Claudeへのエンタープライズ向けAPI呼び出しを並列化する：** トークン数が多い長い文書では、Claudeが要約を生成するのに最大1分かかる場合があります。大規模な文書コレクションの場合、要約を合理的な時間枠で完了できるように、Claudeへのエンタープライズ向けAPI呼び出しを並列で送信することをお勧めします。Anthropicの[レート制限](/docs/ja/api/rate-limits#rate-limits)を参照して、並列で実行できるエンタープライズ向けAPI呼び出しの最大数を決定してください。

---

## パフォーマンスを向上させる

複雑なシナリオでは、標準的な[プロンプトエンジニアリング技術](/docs/ja/build-with-claude/prompt-engineering/overview)を超えてパフォーマンスを向上させるための追加戦略を検討することが役立つ場合があります。以下は、いくつかの高度な戦略です：

### メタ要約を実行して長い文書を要約する

法的要約には、長い文書または多くの関連文書を一度に処理することが含まれることが多く、Claudeのコンテキストウィンドウを超える場合があります。メタ要約として知られるチャンキング方法を使用して、このユースケースを処理できます。この技術には、文書を小さく管理可能なチャンクに分割し、各チャンクを個別に処理することが含まれます。その後、各チャンクの要約を組み合わせて、文書全体のメタ要約を作成できます。

メタ要約を実行する方法の例を以下に示します：

```python
import anthropic

# Initialize the Anthropic client
client = anthropic.Anthropic()

def chunk_text(text, chunk_size=20000):
    return [text[i:i+chunk_size] for i in range(0, len(text), chunk_size)]

def summarize_long_document(text, details_to_extract, model="claude-sonnet-4-5", max_tokens=1000):

    # Format the details to extract to be placed within the prompt's context
    details_to_extract_str = '\n'.join(details_to_extract)

    # Iterate over chunks and summarize each one
    chunk_summaries = [summarize_document(chunk, details_to_extract, model=model, max_tokens=max_tokens) for chunk in chunk_text(text)]
    
    final_summary_prompt = f"""
    
    You are looking at the chunked summaries of multiple documents that are all related. 
    Combine the following summaries of the document from different truthful sources into a coherent overall summary:

    <chunked_summaries>
    {"".join(chunk_summaries)}
    </chunked_summaries>

    Focus on these key aspects:
    {details_to_extract_str})

    Provide the summary in bullet points nested within the XML header for each section. For example:

    <parties involved>
    - Sublessor: [Name]
    // Add more details as needed
    </parties involved>
    
    If any information is not explicitly stated in the document, note it as "Not specified". Do not preamble.
    """

    response = client.messages.create(
        model=model,
        max_tokens=max_tokens,
        system="You are a legal expert that summarizes notes on one document.",
        messages=[
            {"role": "user",  "content": final_summary_prompt},
            {"role": "assistant", "content": "Here is the summary of the sublease agreement: <summary>"}

        ],
        stop_sequences=["</summary>"]
    )
    
    return response.content[0].text

long_summary = summarize_long_document(document_text, details_to_extract)
print(long_summary)
```

`summarize_long_document`関数は、文書を小さなチャンクに分割し、各チャンクを個別に要約することで、前の`summarize_document`関数に基づいています。

このコードは、元の文書内の20,000文字の各チャンクに`summarize_document`関数を適用することで、これを実現します。個別の要約が組み合わされ、これらのチャンク要約から最終要約が作成されます。

`summarize_long_document`関数は、例のPDFでは厳密には必要ありません。これは、文書全体がClaudeのコンテキストウィンドウ内に収まるためです。ただし、Claudeのコンテキストウィンドウを超える文書を処理する場合、または複数の関連文書を一度に要約する場合に不可欠になります。いずれにせよ、このメタ要約技術は、以前の単一要約アプローチで見落とされた最終要約に追加の重要な詳細をキャプチャすることがよくあります。

### 要約インデックス付き文書を使用して大規模な文書コレクションを探索する

LLMを使用した文書コレクションの検索には、通常、検索拡張生成（RAG）が含まれます。ただし、大規模な文書を含むシナリオ、または正確な情報検索が重要な場合、基本的なRAGアプローチは不十分な場合があります。要約インデックス付き文書は、高度なRAGアプローチであり、従来のRAG方法よりも少ないコンテキストを使用して、検索用の文書をランク付けするより効率的な方法を提供します。このアプローチでは、まずClaudeを使用してコーパス内の各文書の簡潔な要約を生成し、次にCladeを使用して、質問されているクエリに対する各要約の関連性をランク付けします。このアプローチの詳細と、コードベースの例については、[要約クックブック](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb)の要約インデックス付き文書セクションを確認してください。

### Claudeを微調整してデータセットから学習する

Claudeの要約生成能力を向上させるためのもう1つの高度な技術は、微調整です。微調整には、法的要約のニーズに特に合わせたカスタムデータセットでClaudeをトレーニングすることが含まれ、Claudeがユースケースに適応することを確保します。微調整を実行する方法の概要を以下に示します：

1. **エラーを特定する：** Claudeの要約が不足している場合を収集することから始めます。これには、重要な法的詳細の欠落、コンテキストの誤解、または不適切な法的用語の使用が含まれる可能性があります。

2. **データセットをキュレーションする：** これらの問題を特定したら、これらの問題のある例のデータセットをコンパイルしてください。このデータセットには、元の法的文書と修正された要約が含まれ、Claudeが望ましい動作を学習することを確保してください。

3. **微調整を実行する：** 微調整には、キュレーションされたデータセットでモデルを再トレーニングして、その重みとパラメータを調整することが含まれます。この再トレーニングにより、Claudeは法的ドメインの特定の要件をより良く理解し、標準に従って文書を要約する能力を向上させます。

4. **反復的な改善：** 微調整は1回限りのプロセスではありません。Claudeが要約を生成し続けるにつれて、パフォーマンスが低下した新しい例を反復的に追加し、その機能をさらに改善できます。時間が経つにつれて、この継続的なフィードバックループにより、法的要約タスク用に高度に特化したモデルが生成されます。

<Tip>微調整は現在、Amazon Bedrockを介してのみ利用可能です。追加の詳細は、[AWSローンチブログ](https://aws.amazon.com/blogs/machine-learning/fine-tune-anthropics-claude-3-haiku-in-amazon-bedrock-to-boost-model-accuracy-and-quality/)で利用可能です。</Tip>

<CardGroup cols={2}> 
  <Card title="要約クックブック" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb">
    Claudeを使用して契約を要約する方法の完全に実装されたコードベースの例を表示します。
  </Card>
  <Card title="引用クックブック" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb">
    情報の正確性と説明可能性を確保する方法に関するガイダンスについては、当社の引用クックブックレシピを探索してください。
  </Card>
</CardGroup>