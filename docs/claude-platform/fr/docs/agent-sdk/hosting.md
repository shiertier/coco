# Hébergement du SDK Agent

Déployer et héberger Claude Agent SDK dans les environnements de production

---

Le SDK Claude Agent diffère des API LLM sans état traditionnelles en ce qu'il maintient l'état conversationnel et exécute des commandes dans un environnement persistant. Ce guide couvre l'architecture, les considérations d'hébergement et les meilleures pratiques pour déployer des agents basés sur le SDK en production.

<Info>
Pour le renforcement de la sécurité au-delà du sandboxing de base, y compris les contrôles réseau, la gestion des identifiants et les options d'isolation, voir [Déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment).
</Info>

## Exigences d'hébergement

### Sandboxing basé sur des conteneurs

Pour la sécurité et l'isolation, le SDK doit s'exécuter dans un environnement de conteneur en sandbox. Cela fournit l'isolation des processus, les limites de ressources, le contrôle réseau et les systèmes de fichiers éphémères.

Le SDK supporte également la [configuration de sandbox programmatique](/docs/fr/agent-sdk/typescript#sandbox-settings) pour l'exécution de commandes.

### Configuration requise du système

Chaque instance du SDK nécessite :

- **Dépendances d'exécution**
  - Python 3.10+ (pour le SDK Python) ou Node.js 18+ (pour le SDK TypeScript)
  - Node.js (requis par Claude Code CLI)
  - Claude Code CLI : `npm install -g @anthropic-ai/claude-code`

- **Allocation de ressources**
  - Recommandé : 1 GiB de RAM, 5 GiB de disque et 1 CPU (ajustez cela en fonction de votre tâche selon les besoins)

- **Accès réseau**
  - HTTPS sortant vers `api.anthropic.com`
  - Optionnel : Accès aux serveurs MCP ou aux outils externes

## Comprendre l'architecture du SDK

Contrairement aux appels API sans état, le SDK Claude Agent fonctionne comme un **processus de longue durée** qui :
- **Exécute des commandes** dans un environnement shell persistant
- **Gère les opérations de fichiers** dans un répertoire de travail
- **Gère l'exécution des outils** avec le contexte des interactions précédentes

## Options de fournisseur de sandbox

Plusieurs fournisseurs se spécialisent dans les environnements de conteneurs sécurisés pour l'exécution de code IA :

- **[Cloudflare Sandboxes](https://github.com/cloudflare/sandbox-sdk)**
- **[Modal Sandboxes](https://modal.com/docs/guide/sandbox)**
- **[Daytona](https://www.daytona.io/)**
- **[E2B](https://e2b.dev/)**
- **[Fly Machines](https://fly.io/docs/machines/)**
- **[Vercel Sandbox](https://vercel.com/docs/functions/sandbox)**

Pour les options auto-hébergées (Docker, gVisor, Firecracker) et la configuration détaillée de l'isolation, voir [Technologies d'isolation](/docs/fr/agent-sdk/secure-deployment#isolation-technologies).

## Modèles de déploiement en production

### Modèle 1 : Sessions éphémères

Créez un nouveau conteneur pour chaque tâche utilisateur, puis détruisez-le une fois terminé.

Idéal pour les tâches ponctuelles, l'utilisateur peut toujours interagir avec l'IA pendant que la tâche se termine, mais une fois terminée, le conteneur est détruit.

**Exemples :**
- Enquête et correction de bogues : Déboguer et résoudre un problème spécifique avec le contexte pertinent
- Traitement des factures : Extraire et structurer les données des reçus/factures pour les systèmes comptables
- Tâches de traduction : Traduire des documents ou des lots de contenu entre les langues
- Traitement d'images/vidéos : Appliquer des transformations, des optimisations ou extraire des métadonnées à partir de fichiers médias

### Modèle 2 : Sessions de longue durée

Maintenir des instances de conteneurs persistantes pour les tâches de longue durée. Souvent, exécuter _plusieurs_ processus Claude Agent à l'intérieur du conteneur en fonction de la demande.

Idéal pour les agents proactifs qui agissent sans l'entrée de l'utilisateur, les agents qui servent du contenu ou les agents qui traitent de grandes quantités de messages.

**Exemples :**
- Agent de messagerie : Surveille les e-mails entrants et trie, répond ou prend des mesures de manière autonome en fonction du contenu
- Générateur de sites : Héberge des sites Web personnalisés par utilisateur avec des capacités d'édition en direct servies via les ports de conteneur
- Chatbots à haute fréquence : Gère les flux de messages continus à partir de plateformes comme Slack où les temps de réponse rapides sont critiques

### Modèle 3 : Sessions hybrides

Conteneurs éphémères qui sont hydratés avec l'historique et l'état, possiblement à partir d'une base de données ou des fonctionnalités de reprise de session du SDK.

Idéal pour les conteneurs avec une interaction intermittente de l'utilisateur qui lance le travail et s'arrête lorsque le travail est terminé mais peut être continué.

**Exemples :**
- Gestionnaire de projets personnels : Aide à gérer les projets en cours avec des points de contrôle intermittents, maintient le contexte des tâches, des décisions et de la progression
- Recherche approfondie : Mène des tâches de recherche de plusieurs heures, enregistre les résultats et reprend l'enquête lorsque l'utilisateur revient
- Agent d'assistance client : Gère les tickets d'assistance qui s'étendent sur plusieurs interactions, charge l'historique des tickets et le contexte client

### Modèle 4 : Conteneurs uniques

Exécutez plusieurs processus Claude Agent SDK dans un conteneur global unique.

Idéal pour les agents qui doivent collaborer étroitement ensemble. C'est probablement le modèle le moins populaire car vous devrez empêcher les agents de se réécrire mutuellement.

**Exemples :**
- **Simulations** : Agents qui interagissent les uns avec les autres dans des simulations telles que des jeux vidéo.

# FAQ

### Comment communiquer avec mes sandboxes ?
Lors de l'hébergement dans des conteneurs, exposez les ports pour communiquer avec vos instances du SDK. Votre application peut exposer des points de terminaison HTTP/WebSocket pour les clients externes tandis que le SDK s'exécute en interne dans le conteneur.

### Quel est le coût d'hébergement d'un conteneur ?
Nous avons constaté que le coût dominant de la fourniture d'agents est les jetons, les conteneurs varient en fonction de ce que vous approvisionnez, mais un coût minimum est d'environ 5 cents par heure d'exécution.

### Quand dois-je arrêter les conteneurs inactifs par rapport à les garder actifs ?
Cela dépend probablement du fournisseur, différents fournisseurs de sandbox vous permettront de définir différents critères pour les délais d'inactivité après lesquels un sandbox pourrait s'arrêter.
Vous voudrez ajuster ce délai d'expiration en fonction de la fréquence à laquelle vous pensez que la réponse de l'utilisateur pourrait se produire.

### À quelle fréquence dois-je mettre à jour Claude Code CLI ?
Claude Code CLI est versionné avec semver, donc tout changement de rupture sera versionné.

### Comment surveiller la santé des conteneurs et les performances des agents ?
Puisque les conteneurs sont juste des serveurs, la même infrastructure de journalisation que vous utilisez pour le backend fonctionnera pour les conteneurs.

### Combien de temps une session d'agent peut-elle s'exécuter avant expiration ?
Une session d'agent n'expirera pas, mais nous recommandons de définir une propriété 'maxTurns' pour empêcher Claude de se bloquer dans une boucle.

## Prochaines étapes

- [Déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment) - Contrôles réseau, gestion des identifiants et renforcement de l'isolation
- [SDK TypeScript - Paramètres de sandbox](/docs/fr/agent-sdk/typescript#sandbox-settings) - Configurer le sandbox par programmation
- [Guide des sessions](/docs/fr/agent-sdk/sessions) - En savoir plus sur la gestion des sessions
- [Permissions](/docs/fr/agent-sdk/permissions) - Configurer les permissions des outils
- [Suivi des coûts](/docs/fr/agent-sdk/cost-tracking) - Surveiller l'utilisation de l'API
- [Intégration MCP](/docs/fr/agent-sdk/mcp) - Étendre avec des outils personnalisés