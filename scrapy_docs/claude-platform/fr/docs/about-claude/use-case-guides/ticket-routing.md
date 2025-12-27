# Routage des tickets

Ce guide explique comment exploiter les capacités avancées de compréhension du langage naturel de Claude pour classer les tickets d'assistance clientèle à grande échelle en fonction de l'intention du client, de l'urgence, de la priorisation, du profil du client, et bien plus.

---

## Déterminez si vous devez utiliser Claude pour le routage des tickets

Voici quelques indicateurs clés montrant que vous devriez utiliser un LLM comme Claude à la place des approches ML traditionnelles pour votre tâche de classification :

    <section title="Vous disposez de données d'entraînement étiquetées limitées">

        Les processus ML traditionnels nécessitent des ensembles de données étiquetées massifs. Le modèle pré-entraîné de Claude peut classer efficacement les tickets avec seulement quelques dizaines d'exemples étiquetés, réduisant considérablement le temps et les coûts de préparation des données.
    
</section>
    <section title="Vos catégories de classification sont susceptibles de changer ou d'évoluer au fil du temps">

        Une fois qu'une approche ML traditionnelle a été établie, la modifier est une entreprise laborieuse et gourmande en données. En revanche, à mesure que votre produit ou les besoins de vos clients évoluent, Claude peut facilement s'adapter aux changements dans les définitions de classe ou aux nouvelles classes sans relabellisation extensive des données d'entraînement.
    
</section>
    <section title="Vous devez gérer des entrées de texte complexes et non structurées">

        Les modèles ML traditionnels ont souvent du mal avec les données non structurées et nécessitent une ingénierie des caractéristiques extensive. La compréhension avancée du langage de Claude permet une classification précise basée sur le contenu et le contexte, plutôt que de s'appuyer sur des structures ontologiques strictes.
    
</section>
    <section title="Vos règles de classification sont basées sur la compréhension sémantique">

        Les approches ML traditionnelles s'appuient souvent sur des modèles de sac de mots ou sur la correspondance de motifs simples. Claude excelle dans la compréhension et l'application des règles sous-jacentes lorsque les classes sont définies par des conditions plutôt que par des exemples.
    
</section>
    <section title="Vous avez besoin d'un raisonnement interprétable pour les décisions de classification">

        De nombreux modèles ML traditionnels fournissent peu d'informations sur leur processus de prise de décision. Claude peut fournir des explications lisibles par l'homme pour ses décisions de classification, renforçant la confiance dans le système d'automatisation et facilitant une adaptation facile si nécessaire.
    
</section>
    <section title="Vous souhaitez gérer les cas limites et les tickets ambigus plus efficacement">

        Les systèmes ML traditionnels ont souvent du mal avec les valeurs aberrantes et les entrées ambiguës, les classant fréquemment mal ou se rabattant sur une catégorie fourre-tout. Les capacités de traitement du langage naturel de Claude lui permettent de mieux interpréter le contexte et la nuance dans les tickets d'assistance, réduisant potentiellement le nombre de tickets mal acheminés ou non classifiés qui nécessitent une intervention manuelle.
    
</section>
    <section title="Vous avez besoin d'un support multilingue sans maintenir des modèles séparés">

        Les approches ML traditionnelles nécessitent généralement des modèles séparés ou des processus de traduction extensive pour chaque langue prise en charge. Les capacités multilingues de Claude lui permettent de classer les tickets dans diverses langues sans avoir besoin de modèles séparés ou de processus de traduction extensive, rationalisant le support pour les bases de clients mondiaux.
    
</section>

***

##  Construisez et déployez votre flux de travail d'assistance LLM

### Comprenez votre approche actuelle du support
Avant de vous lancer dans l'automatisation, il est crucial de comprendre votre système de ticketing existant. Commencez par enquêter sur la façon dont votre équipe d'assistance gère actuellement le routage des tickets.

