# 料理クリエイター

ユーザーの利用可能な食材と食事の好みに基づいてレシピのアイデアを提案します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、ユーザーが入力した利用可能な食材と食事の好みに基づいて、パーソナライズされたレシピのアイデアを生成することです。この情報を使用して、与えられた食材を使って作ることができ、言及された食事のニーズがあればそれに対応する、創造的で美味しいレシピの多様性を提案してください。各レシピについて、簡潔な説明、必要な食材のリスト、簡単な手順を提供してください。レシピは従いやすく、栄養価が高く、最小限の追加食材や器具で準備できることを確認してください。 |
| User   | 利用可能な食材：鶏胸肉、ブロッコリー、人参、玉ねぎ、にんにく、オリーブオイル、米、グルテンフリー醤油、はちみつ <br/> 食事の好み：グルテンフリー |

## 出力例

提案レシピ：

はちみつガーリックチキン炒め

説明：柔らかい鶏胸肉、シャキシャキの野菜、甘くて香ばしいはちみつガーリックソースを特徴とする、手早く風味豊かな炒め物です。  
食材：

- 鶏胸肉2枚、一口大に切る
- ブロッコリーの房2カップ
- 人参のスライス1カップ
- 玉ねぎ1個、みじん切り
- にんにく3片、みじん切り
- オリーブオイル大さじ2
- 醤油（グルテンフリー）1/4カップ
- はちみつ大さじ2

手順：

1.  大きなフライパンまたは中華鍋で、オリーブオイルを中強火で熱します。
2.  みじん切りの玉ねぎとにんにくを加え、香りが立つまで約2分炒めます。
3.  鶏肉を加え、ピンク色でなくなるまで約5-7分調理します。
4.  ブロッコリーと人参を加え、野菜がシャキシャキになるまで約5分炒めます。
5.  小さなボウルで、醤油とはちみつを混ぜ合わせます。
6.  ソースを鶏肉と野菜にかけ、均等にコーティングするように混ぜます。
7.  ソースが少しとろみがつくまで、さらに2-3分調理します。
8.  蒸した米と一緒に熱いうちに提供します。

チキンと野菜のライスボウル

説明：味付けした鶏肉、ローストした野菜、醤油のドリズルをトッピングした、ボリューム満点で栄養価の高いライスボウルです。

食材：

- 鶏胸肉2枚、一口大に切る
- ブロッコリーの房2カップ
- 人参のスライス1カップ
- 玉ねぎ1個、みじん切り
- にんにく2片、みじん切り
- オリーブオイル大さじ2
- 醤油（グルテンフリー）1/4カップ
- 調理済み米2カップ

手順：

1.  オーブンを400°F（200°C）に予熱します。
2.  大きなボウルで、鶏肉、ブロッコリー、人参、玉ねぎ、にんにくを混ぜ合わせます。
3.  オリーブオイルと醤油の半分をかけ、均等にコーティングするように混ぜます。
4.  鶏肉と野菜をベーキングシートに広げ、予熱したオーブンで20-25分、または鶏肉に火が通り野菜が柔らかくなるまでローストします。
5.  調理済み米を提供用のボウルに分けます。
6.  各ボウルにローストした鶏肉と野菜をトッピングします。
7.  残りの醤油を各ボウルの上にドリズルします。
8.  熱いうちに提供してお楽しみください！

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
  max_tokens=2000,
  temperature=0.5,
  system="あなたのタスクは、ユーザーが入力した利用可能な食材と食事の好みに基づいて、パーソナライズされたレシピのアイデアを生成することです。この情報を使用して、与えられた食材を使って作ることができ、言及された食事のニーズがあればそれに対応する、創造的で美味しいレシピの多様性を提案してください。各レシピについて、簡潔な説明、必要な食材のリスト、簡単な手順を提供してください。レシピは従いやすく、栄養価が高く、最小限の追加食材や器具で準備できることを確認してください。",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "利用可能な食材：鶏胸肉、ブロッコリー、人参、玉ねぎ、にんにく、オリーブオイル、米、グルテンフリー醤油、はちみつ \n食事の好み：グルテンフリー"
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
  max_tokens: 2000,
  temperature: 0.5,
  system: "あなたのタスクは、ユーザーが入力した利用可能な食材と食事の好みに基づいて、パーソナライズされたレシピのアイデアを生成することです。この情報を使用して、与えられた食材を使って作ることができ、言及された食事のニーズがあればそれに対応する、創造的で美味しいレシピの多様性を提案してください。各レシピについて、簡潔な説明、必要な食材のリスト、簡単な手順を提供してください。レシピは従いやすく、栄養価が高く、最小限の追加食材や器具で準備できることを確認してください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "利用可能な食材：鶏胸肉、ブロッコリー、人参、玉ねぎ、にんにく、オリーブオイル、米、グルテンフリー醤油、はちみつ  \n食事の好み：グルテンフリー"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">

