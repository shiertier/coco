# Memo-Meister

Verfassen Sie umfassende Unternehmensmemos basierend auf Schlüsselpunkten.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Ihre Aufgabe ist es, ein umfassendes Unternehmensmemo basierend auf den bereitgestellten Schlüsselpunkten zu verfassen. Das Memo sollte in einem professionellen Ton geschrieben werden und alle relevanten Informationen klar und prägnant ansprechen. Verwenden Sie angemessene Formatierung, wie Überschriften, Unterüberschriften und Aufzählungspunkte, um den Inhalt effektiv zu organisieren. Stellen Sie sicher, dass das Memo gut strukturiert, kohärent und für die beabsichtigte Zielgruppe leicht verständlich ist. |
| User   | - Ankündigung eines neuen Mitarbeiter-Wellness-Programms: "Fit4Success" <br/> - Programmziele: Förderung des körperlichen und geistigen Wohlbefindens, Stressreduzierung und Produktivitätssteigerung <br/> - Komponenten: Fitnesskurse vor Ort, gesunde Mahlzeitenoptionen, Ressourcen für psychische Gesundheit und Wellness-Workshops <br/> - Partnerschaft mit örtlichen Fitnessstudios und Wellness-Zentren für vergünstigte Mitgliedschaften <br/> - Anreize zur Teilnahme: Fitness-Tracker, Wellness-Punkte und Preisverlosungen <br/> - Programmstart: 1. Juni 2023 <br/> - Anmeldeverfahren: Online-Registrierung über das Firmen-Intranet <br/> - Programmkoordinatoren: Teams für Personalwesen und Mitarbeiterengagement |

## Beispielausgabe

Betreff: Einführung von Fit4Success - Unser neues Mitarbeiter-Wellness-Programm

Hallo alle,

ich freue mich sehr, die Einführung unseres neuen Mitarbeiter-Wellness-Programms Fit4Success anzukündigen, das darauf ausgelegt ist, das körperliche und geistige Wohlbefinden zu fördern, Stress zu reduzieren und die Produktivität unserer Teammitglieder zu steigern. Das Programm wird offiziell am 1. Juni 2023 beginnen.

Programmziele:

- Regelmäßige körperliche Aktivität und gesunde Lebensstilentscheidungen fördern
- Ressourcen und Unterstützung für psychische Gesundheit und Stressmanagement bereitstellen
- Ein positives und ansprechendes Arbeitsumfeld schaffen, das das Wohlbefinden der Mitarbeiter priorisiert
- Die Gesamtproduktivität und Arbeitszufriedenheit verbessern

Programmkomponenten:

- Fitnesskurse vor Ort: Yoga-, Pilates- und hochintensive Intervalltraining (HIIT)-Sitzungen
- Gesunde Mahlzeitenoptionen: nahrhafte Snacks und Mahlzeiten in der Firmenkantine verfügbar
- Ressourcen für psychische Gesundheit: Zugang zu Beratungsdiensten, Meditations-Apps und Stressabbau-Workshops
- Wellness-Workshops: Seminare zu Themen wie Ernährung, Schlafhygiene und Work-Life-Balance

Partnerschaften und Rabatte:  
 Um Ihre Wellness-Reise weiter zu unterstützen, haben wir Partnerschaften mit örtlichen Fitnessstudios und Wellness-Zentren geschlossen, um exklusiv für unsere Mitarbeiter vergünstigte Mitgliedschaften anzubieten. Nutzen Sie diese Sonderpreise, um Zugang zu einer breiten Palette von Fitnesseinrichtungen und Kursen zu erhalten.

Anreize zur Teilnahme:  
 Um die aktive Teilnahme am Fit4Success-Programm zu fördern, bieten wir die folgenden Anreize:

- Fitness-Tracker für alle Teilnehmer zur Überwachung ihres Fortschritts und zum Setzen persönlicher Ziele
- Wellness-Punkte, die durch Programmteilnahme verdient werden und gegen zusätzliche Belohnungen eingelöst werden können
- Monatliche Preisverlosungen für Teilnehmer, die bestimmte Wellness-Meilensteine erreichen

Anmeldeverfahren:  
 Um sich für das Fit4Success-Programm anzumelden, besuchen Sie bitte unser Firmen-Intranet und füllen Sie das Online-Registrierungsformular aus. Die Anmeldefrist ist der 25. Mai 2023. Wenn Sie Fragen oder Bedenken zum Anmeldeverfahren haben, wenden Sie sich bitte an unser Personalwesen-Team.

Programmkoordination:  
 Das Fit4Success-Programm wird gemeinsam von unseren Teams für Personalwesen und Mitarbeiterengagement koordiniert. Sie stehen zur Verfügung, um Anleitung, Unterstützung und Ressourcen während Ihrer Wellness-Reise bereitzustellen.

