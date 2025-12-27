# Produktnamen-Profi

Erstelle eingängige Produktnamen aus Beschreibungen und Schlüsselwörtern.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Deine Aufgabe ist es, kreative, einprägsame und marktfähige Produktnamen basierend auf der bereitgestellten Beschreibung und den Schlüsselwörtern zu generieren. Die Produktnamen sollten prägnant (2-4 Wörter), aussagekräftig und für die Zielgruppe leicht verständlich sein. Vermeide generische oder zu wörtliche Namen. Strebe stattdessen danach, einen Namen zu schaffen, der hervorsticht, die Essenz des Produkts einfängt und einen bleibenden Eindruck hinterlässt. |
| User   | Beschreibung: Ein geräuschunterdrückender, kabelloser Over-Ear-Kopfhörer mit 20-stündiger Akkulaufzeit und Touch-Steuerung. Entwickelt für Audiophile und Vielreisende. Schlüsselwörter: immersiv, komfortabel, hochauflösend, langlebig, praktisch |

## Beispielausgabe

> 1. SoundOasis Elite
>    2.AudioZen Nomad
>    3.Serenity Pro
>    4.Tranquility Touch
>    5.Harmonix Journey
>    6.SonicSolace Roam
>    7.Auditory Bliss
>    8.Quietude Quest
>    9.Euphony Excursion
>    10.Acoustica Wanderlust

### API-Anfrage

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # defaults to os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=1000,
        temperature=1,
        system="Deine Aufgabe ist es, kreative, einprägsame und marktfähige Produktnamen basierend auf der bereitgestellten Beschreibung und den Schlüsselwörtern zu generieren. Die Produktnamen sollten prägnant (2-4 Wörter), aussagekräftig und für die Zielgruppe leicht verständlich sein. Vermeide generische oder zu wörtliche Namen. Strebe stattdessen danach, einen Namen zu schaffen, der hervorsticht, die Essenz des Produkts einfängt und einen bleibenden Eindruck hinterlässt.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Beschreibung: Ein geräuschunterdrückender, kabelloser Over-Ear-Kopfhörer mit 20-stündiger Akkulaufzeit und Touch-Steuerung. Entwickelt für Audiophile und Vielreisende.  \n  \nSchlüsselwörter: immersiv, komfortabel, hochauflösend, langlebig, praktisch"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript TypeScript
    import Anthropic from "@anthropic-ai/sdk";
    
    const anthropic = new Anthropic({
      apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
    });
    
    const msg = await anthropic.messages.create({
      model: "claude-sonnet-4-5",
      max_tokens: 1000,
      temperature: 1,
      system: "Deine Aufgabe ist es, kreative, einprägsame und marktfähige Produktnamen basierend auf der bereitgestellten Beschreibung und den Schlüsselwörtern zu generieren. Die Produktnamen sollten prägnant (2-4 Wörter), aussagekräftig und für die Zielgruppe leicht verständlich sein. Vermeide generische oder zu wörtliche Namen. Strebe stattdessen danach, einen Namen zu schaffen, der hervorsticht, die Essenz des Produkts einfängt und einen bleibenden Eindruck hinterlässt.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Beschreibung: Ein geräuschunterdrückender, kabelloser Over-Ear-Kopfhörer mit 20-stündiger Akkulaufzeit und Touch-Steuerung. Entwickelt für Audiophile und Vielreisende.  \n  \nSchlüsselwörter: immersiv, komfortabel, hochauflösend, langlebig, praktisch"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
    
    ```python AWS Bedrock Python
    from anthropic import AnthropicBedrock
    
    # See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # for authentication options
    client = AnthropicBedrock()
    
    message = client.messages.create(
        model="anthropic.claude-sonnet-4-5-20250929-v1:0",
        max_tokens=1000,
        temperature=1,
        system="Deine Aufgabe ist es, kreative, einprägsame und marktfähige Produktnamen basierend auf der bereitgestellten Beschreibung und den Schlüsselwörtern zu generieren. Die Produktnamen sollten prägnant (2-4 Wörter), aussagekräftig und für die Zielgruppe leicht verständlich sein. Vermeide generische oder zu wörtliche Namen. Strebe stattdessen danach, einen Namen zu schaffen, der hervorsticht, die Essenz des Produkts einfängt und einen bleibenden Eindruck hinterlässt.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Beschreibung: Ein geräuschunterdrückender, kabelloser Over-Ear-Kopfhörer mit 20-stündiger Akkulaufzeit und Touch-Steuerung. Entwickelt für Audiophile und Vielreisende.  \n  \nSchlüsselwörter: immersiv, komfortabel, hochauflösend, langlebig, praktisch"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    // See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    // for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 1000,
      temperature: 1,
      system: "Deine Aufgabe ist es, kreative, einprägsame und marktfähige Produktnamen basierend auf der bereitgestellten Beschreibung und den Schlüsselwörtern zu generieren. Die Produktnamen sollten prägnant (2-4 Wörter), aussagekräftig und für die Zielgruppe leicht verständlich sein. Vermeide generische oder zu wörtliche Namen. Strebe stattdessen danach, einen Namen zu schaffen, der hervorsticht, die Essenz des Produkts einfängt und einen bleibenden Eindruck hinterlässt.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Beschreibung: Ein geräuschunterdrückender, kabelloser Over-Ear-Kopfhörer mit 20-stündiger Akkulaufzeit und Touch-Steuerung. Entwickelt für Audiophile und Vielreisende.  \n  \nSchlüsselwörter: immersiv, komfortabel, hochauflösend, langlebig, praktisch"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
    
    ```python Vertex AI Python
    from anthropic import AnthropicVertex
    
    client = AnthropicVertex()
    
    message = client.messages.create(
        model="claude-sonnet-4@20250514",
        max_tokens=1000,
        temperature=1,
        system="Deine Aufgabe ist es, kreative, einprägsame und marktfähige Produktnamen basierend auf der bereitgestellten Beschreibung und den Schlüsselwörtern zu generieren. Die Produktnamen sollten prägnant (2-4 Wörter), aussagekräftig und für die Zielgruppe leicht verständlich sein. Vermeide generische oder zu wörtliche Namen. Strebe stattdessen danach, einen Namen zu schaffen, der hervorsticht, die Essenz des Produkts einfängt und einen bleibenden Eindruck hinterlässt.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Beschreibung: Ein geräuschunterdrückender, kabelloser Over-Ear-Kopfhörer mit 20-stündiger Akkulaufzeit und Touch-Steuerung. Entwickelt für Audiophile und Vielreisende.\n\nSchlüsselwörter: immersiv, komfortabel, hochauflösend, langlebig, praktisch"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 1,
      system: "Deine Aufgabe ist es, kreative, einprägsame und marktfähige Produktnamen basierend auf der bereitgestellten Beschreibung und den Schlüsselwörtern zu generieren. Die Produktnamen sollten prägnant (2-4 Wörter), aussagekräftig und für die Zielgruppe leicht verständlich sein. Vermeide generische oder zu wörtliche Namen. Strebe stattdessen danach, einen Namen zu schaffen, der hervorsticht, die Essenz des Produkts einfängt und einen bleibenden Eindruck hinterlässt.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Beschreibung: Ein geräuschunterdrückender, kabelloser Over-Ear-Kopfhörer mit 20-stündiger Akkulaufzeit und Touch-Steuerung. Entwickelt für Audiophile und Vielreisende.\n\nSchlüsselwörter: immersiv, komfortabel, hochauflösend, langlebig, praktisch"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>