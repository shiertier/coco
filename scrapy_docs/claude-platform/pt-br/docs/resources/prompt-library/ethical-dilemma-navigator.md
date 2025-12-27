# Navegador de dilemas éticos

Ajude o usuário a pensar através de dilemas éticos complexos e forneça diferentes perspectivas.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Ajude o usuário a navegar por um dilema ético complexo identificando princípios éticos fundamentais, explorando diferentes estruturas éticas, considerando consequências potenciais, reconhecendo complexidade, encorajando reflexão pessoal e oferecendo recursos adicionais. Mantenha um tom objetivo e sem julgamentos e enfatize o pensamento crítico, empatia e tomada de decisão responsável. |
| Usuário   | Sou um jornalista que descobriu evidências de corrupção envolvendo um funcionário governamental de alto escalão. Publicar a história poderia levar à indignação pública e pedidos de renúncia do funcionário, mas também pode desestabilizar o governo e causar incerteza econômica. O funcionário me ofereceu um suborno significativo para manter a história em segredo. Devo aceitar o suborno, publicar a história ou encontrar outra solução? |

## Exemplo de Saída

Este dilema envolve princípios como dizer a verdade, interesse público, integridade pessoal e responsabilidade social. Diferentes estruturas éticas fornecem insights:

- Utilitarismo: Publicar a história poderia levar a um maior bem-estar geral ao expor a corrupção e promover governança responsável, apesar da instabilidade de curto prazo.
- Deontologia: Aceitar um suborno viola deveres éticos de honestidade e imparcialidade. Pode haver uma obrigação moral de relatar a verdade, independentemente das consequências.
- Ética da virtude: Um jornalista virtuoso priorizaria integridade, coragem e justiça, resistindo ao ganho pessoal de um suborno e defendendo o papel da imprensa em uma sociedade democrática.
- Ética do cuidado: Considere o impacto nos relacionamentos e confiança, tanto com o público quanto dentro da comunidade jornalística.

Consequências de curto prazo da publicação incluem indignação pública, instabilidade política e incerteza econômica. Implicações de longo prazo podem envolver governança melhorada, aumento da confiança pública na mídia e integridade pessoal/profissional.

Aceitar o suborno compromete a ética jornalística e contribui para uma cultura de corrupção. Pode levar à culpa pessoal, perda de credibilidade e permitir comportamento antiético adicional.

Considere seus valores, ética profissional e o papel do jornalismo na sociedade. Busque orientação de códigos de conduta éticos, consultores jurídicos e colegas de confiança.

Recursos adicionais incluem estudos de caso de jornalistas enfrentando dilemas similares, diretrizes de ética da mídia e discussões sobre o papel da imprensa em responsabilizar o poder.

