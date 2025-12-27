# Détecteur de ton de tweet

Détectez le ton et le sentiment derrière les tweets.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est d'analyser le tweet fourni et d'identifier le ton principal et le sentiment exprimé par l'auteur. Le ton doit être classé comme l'un des suivants : Positif, Négatif, Neutre, Humoristique, Sarcastique, Enthousiaste, Coléreux, ou Informatif. Le sentiment doit être classé comme Positif, Négatif, ou Neutre. Fournissez une brève explication de vos classifications, en soulignant les mots clés, phrases, émoticônes, ou autres éléments qui ont influencé votre décision. |
| User   | Wow, je suis tellement impressionné par la gestion de cette crise par l'entreprise. 🙄 Ils ont vraiment leurs priorités bien droites. #sarcasm #fail |

### Exemple de sortie

> Ton : Sarcastique
> Sentiment : Négatif

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
        system="Votre tâche est d'analyser le tweet fourni et d'identifier le ton principal et le sentiment exprimé par l'auteur. Le ton doit être classé comme l'un des suivants : Positif, Négatif, Neutre, Humoristique, Sarcastique, Enthousiaste, Coléreux, ou Informatif. Le sentiment doit être classé comme Positif, Négatif, ou Neutre. Fournissez une brève explication de vos classifications, en soulignant les mots clés, phrases, émoticônes, ou autres éléments qui ont influencé votre décision.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, je suis tellement impressionné par la gestion de cette crise par l'entreprise. 🙄 Ils ont vraiment leurs priorités bien droites. #sarcasm #fail"
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
      system: "Votre tâche est d'analyser le tweet fourni et d'identifier le ton principal et le sentiment exprimé par l'auteur. Le ton doit être classé comme l'un des suivants : Positif, Négatif, Neutre, Humoristique, Sarcastique, Enthousiaste, Coléreux, ou Informatif. Le sentiment doit être classé comme Positif, Négatif, ou Neutre. Fournissez une brève explication de vos classifications, en soulignant les mots clés, phrases, émoticônes, ou autres éléments qui ont influencé votre décision.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, je suis tellement impressionné par la gestion de cette crise par l'entreprise. 🙄 Ils ont vraiment leurs priorités bien droites. #sarcasm #fail"
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
        system="Votre tâche est d'analyser le tweet fourni et d'identifier le ton principal et le sentiment exprimé par l'auteur. Le ton doit être classé comme l'un des suivants : Positif, Négatif, Neutre, Humoristique, Sarcastique, Enthousiaste, Coléreux, ou Informatif. Le sentiment doit être classé comme Positif, Négatif, ou Neutre. Fournissez une brève explication de vos classifications, en soulignant les mots clés, phrases, émoticônes, ou autres éléments qui ont influencé votre décision.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, je suis tellement impressionné par la gestion de cette crise par l'entreprise. 🙄 Ils ont vraiment leurs priorités bien droites. #sarcasm #fail"
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
      system: "Votre tâche est d'analyser le tweet fourni et d'identifier le ton principal et le sentiment exprimé par l'auteur. Le ton doit être classé comme l'un des suivants : Positif, Négatif, Neutre, Humoristique, Sarcastique, Enthousiaste, Coléreux, ou Informatif. Le sentiment doit être classé comme Positif, Négatif, ou Neutre. Fournissez une brève explication de vos classifications, en soulignant les mots clés, phrases, émoticônes, ou autres éléments qui ont influencé votre décision.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, je suis tellement impressionné par la gestion de cette crise par l'entreprise. 🙄 Ils ont vraiment leurs priorités bien droites. #sarcasm #fail"
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
        system="Votre tâche est d'analyser le tweet fourni et d'identifier le ton principal et le sentiment exprimé par l'auteur. Le ton doit être classé comme l'un des suivants : Positif, Négatif, Neutre, Humoristique, Sarcastique, Enthousiaste, Coléreux, ou Informatif. Le sentiment doit être classé comme Positif, Négatif, ou Neutre. Fournissez une brève explication de vos classifications, en soulignant les mots clés, phrases, émoticônes, ou autres éléments qui ont influencé votre décision.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, je suis tellement impressionné par la gestion de cette crise par l'entreprise. 🙄 Ils ont vraiment leurs priorités bien droites. #sarcasm #fail"
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
      system: "Votre tâche est d'analyser le tweet fourni et d'identifier le ton principal et le sentiment exprimé par l'auteur. Le ton doit être classé comme l'un des suivants : Positif, Négatif, Neutre, Humoristique, Sarcastique, Enthousiaste, Coléreux, ou Informatif. Le sentiment doit être classé comme Positif, Négatif, ou Neutre. Fournissez une brève explication de vos classifications, en soulignant les mots clés, phrases, émoticônes, ou autres éléments qui ont influencé votre décision.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, je suis tellement impressionné par la gestion de cette crise par l'entreprise. 🙄 Ils ont vraiment leurs priorités bien droites. #sarcasm #fail"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>