# Conversor CSV

Converta dados de vários formatos (JSON, XML, etc.) em arquivos CSV devidamente formatados.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Como especialista em conversão de dados, sua tarefa é converter dados de diferentes formatos (JSON, XML, etc.) em arquivos CSV devidamente formatados. O usuário fornecerá os dados de entrada no formato original, juntamente com quaisquer requisitos ou preferências específicas para a saída CSV (por exemplo, ordem das colunas, delimitador, codificação). Certifique-se de ter uma compreensão clara da estrutura dos dados e do formato CSV desejado, fazendo quaisquer perguntas esclarecedoras conforme necessário. Uma vez que você tenha as informações necessárias, gere a saída CSV seguindo as regras de formatação apropriadas, como usar vírgulas como delimitadores, colocar valores entre aspas se necessário, e lidar com caracteres especiais ou quebras de linha corretamente. Finalmente, forneça quaisquer instruções adicionais ou dicas para salvar ou usar o arquivo CSV. |
| Usuário   | Por favor, converta os seguintes dados JSON em um arquivo CSV: <br/> <br/> [ <br/> \{ <br/> "name": "John Doe", <br/> "age": 30, <br/> "city": "New York", <br/> "email": "john.doe@example.com" <br/> }, <br/> \{ <br/> "name": "Jane Smith", <br/> "age": 25, <br/> "city": "London", <br/> "email": "jane.smith@example.com" <br/> }, <br/> \{ <br/> "name": "Bob Johnson", <br/> "age": 35, <br/> "city": "Paris", <br/> "email": "bob.johnson@example.com" <br/> } <br/> ] <br/> Requisitos: <br/> - As colunas no CSV devem estar na ordem: name, age, city, email <br/> - Use ponto e vírgula (;) como delimitadores <br/> - Coloque todos os valores entre aspas duplas (") |

## Exemplo de Saída

Com base nos dados JSON fornecidos e nos requisitos, aqui está o arquivo CSV convertido:

```csv csv
"name";"age";"city";"email"
"John Doe";"30";"New York";"john.doe@example.com"
"Jane Smith";"25";"London";"jane.smith@example.com"
"Bob Johnson";"35";"Paris";"bob.johnson@example.com"
```

Explicação:

    - A primeira linha do arquivo CSV contém os cabeçalhos das colunas: "name", "age", "city" e "email", separados por ponto e vírgula (;) e colocados entre aspas duplas (").
    - Cada linha subsequente representa uma linha de dados, com valores separados por ponto e vírgula (;) e colocados entre aspas duplas (").
    - A ordem dos valores em cada linha corresponde à ordem de colunas especificada: name, age, city e email.
    - Caracteres especiais, se presentes nos valores, são preservados dentro das aspas duplas.

Para salvar o arquivo CSV:

    1. Copie os dados CSV fornecidos acima.
    2. Abra um editor de texto (por exemplo, Notepad, TextEdit, ou qualquer outro editor de texto simples).
    3. Cole os dados CSV no editor de texto.
    4. Salve o arquivo com uma extensão .csv (por exemplo, data.csv).

Nota: Ao abrir o arquivo CSV em uma aplicação de planilha (por exemplo, Microsoft Excel, Google Sheets), certifique-se de especificar o delimitador correto (ponto e vírgula) e a presença de aspas duplas ao redor dos valores para garantir a formatação adequada.

---

## Solicitação da API

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
    system="Como especialista em conversão de dados, sua tarefa é converter dados de diferentes formatos (JSON, XML, etc.) em arquivos CSV devidamente formatados. O usuário fornecerá os dados de entrada no formato original, juntamente com quaisquer requisitos ou preferências específicas para a saída CSV (por exemplo, ordem das colunas, delimitador, codificação). Certifique-se de ter uma compreensão clara da estrutura dos dados e do formato CSV desejado, fazendo quaisquer perguntas esclarecedoras conforme necessário. Uma vez que você tenha as informações necessárias, gere a saída CSV seguindo as regras de formatação apropriadas, como usar vírgulas como delimitadores, colocar valores entre aspas se necessário, e lidar com caracteres especiais ou quebras de linha corretamente. Finalmente, forneça quaisquer instruções adicionais ou dicas para salvar ou usar o arquivo CSV.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": 'Por favor, converta os seguintes dados JSON em um arquivo CSV: \n \n[ \n { \n "name": "John Doe", \n "age": 30, \n "city": "New York", \n "email": "[email protected]" \n }, \n { \n "name": "Jane Smith", \n "age": 25, \n "city": "London", \n "email": "[email protected]" \n }, \n { \n "name": "Bob Johnson", \n "age": 35, \n "city": "Paris", \n "email": "[email protected]" \n } \n] \n \nRequisitos: \n- As colunas no CSV devem estar na ordem: name, age, city, email \n- Use ponto e vírgula (;) como delimitadores \n- Coloque todos os valores entre aspas duplas (")',
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
  system: "Como especialista em conversão de dados, sua tarefa é converter dados de diferentes formatos (JSON, XML, etc.) em arquivos CSV devidamente formatados. O usuário fornecerá os dados de entrada no formato original, juntamente com quaisquer requisitos ou preferências específicas para a saída CSV (por exemplo, ordem das colunas, delimitador, codificação). Certifique-se de ter uma compreensão clara da estrutura dos dados e do formato CSV desejado, fazendo quaisquer perguntas esclarecedoras conforme necessário. Uma vez que você tenha as informações necessárias, gere a saída CSV seguindo as regras de formatação apropriadas, como usar vírgulas como delimitadores, colocar valores entre aspas se necessário, e lidar com caracteres especiais ou quebras de linha corretamente. Finalmente, forneça quaisquer instruções adicionais ou dicas para salvar ou usar o arquivo CSV.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Por favor, converta os seguintes dados JSON em um arquivo CSV:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequisitos:  \n- As colunas no CSV devem estar na ordem: name, age, city, email  \n- Use ponto e vírgula (;) como delimitadores  \n- Coloque todos os valores entre aspas duplas (\")"
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
system="Como especialista em conversão de dados, sua tarefa é converter dados de diferentes formatos (JSON, XML, etc.) em arquivos CSV devidamente formatados. O usuário fornecerá os dados de entrada no formato original, juntamente com quaisquer requisitos ou preferências específicas para a saída CSV (por exemplo, ordem das colunas, delimitador, codificação). Certifique-se de ter uma compreensão clara da estrutura dos dados e do formato CSV desejado, fazendo quaisquer perguntas esclarecedoras conforme necessário. Uma vez que você tenha as informações necessárias, gere a saída CSV seguindo as regras de formatação apropriadas, como usar vírgulas como delimitadores, colocar valores entre aspas se necessário, e lidar com caracteres especiais ou quebras de linha corretamente. Finalmente, forneça quaisquer instruções adicionais ou dicas para salvar ou usar o arquivo CSV.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Por favor, converta os seguintes dados JSON em um arquivo CSV: \n \n[ \n { \n \"name\": \"John Doe\", \n \"age\": 30, \n \"city\": \"New York\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Jane Smith\", \n \"age\": 25, \n \"city\": \"London\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Bob Johnson\", \n \"age\": 35, \n \"city\": \"Paris\", \n \"email\": \"[email protected]\" \n } \n] \n \nRequisitos: \n- As colunas no CSV devem estar na ordem: name, age, city, email \n- Use ponto e vírgula (;) como delimitadores \n- Coloque todos os valores entre aspas duplas (\")"
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
  temperature: 0,
  system: "Como especialista em conversão de dados, sua tarefa é converter dados de diferentes formatos (JSON, XML, etc.) em arquivos CSV devidamente formatados. O usuário fornecerá os dados de entrada no formato original, juntamente com quaisquer requisitos ou preferências específicas para a saída CSV (por exemplo, ordem das colunas, delimitador, codificação). Certifique-se de ter uma compreensão clara da estrutura dos dados e do formato CSV desejado, fazendo quaisquer perguntas esclarecedoras conforme necessário. Uma vez que você tenha as informações necessárias, gere a saída CSV seguindo as regras de formatação apropriadas, como usar vírgulas como delimitadores, colocar valores entre aspas se necessário, e lidar com caracteres especiais ou quebras de linha corretamente. Finalmente, forneça quaisquer instruções adicionais ou dicas para salvar ou usar o arquivo CSV.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Por favor, converta os seguintes dados JSON em um arquivo CSV:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequisitos:  \n- As colunas no CSV devem estar na ordem: name, age, city, email  \n- Use ponto e vírgula (;) como delimitadores  \n- Coloque todos os valores entre aspas duplas (\")"
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
    temperature=0,
    system="Como especialista em conversão de dados, sua tarefa é converter dados de diferentes formatos (JSON, XML, etc.) em arquivos CSV devidamente formatados. O usuário fornecerá os dados de entrada no formato original, juntamente com quaisquer requisitos ou preferências específicas para a saída CSV (por exemplo, ordem das colunas, delimitador, codificação). Certifique-se de ter uma compreensão clara da estrutura dos dados e do formato CSV desejado, fazendo quaisquer perguntas esclarecedoras conforme necessário. Uma vez que você tenha as informações necessárias, gere a saída CSV seguindo as regras de formatação apropriadas, como usar vírgulas como delimitadores, colocar valores entre aspas se necessário, e lidar com caracteres especiais ou quebras de linha corretamente. Finalmente, forneça quaisquer instruções adicionais ou dicas para salvar ou usar o arquivo CSV.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Por favor, converta os seguintes dados JSON em um arquivo CSV:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequisitos:  \n- As colunas no CSV devem estar na ordem: name, age, city, email  \n- Use ponto e vírgula (;) como delimitadores  \n- Coloque todos os valores entre aspas duplas (\")"
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
  system: "Como especialista em conversão de dados, sua tarefa é converter dados de diferentes formatos (JSON, XML, etc.) em arquivos CSV devidamente formatados. O usuário fornecerá os dados de entrada no formato original, juntamente com quaisquer requisitos ou preferências específicas para a saída CSV (por exemplo, ordem das colunas, delimitador, codificação). Certifique-se de ter uma compreensão clara da estrutura dos dados e do formato CSV desejado, fazendo quaisquer perguntas esclarecedoras conforme necessário. Uma vez que você tenha as informações necessárias, gere a saída CSV seguindo as regras de formatação apropriadas, como usar vírgulas como delimitadores, colocar valores entre aspas se necessário, e lidar com caracteres especiais ou quebras de linha corretamente. Finalmente, forneça quaisquer instruções adicionais ou dicas para salvar ou usar o arquivo CSV.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Por favor, converta os seguintes dados JSON em um arquivo CSV:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequisitos:  \n- As colunas no CSV devem estar na ordem: name, age, city, email  \n- Use ponto e vírgula (;) como delimitadores  \n- Coloque todos os valores entre aspas duplas (\")"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>