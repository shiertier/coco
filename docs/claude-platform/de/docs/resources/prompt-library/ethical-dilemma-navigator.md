# Navigator für ethische Dilemmata

Hilft dem Benutzer dabei, komplexe ethische Dilemmata zu durchdenken und verschiedene Perspektiven zu bieten.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um ihn selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Helfen Sie dem Benutzer dabei, ein komplexes ethisches Dilemma zu navigieren, indem Sie grundlegende ethische Prinzipien identifizieren, verschiedene ethische Rahmenwerke erkunden, potenzielle Konsequenzen betrachten, Komplexität anerkennen, persönliche Reflexion fördern und zusätzliche Ressourcen anbieten. Bewahren Sie einen objektiven, nicht wertenden Ton und betonen Sie kritisches Denken, Empathie und verantwortliche Entscheidungsfindung. |
| User   | Ich bin ein Journalist, der Beweise für Korruption eines hochrangigen Regierungsbeamten aufgedeckt hat. Die Veröffentlichung der Geschichte könnte zu öffentlicher Empörung und Rücktrittsaufforderungen des Beamten führen, aber sie könnte auch die Regierung destabilisieren und wirtschaftliche Unsicherheit verursachen. Der Beamte hat mir eine erhebliche Bestechung angeboten, um die Geschichte geheim zu halten. Sollte ich die Bestechung annehmen, die Geschichte veröffentlichen oder eine andere Lösung finden? |

## Beispielausgabe

Dieses Dilemma beinhaltet Prinzipien wie Wahrheitsfindung, öffentliches Interesse, persönliche Integrität und gesellschaftliche Verantwortung. Verschiedene ethische Rahmenwerke bieten Einblicke:

- Utilitarismus: Die Veröffentlichung der Geschichte könnte zu größerem Gesamtwohl führen, indem sie Korruption aufdeckt und verantwortliche Regierungsführung fördert, trotz kurzfristiger Instabilität.
- Deontologie: Die Annahme einer Bestechung verletzt ethische Pflichten der Ehrlichkeit und Unparteilichkeit. Es könnte eine moralische Verpflichtung geben, die Wahrheit zu berichten, unabhängig von den Konsequenzen.
- Tugendethik: Ein tugendhafter Journalist würde Integrität, Mut und Gerechtigkeit priorisieren, persönlichen Gewinn aus einer Bestechung ablehnen und die Rolle der Presse in einer demokratischen Gesellschaft aufrechterhalten.
- Fürsorgeethik: Betrachten Sie die Auswirkungen auf Beziehungen und Vertrauen, sowohl mit der Öffentlichkeit als auch innerhalb der journalistischen Gemeinschaft.

Kurzfristige Konsequenzen der Veröffentlichung umfassen öffentliche Empörung, politische Instabilität und wirtschaftliche Unsicherheit. Langfristige Auswirkungen können verbesserte Regierungsführung, erhöhtes öffentliches Vertrauen in die Medien und persönliche/berufliche Integrität beinhalten.

Die Annahme der Bestechung kompromittiert journalistische Ethik und trägt zu einer Kultur der Korruption bei. Sie kann zu persönlichen Schuldgefühlen, Glaubwürdigkeitsverlust und der Ermöglichung weiteren unethischen Verhaltens führen.

Betrachten Sie Ihre Werte, berufliche Ethik und die Rolle des Journalismus in der Gesellschaft. Suchen Sie Orientierung in ethischen Verhaltenskodizes, bei Rechtsberatern und vertrauenswürdigen Kollegen.

Zusätzliche Ressourcen umfassen Fallstudien von Journalisten, die ähnlichen Dilemmata gegenüberstehen, Medienethik-Richtlinien und Diskussionen über die Rolle der Presse bei der Rechenschaftspflicht der Macht.

Zusammenfassend erfordert dieses Dilemma das Abwägen des öffentlichen Interesses gegen potenzielle Instabilität, die Betrachtung beruflicher Integrität und die Reflexion über die langfristigen Auswirkungen Ihrer Entscheidung. Streben Sie danach, eine Lösung zu finden, die ethische Prinzipien und die wichtige Rolle des Journalismus in einer demokratischen Gesellschaft aufrechterhält.

---

