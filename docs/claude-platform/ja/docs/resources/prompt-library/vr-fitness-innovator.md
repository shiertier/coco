# VRフィットネスイノベーター

バーチャルリアリティフィットネスゲームの創造的なアイデアをブレインストーミングします。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

| 役割 | 内容 |
| ---- | --- |
| ユーザー | あなたのタスクは、バーチャルリアリティ（VR）フィットネスゲームの革新的で魅力的なアイデアのリストを生成することです。様々なゲームジャンル、ユニークなゲームプレイメカニクス、没入感のある環境、そして身体的な運動や動きを組み込む創造的な方法を考慮してください。アイデアは幅広いフィットネス愛好家やゲーマーにアピールし、VRでの運動中にアクティブに過ごし、楽しみながら運動することを促すものでなければなりません。各アイデアについて、ゲームコンセプト、主要機能、そしてフィットネスをどのように促進するかの簡潔な説明を提供してください。 |

## 出力例

1.  アイデア: VRパルクールアドベンチャー  
    説明: プレイヤーはダイナミックな都市環境をナビゲートし、走り、ジャンプし、登り、パルクールスタントを実行して障害物を克服し、目標に到達します。ゲームはプレイヤーの動きに適応し、バーチャル都市を探索しながら激しい全身ワークアウトを提供します。  
    主要機能: リアルなパルクールメカニクス、手続き的に生成される環境、マルチプレイヤーレース、チャレンジ。  
    フィットネス面: 有酸素運動、敏捷性、協調性、下半身の筋力。
2.  アイデア: リズムボクシング  
    説明: 人気音楽トラックのビートに合わせた高エネルギーボクシングゲーム。プレイヤーはリズムに合わせてパンチ、回避、ブロックを行い、ポイントを獲得し、ますます困難になるレベルを進行します。ゲームは様々なボクシングスタイルとトレーニングモードを提供します。  
    主要機能: ライセンス音楽、カスタマイズ可能なボクサー、オンラインマルチプレイヤー、多様なボクシングテクニック。  
    フィットネス面: 有酸素運動、上半身の筋力、反射神経、持久力。
3.  アイデア: VRフィットネスRPG  
    説明: プレイヤーが自分のキャラクターを作成し、ファンタジー世界を救うクエストに乗り出す没入型ロールプレイングゲーム。ゲームは伝統的なRPG要素とフィットネスチャレンジを組み合わせ、プレイヤーが呪文を唱え、敵を倒し、キャラクターをレベルアップするために身体的な運動を実行することを要求します。  
    主要機能: キャラクターカスタマイゼーション、スキルツリー、壮大なボス戦、筋力、有酸素運動、柔軟性運動の組み合わせ。  
    フィットネス面: 全身ワークアウト、筋力トレーニング、有酸素運動、柔軟性。

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
  temperature=1,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Your task is to generate a list of innovative and engaging ideas for virtual reality (VR) fitness games. Consider various game genres, unique gameplay mechanics, immersive environments, and creative ways to incorporate physical exercises and movements. The ideas should be appealing to a wide range of fitness enthusiasts and gamers, encouraging them to stay active and have fun while exercising in VR. For each idea, provide a brief description of the game concept, key features, and how it promotes fitness."
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
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Your task is to generate a list of innovative and engaging ideas for virtual reality (VR) fitness games. Consider various game genres, unique gameplay mechanics, immersive environments, and creative ways to incorporate physical exercises and movements. The ideas should be appealing to a wide range of fitness enthusiasts and gamers, encouraging them to stay active and have fun while exercising in VR. For each idea, provide a brief description of the game concept, key features, and how it promotes fitness."
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
    temperature=1,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Your task is to generate a list of innovative and engaging ideas for virtual reality (VR) fitness games. Consider various game genres, unique gameplay mechanics, immersive environments, and creative ways to incorporate physical exercises and movements. The ideas should be appealing to a wide range of fitness enthusiasts and gamers, encouraging them to stay active and have fun while exercising in VR. For each idea, provide a brief description of the game concept, key features, and how it promotes fitness."
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
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Your task is to generate a list of innovative and engaging ideas for virtual reality (VR) fitness games. Consider various game genres, unique gameplay mechanics, immersive environments, and creative ways to incorporate physical exercises and movements. The ideas should be appealing to a wide range of fitness enthusiasts and gamers, encouraging them to stay active and have fun while exercising in VR. For each idea, provide a brief description of the game concept, key features, and how it promotes fitness."
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
    max_tokens=1000,
    temperature=1,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Your task is to generate a list of innovative and engaging ideas for virtual reality (VR) fitness games. Consider various game genres, unique gameplay mechanics, immersive environments, and creative ways to incorporate physical exercises and movements. The ideas should be appealing to a wide range of fitness enthusiasts and gamers, encouraging them to stay active and have fun while exercising in VR. For each idea, provide a brief description of the game concept, key features, and how it promotes fitness."
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
  max_tokens: 1000,
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Your task is to generate a list of innovative and engaging ideas for virtual reality (VR) fitness games. Consider various game genres, unique gameplay mechanics, immersive environments, and creative ways to incorporate physical exercises and movements. The ideas should be appealing to a wide range of fitness enthusiasts and gamers, encouraging them to stay active and have fun while exercising in VR. For each idea, provide a brief description of the game concept, key features, and how it promotes fitness."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>