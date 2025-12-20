# Décodeur de directions

Transformez le langage naturel en instructions étape par étape.

---

> Copiez cette invite dans notre [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche consiste à prendre la description en langage naturel fournie d'un processus ou d'une tâche et à la transformer en instructions claires, concises et étape par étape qui sont logiques, séquentielles et faciles à suivre. Utilisez un langage impératif et commencez chaque étape par un verbe d'action. Fournissez les détails et explications nécessaires pour vous assurer que le lecteur peut accomplir la tâche avec succès. Si la description originale n'est pas claire, ambiguë ou manque d'informations suffisantes, demandez des clarifications ou des détails supplémentaires. |
| User   | Pour faire une tasse de thé, commencez par faire bouillir de l'eau dans une bouilloire. Pendant que l'eau chauffe, prenez une tasse et mettez-y un sachet de thé. Une fois que l'eau bout, versez-la soigneusement dans la tasse, sur le sachet de thé. Laissez le thé infuser pendant quelques minutes, selon la force que vous aimez. Après l'infusion, retirez le sachet de thé et ajoutez du lait et du sucre selon votre goût. Mélangez le tout et savourez votre tasse de thé chaud. |

### Exemple de sortie

> 1. Remplissez une bouilloire d'eau et placez-la sur la cuisinière pour la faire bouillir.
> 2. En attendant que l'eau bouille, sélectionnez une tasse et placez un sachet de thé à l'intérieur.
> 3. Une fois que l'eau atteint l'ébullition, versez soigneusement l'eau chaude dans la tasse, sur le sachet de thé.
> 4. Laissez le thé infuser dans l'eau chaude pendant la durée souhaitée, généralement 2 à 5 minutes, selon la force préférée.
> 5. Une fois le temps d'infusion écoulé, retirez le sachet de thé de la tasse et jetez-le.
> 6. Ajoutez du lait et du sucre au thé, selon vos préférences gustatives.
> 7. À l'aide d'une cuillère, mélangez le thé, le lait et le sucre ensemble jusqu'à ce qu'ils soient bien combinés.
> 8. Votre tasse de thé est maintenant prête à être savourée. Buvez-la pendant qu'elle est encore chaude.

---

## Requête API

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
        system="Votre tâche consiste à prendre la description en langage naturel fournie d'un processus ou d'une tâche et à la transformer en instructions claires, concises et étape par étape qui sont logiques, séquentielles et faciles à suivre. Utilisez un langage impératif et commencez chaque étape par un verbe d'action. Fournissez les détails et explications nécessaires pour vous assurer que le lecteur peut accomplir la tâche avec succès. Si la description originale n'est pas claire, ambiguë ou manque d'informations suffisantes, demandez des clarifications ou des détails supplémentaires.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Pour faire une tasse de thé, commencez par faire bouillir de l'eau dans une bouilloire. Pendant que l'eau chauffe, prenez une tasse et mettez-y un sachet de thé. Une fois que l'eau bout, versez-la soigneusement dans la tasse, sur le sachet de thé. Laissez le thé infuser pendant quelques minutes, selon la force que vous aimez. Après l'infusion, retirez le sachet de thé et ajoutez du lait et du sucre selon votre goût. Mélangez le tout et savourez votre tasse de thé chaud."
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
      system: "Votre tâche consiste à prendre la description en langage naturel fournie d'un processus ou d'une tâche et à la transformer en instructions claires, concises et étape par étape qui sont logiques, séquentielles et faciles à suivre. Utilisez un langage impératif et commencez chaque étape par un verbe d'action. Fournissez les détails et explications nécessaires pour vous assurer que le lecteur peut accomplir la tâche avec succès. Si la description originale n'est pas claire, ambiguë ou manque d'informations suffisantes, demandez des clarifications ou des détails supplémentaires.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Pour faire une tasse de thé, commencez par faire bouillir de l'eau dans une bouilloire. Pendant que l'eau chauffe, prenez une tasse et mettez-y un sachet de thé. Une fois que l'eau bout, versez-la soigneusement dans la tasse, sur le sachet de thé. Laissez le thé infuser pendant quelques minutes, selon la force que vous aimez. Après l'infusion, retirez le sachet de thé et ajoutez du lait et du sucre selon votre goût. Mélangez le tout et savourez votre tasse de thé chaud."
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
        system="Votre tâche consiste à prendre la description en langage naturel fournie d'un processus ou d'une tâche et à la transformer en instructions claires, concises et étape par étape qui sont logiques, séquentielles et faciles à suivre. Utilisez un langage impératif et commencez chaque étape par un verbe d'action. Fournissez les détails et explications nécessaires pour vous assurer que le lecteur peut accomplir la tâche avec succès. Si la description originale n'est pas claire, ambiguë ou manque d'informations suffisantes, demandez des clarifications ou des détails supplémentaires.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Pour faire une tasse de thé, commencez par faire bouillir de l'eau dans une bouilloire. Pendant que l'eau chauffe, prenez une tasse et mettez-y un sachet de thé. Une fois que l'eau bout, versez-la soigneusement dans la tasse, sur le sachet de thé. Laissez le thé infuser pendant quelques minutes, selon la force que vous aimez. Après l'infusion, retirez le sachet de thé et ajoutez du lait et du sucre selon votre goût. Mélangez le tout et savourez votre tasse de thé chaud."
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
      system: "Votre tâche consiste à prendre la description en langage naturel fournie d'un processus ou d'une tâche et à la transformer en instructions claires, concises et étape par étape qui sont logiques, séquentielles et faciles à suivre. Utilisez un langage impératif et commencez chaque étape par un verbe d'action. Fournissez les détails et explications nécessaires pour vous assurer que le lecteur peut accomplir la tâche avec succès. Si la description originale n'est pas claire, ambiguë ou manque d'informations suffisantes, demandez des clarifications ou des détails supplémentaires.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Pour faire une tasse de thé, commencez par faire bouillir de l'eau dans une bouilloire. Pendant que l'eau chauffe, prenez une tasse et mettez-y un sachet de thé. Une fois que l'eau bout, versez-la soigneusement dans la tasse, sur le sachet de thé. Laissez le thé infuser pendant quelques minutes, selon la force que vous aimez. Après l'infusion, retirez le sachet de thé et ajoutez du lait et du sucre selon votre goût. Mélangez le tout et savourez votre tasse de thé chaud."
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
        system="Votre tâche consiste à prendre la description en langage naturel fournie d'un processus ou d'une tâche et à la transformer en instructions claires, concises et étape par étape qui sont logiques, séquentielles et faciles à suivre. Utilisez un langage impératif et commencez chaque étape par un verbe d'action. Fournissez les détails et explications nécessaires pour vous assurer que le lecteur peut accomplir la tâche avec succès. Si la description originale n'est pas claire, ambiguë ou manque d'informations suffisantes, demandez des clarifications ou des détails supplémentaires.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Pour faire une tasse de thé, commencez par faire bouillir de l'eau dans une bouilloire. Pendant que l'eau chauffe, prenez une tasse et mettez-y un sachet de thé. Une fois que l'eau bout, versez-la soigneusement dans la tasse, sur le sachet de thé. Laissez le thé infuser pendant quelques minutes, selon la force que vous aimez. Après l'infusion, retirez le sachet de thé et ajoutez du lait et du sucre selon votre goût. Mélangez le tout et savourez votre tasse de thé chaud."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Lit à partir des variables d'environnement `CLOUD_ML_REGION` et `ANTHROPIC_VERTEX_PROJECT_ID`.
    // Passe également par le flux standard `google-auth-library`.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 0,
      system: "Votre tâche consiste à prendre la description en langage naturel fournie d'un processus ou d'une tâche et à la transformer en instructions claires, concises et étape par étape qui sont logiques, séquentielles et faciles à suivre. Utilisez un langage impératif et commencez chaque étape par un verbe d'action. Fournissez les détails et explications nécessaires pour vous assurer que le lecteur peut accomplir la tâche avec succès. Si la description originale n'est pas claire, ambiguë ou manque d'informations suffisantes, demandez des clarifications ou des détails supplémentaires.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Pour faire une tasse de thé, commencez par faire bouillir de l'eau dans une bouilloire. Pendant que l'eau chauffe, prenez une tasse et mettez-y un sachet de thé. Une fois que l'eau bout, versez-la soigneusement dans la tasse, sur le sachet de thé. Laissez le thé infuser pendant quelques minutes, selon la force que vous aimez. Après l'infusion, retirez le sachet de thé et ajoutez du lait et du sucre selon votre goût. Mélangez le tout et savourez votre tasse de thé chaud."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>