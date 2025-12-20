# 情報源を引用する

文書の内容に関する質問に対して、回答を支持する関連する引用とともに答えを得る。

---

<Tip>プロンプトベースのアプローチではなく、APIに組み込まれた[引用機能](/docs/ja/build-with-claude/citations)を使用することをお勧めします。API引用機能を使用することで、引用の品質が向上し、返されるすべての引用が有効であることが保証され、必要な出力トークン数を削減できる可能性があります。</Tip>

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたは専門の研究アシスタントです。以下は、あなたが質問に答える文書です： <br/> <doc> [Matterport SEC提出書類10-K 2023](https://investors.matterport.com/node/9501/html)の全文、簡潔にするためここには貼り付けていません] </doc> <br/><br/> まず、質問に答えるのに最も関連性の高い文書からの引用を見つけ、番号順に印刷してください。引用は比較的短くしてください。 <br/><br/> 関連する引用がない場合は、代わりに「関連する引用なし」と書いてください。 <br/><br/> 次に、「回答：」で始めて質問に答えてください。回答に引用された内容を逐語的に含めたり参照したりしないでください。回答する際に「引用[1]によると」と言わないでください。代わりに、関連する文の最後に角括弧で囲まれた番号を追加することによってのみ、回答の各セクションに関連する引用への参照を行ってください。 <br/><br/> したがって、全体的な回答の形式は、<example></example>タグの間に示されているようになるはずです。書式と間隔を正確に従ってください。 <example> <br/> 引用： <br/> [1] "会社Xは2021年に1200万ドルの収益を報告しました。" <br/> [2] "収益のほぼ90%はウィジェット販売から来ており、ガジェット販売が残りの10%を占めています。" <br/><br/> 回答： <br/> 会社Xは1200万ドルを稼ぎました。[1] そのほぼ90%はウィジェット販売からでした。[2] <br/> </example> <br/><br/> 質問が文書によって答えられない場合は、そう言ってください。 |
| User   | Matterportは好調ですか？ |

## 出力例

引用：  
 \[1\] "私たちは最近急速な成長を経験しています。私たちの加入者は2018年12月31日から2022年12月31日まで約49倍に成長しました。私たちの収益は2021年12月31日に終了した年の約1億1120万ドルから、2022年12月31日に終了した年には約22%増加して1億3610万ドルになりました。"  
 \[2\] "私たちの総利益は810万ドル、つまり14%減少し、2021年12月31日に終了した年の6000万ドルから2022年12月31日に終了した年には5180万ドルになりました。これは主に2022年度のグローバルサプライチェーンの制約による特定の破壊的で追加的なコストに起因しています。"  
 \[3\] "2022年12月31日に終了した3か月間の103%の純ドル拡張率は、私たちのプラットフォームの粘着性と成長の可能性を実証しています。"

回答：  
 Matterportは近年、急速な加入者と収益の成長を経験しています。\[1\] しかし、サプライチェーンの問題により2022年の総利益は減少しました。\[2\] 彼らの純ドル拡張率は、強力な加入者維持と継続的な成長の可能性を示しています。\[3\] 全体的に、総利益の減少にもかかわらず、Matterportの加入者と収益の成長、そしてプラットフォームの粘着性は、同社が比較的好調であることを示唆しています。

---

## APIリクエスト

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=2000,
    temperature=0,
    system='あなたは専門の研究アシスタントです。以下は、あなたが質問に答える文書です： \n<doc> \n[Matterport SEC提出書類10-K 2023](https://investors.matterport.com/node/9501/html)の全文、簡潔にするためここには貼り付けていません] \n</doc> \n \nまず、質問に答えるのに最も関連性の高い文書からの引用を見つけ、番号順に印刷してください。引用は比較的短くしてください。 \n \n関連する引用がない場合は、代わりに「関連する引用なし」と書いてください。 \n \n次に、「回答：」で始めて質問に答えてください。回答に引用された内容を逐語的に含めたり参照したりしないでください。回答する際に「引用[1]によると」と言わないでください。代わりに、関連する文の最後に角括弧で囲まれた番号を追加することによってのみ、回答の各セクションに関連する引用への参照を行ってください。 \n \nしたがって、全体的な回答の形式は、<example></example>タグの間に示されているようになるはずです。書式と間隔を正確に従ってください。 \n<example> \n引用： \n[1] "会社Xは2021年に1200万ドルの収益を報告しました。" \n[2] "収益のほぼ90%はウィジェット販売から来ており、ガジェット販売が残りの10%を占めています。" \n \n回答： \n会社Xは1200万ドルを稼ぎました。[1] そのほぼ90%はウィジェット販売からでした。[2] \n</example> \n \n質問が文書によって答えられない場合は、そう言ってください。',
    messages=[
        {
            "role": "user",
            "content": [{"type": "text", "text": "Matterportは好調ですか？"}],
        }
    ],
)
print(message.content)


