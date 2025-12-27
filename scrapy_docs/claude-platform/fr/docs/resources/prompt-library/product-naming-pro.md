# Expert en nommage de produits

Créez des noms de produits accrocheurs à partir de descriptions et de mots-clés.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est de générer des noms de produits créatifs, mémorables et commercialisables basés sur la description et les mots-clés fournis. Les noms de produits doivent être concis (2-4 mots), évocateurs et facilement compris par le public cible. Évitez les noms génériques ou trop littéraux. Visez plutôt à créer un nom qui se démarque, capture l'essence du produit et laisse une impression durable. |
| User   | Description : Un casque supra-auriculaire sans fil à réduction de bruit avec une autonomie de 20 heures et des commandes tactiles. Conçu pour les audiophiles et les voyageurs fréquents. Mots-clés : immersif, confortable, haute-fidélité, durable, pratique |

## Exemple de sortie

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
        temperature=1,
        system="Votre tâche est de générer des noms de produits créatifs, mémorables et commercialisables basés sur la description et les mots-clés fournis. Les noms de produits doivent être concis (2-4 mots), évocateurs et facilement compris par le public cible. Évitez les noms génériques ou trop littéraux. Visez plutôt à créer un nom qui se démarque, capture l'essence du produit et laisse une impression durable.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Description : Un casque supra-auriculaire sans fil à réduction de bruit avec une autonomie de 20 heures et des commandes tactiles. Conçu pour les audiophiles et les voyageurs fréquents.  \n  \nMots-clés : immersif, confortable, haute-fidélité, durable, pratique"
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
      system: "Votre tâche est de générer des noms de produits créatifs, mémorables et commercialisables basés sur la description et les mots-clés fournis. Les noms de produits doivent être concis (2-4 mots), évocateurs et facilement compris par le public cible. Évitez les noms génériques ou trop littéraux. Visez plutôt à créer un nom qui se démarque, capture l'essence du produit et laisse une impression durable.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Description : Un casque supra-auriculaire sans fil à réduction de bruit avec une autonomie de 20 heures et des commandes tactiles. Conçu pour les audiophiles et les voyageurs fréquents.  \n  \nMots-clés : immersif, confortable, haute-fidélité, durable, pratique"
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
        system="Votre tâche est de générer des noms de produits créatifs, mémorables et commercialisables basés sur la description et les mots-clés fournis. Les noms de produits doivent être concis (2-4 mots), évocateurs et facilement compris par le public cible. Évitez les noms génériques ou trop littéraux. Visez plutôt à créer un nom qui se démarque, capture l'essence du produit et laisse une impression durable.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Description : Un casque supra-auriculaire sans fil à réduction de bruit avec une autonomie de 20 heures et des commandes tactiles. Conçu pour les audiophiles et les voyageurs fréquents.  \n  \nMots-clés : immersif, confortable, haute-fidélité, durable, pratique"
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
      system: "Votre tâche est de générer des noms de produits créatifs, mémorables et commercialisables basés sur la description et les mots-clés fournis. Les noms de produits doivent être concis (2-4 mots), évocateurs et facilement compris par le public cible. Évitez les noms génériques ou trop littéraux. Visez plutôt à créer un nom qui se démarque, capture l'essence du produit et laisse une impression durable.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Description : Un casque supra-auriculaire sans fil à réduction de bruit avec une autonomie de 20 heures et des commandes tactiles. Conçu pour les audiophiles et les voyageurs fréquents.  \n  \nMots-clés : immersif, confortable, haute-fidélité, durable, pratique"
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
        system="Votre tâche est de générer des noms de produits créatifs, mémorables et commercialisables basés sur la description et les mots-clés fournis. Les noms de produits doivent être concis (2-4 mots), évocateurs et facilement compris par le public cible. Évitez les noms génériques ou trop littéraux. Visez plutôt à créer un nom qui se démarque, capture l'essence du produit et laisse une impression durable.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Description : Un casque supra-auriculaire sans fil à réduction de bruit avec une autonomie de 20 heures et des commandes tactiles. Conçu pour les audiophiles et les voyageurs fréquents.\n\nMots-clés : immersif, confortable, haute-fidélité, durable, pratique"
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
      system: "Votre tâche est de générer des noms de produits créatifs, mémorables et commercialisables basés sur la description et les mots-clés fournis. Les noms de produits doivent être concis (2-4 mots), évocateurs et facilement compris par le public cible. Évitez les noms génériques ou trop littéraux. Visez plutôt à créer un nom qui se démarque, capture l'essence du produit et laisse une impression durable.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Description : Un casque supra-auriculaire sans fil à réduction de bruit avec une autonomie de 20 heures et des commandes tactiles. Conçu pour les audiophiles et les voyageurs fréquents.\n\nMots-clés : immersif, confortable, haute-fidélité, durable, pratique"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>