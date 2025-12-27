# Clarificateur de code

Simplifiez et expliquez du code complexe en langage simple.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est de prendre l'extrait de code fourni et de l'expliquer dans un langage simple et facile à comprendre. Décomposez la fonctionnalité, le but et les composants clés du code. Utilisez des analogies, des exemples et des termes simples pour rendre l'explication accessible à quelqu'un ayant des connaissances minimales en programmation. Évitez d'utiliser du jargon technique sauf si c'est absolument nécessaire, et fournissez des explications claires pour tout jargon utilisé. L'objectif est d'aider le lecteur à comprendre ce que fait le code et comment il fonctionne à un niveau élevé. |
| User   | import random def bubble*sort(arr): n = len(arr) for i in range(n-1): for j in range(n-i-1): if arr[j] > arr[j+1]: arr[j], arr[j+1] = arr[j+1], arr[j] return arr numbers = [random.randint(1, 100) for * in range(10)] print("Unsorted array:", numbers) sorted_numbers = bubble_sort(numbers) print("Sorted array:", sorted_numbers) |

### Exemple de sortie

> Ce code démontre l'implémentation de l'algorithme de tri à bulles, qui est utilisé pour trier une liste de nombres par ordre croissant. Décomposons-le étape par étape :
>
> 1.  Le code commence par importer le module 'random', qui sera utilisé pour générer des nombres aléatoires pour l'exemple.
> 2.  La fonction 'bubble_sort' est définie, qui prend un tableau (liste) de nombres en entrée. Cette fonction effectuera l'opération de tri.
> 3.  À l'intérieur de la fonction 'bubble_sort', la variable 'n' se voit attribuer la longueur du tableau d'entrée. Cela sera utilisé pour déterminer le nombre d'itérations nécessaires pour le tri.
> 4.  La première boucle, contrôlée par la variable 'i', itère 'n-1' fois. C'est parce qu'après 'n-1' passages, le plus grand nombre sera à la fin du tableau, donc nous n'avons pas besoin de le comparer à nouveau.
> 5.  La deuxième boucle, contrôlée par la variable 'j', itère 'n-i-1' fois. C'est parce qu'après chaque passage, le plus grand nombre parmi les éléments non triés sera placé à la fin, donc nous n'avons pas besoin de le comparer dans le passage suivant.
> 6.  À l'intérieur de la deuxième boucle, il y a une instruction if qui compare les éléments adjacents dans le tableau. Si l'élément à l'index 'j' est plus grand que l'élément à l'index 'j+1', ils sont échangés. Cela garantit que l'élément le plus grand se déplace vers la fin du tableau.
> 7.  L'échange se fait en utilisant la syntaxe Python 'arr\[j\], arr\[j+1\] = arr\[j+1\], arr\[j\]', qui assigne simultanément les valeurs de 'arr\[j+1\]' à 'arr\[j\]' et 'arr\[j\]' à 'arr\[j+1\]'.
> 8.  Après que toutes les itérations sont terminées, le tableau trié est retourné par la fonction 'bubble_sort'.
> 9.  Dans le code d'exemple, une liste de 10 nombres aléatoires entre 1 et 100 est générée en utilisant une compréhension de liste : '\[random.randint(1, 100) for \_ in range(10)\]'. Le trait de soulignement '\_' est utilisé comme variable de substitution puisque nous n'avons pas besoin des valeurs d'index.
> 10. Le tableau non trié est affiché pour montrer l'ordre initial des nombres.
>
> 11. La fonction 'bubble_sort' est appelée avec le tableau 'numbers' en entrée, et le tableau trié est stocké dans la variable 'sorted_numbers'.
>
> 12. Finalement, le tableau trié est affiché pour montrer les nombres par ordre croissant.
>
> En termes plus simples, imaginez que vous avez une liste de nombres qui sont dans le désordre. L'algorithme de tri à bulles fonctionne en comparant chaque nombre avec celui qui est à côté, et s'ils sont dans le mauvais ordre, il les échange. Ce processus est répété plusieurs fois jusqu'à ce que toute la liste soit triée. C'est comme des bulles qui remontent à la surface, avec les nombres les plus grands qui se déplacent progressivement vers la fin de la liste à chaque passage.

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
        max_tokens=1000,
        temperature=0,
        system="Votre tâche est de prendre l'extrait de code fourni et de l'expliquer dans un langage simple et facile à comprendre. Décomposez la fonctionnalité, le but et les composants clés du code. Utilisez des analogies, des exemples et des termes simples pour rendre l'explication accessible à quelqu'un ayant des connaissances minimales en programmation. Évitez d'utiliser du jargon technique sauf si c'est absolument nécessaire, et fournissez des explications claires pour tout jargon utilisé. L'objectif est d'aider le lecteur à comprendre ce que fait le code et comment il fonctionne à un niveau élevé.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
      system: "Votre tâche est de prendre l'extrait de code fourni et de l'expliquer dans un langage simple et facile à comprendre. Décomposez la fonctionnalité, le but et les composants clés du code. Utilisez des analogies, des exemples et des termes simples pour rendre l'explication accessible à quelqu'un ayant des connaissances minimales en programmation. Évitez d'utiliser du jargon technique sauf si c'est absolument nécessaire, et fournissez des explications claires pour tout jargon utilisé. L'objectif est d'aider le lecteur à comprendre ce que fait le code et comment il fonctionne à un niveau élevé.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
        system="Votre tâche est de prendre l'extrait de code fourni et de l'expliquer dans un langage simple et facile à comprendre. Décomposez la fonctionnalité, le but et les composants clés du code. Utilisez des analogies, des exemples et des termes simples pour rendre l'explication accessible à quelqu'un ayant des connaissances minimales en programmation. Évitez d'utiliser du jargon technique sauf si c'est absolument nécessaire, et fournissez des explications claires pour tout jargon utilisé. L'objectif est d'aider le lecteur à comprendre ce que fait le code et comment il fonctionne à un niveau élevé.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
      system: "Votre tâche est de prendre l'extrait de code fourni et de l'expliquer dans un langage simple et facile à comprendre. Décomposez la fonctionnalité, le but et les composants clés du code. Utilisez des analogies, des exemples et des termes simples pour rendre l'explication accessible à quelqu'un ayant des connaissances minimales en programmation. Évitez d'utiliser du jargon technique sauf si c'est absolument nécessaire, et fournissez des explications claires pour tout jargon utilisé. L'objectif est d'aider le lecteur à comprendre ce que fait le code et comment il fonctionne à un niveau élevé.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
        system="Votre tâche est de prendre l'extrait de code fourni et de l'expliquer dans un langage simple et facile à comprendre. Décomposez la fonctionnalité, le but et les composants clés du code. Utilisez des analogies, des exemples et des termes simples pour rendre l'explication accessible à quelqu'un ayant des connaissances minimales en programmation. Évitez d'utiliser du jargon technique sauf si c'est absolument nécessaire, et fournissez des explications claires pour tout jargon utilisé. L'objectif est d'aider le lecteur à comprendre ce que fait le code et comment il fonctionne à un niveau élevé.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
      system: "Votre tâche est de prendre l'extrait de code fourni et de l'expliquer dans un langage simple et facile à comprendre. Décomposez la fonctionnalité, le but et les composants clés du code. Utilisez des analogies, des exemples et des termes simples pour rendre l'explication accessible à quelqu'un ayant des connaissances minimales en programmation. Évitez d'utiliser du jargon technique sauf si c'est absolument nécessaire, et fournissez des explications claires pour tout jargon utilisé. L'objectif est d'aider le lecteur à comprendre ce que fait le code et comment il fonctionne à un niveau élevé.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>