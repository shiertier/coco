# Glossaire

Ces concepts ne sont pas uniques aux modèles de langage d'Anthropic, mais nous présentons un bref résumé des termes clés ci-dessous.

---

## Fenêtre de contexte

La "fenêtre de contexte" fait référence à la quantité de texte qu'un modèle de langage peut consulter et référencer lors de la génération de nouveau texte. Ceci est différent du large corpus de données sur lequel le modèle de langage a été entraîné, et représente plutôt une "mémoire de travail" pour le modèle. Une fenêtre de contexte plus large permet au modèle de comprendre et de répondre à des invites plus complexes et plus longues, tandis qu'une fenêtre de contexte plus petite peut limiter la capacité du modèle à gérer des invites plus longues ou à maintenir la cohérence lors de conversations étendues.

Consultez notre [guide pour comprendre les fenêtres de contexte](/docs/fr/build-with-claude/context-windows) pour en savoir plus.

## Ajustement fin

L'ajustement fin est le processus de formation supplémentaire d'un modèle de langage pré-entraîné en utilisant des données additionnelles. Cela amène le modèle à commencer à représenter et imiter les motifs et caractéristiques du jeu de données d'ajustement fin. Claude n'est pas un modèle de langage brut ; il a déjà été ajusté finement pour être un assistant utile. Notre API n'offre actuellement pas d'ajustement fin, mais veuillez demander à votre contact Anthropic si vous êtes intéressé à explorer cette option. L'ajustement fin peut être utile pour adapter un modèle de langage à un domaine spécifique, une tâche ou un style d'écriture, mais il nécessite une considération attentive des données d'ajustement fin et de l'impact potentiel sur les performances et les biais du modèle.

## HHH

Ces trois H représentent les objectifs d'Anthropic pour s'assurer que Claude soit bénéfique à la société :

- Une IA **utile** tentera d'accomplir la tâche ou de répondre à la question posée au mieux de ses capacités, en fournissant des informations pertinentes et utiles.
- Une IA **honnête** donnera des informations précises, et n'hallucinera pas ou ne confabulera pas. Elle reconnaîtra ses limitations et incertitudes quand approprié.
- Une IA **inoffensive** ne sera pas offensante ou discriminatoire, et lorsqu'on lui demande d'aider dans un acte dangereux ou non éthique, l'IA devrait poliment refuser et expliquer pourquoi elle ne peut pas se conformer.

## Latence

La latence, dans le contexte de l'IA générative et des grands modèles de langage, fait référence au temps qu'il faut au modèle pour répondre à une invite donnée. C'est le délai entre la soumission d'une invite et la réception de la sortie générée. Une latence plus faible indique des temps de réponse plus rapides, ce qui est crucial pour les applications en temps réel, les chatbots et les expériences interactives. Les facteurs qui peuvent affecter la latence incluent la taille du modèle, les capacités matérielles, les conditions réseau, et la complexité de l'invite et de la réponse générée.

## LLM

Les grands modèles de langage (LLM) sont des modèles de langage IA avec de nombreux paramètres qui sont capables d'effectuer une variété de tâches étonnamment utiles. Ces modèles sont entraînés sur de vastes quantités de données textuelles et peuvent générer du texte semblable à celui des humains, répondre à des questions, résumer des informations, et plus encore. Claude est un assistant conversationnel basé sur un grand modèle de langage qui a été ajusté finement et entraîné en utilisant RLHF pour être plus utile, honnête et inoffensif.

## MCP (Model Context Protocol)

Le Model Context Protocol (MCP) est un protocole ouvert qui standardise la façon dont les applications fournissent du contexte aux LLM. Comme un port USB-C pour les applications IA, MCP fournit une façon unifiée de connecter les modèles IA à différentes sources de données et outils. MCP permet aux systèmes IA de maintenir un contexte cohérent à travers les interactions et d'accéder aux ressources externes de manière standardisée. Consultez notre [documentation MCP](/docs/fr/mcp) pour en savoir plus.

## Connecteur MCP

Le connecteur MCP est une fonctionnalité qui permet aux utilisateurs de l'API de se connecter aux serveurs MCP directement depuis l'API Messages sans construire un client MCP. Cela permet une intégration transparente avec les outils et services compatibles MCP à travers l'API Claude. Le connecteur MCP prend en charge des fonctionnalités comme l'appel d'outils et est disponible en bêta publique. Consultez notre [documentation du connecteur MCP](/docs/fr/agents-and-tools/mcp-connector) pour en savoir plus.

## Pré-entraînement

Le pré-entraînement est le processus initial d'entraînement des modèles de langage sur un large corpus de texte non étiqueté. Dans le cas de Claude, les modèles de langage autorégressifs (comme le modèle sous-jacent de Claude) sont pré-entraînés pour prédire le mot suivant, étant donné le contexte précédent du texte dans le document. Ces modèles pré-entraînés ne sont pas intrinsèquement bons pour répondre aux questions ou suivre des instructions, et nécessitent souvent une compétence approfondie en ingénierie d'invite pour susciter les comportements désirés. L'ajustement fin et RLHF sont utilisés pour raffiner ces modèles pré-entraînés, les rendant plus utiles pour une large gamme de tâches.

