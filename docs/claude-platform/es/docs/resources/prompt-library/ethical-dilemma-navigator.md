# Navegador de dilemas éticos

Ayuda al usuario a reflexionar sobre dilemas éticos complejos y proporciona diferentes perspectivas.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Ayuda al usuario a navegar un dilema ético complejo identificando principios éticos fundamentales, explorando diferentes marcos éticos, considerando consecuencias potenciales, reconociendo la complejidad, fomentando la reflexión personal y ofreciendo recursos adicionales. Mantén un tono objetivo y sin juicios, y enfatiza el pensamiento crítico, la empatía y la toma de decisiones responsable. |
| Usuario   | Soy un periodista que ha descubierto evidencia de corrupción que involucra a un funcionario gubernamental de alto rango. Publicar la historia podría llevar a indignación pública y llamados a la renuncia del funcionario, pero también podría desestabilizar el gobierno y causar incertidumbre económica. El funcionario me ha ofrecido un soborno significativo para mantener la historia en silencio. ¿Debería aceptar el soborno, publicar la historia o encontrar otra solución? |

## Ejemplo de Salida

Este dilema involucra principios como decir la verdad, interés público, integridad personal y responsabilidad social. Diferentes marcos éticos proporcionan perspectivas:

- Utilitarismo: Publicar la historia podría llevar a un mayor bienestar general al exponer la corrupción y promover una gobernanza responsable, a pesar de la inestabilidad a corto plazo.
- Deontología: Aceptar un soborno viola los deberes éticos de honestidad e imparcialidad. Puede haber una obligación moral de reportar la verdad, independientemente de las consecuencias.
- Ética de la virtud: Un periodista virtuoso priorizaría la integridad, el coraje y la justicia, resistiendo el beneficio personal de un soborno y defendiendo el papel de la prensa en una sociedad democrática.
- Ética del cuidado: Considera el impacto en las relaciones y la confianza, tanto con el público como dentro de la comunidad periodística.

Las consecuencias a corto plazo de publicar incluyen indignación pública, inestabilidad política e incertidumbre económica. Las implicaciones a largo plazo pueden involucrar una mejor gobernanza, mayor confianza pública en los medios e integridad personal/profesional.

Aceptar el soborno compromete la ética periodística y contribuye a una cultura de corrupción. Puede llevar a culpa personal, pérdida de credibilidad y permitir más comportamiento no ético.

Considera tus valores, ética profesional y el papel del periodismo en la sociedad. Busca orientación de códigos de conducta ética, asesores legales y colegas de confianza.

Los recursos adicionales incluyen estudios de casos de periodistas enfrentando dilemas similares, pautas de ética mediática y discusiones sobre el papel de la prensa en hacer responsable al poder.

En resumen, este dilema requiere sopesar el interés público contra la inestabilidad potencial, considerar la integridad profesional y reflexionar sobre las implicaciones a largo plazo de tu decisión. Esfuérzate por encontrar una solución que defienda los principios éticos y el papel vital del periodismo en una sociedad democrática.

---

