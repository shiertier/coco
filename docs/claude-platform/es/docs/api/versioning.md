# Versiones

Al realizar solicitudes de API, debes enviar un encabezado de solicitud `anthropic-version`. Por ejemplo, `anthropic-version: 2023-06-01`. Si estás usando nuestros [SDKs de cliente](/docs/es/api/client-sdks), esto se maneja automáticamente para ti.

---

Para cualquier versión de API dada, preservaremos:

* Parámetros de entrada existentes
* Parámetros de salida existentes

Sin embargo, podemos hacer lo siguiente:

* Agregar entradas opcionales adicionales
* Agregar valores adicionales a la salida
* Cambiar condiciones para tipos de error específicos
* Agregar nuevas variantes a valores de salida tipo enum (por ejemplo, tipos de eventos de streaming)

Generalmente, si estás usando la API como se documenta en esta referencia, no romperemos tu uso.

## Historial de versiones

Siempre recomendamos usar la versión más reciente de la API cuando sea posible. Las versiones anteriores se consideran obsoletas y pueden no estar disponibles para nuevos usuarios.

* `2023-06-01`  
   * Nuevo formato para eventos enviados por el servidor (SSE) de [streaming](/docs/es/api/streaming):  
         * Las completaciones son incrementales. Por ejemplo, `" Hola"`, `" mi"`, `" nombre"`, `" es"`, `" Claude." ` en lugar de `" Hola"`, `" Hola mi"`, `" Hola mi nombre"`, `" Hola mi nombre es"`, `" Hola mi nombre es Claude."`.  
         * Todos los eventos son [eventos nombrados](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#named%5Fevents), en lugar de [eventos solo de datos](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#data-only%5Fmessages).  
         * Se eliminó el evento innecesario `data: [DONE]`.  
   * Se eliminaron los valores heredados `exception` y `truncated` en las respuestas.
* `2023-01-01`: Lanzamiento inicial.