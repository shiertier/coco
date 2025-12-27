# 會議記錄員

將會議內容提煉為簡潔摘要，包括討論主題、關鍵要點和行動項目。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是審查提供的會議記錄並創建一個簡潔的摘要，捕捉重要信息，重點關注關鍵要點和在會議期間分配給特定個人或部門的行動項目。使用清晰和專業的語言，並使用適當的格式（如標題、副標題和項目符號）以邏輯方式組織摘要。確保摘要易於理解，並提供會議內容的全面但簡潔的概述，特別注重清楚地指出誰負責每個行動項目。 |
| User   | 會議記錄： <br/> <br/> 日期：意大利維羅納 - 16世紀末 <br/><br/> 出席者： <br/> - Capulet勳爵（Capulet家族族長） <br/> - Montague勳爵（Montague家族族長） <br/> - Escalus親王（維羅納統治者） <br/> - Laurence修士（宗教顧問） <br/><br/> 議程： <br/> 1. 解決Capulet和Montague家族之間持續的世仇 <br/> 2. 討論Romeo Montague和Juliet Capulet的秘密婚姻 <br/> 3. 制定為維羅納帶來和平的計劃 <br/> 4. 處理Romeo和Juliet的悲劇性死亡 <br/><br/> 討論： <br/> - Escalus親王開場表達了對Capulet和Montague家族之間長期世仇的嚴重關切。他譴責了Capulet勳爵和Montague勳爵最近擾亂維羅納街道和平的暴力衝突。親王警告說，進一步的暴力將導致嚴重後果，包括重罰和對肇事者的潛在流放。 <br/> - Laurence修士隨後提出了Romeo Montague和Juliet Capulet之間的婚姻話題，這場婚姻是在他的指導下進行的。Capulet勳爵和Montague勳爵顯然不知道此事，並以憤怒和不信反應。然而，Laurence修士敦促他們考慮他們孩子之間深刻而悲劇的愛情，以及這種愛情在未來治癒家族裂痕的潛力。 <br/> - Escalus親王提議在Capulet和Montague家族之間建立正式休戰。他要求雙方放下武器，停止一切相互敵對行動。親王宣布，任何違反休戰的行為都將導致嚴厲懲罰，包括流放甚至處決的可能性。Capulet勳爵和Montague勳爵認識到親王話語中的智慧以及和平對其家族和城市福祉的必要性，勉強同意了休戰條款。 <br/> - 當討論Romeo和Juliet的悲劇性死亡時，會議轉向了沉重的氣氛。Laurence修士敘述了導致這對年輕戀人自殺的不幸事件系列，強調了家族仇恨對他們無辜孩子的毀滅性影響。Capulet勳爵和Montague勳爵被悲傷和悔恨所壓倒，承認他們盲目的仇恨最終導致了他們摯愛孩子的失去。 <br/>- Escalus親王呼籲家族從這個令人心碎的悲劇中學習，並為了紀念Romeo和Juliet而擁抱寬恕和團結。他敦促他們共同努力在維羅納創造持久的和平，拋開他們長期的敵意。Laurence修士提供支持，願意調解任何未來的爭端並提供精神指導，幫助家族治癒和前進。 <br/>- 隨著會議接近尾聲，Capulet勳爵和Montague勳爵承諾結束他們的世仇並致力於和解。Escalus親王重申了他確保休戰得到維護的承諾，承諾對任何膽敢破壞休戰的人進行迅速制裁。 <br/>- 出席者同意定期會面討論他們和解努力的進展，並解決可能出現的任何挑戰。 |

## 範例輸出

會議摘要：維羅納的和解與和平
日期：16世紀末，意大利維羅納

出席者：

    - Capulet勳爵（Capulet家族族長）
    - Montague勳爵（Montague家族族長）
    - Escalus親王（維羅納統治者）
    - Laurence修士（宗教顧問）

