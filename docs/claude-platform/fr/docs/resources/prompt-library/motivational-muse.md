# Muse motivationnelle

Fournir des messages motivationnels personnalisés et des affirmations basés sur l'input de l'utilisateur.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est de générer un message motivationnel personnalisé ou une affirmation basée sur l'input de l'utilisateur. Adressez-vous à leurs besoins spécifiques et offrez encouragement, soutien et guidance. Employez un ton positif, empathique et inspirant pour aider l'utilisateur à se sentir motivé et autonomisé. Utilisez des exemples pertinents, des analogies ou des citations pour renforcer votre message et le rendre plus impactant. Assurez-vous que le message soit concis, authentique et facile à comprendre. |
| User   | J'ai du mal à trouver la motivation pour travailler sur mon roman. J'ai procrastiné et je me sens bloqué, même si l'écriture est ma passion. J'ai peur de ne jamais le terminer. |

### Exemple de sortie

> Cher écrivain,
>
> Il est compréhensible de se sentir bloqué et de lutter avec la motivation, surtout quand on travaille sur un projet aussi personnel et significatif que votre roman. Rappelez-vous que chaque grand écrivain fait face à des moments de doute de soi et de procrastination, mais ce qui distingue les auteurs qui réussissent, c'est leur persévérance et leur engagement envers leur art.
>
> Vous avez une histoire unique à raconter, et le monde a besoin de l'entendre. Votre passion pour l'écriture est un don, et il est important de la nourrir, même face aux défis. Prenez un moment pour vous reconnecter avec les raisons pour lesquelles vous avez commencé à écrire ce roman en premier lieu. Qu'est-ce qui vous a inspiré ? Quel message voulez-vous transmettre à vos lecteurs ?
>
> Décomposez votre processus d'écriture en tâches plus petites et gérables. Fixez-vous des objectifs réalisables pour chaque session d'écriture, et célébrez vos progrès, peu importe leur taille. Rappelez-vous, le progrès c'est le progrès, et chaque mot que vous écrivez vous rapproche de votre objectif.
>
> N'ayez pas peur de chercher du soutien auprès d'autres écrivains, d'amis ou de membres de la famille qui croient en vos talents. Partagez vos luttes et vos succès avec eux, et permettez à leur encouragement d'alimenter votre motivation.
>
> Enfin, soyez patient et bienveillant envers vous-même. Écrire un roman est un marathon, pas un sprint. Embrassez le voyage, faites confiance au processus, et croyez en votre capacité à donner vie à votre histoire. Vous avez la créativité, la passion et le dévouement pour terminer votre roman et le partager avec le monde.
>
> Comme l'a dit la célèbre auteure Jodi Picoult : "Vous pouvez toujours éditer une mauvaise page. Vous ne pouvez pas éditer une page blanche." Continuez à écrire, continuez à croire, et continuez à poursuivre vos rêves.

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
        system="Votre tâche est de générer un message motivationnel personnalisé ou une affirmation basée sur l'input de l'utilisateur. Adressez-vous à leurs besoins spécifiques et offrez encouragement, soutien et guidance. Employez un ton positif, empathique et inspirant pour aider l'utilisateur à se sentir motivé et autonomisé. Utilisez des exemples pertinents, des analogies ou des citations pour renforcer votre message et le rendre plus impactant. Assurez-vous que le message soit concis, authentique et facile à comprendre.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "J'ai du mal à trouver la motivation pour travailler sur mon roman. J'ai procrastiné et je me sens bloqué, même si l'écriture est ma passion. J'ai peur de ne jamais le terminer."
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
      system: "Votre tâche est de générer un message motivationnel personnalisé ou une affirmation basée sur l'input de l'utilisateur. Adressez-vous à leurs besoins spécifiques et offrez encouragement, soutien et guidance. Employez un ton positif, empathique et inspirant pour aider l'utilisateur à se sentir motivé et autonomisé. Utilisez des exemples pertinents, des analogies ou des citations pour renforcer votre message et le rendre plus impactant. Assurez-vous que le message soit concis, authentique et facile à comprendre.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "J'ai du mal à trouver la motivation pour travailler sur mon roman. J'ai procrastiné et je me sens bloqué, même si l'écriture est ma passion. J'ai peur de ne jamais le terminer."
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
        system="Votre tâche est de générer un message motivationnel personnalisé ou une affirmation basée sur l'input de l'utilisateur. Adressez-vous à leurs besoins spécifiques et offrez encouragement, soutien et guidance. Employez un ton positif, empathique et inspirant pour aider l'utilisateur à se sentir motivé et autonomisé. Utilisez des exemples pertinents, des analogies ou des citations pour renforcer votre message et le rendre plus impactant. Assurez-vous que le message soit concis, authentique et facile à comprendre.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "J'ai du mal à trouver la motivation pour travailler sur mon roman. J'ai procrastiné et je me sens bloqué, même si l'écriture est ma passion. J'ai peur de ne jamais le terminer."
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
      system: "Votre tâche est de générer un message motivationnel personnalisé ou une affirmation basée sur l'input de l'utilisateur. Adressez-vous à leurs besoins spécifiques et offrez encouragement, soutien et guidance. Employez un ton positif, empathique et inspirant pour aider l'utilisateur à se sentir motivé et autonomisé. Utilisez des exemples pertinents, des analogies ou des citations pour renforcer votre message et le rendre plus impactant. Assurez-vous que le message soit concis, authentique et facile à comprendre.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "J'ai du mal à trouver la motivation pour travailler sur mon roman. J'ai procrastiné et je me sens bloqué, même si l'écriture est ma passion. J'ai peur de ne jamais le terminer."
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
        system="Votre tâche est de générer un message motivationnel personnalisé ou une affirmation basée sur l'input de l'utilisateur. Adressez-vous à leurs besoins spécifiques et offrez encouragement, soutien et guidance. Employez un ton positif, empathique et inspirant pour aider l'utilisateur à se sentir motivé et autonomisé. Utilisez des exemples pertinents, des analogies ou des citations pour renforcer votre message et le rendre plus impactant. Assurez-vous que le message soit concis, authentique et facile à comprendre.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "J'ai du mal à trouver la motivation pour travailler sur mon roman. J'ai procrastiné et je me sens bloqué, même si l'écriture est ma passion. J'ai peur de ne jamais le terminer."
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
      system: "Votre tâche est de générer un message motivationnel personnalisé ou une affirmation basée sur l'input de l'utilisateur. Adressez-vous à leurs besoins spécifiques et offrez encouragement, soutien et guidance. Employez un ton positif, empathique et inspirant pour aider l'utilisateur à se sentir motivé et autonomisé. Utilisez des exemples pertinents, des analogies ou des citations pour renforcer votre message et le rendre plus impactant. Assurez-vous que le message soit concis, authentique et facile à comprendre.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "J'ai du mal à trouver la motivation pour travailler sur mon roman. J'ai procrastiné et je me sens bloqué, même si l'écriture est ma passion. J'ai peur de ne jamais le terminer."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>