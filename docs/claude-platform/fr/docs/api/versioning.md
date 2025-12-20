# Versions

Lors de l'envoi de requêtes API, vous devez envoyer un en-tête de requête `anthropic-version`. Par exemple, `anthropic-version: 2023-06-01`. Si vous utilisez nos [SDK clients](/docs/fr/api/client-sdks), ceci est géré automatiquement pour vous.

---

Pour toute version d'API donnée, nous préserverons :

* Les paramètres d'entrée existants
* Les paramètres de sortie existants

Cependant, nous pouvons faire ce qui suit :

* Ajouter des entrées optionnelles supplémentaires
* Ajouter des valeurs supplémentaires à la sortie
* Modifier les conditions pour des types d'erreur spécifiques
* Ajouter de nouvelles variantes aux valeurs de sortie de type enum (par exemple, les types d'événements de streaming)

En général, si vous utilisez l'API comme documenté dans cette référence, nous ne casserons pas votre utilisation.

## Historique des versions

Nous recommandons toujours d'utiliser la dernière version de l'API chaque fois que possible. Les versions précédentes sont considérées comme obsolètes et peuvent être indisponibles pour les nouveaux utilisateurs.

* `2023-06-01`  
   * Nouveau format pour les événements server-sent (SSE) de [streaming](/docs/fr/api/streaming) :  
         * Les complétions sont incrémentales. Par exemple, `" Hello"`, `" my"`, `" name"`, `" is"`, `" Claude." ` au lieu de `" Hello"`, `" Hello my"`, `" Hello my name"`, `" Hello my name is"`, `" Hello my name is Claude."`.  
         * Tous les événements sont des [événements nommés](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#named%5Fevents), plutôt que des [événements de données uniquement](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#data-only%5Fmessages).  
         * Suppression de l'événement `data: [DONE]` inutile.  
   * Suppression des valeurs héritées `exception` et `truncated` dans les réponses.
* `2023-01-01` : Version initiale.