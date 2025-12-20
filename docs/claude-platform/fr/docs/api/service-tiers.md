# Niveaux de service

Différents niveaux de service vous permettent d'équilibrer la disponibilité, les performances et les coûts prévisibles en fonction des besoins de votre application.

---

Nous proposons trois niveaux de service :
- **Niveau Prioritaire :** Idéal pour les workflows déployés en production où le temps, la disponibilité et les tarifs prévisibles sont importants
- **Standard :** Niveau de service par défaut pour les tests pilotes et la mise à l'échelle des cas d'usage quotidiens
- **Batch :** Idéal pour les workflows asynchrones qui peuvent attendre ou bénéficier d'être en dehors de votre capacité normale

## Niveau Standard

Le niveau standard est le niveau de service par défaut pour toutes les demandes d'API. Les demandes dans ce niveau sont priorisées aux côtés de toutes les autres demandes et observent une disponibilité au mieux de nos efforts.

## Niveau Prioritaire

Les demandes dans ce niveau sont priorisées par rapport à toutes les autres demandes adressées à Anthropic. Cette priorisation aide à minimiser les erreurs [« serveur surchargé »](/docs/fr/api/errors#http-errors), même pendant les heures de pointe.

Pour plus d'informations, consultez [Commencer avec le Niveau Prioritaire](#get-started-with-priority-tier)

## Comment les demandes se voient attribuer des niveaux

Lors du traitement d'une demande, Anthropic décide d'attribuer une demande au Niveau Prioritaire dans les scénarios suivants :
- Votre organisation dispose d'une capacité de niveau prioritaire suffisante en **tokens d'entrée** par minute
- Votre organisation dispose d'une capacité de niveau prioritaire suffisante en **tokens de sortie** par minute

Anthropic compte l'utilisation par rapport à la capacité du Niveau Prioritaire comme suit :

**Tokens d'entrée**
- Les lectures de cache comptent comme 0,1 token par token lu du cache
- Les écritures de cache comptent comme 1,25 token par token écrit dans le cache avec un TTL de 5 minutes
- Les écritures de cache comptent comme 2,00 tokens par token écrit dans le cache avec un TTL d'1 heure
- Pour les demandes [long-context](/docs/fr/build-with-claude/context-windows) (>200k tokens d'entrée), les tokens d'entrée comptent comme 2 tokens par token
- Tous les autres tokens d'entrée comptent comme 1 token par token

**Tokens de sortie**
- Pour les demandes [long-context](/docs/fr/build-with-claude/context-windows) (>200k tokens d'entrée), les tokens de sortie comptent comme 1,5 token par token
- Tous les autres tokens de sortie comptent comme 1 token par token

Sinon, les demandes procèdent au niveau standard.

<Note>
Les demandes attribuées au Niveau Prioritaire tirent à la fois de la capacité du Niveau Prioritaire et des limites de débit régulières.
Si le traitement de la demande dépasserait les limites de débit, la demande est refusée.
</Note>

## Utilisation des niveaux de service

Vous pouvez contrôler quels niveaux de service peuvent être utilisés pour une demande en définissant le paramètre `service_tier` :

```python
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello, Claude!"}],
    service_tier="auto"  # Automatically use Priority Tier when available, fallback to standard
)
```

Le paramètre `service_tier` accepte les valeurs suivantes :

- `"auto"` (par défaut) - Utilise la capacité du Niveau Prioritaire si disponible, sinon revient à votre autre capacité
- `"standard_only"` - Utilise uniquement la capacité du niveau standard, utile si vous ne voulez pas utiliser votre capacité du Niveau Prioritaire

L'objet `usage` de la réponse inclut également le niveau de service attribué à la demande :

```json
{
  "usage": {
    "input_tokens": 410,
    "cache_creation_input_tokens": 0,
    "cache_read_input_tokens": 0,
    "output_tokens": 585,
    "service_tier": "priority"
  }
}
```
Cela vous permet de déterminer quel niveau de service a été attribué à la demande.

Lors de la demande de `service_tier="auto"` avec un modèle ayant un engagement de Niveau Prioritaire, ces en-têtes de réponse fournissent des informations :
```
anthropic-priority-input-tokens-limit: 10000
anthropic-priority-input-tokens-remaining: 9618
anthropic-priority-input-tokens-reset: 2025-01-12T23:11:59Z
anthropic-priority-output-tokens-limit: 10000
anthropic-priority-output-tokens-remaining: 6000
anthropic-priority-output-tokens-reset: 2025-01-12T23:12:21Z
```
Vous pouvez utiliser la présence de ces en-têtes pour détecter si votre demande était éligible au Niveau Prioritaire, même si elle dépassait la limite.

## Commencer avec le Niveau Prioritaire

Vous voudrez peut-être vous engager dans la capacité du Niveau Prioritaire si vous êtes intéressé par :
- **Disponibilité plus élevée** : Cible 99,5 % de disponibilité avec des ressources informatiques priorisées
- **Contrôle des coûts** : Dépenses prévisibles et réductions pour les engagements plus longs
- **Débordement flexible** : Revient automatiquement au niveau standard lorsque vous dépassez votre capacité engagée

S'engager dans le Niveau Prioritaire impliquera de décider :
- Un nombre de tokens d'entrée par minute
- Un nombre de tokens de sortie par minute
- Une durée d'engagement (1, 3, 6 ou 12 mois)
- Une version de modèle spécifique

<Note>
Le ratio de tokens d'entrée à tokens de sortie que vous achetez est important. Dimensionner votre capacité du Niveau Prioritaire pour l'aligner avec vos modèles de trafic réels vous aide à maximiser l'utilisation de vos tokens achetés.
</Note>

### Modèles supportés

Le Niveau Prioritaire est supporté par :

- Claude Opus 4.5
- Claude Sonnet 4.5
- Claude Haiku 4.5
- Claude Opus 4.1
- Claude Opus 4
- Claude Sonnet 4
- Claude Sonnet 3.7 ([deprecated](/docs/fr/about-claude/model-deprecations))
- Claude Haiku 3.5 ([deprecated](/docs/fr/about-claude/model-deprecations))

Consultez la [page d'aperçu des modèles](/docs/fr/about-claude/models/overview) pour plus de détails sur nos modèles.

### Comment accéder au Niveau Prioritaire

Pour commencer à utiliser le Niveau Prioritaire :

1. [Contactez les ventes](https://claude.com/contact-sales/priority-tier) pour terminer l'approvisionnement
2. (Optionnel) Mettez à jour vos demandes d'API pour définir optionnellement le paramètre `service_tier` sur `auto`
3. Surveillez votre utilisation via les en-têtes de réponse et la Console Claude