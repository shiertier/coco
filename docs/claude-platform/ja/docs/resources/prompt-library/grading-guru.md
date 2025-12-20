# 採点の達人

ユーザー定義の基準と標準に基づいて、書かれたテキストの品質を比較・評価します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

| 役割 | 内容 |
| ---- | --- |
| ユーザー | 以下の2つのテキストを与えられた基準に基づいて評価してください： <br/> <br/> テキスト1： <br/> 太陽が山の向こうに沈み、谷全体に暖かい金色の輝きを投げかけていた。鳥たちが夕べの歌を囀り、穏やかな風が葉を通り抜けてさらさらと音を立てていた。それは平和で静寂な瞬間であり、心を喜びと満足で満たすものだった。 <br/><br/> テキスト2： <br/> 夕日がきれいだった。鳥が歌っていて、風が吹いていた。それは私を幸せにした。 <br/><br/> 基準： <br/> 1. 描写的な言語と映像表現 <br/> 2. 文構造と多様性 <br/> 3. 感情的インパクトと関与度 <br/> 4. 文法と句読点 |

## 出力例

提供された基準に基づいて、2つのテキストの評価は以下の通りです：

テキスト1：

1. 描写的な言語と映像表現：このテキストは鮮明で喚起的な言語を使用して、設定の絵を描いています。「暖かい金色の輝き」、「鳥たちが夕べの歌を囀り」、「穏やかな風が葉を通り抜けてさらさらと音を立てていた」などのフレーズは、強い視覚的・聴覚的映像を作り出し、読者をその場面に没入させます。

2. 文構造と多様性：このテキストは、単純文と複合文の両方を含む、文構造の良いミックスを示しています。この多様性は滑らかな流れを維持し、読者の関心を引き続けるのに役立ちます。

3. 感情的インパクトと関与度：このテキストは平和、喜び、満足感を効果的に伝え、読者をその瞬間の感情的体験に参加させます。設定の描写と肯定的な言語の使用が強い感情的つながりを作り出します。

4. 文法と句読点：このテキストは文法的に正しく、適切に句読点が打たれているように見え、明らかなエラーや問題はありません。

テキスト2：

1. 描写的な言語と映像表現：このテキストは詳細な描写的言語と映像表現に欠けています。夕日、鳥、風について言及していますが、読者がその場面を視覚化するのに役立つ鮮明で喚起的な詳細は提供していません。

2. 文構造と多様性：このテキストは構造にほとんど変化のない短い単純文で構成されています。この多様性の欠如は、文章を単調で魅力に欠けるものにする可能性があります。

3. 感情的インパクトと関与度：このテキストは「それは私を幸せにした」と述べることで肯定的な感情を伝えていますが、描写的言語と映像表現の欠如により、強い感情的つながりや関与度を提供していません。

4. 文法と句読点：このテキストは文法的に正しく、適切に句読点が打たれているように見え、明らかなエラーや問題はありません。

比較分析：  
評価に基づくと、テキスト1は描写的言語、映像表現、文構造、多様性、感情的インパクトの点でテキスト2を大幅に上回っています。テキスト1は読者にとって鮮明で没入感のある体験を作り出し、強い描写的言語、多様な文構造、効果的な感情的関与により、与えられた基準に基づいてより高い品質の文章を示しています。テキスト2は文法的には正しいものの、テキスト1に存在する描写の豊かさと感情的インパクトに欠けています。

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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "以下の2つのテキストを与えられた基準に基づいて評価してください： \n \nテキスト1： \n太陽が山の向こうに沈み、谷全体に暖かい金色の輝きを投げかけていた。鳥たちが夕べの歌を囀り、穏やかな風が葉を通り抜けてさらさらと音を立てていた。それは平和で静寂な瞬間であり、心を喜びと満足で満たすものだった。 \n \nテキスト2： \n夕日がきれいだった。鳥が歌っていて、風が吹いていた。それは私を幸せにした。 \n \n基準： \n1. 描写的な言語と映像表現 \n2. 文構造と多様性 \n3. 感情的インパクトと関与度 \n4. 文法と句読点",
                }
            ],
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "以下の2つのテキストを与えられた基準に基づいて評価してください：  \n  \nテキスト1：  \n太陽が山の向こうに沈み、谷全体に暖かい金色の輝きを投げかけていた。鳥たちが夕べの歌を囀り、穏やかな風が葉を通り抜けてさらさらと音を立てていた。それは平和で静寂な瞬間であり、心を喜びと満足で満たすものだった。  \n  \nテキスト2：  \n夕日がきれいだった。鳥が歌っていて、風が吹いていた。それは私を幸せにした。  \n  \n基準：  \n1. 描写的な言語と映像表現  \n2. 文構造と多様性  \n3. 感情的インパクトと関与度  \n4. 文法と句読点"
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
                    "text": "以下の2つのテキストを与えられた基準に基づいて評価してください：  \n  \nテキスト1：  \n太陽が山の向こうに沈み、谷全体に暖かい金色の輝きを投げかけていた。鳥たちが夕べの歌を囀り、穏やかな風が葉を通り抜けてさらさらと音を立てていた。それは平和で静寂な瞬間であり、心を喜びと満足で満たすものだった。  \n  \nテキスト2：  \n夕日がきれいだった。鳥が歌っていて、風が吹いていた。それは私を幸せにした。  \n  \n基準：  \n1. 描写的な言語と映像表現  \n2. 文構造と多様性  \n3. 感情的インパクトと関与度  \n4. 文法と句読点"
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "以下の2つのテキストを与えられた基準に基づいて評価してください：  \n  \nテキスト1：  \n太陽が山の向こうに沈み、谷全体に暖かい金色の輝きを投げかけていた。鳥たちが夕べの歌を囀り、穏やかな風が葉を通り抜けてさらさらと音を立てていた。それは平和で静寂な瞬間であり、心を喜びと満足で満たすものだった。  \n  \nテキスト2：  \n夕日がきれいだった。鳥が歌っていて、風が吹いていた。それは私を幸せにした。  \n  \n基準：  \n1. 描写的な言語と映像表現  \n2. 文構造と多様性  \n3. 感情的インパクトと関与度  \n4. 文法と句読点"
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "以下の2つのテキストを与えられた基準に基づいて評価してください：  \n  \nテキスト1：  \n太陽が山の向こうに沈み、谷全体に暖かい金色の輝きを投げかけていた。鳥たちが夕べの歌を囀り、穏やかな風が葉を通り抜けてさらさらと音を立てていた。それは平和で静寂な瞬間であり、心を喜びと満足で満たすものだった。  \n  \nテキスト2：  \n夕日がきれいだった。鳥が歌っていて、風が吹いていた。それは私を幸せにした。  \n  \n基準：  \n1. 描写的な言語と映像表現  \n2. 文構造と多様性  \n3. 感情的インパクトと関与度  \n4. 文法と句読点"
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
          "text": "以下の2つのテキストを与えられた基準に基づいて評価してください：  \n  \nテキスト1：  \n太陽が山の向こうに沈み、谷全体に暖かい金色の輝きを投げかけていた。鳥たちが夕べの歌を囀り、穏やかな風が葉を通り抜けてさらさらと音を立てていた。それは平和で静寂な瞬間であり、心を喜びと満足で満たすものだった。  \n  \nテキスト2：  \n夕日がきれいだった。鳥が歌っていて、風が吹いていた。それは私を幸せにした。  \n  \n基準：  \n1. 描写的な言語と映像表現  \n2. 文構造と多様性  \n3. 感情的インパクトと関与度  \n4. 文法と句読点"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>