# トリビアジェネレーター

幅広いトピックでトリビア問題を生成し、必要に応じてヒントを提供します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

| Role | Content |
| ---- | --- |
| User | 様々なトピックでトリビア問題を生成し、ユーザーが正解に辿り着けるようヒントを提供してください。多様なカテゴリから選択し、ユーザーの知識や推理力をテストする問題を作成してください。ユーザーを解答に導くため、段階的により具体的なヒントを提供してください。問題は挑戦的で、ヒントは答えを簡単に教えすぎることなく、ユーザーを助けるのに十分な情報を提供するようにしてください。 |

## 出力例

カテゴリ: 地理

問題:  
 フランスとスペインの両方と国境を接する世界で唯一の国はどこですか？

ヒント1: この国はフランスとスペインの間のピレネー山脈に位置しています。  
 ヒント2: 美しい山の景色、スキーリゾート、免税ショッピングで知られています。  
 ヒント3: この国の首都の名前には、国名自体が含まれています。

答え: アンドラ

カテゴリ: 科学

問題:  
 植物が太陽光、水、二酸化炭素をエネルギーと酸素に変換するプロセスは何ですか？

ヒント1: このプロセスは地球上の生命にとって不可欠で、食物と酸素の両方を提供します。  
 ヒント2: 主に植物の葉で行われます。  
 ヒント3: このプロセスには、太陽光を吸収する緑色の色素であるクロロフィルが関与しています。

答え: 光合成

カテゴリ: 文学

問題:  
 1813年に初版が出版された古典小説「高慢と偏見」を書いたのは誰ですか？

ヒント1: 著者は1775年から1817年まで生きたイギリス人女性です。  
 ヒント2: 彼女は英国社会とマナーの機知に富んだ洞察力のある描写で知られています。  
 ヒント3: 彼女の他の有名な作品には「分別と多感」と「エマ」があります。

答え: ジェーン・オースティン

カテゴリ: 歴史

問題:  
 東西ドイツを分けていたベルリンの壁が取り壊され始めたのは何年ですか？

ヒント1: この出来事は20世紀後半に起こりました。  
 ヒント2: 冷戦終結の重要な瞬間を示しました。  
 ヒント3: この年はヨーロッパでの「共産主義の崩壊」という言葉とよく関連付けられます。

答え: 1989年

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
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "様々なトピックでトリビア問題を生成し、ユーザーが正解に辿り着けるようヒントを提供してください。多様なカテゴリから選択し、ユーザーの知識や推理力をテストする問題を作成してください。ユーザーを解答に導くため、段階的により具体的なヒントを提供してください。問題は挑戦的で、ヒントは答えを簡単に教えすぎることなく、ユーザーを助けるのに十分な情報を提供するようにしてください。"
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "様々なトピックでトリビア問題を生成し、ユーザーが正解に辿り着けるようヒントを提供してください。多様なカテゴリから選択し、ユーザーの知識や推理力をテストする問題を作成してください。ユーザーを解答に導くため、段階的により具体的なヒントを提供してください。問題は挑戦的で、ヒントは答えを簡単に教えすぎることなく、ユーザーを助けるのに十分な情報を提供するようにしてください。"
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
temperature=0.5,
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "様々なトピックでトリビア問題を生成し、ユーザーが正解に辿り着けるようヒントを提供してください。多様なカテゴリから選択し、ユーザーの知識や推理力をテストする問題を作成してください。ユーザーを解答に導くため、段階的により具体的なヒントを提供してください。問題は挑戦的で、ヒントは答えを簡単に教えすぎることなく、ユーザーを助けるのに十分な情報を提供するようにしてください。"
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
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "様々なトピックでトリビア問題を生成し、ユーザーが正解に辿り着けるようヒントを提供してください。多様なカテゴリから選択し、ユーザーの知識や推理力をテストする問題を作成してください。ユーザーを解答に導くため、段階的により具体的なヒントを提供してください。問題は挑戦的で、ヒントは答えを簡単に教えすぎることなく、ユーザーを助けるのに十分な情報を提供するようにしてください。"
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
    temperature=0.5,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "様々なトピックでトリビア問題を生成し、ユーザーが正解に辿り着けるようヒントを提供してください。多様なカテゴリから選択し、ユーザーの知識や推理力をテストする問題を作成してください。ユーザーを解答に導くため、段階的により具体的なヒントを提供してください。問題は挑戦的で、ヒントは答えを簡単に教えすぎることなく、ユーザーを助けるのに十分な情報を提供するようにしてください。"
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
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "様々なトピックでトリビア問題を生成し、ユーザーが正解に辿り着けるようヒントを提供してください。多様なカテゴリから選択し、ユーザーの知識や推理力をテストする問題を作成してください。ユーザーを解答に導くため、段階的により具体的なヒントを提供してください。問題は挑戦的で、ヒントは答えを簡単に教えすぎることなく、ユーザーを助けるのに十分な情報を提供するようにしてください。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>