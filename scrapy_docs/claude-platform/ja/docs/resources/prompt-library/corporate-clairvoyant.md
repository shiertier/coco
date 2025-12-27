# 企業の千里眼

長い企業レポートから洞察を抽出し、リスクを特定し、重要な情報を単一のメモに要約します。

---

> このプロンプトを私たちの開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|      | コンテンツ |
| ---- | --- |
| ユーザー | あなたのタスクは以下のレポートを分析することです： <br /> <br /> <report> [Matterport SEC提出書類10-K 2023の全文](https://investors.matterport.com/node/9501/html)、簡潔性のためここには貼り付けていません] </report> <br /> <br /> この年次報告書を簡潔で明確な方法で要約し、主要な市場トレンドと要点を特定してください。私のチームに送ることができる短いメモとして調査結果を出力してください。メモの目標は、私のチームが金融機関の状況について最新情報を把握し、来四半期に予想される運営および収益リスクがあるかどうかを定性的に予測し特定することです。要約と分析にすべての関連詳細を含めるようにしてください。 |

## 出力例

> 宛先：投資チーム  
> 差出人：\[あなたの名前\]  
> 日付：2024年3月4日  
> 件名：Matterport 2022年10-Kハイライトと財務見通し
>
> チームの皆様、
>
> Matterportの2022年年次報告書から主要な要点を特定しました：
>
> 財務パフォーマンス：
>
> - 2022年の収益は22％増加し1億3610万ドルとなり、サブスクリプションとサービス収益の成長によって牽引されました
> - しかし、製品収益の増加コストとサプライチェーンの課題により、粗利益率は54％から38％に減少しました
> - 成長を促進するためにR&Dと営業・マーケティングに大きく投資したため、営業損失は2億7550万ドルに拡大しました
> - 2022年末時点で運営資金として4億7690万ドルの現金と投資を保有していました
>
> 事業ハイライト：
>
> - 加入者ベースは39％成長し、2022年末時点で70万1000人を超えました。管理下のスペースは920万に達しました
> - Matterport Axisモーター化マウント、Pro3カメラ、クラウドソフトウェアの主要アップデートの発売でプラットフォームを拡張しました
> - Enview（3D空間データ用AI）とVHT Studios（不動産マーケティング）の買収を完了し、提供サービスを強化しました
> - Burns & McDonnellとCGSとの注目すべきパートナーシップを締結し、Matterportのソリューションを企業顧客に提供しました
>
> 見通し：
>
> - Matterportは世界の建物と空間をデジタル化する巨大な市場機会を見込んでいます（2400億ドル以上のTAMと推定）
> - 不動産、AEC、施設管理などの主要業界の大企業顧客での採用促進に焦点を当てています
> - ほぼ未開拓のグローバル建物市場に浸透するため、R&Dと国際展開への投資を継続します
> - 短期的な成長とマージンはマクロ経済の逆風とサプライチェーン問題の影響を受ける可能性があります
> - しかし、健全な流動性ポジションと比較的粘着性のある顧客ベースが下方リスクを軽減します
>
> 要約すると、Matterportは巨大なグローバル市場への浸透を進める中で2022年に強いトップライン成長を達成しました。成長のための大きな投資により収益性は依然として課題ですが、バランスシートは堅調に見えます。継続的なサプライチェーン問題を除けば、主要な運営指標に基づく基本的な事業勢いは維持されているようです。深刻な経済低迷がない限り、Matterportは新興の建物デジタル化分野でシェアを獲得し続ける良いポジションにあるようです。

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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "あなたのタスクは以下のレポートを分析することです：  \n<report>  \n[Matterport SEC提出書類10-K 2023の全文](https://investors.matterport.com/node/9501/html)、簡潔性のためここには貼り付けていません]  \n</report>  \n  \nこの年次報告書を簡潔で明確な方法で要約し、主要な市場トレンドと要点を特定してください。私のチームに送ることができる短いメモとして調査結果を出力してください。メモの目標は、私のチームが金融機関の状況について最新情報を把握し、来四半期に予想される運営および収益リスクがあるかどうかを定性的に予測し特定することです。要約と分析にすべての関連詳細を含めるようにしてください。"
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "あなたのタスクは以下のレポートを分析することです：  \n<report>  \n[Matterport SEC提出書類10-K 2023の全文](https://investors.matterport.com/node/9501/html)、簡潔性のためここには貼り付けていません]  \n</report>  \n  \nこの年次報告書を簡潔で明確な方法で要約し、主要な市場トレンドと要点を特定してください。私のチームに送ることができる短いメモとして調査結果を出力してください。メモの目標は、私のチームが金融機関の状況について最新情報を把握し、来四半期に予想される運営および収益リスクがあるかどうかを定性的に予測し特定することです。要約と分析にすべての関連詳細を含めるようにしてください。"
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
max_tokens=2000,
temperature=0,
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "あなたのタスクは以下のレポートを分析することです： \n<report> \n[Matterport SEC提出書類10-K 2023の全文](https://investors.matterport.com/node/9501/html)、簡潔性のためここには貼り付けていません] \n</report> \n \nこの年次報告書を簡潔で明確な方法で要約し、主要な市場トレンドと要点を特定してください。私のチームに送ることができる短いメモとして調査結果を出力してください。メモの目標は、私のチームが金融機関の状況について最新情報を把握し、来四半期に予想される運営および収益リスクがあるかどうかを定性的に予測し特定することです。要約と分析にすべての関連詳細を含めるようにしてください。"
}
]
}
]
)
print(message.content)