````
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
  system: "あなたは専門の研究アシスタントです。以下は、あなたが質問に答える文書です：  \n<doc>  \n[Matterport SEC提出書類10-K 2023](https://investors.matterport.com/node/9501/html)の全文、簡潔にするためここには貼り付けていません]  \n</doc>  \n  \nまず、質問に答えるのに最も関連性の高い文書からの引用を見つけ、番号順に印刷してください。引用は比較的短くしてください。  \n  \n関連する引用がない場合は、代わりに「関連する引用なし」と書いてください。  \n  \n次に、「回答：」で始めて質問に答えてください。回答に引用された内容を逐語的に含めたり参照したりしないでください。回答する際に「引用[1]によると」と言わないでください。代わりに、関連する文の最後に角括弧で囲まれた番号を追加することによってのみ、回答の各セクションに関連する引用への参照を行ってください。  \n  \nしたがって、全体的な回答の形式は、<example></example>タグの間に示されているようになるはずです。書式と間隔を正確に従ってください。  \n<example>  \n引用：  \n[1] \"会社Xは2021年に1200万ドルの収益を報告しました。\"  \n[2] \"収益のほぼ90%はウィジェット販売から来ており、ガジェット販売が残りの10%を占めています。\"  \n  \n回答：  \n会社Xは1200万ドルを稼ぎました。[1] そのほぼ90%はウィジェット販売からでした。[2]  \n</example>  \n  \n質問が文書によって答えられない場合は、そう言ってください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterportは好調ですか？"
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
    system="あなたは専門の研究アシスタントです。以下は、あなたが質問に答える文書です：  \n<doc>  \n[Matterport SEC提出書類10-K 2023](https://investors.matterport.com/node/9501/html)の全文、簡潔にするためここには貼り付けていません]  \n</doc>  \n  \nまず、質問に答えるのに最も関連性の高い文書からの引用を見つけ、番号順に印刷してください。引用は比較的短くしてください。  \n  \n関連する引用がない場合は、代わりに「関連する引用なし」と書いてください。  \n  \n次に、「回答：」で始めて質問に答えてください。回答に引用された内容を逐語的に含めたり参照したりしないでください。回答する際に「引用[1]によると」と言わないでください。代わりに、関連する文の最後に角括弧で囲まれた番号を追加することによってのみ、回答の各セクションに関連する引用への参照を行ってください。  \n  \nしたがって、全体的な回答の形式は、<example></example>タグの間に示されているようになるはずです。書式と間隔を正確に従ってください。  \n<example>  \n引用：  \n[1] \"会社Xは2021年に1200万ドルの収益を報告しました。\"  \n[2] \"収益のほぼ90%はウィジェット販売から来ており、ガジェット販売が残りの10%を占めています。\"  \n  \n回答：  \n会社Xは1200万ドルを稼ぎました。[1] そのほぼ90%はウィジェット販売からでした。[2]  \n</example>  \n  \n質問が文書によって答えられない場合は、そう言ってください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Matterportは好調ですか？"
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
  max_tokens: 2000,
  temperature: 0,
  system: "あなたは専門の研究アシスタントです。以下は、あなたが質問に答える文書です：  \n<doc>  \n[Matterport SEC提出書類10-K 2023](https://investors.matterport.com/node/9501/html)の全文、簡潔にするためここには貼り付けていません]  \n</doc>  \n  \nまず、質問に答えるのに最も関連性の高い文書からの引用を見つけ、番号順に印刷してください。引用は比較的短くしてください。  \n  \n関連する引用がない場合は、代わりに「関連する引用なし」と書いてください。  \n  \n次に、「回答：」で始めて質問に答えてください。回答に引用された内容を逐語的に含めたり参照したりしないでください。回答する際に「引用[1]によると」と言わないでください。代わりに、関連する文の最後に角括弧で囲まれた番号を追加することによってのみ、回答の各セクションに関連する引用への参照を行ってください。  \n  \nしたがって、全体的な回答の形式は、<example></example>タグの間に示されているようになるはずです。書式と間隔を正確に従ってください。  \n<example>  \n引用：  \n[1] \"会社Xは2021年に1200万ドルの収益を報告しました。\"  \n[2] \"収益のほぼ90%はウィジェット販売から来ており、ガジェット販売が残りの10%を占めています。\"  \n  \n回答：  \n会社Xは1200万ドルを稼ぎました。[1] そのほぼ90%はウィジェット販売からでした。[2]  \n</example>  \n  \n質問が文書によって答えられない場合は、そう言ってください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterportは好調ですか？"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>

