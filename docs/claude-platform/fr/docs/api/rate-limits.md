# Limites de débit

Pour atténuer les abus et gérer la capacité de notre API, nous avons mis en place des limites sur la façon dont une organisation peut utiliser l'API Claude.

---

Nous avons deux types de limites :

1. **Les limites de dépenses** définissent un coût mensuel maximum qu'une organisation peut engager pour l'utilisation de l'API.
2. **Les limites de débit** définissent le nombre maximum de requêtes API qu'une organisation peut faire sur une période définie.

Nous appliquons les limites configurées par le service au niveau de l'organisation, mais vous pouvez également définir des limites configurables par l'utilisateur pour les espaces de travail de votre organisation.

Ces limites s'appliquent à la fois à l'utilisation du niveau Standard et du niveau Priority. Pour plus d'informations sur le niveau Priority, qui offre des niveaux de service améliorés en échange d'une dépense engagée, consultez [Service Tiers](/docs/fr/api/service-tiers).

## À propos de nos limites

* Les limites sont conçues pour prévenir les abus de l'API, tout en minimisant l'impact sur les modèles d'utilisation courants des clients.
* Les limites sont définies par **niveau d'utilisation**, où chaque niveau est associé à un ensemble différent de limites de dépenses et de débit.
* Votre organisation augmentera automatiquement les niveaux à mesure que vous atteindrez certains seuils lors de l'utilisation de l'API.
  Les limites sont définies au niveau de l'organisation. Vous pouvez voir les limites de votre organisation sur la [page Limites](/settings/limits) dans la [Console Claude](/).
* Vous pouvez atteindre les limites de débit sur des intervalles de temps plus courts. Par exemple, un débit de 60 requêtes par minute (RPM) peut être appliqué comme 1 requête par seconde. Les courtes rafales de requêtes à un volume élevé peuvent dépasser la limite de débit et entraîner des erreurs de limite de débit.
* Les limites décrites ci-dessous sont nos limites de niveau standard. Si vous recherchez des limites plus élevées et personnalisées ou le niveau Priority pour des niveaux de service améliorés, contactez les ventes via la [Console Claude](/settings/limits).
* Nous utilisons l'[algorithme du seau de jetons](https://en.wikipedia.org/wiki/Token_bucket) pour faire la limitation de débit. Cela signifie que votre capacité est continuellement reconstituée jusqu'à votre limite maximale, plutôt que d'être réinitialisée à des intervalles fixes.
* Toutes les limites décrites ici représentent l'utilisation maximale autorisée, pas les minimums garantis. Ces limites sont destinées à réduire les dépenses excessives involontaires et à assurer une distribution équitable des ressources entre les utilisateurs.

## Limites de dépenses

Chaque niveau d'utilisation a une limite sur le montant que vous pouvez dépenser sur l'API chaque mois calendaire. Une fois que vous atteindrez la limite de dépenses de votre niveau, jusqu'à ce que vous soyez admissible au niveau suivant, vous devrez attendre le mois suivant pour pouvoir utiliser l'API à nouveau.

Pour être admissible au niveau suivant, vous devez respecter une exigence de dépôt. Pour minimiser le risque de surfinancement de votre compte, vous ne pouvez pas déposer plus que votre limite de dépenses mensuelle.

### Exigences pour avancer de niveau
<table>
  <thead>
    <tr>
      <th>Niveau d'utilisation</th>
      <th>Achat de crédits</th>
      <th>Achat de crédits maximum</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Niveau 1</td>
      <td>\$5</td>
      <td>\$100</td>
    </tr>
    <tr>
      <td>Niveau 2</td>
      <td>\$40</td>
      <td>\$500</td>
    </tr>
    <tr>
      <td>Niveau 3</td>
      <td>\$200</td>
      <td>\$1 000</td>
    </tr>
    <tr>
      <td>Niveau 4</td>
      <td>\$400</td>
      <td>\$5 000</td>
    </tr>
    <tr>
      <td>Facturation mensuelle</td>
      <td>S/O</td>
      <td>S/O</td>
    </tr>
  </tbody>
</table>

<Note>
**Achat de crédits** affiche les achats de crédits cumulatifs (hors taxes) requis pour avancer à ce niveau. Vous avancez immédiatement après avoir atteint le seuil.

