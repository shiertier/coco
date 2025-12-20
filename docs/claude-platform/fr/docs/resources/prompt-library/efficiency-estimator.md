# Estimateur d'efficacité

Calculez la complexité temporelle des fonctions et algorithmes.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| Système | Votre tâche est d'analyser la fonction ou l'algorithme fourni et de calculer sa complexité temporelle en utilisant la notation Big O. Expliquez votre raisonnement étape par étape, en décrivant comment vous êtes arrivé à la complexité temporelle finale. Considérez le scénario du pire cas lors de la détermination de la complexité temporelle. Si la fonction ou l'algorithme contient plusieurs étapes ou boucles imbriquées, fournissez la complexité temporelle pour chaque étape puis donnez la complexité temporelle globale pour l'ensemble de la fonction ou de l'algorithme. Supposez que toutes les fonctions ou opérations intégrées utilisées ont une complexité temporelle de O(1) sauf indication contraire. |
| Utilisateur   | def example_function(n): for i in range(n): print(i) for j in range(n): for k in range(n): print(j, k) |

### Exemple de sortie

> Pour analyser la complexité temporelle de la fonction donnée `example_function(n)`, nous allons la décomposer en étapes et considérer le scénario du pire cas.
>
> Étape 1 : Analyse de la première boucle
>
> ```
> for i in range(n):
>    print(i)
>
> ```

> Cette boucle itère `n` fois et effectue une opération à temps constant (`print(i)`) à chaque itération. Par conséquent, la complexité temporelle de cette étape est O(n).
>
> Étape 2 : Analyse des boucles imbriquées
>
> ```
> for j in range(n):
>     for k in range(n):
>         print(j, k)
>
> ```
>
> La boucle externe itère `n` fois, et pour chaque itération, la boucle interne itère également `n` fois. À chaque itération de la boucle interne, une opération à temps constant (`print(j, k)`) est effectuée. Par conséquent, la complexité temporelle des boucles imbriquées est O(n \* n) = O(n^2).
>
> Étape 3 : Combinaison des complexités temporelles  
> La fonction `example_function(n)` consiste en deux étapes : la première boucle et les boucles imbriquées. La complexité temporelle globale est la somme des complexités temporelles de chaque étape.
>
> Complexité temporelle de la première boucle : O(n)  
> Complexité temporelle des boucles imbriquées : O(n^2)
>
> Pour déterminer la complexité temporelle globale, nous considérons le terme dominant, qui est le terme avec l'ordre de croissance le plus élevé. Dans ce cas, O(n^2) domine O(n).
>
> Par conséquent, la complexité temporelle globale de la fonction `example_function(n)` est O(n^2).
>
> Dans le scénario du pire cas, lorsque `n` est grand, les boucles imbriquées domineront le temps d'exécution, et la fonction aura une complexité temporelle quadratique.

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
        system="Votre tâche est d'analyser la fonction ou l'algorithme fourni et de calculer sa complexité temporelle en utilisant la notation Big O. Expliquez votre raisonnement étape par étape, en décrivant comment vous êtes arrivé à la complexité temporelle finale. Considérez le scénario du pire cas lors de la détermination de la complexité temporelle. Si la fonction ou l'algorithme contient plusieurs étapes ou boucles imbriquées, fournissez la complexité temporelle pour chaque étape puis donnez la complexité temporelle globale pour l'ensemble de la fonction ou de l'algorithme. Supposez que toutes les fonctions ou opérations intégrées utilisées ont une complexité temporelle de O(1) sauf indication contraire.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
      system: "Votre tâche est d'analyser la fonction ou l'algorithme fourni et de calculer sa complexité temporelle en utilisant la notation Big O. Expliquez votre raisonnement étape par étape, en décrivant comment vous êtes arrivé à la complexité temporelle finale. Considérez le scénario du pire cas lors de la détermination de la complexité temporelle. Si la fonction ou l'algorithme contient plusieurs étapes ou boucles imbriquées, fournissez la complexité temporelle pour chaque étape puis donnez la complexité temporelle globale pour l'ensemble de la fonction ou de l'algorithme. Supposez que toutes les fonctions ou opérations intégrées utilisées ont une complexité temporelle de O(1) sauf indication contraire.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
        system="Votre tâche est d'analyser la fonction ou l'algorithme fourni et de calculer sa complexité temporelle en utilisant la notation Big O. Expliquez votre raisonnement étape par étape, en décrivant comment vous êtes arrivé à la complexité temporelle finale. Considérez le scénario du pire cas lors de la détermination de la complexité temporelle. Si la fonction ou l'algorithme contient plusieurs étapes ou boucles imbriquées, fournissez la complexité temporelle pour chaque étape puis donnez la complexité temporelle globale pour l'ensemble de la fonction ou de l'algorithme. Supposez que toutes les fonctions ou opérations intégrées utilisées ont une complexité temporelle de O(1) sauf indication contraire.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
      system: "Votre tâche est d'analyser la fonction ou l'algorithme fourni et de calculer sa complexité temporelle en utilisant la notation Big O. Expliquez votre raisonnement étape par étape, en décrivant comment vous êtes arrivé à la complexité temporelle finale. Considérez le scénario du pire cas lors de la détermination de la complexité temporelle. Si la fonction ou l'algorithme contient plusieurs étapes ou boucles imbriquées, fournissez la complexité temporelle pour chaque étape puis donnez la complexité temporelle globale pour l'ensemble de la fonction ou de l'algorithme. Supposez que toutes les fonctions ou opérations intégrées utilisées ont une complexité temporelle de O(1) sauf indication contraire.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
        system="Votre tâche est d'analyser la fonction ou l'algorithme fourni et de calculer sa complexité temporelle en utilisant la notation Big O. Expliquez votre raisonnement étape par étape, en décrivant comment vous êtes arrivé à la complexité temporelle finale. Considérez le scénario du pire cas lors de la détermination de la complexité temporelle. Si la fonction ou l'algorithme contient plusieurs étapes ou boucles imbriquées, fournissez la complexité temporelle pour chaque étape puis donnez la complexité temporelle globale pour l'ensemble de la fonction ou de l'algorithme. Supposez que toutes les fonctions ou opérations intégrées utilisées ont une complexité temporelle de O(1) sauf indication contraire.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Lit depuis les variables d'environnement `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID`.
    // Passe également par le flux standard `google-auth-library`.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      system: "Votre tâche est d'analyser la fonction ou l'algorithme fourni et de calculer sa complexité temporelle en utilisant la notation Big O. Expliquez votre raisonnement étape par étape, en décrivant comment vous êtes arrivé à la complexité temporelle finale. Considérez le scénario du pire cas lors de la détermination de la complexité temporelle. Si la fonction ou l'algorithme contient plusieurs étapes ou boucles imbriquées, fournissez la complexité temporelle pour chaque étape puis donnez la complexité temporelle globale pour l'ensemble de la fonction ou de l'algorithme. Supposez que toutes les fonctions ou opérations intégrées utilisées ont une complexité temporelle de O(1) sauf indication contraire.",
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>