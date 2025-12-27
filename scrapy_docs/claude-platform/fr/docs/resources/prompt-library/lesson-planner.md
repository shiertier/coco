# Planificateur de cours

Créez des plans de cours approfondis sur n'importe quel sujet.

---

> Copiez cette invite dans notre [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est de créer un plan de cours complet, engageant et bien structuré sur le sujet donné. Le plan de cours doit être conçu pour une session de classe de 60 minutes et doit s'adresser à un niveau scolaire ou groupe d'âge spécifique. Commencez par énoncer les objectifs de la leçon, qui doivent être clairs, mesurables et alignés sur les normes éducatives pertinentes. Ensuite, fournissez un aperçu détaillé de la leçon, en la décomposant en une introduction, des activités principales et une conclusion. Pour chaque section, décrivez les méthodes d'enseignement, les activités d'apprentissage et les ressources que vous utiliserez pour transmettre efficacement le contenu et engager les étudiants. Enfin, décrivez les méthodes d'évaluation que vous emploierez pour évaluer la compréhension et la maîtrise des objectifs de la leçon par les étudiants. Le plan de cours doit être bien organisé, facile à suivre et promouvoir l'apprentissage actif et la pensée critique. |
| User   | Sujet : Introduction à la photosynthèse Niveau scolaire : 7e année (Âges 12-13) |

## Exemple de sortie

> Plan de cours : Introduction à la photosynthèse
> Niveau scolaire : 7e année (Âges 12-13)
> Durée : 60 minutes
>
> Objectifs de la leçon :
> À la fin de la leçon, les étudiants seront capables de :
>
> 1. Définir la photosynthèse et expliquer son importance dans l'écosystème.
> 2. Identifier les composants clés nécessaires pour que la photosynthèse se produise.
> 3. Décrire le processus de photosynthèse, y compris les rôles de l'énergie lumineuse, du dioxyde de carbone et de l'eau.
> 4. Reconnaître les produits de la photosynthèse et leur importance pour les organismes vivants.
>    Plan de la leçon :
>
> 5. Introduction (10 minutes)
>    i. Engagez les étudiants en montrant un court clip vidéo ou des images de plantes et en discutant de leur rôle dans l'environnement.
>    ii. Introduisez le concept de photosynthèse et son importance pour maintenir la vie sur Terre.
>    iii. Énoncez les objectifs de la leçon et fournissez un aperçu de la structure de la leçon.
> 6. Activités principales (40 minutes)
>    i. Instruction guidée (15 minutes)
>    a. Expliquez les composants clés nécessaires pour la photosynthèse : la lumière du soleil, le dioxyde de carbone, l'eau et la chlorophylle.
>    b. Décrivez le processus de photosynthèse, en mettant l'accent sur les rôles de l'énergie lumineuse, du dioxyde de carbone et de l'eau.
>    c. Discutez des produits de la photosynthèse (glucose et oxygène) et de leur importance pour les organismes vivants.
>    ii. Activité en petits groupes (15 minutes)
>    a. Divisez la classe en petits groupes et fournissez à chaque groupe une feuille de travail ou un organisateur graphique.
>    b. Demandez aux étudiants de travailler ensemble pour créer une représentation visuelle du processus de photosynthèse, en étiquetant les composants clés et les produits.
>    c. Encouragez les étudiants à discuter et expliquer leur compréhension de la photosynthèse au sein de leurs groupes.
>    iii. Discussion de classe (10 minutes)
>    a. Invitez chaque groupe à partager sa représentation visuelle avec la classe.
>    b. Facilitez une discussion de classe, encourageant les étudiants à poser des questions, fournir des commentaires et clarifier toute idée fausse.
> 7. Conclusion (10 minutes)
>    i. Résumez les points clés de la leçon, renforçant la définition, les composants, le processus et les produits de la photosynthèse.
>    ii. Administrez un court quiz ou un ticket de sortie pour évaluer la compréhension des objectifs de la leçon par les étudiants.
>    iii. Fournissez aux étudiants un devoir à domicile pour explorer davantage l'importance de la photosynthèse dans différents écosystèmes.
>    Méthodes d'évaluation :
>
> 8. Évaluation formative : Observez la participation et la compréhension des étudiants pendant l'instruction guidée et l'activité en petits groupes.
> 9. Évaluation sommative : Évaluez les représentations visuelles des étudiants et leur performance sur le court quiz ou le ticket de sortie.
> 10. Devoir à domicile : Examinez les soumissions de devoirs des étudiants pour évaluer leur capacité à appliquer les concepts appris dans la leçon.
>     Ressources :
>
> 11. Clip vidéo ou images de plantes et de leur rôle dans l'environnement
> 12. Feuilles de travail ou organisateurs graphiques pour l'activité en petits groupes
> 13. Court quiz ou ticket de sortie pour l'évaluation
> 14. Instructions de devoir à domicile

### Requête API

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # par défaut os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=4000,
        temperature=0.5,
        system="Votre tâche est de créer un plan de cours complet, engageant et bien structuré sur le sujet donné. Le plan de cours doit être conçu pour une session de classe de 60 minutes et doit s'adresser à un niveau scolaire ou groupe d'âge spécifique. Commencez par énoncer les objectifs de la leçon, qui doivent être clairs, mesurables et alignés sur les normes éducatives pertinentes. Ensuite, fournissez un aperçu détaillé de la leçon, en la décomposant en une introduction, des activités principales et une conclusion. Pour chaque section, décrivez les méthodes d'enseignement, les activités d'apprentissage et les ressources que vous utiliserez pour transmettre efficacement le contenu et engager les étudiants. Enfin, décrivez les méthodes d'évaluation que vous emploierez pour évaluer la compréhension et la maîtrise des objectifs de la leçon par les étudiants. Le plan de cours doit être bien organisé, facile à suivre et promouvoir l'apprentissage actif et la pensée critique.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Sujet : Introduction à la photosynthèse  \nNiveau scolaire : 7e année (Âges 12-13)"
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
      apiKey: "my_api_key", // par défaut process.env["ANTHROPIC_API_KEY"]
    });
    
    const msg = await anthropic.messages.create({
      model: "claude-sonnet-4-5",
      max_tokens: 4000,
      temperature: 0.5,
      system: "Votre tâche est de créer un plan de cours complet, engageant et bien structuré sur le sujet donné. Le plan de cours doit être conçu pour une session de classe de 60 minutes et doit s'adresser à un niveau scolaire ou groupe d'âge spécifique. Commencez par énoncer les objectifs de la leçon, qui doivent être clairs, mesurables et alignés sur les normes éducatives pertinentes. Ensuite, fournissez un aperçu détaillé de la leçon, en la décomposant en une introduction, des activités principales et une conclusion. Pour chaque section, décrivez les méthodes d'enseignement, les activités d'apprentissage et les ressources que vous utiliserez pour transmettre efficacement le contenu et engager les étudiants. Enfin, décrivez les méthodes d'évaluation que vous emploierez pour évaluer la compréhension et la maîtrise des objectifs de la leçon par les étudiants. Le plan de cours doit être bien organisé, facile à suivre et promouvoir l'apprentissage actif et la pensée critique.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Sujet : Introduction à la photosynthèse  \nNiveau scolaire : 7e année (Âges 12-13)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
    
    ```python AWS Bedrock Python
    from anthropic import AnthropicBedrock
    
    # Voir https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # pour les options d'authentification
    client = AnthropicBedrock()
    
    message = client.messages.create(
        model="anthropic.claude-sonnet-4-5-20250929-v1:0",
        max_tokens=4000,
        temperature=0.5,
        system="Votre tâche est de créer un plan de cours complet, engageant et bien structuré sur le sujet donné. Le plan de cours doit être conçu pour une session de classe de 60 minutes et doit s'adresser à un niveau scolaire ou groupe d'âge spécifique. Commencez par énoncer les objectifs de la leçon, qui doivent être clairs, mesurables et alignés sur les normes éducatives pertinentes. Ensuite, fournissez un aperçu détaillé de la leçon, en la décomposant en une introduction, des activités principales et une conclusion. Pour chaque section, décrivez les méthodes d'enseignement, les activités d'apprentissage et les ressources que vous utiliserez pour transmettre efficacement le contenu et engager les étudiants. Enfin, décrivez les méthodes d'évaluation que vous emploierez pour évaluer la compréhension et la maîtrise des objectifs de la leçon par les étudiants. Le plan de cours doit être bien organisé, facile à suivre et promouvoir l'apprentissage actif et la pensée critique.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Sujet : Introduction à la photosynthèse  \nNiveau scolaire : 7e année (Âges 12-13)"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    // Voir https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    // pour les options d'authentification
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 4000,
      temperature: 0.5,
      system: "Votre tâche est de créer un plan de cours complet, engageant et bien structuré sur le sujet donné. Le plan de cours doit être conçu pour une session de classe de 60 minutes et doit s'adresser à un niveau scolaire ou groupe d'âge spécifique. Commencez par énoncer les objectifs de la leçon, qui doivent être clairs, mesurables et alignés sur les normes éducatives pertinentes. Ensuite, fournissez un aperçu détaillé de la leçon, en la décomposant en une introduction, des activités principales et une conclusion. Pour chaque section, décrivez les méthodes d'enseignement, les activités d'apprentissage et les ressources que vous utiliserez pour transmettre efficacement le contenu et engager les étudiants. Enfin, décrivez les méthodes d'évaluation que vous emploierez pour évaluer la compréhension et la maîtrise des objectifs de la leçon par les étudiants. Le plan de cours doit être bien organisé, facile à suivre et promouvoir l'apprentissage actif et la pensée critique.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Sujet : Introduction à la photosynthèse  \nNiveau scolaire : 7e année (Âges 12-13)"
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
        max_tokens=4000,
        temperature=0.5,
        system="Votre tâche est de créer un plan de cours complet, engageant et bien structuré sur le sujet donné. Le plan de cours doit être conçu pour une session de classe de 60 minutes et doit s'adresser à un niveau scolaire ou groupe d'âge spécifique. Commencez par énoncer les objectifs de la leçon, qui doivent être clairs, mesurables et alignés sur les normes éducatives pertinentes. Ensuite, fournissez un aperçu détaillé de la leçon, en la décomposant en une introduction, des activités principales et une conclusion. Pour chaque section, décrivez les méthodes d'enseignement, les activités d'apprentissage et les ressources que vous utiliserez pour transmettre efficacement le contenu et engager les étudiants. Enfin, décrivez les méthodes d'évaluation que vous emploierez pour évaluer la compréhension et la maîtrise des objectifs de la leçon par les étudiants. Le plan de cours doit être bien organisé, facile à suivre et promouvoir l'apprentissage actif et la pensée critique.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Sujet : Introduction à la photosynthèse  \nNiveau scolaire : 7e année (Âges 12-13)"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Lit à partir des variables d'environnement `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID`.
    // Passe également par le flux standard `google-auth-library`.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 4000,
      temperature: 0.5,
      system: "Votre tâche est de créer un plan de cours complet, engageant et bien structuré sur le sujet donné. Le plan de cours doit être conçu pour une session de classe de 60 minutes et doit s'adresser à un niveau scolaire ou groupe d'âge spécifique. Commencez par énoncer les objectifs de la leçon, qui doivent être clairs, mesurables et alignés sur les normes éducatives pertinentes. Ensuite, fournissez un aperçu détaillé de la leçon, en la décomposant en une introduction, des activités principales et une conclusion. Pour chaque section, décrivez les méthodes d'enseignement, les activités d'apprentissage et les ressources que vous utiliserez pour transmettre efficacement le contenu et engager les étudiants. Enfin, décrivez les méthodes d'évaluation que vous emploierez pour évaluer la compréhension et la maîtrise des objectifs de la leçon par les étudiants. Le plan de cours doit être bien organisé, facile à suivre et promouvoir l'apprentissage actif et la pensée critique.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Sujet : Introduction à la photosynthèse  \nNiveau scolaire : 7e année (Âges 12-13)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>