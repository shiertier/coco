# Clasificador de reseñas

Categoriza comentarios en etiquetas y categorizaciones preespecificadas.

---

|        | Contenido |
| --- | --- |
| Sistema | Eres un asistente de IA entrenado para categorizar comentarios de usuarios en categorías predefinidas, junto con análisis de sentimientos para cada categoría. Tu objetivo es analizar cada pieza de comentario, asignar las categorías más relevantes y determinar el sentimiento (positivo, negativo o neutral) asociado con cada categoría basado en el contenido del comentario. Categorías Predefinidas: Características y Funcionalidad del Producto Características Principales Complementos e Integraciones Personalización y Configuración Experiencia de Usuario y Diseño Facilidad de Uso Navegación y Descubribilidad Diseño Visual y Estética Accesibilidad Rendimiento y Confiabilidad Velocidad y Capacidad de Respuesta Tiempo de Actividad y Disponibilidad Escalabilidad Corrección de Errores y Manejo de Errores Soporte al Cliente y Servicio Capacidad de Respuesta y Disponibilidad Conocimiento y Experiencia Resolución de Problemas y Seguimiento Recursos de Autoservicio Facturación, Precios y Licencias Modelo de Precios y Niveles Procesos de Facturación y Facturación Gestión de Licencias Actualizaciones y Renovaciones Seguridad, Cumplimiento y Privacidad Protección de Datos y Confidencialidad Control de Acceso y Autenticación Cumplimiento Regulatorio Respuesta a Incidentes y Monitoreo Compatibilidad Móvil y Multiplataforma Funcionalidad de Aplicación Móvil Sincronización y Consistencia de Datos Diseño Responsivo Compatibilidad de Dispositivos y SO Integraciones de Terceros y API Funcionalidad y Confiabilidad de Integración Documentación y Soporte de API Personalización y Extensibilidad Incorporación, Capacitación y Documentación Guías de Usuario y Tutoriales Orientación en la Aplicación y Consejos Webinars y Capacitación en Vivo Base de Conocimientos y Preguntas Frecuentes |
| Usuario   | He estado usando su plataforma de marketing por correo electrónico durante unas semanas, y debo decir que las características principales como la creación de campañas, plantillas de correo electrónico y gestión de contactos son fantásticas. El editor de arrastrar y soltar hace que sea fácil diseñar correos electrónicos de aspecto profesional, y las opciones de segmentación me ayudan a dirigirme a la audiencia correcta. Sin embargo, he tenido algunos problemas con la capacidad de respuesta móvil de los correos electrónicos. Algunos de mis suscriptores han reportado que los diseños se ven rotos en sus teléfonos inteligentes, lo cual es preocupante. Me encantaría ver mejoras en esta área. Además, noté que la plataforma carece de algunas integraciones clave con herramientas CRM populares, lo cual sería increíblemente útil para gestionar nuestros prospectos y clientes. En una nota positiva, el equipo de soporte al cliente ha sido receptivo y útil cada vez que me he comunicado con preguntas. En general, es una gran herramienta, pero definitivamente hay espacio para mejoras en términos de compatibilidad móvil e integraciones de terceros. |

### Ejemplo de salida

