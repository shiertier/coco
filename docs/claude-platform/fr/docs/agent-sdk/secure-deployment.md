# Déploiement sécurisé d'agents IA

Un guide pour sécuriser les déploiements de Claude Code et du SDK Agent avec l'isolation, la gestion des identifiants et les contrôles réseau

---

Claude Code et le SDK Agent sont des outils puissants qui peuvent exécuter du code, accéder à des fichiers et interagir avec des services externes en votre nom. Comme tout outil ayant ces capacités, les déployer de manière réfléchie garantit que vous bénéficiez des avantages tout en maintenant des contrôles appropriés.

Contrairement aux logiciels traditionnels qui suivent des chemins de code prédéterminés, ces outils génèrent leurs actions de manière dynamique en fonction du contexte et des objectifs. Cette flexibilité est ce qui les rend utiles, mais cela signifie également que leur comportement peut être influencé par le contenu qu'ils traitent : fichiers, pages web ou entrées utilisateur. C'est parfois appelé injection de prompt. Par exemple, si le README d'un référentiel contient des instructions inhabituelles, Claude Code pourrait les incorporer dans ses actions de manière que l'opérateur n'avait pas anticipée. Ce guide couvre des moyens pratiques de réduire ce risque.

La bonne nouvelle est que sécuriser un déploiement d'agent ne nécessite pas d'infrastructure exotique. Les mêmes principes qui s'appliquent à l'exécution de n'importe quel code semi-approuvé s'appliquent ici : isolation, moindre privilège et défense en profondeur. Claude Code inclut plusieurs fonctionnalités de sécurité qui aident avec les préoccupations courantes, et ce guide les parcourt ainsi que des options de durcissement supplémentaires pour ceux qui en ont besoin.

Tous les déploiements n'ont pas besoin d'une sécurité maximale. Un développeur exécutant Claude Code sur son ordinateur portable a des exigences différentes d'une entreprise traitant les données des clients dans un environnement multi-locataire. Ce guide présente des options allant des fonctionnalités de sécurité intégrées de Claude Code aux architectures de production durcies, afin que vous puissiez choisir ce qui convient à votre situation.

## Contre quoi nous protégeons-nous ?

