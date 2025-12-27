# Cite suas fontes

Obtenha respostas para perguntas sobre o conteúdo de um documento com citações relevantes que apoiam a resposta.

---

<Tip>Recomendamos usar o [recurso de citação](/docs/pt-BR/build-with-claude/citations) integrado na API em vez de uma abordagem baseada em prompt. Usar o recurso de citação da API melhorará a qualidade das citações, garantirá que todas as citações retornadas sejam válidas e potencialmente reduzirá o número de tokens de saída necessários.</Tip>

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Você é um assistente de pesquisa especialista. Aqui está um documento sobre o qual você responderá perguntas: <br/> <doc> [Texto completo do [arquivo SEC 10-K 2023 da Matterport](https://investors.matterport.com/node/9501/html), não colado aqui por brevidade] </doc> <br/><br/> Primeiro, encontre as citações do documento que são mais relevantes para responder à pergunta e, em seguida, imprima-as em ordem numerada. As citações devem ser relativamente curtas. <br/><br/> Se não houver citações relevantes, escreva "Nenhuma citação relevante" em vez disso. <br/><br/> Em seguida, responda à pergunta, começando com "Resposta:". Não inclua ou referencie conteúdo citado textualmente na resposta. Não diga "De acordo com a Citação [1]" ao responder. Em vez disso, faça referências às citações relevantes para cada seção da resposta apenas adicionando seus números entre colchetes no final das frases relevantes. <br/><br/> Assim, o formato de sua resposta geral deve parecer com o que está mostrado entre as tags <example></example>. Certifique-se de seguir a formatação e o espaçamento exatamente. <example> <br/> Citações: <br/> [1] "A Empresa X relatou receita de \$12 milhões em 2021." <br/> [2] "Quase 90% da receita veio de vendas de widgets, com vendas de gadgets representando os 10% restantes." <br/><br/> Resposta: <br/> A Empresa X ganhou \$12 milhões. [1] Quase 90% disso foi de vendas de widgets. [2] <br/> </example> <br/><br/> Se a pergunta não puder ser respondida pelo documento, diga isso. |
| Usuário   | A Matterport está indo bem? |

## Exemplo de saída

Citações:  
 \[1\] "Experimentamos recentemente um crescimento rápido. Nossos assinantes cresceram aproximadamente 49 vezes de 31 de dezembro de 2018 para 31 de dezembro de 2022. Nossa receita aumentou aproximadamente 22% para \$136,1 milhões para o ano encerrado em 31 de dezembro de 2022, de aproximadamente \$111,2 milhões para o ano encerrado em 31 de dezembro de 2021."  
 \[2\] "Nosso lucro bruto diminuiu em \$8,1 milhões ou 14%, para \$51,8 milhões para o ano encerrado em 31 de dezembro de 2022, de \$60,0 milhões para o ano encerrado em 31 de dezembro de 2021, principalmente atribuível a certos custos disruptivos e incrementais devido às restrições da cadeia de suprimentos global no ano fiscal de 2022."  
 \[3\] "Nossa taxa de expansão líquida em dólares de 103% para os três meses encerrados em 31 de dezembro de 2022 demonstra a aderência e o potencial de crescimento de nossa plataforma."

Resposta:  
 A Matterport experimentou um crescimento rápido de assinantes e receita nos últimos anos. \[1\] No entanto, seu lucro bruto declinou em 2022 devido a problemas na cadeia de suprimentos. \[2\] Sua taxa de expansão líquida em dólares indica forte retenção de assinantes e potencial para crescimento contínuo. \[3\] No geral, apesar da queda no lucro bruto, o crescimento de assinantes e receita da Matterport, juntamente com a aderência de sua plataforma, sugere que a empresa está indo relativamente bem.

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
    max_tokens=2000,
    temperature=0,
    system='Você é um assistente de pesquisa especialista. Aqui está um documento sobre o qual você responderá perguntas: \n<doc> \n[Texto completo do [arquivo SEC 10-K 2023 da Matterport](https://investors.matterport.com/node/9501/html), não colado aqui por brevidade] \n</doc> \n \nPrimeiro, encontre as citações do documento que são mais relevantes para responder à pergunta e, em seguida, imprima-as em ordem numerada. As citações devem ser relativamente curtas. \n \nSe não houver citações relevantes, escreva "Nenhuma citação relevante" em vez disso. \n \nEm seguida, responda à pergunta, começando com "Resposta:". Não inclua ou referencie conteúdo citado textualmente na resposta. Não diga "De acordo com a Citação [1]" ao responder. Em vez disso, faça referências às citações relevantes para cada seção da resposta apenas adicionando seus números entre colchetes no final das frases relevantes. \n \nAssim, o formato de sua resposta geral deve parecer com o que está mostrado entre as tags <example></example>. Certifique-se de seguir a formatação e o espaçamento exatamente. \n<example> \nCitações: \n[1] "A Empresa X relatou receita de \$12 milhões em 2021." \n[2] "Quase 90% da receita veio de vendas de widgets, com vendas de gadgets representando os 10% restantes." \n \nResposta: \nA Empresa X ganhou \$12 milhões. [1] Quase 90% disso foi de vendas de widgets. [2] \n</example> \n \nSe a pergunta não puder ser respondida pelo documento, diga isso.',
    messages=[
        {
            "role": "user",
            "content": [{"type": "text", "text": "A Matterport está indo bem?"}],
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
  system: "Você é um assistente de pesquisa especialista. Aqui está um documento sobre o qual você responderá perguntas:  \n<doc>  \n[Texto completo do [arquivo SEC 10-K 2023 da Matterport](https://investors.matterport.com/node/9501/html), não colado aqui por brevidade]  \n</doc>  \n  \nPrimeiro, encontre as citações do documento que são mais relevantes para responder à pergunta e, em seguida, imprima-as em ordem numerada. As citações devem ser relativamente curtas.  \n  \nSe não houver citações relevantes, escreva \"Nenhuma citação relevante\" em vez disso.  \n  \nEm seguida, responda à pergunta, começando com \"Resposta:\". Não inclua ou referencie conteúdo citado textualmente na resposta. Não diga \"De acordo com a Citação [1]\" ao responder. Em vez disso, faça referências às citações relevantes para cada seção da resposta apenas adicionando seus números entre colchetes no final das frases relevantes.  \n  \nAssim, o formato de sua resposta geral deve parecer com o que está mostrado entre as tags <example></example>. Certifique-se de seguir a formatação e o espaçamento exatamente.  \n<example>  \nCitações:  \n[1] \"A Empresa X relatou receita de \$12 milhões em 2021.\"  \n[2] \"Quase 90% da receita veio de vendas de widgets, com vendas de gadgets representando os 10% restantes.\"  \n  \nResposta:  \nA Empresa X ganhou \$12 milhões. [1] Quase 90% disso foi de vendas de widgets. [2]  \n</example>  \n  \nSe a pergunta não puder ser respondida pelo documento, diga isso.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "A Matterport está indo bem?"
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
    system="Você é um assistente de pesquisa especialista. Aqui está um documento sobre o qual você responderá perguntas:  \n<doc>  \n[Texto completo do [arquivo SEC 10-K 2023 da Matterport](https://investors.matterport.com/node/9501/html), não colado aqui por brevidade]  \n</doc>  \n  \nPrimeiro, encontre as citações do documento que são mais relevantes para responder à pergunta e, em seguida, imprima-as em ordem numerada. As citações devem ser relativamente curtas.  \n  \nSe não houver citações relevantes, escreva \"Nenhuma citação relevante\" em vez disso.  \n  \nEm seguida, responda à pergunta, começando com \"Resposta:\". Não inclua ou referencie conteúdo citado textualmente na resposta. Não diga \"De acordo com a Citação [1]\" ao responder. Em vez disso, faça referências às citações relevantes para cada seção da resposta apenas adicionando seus números entre colchetes no final das frases relevantes.  \n  \nAssim, o formato de sua resposta geral deve parecer com o que está mostrado entre as tags <example></example>. Certifique-se de seguir a formatação e o espaçamento exatamente.  \n<example>  \nCitações:  \n[1] \"A Empresa X relatou receita de \$12 milhões em 2021.\"  \n[2] \"Quase 90% da receita veio de vendas de widgets, com vendas de gadgets representando os 10% restantes.\"  \n  \nResposta:  \nA Empresa X ganhou \$12 milhões. [1] Quase 90% disso foi de vendas de widgets. [2]  \n</example>  \n  \nSe a pergunta não puder ser respondida pelo documento, diga isso.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "A Matterport está indo bem?"
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
  system: "Você é um assistente de pesquisa especialista. Aqui está um documento sobre o qual você responderá perguntas:  \n<doc>  \n[Texto completo do [arquivo SEC 10-K 2023 da Matterport](https://investors.matterport.com/node/9501/html), não colado aqui por brevidade]  \n</doc>  \n  \nPrimeiro, encontre as citações do documento que são mais relevantes para responder à pergunta e, em seguida, imprima-as em ordem numerada. As citações devem ser relativamente curtas.  \n  \nSe não houver citações relevantes, escreva \"Nenhuma citação relevante\" em vez disso.  \n  \nEm seguida, responda à pergunta, começando com \"Resposta:\". Não inclua ou referencie conteúdo citado textualmente na resposta. Não diga \"De acordo com a Citação [1]\" ao responder. Em vez disso, faça referências às citações relevantes para cada seção da resposta apenas adicionando seus números entre colchetes no final das frases relevantes.  \n  \nAssim, o formato de sua resposta geral deve parecer com o que está mostrado entre as tags <example></example>. Certifique-se de seguir a formatação e o espaçamento exatamente.  \n<example>  \nCitações:  \n[1] \"A Empresa X relatou receita de \$12 milhões em 2021.\"  \n[2] \"Quase 90% da receita veio de vendas de widgets, com vendas de gadgets representando os 10% restantes.\"  \n  \nResposta:  \nA Empresa X ganhou \$12 milhões. [1] Quase 90% disso foi de vendas de widgets. [2]  \n</example>  \n  \nSe a pergunta não puder ser respondida pelo documento, diga isso.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "A Matterport está indo bem?"
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
    system="Você é um assistente de pesquisa especialista. Aqui está um documento sobre o qual você responderá perguntas:  \n<doc>  \n[Texto completo do [arquivo SEC 10-K 2023 da Matterport](https://investors.matterport.com/node/9501/html), não colado aqui por brevidade]  \n</doc>  \n  \nPrimeiro, encontre as citações do documento que são mais relevantes para responder à pergunta e, em seguida, imprima-as em ordem numerada. As citações devem ser relativamente curtas.  \n  \nSe não houver citações relevantes, escreva \"Nenhuma citação relevante\" em vez disso.  \n  \nEm seguida, responda à pergunta, começando com \"Resposta:\". Não inclua ou referencie conteúdo citado textualmente na resposta. Não diga \"De acordo com a Citação [1]\" ao responder. Em vez disso, faça referências às citações relevantes para cada seção da resposta apenas adicionando seus números entre colchetes no final das frases relevantes.  \n  \nAssim, o formato de sua resposta geral deve parecer com o que está mostrado entre as tags <example></example>. Certifique-se de seguir a formatação e o espaçamento exatamente.  \n<example>  \nCitações:  \n[1] \"A Empresa X relatou receita de \$12 milhões em 2021.\"  \n[2] \"Quase 90% da receita veio de vendas de widgets, com vendas de gadgets representando os 10% restantes.\"  \n  \nResposta:  \nA Empresa X ganhou \$12 milhões. [1] Quase 90% disso foi de vendas de widgets. [2]  \n</example>  \n  \nSe a pergunta não puder ser respondida pelo documento, diga isso.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "A Matterport está indo bem?"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>

<Tab title=" Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0,
  system: "Você é um assistente de pesquisa especialista. Aqui está um documento sobre o qual você responderá perguntas:  \n<doc>  \n[Texto completo do [arquivo SEC 10-K 2023 da Matterport](https://investors.matterport.com/node/9501/html), não colado aqui por brevidade]  \n</doc>  \n  \nPrimeiro, encontre as citações do documento que são mais relevantes para responder à pergunta e, em seguida, imprima-as em ordem numerada. As citações devem ser relativamente curtas.  \n  \nSe não houver citações relevantes, escreva \"Nenhuma citação relevante\" em vez disso.  \n  \nEm seguida, responda à pergunta, começando com \"Resposta:\". Não inclua ou referencie conteúdo citado textualmente na resposta. Não diga \"De acordo com a Citação [1]\" ao responder. Em vez disso, faça referências às citações relevantes para cada seção da resposta apenas adicionando seus números entre colchetes no final das frases relevantes.  \n  \nAssim, o formato de sua resposta geral deve parecer com o que está mostrado entre as tags <example></example>. Certifique-se de seguir a formatação e o espaçamento exatamente.  \n<example>  \nCitações:  \n[1] \"A Empresa X relatou receita de \$12 milhões em 2021.\"  \n[2] \"Quase 90% da receita veio de vendas de widgets, com vendas de gadgets representando os 10% restantes.\"  \n  \nResposta:  \nA Empresa X ganhou \$12 milhões. [1] Quase 90% disso foi de vendas de widgets. [2]  \n</example>  \n  \nSe a pergunta não puder ser respondida pelo documento, diga isso.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "A Matterport está indo bem?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>