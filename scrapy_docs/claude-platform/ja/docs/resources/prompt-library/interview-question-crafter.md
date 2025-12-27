# 面接質問作成者

面接用の質問を生成します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、与えられたコンテキストに基づいて、面接用の思慮深いオープンエンドな質問シリーズを生成することです。質問は面接対象者から洞察に富んだ詳細な回答を引き出すように設計され、彼らの知識、経験、批判的思考スキルを披露できるようにする必要があります。はい/いいえの質問や明らかな答えがある質問は避けてください。代わりに、内省、自己評価、具体的な例や逸話の共有を促す質問に焦点を当ててください。 |
| User   | あなたは確立されたeコマース企業でマーケティングマネージャーのポジションの候補者を面接しています。理想的な候補者は、マルチチャネルマーケティングキャンペーンの開発と実行の経験、強力な分析スキル、機能横断チームとの効果的な協力能力を持っている必要があります。また、最新のマーケティングトレンドとテクノロジーについて常に最新情報を把握することに情熱を持っている必要があります。 |

## 出力例

1.  以前の役職で開発・実行した成功したマルチチャネルマーケティングキャンペーンについて説明していただけますか？直面した課題と、それをどのように克服したかを教えてください。
2.  マーケティングキャンペーンの成功をどのように測定し、どの指標に焦点を当てていますか？データと分析を使用してキャンペーンのパフォーマンスを最適化した例を提供していただけますか？
3.  あなたの経験では、成功するeコマースマーケティング戦略の重要な要素は何ですか？戦略が会社の全体的なビジネス目標と一致することをどのように確保していますか？
4.  マーケティング目標を達成するために機能横断チーム（例：製品、営業、カスタマーサービス）と協力しなければならなかった時の例を共有していただけますか？どのような課題に遭遇し、効果的な協力をどのように促進しましたか？
5.  最新のマーケティングトレンドとテクノロジーについてどのように情報を得ており、この知識をマーケティング戦略やキャンペーンの改善にどのように適用しましたか？
6.  関わったマーケティングキャンペーンが期待通りに機能しなかった時について説明していただけますか？その経験から何を学び、その教訓を将来のキャンペーンにどのように適用しましたか？
7.  当社はイノベーションと継続的改善を重視しています。チームのパフォーマンスや結果に大きな影響を与えた新しいマーケティングアプローチ、ツール、またはテクノロジーを導入した時の例を共有していただけますか？

---

## APIリクエスト

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic(
  # defaults to os.environ.get("ANTHROPIC_API_KEY")
  api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=1000,
  temperature=0.5,
  system="あなたのタスクは、与えられたコンテキストに基づいて、面接用の思慮深いオープンエンドな質問シリーズを生成することです。質問は面接対象者から洞察に富んだ詳細な回答を引き出すように設計され、彼らの知識、経験、批判的思考スキルを披露できるようにする必要があります。はい/いいえの質問や明らかな答えがある質問は避けてください。代わりに、内省、自己評価、具体的な例や逸話の共有を促す質問に焦点を当ててください。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "あなたは確立されたeコマース企業でマーケティングマネージャーのポジションの候補者を面接しています。理想的な候補者は、マルチチャネルマーケティングキャンペーンの開発と実行の経験、強力な分析スキル、機能横断チームとの効果的な協力能力を持っている必要があります。また、最新のマーケティングトレンドとテクノロジーについて常に最新情報を把握することに情熱を持っている必要があります。"
        }
      ]
    }
  ]
)
print(message.content)

