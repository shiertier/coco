# Créateur de marque

Créez un brief de design pour une identité de marque holistique.

---

> Copiez cette invite dans notre [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est de créer un brief de design complet pour une identité de marque holistique basée sur les spécifications données. L'identité de marque doit englober divers éléments tels que des suggestions pour le nom de marque, le logo, la palette de couleurs, la typographie, le style visuel, le ton de voix et la personnalité globale de la marque. Assurez-vous que tous les éléments fonctionnent ensemble harmonieusement pour créer une expérience de marque cohérente et mémorable qui communique efficacement les valeurs, la mission et la proposition de vente unique de la marque à son public cible. Soyez détaillé et complet et fournissez suffisamment de détails spécifiques pour que quelqu'un puisse créer une identité de marque vraiment unique. |
| User   | Spécifications de la marque : Il s'agit d'une marque qui se concentre sur la création de vêtements et d'accessoires de haute qualité et élégants en utilisant des matériaux respectueux de l'environnement et des méthodes de production éthiques. La marque cible les consommateurs soucieux de l'environnement âgés de 25 à 40 ans qui valorisent la mode, la durabilité et la responsabilité sociale. L'identité de marque devrait atteindre les objectifs suivants : 1. Refléter l'engagement de la marque envers la durabilité, les pratiques éthiques et la gestion environnementale. 2. Plaire au public cible en transmettant un sentiment de style, de qualité et de tendance. 3. Différencier la marque des concurrents sur le marché de la mode durable. 4. Créer une connexion émotionnelle forte avec les consommateurs et les inspirer à faire des choix plus respectueux de l'environnement. |

### Exemple de sortie

> Brief de Design : Identité de Marque pour une Marque de Mode Éco-Responsable
> Suggestion de Nom de Marque : "Sēnsus" (Latin pour "sens" ou "perception", évoquant un sentiment de pleine conscience et de connexion avec l'environnement)
>
> Logo :
>
> - Un logo minimaliste et moderne qui combine un motif stylisé de feuille ou de plante avec une police de caractères sans-serif élégante.
> - L'élément feuille ou plante symbolise l'engagement de la marque envers la durabilité et la gestion environnementale, tandis que la typographie épurée transmet un sentiment de sophistication et de style.
> - Le logo doit être polyvalent et bien fonctionner dans diverses applications, y compris les étiquettes de vêtements, l'emballage et les plateformes numériques.
>   Palette de Couleurs :
>
> - Une palette de couleurs neutres et terreuses qui reflète l'éthique éco-responsable de la marque, comme des nuances de vert, de beige et de gris chauds.
> - Des couleurs d'accent comme le bleu-vert profond ou le jaune moutarde peuvent être utilisées avec parcimonie pour ajouter de la vivacité et de la profondeur.
> - La palette de couleurs doit évoquer un sentiment d'harmonie et d'équilibre naturels, tout en restant avant-gardiste et attrayante pour le public cible.
>   Typographie :
>
> - Une combinaison d'une police de caractères sans-serif propre et moderne pour les titres et d'une police serif complémentaire pour le corps du texte.
> - La typographie doit être lisible et visuellement attrayante, transmettant un sentiment de qualité et de sophistication.
> - L'utilisation cohérente de la hiérarchie typographique et de l'espacement créera une identité de marque cohérente et organisée.
>   Style Visuel :
>
> - Un style visuel minimaliste et épuré qui met l'accent sur la photographie de produits de haute qualité et des mises en page propres et modernes.
> - Incorporer des éléments naturels, tels que des motifs de plantes ou des textures, pour renforcer les valeurs éco-responsables de la marque et sa connexion avec la nature.
> - Utiliser efficacement l'espace négatif pour créer un sentiment d'ouverture et de respirabilité, reflétant l'engagement de la marque envers la durabilité.
>   Ton de Voix :
>
> - Le ton de voix de la marque doit être accessible, compétent et authentique.
> - Utiliser un langage qui éduque et inspire les consommateurs sur les avantages de la mode durable et des méthodes de production éthiques.
> - Incorporer une touche de sophistication et de style pour plaire aux sensibilités soucieuses de la mode du public cible.
>   Personnalité de Marque :
>
> - Éco-consciente et sensible à l'environnement, tout en étant élégante et avant-gardiste en matière de mode.
> - Confiante et passionnée par la promotion de pratiques durables et de choix éthiques.
> - Inclusive et accueillante, invitant les consommateurs à rejoindre une communauté partageant les mêmes idées et engagée à apporter des changements positifs.
>   Dans l'ensemble, l'identité de marque pour "Sēnsus" devrait trouver un équilibre entre les valeurs éco-responsables et l'esthétique avant-gardiste de la mode, attirant les consommateurs qui veulent avoir l'air élégants tout en faisant des choix environnementalement responsables. Les éléments visuels et verbaux cohérents devraient créer une connexion émotionnelle forte avec le public cible et les inspirer à adopter un mode de vie plus durable.

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
        system="Votre tâche est de créer un brief de design complet pour une identité de marque holistique basée sur les spécifications données. L'identité de marque doit englober divers éléments tels que des suggestions pour le nom de marque, le logo, la palette de couleurs, la typographie, le style visuel, le ton de voix et la personnalité globale de la marque. Assurez-vous que tous les éléments fonctionnent ensemble harmonieusement pour créer une expérience de marque cohérente et mémorable qui communique efficacement les valeurs, la mission et la proposition de vente unique de la marque à son public cible. Soyez détaillé et complet et fournissez suffisamment de détails spécifiques pour que quelqu'un puisse créer une identité de marque vraiment unique.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Spécifications de la marque :\nIl s'agit d'une marque qui se concentre sur la création de vêtements et d'accessoires de haute qualité et élégants en utilisant des matériaux respectueux de l'environnement et des méthodes de production éthiques\nLa marque cible les consommateurs soucieux de l'environnement âgés de 25 à 40 ans qui valorisent la mode, la durabilité et la responsabilité sociale.\nL'identité de marque devrait atteindre les objectifs suivants :\n1. Refléter l'engagement de la marque envers la durabilité, les pratiques éthiques et la gestion environnementale.\n2. Plaire au public cible en transmettant un sentiment de style, de qualité et de tendance.\n3. Différencier la marque des concurrents sur le marché de la mode durable.\n4. Créer une connexion émotionnelle forte avec les consommateurs et les inspirer à faire des choix plus respectueux de l'environnement."
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
      system: "Votre tâche est de créer un brief de design complet pour une identité de marque holistique basée sur les spécifications données. L'identité de marque doit englober divers éléments tels que des suggestions pour le nom de marque, le logo, la palette de couleurs, la typographie, le style visuel, le ton de voix et la personnalité globale de la marque. Assurez-vous que tous les éléments fonctionnent ensemble harmonieusement pour créer une expérience de marque cohérente et mémorable qui communique efficacement les valeurs, la mission et la proposition de vente unique de la marque à son public cible. Soyez détaillé et complet et fournissez suffisamment de détails spécifiques pour que quelqu'un puisse créer une identité de marque vraiment unique.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Spécifications de la marque :\nIl s'agit d'une marque qui se concentre sur la création de vêtements et d'accessoires de haute qualité et élégants en utilisant des matériaux respectueux de l'environnement et des méthodes de production éthiques\nLa marque cible les consommateurs soucieux de l'environnement âgés de 25 à 40 ans qui valorisent la mode, la durabilité et la responsabilité sociale.\nL'identité de marque devrait atteindre les objectifs suivants :\n1. Refléter l'engagement de la marque envers la durabilité, les pratiques éthiques et la gestion environnementale.\n2. Plaire au public cible en transmettant un sentiment de style, de qualité et de tendance.\n3. Différencier la marque des concurrents sur le marché de la mode durable.\n4. Créer une connexion émotionnelle forte avec les consommateurs et les inspirer à faire des choix plus respectueux de l'environnement."
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
        system="Votre tâche est de créer un brief de design complet pour une identité de marque holistique basée sur les spécifications données. L'identité de marque doit englober divers éléments tels que des suggestions pour le nom de marque, le logo, la palette de couleurs, la typographie, le style visuel, le ton de voix et la personnalité globale de la marque. Assurez-vous que tous les éléments fonctionnent ensemble harmonieusement pour créer une expérience de marque cohérente et mémorable qui communique efficacement les valeurs, la mission et la proposition de vente unique de la marque à son public cible. Soyez détaillé et complet et fournissez suffisamment de détails spécifiques pour que quelqu'un puisse créer une identité de marque vraiment unique.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Spécifications de la marque :\nIl s'agit d'une marque qui se concentre sur la création de vêtements et d'accessoires de haute qualité et élégants en utilisant des matériaux respectueux de l'environnement et des méthodes de production éthiques\nLa marque cible les consommateurs soucieux de l'environnement âgés de 25 à 40 ans qui valorisent la mode, la durabilité et la responsabilité sociale.\nL'identité de marque devrait atteindre les objectifs suivants :\n1. Refléter l'engagement de la marque envers la durabilité, les pratiques éthiques et la gestion environnementale.\n2. Plaire au public cible en transmettant un sentiment de style, de qualité et de tendance.\n3. Différencier la marque des concurrents sur le marché de la mode durable.\n4. Créer une connexion émotionnelle forte avec les consommateurs et les inspirer à faire des choix plus respectueux de l'environnement."
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
      system: "Votre tâche est de créer un brief de design complet pour une identité de marque holistique basée sur les spécifications données. L'identité de marque doit englober divers éléments tels que des suggestions pour le nom de marque, le logo, la palette de couleurs, la typographie, le style visuel, le ton de voix et la personnalité globale de la marque. Assurez-vous que tous les éléments fonctionnent ensemble harmonieusement pour créer une expérience de marque cohérente et mémorable qui communique efficacement les valeurs, la mission et la proposition de vente unique de la marque à son public cible. Soyez détaillé et complet et fournissez suffisamment de détails spécifiques pour que quelqu'un puisse créer une identité de marque vraiment unique.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Spécifications de la marque :\nIl s'agit d'une marque qui se concentre sur la création de vêtements et d'accessoires de haute qualité et élégants en utilisant des matériaux respectueux de l'environnement et des méthodes de production éthiques\nLa marque cible les consommateurs soucieux de l'environnement âgés de 25 à 40 ans qui valorisent la mode, la durabilité et la responsabilité sociale.\nL'identité de marque devrait atteindre les objectifs suivants :\n1. Refléter l'engagement de la marque envers la durabilité, les pratiques éthiques et la gestion environnementale.\n2. Plaire au public cible en transmettant un sentiment de style, de qualité et de tendance.\n3. Différencier la marque des concurrents sur le marché de la mode durable.\n4. Créer une connexion émotionnelle forte avec les consommateurs et les inspirer à faire des choix plus respectueux de l'environnement."
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
        system="Votre tâche est de créer un brief de design complet pour une identité de marque holistique basée sur les spécifications données. L'identité de marque doit englober divers éléments tels que des suggestions pour le nom de marque, le logo, la palette de couleurs, la typographie, le style visuel, le ton de voix et la personnalité globale de la marque. Assurez-vous que tous les éléments fonctionnent ensemble harmonieusement pour créer une expérience de marque cohérente et mémorable qui communique efficacement les valeurs, la mission et la proposition de vente unique de la marque à son public cible. Soyez détaillé et complet et fournissez suffisamment de détails spécifiques pour que quelqu'un puisse créer une identité de marque vraiment unique.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Spécifications de la marque :\nIl s'agit d'une marque qui se concentre sur la création de vêtements et d'accessoires de haute qualité et élégants en utilisant des matériaux respectueux de l'environnement et des méthodes de production éthiques\nLa marque cible les consommateurs soucieux de l'environnement âgés de 25 à 40 ans qui valorisent la mode, la durabilité et la responsabilité sociale.\nL'identité de marque devrait atteindre les objectifs suivants :\n1. Refléter l'engagement de la marque envers la durabilité, les pratiques éthiques et la gestion environnementale.\n2. Plaire au public cible en transmettant un sentiment de style, de qualité et de tendance.\n3. Différencier la marque des concurrents sur le marché de la mode durable.\n4. Créer une connexion émotionnelle forte avec les consommateurs et les inspirer à faire des choix plus respectueux de l'environnement."
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
      system: "Votre tâche est de créer un brief de design complet pour une identité de marque holistique basée sur les spécifications données. L'identité de marque doit englober divers éléments tels que des suggestions pour le nom de marque, le logo, la palette de couleurs, la typographie, le style visuel, le ton de voix et la personnalité globale de la marque. Assurez-vous que tous les éléments fonctionnent ensemble harmonieusement pour créer une expérience de marque cohérente et mémorable qui communique efficacement les valeurs, la mission et la proposition de vente unique de la marque à son public cible. Soyez détaillé et complet et fournissez suffisamment de détails spécifiques pour que quelqu'un puisse créer une identité de marque vraiment unique.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Spécifications de la marque :\nIl s'agit d'une marque qui se concentre sur la création de vêtements et d'accessoires de haute qualité et élégants en utilisant des matériaux respectueux de l'environnement et des méthodes de production éthiques\nLa marque cible les consommateurs soucieux de l'environnement âgés de 25 à 40 ans qui valorisent la mode, la durabilité et la responsabilité sociale.\nL'identité de marque devrait atteindre les objectifs suivants :\n1. Refléter l'engagement de la marque envers la durabilité, les pratiques éthiques et la gestion environnementale.\n2. Plaire au public cible en transmettant un sentiment de style, de qualité et de tendance.\n3. Différencier la marque des concurrents sur le marché de la mode durable.\n4. Créer une connexion émotionnelle forte avec les consommateurs et les inspirer à faire des choix plus respectueux de l'environnement."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>