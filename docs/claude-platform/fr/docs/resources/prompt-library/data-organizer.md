# Organisateur de données

Transformez du texte non structuré en tableaux JSON personnalisés.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche consiste à prendre le texte non structuré fourni et à le convertir en un format de tableau bien organisé en utilisant JSON. Identifiez les principales entités, attributs ou catégories mentionnés dans le texte et utilisez-les comme clés dans l'objet JSON. Ensuite, extrayez les informations pertinentes du texte et remplissez les valeurs correspondantes dans l'objet JSON. Assurez-vous que les données sont représentées avec précision et correctement formatées dans la structure JSON. Le tableau JSON résultant devrait fournir un aperçu clair et structuré des informations présentées dans le texte original. |
| User   | Silvermist Hollow, un charmant village, abritait un groupe extraordinaire d'individus. Parmi eux se trouvait le Dr Liam Patel, un neurochirurgien de 45 ans formé à Yale qui a révolutionné les techniques chirurgicales au centre médical régional. Olivia Chen, à 28 ans, était une architecte innovante de UC Berkeley qui a transformé le paysage du village avec ses conceptions durables et époustouflantes. Le théâtre local était honoré par les symphonies enchanteresses d'Ethan Kovacs, un musicien et compositeur de 72 ans formé à Juilliard. Isabella Torres, une chef autodidacte passionnée par les ingrédients locaux, a créé une sensation culinaire avec son restaurant de la ferme à la table, qui est devenu une destination incontournable pour les amateurs de gastronomie. Ces individus remarquables, chacun avec leurs talents distincts, ont contribué à la tapisserie vibrante de la vie à Silvermist Hollow. |

### Exemple de sortie

```json
[
  {
    "name": "Dr. Liam Patel",
    "age": 45,
    "profession": "Neurochirurgien",
    "education": "Yale",
    "accomplishments": "A révolutionné les techniques chirurgicales au centre médical régional"
  },
  {
    "name": "Olivia Chen",
    "age": 28,
    "profession": "Architecte",
    "education": "UC Berkeley",
    "accomplishments": "A transformé le paysage du village avec des conceptions durables et époustouflantes"
  },
  {
    "name": "Ethan Kovacs",
    "age": 72,
    "profession": "Musicien et Compositeur",
    "education": "Juilliard",
    "accomplishments": "A honoré le théâtre local avec des symphonies enchanteresses"
  },
  {
    "name": "Isabella Torres",
    "age": null,
    "profession": "Chef",
    "education": "Autodidacte",
    "accomplishments": "A créé une sensation culinaire avec son restaurant de la ferme à la table, qui est devenu une destination incontournable pour les amateurs de gastronomie"
  }
]
```