```

</Tab>
<Tab title="TypeScript">

```typescript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic({
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0.5,
  system: "あなたのタスクは、与えられたコンテキストに基づいて、面接用の思慮深いオープンエンドな質問シリーズを生成することです。質問は面接対象者から洞察に富んだ詳細な回答を引き出すように設計され、彼らの知識、経験、批判的思考スキルを披露できるようにする必要があります。はい/いいえの質問や明らかな答えがある質問は避けてください。代わりに、内省、自己評価、具体的な例や逸話の共有を促す質問に焦点を当ててください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "あなたは確立されたeコマース企業でマーケティングマネージャーのポジションの候補者を面接しています。理想的な候補者は、マルチチャネルマーケティングキャンペーンの開発と実行の経験、強力な分析スキル、機能横断チームとの効果的な協力能力を持っている必要があります。また、最新のマーケティングトレンドとテクノロジーについて常に最新情報を把握することに情熱を持っている必要があります。"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">

```python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=1000,
    temperature=0.5,
    system="あなたのタスクは、与えられたコンテキストに基づいて、面接用の思慮深いオープンエンドな質問シリーズを生成することです。質問は面接対象者から洞察に富んだ詳細な回答を引き出すように設計され、彼らの知識、経験、批判的思考スキルを披露できるようにする必要があります。はい/いいえの質問や明らかな答えがある質問は避けてください。代わりに、内省、自己評価、具体的な例や逸話の共有を促す質問に焦点を当ててください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "あなたは確立されたeコマース企業でマーケティングマネージャーのポジションの候補者を面接しています。理想的な候補者は、マルチチャネルマーケティングキャンペーンの開発と実行の経験、強力な分析スキル、機能横断チームとの効果的な協力能力を持っている必要があります。また、最新のマーケティングトレンドとテクノロジーについて常に最新情報を把握することに情熱を持っている必要があります。"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="AWS Bedrock TypeScript">
```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens: 1000,
temperature: 0.5,
system: "あなたのタスクは、与えられたコンテキストに基づいて、面接用の思慮深いオープンエンドな質問シリーズを生成することです。質問は面接対象者から洞察に富んだ詳細な回答を引き出すように設計され、彼らの知識、経験、批判的思考スキルを披露できるようにする必要があります。はい/いいえの質問や明らかな答えがある質問は避けてください。代わりに、内省、自己評価、具体的な例や逸話の共有を促す質問に焦点を当ててください。",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "あなたは確立されたeコマース企業でマーケティングマネージャーのポジションの候補者を面接しています。理想的な候補者は、マルチチャネルマーケティングキャンペーンの開発と実行の経験、強力な分析スキル、機能横断チームとの効果的な協力能力を持っている必要があります。また、最新のマーケティングトレンドとテクノロジーについて常に最新情報を把握することに情熱を持っている必要があります。"
}
]
}
]
});
console.log(msg);

````
</Tab>
<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=1000,
    temperature=0.5,
    system="あなたのタスクは、与えられたコンテキストに基づいて、面接用の思慮深いオープンエンドな質問シリーズを生成することです。質問は面接対象者から洞察に富んだ詳細な回答を引き出すように設計され、彼らの知識、経験、批判的思考スキルを披露できるようにする必要があります。はい/いいえの質問や明らかな答えがある質問は避けてください。代わりに、内省、自己評価、具体的な例や逸話の共有を促す質問に焦点を当ててください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "あなたは確立されたeコマース企業でマーケティングマネージャーのポジションの候補者を面接しています。理想的な候補者は、マルチチャネルマーケティングキャンペーンの開発と実行の経験、強力な分析スキル、機能横断チームとの効果的な協力能力を持っている必要があります。また、最新のマーケティングトレンドとテクノロジーについて常に最新情報を把握することに情熱を持っている必要があります。"
                }
            ]
        }
    ]
)
print(message.content)

````

</Tab>
<Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 0.5,
  system: "あなたのタスクは、与えられたコンテキストに基づいて、面接用の思慮深いオープンエンドな質問シリーズを生成することです。質問は面接対象者から洞察に富んだ詳細な回答を引き出すように設計され、彼らの知識、経験、批判的思考スキルを披露できるようにする必要があります。はい/いいえの質問や明らかな答えがある質問は避けてください。代わりに、内省、自己評価、具体的な例や逸話の共有を促す質問に焦点を当ててください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "あなたは確立されたeコマース企業でマーケティングマネージャーのポジションの候補者を面接しています。理想的な候補者は、マルチチャネルマーケティングキャンペーンの開発と実行の経験、強力な分析スキル、機能横断チームとの効果的な協力能力を持っている必要があります。また、最新のマーケティングトレンドとテクノロジーについて常に最新情報を把握することに情熱を持っている必要があります。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>