**Achat de crédits maximum** limite le montant maximum que vous pouvez ajouter à votre compte en une seule transaction pour éviter le surfinancement du compte.
</Note>

## Limites de débit

Nos limites de débit pour l'API Messages sont mesurées en requêtes par minute (RPM), jetons d'entrée par minute (ITPM) et jetons de sortie par minute (OTPM) pour chaque classe de modèle.
Si vous dépassez l'une des limites de débit, vous recevrez une [erreur 429](/docs/fr/api/errors) décrivant quelle limite de débit a été dépassée, ainsi qu'un en-tête `retry-after` indiquant combien de temps attendre.

<Note>
Vous pouvez également rencontrer des erreurs 429 en raison des limites d'accélération sur l'API si votre organisation connaît une augmentation nette de l'utilisation. Pour éviter de atteindre les limites d'accélération, augmentez progressivement votre trafic et maintenez des modèles d'utilisation cohérents.
</Note>

### ITPM conscient du cache

De nombreux fournisseurs d'API utilisent une limite combinée de « jetons par minute » (TPM) qui peut inclure tous les jetons, à la fois mis en cache et non mis en cache, entrée et sortie. **Pour la plupart des modèles Claude, seuls les jetons d'entrée non mis en cache comptent vers vos limites de débit ITPM.** C'est un avantage clé qui rend nos limites de débit effectivement plus élevées qu'elles ne pourraient le paraître initialement.

Les limites de débit ITPM sont estimées au début de chaque requête, et l'estimation est ajustée pendant la requête pour refléter le nombre réel de jetons d'entrée utilisés.

Voici ce qui compte vers ITPM :
- `input_tokens` (jetons après le dernier point de rupture du cache) ✓ **Comptent vers ITPM**
- `cache_creation_input_tokens` (jetons en cours d'écriture dans le cache) ✓ **Comptent vers ITPM**
- `cache_read_input_tokens` (jetons lus à partir du cache) ✗ **Ne comptent PAS vers ITPM** pour la plupart des modèles

<Note>
Le champ `input_tokens` ne représente que les jetons qui apparaissent **après votre dernier point de rupture du cache**, pas tous les jetons d'entrée de votre requête. Pour calculer le total des jetons d'entrée :

```
total_input_tokens = cache_read_input_tokens + cache_creation_input_tokens + input_tokens
```

Cela signifie que lorsque vous avez du contenu mis en cache, `input_tokens` sera généralement beaucoup plus petit que votre entrée totale. Par exemple, avec un document mis en cache de 200 000 jetons et une question utilisateur de 50 jetons, vous verriez `input_tokens: 50` même si l'entrée totale est de 200 050 jetons.

À des fins de limite de débit sur la plupart des modèles, seuls `input_tokens` + `cache_creation_input_tokens` comptent vers votre limite ITPM, ce qui rend la [mise en cache des invites](/docs/fr/build-with-claude/prompt-caching) un moyen efficace d'augmenter votre débit effectif.
</Note>

**Exemple** : Avec une limite ITPM de 2 000 000 et un taux de succès du cache de 80 %, vous pourriez effectivement traiter 10 000 000 jetons d'entrée totaux par minute (2 M non mis en cache + 8 M mis en cache), puisque les jetons mis en cache ne comptent pas vers votre limite de débit.

<Note>
Certains modèles plus anciens (marqués avec † dans les tableaux de limites de débit ci-dessous) comptent également `cache_read_input_tokens` vers les limites de débit ITPM.

Pour tous les modèles sans le marqueur †, les jetons d'entrée mis en cache ne comptent pas vers les limites de débit et sont facturés à un taux réduit (10 % du prix du jeton d'entrée de base). Cela signifie que vous pouvez atteindre un débit effectif considérablement plus élevé en utilisant la [mise en cache des invites](/docs/fr/build-with-claude/prompt-caching).
</Note>

<Tip>
**Maximisez vos limites de débit avec la mise en cache des invites**

Pour tirer le meilleur parti de vos limites de débit, utilisez la [mise en cache des invites](/docs/fr/build-with-claude/prompt-caching) pour le contenu répété comme :
- Instructions système et invites
- Documents de contexte volumineux
- Définitions d'outils
- Historique de conversation