Posez-vous des questions comme :
* Quels critères sont utilisés pour déterminer quel SLA/offre de service est appliqué ?
* Le routage des tickets est-il utilisé pour déterminer quel niveau d'assistance ou quel spécialiste produit reçoit le ticket ?
* Y a-t-il des règles ou des flux de travail automatisés déjà en place ? Dans quels cas échouent-ils ?
* Comment les cas limites ou les tickets ambigus sont-ils gérés ?
* Comment l'équipe priorise-t-elle les tickets ?

Plus vous en saurez sur la façon dont les humains gèrent certains cas, mieux vous pourrez travailler avec Claude pour accomplir la tâche.

### Définissez les catégories d'intention de l'utilisateur
Une liste bien définie de catégories d'intention de l'utilisateur est cruciale pour une classification précise des tickets d'assistance avec Claude. La capacité de Claude à acheminer les tickets efficacement dans votre système est directement proportionnelle à la qualité de la définition des catégories de votre système.

Voici quelques exemples de catégories et sous-catégories d'intention de l'utilisateur.

    <section title="Problème technique">

        * Problème matériel
        * Bogue logiciel
        * Problème de compatibilité
        * Problème de performance
    
</section>
    <section title="Gestion de compte">

        * Réinitialisation de mot de passe
        * Problèmes d'accès au compte
        * Demandes de facturation
        * Modifications d'abonnement
    
</section>
    <section title="Informations sur le produit">

        * Demandes de fonctionnalités
        * Questions de compatibilité des produits
        * Informations sur les tarifs
        * Demandes de disponibilité
    
</section>
    <section title="Orientation utilisateur">

        * Questions pratiques
        * Assistance à l'utilisation des fonctionnalités
        * Conseils sur les meilleures pratiques
        * Conseils de dépannage
    
</section>
    <section title="Retours d'expérience">

        * Rapports de bogues
        * Demandes de fonctionnalités
        * Retours ou suggestions généraux
        * Plaintes
    
</section>
    <section title="Lié aux commandes">

        * Demandes de statut de commande
        * Informations d'expédition
        * Retours et échanges
        * Modifications de commande
    
</section>
    <section title="Demande de service">

        * Assistance à l'installation
        * Demandes de mise à niveau
        * Planification de la maintenance
        * Annulation de service
    
</section>
    <section title="Préoccupations de sécurité">

        * Demandes de confidentialité des données
        * Rapports d'activité suspecte
        * Assistance aux fonctionnalités de sécurité
    
</section>
    <section title="Conformité et questions juridiques">

        * Questions de conformité réglementaire
        * Demandes de conditions d'utilisation
        * Demandes de documentation juridique
    
</section>
    <section title="Support d'urgence">

        * Défaillances critiques du système
        * Problèmes de sécurité urgents
        * Problèmes sensibles au facteur temps
    
</section>
    <section title="Formation et éducation">

        * Demandes de formation produit
        * Demandes de documentation
        * Informations sur les webinaires ou ateliers
    
</section>
    <section title="Intégration et API">

        * Assistance à l'intégration
        * Questions d'utilisation de l'API
        * Demandes de compatibilité avec des tiers
    
</section>

En plus de l'intention, le routage et la priorisation des tickets peuvent également être influencés par d'autres facteurs tels que l'urgence, le type de client, les SLA ou la langue. Assurez-vous de considérer d'autres critères de routage lors de la construction de votre système de routage automatisé.

### Établissez les critères de succès

Travaillez avec votre équipe d'assistance pour [définir des critères de succès clairs](/docs/fr/test-and-evaluate/define-success) avec des repères mesurables, des seuils et des objectifs.

Voici quelques critères et repères standards lors de l'utilisation d'LLM pour le routage des tickets d'assistance :

    <section title="Cohérence de la classification">

        Cette métrique évalue la cohérence avec laquelle Claude classe les tickets similaires au fil du temps. C'est crucial pour maintenir la fiabilité du routage. Mesurez cela en testant périodiquement le modèle avec un ensemble d'entrées standardisées et en visant un taux de cohérence de 95 % ou plus.
    
