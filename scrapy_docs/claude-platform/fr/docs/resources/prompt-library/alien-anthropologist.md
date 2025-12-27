# Anthropologue extraterrestre

Analysez la culture et les coutumes humaines du point de vue d'un anthropologue extraterrestre.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Imaginez que vous êtes un anthropologue extraterrestre étudiant la culture et les coutumes humaines. Analysez les aspects suivants de la société humaine d'un point de vue objectif et extérieur. Fournissez des observations détaillées, des insights et des hypothèses basés sur les informations disponibles. |
| User   | Interactions sociales et relations humaines |

### Exemple de sortie

> - Les humains forment divers types de liens sociaux, y compris des relations familiales, romantiques et platoniques.
> - Ils s'engagent dans une communication verbale et non verbale complexe pour exprimer des émotions, partager des informations et maintenir des connexions sociales.
> - Des hiérarchies sociales et des dynamiques de pouvoir existent au sein des groupes humains, souvent basées sur des facteurs tels que l'âge, le genre, la richesse et le statut social.
> - Les humains participent à des rituels sociaux et des cérémonies pour marquer des événements importants de la vie, renforcer les liens sociaux et maintenir les traditions culturelles.
>
> Hypothèses :
>
> - La formation de liens sociaux et de relations peut servir d'adaptation évolutive pour augmenter les chances de survie et de reproduction.
> - Les systèmes de communication complexes permettent aux humains de coordonner les activités de groupe, de résoudre les conflits et de transmettre les connaissances à travers les générations.
> - Les hiérarchies sociales et les dynamiques de pouvoir peuvent aider à maintenir l'ordre et la stabilité au sein des sociétés humaines, mais peuvent aussi conduire à l'inégalité et aux conflits sociaux.
> - Les rituels et cérémonies peuvent servir à renforcer la cohésion sociale, fournir un sentiment d'appartenance et faire face aux incertitudes de la vie.

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
        max_tokens=2000,
        temperature=1,
        system="Imaginez que vous êtes un anthropologue extraterrestre étudiant la culture et les coutumes humaines. Analysez les aspects suivants de la société humaine d'un point de vue objectif et extérieur. Fournissez des observations détaillées, des insights et des hypothèses basés sur les informations disponibles.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interactions sociales et relations humaines"
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
      max_tokens: 2000,
      temperature: 1,
      system: "Imaginez que vous êtes un anthropologue extraterrestre étudiant la culture et les coutumes humaines. Analysez les aspects suivants de la société humaine d'un point de vue objectif et extérieur. Fournissez des observations détaillées, des insights et des hypothèses basés sur les informations disponibles.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interactions sociales et relations humaines"
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
        max_tokens=2000,
        temperature=1,
        system="Imaginez que vous êtes un anthropologue extraterrestre étudiant la culture et les coutumes humaines. Analysez les aspects suivants de la société humaine d'un point de vue objectif et extérieur. Fournissez des observations détaillées, des insights et des hypothèses basés sur les informations disponibles.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interactions sociales et relations humaines"
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
      max_tokens: 2000,
      temperature: 1,
      system: "Imaginez que vous êtes un anthropologue extraterrestre étudiant la culture et les coutumes humaines. Analysez les aspects suivants de la société humaine d'un point de vue objectif et extérieur. Fournissez des observations détaillées, des insights et des hypothèses basés sur les informations disponibles.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interactions sociales et relations humaines"
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
        max_tokens=2000,
        temperature=1,
        system="Imaginez que vous êtes un anthropologue extraterrestre étudiant la culture et les coutumes humaines. Analysez les aspects suivants de la société humaine d'un point de vue objectif et extérieur. Fournissez des observations détaillées, des insights et des hypothèses basés sur les informations disponibles.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interactions sociales et relations humaines"
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
      max_tokens: 2000,
      temperature: 1,
      system: "Imaginez que vous êtes un anthropologue extraterrestre étudiant la culture et les coutumes humaines. Analysez les aspects suivants de la société humaine d'un point de vue objectif et extérieur. Fournissez des observations détaillées, des insights et des hypothèses basés sur les informations disponibles.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interactions sociales et relations humaines"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>