## RAG (Génération augmentée par récupération)

La génération augmentée par récupération (RAG) est une technique qui combine la récupération d'informations avec la génération de modèles de langage pour améliorer la précision et la pertinence du texte généré, et pour mieux ancrer la réponse du modèle dans les preuves. Dans RAG, un modèle de langage est augmenté avec une base de connaissances externe ou un ensemble de documents qui est passé dans la fenêtre de contexte. Les données sont récupérées au moment de l'exécution lorsqu'une requête est envoyée au modèle, bien que le modèle lui-même ne récupère pas nécessairement les données (mais peut le faire avec [l'utilisation d'outils](/docs/fr/agents-and-tools/tool-use/overview) et une fonction de récupération). Lors de la génération de texte, les informations pertinentes doivent d'abord être récupérées de la base de connaissances basée sur l'invite d'entrée, puis passées au modèle avec la requête originale. Le modèle utilise ces informations pour guider la sortie qu'il génère. Cela permet au modèle d'accéder et d'utiliser des informations au-delà de ses données d'entraînement, réduisant la dépendance à la mémorisation et améliorant la précision factuelle du texte généré. RAG peut être particulièrement utile pour les tâches qui nécessitent des informations à jour, des connaissances spécifiques au domaine, ou une citation explicite des sources. Cependant, l'efficacité de RAG dépend de la qualité et de la pertinence de la base de connaissances externe et des connaissances qui sont récupérées au moment de l'exécution.

## RLHF

L'Apprentissage par Renforcement à partir de Retours Humains (RLHF) est une technique utilisée pour entraîner un modèle de langage pré-entraîné à se comporter de manières qui sont cohérentes avec les préférences humaines. Cela peut inclure aider le modèle à suivre les instructions plus efficacement ou agir plus comme un chatbot. Les retours humains consistent à classer un ensemble de deux ou plusieurs textes d'exemple, et le processus d'apprentissage par renforcement encourage le modèle à préférer les sorties qui sont similaires à celles classées plus haut. Claude a été entraîné en utilisant RLHF pour être un assistant plus utile. Pour plus de détails, vous pouvez lire [l'article d'Anthropic sur le sujet](https://arxiv.org/abs/2204.05862).

## Température

La température est un paramètre qui contrôle le caractère aléatoire des prédictions d'un modèle lors de la génération de texte. Des températures plus élevées conduisent à des sorties plus créatives et diverses, permettant de multiples variations dans la formulation et, dans le cas de la fiction, des variations dans les réponses également. Des températures plus basses résultent en des sorties plus conservatrices et déterministes qui s'en tiennent à la formulation et aux réponses les plus probables. Ajuster la température permet aux utilisateurs d'encourager un modèle de langage à explorer des choix et séquences de mots rares, peu communs ou surprenants, plutôt que de seulement sélectionner les prédictions les plus probables.

## TTFT (Temps jusqu'au premier jeton)

Le Temps jusqu'au Premier Jeton (TTFT) est une métrique de performance qui mesure le temps qu'il faut à un modèle de langage pour générer le premier jeton de sa sortie après avoir reçu une invite. C'est un indicateur important de la réactivité du modèle et est particulièrement pertinent pour les applications interactives, les chatbots et les systèmes en temps réel où les utilisateurs s'attendent à un retour initial rapide. Un TTFT plus bas indique que le modèle peut commencer à générer une réponse plus rapidement, fournissant une expérience utilisateur plus fluide et engageante. Les facteurs qui peuvent influencer TTFT incluent la taille du modèle, les capacités matérielles, les conditions réseau, et la complexité de l'invite.

## Jetons

Les jetons sont les plus petites unités individuelles d'un modèle de langage, et peuvent correspondre à des mots, sous-mots, caractères, ou même des octets (dans le cas d'Unicode). Pour Claude, un jeton représente approximativement 3,5 caractères anglais, bien que le nombre exact puisse varier selon la langue utilisée. Les jetons sont typiquement cachés lors de l'interaction avec les modèles de langage au niveau "texte" mais deviennent pertinents lors de l'examen des entrées et sorties exactes d'un modèle de langage. Lorsque Claude reçoit du texte à évaluer, le texte (consistant en une série de caractères) est encodé en une série de jetons pour que le modèle puisse traiter. Des jetons plus grands permettent l'efficacité des données pendant l'inférence et le pré-entraînement (et sont utilisés quand possible), tandis que des jetons plus petits permettent à un modèle de gérer des mots peu communs ou jamais vus auparavant. Le choix de la méthode de tokenisation peut impacter les performances du modèle, la taille du vocabulaire, et la capacité à gérer les mots hors vocabulaire.