````
  </Tab>
    <Tab title="AWS Bedrock TypeScript">
```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "あなたのタスクは以下のレポートを分析することです：  \n<report>  \n[Matterport SEC提出書類10-K 2023の全文](https://investors.matterport.com/node/9501/html)、簡潔性のためここには貼り付けていません]  \n</report>  \n  \nこの年次報告書を簡潔で明確な方法で要約し、主要な市場トレンドと要点を特定してください。私のチームに送ることができる短いメモとして調査結果を出力してください。メモの目標は、私のチームが金融機関の状況について最新情報を把握し、来四半期に予想される運営および収益リスクがあるかどうかを定性的に予測し特定することです。要約と分析にすべての関連詳細を含めるようにしてください。"
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
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "あなたのタスクは以下のレポートを分析することです：  \n<report>  \n[Matterport SEC提出書類10-K 2023の全文](https://investors.matterport.com/node/9501/html)、簡潔性のためここには貼り付けていません]  \n</report>  \n  \nこの年次報告書を簡潔で明確な方法で要約し、主要な市場トレンドと要点を特定してください。私のチームに送ることができる短いメモとして調査結果を出力してください。メモの目標は、私のチームが金融機関の状況について最新情報を把握し、来四半期に予想される運営および収益リスクがあるかどうかを定性的に予測し特定することです。要約と分析にすべての関連詳細を含めるようにしてください。"
                }
            ]
        }
    ]
)
print(message.content)

```

  </Tab>

    <Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "あなたのタスクは以下のレポートを分析することです：  \n<report>  \n[Matterport SEC提出書類10-K 2023の全文](https://investors.matterport.com/node/9501/html)、簡潔性のためここには貼り付けていません]  \n</report>  \n  \nこの年次報告書を簡潔で明確な方法で要約し、主要な市場トレンドと要点を特定してください。私のチームに送ることができる短いメモとして調査結果を出力してください。メモの目標は、私のチームが金融機関の状況について最新情報を把握し、来四半期に予想される運営および収益リスクがあるかどうかを定性的に予測し特定することです。要約と分析にすべての関連詳細を含めるようにしてください。"
        }
      ]
    }
  ]
});
console.log(msg);

```

  </Tab>
</Tabs>