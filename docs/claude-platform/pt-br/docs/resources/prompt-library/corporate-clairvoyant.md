# Clarividente corporativo

Extraia insights, identifique riscos e destile informações-chave de relatórios corporativos longos em um único memorando.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|      | Conteúdo |
| ---- | --- |
| Usuário | Sua tarefa é analisar o seguinte relatório: <br /> <br /> <report> [Texto completo do [arquivo SEC 10-K 2023 da Matterport](https://investors.matterport.com/node/9501/html), não colado aqui por brevidade] </report> <br /> <br /> Resuma este relatório anual de forma concisa e clara, e identifique as principais tendências de mercado e conclusões. Apresente suas descobertas como um memorando curto que posso enviar para minha equipe. O objetivo do memorando é garantir que minha equipe se mantenha atualizada sobre como as instituições financeiras estão se saindo e fazer previsões qualitativas e identificar se há riscos operacionais e de receita a serem esperados no próximo trimestre. Certifique-se de incluir todos os detalhes relevantes em seu resumo e análise. |

## Exemplo de Saída

> Para: Equipe de Investimento  
> De: \[Seu Nome\]  
> Data: 4 de março de 2024  
> Assunto: Destaques do 10-K 2022 da Matterport e Perspectivas Financeiras
>
> Olá equipe,
>
> Identifiquei algumas conclusões-chave do relatório anual de 2022 da Matterport:
>
> Desempenho Financeiro:
>
> - A receita aumentou 22% para \$136,1M em 2022, impulsionada pelo crescimento na receita de assinaturas e serviços
> - No entanto, a margem bruta diminuiu de 54% para 38% devido ao aumento dos custos da receita de produtos e desafios da cadeia de suprimentos
> - As perdas operacionais se ampliaram para \$275,5M enquanto a empresa investiu pesadamente em P&D e vendas e marketing para impulsionar o crescimento
> - Terminou 2022 com \$476,9M em caixa e investimentos para financiar as operações
>
> Destaques do Negócio:
>
> - A base de assinantes cresceu 39% para mais de 701.000 no final de 2022. Os espaços sob gestão chegaram a 9,2 milhões
> - Expandiu a plataforma com o lançamento do suporte motorizado Matterport Axis, câmera Pro3 e grandes atualizações no software em nuvem
> - Completou aquisições da Enview (IA para dados espaciais 3D) e VHT Studios (marketing imobiliário) para aprimorar as ofertas
> - Assinou parcerias notáveis com Burns & McDonnell e CGS para entregar as soluções da Matterport aos seus clientes empresariais
>
> Perspectivas:
>
> - A Matterport vê uma oportunidade de mercado massiva na digitalização dos edifícios e espaços do mundo (estima TAM de \$240B+)
> - Focada em impulsionar a adoção com grandes clientes empresariais em verticais-chave como imobiliário, AEC, gestão de instalações e mais
> - Continuará investindo em P&D e expansão internacional para penetrar no mercado global de edifícios amplamente inexplorado
> - O crescimento e margens de curto prazo podem ser impactados por ventos contrários macroeconômicos e questões da cadeia de suprimentos
> - No entanto, a posição de liquidez saudável e base de clientes relativamente aderente fornecem proteção contra quedas
>
> Em resumo, a Matterport entregou forte crescimento de receita em 2022 enquanto fez progresso penetrando um mercado global enorme. Embora a lucratividade permaneça desafiada enquanto a empresa investe pesadamente para crescimento, seu balanço parece sólido. Além de algumas questões contínuas da cadeia de suprimentos, o momentum do negócio subjacente parece intacto baseado nas métricas operacionais-chave. Exceto por uma severa recessão econômica, a Matterport parece bem posicionada para continuar ganhando participação no espaço nascente de digitalização de edifícios.

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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Sua tarefa é analisar o seguinte relatório:  \n<report>  \n[Texto completo do [arquivo SEC 10-K 2023 da Matterport](https://investors.matterport.com/node/9501/html), não colado aqui por brevidade]  \n</report>  \n  \nResuma este relatório anual de forma concisa e clara, e identifique as principais tendências de mercado e conclusões. Apresente suas descobertas como um memorando curto que posso enviar para minha equipe. O objetivo do memorando é garantir que minha equipe se mantenha atualizada sobre como as instituições financeiras estão se saindo e fazer previsões qualitativas e identificar se há riscos operacionais e de receita a serem esperados no próximo trimestre. Certifique-se de incluir todos os detalhes relevantes em seu resumo e análise."
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
          "text": "Sua tarefa é analisar o seguinte relatório:  \n<report>  \n[Texto completo do [arquivo SEC 10-K 2023 da Matterport](https://investors.matterport.com/node/9501/html), não colado aqui por brevidade]  \n</report>  \n  \nResuma este relatório anual de forma concisa e clara, e identifique as principais tendências de mercado e conclusões. Apresente suas descobertas como um memorando curto que posso enviar para minha equipe. O objetivo do memorando é garantir que minha equipe se mantenha atualizada sobre como as instituições financeiras estão se saindo e fazer previsões qualitativas e identificar se há riscos operacionais e de receita a serem esperados no próximo trimestre. Certifique-se de incluir todos os detalhes relevantes em seu resumo e análise."
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
"text": "Sua tarefa é analisar o seguinte relatório: \n<report> \n[Texto completo do [arquivo SEC 10-K 2023 da Matterport](https://investors.matterport.com/node/9501/html), não colado aqui por brevidade] \n</report> \n \nResuma este relatório anual de forma concisa e clara, e identifique as principais tendências de mercado e conclusões. Apresente suas descobertas como um memorando curto que posso enviar para minha equipe. O objetivo do memorando é garantir que minha equipe se mantenha atualizada sobre como as instituições financeiras estão se saindo e fazer previsões qualitativas e identificar se há riscos operacionais e de receita a serem esperados no próximo trimestre. Certifique-se de incluir todos os detalhes relevantes em seu resumo e análise."
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
          "text": "Sua tarefa é analisar o seguinte relatório:  \n<report>  \n[Texto completo do [arquivo SEC 10-K 2023 da Matterport](https://investors.matterport.com/node/9501/html), não colado aqui por brevidade]  \n</report>  \n  \nResuma este relatório anual de forma concisa e clara, e identifique as principais tendências de mercado e conclusões. Apresente suas descobertas como um memorando curto que posso enviar para minha equipe. O objetivo do memorando é garantir que minha equipe se mantenha atualizada sobre como as instituições financeiras estão se saindo e fazer previsões qualitativas e identificar se há riscos operacionais e de receita a serem esperados no próximo trimestre. Certifique-se de incluir todos os detalhes relevantes em seu resumo e análise."
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
                    "text": "Sua tarefa é analisar o seguinte relatório:  \n<report>  \n[Texto completo do [arquivo SEC 10-K 2023 da Matterport](https://investors.matterport.com/node/9501/html), não colado aqui por brevidade]  \n</report>  \n  \nResuma este relatório anual de forma concisa e clara, e identifique as principais tendências de mercado e conclusões. Apresente suas descobertas como um memorando curto que posso enviar para minha equipe. O objetivo do memorando é garantir que minha equipe se mantenha atualizada sobre como as instituições financeiras estão se saindo e fazer previsões qualitativas e identificar se há riscos operacionais e de receita a serem esperados no próximo trimestre. Certifique-se de incluir todos os detalhes relevantes em seu resumo e análise."
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
          "text": "Sua tarefa é analisar o seguinte relatório:  \n<report>  \n[Texto completo do [arquivo SEC 10-K 2023 da Matterport](https://investors.matterport.com/node/9501/html), não colado aqui por brevidade]  \n</report>  \n  \nResuma este relatório anual de forma concisa e clara, e identifique as principais tendências de mercado e conclusões. Apresente suas descobertas como um memorando curto que posso enviar para minha equipe. O objetivo do memorando é garantir que minha equipe se mantenha atualizada sobre como as instituições financeiras estão se saindo e fazer previsões qualitativas e identificar se há riscos operacionais e de receita a serem esperados no próximo trimestre. Certifique-se de incluir todos os detalhes relevantes em seu resumo e análise."
        }
      ]
    }
  ]
});
console.log(msg);

```

  </Tab>
</Tabs>