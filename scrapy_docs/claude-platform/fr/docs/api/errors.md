# Erreurs

---

## Erreurs HTTP

Notre API suit un format de code d'erreur HTTP prévisible :

* 400 - `invalid_request_error` : Il y a eu un problème avec le format ou le contenu de votre requête. Nous pouvons également utiliser ce type d'erreur pour d'autres codes de statut 4XX non listés ci-dessous.
* 401 - `authentication_error` : Il y a un problème avec votre clé API.
* 403 - `permission_error` : Votre clé API n'a pas la permission d'utiliser la ressource spécifiée.
* 404 - `not_found_error` : La ressource demandée n'a pas été trouvée.
* 413 - `request_too_large` : La requête dépasse le nombre maximum d'octets autorisé. La taille maximale de requête est de 32 MB pour les points de terminaison API standard.
* 429 - `rate_limit_error` : Votre compte a atteint une limite de taux.
* 500 - `api_error` : Une erreur inattendue s'est produite à l'interne des systèmes d'Anthropic.
* 529 - `overloaded_error` : L'API est temporairement surchargée.

  <Warning>
  Les erreurs 529 peuvent se produire lorsque les API connaissent un trafic élevé pour tous les utilisateurs.
  
  Dans de rares cas, si votre organisation a une augmentation soudaine de l'utilisation, vous pourriez voir des erreurs 429 dues aux limites d'accélération sur l'API. Pour éviter d'atteindre les limites d'accélération, augmentez votre trafic progressivement et maintenez des modèles d'utilisation cohérents.
  </Warning>

Lors de la réception d'une réponse en [streaming](/docs/fr/build-with-claude/streaming) via SSE, il est possible qu'une erreur se produise après avoir retourné une réponse 200, auquel cas la gestion des erreurs ne suivrait pas ces mécanismes standard.

## Limites de taille de requête

L'API applique des limites de taille de requête pour assurer des performances optimales :

| Type de Point de Terminaison | Taille Maximale de Requête |
|:---|:---|
| API Messages | 32 MB |
| API de Comptage de Jetons | 32 MB |
| [API Batch](/docs/fr/build-with-claude/batch-processing) | 256 MB |
| [API Files](/docs/fr/build-with-claude/files) | 500 MB |

Si vous dépassez ces limites, vous recevrez une erreur 413 `request_too_large`. L'erreur est retournée par Cloudflare avant que la requête n'atteigne nos serveurs API.

## Formes d'erreur

Les erreurs sont toujours retournées en JSON, avec un objet `error` de niveau supérieur qui inclut toujours une valeur `type` et `message`. La réponse inclut également un champ `request_id` pour un suivi et un débogage plus faciles. Par exemple :

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

Conformément à notre politique de [versioning](/docs/fr/api/versioning), nous pouvons étendre les valeurs dans ces objets, et il est possible que les valeurs `type` augmentent avec le temps.

## ID de requête

Chaque réponse API inclut un en-tête unique `request-id`. Cet en-tête contient une valeur telle que `req_018EeWyXxfu5pfWkrYcMdjWG`. Lorsque vous contactez le support concernant une requête spécifique, veuillez inclure cet ID pour nous aider à résoudre rapidement votre problème.

Nos SDK officiels fournissent cette valeur comme propriété sur les objets de réponse de niveau supérieur, contenant la valeur de l'en-tête `request-id` :

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

## Requêtes longues

<Warning>
 Nous encourageons fortement l'utilisation de l'[API Messages en streaming](/docs/fr/build-with-claude/streaming) ou de l'[API Message Batches](/docs/fr/api/creating-message-batches) pour les requêtes de longue durée, en particulier celles de plus de 10 minutes.
</Warning>

Nous ne recommandons pas de définir de grandes valeurs `max_tokens` sans utiliser notre [API Messages en streaming](/docs/fr/build-with-claude/streaming)
ou l'[API Message Batches](/docs/fr/api/creating-message-batches) :

- Certains réseaux peuvent abandonner les connexions inactives après une période de temps variable, ce qui
peut causer l'échec ou l'expiration de la requête sans recevoir de réponse d'Anthropic.
- Les réseaux diffèrent en fiabilité ; notre [API Message Batches](/docs/fr/api/creating-message-batches) peut vous aider à
gérer le risque de problèmes de réseau en vous permettant d'interroger les résultats plutôt que d'exiger une connexion réseau ininterrompue.

Si vous construisez une intégration API directe, vous devriez être conscient que définir un [TCP socket keep-alive](https://tldp.org/HOWTO/TCP-Keepalive-HOWTO/programming.html) peut réduire l'impact des délais d'expiration de connexion inactive sur certains réseaux.

Nos [SDK](/docs/fr/api/client-sdks) valideront que vos requêtes API Messages non-streaming ne sont pas censées dépasser un délai d'expiration de 10 minutes et
définiront également une option de socket pour TCP keep-alive.