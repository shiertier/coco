# Criador culinário

Sugira ideias de receitas baseadas nos ingredientes disponíveis do usuário e preferências dietéticas.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Sua tarefa é gerar ideias de receitas personalizadas baseadas na entrada do usuário de ingredientes disponíveis e preferências dietéticas. Use essas informações para sugerir uma variedade de receitas criativas e deliciosas que podem ser feitas usando os ingredientes fornecidos enquanto acomoda as necessidades dietéticas do usuário, se alguma for mencionada. Para cada receita, forneça uma breve descrição, uma lista de ingredientes necessários e um conjunto simples de instruções. Certifique-se de que as receitas sejam fáceis de seguir, nutritivas e possam ser preparadas com ingredientes ou equipamentos adicionais mínimos. |
| Usuário   | Ingredientes disponíveis: peito de frango, brócolis, cenouras, cebola, alho, azeite de oliva, arroz, molho de soja sem glúten, mel <br/> Preferências dietéticas: sem glúten |

## Exemplo de Saída

Receitas sugeridas:

Refogado de Frango com Mel e Alho

Descrição: Um refogado rápido e saboroso com peito de frango macio, vegetais crocantes e um molho doce e salgado de mel e alho.  
 Ingredientes:

- 2 peitos de frango, cortados em pedaços pequenos
- 2 xícaras de floretes de brócolis
- 1 xícara de cenouras fatiadas
- 1 cebola, cortada em cubos
- 3 dentes de alho, picados
- 2 colheres de sopa de azeite de oliva
- 1/4 xícara de molho de soja (sem glúten)
- 2 colheres de sopa de mel

Instruções:

1.  Em uma frigideira grande ou wok, aqueça o azeite de oliva em fogo médio-alto.
2.  Adicione a cebola cortada em cubos e o alho picado, e refogue até ficar aromático, cerca de 2 minutos.
3.  Adicione os pedaços de frango e cozinhe até não estar mais rosado, cerca de 5-7 minutos.
4.  Adicione o brócolis e as cenouras, e refogue até os vegetais ficarem macios-crocantes, cerca de 5 minutos.
5.  Em uma tigela pequena, misture o molho de soja e o mel.
6.  Despeje o molho sobre o frango e os vegetais, e mexa para cobrir uniformemente.
7.  Cozinhe por mais 2-3 minutos, ou até o molho engrossar ligeiramente.
8.  Sirva quente com arroz cozido no vapor.

Tigelas de Arroz com Frango e Vegetais

Descrição: Uma tigela de arroz substanciosa e nutritiva coberta com frango temperado, vegetais assados e um fio de molho de soja.

Ingredientes:

- 2 peitos de frango, cortados em pedaços pequenos
- 2 xícaras de floretes de brócolis
- 1 xícara de cenouras fatiadas
- 1 cebola, cortada em cubos
- 2 dentes de alho, picados
- 2 colheres de sopa de azeite de oliva
- 1/4 xícara de molho de soja (sem glúten)
- 2 xícaras de arroz cozido

Instruções:

1.  Pré-aqueça o forno a 400°F (200°C).
2.  Em uma tigela grande, combine os pedaços de frango, brócolis, cenouras, cebola e alho.
3.  Regue com azeite de oliva e metade do molho de soja, e misture para cobrir uniformemente.
4.  Espalhe o frango e os vegetais em uma assadeira e asse no forno pré-aquecido por 20-25 minutos, ou até o frango estar completamente cozido e os vegetais estarem macios.
5.  Divida o arroz cozido entre as tigelas de servir.
6.  Cubra cada tigela com o frango e vegetais assados.
7.  Regue o molho de soja restante sobre cada tigela.
8.  Sirva quente e aproveite!

---