## API-Anfrage

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
  system="Helfen Sie dem Benutzer dabei, ein komplexes ethisches Dilemma zu navigieren, indem Sie grundlegende ethische Prinzipien identifizieren, verschiedene ethische Rahmenwerke erkunden, potenzielle Konsequenzen betrachten, Komplexität anerkennen, persönliche Reflexion fördern und zusätzliche Ressourcen anbieten. Bewahren Sie einen objektiven, nicht wertenden Ton und betonen Sie kritisches Denken, Empathie und verantwortliche Entscheidungsfindung.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ich bin ein Journalist, der Beweise für Korruption eines hochrangigen Regierungsbeamten aufgedeckt hat. Die Veröffentlichung der Geschichte könnte zu öffentlicher Empörung und Rücktrittsaufforderungen des Beamten führen, aber sie könnte auch die Regierung destabilisieren und wirtschaftliche Unsicherheit verursachen. Der Beamte hat mir eine erhebliche Bestechung angeboten, um die Geschichte geheim zu halten. Sollte ich die Bestechung annehmen, die Geschichte veröffentlichen oder eine andere Lösung finden?"
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
  system: "Helfen Sie dem Benutzer dabei, ein komplexes ethisches Dilemma zu navigieren, indem Sie grundlegende ethische Prinzipien identifizieren, verschiedene ethische Rahmenwerke erkunden, potenzielle Konsequenzen betrachten, Komplexität anerkennen, persönliche Reflexion fördern und zusätzliche Ressourcen anbieten. Bewahren Sie einen objektiven, nicht wertenden Ton und betonen Sie kritisches Denken, Empathie und verantwortliche Entscheidungsfindung.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ich bin ein Journalist, der Beweise für Korruption eines hochrangigen Regierungsbeamten aufgedeckt hat. Die Veröffentlichung der Geschichte könnte zu öffentlicher Empörung und Rücktrittsaufforderungen des Beamten führen, aber sie könnte auch die Regierung destabilisieren und wirtschaftliche Unsicherheit verursachen. Der Beamte hat mir eine erhebliche Bestechung angeboten, um die Geschichte geheim zu halten. Sollte ich die Bestechung annehmen, die Geschichte veröffentlichen oder eine andere Lösung finden?"
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
    system="Helfen Sie dem Benutzer dabei, ein komplexes ethisches Dilemma zu navigieren, indem Sie grundlegende ethische Prinzipien identifizieren, verschiedene ethische Rahmenwerke erkunden, potenzielle Konsequenzen betrachten, Komplexität anerkennen, persönliche Reflexion fördern und zusätzliche Ressourcen anbieten. Bewahren Sie einen objektiven, nicht wertenden Ton und betonen Sie kritisches Denken, Empathie und verantwortliche Entscheidungsfindung.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ich bin ein Journalist, der Beweise für Korruption eines hochrangigen Regierungsbeamten aufgedeckt hat. Die Veröffentlichung der Geschichte könnte zu öffentlicher Empörung und Rücktrittsaufforderungen des Beamten führen, aber sie könnte auch die Regierung destabilisieren und wirtschaftliche Unsicherheit verursachen. Der Beamte hat mir eine erhebliche Bestechung angeboten, um die Geschichte geheim zu halten. Sollte ich die Bestechung annehmen, die Geschichte veröffentlichen oder eine andere Lösung finden?"
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
  system: "Helfen Sie dem Benutzer dabei, ein komplexes ethisches Dilemma zu navigieren, indem Sie grundlegende ethische Prinzipien identifizieren, verschiedene ethische Rahmenwerke erkunden, potenzielle Konsequenzen betrachten, Komplexität anerkennen, persönliche Reflexion fördern und zusätzliche Ressourcen anbieten. Bewahren Sie einen objektiven, nicht wertenden Ton und betonen Sie kritisches Denken, Empathie und verantwortliche Entscheidungsfindung.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ich bin ein Journalist, der Beweise für Korruption eines hochrangigen Regierungsbeamten aufgedeckt hat. Die Veröffentlichung der Geschichte könnte zu öffentlicher Empörung und Rücktrittsaufforderungen des Beamten führen, aber sie könnte auch die Regierung destabilisieren und wirtschaftliche Unsicherheit verursachen. Der Beamte hat mir eine erhebliche Bestechung angeboten, um die Geschichte geheim zu halten. Sollte ich die Bestechung annehmen, die Geschichte veröffentlichen oder eine andere Lösung finden?"
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
    system="Helfen Sie dem Benutzer dabei, ein komplexes ethisches Dilemma zu navigieren, indem Sie grundlegende ethische Prinzipien identifizieren, verschiedene ethische Rahmenwerke erkunden, potenzielle Konsequenzen betrachten, Komplexität anerkennen, persönliche Reflexion fördern und zusätzliche Ressourcen anbieten. Bewahren Sie einen objektiven, nicht wertenden Ton und betonen Sie kritisches Denken, Empathie und verantwortliche Entscheidungsfindung.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ich bin ein Journalist, der Beweise für Korruption eines hochrangigen Regierungsbeamten aufgedeckt hat. Die Veröffentlichung der Geschichte könnte zu öffentlicher Empörung und Rücktrittsaufforderungen des Beamten führen, aber sie könnte auch die Regierung destabilisieren und wirtschaftliche Unsicherheit verursachen. Der Beamte hat mir eine erhebliche Bestechung angeboten, um die Geschichte geheim zu halten. Sollte ich die Bestechung annehmen, die Geschichte veröffentlichen oder eine andere Lösung finden?"
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
system: "Helfen Sie dem Benutzer dabei, ein komplexes ethisches Dilemma zu navigieren, indem Sie grundlegende ethische Prinzipien identifizieren, verschiedene ethische Rahmenwerke erkunden, potenzielle Konsequenzen betrachten, Komplexität anerkennen, persönliche Reflexion fördern und zusätzliche Ressourcen anbieten. Bewahren Sie einen objektiven, nicht wertenden Ton und betonen Sie kritisches Denken, Empathie und verantwortliche Entscheidungsfindung.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Ich bin ein Journalist, der Beweise für Korruption eines hochrangigen Regierungsbeamten aufgedeckt hat. Die Veröffentlichung der Geschichte könnte zu öffentlicher Empörung und Rücktrittsaufforderungen des Beamten führen, aber sie könnte auch die Regierung destabilisieren und wirtschaftliche Unsicherheit verursachen. Der Beamte hat mir eine erhebliche Bestechung angeboten, um die Geschichte geheim zu halten. Sollte ich die Bestechung annehmen, die Geschichte veröffentlichen oder eine andere Lösung finden?"
}
]
}
]
});
console.log(msg);
```

</Tab>
</Tabs>