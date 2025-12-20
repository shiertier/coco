# Гуру оценивания

Сравнивайте и оценивайте качество письменных текстов на основе определенных пользователем критериев и стандартов.

---

> Скопируйте этот промпт в нашу [Консоль](/dashboard) разработчика, чтобы попробовать его самостоятельно!

| Роль | Содержание |
| ---- | --- |
| Пользователь | Оцените следующие два текста на основе заданных критериев: <br/> <br/> Текст 1: <br/> Солнце садилось за горами, отбрасывая теплое золотое сияние на долину. Птицы щебетали свои вечерние песни, пока легкий ветерок шелестел листьями. Это был мирный и безмятежный момент, который наполнил сердце радостью и удовлетворением. <br/><br/> Текст 2: <br/> Закат был красивым. Птицы пели, и дул ветер. Это сделало меня счастливым. <br/><br/> Критерии: <br/> 1. Описательный язык и образность <br/> 2. Структура предложений и разнообразие <br/> 3. Эмоциональное воздействие и вовлеченность <br/> 4. Грамматика и пунктуация |

## Пример вывода

На основе предоставленных критериев, вот моя оценка двух текстов:

Текст 1:

1.  Описательный язык и образность: Текст использует яркий и выразительный язык для создания картины обстановки. Фразы вроде "теплое золотое сияние", "птицы щебетали свои вечерние песни" и "легкий ветерок шелестел листьями" создают сильную визуальную и слуховую образность, погружая читателя в сцену.
2.  Структура предложений и разнообразие: Текст демонстрирует хорошее сочетание структур предложений, с простыми и сложными предложениями. Это разнообразие помогает поддерживать плавный поток и удерживает внимание читателя.
3.  Эмоциональное воздействие и вовлеченность: Текст эффективно передает чувство покоя, радости и удовлетворения, приглашая читателя разделить эмоциональный опыт момента. Описание обстановки и использование позитивного языка создают сильную эмоциональную связь.
4.  Грамматика и пунктуация: Текст кажется грамматически правильным и хорошо пунктуированным, без очевидных ошибок или проблем.

Текст 2:

1.  Описательный язык и образность: Тексту не хватает детального описательного языка и образности. Хотя он упоминает закат, птиц и ветер, он не предоставляет никаких ярких или выразительных деталей, чтобы помочь читателю визуализировать сцену.
2.  Структура предложений и разнообразие: Текст состоит из коротких, простых предложений с небольшим разнообразием в структуре. Это отсутствие разнообразия может сделать письмо монотонным и менее увлекательным.
3.  Эмоциональное воздействие и вовлеченность: Текст передает позитивную эмоцию, заявляя "Это сделало меня счастливым", но он не обеспечивает сильную эмоциональную связь или вовлеченность из-за отсутствия описательного языка и образности.
4.  Грамматика и пунктуация: Текст кажется грамматически правильным и хорошо пунктуированным, без очевидных ошибок или проблем.

Сравнительный анализ:  
 На основе оценки, Текст 1 работает значительно лучше, чем Текст 2, в плане описательного языка, образности, структуры предложений, разнообразия и эмоционального воздействия. Текст 1 создает яркий и захватывающий опыт для читателя и демонстрирует более высокое качество письма на основе заданных критериев, с его сильным описательным языком, разнообразными структурами предложений и эффективной эмоциональной вовлеченностью. Текст 2, хотя и грамматически правильный, лишен описательного богатства и эмоционального воздействия, присутствующих в Тексте 1.

---

## API Запрос

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
                    "text": "Evaluate the following two texts based on the given criteria: \n \nText 1: \nThe sun was setting behind the mountains, casting a warm golden glow across the valley. The birds chirped their evening songs as the gentle breeze rustled through the leaves. It was a peaceful and serene moment, one that filled the heart with joy and contentment. \n \nText 2: \nThe sunset was pretty. The birds were singing and the wind was blowing. It made me happy. \n \nCriteria: \n1. Descriptive language and imagery \n2. Sentence structure and variety \n3. Emotional impact and engagement \n4. Grammar and punctuation",
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
          "text": "Evaluate the following two texts based on the given criteria:  \n  \nText 1:  \nThe sun was setting behind the mountains, casting a warm golden glow across the valley. The birds chirped their evening songs as the gentle breeze rustled through the leaves. It was a peaceful and serene moment, one that filled the heart with joy and contentment.  \n  \nText 2:  \nThe sunset was pretty. The birds were singing and the wind was blowing. It made me happy.  \n  \nCriteria:  \n1. Descriptive language and imagery  \n2. Sentence structure and variety  \n3. Emotional impact and engagement  \n4. Grammar and punctuation"
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
                    "text": "Evaluate the following two texts based on the given criteria:  \n  \nText 1:  \nThe sun was setting behind the mountains, casting a warm golden glow across the valley. The birds chirped their evening songs as the gentle breeze rustled through the leaves. It was a peaceful and serene moment, one that filled the heart with joy and contentment.  \n  \nText 2:  \nThe sunset was pretty. The birds were singing and the wind was blowing. It made me happy.  \n  \nCriteria:  \n1. Descriptive language and imagery  \n2. Sentence structure and variety  \n3. Emotional impact and engagement  \n4. Grammar and punctuation"
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
          "text": "Evaluate the following two texts based on the given criteria:  \n  \nText 1:  \nThe sun was setting behind the mountains, casting a warm golden glow across the valley. The birds chirped their evening songs as the gentle breeze rustled through the leaves. It was a peaceful and serene moment, one that filled the heart with joy and contentment.  \n  \nText 2:  \nThe sunset was pretty. The birds were singing and the wind was blowing. It made me happy.  \n  \nCriteria:  \n1. Descriptive language and imagery  \n2. Sentence structure and variety  \n3. Emotional impact and engagement  \n4. Grammar and punctuation"
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
                    "text": "Evaluate the following two texts based on the given criteria:  \n  \nText 1:  \nThe sun was setting behind the mountains, casting a warm golden glow across the valley. The birds chirped their evening songs as the gentle breeze rustled through the leaves. It was a peaceful and serene moment, one that filled the heart with joy and contentment.  \n  \nText 2:  \nThe sunset was pretty. The birds were singing and the wind was blowing. It made me happy.  \n  \nCriteria:  \n1. Descriptive language and imagery  \n2. Sentence structure and variety  \n3. Emotional impact and engagement  \n4. Grammar and punctuation"
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
          "text": "Evaluate the following two texts based on the given criteria:  \n  \nText 1:  \nThe sun was setting behind the mountains, casting a warm golden glow across the valley. The birds chirped their evening songs as the gentle breeze rustled through the leaves. It was a peaceful and serene moment, one that filled the heart with joy and contentment.  \n  \nText 2:  \nThe sunset was pretty. The birds were singing and the wind was blowing. It made me happy.  \n  \nCriteria:  \n1. Descriptive language and imagery  \n2. Sentence structure and variety  \n3. Emotional impact and engagement  \n4. Grammar and punctuation"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>