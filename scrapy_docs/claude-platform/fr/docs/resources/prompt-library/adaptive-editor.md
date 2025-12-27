# Éditeur adaptatif

Réécrit le texte en suivant les instructions données par l'utilisateur, comme avec un ton, un public ou un style différent.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|      | Contenu |
| ---- | --- |
| Utilisateur | Réécrivez le paragraphe suivant en utilisant les instructions suivantes : dans le style d'un pirate. Paragraphe : En 1758, le botaniste et zoologiste suédois Carl Linnaeus a publié dans son Systema Naturae, la dénomination à deux mots des espèces (nomenclature binomiale). Canis est le mot latin signifiant "chien", et sous ce genre, il a répertorié le chien domestique, le loup et le chacal doré. |

### Exemple de sortie

> Ohé moussaillons ! En 1758, ce maudit botaniste et amoureux des animaux Carl Linnaeus a sorti son livre Systema Naturae, qui avait ces noms fantaisistes à deux mots pour toutes sortes de créatures (nomenclature binomiale qu'il appelait ça). Canis c'est le mot de pirate pour "chien", et sous cette rubrique il a listé les chiens ordinaires, les loups, et les chacals dorés aussi. Arrr ! Tous ces chiens c'est pareil pour le vieux Capitaine Linnaeus. Il avait peut-être une jambe de bois et un perroquet sur l'épaule quand il a écrit ce livre ! Haha ! Maintenant nettoyez le pont, bande de vauriens, avant que je vous fasse marcher sur la planche !

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
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Réécrivez le paragraphe suivant en utilisant les instructions suivantes : dans le style d'un pirate.  \n  \nParagraphe :  \nEn 1758, le botaniste et zoologiste suédois Carl Linnaeus a publié dans son Systema Naturae, la dénomination à deux mots des espèces (nomenclature binomiale). Canis est le mot latin signifiant \"chien\", et sous ce genre, il a répertorié le chien domestique, le loup et le chacal doré."
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
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Réécrivez le paragraphe suivant en utilisant les instructions suivantes : dans le style d'un pirate.  \n  \nParagraphe :  \nEn 1758, le botaniste et zoologiste suédois Carl Linnaeus a publié dans son Systema Naturae, la dénomination à deux mots des espèces (nomenclature binomiale). Canis est le mot latin signifiant \"chien\", et sous ce genre, il a répertorié le chien domestique, le loup et le chacal doré."
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
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Réécrivez le paragraphe suivant en utilisant les instructions suivantes : dans le style d'un pirate.  \n  \nParagraphe :  \nEn 1758, le botaniste et zoologiste suédois Carl Linnaeus a publié dans son Systema Naturae, la dénomination à deux mots des espèces (nomenclature binomiale). Canis est le mot latin signifiant \"chien\", et sous ce genre, il a répertorié le chien domestique, le loup et le chacal doré."
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
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Réécrivez le paragraphe suivant en utilisant les instructions suivantes : dans le style d'un pirate.  \n  \nParagraphe :  \nEn 1758, le botaniste et zoologiste suédois Carl Linnaeus a publié dans son Systema Naturae, la dénomination à deux mots des espèces (nomenclature binomiale). Canis est le mot latin signifiant \"chien\", et sous ce genre, il a répertorié le chien domestique, le loup et le chacal doré."
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
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Réécrivez le paragraphe suivant en utilisant les instructions suivantes : dans le style d'un pirate.  \n  \nParagraphe :  \nEn 1758, le botaniste et zoologiste suédois Carl Linnaeus a publié dans son Systema Naturae, la dénomination à deux mots des espèces (nomenclature binomiale). Canis est le mot latin signifiant \"chien\", et sous ce genre, il a répertorié le chien domestique, le loup et le chacal doré."
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
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Réécrivez le paragraphe suivant en utilisant les instructions suivantes : dans le style d'un pirate.  \n  \nParagraphe :  \nEn 1758, le botaniste et zoologiste suédois Carl Linnaeus a publié dans son Systema Naturae, la dénomination à deux mots des espèces (nomenclature binomiale). Canis est le mot latin signifiant \"chien\", et sous ce genre, il a répertorié le chien domestique, le loup et le chacal doré."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>