```
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=2000,
    temperature=0.5,
    system="あなたのタスクは、ユーザーが入力した利用可能な食材と食事の好みに基づいて、パーソナライズされたレシピのアイデアを生成することです。この情報を使用して、与えられた食材を使って作ることができ、言及された食事のニーズがあればそれに対応する、創造的で美味しいレシピの多様性を提案してください。各レシピについて、簡潔な説明、必要な食材のリスト、簡単な手順を提供してください。レシピは従いやすく、栄養価が高く、最小限の追加食材や器具で準備できることを確認してください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "利用可能な食材：鶏胸肉、ブロッコリー、人参、玉ねぎ、にんにく、オリーブオイル、米、グルテンフリー醤油、はちみつ  \n食事の好み：グルテンフリー"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="AWS Bedrock TypeScript">

```
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0.5,
  system: "あなたのタスクは、ユーザーが入力した利用可能な食材と食事の好みに基づいて、パーソナライズされたレシピのアイデアを生成することです。この情報を使用して、与えられた食材を使って作ることができ、言及された食事のニーズがあればそれに対応する、創造的で美味しいレシピの多様性を提案してください。各レシピについて、簡潔な説明、必要な食材のリスト、簡単な手順を提供してください。レシピは従いやすく、栄養価が高く、最小限の追加食材や器具で準備できることを確認してください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "利用可能な食材：鶏胸肉、ブロッコリー、人参、玉ねぎ、にんにく、オリーブオイル、米、グルテンフリー醤油、はちみつ  \n食事の好み：グルテンフリー"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">

```
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=2000,
    temperature=0.5,
    system="あなたのタスクは、ユーザーが入力した利用可能な食材と食事の好みに基づいて、パーソナライズされたレシピのアイデアを生成することです。この情報を使用して、与えられた食材を使って作ることができ、言及された食事のニーズがあればそれに対応する、創造的で美味しいレシピの多様性を提案してください。各レシピについて、簡潔な説明、必要な食材のリスト、簡単な手順を提供してください。レシピは従いやすく、栄養価が高く、最小限の追加食材や器具で準備できることを確認してください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "利用可能な食材：鶏胸肉、ブロッコリー、人参、玉ねぎ、にんにく、オリーブオイル、米、グルテンフリー醤油、はちみつ  \n食事の好み：グルテンフリー"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="Vertex AI TypeScript">

```
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0.5,
  system: "あなたのタスクは、ユーザーが入力した利用可能な食材と食事の好みに基づいて、パーソナライズされたレシピのアイデアを生成することです。この情報を使用して、与えられた食材を使って作ることができ、言及された食事のニーズがあればそれに対応する、創造的で美味しいレシピの多様性を提案してください。各レシピについて、簡潔な説明、必要な食材のリスト、簡単な手順を提供してください。レシピは従いやすく、栄養価が高く、最小限の追加食材や器具で準備できることを確認してください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "利用可能な食材：鶏胸肉、ブロッコリー、人参、玉ねぎ、にんにく、オリーブオイル、米、グルテンフリー醤油、はちみつ  \n食事の好み：グルテンフリー"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>