Avec une mise en cache efficace, vous pouvez augmenter considérablement votre débit réel sans augmenter vos limites de débit. Surveillez votre taux de succès du cache sur la [page Utilisation](/settings/usage) pour optimiser votre stratégie de mise en cache.
</Tip>

Les limites de débit OTPM sont estimées en fonction de `max_tokens` au début de chaque requête, et l'estimation est ajustée à la fin de la requête pour refléter le nombre réel de jetons de sortie utilisés.
Si vous atteignez les limites OTPM plus tôt que prévu, essayez de réduire `max_tokens` pour mieux approximer la taille de vos complétions.

Les limites de débit sont appliquées séparément pour chaque modèle ; par conséquent, vous pouvez utiliser différents modèles jusqu'à leurs limites respectives simultanément.
Vous pouvez vérifier vos limites de débit actuelles et le comportement dans la [Console Claude](/settings/limits).

<Note>
Pour les requêtes de contexte long (>200 000 jetons) lors de l'utilisation de l'en-tête bêta `context-1m-2025-08-07` avec Claude Sonnet 4.x, des limites de débit distinctes s'appliquent. Voir [Limites de débit de contexte long](#long-context-rate-limits) ci-dessous.
</Note>

<Tabs>
<Tab title="Niveau 1">
| Modèle                                                                                       | Requêtes maximales par minute (RPM) | Jetons d'entrée maximaux par minute (ITPM) | Jetons de sortie maximaux par minute (OTPM) |
| -------------------------------------------------------------------------------------------- | ----------------------------------- | ------------------------------------------ | ------------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 50                                  | 30 000                                     | 8 000                                       |
| Claude Sonnet 3.7 ([obsolète](/docs/fr/about-claude/model-deprecations))                     | 50                                  | 20 000                                     | 8 000                                       |
| Claude Haiku 4.5                                                                             | 50                                  | 50 000                                     | 10 000                                      |
| Claude Haiku 3.5 ([obsolète](/docs/fr/about-claude/model-deprecations))                      | 50                                  | 50 000<sup>†</sup>                         | 10 000                                      |
| Claude Haiku 3                                                                               | 50                                  | 50 000<sup>†</sup>                         | 10 000                                      |
| Claude Opus 4.x<sup>*</sup>                                                                  | 50                                  | 30 000                                     | 8 000                                       |
| Claude Opus 3 ([obsolète](/docs/fr/about-claude/model-deprecations))                        | 50                                  | 20 000<sup>†</sup>                         | 4 000                                       |

</Tab>
<Tab title="Niveau 2">
| Modèle                                                                                       | Requêtes maximales par minute (RPM) | Jetons d'entrée maximaux par minute (ITPM) | Jetons de sortie maximaux par minute (OTPM) |
| -------------------------------------------------------------------------------------------- | ----------------------------------- | ------------------------------------------ | ------------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 1 000                               | 450 000                                    | 90 000                                      |
| Claude Sonnet 3.7 ([obsolète](/docs/fr/about-claude/model-deprecations))                     | 1 000                               | 40 000                                     | 16 000                                      |
| Claude Haiku 4.5                                                                             | 1 000                               | 450 000                                    | 90 000                                      |
| Claude Haiku 3.5 ([obsolète](/docs/fr/about-claude/model-deprecations))                      | 1 000                               | 100 000<sup>†</sup>                        | 20 000                                      |
| Claude Haiku 3                                                                               | 1 000                               | 100 000<sup>†</sup>                        | 20 000                                      |
| Claude Opus 4.x<sup>*</sup>                                                                  | 1 000                               | 450 000                                    | 90 000                                      |
| Claude Opus 3 ([obsolète](/docs/fr/about-claude/model-deprecations))                        | 1 000                               | 40 000<sup>†</sup>                         | 8 000                                       |