## Solicitação de API

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
  system="Sua tarefa é gerar ideias de receitas personalizadas baseadas na entrada do usuário de ingredientes disponíveis e preferências dietéticas. Use essas informações para sugerir uma variedade de receitas criativas e deliciosas que podem ser feitas usando os ingredientes fornecidos enquanto acomoda as necessidades dietéticas do usuário, se alguma for mencionada. Para cada receita, forneça uma breve descrição, uma lista de ingredientes necessários e um conjunto simples de instruções. Certifique-se de que as receitas sejam fáceis de seguir, nutritivas e possam ser preparadas com ingredientes ou equipamentos adicionais mínimos.",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Ingredientes disponíveis: peito de frango, brócolis, cenouras, cebola, alho, azeite de oliva, arroz, molho de soja sem glúten, mel \nPreferências dietéticas: sem glúten"
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
  system: "Sua tarefa é gerar ideias de receitas personalizadas baseadas na entrada do usuário de ingredientes disponíveis e preferências dietéticas. Use essas informações para sugerir uma variedade de receitas criativas e deliciosas que podem ser feitas usando os ingredientes fornecidos enquanto acomoda as necessidades dietéticas do usuário, se alguma for mencionada. Para cada receita, forneça uma breve descrição, uma lista de ingredientes necessários e um conjunto simples de instruções. Certifique-se de que as receitas sejam fáceis de seguir, nutritivas e possam ser preparadas com ingredientes ou equipamentos adicionais mínimos.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ingredientes disponíveis: peito de frango, brócolis, cenouras, cebola, alho, azeite de oliva, arroz, molho de soja sem glúten, mel  \nPreferências dietéticas: sem glúten"
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
    system="Sua tarefa é gerar ideias de receitas personalizadas baseadas na entrada do usuário de ingredientes disponíveis e preferências dietéticas. Use essas informações para sugerir uma variedade de receitas criativas e deliciosas que podem ser feitas usando os ingredientes fornecidos enquanto acomoda as necessidades dietéticas do usuário, se alguma for mencionada. Para cada receita, forneça uma breve descrição, uma lista de ingredientes necessários e um conjunto simples de instruções. Certifique-se de que as receitas sejam fáceis de seguir, nutritivas e possam ser preparadas com ingredientes ou equipamentos adicionais mínimos.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ingredientes disponíveis: peito de frango, brócolis, cenouras, cebola, alho, azeite de oliva, arroz, molho de soja sem glúten, mel  \nPreferências dietéticas: sem glúten"
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
  system: "Sua tarefa é gerar ideias de receitas personalizadas baseadas na entrada do usuário de ingredientes disponíveis e preferências dietéticas. Use essas informações para sugerir uma variedade de receitas criativas e deliciosas que podem ser feitas usando os ingredientes fornecidos enquanto acomoda as necessidades dietéticas do usuário, se alguma for mencionada. Para cada receita, forneça uma breve descrição, uma lista de ingredientes necessários e um conjunto simples de instruções. Certifique-se de que as receitas sejam fáceis de seguir, nutritivas e possam ser preparadas com ingredientes ou equipamentos adicionais mínimos.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ingredientes disponíveis: peito de frango, brócolis, cenouras, cebola, alho, azeite de oliva, arroz, molho de soja sem glúten, mel  \nPreferências dietéticas: sem glúten"
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
    system="Sua tarefa é gerar ideias de receitas personalizadas baseadas na entrada do usuário de ingredientes disponíveis e preferências dietéticas. Use essas informações para sugerir uma variedade de receitas criativas e deliciosas que podem ser feitas usando os ingredientes fornecidos enquanto acomoda as necessidades dietéticas do usuário, se alguma for mencionada. Para cada receita, forneça uma breve descrição, uma lista de ingredientes necessários e um conjunto simples de instruções. Certifique-se de que as receitas sejam fáceis de seguir, nutritivas e possam ser preparadas com ingredientes ou equipamentos adicionais mínimos.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ingredientes disponíveis: peito de frango, brócolis, cenouras, cebola, alho, azeite de oliva, arroz, molho de soja sem glúten, mel  \nPreferências dietéticas: sem glúten"
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
  system: "Sua tarefa é gerar ideias de receitas personalizadas baseadas na entrada do usuário de ingredientes disponíveis e preferências dietéticas. Use essas informações para sugerir uma variedade de receitas criativas e deliciosas que podem ser feitas usando os ingredientes fornecidos enquanto acomoda as necessidades dietéticas do usuário, se alguma for mencionada. Para cada receita, forneça uma breve descrição, uma lista de ingredientes necessários e um conjunto simples de instruções. Certifique-se de que as receitas sejam fáceis de seguir, nutritivas e possam ser preparadas com ingredientes ou equipamentos adicionais mínimos.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ingredientes disponíveis: peito de frango, brócolis, cenouras, cebola, alho, azeite de oliva, arroz, molho de soja sem glúten, mel  \nPreferências dietéticas: sem glúten"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>