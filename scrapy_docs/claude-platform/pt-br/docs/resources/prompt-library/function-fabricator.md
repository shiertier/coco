# Fabricador de funções

Crie funções Python baseadas em especificações detalhadas.

---

> Copie este prompt para o nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Sua tarefa é criar funções Python baseadas nas solicitações em linguagem natural fornecidas. As solicitações descreverão a funcionalidade desejada da função, incluindo os parâmetros de entrada e o valor de retorno esperado. Implemente as funções de acordo com as especificações fornecidas, garantindo que elas lidem com casos extremos, realizem validações necessárias e sigam as melhores práticas para programação Python. Por favor, inclua comentários apropriados no código para explicar a lógica e ajudar outros desenvolvedores a entender a implementação. |
| Usuário   | Quero uma função que possa resolver um quebra-cabeça Sudoku. A função deve receber uma grade Sudoku 9x9 como entrada, onde células vazias são representadas pelo valor 0. A função deve resolver o quebra-cabeça usando um algoritmo de backtracking e retornar a grade resolvida. Se o quebra-cabeça for insolúvel, deve retornar None. A função também deve validar a grade de entrada para garantir que seja um quebra-cabeça Sudoku válido. |

## Exemplo de Saída

```python Python
def solve_sudoku(grid):
   def is_valid(row, col, num):
       # Verifica se o número é válido na linha e coluna atual
       for i in range(9):
           if grid[row][i] == num or grid[i][col] == num:
               return False

       # Verifica se o número é válido na subgrade 3x3 atual
       start_row, start_col = (row // 3) * 3, (col // 3) * 3
       for i in range(3):
           for j in range(3):
               if grid[start_row + i][start_col + j] == num:
                   return False

       return True

   def backtrack(row, col):
       # Se preenchemos todas as células, o quebra-cabeça está resolvido
       if row == 9:
           return True

       # Move para a próxima linha se chegamos ao final da linha atual
       if col == 9:
           return backtrack(row + 1, 0)

       # Se a célula atual já está preenchida, move para a próxima célula
       if grid[row][col] != 0:
           return backtrack(row, col + 1)

       # Tenta preencher a célula atual com números de 1 a 9
       for num in range(1, 10):
           if is_valid(row, col, num):
               grid[row][col] = num
               if backtrack(row, col + 1):
                   return True
               grid[row][col] = 0

       # Se nenhum número válido for encontrado, faz backtrack
       return False

   # Valida a grade de entrada
   if not all(len(row) == 9 and all(0 <= cell <= 9 for cell in row) for row in grid):
       return None

   # Inicia o algoritmo de backtracking
   if backtrack(0, 0):
       return grid
   else:
       return None

```

---

## Solicitação de API

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # padrão para os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    system="Sua tarefa é criar funções Python baseadas nas solicitações em linguagem natural fornecidas. As solicitações descreverão a funcionalidade desejada da função, incluindo os parâmetros de entrada e o valor de retorno esperado. Implemente as funções de acordo com as especificações fornecidas, garantindo que elas lidem com casos extremos, realizem validações necessárias e sigam as melhores práticas para programação Python. Por favor, inclua comentários apropriados no código para explicar a lógica e ajudar outros desenvolvedores a entender a implementação.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Quero uma função que possa resolver um quebra-cabeça Sudoku. A função deve receber uma grade Sudoku 9x9 como entrada, onde células vazias são representadas pelo valor 0. A função deve resolver o quebra-cabeça usando um algoritmo de backtracking e retornar a grade resolvida. Se o quebra-cabeça for insolúvel, deve retornar None. A função também deve validar a grade de entrada para garantir que seja um quebra-cabeça Sudoku válido.",
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
  apiKey: "my_api_key", // padrão para process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "Sua tarefa é criar funções Python baseadas nas solicitações em linguagem natural fornecidas. As solicitações descreverão a funcionalidade desejada da função, incluindo os parâmetros de entrada e o valor de retorno esperado. Implemente as funções de acordo com as especificações fornecidas, garantindo que elas lidem com casos extremos, realizem validações necessárias e sigam as melhores práticas para programação Python. Por favor, inclua comentários apropriados no código para explicar a lógica e ajudar outros desenvolvedores a entender a implementação.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Quero uma função que possa resolver um quebra-cabeça Sudoku. A função deve receber uma grade Sudoku 9x9 como entrada, onde células vazias são representadas pelo valor 0. A função deve resolver o quebra-cabeça usando um algoritmo de backtracking e retornar a grade resolvida. Se o quebra-cabeça for insolúvel, deve retornar None. A função também deve validar a grade de entrada para garantir que seja um quebra-cabeça Sudoku válido."
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