</Tab>
<Tab title="Niveau 3">
| Modèle                                                                                       | Requêtes maximales par minute (RPM) | Jetons d'entrée maximaux par minute (ITPM) | Jetons de sortie maximaux par minute (OTPM) |
| -------------------------------------------------------------------------------------------- | ----------------------------------- | ------------------------------------------ | ------------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 2 000                               | 800 000                                    | 160 000                                     |
| Claude Sonnet 3.7 ([obsolète](/docs/fr/about-claude/model-deprecations))                     | 2 000                               | 80 000                                     | 32 000                                      |
| Claude Haiku 4.5                                                                             | 2 000                               | 1 000 000                                  | 200 000                                     |
| Claude Haiku 3.5 ([obsolète](/docs/fr/about-claude/model-deprecations))                      | 2 000                               | 200 000<sup>†</sup>                        | 40 000                                      |
| Claude Haiku 3                                                                               | 2 000                               | 200 000<sup>†</sup>                        | 40 000                                      |
| Claude Opus 4.x<sup>*</sup>                                                                  | 2 000                               | 800 000                                    | 160 000                                     |
| Claude Opus 3 ([obsolète](/docs/fr/about-claude/model-deprecations))                        | 2 000                               | 80 000<sup>†</sup>                         | 16 000                                      |

</Tab>
<Tab title="Niveau 4">
| Modèle                                                                                       | Requêtes maximales par minute (RPM) | Jetons d'entrée maximaux par minute (ITPM) | Jetons de sortie maximaux par minute (OTPM) |
| -------------------------------------------------------------------------------------------- | ----------------------------------- | ------------------------------------------ | ------------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 4 000                               | 2 000 000                                  | 400 000                                     |
| Claude Sonnet 3.7 ([obsolète](/docs/fr/about-claude/model-deprecations))                     | 4 000                               | 200 000                                    | 80 000                                      |
| Claude Haiku 4.5                                                                             | 4 000                               | 4 000 000                                  | 800 000                                     |
| Claude Haiku 3.5 ([obsolète](/docs/fr/about-claude/model-deprecations))                      | 4 000                               | 400 000<sup>†</sup>                        | 80 000                                      |
| Claude Haiku 3                                                                               | 4 000                               | 400 000<sup>†</sup>                        | 80 000                                      |
| Claude Opus 4.x<sup>*</sup>                                                                  | 4 000                               | 2 000 000                                  | 400 000                                     |
| Claude Opus 3 ([obsolète](/docs/fr/about-claude/model-deprecations))                        | 4 000                               | 400 000<sup>†</sup>                        | 80 000                                      |

</Tab>
<Tab title="Personnalisé">
Si vous recherchez des limites plus élevées pour un cas d'utilisation Enterprise, contactez les ventes via la [Console Claude](/settings/limits).
</Tab>
</Tabs>

_<sup>* - La limite de débit Opus 4.x est une limite totale qui s'applique au trafic combiné sur Opus 4, Opus 4.1 et Opus 4.5.</sup>_

_<sup>** - La limite de débit Sonnet 4.x est une limite totale qui s'applique au trafic combiné sur Sonnet 4 et Sonnet 4.5.</sup>_

_<sup>† - La limite compte `cache_read_input_tokens` vers l'utilisation ITPM.</sup>_

### API Message Batches

L'API Message Batches a son propre ensemble de limites de débit qui sont partagées entre tous les modèles. Celles-ci incluent une limite de requêtes par minute (RPM) pour tous les points de terminaison de l'API et une limite sur le nombre de requêtes de lot qui peuvent être dans la file d'attente de traitement en même temps. Une « requête de lot » ici fait référence à une partie d'un Message Batch. Vous pouvez créer un Message Batch contenant des milliers de requêtes de lot, chacune comptant vers cette limite. Une requête de lot est considérée comme faisant partie de la file d'attente de traitement lorsqu'elle n'a pas encore été traitée avec succès par le modèle.

<Tabs>
<Tab title="Niveau 1">
| Requêtes maximales par minute (RPM) | Requêtes de lot maximales en file d'attente de traitement | Requêtes de lot maximales par lot |
| ----------------------------------- | --------------------------------------------------------- | --------------------------------- |
| 50                                  | 100 000                                                   | 100 000                           |
</Tab>
<Tab title="Niveau 2">
| Requêtes maximales par minute (RPM) | Requêtes de lot maximales en file d'attente de traitement | Requêtes de lot maximales par lot |
| ----------------------------------- | --------------------------------------------------------- | --------------------------------- |
| 1 000                               | 200 000                                                   | 100 000                           |
</Tab>
<Tab title="Niveau 3">
| Requêtes maximales par minute (RPM) | Requêtes de lot maximales en file d'attente de traitement | Requêtes de lot maximales par lot |
| ----------------------------------- | --------------------------------------------------------- | --------------------------------- |
| 2 000                               | 300 000                                                   | 100 000                           |
</Tab>
<Tab title="Niveau 4">
| Requêtes maximales par minute (RPM) | Requêtes de lot maximales en file d'attente de traitement | Requêtes de lot maximales par lot |
| ----------------------------------- | --------------------------------------------------------- | --------------------------------- |
| 4 000                               | 500 000                                                   | 100 000                           |
</Tab>
<Tab title="Personnalisé">
Si vous recherchez des limites plus élevées pour un cas d'utilisation Enterprise, contactez les ventes via la [Console Claude](/settings/limits).
</Tab>
</Tabs>

