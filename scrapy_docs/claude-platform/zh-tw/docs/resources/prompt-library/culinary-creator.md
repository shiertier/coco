# 烹飪創作者

根據用戶可用的食材和飲食偏好建議食譜創意。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是根據用戶輸入的可用食材和飲食偏好生成個性化的食譜創意。使用這些資訊建議各種創意且美味的食譜，這些食譜可以使用給定的食材製作，同時滿足用戶的飲食需求（如果有提及的話）。對於每個食譜，請提供簡要描述、所需食材清單和簡單的製作說明。確保食譜易於遵循、營養豐富，並且可以用最少的額外食材或設備製作。 |
| User   | 可用食材：雞胸肉、花椰菜、胡蘿蔔、洋蔥、大蒜、橄欖油、米飯、無麩質醬油、蜂蜜 <br/> 飲食偏好：無麩質 |

## 範例輸出

建議食譜：

蜂蜜蒜香雞肉炒菜

描述：一道快速且風味豐富的炒菜，特色是嫩滑的雞胸肉、爽脆的蔬菜，以及甜鹹的蜂蜜蒜香醬汁。  
 食材：

- 2塊雞胸肉，切成一口大小的塊狀
- 2杯花椰菜小朵
- 1杯切片胡蘿蔔
- 1個洋蔥，切丁
- 3瓣大蒜，切碎
- 2大匙橄欖油
- 1/4杯醬油（無麩質）
- 2大匙蜂蜜

製作說明：

1.  在大平底鍋或炒鍋中，用中高火加熱橄欖油。
2.  加入切丁洋蔥和切碎大蒜，炒至香味四溢，約2分鐘。
3.  加入雞肉塊，炒至不再呈粉紅色，約5-7分鐘。
4.  加入花椰菜和胡蘿蔔，炒至蔬菜呈嫩脆狀，約5分鐘。
5.  在小碗中，將醬油和蜂蜜攪拌均勻。
6.  將醬汁倒在雞肉和蔬菜上，攪拌至均勻裹上醬汁。
7.  再炒2-3分鐘，或直到醬汁稍微變稠。
8.  趁熱配蒸米飯享用。

雞肉蔬菜米飯碗

描述：一道豐盛且營養的米飯碗，配上調味雞肉、烤蔬菜，並淋上醬油。

食材：

- 2塊雞胸肉，切成一口大小的塊狀
- 2杯花椰菜小朵
- 1杯切片胡蘿蔔
- 1個洋蔥，切丁
- 2瓣大蒜，切碎
- 2大匙橄欖油
- 1/4杯醬油（無麩質）
- 2杯煮熟的米飯

製作說明：

1.  將烤箱預熱至400°F（200°C）。
2.  在大碗中，將雞肉塊、花椰菜、胡蘿蔔、洋蔥和大蒜混合。
3.  淋上橄欖油和一半的醬油，攪拌至均勻裹上。
4.  將雞肉和蔬菜攤在烤盤上，在預熱的烤箱中烤20-25分鐘，或直到雞肉完全熟透且蔬菜變軟。
5.  將煮熟的米飯分裝到各個餐碗中。
6.  在每個碗上放上烤雞肉和蔬菜。
7.  在每個碗的頂部淋上剩餘的醬油。
8.  趁熱享用！

---

## API 請求

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
  system="您的任務是根據用戶輸入的可用食材和飲食偏好生成個性化的食譜創意。使用這些資訊建議各種創意且美味的食譜，這些食譜可以使用給定的食材製作，同時滿足用戶的飲食需求（如果有提及的話）。對於每個食譜，請提供簡要描述、所需食材清單和簡單的製作說明。確保食譜易於遵循、營養豐富，並且可以用最少的額外食材或設備製作。",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "可用食材：雞胸肉、花椰菜、胡蘿蔔、洋蔥、大蒜、橄欖油、米飯、無麩質醬油、蜂蜜 \n飲食偏好：無麩質"
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
  system: "您的任務是根據用戶輸入的可用食材和飲食偏好生成個性化的食譜創意。使用這些資訊建議各種創意且美味的食譜，這些食譜可以使用給定的食材製作，同時滿足用戶的飲食需求（如果有提及的話）。對於每個食譜，請提供簡要描述、所需食材清單和簡單的製作說明。確保食譜易於遵循、營養豐富，並且可以用最少的額外食材或設備製作。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "可用食材：雞胸肉、花椰菜、胡蘿蔔、洋蔥、大蒜、橄欖油、米飯、無麩質醬油、蜂蜜  \n飲食偏好：無麩質"
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
    system="您的任務是根據用戶輸入的可用食材和飲食偏好生成個性化的食譜創意。使用這些資訊建議各種創意且美味的食譜，這些食譜可以使用給定的食材製作，同時滿足用戶的飲食需求（如果有提及的話）。對於每個食譜，請提供簡要描述、所需食材清單和簡單的製作說明。確保食譜易於遵循、營養豐富，並且可以用最少的額外食材或設備製作。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "可用食材：雞胸肉、花椰菜、胡蘿蔔、洋蔥、大蒜、橄欖油、米飯、無麩質醬油、蜂蜜  \n飲食偏好：無麩質"
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
  system: "您的任務是根據用戶輸入的可用食材和飲食偏好生成個性化的食譜創意。使用這些資訊建議各種創意且美味的食譜，這些食譜可以使用給定的食材製作，同時滿足用戶的飲食需求（如果有提及的話）。對於每個食譜，請提供簡要描述、所需食材清單和簡單的製作說明。確保食譜易於遵循、營養豐富，並且可以用最少的額外食材或設備製作。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "可用食材：雞胸肉、花椰菜、胡蘿蔔、洋蔥、大蒜、橄欖油、米飯、無麩質醬油、蜂蜜  \n飲食偏好：無麩質"
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
    system="您的任務是根據用戶輸入的可用食材和飲食偏好生成個性化的食譜創意。使用這些資訊建議各種創意且美味的食譜，這些食譜可以使用給定的食材製作，同時滿足用戶的飲食需求（如果有提及的話）。對於每個食譜，請提供簡要描述、所需食材清單和簡單的製作說明。確保食譜易於遵循、營養豐富，並且可以用最少的額外食材或設備製作。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "可用食材：雞胸肉、花椰菜、胡蘿蔔、洋蔥、大蒜、橄欖油、米飯、無麩質醬油、蜂蜜  \n飲食偏好：無麩質"
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
  system: "您的任務是根據用戶輸入的可用食材和飲食偏好生成個性化的食譜創意。使用這些資訊建議各種創意且美味的食譜，這些食譜可以使用給定的食材製作，同時滿足用戶的飲食需求（如果有提及的話）。對於每個食譜，請提供簡要描述、所需食材清單和簡單的製作說明。確保食譜易於遵循、營養豐富，並且可以用最少的額外食材或設備製作。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "可用食材：雞胸肉、花椰菜、胡蘿蔔、洋蔥、大蒜、橄欖油、米飯、無麩質醬油、蜂蜜  \n飲食偏好：無麩質"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>