---

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
        system="Votre tâche consiste à prendre le texte non structuré fourni et à le convertir en un format de tableau bien organisé en utilisant JSON. Identifiez les principales entités, attributs ou catégories mentionnés dans le texte et utilisez-les comme clés dans l'objet JSON. Ensuite, extrayez les informations pertinentes du texte et remplissez les valeurs correspondantes dans l'objet JSON. Assurez-vous que les données sont représentées avec précision et correctement formatées dans la structure JSON. Le tableau JSON résultant devrait fournir un aperçu clair et structuré des informations présentées dans le texte original.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, un charmant village, abritait un groupe extraordinaire d'individus. Parmi eux se trouvait le Dr Liam Patel, un neurochirurgien de 45 ans formé à Yale qui a révolutionné les techniques chirurgicales au centre médical régional. Olivia Chen, à 28 ans, était une architecte innovante de UC Berkeley qui a transformé le paysage du village avec ses conceptions durables et époustouflantes. Le théâtre local était honoré par les symphonies enchanteresses d'Ethan Kovacs, un musicien et compositeur de 72 ans formé à Juilliard. Isabella Torres, une chef autodidacte passionnée par les ingrédients locaux, a créé une sensation culinaire avec son restaurant de la ferme à la table, qui est devenu une destination incontournable pour les amateurs de gastronomie. Ces individus remarquables, chacun avec leurs talents distincts, ont contribué à la tapisserie vibrante de la vie à Silvermist Hollow."
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
      system: "Votre tâche consiste à prendre le texte non structuré fourni et à le convertir en un format de tableau bien organisé en utilisant JSON. Identifiez les principales entités, attributs ou catégories mentionnés dans le texte et utilisez-les comme clés dans l'objet JSON. Ensuite, extrayez les informations pertinentes du texte et remplissez les valeurs correspondantes dans l'objet JSON. Assurez-vous que les données sont représentées avec précision et correctement formatées dans la structure JSON. Le tableau JSON résultant devrait fournir un aperçu clair et structuré des informations présentées dans le texte original.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, un charmant village, abritait un groupe extraordinaire d'individus. Parmi eux se trouvait le Dr Liam Patel, un neurochirurgien de 45 ans formé à Yale qui a révolutionné les techniques chirurgicales au centre médical régional. Olivia Chen, à 28 ans, était une architecte innovante de UC Berkeley qui a transformé le paysage du village avec ses conceptions durables et époustouflantes. Le théâtre local était honoré par les symphonies enchanteresses d'Ethan Kovacs, un musicien et compositeur de 72 ans formé à Juilliard. Isabella Torres, une chef autodidacte passionnée par les ingrédients locaux, a créé une sensation culinaire avec son restaurant de la ferme à la table, qui est devenu une destination incontournable pour les amateurs de gastronomie. Ces individus remarquables, chacun avec leurs talents distincts, ont contribué à la tapisserie vibrante de la vie à Silvermist Hollow."
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
        system="Votre tâche consiste à prendre le texte non structuré fourni et à le convertir en un format de tableau bien organisé en utilisant JSON. Identifiez les principales entités, attributs ou catégories mentionnés dans le texte et utilisez-les comme clés dans l'objet JSON. Ensuite, extrayez les informations pertinentes du texte et remplissez les valeurs correspondantes dans l'objet JSON. Assurez-vous que les données sont représentées avec précision et correctement formatées dans la structure JSON. Le tableau JSON résultant devrait fournir un aperçu clair et structuré des informations présentées dans le texte original.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, un charmant village, abritait un groupe extraordinaire d'individus. Parmi eux se trouvait le Dr Liam Patel, un neurochirurgien de 45 ans formé à Yale qui a révolutionné les techniques chirurgicales au centre médical régional. Olivia Chen, à 28 ans, était une architecte innovante de UC Berkeley qui a transformé le paysage du village avec ses conceptions durables et époustouflantes. Le théâtre local était honoré par les symphonies enchanteresses d'Ethan Kovacs, un musicien et compositeur de 72 ans formé à Juilliard. Isabella Torres, une chef autodidacte passionnée par les ingrédients locaux, a créé une sensation culinaire avec son restaurant de la ferme à la table, qui est devenu une destination incontournable pour les amateurs de gastronomie. Ces individus remarquables, chacun avec leurs talents distincts, ont contribué à la tapisserie vibrante de la vie à Silvermist Hollow."
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
      system: "Votre tâche consiste à prendre le texte non structuré fourni et à le convertir en un format de tableau bien organisé en utilisant JSON. Identifiez les principales entités, attributs ou catégories mentionnés dans le texte et utilisez-les comme clés dans l'objet JSON. Ensuite, extrayez les informations pertinentes du texte et remplissez les valeurs correspondantes dans l'objet JSON. Assurez-vous que les données sont représentées avec précision et correctement formatées dans la structure JSON. Le tableau JSON résultant devrait fournir un aperçu clair et structuré des informations présentées dans le texte original.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, un charmant village, abritait un groupe extraordinaire d'individus. Parmi eux se trouvait le Dr Liam Patel, un neurochirurgien de 45 ans formé à Yale qui a révolutionné les techniques chirurgicales au centre médical régional. Olivia Chen, à 28 ans, était une architecte innovante de UC Berkeley qui a transformé le paysage du village avec ses conceptions durables et époustouflantes. Le théâtre local était honoré par les symphonies enchanteresses d'Ethan Kovacs, un musicien et compositeur de 72 ans formé à Juilliard. Isabella Torres, une chef autodidacte passionnée par les ingrédients locaux, a créé une sensation culinaire avec son restaurant de la ferme à la table, qui est devenu une destination incontournable pour les amateurs de gastronomie. Ces individus remarquables, chacun avec leurs talents distincts, ont contribué à la tapisserie vibrante de la vie à Silvermist Hollow."
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
        system="Votre tâche consiste à prendre le texte non structuré fourni et à le convertir en un format de tableau bien organisé en utilisant JSON. Identifiez les principales entités, attributs ou catégories mentionnés dans le texte et utilisez-les comme clés dans l'objet JSON. Ensuite, extrayez les informations pertinentes du texte et remplissez les valeurs correspondantes dans l'objet JSON. Assurez-vous que les données sont représentées avec précision et correctement formatées dans la structure JSON. Le tableau JSON résultant devrait fournir un aperçu clair et structuré des informations présentées dans le texte original.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, un charmant village, abritait un groupe extraordinaire d'individus. Parmi eux se trouvait le Dr Liam Patel, un neurochirurgien de 45 ans formé à Yale qui a révolutionné les techniques chirurgicales au centre médical régional. Olivia Chen, à 28 ans, était une architecte innovante de UC Berkeley qui a transformé le paysage du village avec ses conceptions durables et époustouflantes. Le théâtre local était honoré par les symphonies enchanteresses d'Ethan Kovacs, un musicien et compositeur de 72 ans formé à Juilliard. Isabella Torres, une chef autodidacte passionnée par les ingrédients locaux, a créé une sensation culinaire avec son restaurant de la ferme à la table, qui est devenu une destination incontournable pour les amateurs de gastronomie. Ces individus remarquables, chacun avec leurs talents distincts, ont contribué à la tapisserie vibrante de la vie à Silvermist Hollow."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI Type
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 0,
      system: "Votre tâche consiste à prendre le texte non structuré fourni et à le convertir en un format de tableau bien organisé en utilisant JSON. Identifiez les principales entités, attributs ou catégories mentionnés dans le texte et utilisez-les comme clés dans l'objet JSON. Ensuite, extrayez les informations pertinentes du texte et remplissez les valeurs correspondantes dans l'objet JSON. Assurez-vous que les données sont représentées avec précision et correctement formatées dans la structure JSON. Le tableau JSON résultant devrait fournir un aperçu clair et structuré des informations présentées dans le texte original.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, un charmant village, abritait un groupe extraordinaire d'individus. Parmi eux se trouvait le Dr Liam Patel, un neurochirurgien de 45 ans formé à Yale qui a révolutionné les techniques chirurgicales au centre médical régional. Olivia Chen, à 28 ans, était une architecte innovante de UC Berkeley qui a transformé le paysage du village avec ses conceptions durables et époustouflantes. Le théâtre local était honoré par les symphonies enchanteresses d'Ethan Kovacs, un musicien et compositeur de 72 ans formé à Juilliard. Isabella Torres, une chef autodidacte passionnée par les ingrédients locaux, a créé une sensation culinaire avec son restaurant de la ferme à la table, qui est devenu une destination incontournable pour les amateurs de gastronomie. Ces individus remarquables, chacun avec leurs talents distincts, ont contribué à la tapisserie vibrante de la vie à Silvermist Hollow."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>