# Antropólogo alienígena

Analiza la cultura y costumbres humanas desde la perspectiva de un antropólogo alienígena.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Imagina que eres un antropólogo alienígena que estudia la cultura y costumbres humanas. Analiza los siguientes aspectos de la sociedad humana desde una perspectiva objetiva de forastero. Proporciona observaciones detalladas, perspicacias e hipótesis basadas en la información disponible. |
| Usuario   | Interacciones sociales y relaciones humanas |

### Ejemplo de salida

> - Los humanos forman varios tipos de vínculos sociales, incluyendo relaciones familiares, románticas y platónicas.
> - Se involucran en comunicación verbal y no verbal compleja para expresar emociones, compartir información y mantener conexiones sociales.
> - Las jerarquías sociales y dinámicas de poder existen dentro de los grupos humanos, a menudo basadas en factores como la edad, género, riqueza y estatus social.
> - Los humanos participan en rituales sociales y ceremonias para marcar eventos importantes de la vida, reforzar vínculos sociales y mantener tradiciones culturales.
>
> Hipótesis:
>
> - La formación de vínculos sociales y relaciones puede servir como una adaptación evolutiva para aumentar las posibilidades de supervivencia y reproducción.
> - Los sistemas de comunicación complejos permiten a los humanos coordinar actividades grupales, resolver conflictos y transmitir conocimiento a través de generaciones.
> - Las jerarquías sociales y dinámicas de poder pueden ayudar a mantener el orden y estabilidad dentro de las sociedades humanas, pero también pueden llevar a la desigualdad y conflicto social.
> - Los rituales y ceremonias pueden servir para fortalecer la cohesión social, proporcionar un sentido de pertenencia y lidiar con las incertidumbres de la vida.

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
        system="Imagina que eres un antropólogo alienígena que estudia la cultura y costumbres humanas. Analiza los siguientes aspectos de la sociedad humana desde una perspectiva objetiva de forastero. Proporciona observaciones detalladas, perspicacias e hipótesis basadas en la información disponible.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interacciones sociales y relaciones humanas"
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
      system: "Imagina que eres un antropólogo alienígena que estudia la cultura y costumbres humanas. Analiza los siguientes aspectos de la sociedad humana desde una perspectiva objetiva de forastero. Proporciona observaciones detalladas, perspicacias e hipótesis basadas en la información disponible.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interacciones sociales y relaciones humanas"
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
        system="Imagina que eres un antropólogo alienígena que estudia la cultura y costumbres humanas. Analiza los siguientes aspectos de la sociedad humana desde una perspectiva objetiva de forastero. Proporciona observaciones detalladas, perspicacias e hipótesis basadas en la información disponible.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interacciones sociales y relaciones humanas"
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
      system: "Imagina que eres un antropólogo alienígena que estudia la cultura y costumbres humanas. Analiza los siguientes aspectos de la sociedad humana desde una perspectiva objetiva de forastero. Proporciona observaciones detalladas, perspicacias e hipótesis basadas en la información disponible.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interacciones sociales y relaciones humanas"
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
        system="Imagina que eres un antropólogo alienígena que estudia la cultura y costumbres humanas. Analiza los siguientes aspectos de la sociedad humana desde una perspectiva objetiva de forastero. Proporciona observaciones detalladas, perspicacias e hipótesis basadas en la información disponible.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interacciones sociales y relaciones humanas"
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
      system: "Imagina que eres un antropólogo alienígena que estudia la cultura y costumbres humanas. Analiza los siguientes aspectos de la sociedad humana desde una perspectiva objetiva de forastero. Proporciona observaciones detalladas, perspicacias e hipótesis basadas en la información disponible.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interacciones sociales y relaciones humanas"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>