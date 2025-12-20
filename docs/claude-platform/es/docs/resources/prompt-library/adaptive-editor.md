# Editor adaptativo

Reescribe texto siguiendo instrucciones dadas por el usuario, como con un tono, audiencia o estilo diferente.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|      | Contenido |
| ---- | --- |
| Usuario | Reescribe el siguiente párrafo usando las siguientes instrucciones: en el estilo de un pirata. Párrafo: En 1758, el botánico y zoólogo sueco Carl Linnaeus publicó en su Systema Naturae, la nomenclatura de dos palabras para las especies (nomenclatura binomial). Canis es la palabra latina que significa "perro", y bajo este género, enumeró el perro doméstico, el lobo y el chacal dorado. |

### Ejemplo de salida

> ¡Ahoy marineros! Allá por 1758, ese escurridizo botánico y amante de los animales Carl Linnaeus sacó su libro Systema Naturae, que tenía esos nombres elegantes de dos palabras para toda clase de criaturas (nomenclatura binomial le llamaba). Canis es la palabra pirata para "perro", y bajo ese encabezado enumeró perros regulares, lobos y chacales dorados también. ¡Arrr! Todos esos perros son lo mismo para el viejo Capitán Linnaeus. ¡Puede que haya tenido una pata de palo y un loro en el hombro cuando escribió ese libro! ¡Jaja! ¡Ahora frieguen la cubierta, bribones, antes de que los haga caminar por la plancha!

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
        max_tokens=2000,
        temperature=1,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Reescribe el siguiente párrafo usando las siguientes instrucciones: en el estilo de un pirata.  \n  \nPárrafo:  \nEn 1758, el botánico y zoólogo sueco Carl Linnaeus publicó en su Systema Naturae, la nomenclatura de dos palabras para las especies (nomenclatura binomial). Canis es la palabra latina que significa \"perro\", y bajo este género, enumeró el perro doméstico, el lobo y el chacal dorado."
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
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Reescribe el siguiente párrafo usando las siguientes instrucciones: en el estilo de un pirata.  \n  \nPárrafo:  \nEn 1758, el botánico y zoólogo sueco Carl Linnaeus publicó en su Systema Naturae, la nomenclatura de dos palabras para las especies (nomenclatura binomial). Canis es la palabra latina que significa \"perro\", y bajo este género, enumeró el perro doméstico, el lobo y el chacal dorado."
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
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Reescribe el siguiente párrafo usando las siguientes instrucciones: en el estilo de un pirata.  \n  \nPárrafo:  \nEn 1758, el botánico y zoólogo sueco Carl Linnaeus publicó en su Systema Naturae, la nomenclatura de dos palabras para las especies (nomenclatura binomial). Canis es la palabra latina que significa \"perro\", y bajo este género, enumeró el perro doméstico, el lobo y el chacal dorado."
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
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Reescribe el siguiente párrafo usando las siguientes instrucciones: en el estilo de un pirata.  \n  \nPárrafo:  \nEn 1758, el botánico y zoólogo sueco Carl Linnaeus publicó en su Systema Naturae, la nomenclatura de dos palabras para las especies (nomenclatura binomial). Canis es la palabra latina que significa \"perro\", y bajo este género, enumeró el perro doméstico, el lobo y el chacal dorado."
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
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Reescribe el siguiente párrafo usando las siguientes instrucciones: en el estilo de un pirata.  \n  \nPárrafo:  \nEn 1758, el botánico y zoólogo sueco Carl Linnaeus publicó en su Systema Naturae, la nomenclatura de dos palabras para las especies (nomenclatura binomial). Canis es la palabra latina que significa \"perro\", y bajo este género, enumeró el perro doméstico, el lobo y el chacal dorado."
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
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Reescribe el siguiente párrafo usando las siguientes instrucciones: en el estilo de un pirata.  \n  \nPárrafo:  \nEn 1758, el botánico y zoólogo sueco Carl Linnaeus publicó en su Systema Naturae, la nomenclatura de dos palabras para las especies (nomenclatura binomial). Canis es la palabra latina que significa \"perro\", y bajo este género, enumeró el perro doméstico, el lobo y el chacal dorado."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>