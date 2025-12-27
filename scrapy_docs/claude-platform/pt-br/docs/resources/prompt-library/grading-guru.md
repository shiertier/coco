# Guru de avaliação

Compare e avalie a qualidade de textos escritos com base em critérios e padrões definidos pelo usuário.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

| Papel | Conteúdo |
| ---- | --- |
| Usuário | Avalie os dois textos a seguir com base nos critérios dados: <br/> <br/> Texto 1: <br/> O sol estava se pondo atrás das montanhas, lançando um brilho dourado e quente através do vale. Os pássaros cantavam suas canções vespertinas enquanto a brisa suave sussurrava através das folhas. Era um momento pacífico e sereno, que enchia o coração de alegria e contentamento. <br/><br/> Texto 2: <br/> O pôr do sol estava bonito. Os pássaros estavam cantando e o vento estava soprando. Isso me fez feliz. <br/><br/> Critérios: <br/> 1. Linguagem descritiva e imagens <br/> 2. Estrutura e variedade das frases <br/> 3. Impacto emocional e engajamento <br/> 4. Gramática e pontuação |

## Exemplo de Saída

Com base nos critérios fornecidos, aqui está minha avaliação dos dois textos:

Texto 1:

1.  Linguagem descritiva e imagens: O texto usa linguagem vívida e evocativa para pintar um quadro do cenário. Frases como "brilho dourado e quente", "pássaros cantavam suas canções vespertinas" e "brisa suave sussurrava através das folhas" criam uma forte imagem visual e auditiva, imergindo o leitor na cena.
2.  Estrutura e variedade das frases: O texto demonstra uma boa mistura de estruturas de frases, com frases simples e compostas. Essa variedade ajuda a manter um fluxo suave e mantém o leitor engajado.
3.  Impacto emocional e engajamento: O texto transmite efetivamente uma sensação de paz, alegria e contentamento, convidando o leitor a compartilhar da experiência emocional do momento. A descrição do cenário e o uso de linguagem positiva criam uma forte conexão emocional.
4.  Gramática e pontuação: O texto parece estar gramaticalmente correto e bem pontuado, sem erros ou problemas óbvios.

Texto 2:

1.  Linguagem descritiva e imagens: O texto carece de linguagem descritiva detalhada e imagens. Embora mencione o pôr do sol, pássaros e vento, não fornece detalhes vívidos ou evocativos para ajudar o leitor a visualizar a cena.
2.  Estrutura e variedade das frases: O texto consiste em frases curtas e simples com pouca variação na estrutura. Essa falta de variedade pode fazer com que a escrita pareça monótona e menos envolvente.
3.  Impacto emocional e engajamento: O texto transmite uma emoção positiva ao afirmar "Isso me fez feliz", mas não fornece uma forte conexão emocional ou engajamento devido à falta de linguagem descritiva e imagens.
4.  Gramática e pontuação: O texto parece estar gramaticalmente correto e bem pontuado, sem erros ou problemas óbvios.

Análise Comparativa:  
 Com base na avaliação, o Texto 1 tem desempenho significativamente melhor que o Texto 2 em termos de linguagem descritiva, imagens, estrutura das frases, variedade e impacto emocional. O Texto 1 cria uma experiência vívida e imersiva para o leitor e demonstra uma qualidade superior de escrita com base nos critérios dados, com sua linguagem descritiva forte, estruturas de frases variadas e engajamento emocional efetivo. O Texto 2, embora gramaticalmente correto, carece da riqueza descritiva e do impacto emocional presentes no Texto 1.

---

