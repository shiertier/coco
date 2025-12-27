# Estimador de eficiencia

Calcula la complejidad temporal de funciones y algoritmos.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es analizar la función o algoritmo proporcionado y calcular su complejidad temporal usando la notación Big O. Explica tu razonamiento paso a paso, describiendo cómo llegaste a la complejidad temporal final. Considera el escenario del peor caso al determinar la complejidad temporal. Si la función o algoritmo contiene múltiples pasos o bucles anidados, proporciona la complejidad temporal para cada paso y luego da la complejidad temporal general para toda la función o algoritmo. Asume que cualquier función incorporada u operación utilizada tiene una complejidad temporal de O(1) a menos que se especifique lo contrario. |
| Usuario   | def example_function(n): for i in range(n): print(i) for j in range(n): for k in range(n): print(j, k) |

### Ejemplo de salida

> Para analizar la complejidad temporal de la función dada `example_function(n)`, la desglosaremos en pasos y consideraremos el escenario del peor caso.
>
> Paso 1: Analizando el primer bucle
>
> ```
> for i in range(n):
>    print(i)
>
> ```

> Este bucle itera `n` veces y realiza una operación de tiempo constante (`print(i)`) en cada iteración. Por lo tanto, la complejidad temporal de este paso es O(n).
>
> Paso 2: Analizando los bucles anidados
>
> ```
> for j in range(n):
>     for k in range(n):
>         print(j, k)
>
> ```
>
> El bucle externo itera `n` veces, y para cada iteración, el bucle interno también itera `n` veces. En cada iteración del bucle interno, se realiza una operación de tiempo constante (`print(j, k)`). Por lo tanto, la complejidad temporal de los bucles anidados es O(n \* n) = O(n^2).
>
> Paso 3: Combinando las complejidades temporales  
> La función `example_function(n)` consiste en dos pasos: el primer bucle y los bucles anidados. La complejidad temporal general es la suma de las complejidades temporales de cada paso.
>
> Complejidad temporal del primer bucle: O(n)  
> Complejidad temporal de los bucles anidados: O(n^2)
>
> Para determinar la complejidad temporal general, consideramos el término dominante, que es el término con el orden más alto de crecimiento. En este caso, O(n^2) domina a O(n).
>
> Por lo tanto, la complejidad temporal general de la función `example_function(n)` es O(n^2).
>
> En el escenario del peor caso, cuando `n` es grande, los bucles anidados dominarán el tiempo de ejecución, y la función tendrá una complejidad temporal cuadrática.

---

### Solicitud de API

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # por defecto es os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=1000,
        temperature=0,
        system="Tu tarea es analizar la función o algoritmo proporcionado y calcular su complejidad temporal usando la notación Big O. Explica tu razonamiento paso a paso, describiendo cómo llegaste a la complejidad temporal final. Considera el escenario del peor caso al determinar la complejidad temporal. Si la función o algoritmo contiene múltiples pasos o bucles anidados, proporciona la complejidad temporal para cada paso y luego da la complejidad temporal general para toda la función o algoritmo. Asume que cualquier función incorporada u operación utilizada tiene una complejidad temporal de O(1) a menos que se especifique lo contrario.",
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
      apiKey: "my_api_key", // por defecto es process.env["ANTHROPIC_API_KEY"]
    });
    
    const msg = await anthropic.messages.create({
      model: "claude-sonnet-4-5",
      max_tokens: 1000,
      temperature: 0,
      system: "Tu tarea es analizar la función o algoritmo proporcionado y calcular su complejidad temporal usando la notación Big O. Explica tu razonamiento paso a paso, describiendo cómo llegaste a la complejidad temporal final. Considera el escenario del peor caso al determinar la complejidad temporal. Si la función o algoritmo contiene múltiples pasos o bucles anidados, proporciona la complejidad temporal para cada paso y luego da la complejidad temporal general para toda la función o algoritmo. Asume que cualquier función incorporada u operación utilizada tiene una complejidad temporal de O(1) a menos que se especifique lo contrario.",
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
    
    # Ver https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # para opciones de autenticación
    client = AnthropicBedrock()
    
    message = client.messages.create(
        model="anthropic.claude-sonnet-4-5-20250929-v1:0",
        max_tokens=1000,
        temperature=0,
        system="Tu tarea es analizar la función o algoritmo proporcionado y calcular su complejidad temporal usando la notación Big O. Explica tu razonamiento paso a paso, describiendo cómo llegaste a la complejidad temporal final. Considera el escenario del peor caso al determinar la complejidad temporal. Si la función o algoritmo contiene múltiples pasos o bucles anidados, proporciona la complejidad temporal para cada paso y luego da la complejidad temporal general para toda la función o algoritmo. Asume que cualquier función incorporada u operación utilizada tiene una complejidad temporal de O(1) a menos que se especifique lo contrario.",
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
    
    // Ver https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    // para opciones de autenticación
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 1000,
      temperature: 0,
      system: "Tu tarea es analizar la función o algoritmo proporcionado y calcular su complejidad temporal usando la notación Big O. Explica tu razonamiento paso a paso, describiendo cómo llegaste a la complejidad temporal final. Considera el escenario del peor caso al determinar la complejidad temporal. Si la función o algoritmo contiene múltiples pasos o bucles anidados, proporciona la complejidad temporal para cada paso y luego da la complejidad temporal general para toda la función o algoritmo. Asume que cualquier función incorporada u operación utilizada tiene una complejidad temporal de O(1) a menos que se especifique lo contrario.",
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
        system="Tu tarea es analizar la función o algoritmo proporcionado y calcular su complejidad temporal usando la notación Big O. Explica tu razonamiento paso a paso, describiendo cómo llegaste a la complejidad temporal final. Considera el escenario del peor caso al determinar la complejidad temporal. Si la función o algoritmo contiene múltiples pasos o bucles anidados, proporciona la complejidad temporal para cada paso y luego da la complejidad temporal general para toda la función o algoritmo. Asume que cualquier función incorporada u operación utilizada tiene una complejidad temporal de O(1) a menos que se especifique lo contrario.",
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
    
    // Lee de las variables de entorno `CLOUD_ML_REGION` y `ANTHROPIC_VERTEX_PROJECT_ID`.
    // Adicionalmente pasa por el flujo estándar de `google-auth-library`.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      system: "Tu tarea es analizar la función o algoritmo proporcionado y calcular su complejidad temporal usando la notación Big O. Explica tu razonamiento paso a paso, describiendo cómo llegaste a la complejidad temporal final. Considera el escenario del peor caso al determinar la complejidad temporal. Si la función o algoritmo contiene múltiples pasos o bucles anidados, proporciona la complejidad temporal para cada paso y luego da la complejidad temporal general para toda la función o algoritmo. Asume que cualquier función incorporada u operación utilizada tiene una complejidad temporal de O(1) a menos que se especifique lo contrario.",
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