> Categorías:
>
> - Características y Funcionalidad del Producto
>   - Características Principales - Positivo
>     Experiencia de Usuario y Diseño
>   - Facilidad de Uso - Positivo
>   - Diseño Visual y Estética - Positivo
> - Compatibilidad Móvil y Multiplataforma
>   - Funcionalidad de Aplicación Móvil - Negativo
>   - Diseño Responsivo - Negativo
>   - Compatibilidad de Dispositivos y SO - Negativo
> - Integraciones de Terceros y API
>   - Funcionalidad y Confiabilidad de Integración - Negativo
> - Soporte al Cliente y Servicio
>   - Capacidad de Respuesta y Disponibilidad - Positivo
>   - Resolución de Problemas y Seguimiento - Positivo

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
        temperature=1,
        system="Eres un asistente de IA entrenado para categorizar comentarios de usuarios en categorías predefinidas, junto con análisis de sentimientos para cada categoría. Tu objetivo es analizar cada pieza de comentario, asignar las categorías más relevantes y determinar el sentimiento (positivo, negativo o neutral) asociado con cada categoría basado en el contenido del comentario.\n\nCategorías Predefinidas:\n- Características y Funcionalidad del Producto\n    - Características Principales\n    - Complementos e Integraciones\n    - Personalización y Configuración\n- Experiencia de Usuario y Diseño\n    - Facilidad de Uso\n    - Navegación y Descubribilidad\n    - Diseño Visual y Estética\n    - Accesibilidad\n- Rendimiento y Confiabilidad\n    - Velocidad y Capacidad de Respuesta\n    - Tiempo de Actividad y Disponibilidad\n    - Escalabilidad\n    - Corrección de Errores y Manejo de Errores\n- Soporte al Cliente y Servicio\n    - Capacidad de Respuesta y Disponibilidad\n    - Conocimiento y Experiencia\n    - Resolución de Problemas y Seguimiento\n    - Recursos de Autoservicio\n- Facturación, Precios y Licencias\n    - Modelo de Precios y Niveles\n    - Procesos de Facturación y Facturación\n    - Gestión de Licencias\n    - Actualizaciones y Renovaciones\n- Seguridad, Cumplimiento y Privacidad\n    - Protección de Datos y Confidencialidad\n    - Control de Acceso y Autenticación\n    - Cumplimiento Regulatorio\n    - Respuesta a Incidentes y Monitoreo\n- Compatibilidad Móvil y Multiplataforma\n    - Funcionalidad de Aplicación Móvil\n    - Sincronización y Consistencia de Datos\n    - Diseño Responsivo\n    - Compatibilidad de Dispositivos y SO\n- Integraciones de Terceros y API\n    - Funcionalidad y Confiabilidad de Integración\n    - Documentación y Soporte de API\n    - Personalización y Extensibilidad\n- Incorporación, Capacitación y Documentación\n    - Guías de Usuario y Tutoriales\n    - Orientación en la Aplicación y Consejos\n    - Webinars y Capacitación en Vivo\n    - Base de Conocimientos y Preguntas Frecuentes",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "He estado usando su plataforma de marketing por correo electrónico durante unas semanas, y debo decir que las características principales como la creación de campañas, plantillas de correo electrónico y gestión de contactos son fantásticas. El editor de arrastrar y soltar hace que sea fácil diseñar correos electrónicos de aspecto profesional, y las opciones de segmentación me ayudan a dirigirme a la audiencia correcta. Sin embargo, he tenido algunos problemas con la capacidad de respuesta móvil de los correos electrónicos. Algunos de mis suscriptores han reportado que los diseños se ven rotos en sus teléfonos inteligentes, lo cual es preocupante. Me encantaría ver mejoras en esta área. Además, noté que la plataforma carece de algunas integraciones clave con herramientas CRM populares, lo cual sería increíblemente útil para gestionar nuestros prospectos y clientes. En una nota positiva, el equipo de soporte al cliente ha sido receptivo y útil cada vez que me he comunicado con preguntas. En general, es una gran herramienta, pero definitivamente hay espacio para mejoras en términos de compatibilidad móvil e integraciones de terceros."
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
      temperature: 1,
      system: "Eres un asistente de IA entrenado para categorizar comentarios de usuarios en categorías predefinidas, junto con análisis de sentimientos para cada categoría. Tu objetivo es analizar cada pieza de comentario, asignar las categorías más relevantes y determinar el sentimiento (positivo, negativo o neutral) asociado con cada categoría basado en el contenido del comentario.\n\nCategorías Predefinidas:\n- Características y Funcionalidad del Producto\n    - Características Principales\n    - Complementos e Integraciones\n    - Personalización y Configuración\n- Experiencia de Usuario y Diseño\n    - Facilidad de Uso\n    - Navegación y Descubribilidad\n    - Diseño Visual y Estética\n    - Accesibilidad\n- Rendimiento y Confiabilidad\n    - Velocidad y Capacidad de Respuesta\n    - Tiempo de Actividad y Disponibilidad\n    - Escalabilidad\n    - Corrección de Errores y Manejo de Errores\n- Soporte al Cliente y Servicio\n    - Capacidad de Respuesta y Disponibilidad\n    - Conocimiento y Experiencia\n    - Resolución de Problemas y Seguimiento\n    - Recursos de Autoservicio\n- Facturación, Precios y Licencias\n    - Modelo de Precios y Niveles\n    - Procesos de Facturación y Facturación\n    - Gestión de Licencias\n    - Actualizaciones y Renovaciones\n- Seguridad, Cumplimiento y Privacidad\n    - Protección de Datos y Confidencialidad\n    - Control de Acceso y Autenticación\n    - Cumplimiento Regulatorio\n    - Respuesta a Incidentes y Monitoreo\n- Compatibilidad Móvil y Multiplataforma\n    - Funcionalidad de Aplicación Móvil\n    - Sincronización y Consistencia de Datos\n    - Diseño Responsivo\n    - Compatibilidad de Dispositivos y SO\n- Integraciones de Terceros y API\n    - Funcionalidad y Confiabilidad de Integración\n    - Documentación y Soporte de API\n    - Personalización y Extensibilidad\n- Incorporación, Capacitación y Documentación\n    - Guías de Usuario y Tutoriales\n    - Orientación en la Aplicación y Consejos\n    - Webinars y Capacitación en Vivo\n    - Base de Conocimientos y Preguntas Frecuentes",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "He estado usando su plataforma de marketing por correo electrónico durante unas semanas, y debo decir que las características principales como la creación de campañas, plantillas de correo electrónico y gestión de contactos son fantásticas. El editor de arrastrar y soltar hace que sea fácil diseñar correos electrónicos de aspecto profesional, y las opciones de segmentación me ayudan a dirigirme a la audiencia correcta. Sin embargo, he tenido algunos problemas con la capacidad de respuesta móvil de los correos electrónicos. Algunos de mis suscriptores han reportado que los diseños se ven rotos en sus teléfonos inteligentes, lo cual es preocupante. Me encantaría ver mejoras en esta área. Además, noté que la plataforma carece de algunas integraciones clave con herramientas CRM populares, lo cual sería increíblemente útil para gestionar nuestros prospectos y clientes. En una nota positiva, el equipo de soporte al cliente ha sido receptivo y útil cada vez que me he comunicado con preguntas. En general, es una gran herramienta, pero definitivamente hay espacio para mejoras en términos de compatibilidad móvil e integraciones de terceros."
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
        temperature=1,
        system="Eres un asistente de IA entrenado para categorizar comentarios de usuarios en categorías predefinidas, junto con análisis de sentimientos para cada categoría. Tu objetivo es analizar cada pieza de comentario, asignar las categorías más relevantes y determinar el sentimiento (positivo, negativo o neutral) asociado con cada categoría basado en el contenido del comentario.\n\nCategorías Predefinidas:\n- Características y Funcionalidad del Producto\n    - Características Principales\n    - Complementos e Integraciones\n    - Personalización y Configuración\n- Experiencia de Usuario y Diseño\n    - Facilidad de Uso\n    - Navegación y Descubribilidad\n    - Diseño Visual y Estética\n    - Accesibilidad\n- Rendimiento y Confiabilidad\n    - Velocidad y Capacidad de Respuesta\n    - Tiempo de Actividad y Disponibilidad\n    - Escalabilidad\n    - Corrección de Errores y Manejo de Errores\n- Soporte al Cliente y Servicio\n    - Capacidad de Respuesta y Disponibilidad\n    - Conocimiento y Experiencia\n    - Resolución de Problemas y Seguimiento\n    - Recursos de Autoservicio\n- Facturación, Precios y Licencias\n    - Modelo de Precios y Niveles\n    - Procesos de Facturación y Facturación\n    - Gestión de Licencias\n    - Actualizaciones y Renovaciones\n- Seguridad, Cumplimiento y Privacidad\n    - Protección de Datos y Confidencialidad\n    - Control de Acceso y Autenticación\n    - Cumplimiento Regulatorio\n    - Respuesta a Incidentes y Monitoreo\n- Compatibilidad Móvil y Multiplataforma\n    - Funcionalidad de Aplicación Móvil\n    - Sincronización y Consistencia de Datos\n    - Diseño Responsivo\n    - Compatibilidad de Dispositivos y SO\n- Integraciones de Terceros y API\n    - Funcionalidad y Confiabilidad de Integración\n    - Documentación y Soporte de API\n    - Personalización y Extensibilidad\n- Incorporación, Capacitación y Documentación\n    - Guías de Usuario y Tutoriales\n    - Orientación en la Aplicación y Consejos\n    - Webinars y Capacitación en Vivo\n    - Base de Conocimientos y Preguntas Frecuentes",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "He estado usando su plataforma de marketing por correo electrónico durante unas semanas, y debo decir que las características principales como la creación de campañas, plantillas de correo electrónico y gestión de contactos son fantásticas. El editor de arrastrar y soltar hace que sea fácil diseñar correos electrónicos de aspecto profesional, y las opciones de segmentación me ayudan a dirigirme a la audiencia correcta. Sin embargo, he tenido algunos problemas con la capacidad de respuesta móvil de los correos electrónicos. Algunos de mis suscriptores han reportado que los diseños se ven rotos en sus teléfonos inteligentes, lo cual es preocupante. Me encantaría ver mejoras en esta área. Además, noté que la plataforma carece de algunas integraciones clave con herramientas CRM populares, lo cual sería increíblemente útil para gestionar nuestros prospectos y clientes. En una nota positiva, el equipo de soporte al cliente ha sido receptivo y útil cada vez que me he comunicado con preguntas. En general, es una gran herramienta, pero definitivamente hay espacio para mejoras en términos de compatibilidad móvil e integraciones de terceros."
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
      temperature: 1,
      system: "Eres un asistente de IA entrenado para categorizar comentarios de usuarios en categorías predefinidas, junto con análisis de sentimientos para cada categoría. Tu objetivo es analizar cada pieza de comentario, asignar las categorías más relevantes y determinar el sentimiento (positivo, negativo o neutral) asociado con cada categoría basado en el contenido del comentario.\n\nCategorías Predefinidas:\n- Características y Funcionalidad del Producto\n    - Características Principales\n    - Complementos e Integraciones\n    - Personalización y Configuración\n- Experiencia de Usuario y Diseño\n    - Facilidad de Uso\n    - Navegación y Descubribilidad\n    - Diseño Visual y Estética\n    - Accesibilidad\n- Rendimiento y Confiabilidad\n    - Velocidad y Capacidad de Respuesta\n    - Tiempo de Actividad y Disponibilidad\n    - Escalabilidad\n    - Corrección de Errores y Manejo de Errores\n- Soporte al Cliente y Servicio\n    - Capacidad de Respuesta y Disponibilidad\n    - Conocimiento y Experiencia\n    - Resolución de Problemas y Seguimiento\n    - Recursos de Autoservicio\n- Facturación, Precios y Licencias\n    - Modelo de Precios y Niveles\n    - Procesos de Facturación y Facturación\n    - Gestión de Licencias\n    - Actualizaciones y Renovaciones\n- Seguridad, Cumplimiento y Privacidad\n    - Protección de Datos y Confidencialidad\n    - Control de Acceso y Autenticación\n    - Cumplimiento Regulatorio\n    - Respuesta a Incidentes y Monitoreo\n- Compatibilidad Móvil y Multiplataforma\n    - Funcionalidad de Aplicación Móvil\n    - Sincronización y Consistencia de Datos\n    - Diseño Responsivo\n    - Compatibilidad de Dispositivos y SO\n- Integraciones de Terceros y API\n    - Funcionalidad y Confiabilidad de Integración\n    - Documentación y Soporte de API\n    - Personalización y Extensibilidad\n- Incorporación, Capacitación y Documentación\n    - Guías de Usuario y Tutoriales\n    - Orientación en la Aplicación y Consejos\n    - Webinars y Capacitación en Vivo\n    - Base de Conocimientos y Preguntas Frecuentes",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "He estado usando su plataforma de marketing por correo electrónico durante unas semanas, y debo decir que las características principales como la creación de campañas, plantillas de correo electrónico y gestión de contactos son fantásticas. El editor de arrastrar y soltar hace que sea fácil diseñar correos electrónicos de aspecto profesional, y las opciones de segmentación me ayudan a dirigirme a la audiencia correcta. Sin embargo, he tenido algunos problemas con la capacidad de respuesta móvil de los correos electrónicos. Algunos de mis suscriptores han reportado que los diseños se ven rotos en sus teléfonos inteligentes, lo cual es preocupante. Me encantaría ver mejoras en esta área. Además, noté que la plataforma carece de algunas integraciones clave con herramientas CRM populares, lo cual sería increíblemente útil para gestionar nuestros prospectos y clientes. En una nota positiva, el equipo de soporte al cliente ha sido receptivo y útil cada vez que me he comunicado con preguntas. En general, es una gran herramienta, pero definitivamente hay espacio para mejoras en términos de compatibilidad móvil e integraciones de terceros."
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
        temperature=1,
        system="Eres un asistente de IA entren ado para categorizar comentarios de usuarios en categorías predefinidas, junto con análisis de sentimientos para cada categoría. Tu objetivo es analizar cada pieza de comentario, asignar las categorías más relevantes y determinar el sentimiento (positivo, negativo o neutral) asociado con cada categoría basado en el contenido del comentario.\n\nCategorías Predefinidas:\n- Características y Funcionalidad del Producto\n    - Características Principales\n    - Complementos e Integraciones\n    - Personalización y Configuración\n- Experiencia de Usuario y Diseño\n    - Facilidad de Uso\n    - Navegación y Descubribilidad\n    - Diseño Visual y Estética\n    - Accesibilidad\n- Rendimiento y Confiabilidad\n    - Velocidad y Capacidad de Respuesta\n    - Tiempo de Actividad y Disponibilidad\n    - Escalabilidad\n    - Corrección de Errores y Manejo de Errores\n- Soporte al Cliente y Servicio\n    - Capacidad de Respuesta y Disponibilidad\n    - Conocimiento y Experiencia\n    - Resolución de Problemas y Seguimiento\n    - Recursos de Autoservicio\n- Facturación, Precios y Licencias\n    - Modelo de Precios y Niveles\n    - Procesos de Facturación y Facturación\n    - Gestión de Licencias\n    - Actualizaciones y Renovaciones\n- Seguridad, Cumplimiento y Privacidad\n    - Protección de Datos y Confidencialidad\n    - Control de Acceso y Autenticación\n    - Cumplimiento Regulatorio\n    - Respuesta a Incidentes y Monitoreo\n- Compatibilidad Móvil y Multiplataforma\n    - Funcionalidad de Aplicación Móvil\n    - Sincronización y Consistencia de Datos\n    - Diseño Responsivo\n    - Compatibilidad de Dispositivos y SO\n- Integraciones de Terceros y API\n    - Funcionalidad y Confiabilidad de Integración\n    - Documentación y Soporte de API\n    - Personalización y Extensibilidad\n- Incorporación, Capacitación y Documentación\n    - Guías de Usuario y Tutoriales\n    - Orientación en la Aplicación y Consejos\n    - Webinars y Capacitación en Vivo\n    - Base de Conocimientos y Preguntas Frecuentes",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "He estado usando su plataforma de marketing por correo electrónico durante unas semanas, y debo decir que las características principales como la creación de campañas, plantillas de correo electrónico y gestión de contactos son fantásticas. El editor de arrastrar y soltar hace que sea fácil diseñar correos electrónicos de aspecto profesional, y las opciones de segmentación me ayudan a dirigirme a la audiencia correcta. Sin embargo, he tenido algunos problemas con la capacidad de respuesta móvil de los correos electrónicos. Algunos de mis suscriptores han reportado que los diseños se ven rotos en sus teléfonos inteligentes, lo cual es preocupante. Me encantaría ver mejoras en esta área. Además, noté que la plataforma carece de algunas integraciones clave con herramientas CRM populares, lo cual sería increíblemente útil para gestionar nuestros prospectos y clientes. En una nota positiva, el equipo de soporte al cliente ha sido receptivo y útil cada vez que me he comunicado con preguntas. En general, es una gran herramienta, pero definitivamente hay espacio para mejoras en términos de compatibilidad móvil e integraciones de terceros."
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
      temperature: 1,
      system: "Eres un asistente de IA entrenado para categorizar comentarios de usuarios en categorías predefinidas, junto con análisis de sentimientos para cada categoría. Tu objetivo es analizar cada pieza de comentario, asignar las categorías más relevantes y determinar el sentimiento (positivo, negativo o neutral) asociado con cada categoría basado en el contenido del comentario.\n\nCategorías Predefinidas:\n- Características y Funcionalidad del Producto\n    - Características Principales\n    - Complementos e Integraciones\n    - Personalización y Configuración\n- Experiencia de Usuario y Diseño\n    - Facilidad de Uso\n    - Navegación y Descubribilidad\n    - Diseño Visual y Estética\n    - Accesibilidad\n- Rendimiento y Confiabilidad\n    - Velocidad y Capacidad de Respuesta\n    - Tiempo de Actividad y Disponibilidad\n    - Escalabilidad\n    - Corrección de Errores y Manejo de Errores\n- Soporte al Cliente y Servicio\n    - Capacidad de Respuesta y Disponibilidad\n    - Conocimiento y Experiencia\n    - Resolución de Problemas y Seguimiento\n    - Recursos de Autoservicio\n- Facturación, Precios y Licencias\n    - Modelo de Precios y Niveles\n    - Procesos de Facturación y Facturación\n    - Gestión de Licencias\n    - Actualizaciones y Renovaciones\n- Seguridad, Cumplimiento y Privacidad\n    - Protección de Datos y Confidencialidad\n    - Control de Acceso y Autenticación\n    - Cumplimiento Regulatorio\n    - Respuesta a Incidentes y Monitoreo\n- Compatibilidad Móvil y Multiplataforma\n    - Funcionalidad de Aplicación Móvil\n    - Sincronización y Consistencia de Datos\n    - Diseño Responsivo\n    - Compatibilidad de Dispositivos y SO\n- Integraciones de Terceros y API\n    - Funcionalidad y Confiabilidad de Integración\n    - Documentación y Soporte de API\n    - Personalización y Extensibilidad\n- Incorporación, Capacitación y Documentación\n    - Guías de Usuario y Tutoriales\n    - Orientación en la Aplicación y Consejos\n    - Webinars y Capacitación en Vivo\n    - Base de Conocimientos y Preguntas Frecuentes",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "He estado usando su plataforma de marketing por correo electrónico durante unas semanas, y debo decir que las características principales como la creación de campañas, plantillas de correo electrónico y gestión de contactos son fantásticas. El editor de arrastrar y soltar hace que sea fácil diseñar correos electrónicos de aspecto profesional, y las opciones de segmentación me ayudan a dirigirme a la audiencia correcta. Sin embargo, he tenido algunos problemas con la capacidad de respuesta móvil de los correos electrónicos. Algunos de mis suscriptores han reportado que los diseños se ven rotos en sus teléfonos inteligentes, lo cual es preocupante. Me encantaría ver mejoras en esta área. Además, noté que la plataforma carece de algunas integraciones clave con herramientas CRM populares, lo cual sería increíblemente útil para gestionar nuestros prospectos y clientes. En una nota positiva, el equipo de soporte al cliente ha sido receptivo y útil cada vez que me he comunicado con preguntas. En general, es una gran herramienta, pero definitivamente hay espacio para mejoras en términos de compatibilidad móvil e integraciones de terceros."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>