Em resumo, este dilema requer pesar o interesse público contra a instabilidade potencial, considerar a integridade profissional e refletir sobre as implicações de longo prazo de sua decisão. Esforce-se para encontrar uma solução que defenda princípios éticos e o papel vital do jornalismo em uma sociedade democrática.

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
  temperature=1,
  system="Ajude o usuário a navegar por um dilema ético complexo identificando princípios éticos fundamentais, explorando diferentes estruturas éticas, considerando consequências potenciais, reconhecendo complexidade, encorajando reflexão pessoal e oferecendo recursos adicionais. Mantenha um tom objetivo e sem julgamentos e enfatize o pensamento crítico, empatia e tomada de decisão responsável.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Sou um jornalista que descobriu evidências de corrupção envolvendo um funcionário governamental de alto escalão. Publicar a história poderia levar à indignação pública e pedidos de renúncia do funcionário, mas também pode desestabilizar o governo e causar incerteza econômica. O funcionário me ofereceu um suborno significativo para manter a história em segredo. Devo aceitar o suborno, publicar a história ou encontrar outra solução?"
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
  temperature: 1,
  system: "Ajude o usuário a navegar por um dilema ético complexo identificando princípios éticos fundamentais, explorando diferentes estruturas éticas, considerando consequências potenciais, reconhecendo complexidade, encorajando reflexão pessoal e oferecendo recursos adicionais. Mantenha um tom objetivo e sem julgamentos e enfatize o pensamento crítico, empatia e tomada de decisão responsável.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Sou um jornalista que descobriu evidências de corrupção envolvendo um funcionário governamental de alto escalão. Publicar a história poderia levar à indignação pública e pedidos de renúncia do funcionário, mas também pode desestabilizar o governo e causar incerteza econômica. O funcionário me ofereceu um suborno significativo para manter a história em segredo. Devo aceitar o suborno, publicar a história ou encontrar outra solução?"
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
    max_tokens=2000,
    temperature=1,
    system="Ajude o usuário a navegar por um dilema ético complexo identificando princípios éticos fundamentais, explorando diferentes estruturas éticas, considerando consequências potenciais, reconhecendo complexidade, encorajando reflexão pessoal e oferecendo recursos adicionais. Mantenha um tom objetivo e sem julgamentos e enfatize o pensamento crítico, empatia e tomada de decisão responsável.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Sou um jornalista que descobriu evidências de corrupção envolvendo um funcionário governamental de alto escalão. Publicar a história poderia levar à indignação pública e pedidos de renúncia do funcionário, mas também pode desestabilizar o governo e causar incerteza econômica. O funcionário me ofereceu um suborno significativo para manter a história em segredo. Devo aceitar o suborno, publicar a história ou encontrar outra solução?"
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
  temperature: 1,
  system: "Ajude o usuário a navegar por um dilema ético complexo identificando princípios éticos fundamentais, explorando diferentes estruturas éticas, considerando consequências potenciais, reconhecendo complexidade, encorajando reflexão pessoal e oferecendo recursos adicionais. Mantenha um tom objetivo e sem julgamentos e enfatize o pensamento crítico, empatia e tomada de decisão responsável.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Sou um jornalista que descobriu evidências de corrupção envolvendo um funcionário governamental de alto escalão. Publicar a história poderia levar à indignação pública e pedidos de renúncia do funcionário, mas também pode desestabilizar o governo e causar incerteza econômica. O funcionário me ofereceu um suborno significativo para manter a história em segredo. Devo aceitar o suborno, publicar a história ou encontrar outra solução?"
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
    temperature=1,
    system="Ajude o usuário a navegar por um dilema ético complexo identificando princípios éticos fundamentais, explorando diferentes estruturas éticas, considerando consequências potenciais, reconhecendo complexidade, encorajando reflexão pessoal e oferecendo recursos adicionais. Mantenha um tom objetivo e sem julgamentos e enfatize o pensamento crítico, empatia e tomada de decisão responsável.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Sou um jornalista que descobriu evidências de corrupção envolvendo um funcionário governamental de alto escalão. Publicar a história poderia levar à indignação pública e pedidos de renúncia do funcionário, mas também pode desestabilizar o governo e causar incerteza econômica. O funcionário me ofereceu um suborno significativo para manter a história em segredo. Devo aceitar o suborno, publicar a história ou encontrar outra solução?"
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
temperature: 1,
system: "Ajude o usuário a navegar por um dilema ético complexo identificando princípios éticos fundamentais, explorando diferentes estruturas éticas, considerando consequências potenciais, reconhecendo complexidade, encorajando reflexão pessoal e oferecendo recursos adicionais. Mantenha um tom objetivo e sem julgamentos e enfatize o pensamento crítico, empatia e tomada de decisão responsável.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Sou um jornalista que descobriu evidências de corrupção envolvendo um funcionário governamental de alto escalão. Publicar a história poderia levar à indignação pública e pedidos de renúncia do funcionário, mas também pode desestabilizar o governo e causar incerteza econômica. O funcionário me ofereceu um suborno significativo para manter a história em segredo. Devo aceitar o suborno, publicar a história ou encontrar outra solução?"
}
]
}
]
});
console.log(msg);
```

</Tab>
</Tabs>