</section>
    <section title="Vitesse d'adaptation">

        Cela mesure la rapidité avec laquelle Claude peut s'adapter à de nouvelles catégories ou à des modèles de tickets changeants. Testez cela en introduisant de nouveaux types de tickets et en mesurant le temps qu'il faut au modèle pour atteindre une précision satisfaisante (par exemple, >90 %) sur ces nouvelles catégories. Visez une adaptation dans 50-100 tickets d'exemple.
    
</section>
    <section title="Gestion multilingue">

        Cela évalue la capacité de Claude à acheminer avec précision les tickets dans plusieurs langues. Mesurez la précision du routage dans différentes langues, en visant une baisse de précision ne dépassant pas 5-10 % pour les langues non primaires.
    
</section>
    <section title="Gestion des cas limites">

        Cela évalue les performances de Claude sur les tickets inhabituels ou complexes. Créez un ensemble de test de cas limites et mesurez la précision du routage, en visant au moins 80 % de précision sur ces entrées difficiles.
    
</section>
    <section title="Atténuation des biais">

        Cela mesure l'équité de Claude dans le routage entre différentes démographies de clients. Auditez régulièrement les décisions de routage pour les biais potentiels, en visant une précision de routage cohérente (dans 2-3 %) dans tous les groupes de clients.
    
</section>
    <section title="Efficacité des invites">

        Dans les situations où minimiser le nombre de jetons est crucial, ce critère évalue la performance de Claude avec un contexte minimal. Mesurez la précision du routage avec différentes quantités de contexte fourni, en visant une précision de 90 %+ avec seulement le titre du ticket et une brève description.
    
</section>
    <section title="Score d'explicabilité">

        Cela évalue la qualité et la pertinence des explications de Claude pour ses décisions de routage. Les évaluateurs humains peuvent noter les explications sur une échelle (par exemple, 1-5), avec l'objectif d'atteindre un score moyen de 4 ou plus.
    
</section>

Voici quelques critères de succès courants qui peuvent être utiles indépendamment du fait qu'un LLM soit utilisé ou non :

    <section title="Précision du routage">

        La précision du routage mesure la fréquence à laquelle les tickets sont correctement assignés à l'équipe ou à la personne appropriée du premier coup. Cela est généralement mesuré en pourcentage de tickets correctement acheminés sur le nombre total de tickets. Les repères de l'industrie visent souvent une précision de 90-95 %, bien que cela puisse varier en fonction de la complexité de la structure d'assistance.
    
</section>
    <section title="Temps d'assignation">

        Cette métrique suit la rapidité avec laquelle les tickets sont assignés après leur soumission. Des temps d'assignation plus rapides conduisent généralement à des résolutions plus rapides et à une satisfaction client améliorée. Les systèmes de classe mondiale réalisent souvent des temps d'assignation moyens inférieurs à 5 minutes, beaucoup visant un routage quasi instantané (ce qui est possible avec les implémentations LLM).
    
</section>
    <section title="Taux de réacheminement">

        Le taux de réacheminement indique la fréquence à laquelle les tickets doivent être réassignés après le routage initial. Un taux plus faible suggère un routage initial plus précis. Visez un taux de réacheminement inférieur à 10 %, les systèmes les plus performants atteignant des taux aussi bas que 5 % ou moins.
    
</section>
    <section title="Taux de résolution au premier contact">

        Cela mesure le pourcentage de tickets résolus lors de la première interaction avec le client. Des taux plus élevés indiquent un routage efficace et des équipes d'assistance bien préparées. Les repères de l'industrie varient généralement de 70-75 %, les meilleurs performants atteignant des taux de 80 % ou plus.
    
</section>
    <section title="Temps de traitement moyen">

        Le temps de traitement moyen mesure le temps qu'il faut pour résoudre un ticket du début à la fin. Un routage efficace peut réduire considérablement ce temps. Les repères varient largement selon l'industrie et la complexité, mais de nombreuses organisations visent à maintenir le temps de traitement moyen en dessous de 24 heures pour les problèmes non critiques.
    