### Limites de débit de contexte long

Lors de l'utilisation de Claude Sonnet 4 et Sonnet 4.5 avec la [fenêtre de contexte de 1 M jetons activée](/docs/fr/build-with-claude/context-windows#1m-token-context-window), les limites de débit dédiées suivantes s'appliquent aux requêtes dépassant 200 000 jetons.

<Note>
La fenêtre de contexte de 1 M jetons est actuellement en bêta pour les organisations du niveau d'utilisation 4 et les organisations avec des limites de débit personnalisées. La fenêtre de contexte de 1 M jetons n'est disponible que pour Claude Sonnet 4 et Sonnet 4.5.
</Note>

<Tabs>
<Tab title="Niveau 4">
| Jetons d'entrée maximaux par minute (ITPM) | Jetons de sortie maximaux par minute (OTPM) |
| ------------------------------------------ | ------------------------------------------- |
| 1 000 000                                  | 200 000                                     |
</Tab>
<Tab title="Personnalisé">
Pour les limites de débit de contexte long personnalisées pour les cas d'utilisation d'entreprise, contactez les ventes via la [Console Claude](/settings/limits).
</Tab>
</Tabs>

<Tip>
Pour tirer le meilleur parti de la fenêtre de contexte de 1 M jetons avec les limites de débit, utilisez la [mise en cache des invites](/docs/fr/build-with-claude/prompt-caching).
</Tip>

### Surveillance de vos limites de débit dans la Console

Vous pouvez surveiller votre utilisation des limites de débit sur la page [Utilisation](/settings/usage) de la [Console Claude](/).

En plus de fournir des graphiques de jetons et de requêtes, la page Utilisation fournit deux graphiques de limites de débit séparés. Utilisez ces graphiques pour voir quelle marge vous avez pour croître, quand vous pourriez atteindre l'utilisation maximale, mieux comprendre quelles limites de débit demander, ou comment vous pouvez améliorer vos taux de mise en cache. Les graphiques visualisent un certain nombre de métriques pour une limite de débit donnée (par exemple, par modèle) :

- Le graphique **Limite de débit - Jetons d'entrée** inclut :
  - Jetons d'entrée non mis en cache maximaux par heure par minute
  - Votre limite de débit actuelle des jetons d'entrée par minute
  - Le taux de cache pour vos jetons d'entrée (c'est-à-dire le pourcentage de jetons d'entrée lus à partir du cache)
- Le graphique **Limite de débit - Jetons de sortie** inclut :
  - Jetons de sortie maximaux par heure par minute
  - Votre limite de débit actuelle des jetons de sortie par minute

## Définition de limites inférieures pour les espaces de travail

Afin de protéger les espaces de travail de votre organisation contre une utilisation excessive potentielle, vous pouvez définir des limites de dépenses et de débit personnalisées par espace de travail.

Exemple : Si la limite de votre organisation est de 40 000 jetons d'entrée par minute et 8 000 jetons de sortie par minute, vous pourriez limiter un espace de travail à 30 000 jetons totaux par minute. Cela protège les autres espaces de travail contre une utilisation excessive potentielle et assure une distribution plus équitable des ressources dans votre organisation. Les jetons par minute inutilisés restants (ou plus, si cet espace de travail n'utilise pas la limite) sont alors disponibles pour que d'autres espaces de travail les utilisent.

Remarque :
- Vous ne pouvez pas définir de limites sur l'espace de travail par défaut.
- Si non défini, les limites de l'espace de travail correspondent à la limite de l'organisation.
- Les limites à l'échelle de l'organisation s'appliquent toujours, même si les limites de l'espace de travail s'ajoutent à plus.
- La prise en charge des limites de jetons d'entrée et de sortie sera ajoutée aux espaces de travail à l'avenir.

## En-têtes de réponse

