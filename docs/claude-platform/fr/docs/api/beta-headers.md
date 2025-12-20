# En-têtes bêta

Documentation pour l'utilisation des en-têtes bêta avec l'API Claude

---

Les en-têtes bêta vous permettent d'accéder aux fonctionnalités expérimentales et aux nouvelles capacités de modèle avant qu'elles ne fassent partie de l'API standard.

Ces fonctionnalités sont sujettes à modification et peuvent être modifiées ou supprimées dans les versions futures.

<Info>
Les en-têtes bêta sont souvent utilisés en conjonction avec l'[espace de noms bêta dans les SDK clients](/docs/fr/api/client-sdks#beta-namespace-in-client-sdks)
</Info>

## Comment utiliser les en-têtes bêta

Pour accéder aux fonctionnalités bêta, incluez l'en-tête `anthropic-beta` dans vos requêtes API :

```http
POST /v1/messages
Content-Type: application/json
X-API-Key: YOUR_API_KEY
anthropic-beta: BETA_FEATURE_NAME
```

Lors de l'utilisation du SDK, vous pouvez spécifier les en-têtes bêta dans les options de requête :

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
Les fonctionnalités bêta sont expérimentales et peuvent :
- Avoir des changements cassants sans préavis
- Être dépréciées ou supprimées
- Avoir des limites de taux ou des tarifs différents
- Ne pas être disponibles dans toutes les régions
</Warning>

### Plusieurs fonctionnalités bêta

Pour utiliser plusieurs fonctionnalités bêta dans une seule requête, incluez tous les noms de fonctionnalités dans l'en-tête séparés par des virgules :

```http
anthropic-beta: feature1,feature2,feature3
```

### Conventions de nommage des versions

Les noms des fonctionnalités bêta suivent généralement le modèle : `feature-name-YYYY-MM-DD`, où la date indique quand la version bêta a été publiée. Utilisez toujours le nom exact de la fonctionnalité bêta tel que documenté.

## Gestion des erreurs

Si vous utilisez un en-tête bêta invalide ou indisponible, vous recevrez une réponse d'erreur :

```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "Unsupported beta header: invalid-beta-name"
  }
}
```

## Obtenir de l'aide

Pour des questions sur les fonctionnalités bêta :

1. Vérifiez la documentation pour la fonctionnalité spécifique
2. Consultez le [journal des modifications de l'API](/docs/fr/api/versioning) pour les mises à jour
3. Contactez le support pour obtenir de l'aide avec l'utilisation en production

Rappelez-vous que les fonctionnalités bêta sont fournies "en l'état" et peuvent ne pas avoir les mêmes garanties SLA que les fonctionnalités API stables.