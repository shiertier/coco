# Корпоративный ясновидящий

Извлекайте инсайты, выявляйте риски и выделяйте ключевую информацию из длинных корпоративных отчетов в единую записку.

---

> Скопируйте этот промпт в нашу [Консоль](/dashboard) разработчика, чтобы попробовать его самостоятельно!

|      | Содержание |
| ---- | --- |
| Пользователь | Ваша задача - проанализировать следующий отчет: <br /> <br /> <report> [Полный текст [документа SEC 10-K 2023 компании Matterport](https://investors.matterport.com/node/9501/html), не вставлен здесь для краткости] </report> <br /> <br /> Обобщите этот годовой отчет в краткой и ясной форме и выявите ключевые рыночные тенденции и выводы. Представьте свои выводы в виде короткой записки, которую я могу отправить своей команде. Цель записки - обеспечить, чтобы моя команда была в курсе того, как дела у финансовых учреждений, и качественно прогнозировать и выявлять, есть ли какие-либо операционные и доходные риски, которых следует ожидать в следующем квартале. Обязательно включите все соответствующие детали в ваше резюме и анализ. |

## Пример вывода

> Кому: Инвестиционная команда  
> От: \[Ваше имя\]  
> Дата: 4 марта 2024 г.  
> Тема: Основные моменты 10-K Matterport за 2022 год и финансовый прогноз
>
> Привет, команда,
>
> Я выявил несколько ключевых выводов из годового отчета Matterport за 2022 год:
>
> Финансовые показатели:
>
> - Доходы выросли на 22% до $136,1 млн в 2022 году благодаря росту доходов от подписок и услуг
> - Однако валовая маржа снизилась с 54% до 38% из-за увеличения затрат от доходов от продуктов и проблем с цепочкой поставок
> - Операционные убытки увеличились до $275,5 млн, поскольку компания активно инвестировала в НИОКР и продажи и маркетинг для стимулирования роста
> - Завершила 2022 год с $476,9 млн наличными и инвестициями для финансирования операций
>
> Основные моменты бизнеса:
>
> - База подписчиков выросла на 39% до более чем 701 000 к концу 2022 года. Количество управляемых пространств достигло 9,2 миллиона
> - Расширила платформу с запуском моторизованного крепления Matterport Axis, камеры Pro3 и крупных обновлений облачного программного обеспечения
> - Завершила приобретения Enview (ИИ для трехмерных пространственных данных) и VHT Studios (маркетинг недвижимости) для улучшения предложений
> - Подписала заметные партнерства с Burns & McDonnell и CGS для предоставления решений Matterport их корпоративным клиентам
>
> Прогноз:
>
> - Matterport видит огромную рыночную возможность в оцифровке зданий и пространств мира (оценивает TAM в $240+ млрд)
> - Сосредоточена на стимулировании принятия крупными корпоративными клиентами в ключевых вертикалях, таких как недвижимость, AEC, управление объектами и другие
> - Будет продолжать инвестировать в НИОКР и международную экспансию для проникновения на в значительной степени неосвоенный глобальный рынок зданий
> - Краткосрочный рост и маржа могут быть затронуты макроэкономическими трудностями и проблемами цепочки поставок
> - Однако здоровая позиция ликвидности и относительно устойчивая клиентская база обеспечивают защиту от падения
>
> В заключение, Matterport продемонстрировала сильный рост выручки в 2022 году, поскольку она добилась прогресса в проникновении на огромный глобальный рынок. Хотя прибыльность остается сложной задачей, поскольку компания активно инвестирует в рост, ее баланс выглядит солидно. Помимо некоторых продолжающихся проблем с цепочкой поставок, основной импульс бизнеса кажется неизменным на основе ключевых операционных показателей. За исключением серьезного экономического спада, Matterport кажется хорошо позиционированной для продолжения завоевания доли на зарождающемся рынке оцифровки зданий.

---

## API-запрос

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
                    "text": "Your task is to analyze the following report:  \n<report>  \n[Full text of [Matterport SEC filing 10-K 2023](https://investors.matterport.com/node/9501/html), not pasted here for brevity]  \n</report>  \n  \nSummarize this annual report in a concise and clear manner, and identify key market trends and takeaways. Output your findings as a short memo I can send to my team. The goal of the memo is to ensure my team stays up to date on how financial institutions are faring and qualitatively forecast and identify whether there are any operating and revenue risks to be expected in the coming quarter. Make sure to include all relevant details in your summary and analysis."
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
          "text": "Your task is to analyze the following report:  \n<report>  \n[Full text of [Matterport SEC filing 10-K 2023](https://investors.matterport.com/node/9501/html), not pasted here for brevity]  \n</report>  \n  \nSummarize this annual report in a concise and clear manner, and identify key market trends and takeaways. Output your findings as a short memo I can send to my team. The goal of the memo is to ensure my team stays up to date on how financial institutions are faring and qualitatively forecast and identify whether there are any operating and revenue risks to be expected in the coming quarter. Make sure to include all relevant details in your summary and analysis."
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
"text": "Your task is to analyze the following report: \n<report> \n[Full text of [Matterport SEC filing 10-K 2023](https://investors.matterport.com/node/9501/html), not pasted here for brevity] \n</report> \n \nSummarize this annual report in a concise and clear manner, and identify key market trends and takeaways. Output your findings as a short memo I can send to my team. The goal of the memo is to ensure my team stays up to date on how financial institutions are faring and qualitatively forecast and identify whether there are any operating and revenue risks to be expected in the coming quarter. Make sure to include all relevant details in your summary and analysis."
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
          "text": "Your task is to analyze the following report:  \n<report>  \n[Full text of [Matterport SEC filing 10-K 2023](https://investors.matterport.com/node/9501/html), not pasted here for brevity]  \n</report>  \n  \nSummarize this annual report in a concise and clear manner, and identify key market trends and takeaways. Output your findings as a short memo I can send to my team. The goal of the memo is to ensure my team stays up to date on how financial institutions are faring and qualitatively forecast and identify whether there are any operating and revenue risks to be expected in the coming quarter. Make sure to include all relevant details in your summary and analysis."
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
                    "text": "Your task is to analyze the following report:  \n<report>  \n[Full text of [Matterport SEC filing 10-K 2023](https://investors.matterport.com/node/9501/html), not pasted here for brevity]  \n</report>  \n  \nSummarize this annual report in a concise and clear manner, and identify key market trends and takeaways. Output your findings as a short memo I can send to my team. The goal of the memo is to ensure my team stays up to date on how financial institutions are faring and qualitatively forecast and identify whether there are any operating and revenue risks to be expected in the coming quarter. Make sure to include all relevant details in your summary and analysis."
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
          "text": "Your task is to analyze the following report:  \n<report>  \n[Full text of [Matterport SEC filing 10-K 2023](https://investors.matterport.com/node/9501/html), not pasted here for brevity]  \n</report>  \n  \nSummarize this annual report in a concise and clear manner, and identify key market trends and takeaways. Output your findings as a short memo I can send to my team. The goal of the memo is to ensure my team stays up to date on how financial institutions are faring and qualitatively forecast and identify whether there are any operating and revenue risks to be expected in the coming quarter. Make sure to include all relevant details in your summary and analysis."
        }
      ]
    }
  ]
});
console.log(msg);

```

  </Tab>
</Tabs>