</section>
    <section title="Scores de satisfaction client">

        Souvent mesurés par des sondages post-interaction, ces scores reflètent la satisfaction globale du client face au processus d'assistance. Un routage efficace contribue à une satisfaction plus élevée. Visez des scores CSAT de 90 % ou plus, les meilleurs performants atteignant souvent des taux de satisfaction de 95 %+.
    
</section>
    <section title="Taux d'escalade">

        Cela mesure la fréquence à laquelle les tickets doivent être escaladés à des niveaux d'assistance supérieurs. Des taux d'escalade plus faibles indiquent souvent un routage initial plus précis. Efforcez-vous d'atteindre un taux d'escalade inférieur à 20 %, les systèmes de classe mondiale atteignant des taux de 10 % ou moins.
    
</section>
    <section title="Productivité des agents">

        Cette métrique examine le nombre de tickets que les agents peuvent traiter efficacement après la mise en œuvre de la solution de routage. Un routage amélioré devrait augmenter la productivité. Mesurez cela en suivant les tickets résolus par agent par jour ou par heure, en visant une amélioration de 10-20 % après la mise en œuvre d'un nouveau système de routage.
    
</section>
    <section title="Taux de déflexion en libre-service">

        Cela mesure le pourcentage de tickets potentiels résolus par des options en libre-service avant d'entrer dans le système de routage. Des taux plus élevés indiquent un triage pré-routage efficace. Visez un taux de déflexion de 20-30 %, les meilleurs performants atteignant des taux de 40 % ou plus.
    
</section>
    <section title="Coût par ticket">

        Cette métrique calcule le coût moyen pour résoudre chaque ticket d'assistance. Un routage efficace devrait aider à réduire ce coût au fil du temps. Bien que les repères varient largement, de nombreuses organisations visent à réduire le coût par ticket de 10-15 % après la mise en œuvre d'un système de routage amélioré.
    
</section>

### Choisissez le bon modèle Claude

Le choix du modèle dépend des compromis entre le coût, la précision et le temps de réponse.

De nombreux clients ont trouvé `claude-haiku-4-5-20251001` un modèle idéal pour le routage des tickets, car c'est le modèle le plus rapide et le plus rentable de la famille Claude 4 tout en offrant d'excellents résultats. Si votre problème de classification nécessite une expertise approfondie en matière de sujet ou un grand volume de catégories d'intention avec un raisonnement complexe, vous pouvez opter pour le [modèle Sonnet plus grand](/docs/fr/about-claude/models).

### Construisez une invite solide

Le routage des tickets est un type de tâche de classification. Claude analyse le contenu d'un ticket d'assistance et le classe dans des catégories prédéfinies en fonction du type de problème, de l'urgence, de l'expertise requise ou d'autres facteurs pertinents.

Écrivons une invite de classification de tickets. Notre invite initiale doit contenir le contenu de la demande de l'utilisateur et retourner à la fois le raisonnement et l'intention.