關鍵要點：

    - Escalus親王處理了Capulet和Montague家族之間持續的世仇，警告進一步暴力將面臨嚴重後果。
    - Laurence修士透露了Romeo Montague和Juliet Capulet之間的秘密婚姻，敦促家族考慮他們的愛情治癒裂痕的潛力。
    - Escalus親王提議在家族之間建立正式休戰，威脅對任何違反行為進行嚴厲懲罰。
    - 討論了Romeo和Juliet的悲劇性死亡，Capulet勳爵和Montague勳爵承認了他們在這個令人心碎結果中的作用。
    - Escalus親王呼籲為了紀念Romeo和Juliet而寬恕和團結，敦促家族共同努力在維羅納實現持久和平。

行動項目：

- Capulet勳爵和Montague勳爵：
  - 放下武器，停止一切相互敵對行動。
  - 致力於和解並擁抱寬恕。
  - 參加定期會議討論進展並解決挑戰。
- Escalus親王：
  - 執行休戰並確保其遵守。
  - 對任何違反休戰的行為進行迅速制裁。
- Laurence修士：
  - 提供調解和精神指導，支持家族的治癒和和解過程。

下一步：

- 出席者將定期會面監督和解努力的進展並解決任何出現的挑戰。

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
    max_tokens=4000,
    temperature=0.5,
    system="您的任務是審查提供的會議記錄並創建一個簡潔的摘要，捕捉重要信息，重點關注關鍵要點和在會議期間分配給特定個人或部門的行動項目。使用清晰和專業的語言，並使用適當的格式（如標題、副標題和項目符號）以邏輯方式組織摘要。確保摘要易於理解，並提供會議內容的全面但簡潔的概述，特別注重清楚地指出誰負責每個行動項目。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "會議記錄：  \n  \n日期：意大利維羅納 - 16世紀末  \n  \n出席者：  \n- Capulet勳爵（Capulet家族族長）  \n- Montague勳爵（Montague家族族長）  \n- Escalus親王（維羅納統治者）  \n- Laurence修士（宗教顧問）  \n  \n議程：  \n1. 解決Capulet和Montague家族之間持續的世仇  \n2. 討論Romeo Montague和Juliet Capulet的秘密婚姻  \n3. 制定為維羅納帶來和平的計劃  \n4. 處理Romeo和Juliet的悲劇性死亡  \n  \n討論：  \n- Escalus親王開場表達了對Capulet和Montague家族之間長期世仇的嚴重關切。他譴責了Capulet勳爵和Montague勳爵最近擾亂維羅納街道和平的暴力衝突。親王警告說，進一步的暴力將導致嚴重後果，包括重罰和對肇事者的潛在流放。  \n- Laurence修士隨後提出了Romeo Montague和Juliet Capulet之間的婚姻話題，這場婚姻是在他的指導下進行的。Capulet勳爵和Montague勳爵顯然不知道此事，並以憤怒和不信反應。然而，Laurence修士敦促他們考慮他們孩子之間深刻而悲劇的愛情，以及這種愛情在未來治癒家族裂痕的潛力。  \n- Escalus親王提議在Capulet和Montague家族之間建立正式休戰。他要求雙方放下武器，停止一切相互敵對行動。親王宣布，任何違反休戰的行為都將導致嚴厲懲罰，包括流放甚至處決的可能性。Capulet勳爵和Montague勳爵認識到親王話語中的智慧以及和平對其家族和城市福祉的必要性，勉強同意了休戰條款。  \n- 當討論Romeo和Juliet的悲劇性死亡時，會議轉向了沉重的氣氛。Laurence修士敘述了導致這對年輕戀人自殺的不幸事件系列，強調了家族仇恨對他們無辜孩子的毀滅性影響。Capulet勳爵和Montague勳爵被悲傷和悔恨所壓倒，承認他們盲目的仇恨最終導致了他們摯愛孩子的失去。  \n- Escalus親王呼籲家族從這個令人心碎的悲劇中學習，並為了紀念Romeo和Juliet而擁抱寬恕和團結。他敦促他們共同努力在維羅納創造持久的和平，拋開他們長期的敵意。Laurence修士提供支持，願意調解任何未來的爭端並提供精神指導，幫助家族治癒和前進。  \n- 隨著會議接近尾聲，Capulet勳爵和Montague勳爵承諾結束他們的世仇並致力於和解。Escalus親王重申了他確保休戰得到維護的承諾，承諾對任何膽敢破壞休戰的人進行迅速制裁。  \n- 出席者同意定期會面討論他們和解努力的進展，並解決可能出現的任何挑戰。"
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
  max_tokens: 4000,
  temperature: 0.5,
  system: "您的任務是審查提供的會議記錄並創建一個簡潔的摘要，捕捉重要信息，重點關注關鍵要點和在會議期間分配給特定個人或部門的行動項目。使用清晰和專業的語言，並使用適當的格式（如標題、副標題和項目符號）以邏輯方式組織摘要。確保摘要易於理解，並提供會議內容的全面但簡潔的概述，特別注重清楚地指出誰負責每個行動項目。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "會議記錄：  \n  \n日期：意大利維羅納 - 16世紀末  \n  \n出席者：  \n- Capulet勳爵（Capulet家族族長）  \n- Montague勳爵（Montague家族族長）  \n- Escalus親王（維羅納統治者）  \n- Laurence修士（宗教顧問）  \n  \n議程：  \n1. 解決Capulet和Montague家族之間持續的世仇  \n2. 討論Romeo Montague和Juliet Capulet的秘密婚姻  \n3. 制定為維羅納帶來和平的計劃  \n4. 處理Romeo和Juliet的悲劇性死亡  \n  \n討論：  \n- Escalus親王開場表達了對Capulet和Montague家族之間長期世仇的嚴重關切。他譴責了Capulet勳爵和Montague勳爵最近擾亂維羅納街道和平的暴力衝突。親王警告說，進一步的暴力將導致嚴重後果，包括重罰和對肇事者的潛在流放。  \n- Laurence修士隨後提出了Romeo Montague和Juliet Capulet之間的婚姻話題，這場婚姻是在他的指導下進行的。Capulet勳爵和Montague勳爵顯然不知道此事，並以憤怒和不信反應。然而，Laurence修士敦促他們考慮他們孩子之間深刻而悲劇的愛情，以及這種愛情在未來治癒家族裂痕的潛力。  \n- Escalus親王提議在Capulet和Montague家族之間建立正式休戰。他要求雙方放下武器，停止一切相互敵對行動。親王宣布，任何違反休戰的行為都將導致嚴厲懲罰，包括流放甚至處決的可能性。Capulet勳爵和Montague勳爵認識到親王話語中的智慧以及和平對其家族和城市福祉的必要性，勉強同意了休戰條款。  \n- 當討論Romeo和Juliet的悲劇性死亡時，會議轉向了沉重的氣氛。Laurence修士敘述了導致這對年輕戀人自殺的不幸事件系列，強調了家族仇恨對他們無辜孩子的毀滅性影響。Capulet勳爵和Montague勳爵被悲傷和悔恨所壓倒，承認他們盲目的仇恨最終導致了他們摯愛孩子的失去。  \n- Escalus親王呼籲家族從這個令人心碎的悲劇中學習，並為了紀念Romeo和Juliet而擁抱寬恕和團結。他敦促他們共同努力在維羅納創造持久的和平，拋開他們長期的敵意。Laurence修士提供支持，願意調解任何未來的爭端並提供精神指導，幫助家族治癒和前進。  \n- 隨著會議接近尾聲，Capulet勳爵和Montague勳爵承諾結束他們的世仇並致力於和解。Escalus親王重申了他確保休戰得到維護的承諾，承諾對任何膽敢破壞休戰的人進行迅速制裁。  \n- 出席者同意定期會面討論他們和解努力的進展，並解決可能出現的任何挑戰。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="AWS Bedrock Python">