## Solicitação de API

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
                    "text": "Avalie os dois textos a seguir com base nos critérios dados: \n \nTexto 1: \nO sol estava se pondo atrás das montanhas, lançando um brilho dourado e quente através do vale. Os pássaros cantavam suas canções vespertinas enquanto a brisa suave sussurrava através das folhas. Era um momento pacífico e sereno, que enchia o coração de alegria e contentamento. \n \nTexto 2: \nO pôr do sol estava bonito. Os pássaros estavam cantando e o vento estava soprando. Isso me fez feliz. \n \nCritérios: \n1. Linguagem descritiva e imagens \n2. Estrutura e variedade das frases \n3. Impacto emocional e engajamento \n4. Gramática e pontuação",
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
          "text": "Avalie os dois textos a seguir com base nos critérios dados:  \n  \nTexto 1:  \nO sol estava se pondo atrás das montanhas, lançando um brilho dourado e quente através do vale. Os pássaros cantavam suas canções vespertinas enquanto a brisa suave sussurrava através das folhas. Era um momento pacífico e sereno, que enchia o coração de alegria e contentamento.  \n  \nTexto 2:  \nO pôr do sol estava bonito. Os pássaros estavam cantando e o vento estava soprando. Isso me fez feliz.  \n  \nCritérios:  \n1. Linguagem descritiva e imagens  \n2. Estrutura e variedade das frases  \n3. Impacto emocional e engajamento  \n4. Gramática e pontuação"
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
                    "text": "Avalie os dois textos a seguir com base nos critérios dados:  \n  \nTexto 1:  \nO sol estava se pondo atrás das montanhas, lançando um brilho dourado e quente através do vale. Os pássaros cantavam suas canções vespertinas enquanto a brisa suave sussurrava através das folhas. Era um momento pacífico e sereno, que enchia o coração de alegria e contentamento.  \n  \nTexto 2:  \nO pôr do sol estava bonito. Os pássaros estavam cantando e o vento estava soprando. Isso me fez feliz.  \n  \nCritérios:  \n1. Linguagem descritiva e imagens  \n2. Estrutura e variedade das frases  \n3. Impacto emocional e engajamento  \n4. Gramática e pontuação"
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
          "text": "Avalie os dois textos a seguir com base nos critérios dados:  \n  \nTexto 1:  \nO sol estava se pondo atrás das montanhas, lançando um brilho dourado e quente através do vale. Os pássaros cantavam suas canções vespertinas enquanto a brisa suave sussurrava através das folhas. Era um momento pacífico e sereno, que enchia o coração de alegria e contentamento.  \n  \nTexto 2:  \nO pôr do sol estava bonito. Os pássaros estavam cantando e o vento estava soprando. Isso me fez feliz.  \n  \nCritérios:  \n1. Linguagem descritiva e imagens  \n2. Estrutura e variedade das frases  \n3. Impacto emocional e engajamento  \n4. Gramática e pontuação"
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
                    "text": "Avalie os dois textos a seguir com base nos critérios dados:  \n  \nTexto 1:  \nO sol estava se pondo atrás das montanhas, lançando um brilho dourado e quente através do vale. Os pássaros cantavam suas canções vespertinas enquanto a brisa suave sussurrava através das folhas. Era um momento pacífico e sereno, que enchia o coração de alegria e contentamento.  \n  \nTexto 2:  \nO pôr do sol estava bonito. Os pássaros estavam cantando e o vento estava soprando. Isso me fez feliz.  \n  \nCritérios:  \n1. Linguagem descritiva e imagens  \n2. Estrutura e variedade das frases  \n3. Impacto emocional e engajamento  \n4. Gramática e pontuação"
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
          "text": "Avalie os dois textos a seguir com base nos critérios dados:  \n  \nTexto 1:  \nO sol estava se pondo atrás das montanhas, lançando um brilho dourado e quente através do vale. Os pássaros cantavam suas canções vespertinas enquanto a brisa suave sussurrava através das folhas. Era um momento pacífico e sereno, que enchia o coração de alegria e contentamento.  \n  \nTexto 2:  \nO pôr do sol estava bonito. Os pássaros estavam cantando e o vento estava soprando. Isso me fez feliz.  \n  \nCritérios:  \n1. Linguagem descritiva e imagens  \n2. Estrutura e variedade das frases  \n3. Impacto emocional e engajamento  \n4. Gramática e pontuação"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>