# Extracteur d'e-mails

Extraire les adresses e-mail d'un document dans une liste formatée en JSON.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Copiez précisément toutes les adresses e-mail du texte suivant puis écrivez-les, une par ligne. N'écrivez une adresse e-mail que si elle est précisément épelée dans le texte d'entrée. S'il n'y a pas d'adresses e-mail dans le texte, écrivez "N/A". Ne dites rien d'autre. |
| User   | Répertoire téléphonique : John Latrabe, 555-232-1995, [john909709@geemail.com] Josie Lana, 555-759-2905, [josie@josielananier.com] Keven Stevens, 555-980-7000, [drkevin22@geemail.com] Le répertoire téléphonique sera tenu à jour par le responsable RH.                      |

### Exemple de sortie

> john909709@geemail.com > josie@josielananier.com > drkevin22@geemail.com

---

### Requête API

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
        temperature=0,
        system="Copiez précisément toutes les adresses e-mail du texte suivant puis écrivez-les, une par ligne. N'écrivez une adresse e-mail que si elle est précisément épelée dans le texte d'entrée. S'il n'y a pas d'adresses e-mail dans le texte, écrivez \"N/A\".  Ne dites rien d'autre.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Répertoire téléphonique :  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nLe répertoire téléphonique sera tenu à jour par le responsable RH."
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
      temperature: 0,
      system: "Copiez précisément toutes les adresses e-mail du texte suivant puis écrivez-les, une par ligne. N'écrivez une adresse e-mail que si elle est précisément épelée dans le texte d'entrée. S'il n'y a pas d'adresses e-mail dans le texte, écrivez \"N/A\".  Ne dites rien d'autre.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Répertoire téléphonique :  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nLe répertoire téléphonique sera tenu à jour par le responsable RH."
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
        temperature=0,
        system="Copiez précisément toutes les adresses e-mail du texte suivant puis écrivez-les, une par ligne. N'écrivez une adresse e-mail que si elle est précisément épelée dans le texte d'entrée. S'il n'y a pas d'adresses e-mail dans le texte, écrivez \"N/A\".  Ne dites rien d'autre.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Répertoire téléphonique :  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nLe répertoire téléphonique sera tenu à jour par le responsable RH."
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
      temperature: 0,
      system: "Copiez précisément toutes les adresses e-mail du texte suivant puis écrivez-les, une par ligne. N'écrivez une adresse e-mail que si elle est précisément épelée dans le texte d'entrée. S'il n'y a pas d'adresses e-mail dans le texte, écrivez \"N/A\".  Ne dites rien d'autre.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Répertoire téléphonique :  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nLe répertoire téléphonique sera tenu à jour par le responsable RH."
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
        temperature=0,
        system="Copiez précisément toutes les adresses e-mail du texte suivant puis écrivez-les, une par ligne. N'écrivez une adresse e-mail que si elle est précisément épelée dans le texte d'entrée. S'il n'y a pas d'adresses e-mail dans le texte, écrivez \"N/A\".  Ne dites rien d'autre.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Répertoire téléphonique :  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nLe répertoire téléphonique sera tenu à jour par le responsable RH."
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
      temperature: 0,
      system: "Copiez précisément toutes les adresses e-mail du texte suivant puis écrivez-les, une par ligne. N'écrivez une adresse e-mail que si elle est précisément épelée dans le texte d'entrée. S'il n'y a pas d'adresses e-mail dans le texte, écrivez \"N/A\".  Ne dites rien d'autre.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Répertoire téléphonique :  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nLe répertoire téléphonique sera tenu à jour par le responsable RH."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>