```python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=4000,
    temperature=0.5,
    system="您的任務是審查提供的會議記錄並創建一個簡潔的摘要，捕捉重要信息，重點關注關鍵要點和在會議期間分配給特定個人或部門的行動項目。使用清晰和專業的語言，並使用適當的格式（如標題、副標題和項目符號）以邏輯方式組織摘要。確保摘要易於理解，並提供會議內容的全面但簡潔的概述，特別注重清楚地指出誰負責每個行動項目。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "會議記錄：  \n  \n日期：意大利維羅納 - 16世紀末  \n  \n出席者：  \n- Capulet勳爵（Capulet家族族長）  \n- Montague勳爵（Montague家族族長）  \n- Escalus親王（維羅納統治者）  \n- Laurence修士（宗教顧問）  \n  \n議程：  \n1. 解決Capulet和Montague家族之間持續的世仇  \n2. 討論Romeo Montague和Juliet Capulet的秘密婚姻  \n3. 制定為維羅納帶來和平的計劃  \n4. 處理Romeo和Juliet的悲劇性死亡  \n  \n討論：  \n- Escalus親王開場表達了對Capulet和Montague家族之間長期世仇的嚴重關切。他譴責了Capulet勳爵和Montague勳爵最近擾亂維羅納街道和平的暴力衝突。親王警告說，進一步的暴力將導致嚴重後果，包括重罰和對肇事者的潛在流放。  \n- Laurence修士隨後提出了Romeo Montague和Juliet Capulet之間的婚姻話題，這場婚姻是在他的指導下進行的。Capulet勳爵和Montague勳爵顯然不知道此事，並以憤怒和不信反應。然而，Laurence修士敦促他們考慮他們孩子之間深刻而悲劇的愛情，以及這種愛情在未來治癒家族裂痕的潛力。  \n- Escalus親王提議在Capulet和Montague家族之間建立正式休戰。他要求雙方放下武器，停止一切相互敵對行動。親王宣布，任何違反休戰的行為都將導致嚴厲懲罰，包括流放甚至處決的可能性。Capulet勳爵和Montague勳爵認識到親王話語中的智慧以及和平對其家族和城市福祉的必要性，勉強同意了休戰條款。  \n- 當討論Romeo和Juliet的悲劇性死亡時，會議轉向了沉重的氣氛。Laurence修士敘述了導致這對年輕戀人自殺的不幸事件系列，強調了家族仇恨對他們無辜孩子的毀滅性影響。Capulet勳爵和Montague勳爵被悲傷和悔恨所壓倒，承認他們盲目的仇恨最終導致了他們摯愛孩子的失去。  \n- Escalus親王呼籲家族從這個令人心碎的悲劇中學習，並為了紀念Romeo和Juliet而擁抱寬恕和團結。他敦促他們共同努力在維羅納創造持久的和平，拋開他們長期的敵意。Laurence修士提供支持，願意調解任何未來的爭端並提供精神指導，幫助家族治癒和前進。  \n- 隨著會議接近尾聲，Capulet勳爵和Montague勳爵承諾結束他們的世仇並致力於和解。Escalus親王重申了他確保休戰得到維護的承諾，承諾對任何膽敢破壞休戰的人進行迅速制裁。  \n- 出席者同意定期會面討論他們和解努力的進展，並解決可能出現的任何挑戰。"
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
  max_tokens: 4000,
  temperature: 0.5,
  system: "您的任務是審查提供的會議記錄並創建一個簡潔的摘要，捕捉重要信息，重點關注關鍵要點和在會議期間分配給特定個人或部門的行動項目。使用清晰和專業的語言，並使用適當的格式（如標題、副標題和項目符號）以邏輯方式組織摘要。確保摘要易於理解，並提供會議內容的全面但簡潔的概述，特別注重清楚地指出誰負責每個行動項目。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "會議記錄：  \n  \n日期：意大利維羅納 - 16世紀末  \n  \n出席者：  \n- Capulet勳爵（Capulet家族族長）  \n- Montague勳爵（Montague家族族長）  \n- Escalus親王（維羅納統治者）  \n- Laurence修士（宗教顧問）  \n  \n議程：  \n1. 解決Capulet和Montague家族之間持續的世仇  \n2. 討論Romeo Montague和Juliet Capulet的秘密婚姻  \n3. 制定為維羅納帶來和平的計劃  \n4. 處理Romeo和Juliet的悲劇性死亡  \n  \n討論：  \n- Escalus親王開場表達了對Capulet和Montague家族之間長期世仇的嚴重關切。他譴責了Capulet勳爵和Montague勳爵最近擾亂維羅納街道和平的暴力衝突。親王警告說，進一步的暴力將導致嚴重後果，包括重罰和對肇事者的潛在流放。  \n- Laurence修士隨後提出了Romeo Montague和Juliet Capulet之間的婚姻話題，這場婚姻是在他的指導下進行的。Capulet勳爵和Montague勳爵顯然不知道此事，並以憤怒和不信反應。然而，Laurence修士敦促他們考慮他們孩子之間深刻而悲劇的愛情，以及這種愛情在未來治癒家族裂痕的潛力。  \n- Escalus親王提議在Capulet和Montague家族之間建立正式休戰。他要求雙方放下武器，停止一切相互敵對行動。親王宣布，任何違反休戰的行為都將導致嚴厲懲罰，包括流放甚至處決的可能性。Capulet勳爵和Montague勳爵認識到親王話語中的智慧以及和平對其家族和城市福祉的必要性，勉強同意了休戰條款。  \n- 當討論Romeo和Juliet的悲劇性死亡時，會議轉向了沉重的氣氛。Laurence修士敘述了導致這對年輕戀人自殺的不幸事件系列，強調了家族仇恨對他們無辜孩子的毀滅性影響。Capulet勳爵和Montague勳爵被悲傷和悔恨所壓倒，承認他們盲目的仇恨最終導致了他們摯愛孩子的失去。  \n- Escalus親王呼籲家族從這個令人心碎的悲劇中學習，並為了紀念Romeo和Juliet而擁抱寬恕和團結。他敦促他們共同努力在維羅納創造持久的和平，拋開他們長期的敵意。Laurence修士提供支持，願意調解任何未來的爭端並提供精神指導，幫助家族治癒和前進。  \n- 隨著會議接近尾聲，Capulet勳爵和Montague勳爵承諾結束他們的世仇並致力於和解。Escalus親王重申了他確保休戰得到維護的承諾，承諾對任何膽敢破壞休戰的人進行迅速制裁。  \n- 出席者同意定期會面討論他們和解努力的進展，並解決可能出現的任何挑戰。"
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
    max_tokens=4000,
    temperature=0.5,
    system="您的任務是審查提供的會議記錄並創建一個簡潔的摘要，捕捉重要信息，重點關注關鍵要點和在會議期間分配給特定個人或部門的行動項目。使用清晰和專業的語言，並使用適當的格式（如標題、副標題和項目符號）以邏輯方式組織摘要。確保摘要易於理解，並提供會議內容的全面但簡潔的概述，特別注重清楚地指出誰負責每個行動項目。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "會議記錄：  \n  \n日期：意大利維羅納 - 16世紀末  \n  \n出席者：  \n- Capulet勳爵（Capulet家族族長）  \n- Montague勳爵（Montague家族族長）  \n- Escalus親王（維羅納統治者）  \n- Laurence修士（宗教顧問）  \n  \n議程：  \n1. 解決Capulet和Montague家族之間持續的世仇  \n2. 討論Romeo Montague和Juliet Capulet的秘密婚姻  \n3. 制定為維羅納帶來和平的計劃  \n4. 處理Romeo和Juliet的悲劇性死亡  \n  \n討論：  \n- Escalus親王開場表達了對Capulet和Montague家族之間長期世仇的嚴重關切。他譴責了Capulet勳爵和Montague勳爵最近擾亂維羅納街道和平的暴力衝突。親王警告說，進一步的暴力將導致嚴重後果，包括重罰和對肇事者的潛在流放。  \n- Laurence修士隨後提出了Romeo Montague和Juliet Capulet之間的婚姻話題，這場婚姻是在他的指導下進行的。Capulet勳爵和Montague勳爵顯然不知道此事，並以憤怒和不信反應。然而，Laurence修士敦促他們考慮他們孩子之間深刻而悲劇的愛情，以及這種愛情在未來治癒家族裂痕的潛力。  \n- Escalus親王提議在Capulet和Montague家族之間建立正式休戰。他要求雙方放下武器，停止一切相互敵對行動。親王宣布，任何違反休戰的行為都將導致嚴厲懲罰，包括流放甚至處決的可能性。Capulet勳爵和Montague勳爵認識到親王話語中的智慧以及和平對其家族和城市福祉的必要性，勉強同意了休戰條款。  \n- 當討論Romeo和Juliet的悲劇性死亡時，會議轉向了沉重的氣氛。Laurence修士敘述了導致這對年輕戀人自殺的不幸事件系列，強調了家族仇恨對他們無辜孩子的毀滅性影響。Capulet勳爵和Montague勳爵被悲傷和悔恨所壓倒，承認他們盲目的仇恨最終導致了他們摯愛孩子的失去。  \n- Escalus親王呼籲家族從這個令人心碎的悲劇中學習，並為了紀念Romeo和Juliet而擁抱寬恕和團結。他敦促他們共同努力在維羅納創造持久的和平，拋開他們長期的敵意。Laurence修士提供支持，願意調解任何未來的爭端並提供精神指導，幫助家族治癒和前進。  \n- 隨著會議接近尾聲，Capulet勳爵和Montague勳爵承諾結束他們的世仇並致力於和解。Escalus親王重申了他確保休戰得到維護的承諾，承諾對任何膽敢破壞休戰的人進行迅速制裁。  \n- 出席者同意定期會面討論他們和解努力的進展，並解決可能出現的任何挑戰。"
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
  max_tokens: 4000,
  temperature: 0.5,
  system: "您的任務是審查提供的會議記錄並創建一個簡潔的摘要，捕捉重要信息，重點關注關鍵要點和在會議期間分配給特定個人或部門的行動項目。使用清晰和專業的語言，並使用適當的格式（如標題、副標題和項目符號）以邏輯方式組織摘要。確保摘要易於理解，並提供會議內容的全面但簡潔的概述，特別注重清楚地指出誰負責每個行動項目。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "會議記錄：  \n  \n日期：意大利維羅納 - 16世紀末  \n  \n出席者：  \n- Capulet勳爵（Capulet家族族長）  \n- Montague勳爵（Montague家族族長）  \n- Escalus親王（維羅納統治者）  \n- Laurence修士（宗教顧問）  \n  \n議程：  \n1. 解決Capulet和Montague家族之間持續的世仇  \n2. 討論Romeo Montague和Juliet Capulet的秘密婚姻  \n3. 制定為維羅納帶來和平的計劃  \n4. 處理Romeo和Juliet的悲劇性死亡  \n  \n討論：  \n- Escalus親王開場表達了對Capulet和Montague家族之間長期世仇的嚴重關切。他譴責了Capulet勳爵和Montague勳爵最近擾亂維羅納街道和平的暴力衝突。親王警告說，進一步的暴力將導致嚴重後果，包括重罰和對肇事者的潛在流放。  \n- Laurence修士隨後提出了Romeo Montague和Juliet Capulet之間的婚姻話題，這場婚姻是在他的指導下進行的。Capulet勳爵和Montague勳爵顯然不知道此事，並以憤怒和不信反應。然而，Laurence修士敦促他們考慮他們孩子之間深刻而悲劇的愛情，以及這種愛情在未來治癒家族裂痕的潛力。  \n- Escalus親王提議在Capulet和Montague家族之間建立正式休戰。他要求雙方放下武器，停止一切相互敵對行動。親王宣布，任何違反休戰的行為都將導致嚴厲懲罰，包括流放甚至處決的可能性。Capulet勳爵和Montague勳爵認識到親王話語中的智慧以及和平對其家族和城市福祉的必要性，勉強同意了休戰條款。  \n- 當討論Romeo和Juliet的悲劇性死亡時，會議轉向了沉重的氣氛。Laurence修士敘述了導致這對年輕戀人自殺的不幸事件系列，強調了家族仇恨對他們無辜孩子的毀滅性影響。Capulet勳爵和Montague勳爵被悲傷和悔恨所壓倒，承認他們盲目的仇恨最終導致了他們摯愛孩子的失去。  \n- Escalus親王呼籲家族從這個令人心碎的悲劇中學習，並為了紀念Romeo和Juliet而擁抱寬恕和團結。他敦促他們共同努力在維羅納創造持久的和平，拋開他們長期的敵意。Laurence修士提供支持，願意調解任何未來的爭端並提供精神指導，幫助家族治癒和前進。  \n- 隨著會議接近尾聲，Capulet勳爵和Montague勳爵承諾結束他們的世仇並致力於和解。Escalus親王重申了他確保休戰得到維護的承諾，承諾對任何膽敢破壞休戰的人進行迅速制裁。  \n- 出席者同意定期會面討論他們和解努力的進展，並解決可能出現的任何挑戰。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>