<Tip>
Essayez le [générateur d'invites](/docs/fr/prompt-generator) sur la [Console Claude](/login) pour que Claude rédige un premier brouillon pour vous.
</Tip>

Voici un exemple d'invite de classification de routage de tickets :
```python
def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. Your task is to analyze customer support requests and output the appropriate classification intent for each request, along with your reasoning. 

        Here is the customer support request you need to classify:

        <request>{ticket_contents}</request>

        Please carefully analyze the above request to determine the customer's core intent and needs. Consider what the customer is asking for has concerns about.

        First, write out your reasoning and analysis of how to classify this request inside <reasoning> tags.

        Then, output the appropriate classification label for the request inside a <intent> tag. The valid intents are:
        <intents>
        <intent>Support, Feedback, Complaint</intent>
        <intent>Order Tracking</intent>
        <intent>Refund/Exchange</intent>
        </intents>

        A request may have ONLY ONE applicable intent. Only include the intent that is most applicable to the request.

        As an example, consider the following request:
        <request>Hello! I had high-speed fiber internet installed on Saturday and my installer, Kevin, was absolutely fantastic! Where can I send my positive review? Thanks for your help!</request>

        Here is an example of how your output should be formatted (for the above example request):
        <reasoning>The user seeks information in order to leave positive feedback.</reasoning>
        <intent>Support, Feedback, Complaint</intent>

        Here are a few more examples:
        <examples>
        <example 2>
        Example 2 Input:
        <request>I wanted to write and personally thank you for the compassion you showed towards my family during my father's funeral this past weekend. Your staff was so considerate and helpful throughout this whole process; it really took a load off our shoulders. The visitation brochures were beautiful. We'll never forget the kindness you showed us and we are so appreciative of how smoothly the proceedings went. Thank you, again, Amarantha Hill on behalf of the Hill Family.</request>

        Example 2 Output:
        <reasoning>User leaves a positive review of their experience.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 2>
        <example 3>

        ...

        </example 8>
        <example 9>
        Example 9 Input:
        <request>Your website keeps sending ad-popups that block the entire screen. It took me twenty minutes just to finally find the phone number to call and complain. How can I possibly access my account information with all of these popups? Can you access my account for me, since your website is broken? I need to know what the address is on file.</request>

        Example 9 Output:
        <reasoning>The user requests help accessing their web account information.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 9>

        Remember to always include your classification reasoning before your actual intent output. The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
```

Décomposons les composants clés de cette invite :
* Nous utilisons les f-strings Python pour créer le modèle d'invite, permettant à `ticket_contents` d'être inséré dans les balises `<request>`.
* Nous donnons à Claude un rôle clairement défini en tant que système de classification qui analyse attentivement le contenu du ticket pour déterminer l'intention et les besoins fondamentaux du client.
* Nous instruisons Claude sur le formatage approprié de la sortie, dans ce cas pour fournir son raisonnement et son analyse à l'intérieur des balises `<reasoning>`, suivis de l'étiquette de classification appropriée à l'intérieur des balises `<intent>`.
* Nous spécifions les catégories d'intention valides : « Support, Feedback, Complaint », « Order Tracking » et « Refund/Exchange ».
* Nous incluons quelques exemples (alias few-shot prompting) pour illustrer comment la sortie doit être formatée, ce qui améliore la précision et la cohérence.

La raison pour laquelle nous voulons que Claude divise sa réponse en différentes sections de balises XML est que nous pouvons utiliser des expressions régulières pour extraire séparément le raisonnement et l'intention de la sortie. Cela nous permet de créer des étapes suivantes ciblées dans le flux de travail de routage des tickets, comme l'utilisation uniquement de l'intention pour décider à qui acheminer le ticket.

### Déployez votre invite

Il est difficile de savoir comment fonctionne bien votre invite sans la déployer dans un environnement de test en production et [exécuter des évaluations](/docs/fr/test-and-evaluate/develop-tests).

Construisons la structure de déploiement. Commencez par définir la signature de méthode pour envelopper notre appel à Claude. Nous prendrons la méthode que nous avons déjà commencé à écrire, qui a `ticket_contents` comme entrée, et maintenant retourner un tuple de `reasoning` et `intent` comme sortie. Si vous avez une automatisation existante utilisant le ML traditionnel, vous voudrez suivre cette signature de méthode à la place.

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ... The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
    # Send the prompt to the API to classify the support request.
    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
        stream=False,
    )
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

    return reasoning, intent