Les agents peuvent prendre des mesures involontaires en raison d'une injection de prompt (instructions intégrées dans le contenu qu'ils traitent) ou d'une erreur du modèle. Les modèles Claude sont conçus pour résister à cela, et comme nous l'avons analysé dans notre [fiche technique du modèle](https://assets.anthropic.com/m/64823ba7485345a7/Claude-Opus-4-5-System-Card.pdf), nous croyons que Claude Opus 4.5 est le modèle de frontier le plus robuste disponible.

La défense en profondeur est néanmoins une bonne pratique. Par exemple, si un agent traite un fichier malveillant qui lui ordonne d'envoyer les données des clients à un serveur externe, les contrôles réseau peuvent bloquer complètement cette demande.

## Fonctionnalités de sécurité intégrées

Claude Code inclut plusieurs fonctionnalités de sécurité qui répondent aux préoccupations courantes. Consultez la [documentation de sécurité](https://code.claude.com/docs/fr/security) pour plus de détails.

- **Système de permissions** : Chaque outil et commande bash peut être configuré pour autoriser, bloquer ou demander l'approbation de l'utilisateur. Utilisez des motifs glob pour créer des règles comme « autoriser toutes les commandes npm » ou « bloquer toute commande avec sudo ». Les organisations peuvent définir des politiques qui s'appliquent à tous les utilisateurs. Voir [contrôle d'accès et permissions](https://code.claude.com/docs/fr/iam#access-control-and-permissions).
- **Analyse statique** : Avant d'exécuter les commandes bash, Claude Code exécute une analyse statique pour identifier les opérations potentiellement risquées. Les commandes qui modifient les fichiers système ou accèdent aux répertoires sensibles sont signalées et nécessitent l'approbation explicite de l'utilisateur.
- **Résumé de recherche web** : Les résultats de recherche sont résumés plutôt que de transmettre le contenu brut directement dans le contexte, réduisant le risque d'injection de prompt à partir de contenu web malveillant.
- **Mode sandbox** : Les commandes bash peuvent s'exécuter dans un environnement en sandbox qui restreint l'accès au système de fichiers et au réseau. Consultez la [documentation du sandboxing](https://code.claude.com/docs/fr/sandboxing) pour plus de détails.

## Principes de sécurité

Pour les déploiements qui nécessitent un durcissement supplémentaire au-delà des paramètres par défaut de Claude Code, ces principes guident les options disponibles.

### Limites de sécurité

Une limite de sécurité sépare les composants ayant des niveaux de confiance différents. Pour les déploiements hautement sécurisés, vous pouvez placer les ressources sensibles (comme les identifiants) en dehors de la limite contenant l'agent. Si quelque chose se passe mal dans l'environnement de l'agent, les ressources en dehors de cette limite restent protégées.

Par exemple, plutôt que de donner à un agent un accès direct à une clé API, vous pourriez exécuter un proxy en dehors de l'environnement de l'agent qui injecte la clé dans les demandes. L'agent peut faire des appels API, mais il ne voit jamais l'identifiant lui-même. Ce modèle est utile pour les déploiements multi-locataires ou lors du traitement de contenu non approuvé.

### Moindre privilège

Si nécessaire, vous pouvez restreindre l'agent aux seules capacités requises pour sa tâche spécifique :

| Ressource | Options de restriction |
|----------|------------------------|
| Système de fichiers | Monter uniquement les répertoires nécessaires, préférer la lecture seule |
| Réseau | Restreindre à des points de terminaison spécifiques via proxy |
| Identifiants | Injecter via proxy plutôt que d'exposer directement |
| Capacités système | Supprimer les capacités Linux dans les conteneurs |

### Défense en profondeur

Pour les environnements hautement sécurisés, superposer plusieurs contrôles fournit une protection supplémentaire. Les options incluent :

- Isolation des conteneurs
- Restrictions réseau
- Contrôles du système de fichiers
- Validation des demandes à un proxy

La bonne combinaison dépend de votre modèle de menace et de vos exigences opérationnelles.

## Technologies d'isolation

Différentes technologies d'isolation offrent différents compromis entre la force de sécurité, les performances et la complexité opérationnelle.

<Info>
Dans toutes ces configurations, Claude Code (ou votre application SDK Agent) s'exécute à l'intérieur de la limite d'isolation—le sandbox, le conteneur ou la VM. Les contrôles de sécurité décrits ci-dessous restreignent ce que l'agent peut accéder depuis cette limite.
</Info>

| Technologie | Force d'isolation | Surcharge de performance | Complexité |
|------------|-------------------|-------------------------|-----------|
| Runtime sandbox | Bonne (paramètres par défaut sécurisés) | Très faible | Faible |
| Conteneurs (Docker) | Dépend de la configuration | Faible | Moyen |
| gVisor | Excellente (avec la bonne configuration) | Moyen/Élevé | Moyen |
| VMs (Firecracker, QEMU) | Excellente (avec la bonne configuration) | Élevé | Moyen/Élevé |

### Runtime sandbox

Pour une isolation légère sans conteneurs, [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime) applique les restrictions du système de fichiers et du réseau au niveau du système d'exploitation.

L'avantage principal est la simplicité : aucune configuration Docker, image de conteneur ou configuration réseau requise. Le proxy et les restrictions du système de fichiers sont intégrés. Vous fournissez un fichier de paramètres spécifiant les domaines et chemins autorisés.

**Comment cela fonctionne :**
- **Système de fichiers** : Utilise les primitives du système d'exploitation (`bubblewrap` sur Linux, `sandbox-exec` sur macOS) pour restreindre l'accès en lecture/écriture aux chemins configurés
- **Réseau** : Supprime l'espace de noms réseau (Linux) ou utilise les profils Seatbelt (macOS) pour acheminer le trafic réseau via un proxy intégré
- **Configuration** : Listes blanches basées sur JSON pour les domaines et chemins du système de fichiers

**Configuration :**
```bash
npm install @anthropic-ai/sandbox-runtime
```

Ensuite, créez un fichier de configuration spécifiant les chemins et domaines autorisés.

**Considérations de sécurité :**

1. **Noyau du même hôte** : Contrairement aux VMs, les processus en sandbox partagent le noyau de l'hôte. Une vulnérabilité du noyau pourrait théoriquement permettre une évasion. Pour certains modèles de menace, c'est acceptable, mais si vous avez besoin d'une isolation au niveau du noyau, utilisez gVisor ou une VM séparée.

2. **Pas d'inspection TLS** : Le proxy met en liste blanche les domaines mais n'inspecte pas le trafic chiffré. Si l'agent a des identifiants permissifs pour un domaine autorisé, assurez-vous qu'il n'est pas possible d'utiliser ce domaine pour déclencher d'autres demandes réseau ou pour exfiltrer des données.

Pour de nombreux cas d'utilisation de développeur unique et CI/CD, sandbox-runtime élève considérablement la barre avec une configuration minimale. Les sections ci-dessous couvrent les conteneurs et les VMs pour les déploiements nécessitant une isolation plus forte.

### Conteneurs

Les conteneurs fournissent une isolation via les espaces de noms Linux. Chaque conteneur a sa propre vue du système de fichiers, de l'arborescence des processus et de la pile réseau, tout en partageant le noyau de l'hôte.

Une configuration de conteneur durcie en sécurité pourrait ressembler à ceci :

```bash
docker run \
  --cap-drop ALL \
  --security-opt no-new-privileges \
  --security-opt seccomp=/path/to/seccomp-profile.json \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /home/agent:rw,noexec,nosuid,size=500m \
  --network none \
  --memory 2g \
  --cpus 2 \
  --pids-limit 100 \
  --user 1000:1000 \
  -v /path/to/code:/workspace:ro \
  -v /var/run/proxy.sock:/var/run/proxy.sock:ro \
  agent-image
```

Voici ce que fait chaque option :

| Option | Objectif |
|--------|----------|
| `--cap-drop ALL` | Supprime les capacités Linux comme `NET_ADMIN` et `SYS_ADMIN` qui pourraient permettre l'escalade de privilèges |
| `--security-opt no-new-privileges` | Empêche les processus de gagner des privilèges via les binaires setuid |
| `--security-opt seccomp=...` | Restreint les appels système disponibles ; la valeur par défaut de Docker en bloque ~44, les profils personnalisés peuvent en bloquer davantage |
| `--read-only` | Rend le système de fichiers racine du conteneur immuable, empêchant l'agent de persister les modifications |
| `--tmpfs /tmp:...` | Fournit un répertoire temporaire accessible en écriture qui est effacé lorsque le conteneur s'arrête |
| `--network none` | Supprime toutes les interfaces réseau ; l'agent communique via la prise Unix montée ci-dessous |
| `--memory 2g` | Limite l'utilisation de la mémoire pour prévenir l'épuisement des ressources |
| `--pids-limit 100` | Limite le nombre de processus pour prévenir les bombes fork |
| `--user 1000:1000` | S'exécute en tant qu'utilisateur non-root |
| `-v ...:/workspace:ro` | Monte le code en lecture seule afin que l'agent puisse l'analyser mais pas le modifier. **Évitez de monter les répertoires d'hôte sensibles comme `~/.ssh`, `~/.aws` ou `~/.config`** |
| `-v .../proxy.sock:...` | Monte une prise Unix connectée à un proxy s'exécutant en dehors du conteneur (voir ci-dessous) |

**Architecture de prise Unix :**

Avec `--network none`, le conteneur n'a aucune interface réseau. La seule façon pour l'agent d'atteindre le monde extérieur est via la prise Unix montée, qui se connecte à un proxy s'exécutant sur l'hôte. Ce proxy peut appliquer les listes blanches de domaines, injecter les identifiants et enregistrer tout le trafic.

C'est la même architecture utilisée par [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime). Même si l'agent est compromis via une injection de prompt, il ne peut pas exfiltrer les données vers des serveurs arbitraires—il ne peut communiquer que via le proxy, qui contrôle les domaines accessibles. Pour plus de détails, consultez le [billet de blog sur le sandboxing de Claude Code](https://www.anthropic.com/engineering/claude-code-sandboxing).

**Options de durcissement supplémentaires :**

| Option | Objectif |
|--------|----------|
| `--userns-remap` | Mappe la racine du conteneur à l'utilisateur d'hôte sans privilèges ; nécessite une configuration du démon mais limite les dégâts en cas d'évasion de conteneur |
| `--ipc private` | Isole la communication inter-processus pour prévenir les attaques entre conteneurs |

### gVisor

Les conteneurs standard partagent le noyau de l'hôte : lorsque le code à l'intérieur d'un conteneur effectue un appel système, il va directement au même noyau qui exécute l'hôte. Cela signifie qu'une vulnérabilité du noyau pourrait permettre l'évasion du conteneur. gVisor résout ce problème en interceptant les appels système en espace utilisateur avant qu'ils n'atteignent le noyau de l'hôte, en implémentant sa propre couche de compatibilité qui gère la plupart des appels système sans impliquer le vrai noyau.

Si un agent exécute du code malveillant (peut-être en raison d'une injection de prompt), ce code s'exécute dans le conteneur et pourrait tenter des exploits du noyau. Avec gVisor, la surface d'attaque est beaucoup plus petite : le code malveillant devrait d'abord exploiter l'implémentation en espace utilisateur de gVisor et aurait un accès limité au vrai noyau.

Pour utiliser gVisor avec Docker, installez le runtime `runsc` et configurez le démon :

```json
// /etc/docker/daemon.json
{
  "runtimes": {
    "runsc": {
      "path": "/usr/local/bin/runsc"
    }
  }
}
```

Ensuite, exécutez les conteneurs avec :

```bash
docker run --runtime=runsc agent-image
```

**Considérations de performance :**

| Charge de travail | Surcharge |
|------------------|-----------|
| Calcul lié au CPU | ~0% (pas d'interception d'appel système) |
| Appels système simples | ~2× plus lent |
| Entrée/sortie intensive de fichiers | Jusqu'à 10-200× plus lent pour les modèles d'ouverture/fermeture lourds |

Pour les environnements multi-locataires ou lors du traitement de contenu non approuvé, l'isolation supplémentaire vaut souvent la surcharge.

### Machines virtuelles

Les VMs fournissent une isolation au niveau matériel via les extensions de virtualisation CPU. Chaque VM exécute son propre noyau, créant une limite forte—une vulnérabilité dans le noyau invité ne compromet pas directement l'hôte. Cependant, les VMs ne sont pas automatiquement « plus sécurisées » que les alternatives comme gVisor. La sécurité des VMs dépend fortement de l'hyperviseur et du code d'émulation des appareils.

Firecracker est conçu pour l'isolation légère des microVMs—il peut démarrer les VMs en moins de 125 ms avec moins de 5 MiB de surcharge mémoire, en supprimant l'émulation d'appareils inutile pour réduire la surface d'attaque.

Avec cette approche, la VM de l'agent n'a pas d'interface réseau externe. Au lieu de cela, elle communique via `vsock` (prises virtuelles). Tout le trafic s'achemine via vsock vers un proxy sur l'hôte, qui applique les listes blanches et injecte les identifiants avant de transférer les demandes.

### Déploiements cloud

Pour les déploiements cloud, vous pouvez combiner l'une des technologies d'isolation ci-dessus avec les contrôles réseau natifs du cloud :

1. Exécutez les conteneurs d'agent dans un sous-réseau privé sans passerelle Internet
2. Configurez les règles de pare-feu cloud (AWS Security Groups, pare-feu VPC GCP) pour bloquer tout le trafic sortant sauf vers votre proxy
3. Exécutez un proxy (tel que [Envoy](https://www.envoyproxy.io/) avec son filtre `credential_injector`) qui valide les demandes, applique les listes blanches de domaines, injecte les identifiants et transfère vers les API externes
4. Attribuez les permissions IAM minimales au compte de service de l'agent, en acheminant l'accès sensible via le proxy si possible
5. Enregistrez tout le trafic au proxy à des fins d'audit

## Gestion des identifiants

Les agents ont souvent besoin d'identifiants pour appeler les API, accéder aux référentiels ou interagir avec les services cloud. Le défi est de fournir cet accès sans exposer les identifiants eux-mêmes.

### Le modèle proxy

L'approche recommandée est d'exécuter un proxy en dehors de la limite de sécurité de l'agent qui injecte les identifiants dans les demandes sortantes. L'agent envoie les demandes sans identifiants, le proxy les ajoute et transfère la demande à sa destination.

Ce modèle a plusieurs avantages :

1. L'agent ne voit jamais les identifiants réels
2. Le proxy peut appliquer une liste blanche des points de terminaison autorisés
3. Le proxy peut enregistrer toutes les demandes à des fins d'audit
4. Les identifiants sont stockés dans un seul endroit sécurisé plutôt que distribués à chaque agent

### Configuration de Claude Code pour utiliser un proxy

Claude Code supporte deux méthodes pour acheminer les demandes d'échantillonnage via un proxy :

**Option 1 : ANTHROPIC_BASE_URL (simple mais uniquement pour les demandes d'API d'échantillonnage)**

```bash
export ANTHROPIC_BASE_URL="http://localhost:8080"
```

Cela indique à Claude Code et au SDK Agent d'envoyer les demandes d'échantillonnage à votre proxy au lieu de l'API Anthropic directement. Votre proxy reçoit les demandes HTTP en texte brut, peut les inspecter et les modifier (y compris injecter les identifiants), puis transfère vers l'API réelle.

**Option 2 : HTTP_PROXY / HTTPS_PROXY (système complet)**

```bash
export HTTP_PROXY="http://localhost:8080"
export HTTPS_PROXY="http://localhost:8080"
```

Claude Code et le SDK Agent respectent ces variables d'environnement standard, acheminant tout le trafic HTTP via le proxy. Pour HTTPS, le proxy crée un tunnel CONNECT chiffré : il ne peut pas voir ou modifier le contenu des demandes sans interception TLS.

### Implémentation d'un proxy

Vous pouvez créer votre propre proxy ou en utiliser un existant :

- [Envoy Proxy](https://www.envoyproxy.io/) — proxy de qualité production avec filtre `credential_injector` pour ajouter les en-têtes d'authentification
- [mitmproxy](https://mitmproxy.org/) — proxy terminant TLS pour inspecter et modifier le trafic HTTPS
- [Squid](http://www.squid-cache.org/) — proxy de mise en cache avec listes de contrôle d'accès
- [LiteLLM](https://github.com/BerriAI/litellm) — passerelle LLM avec injection d'identifiants et limitation de débit

### Identifiants pour d'autres services

Au-delà de l'échantillonnage à partir de l'API Anthropic, les agents ont souvent besoin d'un accès authentifié à d'autres services—référentiels git, bases de données, API internes. Il y a deux approches principales :

#### Outils personnalisés

Fournissez l'accès via un serveur MCP ou un outil personnalisé qui achemine les demandes vers un service s'exécutant en dehors de la limite de sécurité de l'agent. L'agent appelle l'outil, mais la demande authentifiée réelle se produit en dehors—l'outil appelle un proxy qui injecte les identifiants.

Par exemple, un serveur MCP git pourrait accepter les commandes de l'agent mais les transférer à un proxy git s'exécutant sur l'hôte, qui ajoute l'authentification avant de contacter le référentiel distant. L'agent ne voit jamais les identifiants.

Avantages :
- **Pas d'interception TLS** : Le service externe effectue les demandes authentifiées directement
- **Les identifiants restent en dehors** : L'agent ne voit que l'interface de l'outil, pas les identifiants sous-jacents

#### Transfert de trafic

Pour les appels à l'API Anthropic, `ANTHROPIC_BASE_URL` vous permet d'acheminer les demandes vers un proxy qui peut les inspecter et les modifier en texte brut. Mais pour les autres services HTTPS (GitHub, registres npm, API internes), le trafic est souvent chiffré de bout en bout—même si vous l'achemineriez via un proxy via `HTTP_PROXY`, le proxy ne voit qu'un tunnel TLS opaque et ne peut pas injecter les identifiants.

Pour modifier le trafic HTTPS vers des services arbitraires, sans utiliser un outil personnalisé, vous avez besoin d'un proxy terminant TLS qui déchiffre le trafic, l'inspecte ou le modifie, puis le re-chiffre avant de le transférer. Cela nécessite :

1. D'exécuter le proxy en dehors du conteneur de l'agent
2. D'installer le certificat CA du proxy dans le magasin de confiance de l'agent (afin que l'agent fasse confiance aux certificats du proxy)
3. De configurer `HTTP_PROXY`/`HTTPS_PROXY` pour acheminer le trafic via le proxy

Cette approche gère n'importe quel service basé sur HTTP sans écrire d'outils personnalisés, mais ajoute de la complexité autour de la gestion des certificats.

Notez que tous les programmes ne respectent pas `HTTP_PROXY`/`HTTPS_PROXY`. La plupart des outils (curl, pip, npm, git) le font, mais certains peuvent contourner ces variables et se connecter directement. Par exemple, `fetch()` de Node.js ignore ces variables par défaut ; dans Node 24+, vous pouvez définir `NODE_USE_ENV_PROXY=1` pour activer le support. Pour une couverture complète, vous pouvez utiliser [proxychains](https://github.com/haad/proxychains) pour intercepter les appels réseau, ou configurer iptables pour rediriger le trafic sortant vers un proxy transparent.

<Info>
Un **proxy transparent** intercepte le trafic au niveau du réseau, donc le client n'a pas besoin d'être configuré pour l'utiliser. Les proxies réguliers nécessitent que les clients se connectent explicitement et parlent HTTP CONNECT ou SOCKS. Les proxies transparents (comme Squid ou mitmproxy en mode transparent) peuvent gérer les connexions TCP brutes redirigées.
</Info>

Les deux approches nécessitent toujours le proxy terminant TLS et le certificat CA de confiance—elles garantissent simplement que le trafic atteint réellement le proxy.

## Configuration du système de fichiers

Les contrôles du système de fichiers déterminent quels fichiers l'agent peut lire et écrire.

### Montage du code en lecture seule

Lorsque l'agent doit analyser le code mais pas le modifier, montez le répertoire en lecture seule :

```bash
docker run -v /path/to/code:/workspace:ro agent-image
```

<Warning>
Même l'accès en lecture seule à un répertoire de code peut exposer les identifiants. Les fichiers courants à exclure ou nettoyer avant le montage :

| Fichier | Risque |
|---------|--------|
| `.env`, `.env.local` | Clés API, mots de passe de base de données, secrets |
| `~/.git-credentials` | Mots de passe/jetons Git en texte brut |
| `~/.aws/credentials` | Clés d'accès AWS |
| `~/.config/gcloud/application_default_credentials.json` | Jetons ADC Google Cloud |
| `~/.azure/` | Identifiants Azure CLI |
| `~/.docker/config.json` | Jetons d'authentification du registre Docker |
| `~/.kube/config` | Identifiants du cluster Kubernetes |
| `.npmrc`, `.pypirc` | Jetons du registre de paquets |
| `*-service-account.json` | Clés de compte de service GCP |
| `*.pem`, `*.key` | Clés privées |

Envisagez de copier uniquement les fichiers source nécessaires, ou d'utiliser le filtrage de style `.dockerignore`.
</Warning>

### Emplacements accessibles en écriture

Si l'agent doit écrire des fichiers, vous avez quelques options selon que vous voulez que les modifications persistent :

Pour les espaces de travail éphémères dans les conteneurs, utilisez les montages `tmpfs` qui existent uniquement en mémoire et sont effacés lorsque le conteneur s'arrête :

```bash
docker run \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /workspace:rw,noexec,size=500m \
  agent-image
```

Si vous voulez examiner les modifications avant de les persister, un système de fichiers overlay permet à l'agent d'écrire sans modifier les fichiers sous-jacents—les modifications sont stockées dans une couche séparée que vous pouvez inspecter, appliquer ou rejeter. Pour une sortie entièrement persistante, montez un volume dédié mais gardez-le séparé des répertoires sensibles.

## Lectures supplémentaires

- [Documentation de sécurité de Claude Code](https://code.claude.com/docs/fr/security)
- [Hébergement du SDK Agent](/docs/fr/agent-sdk/hosting)
- [Gestion des permissions](/docs/fr/agent-sdk/permissions)
- [Runtime sandbox](https://github.com/anthropic-experimental/sandbox-runtime)
- [The Lethal Trifecta for AI Agents](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/)
- [OWASP Top 10 for LLM Applications](https://owasp.org/www-project-top-10-for-large-language-model-applications/)
- [Docker Security Best Practices](https://docs.docker.com/engine/security/)
- [gVisor Documentation](https://gvisor.dev/docs/)
- [Firecracker Documentation](https://firecracker-microvm.github.io/)