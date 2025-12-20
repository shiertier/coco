# VR-Fitness-Innovator

Brainstorming kreativer Ideen für Virtual-Reality-Fitnessspiele.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

| Rolle | Inhalt |
| ---- | --- |
| Benutzer | Ihre Aufgabe ist es, eine Liste innovativer und ansprechender Ideen für Virtual-Reality-(VR)-Fitnessspiele zu erstellen. Berücksichtigen Sie verschiedene Spielgenres, einzigartige Gameplay-Mechaniken, immersive Umgebungen und kreative Wege, körperliche Übungen und Bewegungen zu integrieren. Die Ideen sollten für eine breite Palette von Fitness-Enthusiasten und Gamern ansprechend sein und sie dazu ermutigen, aktiv zu bleiben und Spaß beim Training in VR zu haben. Geben Sie für jede Idee eine kurze Beschreibung des Spielkonzepts, der wichtigsten Funktionen und wie es Fitness fördert. |

## Beispielausgabe

1.  Idee: VR-Parkour-Abenteuer  
    Beschreibung: Spieler navigieren durch eine dynamische urbane Umgebung, laufen, springen, klettern und führen Parkour-Stunts aus, um Hindernisse zu überwinden und Ziele zu erreichen. Das Spiel passt sich den Bewegungen des Spielers an und bietet ein intensives Ganzkörpertraining beim Erkunden der virtuellen Stadt.  
    Hauptmerkmale: Realistische Parkour-Mechaniken, prozedural generierte Umgebungen, Multiplayer-Rennen und Herausforderungen.  
    Fitness-Aspekte: Cardio, Agilität, Koordination und Unterkörperkraft.
2.  Idee: Rhythmus-Boxen  
    Beschreibung: Ein energiegeladenes Boxspiel im Takt beliebter Musiktitel. Spieler müssen im Rhythmus schlagen, ausweichen und blocken, um Punkte zu sammeln und durch zunehmend herausfordernde Level zu gelangen. Das Spiel bietet eine Vielzahl von Boxstilen und Trainingsmodi.  
    Hauptmerkmale: Lizenzierte Musik, anpassbare Boxer, Online-Multiplayer und eine vielfältige Palette von Boxtechniken.  
    Fitness-Aspekte: Cardio, Oberkörperkraft, Reflexe und Ausdauer.
3.  Idee: VR-Fitness-RPG  
    Beschreibung: Ein immersives Rollenspiel, bei dem Spieler ihren eigenen Charakter erstellen und sich auf eine Quest begeben, um eine Fantasy-Welt zu retten. Das Spiel kombiniert traditionelle RPG-Elemente mit Fitness-Herausforderungen und erfordert von den Spielern, körperliche Übungen auszuführen, um Zauber zu wirken, Feinde zu besiegen und ihren Charakter zu leveln.  
    Hauptmerkmale: Charakteranpassung, Fähigkeitsbäume, epische Boss-Kämpfe und eine Mischung aus Kraft-, Cardio- und Flexibilitätsübungen.  
    Fitness-Aspekte: Ganzkörpertraining, Krafttraining, Cardio und Flexibilität.

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
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ihre Aufgabe ist es, eine Liste innovativer und ansprechender Ideen für Virtual-Reality-(VR)-Fitnessspiele zu erstellen. Berücksichtigen Sie verschiedene Spielgenres, einzigartige Gameplay-Mechaniken, immersive Umgebungen und kreative Wege, körperliche Übungen und Bewegungen zu integrieren. Die Ideen sollten für eine breite Palette von Fitness-Enthusiasten und Gamern ansprechend sein und sie dazu ermutigen, aktiv zu bleiben und Spaß beim Training in VR zu haben. Geben Sie für jede Idee eine kurze Beschreibung des Spielkonzepts, der wichtigsten Funktionen und wie es Fitness fördert."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ihre Aufgabe ist es, eine Liste innovativer und ansprechender Ideen für Virtual-Reality-(VR)-Fitnessspiele zu erstellen. Berücksichtigen Sie verschiedene Spielgenres, einzigartige Gameplay-Mechaniken, immersive Umgebungen und kreative Wege, körperliche Übungen und Bewegungen zu integrieren. Die Ideen sollten für eine breite Palette von Fitness-Enthusiasten und Gamern ansprechend sein und sie dazu ermutigen, aktiv zu bleiben und Spaß beim Training in VR zu haben. Geben Sie für jede Idee eine kurze Beschreibung des Spielkonzepts, der wichtigsten Funktionen und wie es Fitness fördert."
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
    max_tokens=1000,
    temperature=1,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ihre Aufgabe ist es, eine Liste innovativer und ansprechender Ideen für Virtual-Reality-(VR)-Fitnessspiele zu erstellen. Berücksichtigen Sie verschiedene Spielgenres, einzigartige Gameplay-Mechaniken, immersive Umgebungen und kreative Wege, körperliche Übungen und Bewegungen zu integrieren. Die Ideen sollten für eine breite Palette von Fitness-Enthusiasten und Gamern ansprechend sein und sie dazu ermutigen, aktiv zu bleiben und Spaß beim Training in VR zu haben. Geben Sie für jede Idee eine kurze Beschreibung des Spielkonzepts, der wichtigsten Funktionen und wie es Fitness fördert."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ihre Aufgabe ist es, eine Liste innovativer und ansprechender Ideen für Virtual-Reality-(VR)-Fitnessspiele zu erstellen. Berücksichtigen Sie verschiedene Spielgenres, einzigartige Gameplay-Mechaniken, immersive Umgebungen und kreative Wege, körperliche Übungen und Bewegungen zu integrieren. Die Ideen sollten für eine breite Palette von Fitness-Enthusiasten und Gamern ansprechend sein und sie dazu ermutigen, aktiv zu bleiben und Spaß beim Training in VR zu haben. Geben Sie für jede Idee eine kurze Beschreibung des Spielkonzepts, der wichtigsten Funktionen und wie es Fitness fördert."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ihre Aufgabe ist es, eine Liste innovativer und ansprechender Ideen für Virtual-Reality-(VR)-Fitnessspiele zu erstellen. Berücksichtigen Sie verschiedene Spielgenres, einzigartige Gameplay-Mechaniken, immersive Umgebungen und kreative Wege, körperliche Übungen und Bewegungen zu integrieren. Die Ideen sollten für eine breite Palette von Fitness-Enthusiasten und Gamern ansprechend sein und sie dazu ermutigen, aktiv zu bleiben und Spaß beim Training in VR zu haben. Geben Sie für jede Idee eine kurze Beschreibung des Spielkonzepts, der wichtigsten Funktionen und wie es Fitness fördert."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ihre Aufgabe ist es, eine Liste innovativer und ansprechender Ideen für Virtual-Reality-(VR)-Fitnessspiele zu erstellen. Berücksichtigen Sie verschiedene Spielgenres, einzigartige Gameplay-Mechaniken, immersive Umgebungen und kreative Wege, körperliche Übungen und Bewegungen zu integrieren. Die Ideen sollten für eine breite Palette von Fitness-Enthusiasten und Gamern ansprechend sein und sie dazu ermutigen, aktiv zu bleiben und Spaß beim Training in VR zu haben. Geben Sie für jede Idee eine kurze Beschreibung des Spielkonzepts, der wichtigsten Funktionen und wie es Fitness fördert."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>