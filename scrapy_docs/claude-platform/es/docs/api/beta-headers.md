# Encabezados beta

Documentación para usar encabezados beta con la API de Claude

---

Los encabezados beta te permiten acceder a características experimentales y nuevas capacidades de modelo antes de que se conviertan en parte de la API estándar.

Estas características están sujetas a cambios y pueden ser modificadas o eliminadas en versiones futuras.

<Info>
Los encabezados beta se usan a menudo en conjunto con el [espacio de nombres beta en los SDKs del cliente](/docs/es/api/client-sdks#beta-namespace-in-client-sdks)
</Info>

## Cómo usar encabezados beta

Para acceder a características beta, incluye el encabezado `anthropic-beta` en tus solicitudes de API:

```http
POST /v1/messages
Content-Type: application/json
X-API-Key: YOUR_API_KEY
anthropic-beta: BETA_FEATURE_NAME
```

Cuando uses el SDK, puedes especificar encabezados beta en las opciones de solicitud:

<CodeGroup>

```python Python
from anthropic import Anthropic

client = Anthropic()

response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[
        {"role": "user", "content": "Hello, Claude"}
    ],
    betas=["beta-feature-name"]
)
```

```typescript TypeScript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

const msg = await anthropic.beta.messages.create({
  model: 'claude-sonnet-4-5',
  max_tokens: 1024,
  messages: [
    { role: 'user', content: 'Hello, Claude' }
  ],
  betas: ['beta-feature-name']
});
```

```bash cURL
curl https://api.anthropic.com/v1/messages \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "anthropic-beta: beta-feature-name" \
  -H "content-type: application/json" \
  -d '{
    "model": "claude-sonnet-4-5",
    "max_tokens": 1024,
    "messages": [
      {"role": "user", "content": "Hello, Claude"}
    ]
  }'
```

</CodeGroup>

<Warning>
Las características beta son experimentales y pueden:
- Tener cambios disruptivos sin aviso
- Ser obsoletas o eliminadas
- Tener diferentes límites de velocidad o precios
- No estar disponibles en todas las regiones
</Warning>

### Múltiples características beta

Para usar múltiples características beta en una sola solicitud, incluye todos los nombres de características en el encabezado separados por comas:

```http
anthropic-beta: feature1,feature2,feature3
```

### Convenciones de nomenclatura de versiones

Los nombres de características beta típicamente siguen el patrón: `feature-name-YYYY-MM-DD`, donde la fecha indica cuándo fue lanzada la versión beta. Siempre usa el nombre exacto de la característica beta como está documentado.

## Manejo de errores

Si usas un encabezado beta inválido o no disponible, recibirás una respuesta de error:

```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "Unsupported beta header: invalid-beta-name"
  }
}
```

## Obtener ayuda

Para preguntas sobre características beta:

1. Revisa la documentación para la característica específica
2. Revisa el [registro de cambios de la API](/docs/es/api/versioning) para actualizaciones
3. Contacta soporte para asistencia con uso en producción

Recuerda que las características beta se proporcionan "tal como están" y pueden no tener las mismas garantías de SLA que las características estables de la API.