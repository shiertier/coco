# Beta-Header

Dokumentation für die Verwendung von Beta-Headern mit der Claude API

---

Beta-Header ermöglichen Ihnen den Zugriff auf experimentelle Funktionen und neue Modellfähigkeiten, bevor sie Teil der Standard-API werden.

Diese Funktionen können sich ändern und in zukünftigen Versionen modifiziert oder entfernt werden.

<Info>
Beta-Header werden oft in Verbindung mit dem [Beta-Namespace in den Client-SDKs](/docs/de/api/client-sdks#beta-namespace-in-client-sdks) verwendet
</Info>

## Wie man Beta-Header verwendet

Um auf Beta-Funktionen zuzugreifen, fügen Sie den `anthropic-beta`-Header in Ihre API-Anfragen ein:

```http
POST /v1/messages
Content-Type: application/json
X-API-Key: YOUR_API_KEY
anthropic-beta: BETA_FEATURE_NAME
```

Bei Verwendung des SDK können Sie Beta-Header in den Anfrage-Optionen angeben:

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
Beta-Funktionen sind experimentell und können:
- Breaking Changes ohne Vorankündigung haben
- Veraltet oder entfernt werden
- Unterschiedliche Rate-Limits oder Preise haben
- Nicht in allen Regionen verfügbar sein
</Warning>

### Mehrere Beta-Funktionen

Um mehrere Beta-Funktionen in einer einzigen Anfrage zu verwenden, fügen Sie alle Funktionsnamen im Header durch Kommas getrennt ein:

```http
anthropic-beta: feature1,feature2,feature3
```

### Versionsnamenkonventionen

Beta-Funktionsnamen folgen typischerweise dem Muster: `feature-name-YYYY-MM-DD`, wobei das Datum angibt, wann die Beta-Version veröffentlicht wurde. Verwenden Sie immer den exakten Beta-Funktionsnamen wie dokumentiert.

## Fehlerbehandlung

Wenn Sie einen ungültigen oder nicht verfügbaren Beta-Header verwenden, erhalten Sie eine Fehlerantwort:

```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "Unsupported beta header: invalid-beta-name"
  }
}
```

## Hilfe erhalten

Für Fragen zu Beta-Funktionen:

1. Überprüfen Sie die Dokumentation für die spezifische Funktion
2. Schauen Sie sich das [API-Changelog](/docs/de/api/versioning) für Updates an
3. Kontaktieren Sie den Support für Unterstützung bei der Produktionsnutzung

Denken Sie daran, dass Beta-Funktionen "wie sie sind" bereitgestellt werden und möglicherweise nicht die gleichen SLA-Garantien wie stabile API-Funktionen haben.