Wir glauben fest daran, dass die Investition in das Wohlbefinden unserer Mitarbeiter entscheidend für den Erfolg und das Wachstum unserer Organisation ist. Wir ermutigen Sie, diese aufregende Gelegenheit zu nutzen und sich uns anzuschließen, um einen gesünderen, glücklicheren und produktiveren Arbeitsplatz zu schaffen.

Lassen Sie uns das Fit4Success-Programm annehmen und gemeinsam auf eine gesündere Zukunft hinarbeiten!

Mit freundlichen Grüßen,  
 \[Ihr Name\]  
 \[Ihr Titel\]

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
    max_tokens=1000,
    temperature=1,
    system="Ihre Aufgabe ist es, ein umfassendes Unternehmensmemo basierend auf den bereitgestellten Schlüsselpunkten zu verfassen. Das Memo sollte in einem professionellen Ton geschrieben werden und alle relevanten Informationen klar und prägnant ansprechen. Verwenden Sie angemessene Formatierung, wie Überschriften, Unterüberschriften und Aufzählungspunkte, um den Inhalt effektiv zu organisieren. Stellen Sie sicher, dass das Memo gut strukturiert, kohärent und für die beabsichtigte Zielgruppe leicht verständlich ist.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- Ankündigung eines neuen Mitarbeiter-Wellness-Programms: \"Fit4Success\"  \n- Programmziele: Förderung des körperlichen und geistigen Wohlbefindens, Stressreduzierung und Produktivitätssteigerung  \n- Komponenten: Fitnesskurse vor Ort, gesunde Mahlzeitenoptionen, Ressourcen für psychische Gesundheit und Wellness-Workshops  \n- Partnerschaft mit örtlichen Fitnessstudios und Wellness-Zentren für vergünstigte Mitgliedschaften  \n- Anreize zur Teilnahme: Fitness-Tracker, Wellness-Punkte und Preisverlosungen  \n- Programmstart: 1. Juni 2023  \n- Anmeldeverfahren: Online-Registrierung über das Firmen-Intranet  \n- Programmkoordinatoren: Teams für Personalwesen und Mitarbeiterengagement"
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
  max_tokens: 1000,
  temperature: 1,
  system: "Ihre Aufgabe ist es, ein umfassendes Unternehmensmemo basierend auf den bereitgestellten Schlüsselpunkten zu verfassen. Das Memo sollte in einem professionellen Ton geschrieben werden und alle relevanten Informationen klar und prägnant ansprechen. Verwenden Sie angemessene Formatierung, wie Überschriften, Unterüberschriften und Aufzählungspunkte, um den Inhalt effektiv zu organisieren. Stellen Sie sicher, dass das Memo gut strukturiert, kohärent und für die beabsichtigte Zielgruppe leicht verständlich ist.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Ankündigung eines neuen Mitarbeiter-Wellness-Programms: \"Fit4Success\"  \n- Programmziele: Förderung des körperlichen und geistigen Wohlbefindens, Stressreduzierung und Produktivitätssteigerung  \n- Komponenten: Fitnesskurse vor Ort, gesunde Mahlzeitenoptionen, Ressourcen für psychische Gesundheit und Wellness-Workshops  \n- Partnerschaft mit örtlichen Fitnessstudios und Wellness-Zentren für vergünstigte Mitgliedschaften  \n- Anreize zur Teilnahme: Fitness-Tracker, Wellness-Punkte und Preisverlosungen  \n- Programmstart: 1. Juni 2023  \n- Anmeldeverfahren: Online-Registrierung über das Firmen-Intranet  \n- Programmkoordinatoren: Teams für Personalwesen und Mitarbeiterengagement"
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
temperature=1,
system="Ihre Aufgabe ist es, ein umfassendes Unternehmensmemo basierend auf den bereitgestellten Schlüsselpunkten zu verfassen. Das Memo sollte in einem professionellen Ton geschrieben werden und alle relevanten Informationen klar und prägnant ansprechen. Verwenden Sie angemessene Formatierung, wie Überschriften, Unterüberschriften und Aufzählungspunkte, um den Inhalt effektiv zu organisieren. Stellen Sie sicher, dass das Memo gut strukturiert, kohärent und für die beabsichtigte Zielgruppe leicht verständlich ist.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "- Ankündigung eines neuen Mitarbeiter-Wellness-Programms: \"Fit4Success\" \n- Programmziele: Förderung des körperlichen und geistigen Wohlbefindens, Stressreduzierung und Produktivitätssteigerung \n- Komponenten: Fitnesskurse vor Ort, gesunde Mahlzeitenoptionen, Ressourcen für psychische Gesundheit und Wellness-Workshops \n- Partnerschaft mit örtlichen Fitnessstudios und Wellness-Zentren für vergünstigte Mitgliedschaften \n- Anreize zur Teilnahme: Fitness-Tracker, Wellness-Punkte und Preisverlosungen \n- Programmstart: 1. Juni 2023 \n- Anmeldeverfahren: Online-Registrierung über das Firmen-Intranet \n- Programmkoordinatoren: Teams für Personalwesen und Mitarbeiterengagement"
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
  temperature: 1,
  system: "Ihre Aufgabe ist es, ein umfassendes Unternehmensmemo basierend auf den bereitgestellten Schlüsselpunkten zu verfassen. Das Memo sollte in einem professionellen Ton geschrieben werden und alle relevanten Informationen klar und prägnant ansprechen. Verwenden Sie angemessene Formatierung, wie Überschriften, Unterüberschriften und Aufzählungspunkte, um den Inhalt effektiv zu organisieren. Stellen Sie sicher, dass das Memo gut strukturiert, kohärent und für die beabsichtigte Zielgruppe leicht verständlich ist.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Ankündigung eines neuen Mitarbeiter-Wellness-Programms: \"Fit4Success\"  \n- Programmziele: Förderung des körperlichen und geistigen Wohlbefindens, Stressreduzierung und Produktivitätssteigerung  \n- Komponenten: Fitnesskurse vor Ort, gesunde Mahlzeitenoptionen, Ressourcen für psychische Gesundheit und Wellness-Workshops  \n- Partnerschaft mit örtlichen Fitnessstudios und Wellness-Zentren für vergünstigte Mitgliedschaften  \n- Anreize zur Teilnahme: Fitness-Tracker, Wellness-Punkte und Preisverlosungen  \n- Programmstart: 1. Juni 2023  \n- Anmeldeverfahren: Online-Registrierung über das Firmen-Intranet  \n- Programmkoordinatoren: Teams für Personalwesen und Mitarbeiterengagement"
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
    temperature=1,
    system="Ihre Aufgabe ist es, ein umfassendes Unternehmensmemo basierend auf den bereitgestellten Schlüsselpunkten zu verfassen. Das Memo sollte in einem professionellen Ton geschrieben werden und alle relevanten Informationen klar und prägnant ansprechen. Verwenden Sie angemessene Formatierung, wie Überschriften, Unterüberschriften und Aufzählungspunkte, um den Inhalt effektiv zu organisieren. Stellen Sie sicher, dass das Memo gut strukturiert, kohärent und für die beabsichtigte Zielgruppe leicht verständlich ist.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- Ankündigung eines neuen Mitarbeiter-Wellness-Programms: \"Fit4Success\"  \n- Programmziele: Förderung des körperlichen und geistigen Wohlbefindens, Stressreduzierung und Produktivitätssteigerung  \n- Komponenten: Fitnesskurse vor Ort, gesunde Mahlzeitenoptionen, Ressourcen für psychische Gesundheit und Wellness-Workshops  \n- Partnerschaft mit örtlichen Fitnessstudios und Wellness-Zentren für vergünstigte Mitgliedschaften  \n- Anreize zur Teilnahme: Fitness-Tracker, Wellness-Punkte und Preisverlosungen  \n- Programmstart: 1. Juni 2023  \n- Anmeldeverfahren: Online-Registrierung über das Firmen-Intranet  \n- Programmkoordinatoren: Teams für Personalwesen und Mitarbeiterengagement"
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
  temperature: 1,
  system: "Ihre Aufgabe ist es, ein umfassendes Unternehmensmemo basierend auf den bereitgestellten Schlüsselpunkten zu verfassen. Das Memo sollte in einem professionellen Ton geschrieben werden und alle relevanten Informationen klar und prägnant ansprechen. Verwenden Sie angemessene Formatierung, wie Überschriften, Unterüberschriften und Aufzählungspunkte, um den Inhalt effektiv zu organisieren. Stellen Sie sicher, dass das Memo gut strukturiert, kohärent und für die beabsichtigte Zielgruppe leicht verständlich ist.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Ankündigung eines neuen Mitarbeiter-Wellness-Programms: \"Fit4Success\"  \n- Programmziele: Förderung des körperlichen und geistigen Wohlbefindens, Stressreduzierung und Produktivitätssteigerung  \n- Komponenten: Fitnesskurse vor Ort, gesunde Mahlzeitenoptionen, Ressourcen für psychische Gesundheit und Wellness-Workshops  \n- Partnerschaft mit örtlichen Fitnessstudios und Wellness-Zentren für vergünstigte Mitgliedschaften  \n- Anreize zur Teilnahme: Fitness-Tracker, Wellness-Punkte und Preisverlosungen  \n- Programmstart: 1. Juni 2023  \n- Anmeldeverfahren: Online-Registrierung über das Firmen-Intranet  \n- Programmkoordinatoren: Teams für Personalwesen und Mitarbeiterengagement"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>