```

Ce code :
* Importe la bibliothèque Anthropic et crée une instance client en utilisant votre clé API.
* Définit une fonction `classify_support_request` qui prend une chaîne `ticket_contents`.
* Envoie le `ticket_contents` à Claude pour classification en utilisant le `classification_prompt`
* Retourne le `reasoning` et `intent` du modèle extraits de la réponse.

Puisque nous devons attendre que tout le texte de raisonnement et d'intention soit généré avant d'analyser, nous définissons `stream=False` (la valeur par défaut).

***

## Évaluez votre invite

L'élaboration d'invites nécessite souvent des tests et une optimisation pour être prête pour la production. Pour déterminer la préparation de votre solution, évaluez les performances en fonction des critères de succès et des seuils que vous avez établis précédemment.

Pour exécuter votre évaluation, vous aurez besoin de cas de test pour l'exécuter. Le reste de ce guide suppose que vous avez déjà [développé vos cas de test](/docs/fr/test-and-evaluate/develop-tests).

### Construisez une fonction d'évaluation

Notre exemple d'évaluation pour ce guide mesure les performances de Claude selon trois métriques clés :
* Précision
* Coût par classification

Vous devrez peut-être évaluer Claude sur d'autres axes en fonction des facteurs qui sont importants pour vous.

Pour évaluer cela, nous devons d'abord modifier le script que nous avons écrit et ajouter une fonction pour comparer l'intention prédite avec l'intention réelle et calculer le pourcentage de prédictions correctes. Nous devons également ajouter la fonctionnalité de calcul des coûts et de mesure du temps.

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(request, actual_intent):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ...The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """

    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
    )
    usage = message.usage  # Get the usage statistics for the API call for how many input and output tokens were used.
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

      # Check if the model's prediction is correct.
    correct = actual_intent.strip() == intent.strip()

    # Return the reasoning, intent, correct, and usage.
    return reasoning, intent, correct, usage
```

Décomposons les modifications que nous avons apportées :
* Nous avons ajouté le `actual_intent` de nos cas de test dans la méthode `classify_support_request` et configuré une comparaison pour évaluer si la classification d'intention de Claude correspond à notre classification d'intention dorée.
* Nous avons extrait les statistiques d'utilisation pour l'appel API pour calculer le coût en fonction des jetons d'entrée et de sortie utilisés

### Exécutez votre évaluation

Une évaluation appropriée nécessite des seuils et des repères clairs pour déterminer ce qui est un bon résultat. Le script ci-dessus nous donnera les valeurs d'exécution pour la précision, le temps de réponse et le coût par classification, mais nous aurions toujours besoin de seuils clairement établis. Par exemple :
* **Précision :** 95 % (sur 100 tests)
* **Coût par classification :** Réduction de 50 % en moyenne (sur 100 tests) par rapport à votre méthode de routage actuelle

Avoir ces seuils vous permet de déterminer rapidement et facilement à grande échelle, et avec un empirisme impartial, quelle méthode est la meilleure pour vous et quels changements pourraient être nécessaires pour mieux répondre à vos exigences.

***

## Améliorez les performances

Dans les scénarios complexes, il peut être utile de considérer des stratégies supplémentaires pour améliorer les performances au-delà des [techniques standard d'ingénierie d'invites](/docs/fr/build-with-claude/prompt-engineering/overview) et des [stratégies de mise en œuvre de garde-fous](/docs/fr/test-and-evaluate/strengthen-guardrails/reduce-hallucinations). Voici quelques scénarios courants :

### Utilisez une hiérarchie taxonomique pour les cas avec 20+ catégories d'intention

À mesure que le nombre de classes augmente, le nombre d'exemples requis augmente également, ce qui peut rendre l'invite difficile à gérer. Comme alternative, vous pouvez envisager de mettre en œuvre un système de classification hiérarchique en utilisant un mélange de classificateurs.
1. Organisez vos intentions dans une structure d'arbre taxonomique.
2. Créez une série de classificateurs à chaque niveau de l'arbre, permettant une approche de routage en cascade.

Par exemple, vous pourriez avoir un classificateur de haut niveau qui catégorise largement les tickets en « Problèmes techniques », « Questions de facturation » et « Demandes générales ». Chacune de ces catégories peut alors avoir son propre sous-classificateur pour affiner davantage la classification.

![](/docs/images/ticket-hierarchy.png)

* **Avantages - plus de nuance et de précision :** Vous pouvez créer différentes invites pour chaque chemin parent, permettant une classification plus ciblée et contextuelle. Cela peut conduire à une précision améliorée et à une gestion plus nuancée des demandes des clients.

* **Inconvénients - latence accrue :** Sachez que plusieurs classificateurs peuvent entraîner une latence accrue, et nous recommandons de mettre en œuvre cette approche avec notre modèle le plus rapide, Haiku.