## Solicitud de API

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
  system="Ayuda al usuario a navegar un dilema ético complejo identificando principios éticos fundamentales, explorando diferentes marcos éticos, considerando consecuencias potenciales, reconociendo la complejidad, fomentando la reflexión personal y ofreciendo recursos adicionales. Mantén un tono objetivo y sin juicios, y enfatiza el pensamiento crítico, la empatía y la toma de decisiones responsable.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Soy un periodista que ha descubierto evidencia de corrupción que involucra a un funcionario gubernamental de alto rango. Publicar la historia podría llevar a indignación pública y llamados a la renuncia del funcionario, pero también podría desestabilizar el gobierno y causar incertidumbre económica. El funcionario me ha ofrecido un soborno significativo para mantener la historia en silencio. ¿Debería aceptar el soborno, publicar la historia o encontrar otra solución?"
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
  system: "Ayuda al usuario a navegar un dilema ético complejo identificando principios éticos fundamentales, explorando diferentes marcos éticos, considerando consecuencias potenciales, reconociendo la complejidad, fomentando la reflexión personal y ofreciendo recursos adicionales. Mantén un tono objetivo y sin juicios, y enfatiza el pensamiento crítico, la empatía y la toma de decisiones responsable.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Soy un periodista que ha descubierto evidencia de corrupción que involucra a un funcionario gubernamental de alto rango. Publicar la historia podría llevar a indignación pública y llamados a la renuncia del funcionario, pero también podría desestabilizar el gobierno y causar incertidumbre económica. El funcionario me ha ofrecido un soborno significativo para mantener la historia en silencio. ¿Debería aceptar el soborno, publicar la historia o encontrar otra solución?"
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
    system="Ayuda al usuario a navegar un dilema ético complejo identificando principios éticos fundamentales, explorando diferentes marcos éticos, considerando consecuencias potenciales, reconociendo la complejidad, fomentando la reflexión personal y ofreciendo recursos adicionales. Mantén un tono objetivo y sin juicios, y enfatiza el pensamiento crítico, la empatía y la toma de decisiones responsable.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Soy un periodista que ha descubierto evidencia de corrupción que involucra a un funcionario gubernamental de alto rango. Publicar la historia podría llevar a indignación pública y llamados a la renuncia del funcionario, pero también podría desestabilizar el gobierno y causar incertidumbre económica. El funcionario me ha ofrecido un soborno significativo para mantener la historia en silencio. ¿Debería aceptar el soborno, publicar la historia o encontrar otra solución?"
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
  system: "Ayuda al usuario a navegar un dilema ético complejo identificando principios éticos fundamentales, explorando diferentes marcos éticos, considerando consecuencias potenciales, reconociendo la complejidad, fomentando la reflexión personal y ofreciendo recursos adicionales. Mantén un tono objetivo y sin juicios, y enfatiza el pensamiento crítico, la empatía y la toma de decisiones responsable.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Soy un periodista que ha descubierto evidencia de corrupción que involucra a un funcionario gubernamental de alto rango. Publicar la historia podría llevar a indignación pública y llamados a la renuncia del funcionario, pero también podría desestabilizar el gobierno y causar incertidumbre económica. El funcionario me ha ofrecido un soborno significativo para mantener la historia en silencio. ¿Debería aceptar el soborno, publicar la historia o encontrar otra solución?"
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
    system="Ayuda al usuario a navegar un dilema ético complejo identificando principios éticos fundamentales, explorando diferentes marcos éticos, considerando consecuencias potenciales, reconociendo la complejidad, fomentando la reflexión personal y ofreciendo recursos adicionales. Mantén un tono objetivo y sin juicios, y enfatiza el pensamiento crítico, la empatía y la toma de decisiones responsable.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Soy un periodista que ha descubierto evidencia de corrupción que involucra a un funcionario gubernamental de alto rango. Publicar la historia podría llevar a indignación pública y llamados a la renuncia del funcionario, pero también podría desestabilizar el gobierno y causar incertidumbre económica. El funcionario me ha ofrecido un soborno significativo para mantener la historia en silencio. ¿Debería aceptar el soborno, publicar la historia o encontrar otra solución?"
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
system: "Ayuda al usuario a navegar un dilema ético complejo identificando principios éticos fundamentales, explorando diferentes marcos éticos, considerando consecuencias potenciales, reconociendo la complejidad, fomentando la reflexión personal y ofreciendo recursos adicionales. Mantén un tono objetivo y sin juicios, y enfatiza el pensamiento crítico, la empatía y la toma de decisiones responsable.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Soy un periodista que ha descubierto evidencia de corrupción que involucra a un funcionario gubernamental de alto rango. Publicar la historia podría llevar a indignación pública y llamados a la renuncia del funcionario, pero también podría desestabilizar el gobierno y causar incertidumbre económica. El funcionario me ha ofrecido un soborno significativo para mantener la historia en silencio. ¿Debería aceptar el soborno, publicar la historia o encontrar otra solución?"
}
]
}
]
});
console.log(msg);
```

</Tab>
</Tabs>