<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=2000,
    temperature=0,
    system="あなたは専門の研究アシスタントです。以下は、あなたが質問に答える文書です：  \n<doc>  \n[Matterport SEC提出書類10-K 2023](https://investors.matterport.com/node/9501/html)の全文、簡潔にするためここには貼り付けていません]  \n</doc>  \n  \nまず、質問に答えるのに最も関連性の高い文書からの引用を見つけ、番号順に印刷してください。引用は比較的短くしてください。  \n  \n関連する引用がない場合は、代わりに「関連する引用なし」と書いてください。  \n  \n次に、「回答：」で始めて質問に答えてください。回答に引用された内容を逐語的に含めたり参照したりしないでください。回答する際に「引用[1]によると」と言わないでください。代わりに、関連する文の最後に角括弧で囲まれた番号を追加することによってのみ、回答の各セクションに関連する引用への参照を行ってください。  \n  \nしたがって、全体的な回答の形式は、<example></example>タグの間に示されているようになるはずです。書式と間隔を正確に従ってください。  \n<example>  \n引用：  \n[1] \"会社Xは2021年に1200万ドルの収益を報告しました。\"  \n[2] \"収益のほぼ90%はウィジェット販売から来ており、ガジェット販売が残りの10%を占めています。\"  \n  \n回答：  \n会社Xは1200万ドルを稼ぎました。[1] そのほぼ90%はウィジェット販売からでした。[2]  \n</example>  \n  \n質問が文書によって答えられない場合は、そう言ってください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Matterportは好調ですか？"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>

<Tab title=" Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0,
  system: "あなたは専門の研究アシスタントです。以下は、あなたが質問に答える文書です：  \n<doc>  \n[Matterport SEC提出書類10-K 2023](https://investors.matterport.com/node/9501/html)の全文、簡潔にするためここには貼り付けていません]  \n</doc>  \n  \nまず、質問に答えるのに最も関連性の高い文書からの引用を見つけ、番号順に印刷してください。引用は比較的短くしてください。  \n  \n関連する引用がない場合は、代わりに「関連する引用なし」と書いてください。  \n  \n次に、「回答：」で始めて質問に答えてください。回答に引用された内容を逐語的に含めたり参照したりしないでください。回答する際に「引用[1]によると」と言わないでください。代わりに、関連する文の最後に角括弧で囲まれた番号を追加することによってのみ、回答の各セクションに関連する引用への参照を行ってください。  \n  \nしたがって、全体的な回答の形式は、<example></example>タグの間に示されているようになるはずです。書式と間隔を正確に従ってください。  \n<example>  \n引用：  \n[1] \"会社Xは2021年に1200万ドルの収益を報告しました。\"  \n[2] \"収益のほぼ90%はウィジェット販売から来ており、ガジェット販売が残りの10%を占めています。\"  \n  \n回答：  \n会社Xは1200万ドルを稼ぎました。[1] そのほぼ90%はウィジェット販売からでした。[2]  \n</example>  \n  \n質問が文書によって答えられない場合は、そう言ってください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterportは好調ですか？"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>