### Utilisez des bases de données vectorielles et la recherche par similarité pour gérer les tickets hautement variables

Malgré le fait que fournir des exemples soit le moyen le plus efficace d'améliorer les performances, si les demandes d'assistance sont hautement variables, il peut être difficile d'inclure suffisamment d'exemples dans une seule invite.

Dans ce scénario, vous pourriez utiliser une base de données vectorielle pour effectuer des recherches de similarité à partir d'un ensemble de données d'exemples et récupérer les exemples les plus pertinents pour une requête donnée.

Cette approche, décrite en détail dans notre [recette de classification](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb), s'est avérée améliorer les performances de 71 % de précision à 93 % de précision.

### Tenez compte spécifiquement des cas limites attendus

Voici quelques scénarios où Claude peut mal classer les tickets (il peut y en avoir d'autres qui sont uniques à votre situation). Dans ces scénarios, envisagez de fournir des instructions explicites ou des exemples dans l'invite de la façon dont Claude doit gérer le cas limite :

    <section title="Les clients font des demandes implicites">

        Les clients expriment souvent les besoins indirectement. Par exemple, « J'attends mon colis depuis plus de deux semaines » peut être une demande indirecte de statut de commande.
        * **Solution :** Fournissez à Claude quelques exemples réels de clients de ces types de demandes, ainsi que l'intention sous-jacente. Vous pouvez obtenir des résultats encore meilleurs si vous incluez une justification de classification pour les intentions de tickets particulièrement nuancées, afin que Claude puisse mieux généraliser la logique à d'autres tickets.
    
</section>
    <section title="Claude priorise l'émotion plutôt que l'intention">

        Lorsque les clients expriment leur insatisfaction, Claude peut prioriser l'adresse de l'émotion plutôt que la résolution du problème sous-jacent.
        * **Solution :** Fournissez à Claude des directives sur quand prioriser le sentiment du client ou non. Cela peut être aussi simple que « Ignorez toutes les émotions des clients. Concentrez-vous uniquement sur l'analyse de l'intention de la demande du client et sur les informations que le client pourrait demander. »
    
</section>
    <section title="Plusieurs problèmes causent une confusion de priorisation des problèmes">

        Lorsque les clients présentent plusieurs problèmes dans une seule interaction, Claude peut avoir du mal à identifier la préoccupation principale.
        * **Solution :** Clarifiez la priorisation des intentions afin que Claude puisse mieux classer les intentions extraites et identifier la préoccupation principale.
    
</section>

***

## Intégrez Claude dans votre flux de travail d'assistance plus large

Une intégration appropriée nécessite que vous preniez certaines décisions concernant la façon dont votre script de routage des tickets basé sur Claude s'intègre dans l'architecture de votre système de routage des tickets plus large. Il y a deux façons de procéder :
* **Basé sur le push :** Le système de tickets d'assistance que vous utilisez (par exemple Zendesk) déclenche votre code en envoyant un événement webhook à votre service de routage, qui classe ensuite l'intention et l'achemine.
    * Cette approche est plus évolutive sur le web, mais vous devez exposer un point de terminaison public.
* **Basé sur le pull :** Votre code extrait les derniers tickets en fonction d'un calendrier donné et les achemine au moment du pull.
    * Cette approche est plus facile à mettre en œuvre mais pourrait faire des appels inutiles au système de tickets d'assistance lorsque la fréquence de pull est trop élevée ou pourrait être excessivement lente lorsque la fréquence de pull est trop faible.

Pour l'une de ces approches, vous devrez envelopper votre script dans un service. Le choix de l'approche dépend des API que votre système de ticketing d'assistance fournit.

***

<CardGroup cols={2}>
    <Card title="Classification cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/tree/main/capabilities/classification">
        Visitez notre classification cookbook pour plus d'exemples de code et des conseils d'évaluation détaillés.
    </Card>
    <Card title="Claude Console" icon="link" href="/dashboard">
        Commencez à construire et évaluer votre flux de travail sur la Console Claude.
    </Card>
</CardGroup>