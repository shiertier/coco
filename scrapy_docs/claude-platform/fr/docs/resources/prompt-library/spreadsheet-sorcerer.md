# Sorcier de feuilles de calcul

Générer des feuilles de calcul CSV avec différents types de données.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est de générer une feuille de calcul CSV contenant le type de données spécifié. La feuille de calcul doit être bien organisée, avec des en-têtes de colonnes clairs et des types de données appropriés pour chaque colonne. Assurez-vous que les données sont réalistes, diversifiées et formatées de manière cohérente. Incluez un minimum de 10 lignes de données, sans compter la ligne d'en-tête. |
| User   | Créez une feuille de calcul contenant des informations sur les livres d'une bibliothèque. Incluez les colonnes suivantes : ID du livre, Titre, Auteur, Genre, Année de publication et Exemplaires disponibles. Assurez-vous que les données couvrent une variété de genres et d'années de publication, et que le nombre d'exemplaires disponibles varie entre les livres.                             |

### Exemple de sortie

> Voici une feuille de calcul CSV contenant des informations sur les livres d'une bibliothèque, basée sur vos exigences :
>
> Book ID,Title,Author,Genre,Publication Year,Available Copies  
> 1,To Kill a Mockingbird,Harper Lee,Classic,1960,5  
> 2,1984,George Orwell,Dystopian,1949,3  
> 3,Pride and Prejudice,Jane Austen,Romance,1813,7  
> 4,The Catcher in the Rye,J.D. Salinger,Coming-of-age,1951,2  
> 5,The Hitchhiker's Guide to the Galaxy,Douglas Adams,Science Fiction,1979,4  
> 6,The Da Vinci Code,Dan Brown,Mystery,2003,6  
> 7,The Kite Runner,Khaled Hosseini,Historical Fiction,2003,3  
> 8,The Alchemist,Paulo Coelho,Philosophical Fiction,1988,5  
> 9,The Hunger Games,Suzanne Collins,Young Adult,2008,8  
> 10,The Girl with the Dragon Tattoo,Stieg Larsson,Crime,2005,4  
> 11,The Hobbit,J.R.R. Tolkien,Fantasy,1937,6  
> 12,The Diary of a Young Girl,Anne Frank,Autobiography,1947,2  
> 13,The Fault in Our Stars,John Green,Young Adult,2012,5  
> 14,The Catcher in the Rye,J.D. Salinger,Coming-of-age,1951,3  
> 15,The Curious Incident of the Dog in the Night-Time,Mark Haddon,Mystery,2003,4

---

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
        max_tokens=1000,
        temperature=0,
        system="Votre tâche est de générer une feuille de calcul CSV contenant le type de données spécifié. La feuille de calcul doit être bien organisée, avec des en-têtes de colonnes clairs et des types de données appropriés pour chaque colonne. Assurez-vous que les données sont réalistes, diversifiées et formatées de manière cohérente. Incluez un minimum de 10 lignes de données, sans compter la ligne d'en-tête.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Créez une feuille de calcul contenant des informations sur les livres d'une bibliothèque. Incluez les colonnes suivantes : ID du livre, Titre, Auteur, Genre, Année de publication et Exemplaires disponibles. Assurez-vous que les données couvrent une variété de genres et d'années de publication, et que le nombre d'exemplaires disponibles varie entre les livres."
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
      max_tokens: 1000,
      temperature: 0,
      system: "Votre tâche est de générer une feuille de calcul CSV contenant le type de données spécifié. La feuille de calcul doit être bien organisée, avec des en-têtes de colonnes clairs et des types de données appropriés pour chaque colonne. Assurez-vous que les données sont réalistes, diversifiées et formatées de manière cohérente. Incluez un minimum de 10 lignes de données, sans compter la ligne d'en-tête.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Créez une feuille de calcul contenant des informations sur les livres d'une bibliothèque. Incluez les colonnes suivantes : ID du livre, Titre, Auteur, Genre, Année de publication et Exemplaires disponibles. Assurez-vous que les données couvrent une variété de genres et d'années de publication, et que le nombre d'exemplaires disponibles varie entre les livres."
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
        max_tokens=1000,
        temperature=0,
        system="Votre tâche est de générer une feuille de calcul CSV contenant le type de données spécifié. La feuille de calcul doit être bien organisée, avec des en-têtes de colonnes clairs et des types de données appropriés pour chaque colonne. Assurez-vous que les données sont réalistes, diversifiées et formatées de manière cohérente. Incluez un minimum de 10 lignes de données, sans compter la ligne d'en-tête.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Créez une feuille de calcul contenant des informations sur les livres d'une bibliothèque. Incluez les colonnes suivantes : ID du livre, Titre, Auteur, Genre, Année de publication et Exemplaires disponibles. Assurez-vous que les données couvrent une variété de genres et d'années de publication, et que le nombre d'exemplaires disponibles varie entre les livres."
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
      max_tokens: 1000,
      temperature: 0,
      system: "Votre tâche est de générer une feuille de calcul CSV contenant le type de données spécifié. La feuille de calcul doit être bien organisée, avec des en-têtes de colonnes clairs et des types de données appropriés pour chaque colonne. Assurez-vous que les données sont réalistes, diversifiées et formatées de manière cohérente. Incluez un minimum de 10 lignes de données, sans compter la ligne d'en-tête.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Créez une feuille de calcul contenant des informations sur les livres d'une bibliothèque. Incluez les colonnes suivantes : ID du livre, Titre, Auteur, Genre, Année de publication et Exemplaires disponibles. Assurez-vous que les données couvrent une variété de genres et d'années de publication, et que le nombre d'exemplaires disponibles varie entre les livres."
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
        system="Votre tâche est de générer une feuille de calcul CSV contenant le type de données spécifié. La feuille de calcul doit être bien organisée, avec des en-têtes de colonnes clairs et des types de données appropriés pour chaque colonne. Assurez-vous que les données sont réalistes, diversifiées et formatées de manière cohérente. Incluez un minimum de 10 lignes de données, sans compter la ligne d'en-tête.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Créez une feuille de calcul contenant des informations sur les livres d'une bibliothèque. Incluez les colonnes suivantes : ID du livre, Titre, Auteur, Genre, Année de publication et Exemplaires disponibles. Assurez-vous que les données couvrent une variété de genres et d'années de publication, et que le nombre d'exemplaires disponibles varie entre les livres."
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
      max_tokens: 1000,
      temperature: 0,
      system: "Votre tâche est de générer une feuille de calcul CSV contenant le type de données spécifié. La feuille de calcul doit être bien organisée, avec des en-têtes de colonnes clairs et des types de données appropriés pour chaque colonne. Assurez-vous que les données sont réalistes, diversifiées et formatées de manière cohérente. Incluez un minimum de 10 lignes de données, sans compter la ligne d'en-tête.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Créez une feuille de calcul contenant des informations sur les livres d'une bibliothèque. Incluez les colonnes suivantes : ID du livre, Titre, Auteur, Genre, Année de publication et Exemplaires disponibles. Assurez-vous que les données couvrent une variété de genres et d'années de publication, et que le nombre d'exemplaires disponibles varie entre les livres."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>