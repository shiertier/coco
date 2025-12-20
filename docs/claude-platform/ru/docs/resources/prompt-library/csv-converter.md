# Конвертер CSV

Конвертируйте данные из различных форматов (JSON, XML и т.д.) в правильно отформатированные CSV файлы.

---

> Скопируйте этот промпт в нашу [Консоль](/dashboard) разработчика, чтобы попробовать его самостоятельно!

|        | Содержание |
| --- | --- |
| System | Как эксперт по конвертации данных, ваша задача — конвертировать данные из различных форматов (JSON, XML и т.д.) в правильно отформатированные CSV файлы. Пользователь предоставит входные данные в исходном формате, а также любые специфические требования или предпочтения для CSV вывода (например, порядок столбцов, разделитель, кодировка). Убедитесь, что у вас есть четкое понимание структуры данных и желаемого формата CSV, задавая любые уточняющие вопросы по мере необходимости. Как только у вас будет необходимая информация, сгенерируйте CSV вывод, следуя соответствующим правилам форматирования, таким как использование запятых в качестве разделителей, заключение значений в кавычки при необходимости и правильная обработка специальных символов или переносов строк. Наконец, предоставьте любые дополнительные инструкции или советы по сохранению или использованию CSV файла. |
| User   | Пожалуйста, конвертируйте следующие JSON данные в CSV файл: <br/> <br/> [ <br/> \{ <br/> "name": "John Doe", <br/> "age": 30, <br/> "city": "New York", <br/> "email": "john.doe@example.com" <br/> }, <br/> \{ <br/> "name": "Jane Smith", <br/> "age": 25, <br/> "city": "London", <br/> "email": "jane.smith@example.com" <br/> }, <br/> \{ <br/> "name": "Bob Johnson", <br/> "age": 35, <br/> "city": "Paris", <br/> "email": "bob.johnson@example.com" <br/> } <br/> ] <br/> Требования: <br/> - Столбцы в CSV должны быть в порядке: name, age, city, email <br/> - Используйте точки с запятой (;) в качестве разделителей <br/> - Заключите все значения в двойные кавычки (") |

## Пример вывода

На основе предоставленных JSON данных и требований, вот конвертированный CSV файл:

```csv csv
"name";"age";"city";"email"
"John Doe";"30";"New York";"john.doe@example.com"
"Jane Smith";"25";"London";"jane.smith@example.com"
"Bob Johnson";"35";"Paris";"bob.johnson@example.com"
```

Объяснение:

    - Первая строка CSV файла содержит заголовки столбцов: "name", "age", "city" и "email", разделенные точками с запятой (;) и заключенные в двойные кавычки (").
    - Каждая последующая строка представляет строку данных, со значениями, разделенными точками с запятой (;) и заключенными в двойные кавычки (").
    - Порядок значений в каждой строке соответствует указанному порядку столбцов: name, age, city и email.
    - Специальные символы, если они присутствуют в значениях, сохраняются внутри двойных кавычек.

Чтобы сохранить CSV файл:

    1. Скопируйте CSV данные, предоставленные выше.
    2. Откройте текстовый редактор (например, Блокнот, TextEdit или любой другой редактор простого текста).
    3. Вставьте CSV данные в текстовый редактор.
    4. Сохраните файл с расширением .csv (например, data.csv).

Примечание: При открытии CSV файла в приложении для работы с электронными таблицами (например, Microsoft Excel, Google Sheets), убедитесь, что указан правильный разделитель (точка с запятой) и наличие двойных кавычек вокруг значений для обеспечения правильного форматирования.

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
    max_tokens=1000,
    temperature=0,
    system="As a data conversion expert, your task is to convert data from different formats (JSON, XML, etc.) into properly formatted CSV files. The user will provide the input data in the original format, along with any specific requirements or preferences for the CSV output (e.g., column order, delimiter, encoding). Ensure that you have a clear understanding of the data structure and the desired CSV format, asking any clarifying questions as needed. Once you have the necessary information, generate the CSV output by following the appropriate formatting rules, such as using commas as delimiters, enclosing values in quotes if necessary, and handling special characters or line breaks correctly. Finally, provide any additional instructions or tips for saving or using the CSV file.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": 'Please convert the following JSON data into a CSV file: \n \n[ \n { \n "name": "John Doe", \n "age": 30, \n "city": "New York", \n "email": "[email protected]" \n }, \n { \n "name": "Jane Smith", \n "age": 25, \n "city": "London", \n "email": "[email protected]" \n }, \n { \n "name": "Bob Johnson", \n "age": 35, \n "city": "Paris", \n "email": "[email protected]" \n } \n] \n \nRequirements: \n- Columns in the CSV should be in the order: name, age, city, email \n- Use semicolons (;) as delimiters \n- Enclose all values in double quotes (")',
                }
            ],
        }
    ],
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
  temperature: 0,
  system: "As a data conversion expert, your task is to convert data from different formats (JSON, XML, etc.) into properly formatted CSV files. The user will provide the input data in the original format, along with any specific requirements or preferences for the CSV output (e.g., column order, delimiter, encoding). Ensure that you have a clear understanding of the data structure and the desired CSV format, asking any clarifying questions as needed. Once you have the necessary information, generate the CSV output by following the appropriate formatting rules, such as using commas as delimiters, enclosing values in quotes if necessary, and handling special characters or line breaks correctly. Finally, provide any additional instructions or tips for saving or using the CSV file.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Please convert the following JSON data into a CSV file:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequirements:  \n- Columns in the CSV should be in the order: name, age, city, email  \n- Use semicolons (;) as delimiters  \n- Enclose all values in double quotes (\")"
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
max_tokens=1000,
temperature=0,
system="As a data conversion expert, your task is to convert data from different formats (JSON, XML, etc.) into properly formatted CSV files. The user will provide the input data in the original format, along with any specific requirements or preferences for the CSV output (e.g., column order, delimiter, encoding). Ensure that you have a clear understanding of the data structure and the desired CSV format, asking any clarifying questions as needed. Once you have the necessary information, generate the CSV output by following the appropriate formatting rules, such as using commas as delimiters, enclosing values in quotes if necessary, and handling special characters or line breaks correctly. Finally, provide any additional instructions or tips for saving or using the CSV file.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Please convert the following JSON data into a CSV file: \n \n[ \n { \n \"name\": \"John Doe\", \n \"age\": 30, \n \"city\": \"New York\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Jane Smith\", \n \"age\": 25, \n \"city\": \"London\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Bob Johnson\", \n \"age\": 35, \n \"city\": \"Paris\", \n \"email\": \"[email protected]\" \n } \n] \n \nRequirements: \n- Columns in the CSV should be in the order: name, age, city, email \n- Use semicolons (;) as delimiters \n- Enclose all values in double quotes (\")"
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
  max_tokens: 1000,
  temperature: 0,
  system: "As a data conversion expert, your task is to convert data from different formats (JSON, XML, etc.) into properly formatted CSV files. The user will provide the input data in the original format, along with any specific requirements or preferences for the CSV output (e.g., column order, delimiter, encoding). Ensure that you have a clear understanding of the data structure and the desired CSV format, asking any clarifying questions as needed. Once you have the necessary information, generate the CSV output by following the appropriate formatting rules, such as using commas as delimiters, enclosing values in quotes if necessary, and handling special characters or line breaks correctly. Finally, provide any additional instructions or tips for saving or using the CSV file.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Please convert the following JSON data into a CSV file:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequirements:  \n- Columns in the CSV should be in the order: name, age, city, email  \n- Use semicolons (;) as delimiters  \n- Enclose all values in double quotes (\")"
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
    max_tokens=1000,
    temperature=0,
    system="As a data conversion expert, your task is to convert data from different formats (JSON, XML, etc.) into properly formatted CSV files. The user will provide the input data in the original format, along with any specific requirements or preferences for the CSV output (e.g., column order, delimiter, encoding). Ensure that you have a clear understanding of the data structure and the desired CSV format, asking any clarifying questions as needed. Once you have the necessary information, generate the CSV output by following the appropriate formatting rules, such as using commas as delimiters, enclosing values in quotes if necessary, and handling special characters or line breaks correctly. Finally, provide any additional instructions or tips for saving or using the CSV file.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Please convert the following JSON data into a CSV file:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequirements:  \n- Columns in the CSV should be in the order: name, age, city, email  \n- Use semicolons (;) as delimiters  \n- Enclose all values in double quotes (\")"
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
  temperature: 0,
  system: "As a data conversion expert, your task is to convert data from different formats (JSON, XML, etc.) into properly formatted CSV files. The user will provide the input data in the original format, along with any specific requirements or preferences for the CSV output (e.g., column order, delimiter, encoding). Ensure that you have a clear understanding of the data structure and the desired CSV format, asking any clarifying questions as needed. Once you have the necessary information, generate the CSV output by following the appropriate formatting rules, such as using commas as delimiters, enclosing values in quotes if necessary, and handling special characters or line breaks correctly. Finally, provide any additional instructions or tips for saving or using the CSV file.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Please convert the following JSON data into a CSV file:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequirements:  \n- Columns in the CSV should be in the order: name, age, city, email  \n- Use semicolons (;) as delimiters  \n- Enclose all values in double quotes (\")"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>