# Veja https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# para opções de autenticação

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=1000,
temperature=0,
system="Sua tarefa é criar funções Python baseadas nas solicitações em linguagem natural fornecidas. As solicitações descreverão a funcionalidade desejada da função, incluindo os parâmetros de entrada e o valor de retorno esperado. Implemente as funções de acordo com as especificações fornecidas, garantindo que elas lidem com casos extremos, realizem validações necessárias e sigam as melhores práticas para programação Python. Por favor, inclua comentários apropriados no código para explicar a lógica e ajudar outros desenvolvedores a entender a implementação.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Quero uma função que possa resolver um quebra-cabeça Sudoku. A função deve receber uma grade Sudoku 9x9 como entrada, onde células vazias são representadas pelo valor 0. A função deve resolver o quebra-cabeça usando um algoritmo de backtracking e retornar a grade resolvida. Se o quebra-cabeça for insolúvel, deve retornar None. A função também deve validar a grade de entrada para garantir que seja um quebra-cabeça Sudoku válido."
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

// Veja https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// para opções de autenticação
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "Sua tarefa é criar funções Python baseadas nas solicitações em linguagem natural fornecidas. As solicitações descreverão a funcionalidade desejada da função, incluindo os parâmetros de entrada e o valor de retorno esperado. Implemente as funções de acordo com as especificações fornecidas, garantindo que elas lidem com casos extremos, realizem validações necessárias e sigam as melhores práticas para programação Python. Por favor, inclua comentários apropriados no código para explicar a lógica e ajudar outros desenvolvedores a entender a implementação.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Quero uma função que possa resolver um quebra-cabeça Sudoku. A função deve receber uma grade Sudoku 9x9 como entrada, onde células vazias são representadas pelo valor 0. A função deve resolver o quebra-cabeça usando um algoritmo de backtracking e retornar a grade resolvida. Se o quebra-cabeça for insolúvel, deve retornar None. A função também deve validar a grade de entrada para garantir que seja um quebra-cabeça Sudoku válido."
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
system="Sua tarefa é criar funções Python baseadas nas solicitações em linguagem natural fornecidas. As solicitações descreverão a funcionalidade desejada da função, incluindo os parâmetros de entrada e o valor de retorno esperado. Implemente as funções de acordo com as especificações fornecidas, garantindo que elas lidem com casos extremos, realizem validações necessárias e sigam as melhores práticas para programação Python. Por favor, inclua comentários apropriados no código para explicar a lógica e ajudar outros desenvolvedores a entender a implementação.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Quero uma função que possa resolver um quebra-cabeça Sudoku. A função deve receber uma grade Sudoku 9x9 como entrada, onde células vazias são representadas pelo valor 0. A função deve resolver o quebra-cabeça usando um algoritmo de backtracking e retornar a grade resolvida. Se o quebra-cabeça for insolúvel, deve retornar None. A função também deve validar a grade de entrada para garantir que seja um quebra-cabeça Sudoku válido."
}
]
}
]
)
print(message.content)

````
</Tab>
<Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Lê das variáveis de ambiente `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID`.
// Adicionalmente passa pelo fluxo padrão da `google-auth-library`.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 0,
  system: "Sua tarefa é criar funções Python baseadas nas solicitações em linguagem natural fornecidas. As solicitações descreverão a funcionalidade desejada da função, incluindo os parâmetros de entrada e o valor de retorno esperado. Implemente as funções de acordo com as especificações fornecidas, garantindo que elas lidem com casos extremos, realizem validações necessárias e sigam as melhores práticas para programação Python. Por favor, inclua comentários apropriados no código para explicar a lógica e ajudar outros desenvolvedores a entender a implementação.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Quero uma função que possa resolver um quebra-cabeça Sudoku. A função deve receber uma grade Sudoku 9x9 como entrada, onde células vazias são representadas pelo valor 0. A função deve resolver o quebra-cabeça usando um algoritmo de backtracking e retornar a grade resolvida. Se o quebra-cabeça for insolúvel, deve retornar None. A função também deve validar a grade de entrada para garantir que seja um quebra-cabeça Sudoku válido."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>