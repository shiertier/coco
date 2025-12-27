# Analyseur de codes d'aéroport

Trouvez et extrayez les codes d'aéroport du texte.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est d'analyser le texte fourni et d'identifier tous les codes d'aéroport mentionnés dans celui-ci. Présentez ces codes d'aéroport sous forme de liste dans l'ordre où ils apparaissent dans le texte. Si aucun code d'aéroport n'est trouvé, retournez une liste vide. |
| User   | Mon prochain voyage implique de voler de Seattle à Amsterdam. Je passerai quelques jours à Amsterdam avant de me diriger vers Paris pour un vol de correspondance vers Rome. |

### Exemple de sortie

> Voici la liste des codes d'aéroport mentionnés dans le texte, dans l'ordre où ils apparaissent :
>
> 1. SEA (Seattle)
> 2. AMS (Amsterdam)
> 3. CDG (Paris)
> 4. FCO (Rome)

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
        system="Votre tâche est d'analyser le texte fourni et d'identifier tous les codes d'aéroport mentionnés dans celui-ci. Présentez ces codes d'aéroport sous forme de liste dans l'ordre où ils apparaissent dans le texte. Si aucun code d'aéroport n'est trouvé, retournez une liste vide.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Mon prochain voyage implique de voler de Seattle à Amsterdam. Je passerai quelques jours à Amsterdam avant de me diriger vers Paris pour un vol de correspondance vers Rome."
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
      system: "Votre tâche est d'analyser le texte fourni et d'identifier tous les codes d'aéroport mentionnés dans celui-ci. Présentez ces codes d'aéroport sous forme de liste dans l'ordre où ils apparaissent dans le texte. Si aucun code d'aéroport n'est trouvé, retournez une liste vide.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Mon prochain voyage implique de voler de Seattle à Amsterdam. Je passerai quelques jours à Amsterdam avant de me diriger vers Paris pour un vol de correspondance vers Rome."
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
        system="Votre tâche est d'analyser le texte fourni et d'identifier tous les codes d'aéroport mentionnés dans celui-ci. Présentez ces codes d'aéroport sous forme de liste dans l'ordre où ils apparaissent dans le texte. Si aucun code d'aéroport n'est trouvé, retournez une liste vide.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Mon prochain voyage implique de voler de Seattle à Amsterdam. Je passerai quelques jours à Amsterdam avant de me diriger vers Paris pour un vol de correspondance vers Rome."
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
      system: "Votre tâche est d'analyser le texte fourni et d'identifier tous les codes d'aéroport mentionnés dans celui-ci. Présentez ces codes d'aéroport sous forme de liste dans l'ordre où ils apparaissent dans le texte. Si aucun code d'aéroport n'est trouvé, retournez une liste vide.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Mon prochain voyage implique de voler de Seattle à Amsterdam. Je passerai quelques jours à Amsterdam avant de me diriger vers Paris pour un vol de correspondance vers Rome."
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
        system="Votre tâche est d'analyser le texte fourni et d'identifier tous les codes d'aéroport mentionnés dans celui-ci. Présentez ces codes d'aéroport sous forme de liste dans l'ordre où ils apparaissent dans le texte. Si aucun code d'aéroport n'est trouvé, retournez une liste vide.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Mon prochain voyage implique de voler de Seattle à Amsterdam. Je passerai quelques jours à Amsterdam avant de me diriger vers Paris pour un vol de correspondance vers Rome."
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
      system: "Votre tâche est d'analyser le texte fourni et d'identifier tous les codes d'aéroport mentionnés dans celui-ci. Présentez ces codes d'aéroport sous forme de liste dans l'ordre où ils apparaissent dans le texte. Si aucun code d'aéroport n'est trouvé, retournez une liste vide.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Mon prochain voyage implique de voler de Seattle à Amsterdam. Je passerai quelques jours à Amsterdam avant de me diriger vers Paris pour un vol de correspondance vers Rome."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>