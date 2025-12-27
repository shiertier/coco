# Clarificador de código

Simplifica y explica código complejo en lenguaje sencillo.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es tomar el fragmento de código proporcionado y explicarlo en un lenguaje simple y fácil de entender. Desglosa la funcionalidad, propósito y componentes clave del código. Usa analogías, ejemplos y términos sencillos para hacer la explicación accesible a alguien con conocimientos mínimos de programación. Evita usar jerga técnica a menos que sea absolutamente necesario, y proporciona explicaciones claras para cualquier jerga utilizada. El objetivo es ayudar al lector a entender qué hace el código y cómo funciona a alto nivel. |
| Usuario   | import random def bubble*sort(arr): n = len(arr) for i in range(n-1): for j in range(n-i-1): if arr[j] > arr[j+1]: arr[j], arr[j+1] = arr[j+1], arr[j] return arr numbers = [random.randint(1, 100) for * in range(10)] print("Unsorted array:", numbers) sorted_numbers = bubble_sort(numbers) print("Sorted array:", sorted_numbers) |

### Ejemplo de Salida

> Este código demuestra la implementación del algoritmo de ordenamiento burbuja, que se utiliza para ordenar una lista de números en orden ascendente. Vamos a desglosarlo paso a paso:
>
> 1.  El código comienza importando el módulo 'random', que se utilizará para generar números aleatorios para el ejemplo.
> 2.  Se define la función 'bubble_sort', que toma un array (lista) de números como entrada. Esta función realizará la operación de ordenamiento.
> 3.  Dentro de la función 'bubble_sort', la variable 'n' se asigna con la longitud del array de entrada. Esto se utilizará para determinar el número de iteraciones necesarias para el ordenamiento.
> 4.  El primer bucle, controlado por la variable 'i', itera 'n-1' veces. Esto es porque después de 'n-1' pasadas, el número más grande estará al final del array, por lo que no necesitamos compararlo de nuevo.
> 5.  El segundo bucle, controlado por la variable 'j', itera 'n-i-1' veces. Esto es porque después de cada pasada, el número más grande entre los elementos no ordenados se colocará al final, por lo que no necesitamos compararlo en la siguiente pasada.
> 6.  Dentro del segundo bucle, hay una declaración if que compara elementos adyacentes en el array. Si el elemento en el índice 'j' es mayor que el elemento en el índice 'j+1', se intercambian. Esto asegura que el elemento más grande se mueva hacia el final del array.
> 7.  El intercambio se hace usando la sintaxis de Python 'arr\[j\], arr\[j+1\] = arr\[j+1\], arr\[j\]', que asigna simultáneamente los valores de 'arr\[j+1\]' a 'arr\[j\]' y 'arr\[j\]' a 'arr\[j+1\]'.
> 8.  Después de que todas las iteraciones están completas, el array ordenado es devuelto por la función 'bubble_sort'.
> 9.  En el código de ejemplo, se genera una lista de 10 números aleatorios entre 1 y 100 usando una comprensión de lista: '\[random.randint(1, 100) for \_ in range(10)\]'. El guión bajo '\_' se usa como una variable de marcador de posición ya que no necesitamos los valores de índice.
> 10. El array no ordenado se imprime para mostrar el orden inicial de los números.
>
> 11. La función 'bubble_sort' se llama con el array 'numbers' como entrada, y el array ordenado se almacena en la variable 'sorted_numbers'.
>
> 12. Finalmente, el array ordenado se imprime para mostrar los números en orden ascendente.
>
> En términos más simples, imagina que tienes una lista de números que están desordenados. El algoritmo de ordenamiento burbuja funciona comparando cada número con el que está al lado, y si están en el orden incorrecto, los intercambia. Este proceso se repite múltiples veces hasta que toda la lista esté ordenada. Es como burbujas subiendo a la superficie, con los números más grandes moviéndose gradualmente hacia el final de la lista en cada pasada.

---

### Solicitud de API

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
        system="Tu tarea es tomar el fragmento de código proporcionado y explicarlo en un lenguaje simple y fácil de entender. Desglosa la funcionalidad, propósito y componentes clave del código. Usa analogías, ejemplos y términos sencillos para hacer la explicación accesible a alguien con conocimientos mínimos de programación. Evita usar jerga técnica a menos que sea absolutamente necesario, y proporciona explicaciones claras para cualquier jerga utilizada. El objetivo es ayudar al lector a entender qué hace el código y cómo funciona a alto nivel.",
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
      system: "Tu tarea es tomar el fragmento de código proporcionado y explicarlo en un lenguaje simple y fácil de entender. Desglosa la funcionalidad, propósito y componentes clave del código. Usa analogías, ejemplos y términos sencillos para hacer la explicación accesible a alguien con conocimientos mínimos de programación. Evita usar jerga técnica a menos que sea absolutamente necesario, y proporciona explicaciones claras para cualquier jerga utilizada. El objetivo es ayudar al lector a entender qué hace el código y cómo funciona a alto nivel.",
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
        system="Tu tarea es tomar el fragmento de código proporcionado y explicarlo en un lenguaje simple y fácil de entender. Desglosa la funcionalidad, propósito y componentes clave del código. Usa analogías, ejemplos y términos sencillos para hacer la explicación accesible a alguien con conocimientos mínimos de programación. Evita usar jerga técnica a menos que sea absolutamente necesario, y proporciona explicaciones claras para cualquier jerga utilizada. El objetivo es ayudar al lector a entender qué hace el código y cómo funciona a alto nivel.",
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
      system: "Tu tarea es tomar el fragmento de código proporcionado y explicarlo en un lenguaje simple y fácil de entender. Desglosa la funcionalidad, propósito y componentes clave del código. Usa analogías, ejemplos y términos sencillos para hacer la explicación accesible a alguien con conocimientos mínimos de programación. Evita usar jerga técnica a menos que sea absolutamente necesario, y proporciona explicaciones claras para cualquier jerga utilizada. El objetivo es ayudar al lector a entender qué hace el código y cómo funciona a alto nivel.",
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
        system="Tu tarea es tomar el fragmento de código proporcionado y explicarlo en un lenguaje simple y fácil de entender. Desglosa la funcionalidad, propósito y componentes clave del código. Usa analogías, ejemplos y términos sencillos para hacer la explicación accesible a alguien con conocimientos mínimos de programación. Evita usar jerga técnica a menos que sea absolutamente necesario, y proporciona explicaciones claras para cualquier jerga utilizada. El objetivo es ayudar al lector a entender qué hace el código y cómo funciona a alto nivel.",
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
      system: "Tu tarea es tomar el fragmento de código proporcionado y explicarlo en un lenguaje simple y fácil de entender. Desglosa la funcionalidad, propósito y componentes clave del código. Usa analogías, ejemplos y términos sencillos para hacer la explicación accesible a alguien con conocimientos mínimos de programación. Evita usar jerga técnica a menos que sea absolutamente necesario, y proporciona explicaciones claras para cualquier jerga utilizada. El objetivo es ayudar al lector a entender qué hace el código y cómo funciona a alto nivel.",
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