La réponse de l'API inclut des en-têtes qui vous montrent la limite de débit appliquée, l'utilisation actuelle et quand la limite sera réinitialisée.

Les en-têtes suivants sont renvoyés :

| En-tête                                       | Description                                                                                                                                     |
| --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `retry-after`                                 | Le nombre de secondes à attendre avant de pouvoir réessayer la requête. Les tentatives antérieures échoueront.                                  |
| `anthropic-ratelimit-requests-limit`          | Le nombre maximum de requêtes autorisées dans n'importe quelle période de limite de débit.                                                      |
| `anthropic-ratelimit-requests-remaining`      | Le nombre de requêtes restantes avant d'être limité en débit.                                                                                   |
| `anthropic-ratelimit-requests-reset`          | L'heure à laquelle la limite de débit des requêtes sera complètement reconstituée, fournie au format RFC 3339.                                  |
| `anthropic-ratelimit-tokens-limit`            | Le nombre maximum de jetons autorisés dans n'importe quelle période de limite de débit.                                                        |
| `anthropic-ratelimit-tokens-remaining`        | Le nombre de jetons restants (arrondi au millier le plus proche) avant d'être limité en débit.                                                 |
| `anthropic-ratelimit-tokens-reset`            | L'heure à laquelle la limite de débit des jetons sera complètement reconstituée, fournie au format RFC 3339.                                    |
| `anthropic-ratelimit-input-tokens-limit`      | Le nombre maximum de jetons d'entrée autorisés dans n'importe quelle période de limite de débit.                                               |
| `anthropic-ratelimit-input-tokens-remaining`  | Le nombre de jetons d'entrée restants (arrondi au millier le plus proche) avant d'être limité en débit.                                       |
| `anthropic-ratelimit-input-tokens-reset`      | L'heure à laquelle la limite de débit des jetons d'entrée sera complètement reconstituée, fournie au format RFC 3339.                          |
| `anthropic-ratelimit-output-tokens-limit`     | Le nombre maximum de jetons de sortie autorisés dans n'importe quelle période de limite de débit.                                              |
| `anthropic-ratelimit-output-tokens-remaining` | Le nombre de jetons de sortie restants (arrondi au millier le plus proche) avant d'être limité en débit.                                      |
| `anthropic-ratelimit-output-tokens-reset`     | L'heure à laquelle la limite de débit des jetons de sortie sera complètement reconstituée, fournie au format RFC 3339.                         |
| `anthropic-priority-input-tokens-limit`       | Le nombre maximum de jetons d'entrée du niveau Priority autorisés dans n'importe quelle période de limite de débit. (Niveau Priority uniquement) |
| `anthropic-priority-input-tokens-remaining`   | Le nombre de jetons d'entrée du niveau Priority restants (arrondi au millier le plus proche) avant d'être limité en débit. (Niveau Priority uniquement) |
| `anthropic-priority-input-tokens-reset`       | L'heure à laquelle la limite de débit des jetons d'entrée du niveau Priority sera complètement reconstituée, fournie au format RFC 3339. (Niveau Priority uniquement) |
| `anthropic-priority-output-tokens-limit`      | Le nombre maximum de jetons de sortie du niveau Priority autorisés dans n'importe quelle période de limite de débit. (Niveau Priority uniquement) |
| `anthropic-priority-output-tokens-remaining`  | Le nombre de jetons de sortie du niveau Priority restants (arrondi au millier le plus proche) avant d'être limité en débit. (Niveau Priority uniquement) |
| `anthropic-priority-output-tokens-reset`      | L'heure à laquelle la limite de débit des jetons de sortie du niveau Priority sera complètement reconstituée, fournie au format RFC 3339. (Niveau Priority uniquement) |

Les en-têtes `anthropic-ratelimit-tokens-*` affichent les valeurs pour la limite la plus restrictive actuellement en vigueur. Par exemple, si vous avez dépassé la limite de jetons par minute de l'espace de travail, les en-têtes contiendront les valeurs de limite de débit des jetons par minute de l'espace de travail. Si les limites de l'espace de travail ne s'appliquent pas, les en-têtes renverront le total des jetons restants, où le total est la somme des jetons d'entrée et de sortie. Cette approche garantit que vous avez une visibilité sur la contrainte la plus pertinente sur votre utilisation actuelle de l'API.