# Errores

---

## Errores HTTP

Nuestra API sigue un formato predecible de códigos de error HTTP:

* 400 - `invalid_request_error`: Hubo un problema con el formato o contenido de tu solicitud. También podemos usar este tipo de error para otros códigos de estado 4XX no listados a continuación.
* 401 - `authentication_error`: Hay un problema con tu clave API.
* 403 - `permission_error`: Tu clave API no tiene permiso para usar el recurso especificado.
* 404 - `not_found_error`: El recurso solicitado no fue encontrado.
* 413 - `request_too_large`: La solicitud excede el número máximo permitido de bytes. El tamaño máximo de solicitud es 32 MB para endpoints estándar de la API.
* 429 - `rate_limit_error`: Tu cuenta ha alcanzado un límite de velocidad.
* 500 - `api_error`: Ha ocurrido un error inesperado interno en los sistemas de Anthropic.
* 529 - `overloaded_error`: La API está temporalmente sobrecargada.

  <Warning>
  Los errores 529 pueden ocurrir cuando las APIs experimentan alto tráfico en todos los usuarios.
  
  En casos raros, si tu organización tiene un aumento repentino en el uso, podrías ver errores 429 debido a límites de aceleración en la API. Para evitar alcanzar límites de aceleración, aumenta tu tráfico gradualmente y mantén patrones de uso consistentes.
  </Warning>

Al recibir una respuesta de [streaming](/docs/es/build-with-claude/streaming) vía SSE, es posible que ocurra un error después de devolver una respuesta 200, en cuyo caso el manejo de errores no seguiría estos mecanismos estándar.

## Límites de tamaño de solicitud

La API impone límites de tamaño de solicitud para asegurar un rendimiento óptimo:

| Tipo de Endpoint | Tamaño Máximo de Solicitud |
|:---|:---|
| Messages API | 32 MB |
| Token Counting API | 32 MB |
| [Batch API](/docs/es/build-with-claude/batch-processing) | 256 MB |
| [Files API](/docs/es/build-with-claude/files) | 500 MB |

Si excedes estos límites, recibirás un error 413 `request_too_large`. El error es devuelto desde Cloudflare antes de que la solicitud llegue a nuestros servidores API.

## Formas de error

Los errores siempre se devuelven como JSON, con un objeto `error` de nivel superior que siempre incluye un valor `type` y `message`. La respuesta también incluye un campo `request_id` para facilitar el seguimiento y depuración. Por ejemplo:

```json JSON
{
  "type": "error",
  "error": {
    "type": "not_found_error",
    "message": "The requested resource could not be found."
  },
  "request_id": "req_011CSHoEeqs5C35K2UUqR7Fy"
}
```

De acuerdo con nuestra política de [versionado](/docs/es/api/versioning), podemos expandir los valores dentro de estos objetos, y es posible que los valores `type` crezcan con el tiempo.

## ID de solicitud

Cada respuesta de la API incluye un encabezado único `request-id`. Este encabezado contiene un valor como `req_018EeWyXxfu5pfWkrYcMdjWG`. Al contactar soporte sobre una solicitud específica, por favor incluye este ID para ayudarnos a resolver rápidamente tu problema.

Nuestros SDKs oficiales proporcionan este valor como una propiedad en objetos de respuesta de nivel superior, conteniendo el valor del encabezado `request-id`:

<CodeGroup>
  ```python Python
  import anthropic

  client = anthropic.Anthropic()

  message = client.messages.create(
      model="claude-sonnet-4-5",
      max_tokens=1024,
      messages=[
          {"role": "user", "content": "Hello, Claude"}
      ]
  )
  print(f"Request ID: {message._request_id}")
  ```

  ```typescript TypeScript
  import Anthropic from '@anthropic-ai/sdk';

  const client = new Anthropic();

  const message = await client.messages.create({
    model: 'claude-sonnet-4-5',
    max_tokens: 1024,
    messages: [
      {"role": "user", "content": "Hello, Claude"}
    ]
  });
  console.log('Request ID:', message._request_id);
  ```
</CodeGroup>

## Solicitudes largas

<Warning>
 Recomendamos encarecidamente usar la [streaming Messages API](/docs/es/build-with-claude/streaming) o [Message Batches API](/docs/es/api/creating-message-batches) para solicitudes de larga duración, especialmente aquellas de más de 10 minutos.
</Warning>

No recomendamos establecer valores grandes de `max_tokens` sin usar nuestra [streaming Messages API](/docs/es/build-with-claude/streaming)
o [Message Batches API](/docs/es/api/creating-message-batches):

- Algunas redes pueden descartar conexiones inactivas después de un período variable de tiempo, lo que
puede causar que la solicitud falle o expire sin recibir una respuesta de Anthropic.
- Las redes difieren en confiabilidad; nuestra [Message Batches API](/docs/es/api/creating-message-batches) puede ayudarte a
gestionar el riesgo de problemas de red permitiéndote consultar resultados en lugar de requerir una conexión de red ininterrumpida.

Si estás construyendo una integración directa de API, debes estar consciente de que establecer un [TCP socket keep-alive](https://tldp.org/HOWTO/TCP-Keepalive-HOWTO/programming.html) puede reducir el impacto de timeouts de conexión inactiva en algunas redes.

Nuestros [SDKs](/docs/es/api/client-sdks) validarán que tus solicitudes de Messages API sin streaming no se espera que excedan un timeout de 10 minutos y
también establecerán una opción de socket para TCP keep-alive.