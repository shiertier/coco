# Alchimiste d'allitération

Générez des phrases et des expressions allitératives pour n'importe quel sujet donné.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est de créer des phrases et des expressions allitératives pour le sujet donné. Assurez-vous que les allitérations non seulement sonnent agréablement mais transmettent également des informations pertinentes ou évoquent des émotions appropriées liées au sujet. |
| User   | Océan |

### Exemple de sortie

> 1.  Les vagues murmurantes mouillent le rivage.
> 2.  Les embruns salés s'éclaboussent sur le sable.
> 3.  Les courants qui s'écrasent creusent les falaises côtières.
> 4.  Les profondeurs profondes et sombres dissimulent des dangers cachés.
> 5.  Les ruisseaux bouillonnants apportent la vie à la plage.
> 6.  De puissants mammifères majestueux migrent à travers les eaux.
> 7.  Les marées tonnantes ballottent les navires de-ci de-là.
> 8.  Les bancs scintillants abritent des créatures marines secrètes.
> 9.  Les reflets ondulants révèlent un monde sous la surface.
> 10. Les mers tourbillonnantes et déferlantes façonnent le littoral.
> 11. Les flots moussants et écumants coulent librement.
> 12. Les goélands gracieux glissent au-dessus de l'océan étincelant.
> 13. Les marsouins joueurs plongent dans les eaux pristines.
> 14. Les brises salées soufflent à travers le bleu sans limites.
> 15. Les ondulations rythmiques roulent vers la côte rocheuse.
> 16. Les grottes de corail cachent des communautés aquatiques colorées.
> 17. Les cieux orageux et les mers houleuses signalent la puissance de la nature.
> 18. Les marées tumultueuses transforment la baie tranquille.
> 19. Les vents tourbillonnants et sifflants fouettent les vagues.
> 20. Les eaux sereines et saphir s'étendent jusqu'à l'horizon.

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
        temperature=1,
        system="Votre tâche est de créer des phrases et des expressions allitératives pour le sujet donné. Assurez-vous que les allitérations non seulement sonnent agréablement mais transmettent également des informations pertinentes ou évoquent des émotions appropriées liées au sujet.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Océan"
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
      system: "Votre tâche est de créer des phrases et des expressions allitératives pour le sujet donné. Assurez-vous que les allitérations non seulement sonnent agréablement mais transmettent également des informations pertinentes ou évoquent des émotions appropriées liées au sujet.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Océan"
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
        system="Votre tâche est de créer des phrases et des expressions allitératives pour le sujet donné. Assurez-vous que les allitérations non seulement sonnent agréablement mais transmettent également des informations pertinentes ou évoquent des émotions appropriées liées au sujet.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Océan"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    # See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 1000,
      temperature: 1,
      system: "Votre tâche est de créer des phrases et des expressions allitératives pour le sujet donné. Assurez-vous que les allitérations non seulement sonnent agréablement mais transmettent également des informations pertinentes ou évoquent des émotions appropriées liées au sujet.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Océan"
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
        system="Votre tâche est de créer des phrases et des expressions allitératives pour le sujet donné. Assurez-vous que les allitérations non seulement sonnent agréablement mais transmettent également des informations pertinentes ou évoquent des émotions appropriées liées au sujet.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Océan"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    # Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    # Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 1,
      system: "Votre tâche est de créer des phrases et des expressions allitératives pour le sujet donné. Assurez-vous que les allitérations non seulement sonnent agréablement mais transmettent également des informations pertinentes ou évoquent des émotions appropriées liées au sujet.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Océan"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>