# Header beta

Documentazione per l'utilizzo degli header beta con l'API Claude

---

Gli header beta ti permettono di accedere a funzionalità sperimentali e nuove capacità del modello prima che diventino parte dell'API standard.

Queste funzionalità sono soggette a modifiche e potrebbero essere modificate o rimosse nelle versioni future.

<Info>
Gli header beta sono spesso utilizzati insieme al [namespace beta negli SDK client](/docs/it/api/client-sdks#beta-namespace-in-client-sdks)
</Info>

## Come utilizzare gli header beta

Per accedere alle funzionalità beta, includi l'header `anthropic-beta` nelle tue richieste API:

```http
POST /v1/messages
Content-Type: application/json
X-API-Key: YOUR_API_KEY
anthropic-beta: BETA_FEATURE_NAME
```

Quando utilizzi l'SDK, puoi specificare gli header beta nelle opzioni della richiesta:

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
Le funzionalità beta sono sperimentali e potrebbero:
- Avere modifiche che interrompono la compatibilità senza preavviso
- Essere deprecate o rimosse
- Avere limiti di velocità o prezzi diversi
- Non essere disponibili in tutte le regioni
</Warning>

### Funzionalità beta multiple

Per utilizzare più funzionalità beta in una singola richiesta, includi tutti i nomi delle funzionalità nell'header separati da virgole:

```http
anthropic-beta: feature1,feature2,feature3
```

### Convenzioni di denominazione delle versioni

I nomi delle funzionalità beta seguono tipicamente il pattern: `feature-name-YYYY-MM-DD`, dove la data indica quando è stata rilasciata la versione beta. Utilizza sempre il nome esatto della funzionalità beta come documentato.

## Gestione degli errori

Se utilizzi un header beta non valido o non disponibile, riceverai una risposta di errore:

```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "Unsupported beta header: invalid-beta-name"
  }
}
```

## Ottenere aiuto

Per domande sulle funzionalità beta:

1. Controlla la documentazione per la funzionalità specifica
2. Rivedi il [changelog dell'API](/docs/it/api/versioning) per gli aggiornamenti
3. Contatta il supporto per assistenza con l'utilizzo in produzione

Ricorda che le funzionalità beta sono fornite "così come sono" e potrebbero non avere le stesse garanzie SLA delle funzionalità API stabili.