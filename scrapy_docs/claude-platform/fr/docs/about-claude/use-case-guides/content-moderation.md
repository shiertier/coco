# Modération de contenu

La modération de contenu est un aspect critique du maintien d'un environnement sûr, respectueux et productif dans les applications numériques. Dans ce guide, nous discuterons de la façon dont Claude peut être utilisé pour modérer le contenu au sein de votre application numérique.

---

> Visitez notre [livre de recettes de modération de contenu](https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fmoderation%5Ffilter.ipynb) pour voir un exemple d'implémentation de modération de contenu utilisant Claude.

<Tip>Ce guide se concentre sur la modération du contenu généré par les utilisateurs au sein de votre application. Si vous cherchez des conseils sur la modération des interactions avec Claude, veuillez vous référer à notre [guide des garde-fous](/docs/fr/test-and-evaluate/strengthen-guardrails/reduce-hallucinations).</Tip>

## Avant de construire avec Claude

### Décider d'utiliser Claude pour la modération de contenu

Voici quelques indicateurs clés que vous devriez utiliser un LLM comme Claude au lieu d'une approche ML traditionnelle ou basée sur des règles pour la modération de contenu :

<section title="Vous voulez une implémentation rentable et rapide">
Les méthodes ML traditionnelles nécessitent des ressources d'ingénierie importantes, une expertise ML et des coûts d'infrastructure. Les systèmes de modération humaine entraînent des coûts encore plus élevés. Avec Claude, vous pouvez avoir un système de modération sophistiqué opérationnel en une fraction du temps pour une fraction du prix.
</section>
<section title="Vous désirez à la fois une compréhension sémantique et des décisions rapides">
Les approches ML traditionnelles, telles que les modèles de sac de mots ou la correspondance de motifs simple, ont souvent du mal à comprendre le ton, l'intention et le contexte du contenu. Bien que les systèmes de modération humaine excellent dans la compréhension du sens sémantique, ils nécessitent du temps pour que le contenu soit examiné. Claude comble le fossé en combinant la compréhension sémantique avec la capacité de livrer des décisions de modération rapidement.
</section>
<section title="Vous avez besoin de décisions politiques cohérentes">
En tirant parti de ses capacités de raisonnement avancées, Claude peut interpréter et appliquer des directives de modération complexes de manière uniforme. Cette cohérence aide à assurer un traitement équitable de tout le contenu, réduisant le risque de décisions de modération incohérentes ou biaisées qui peuvent saper la confiance des utilisateurs.
</section>
<section title="Vos politiques de modération sont susceptibles de changer ou d'évoluer au fil du temps">
Une fois qu'une approche ML traditionnelle a été établie, la changer est une entreprise laborieuse et intensive en données. D'autre part, à mesure que votre produit ou les besoins de vos clients évoluent, Claude peut facilement s'adapter aux changements ou aux ajouts aux politiques de modération sans étiquetage extensif des données d'entraînement.
</section>
<section title="Vous nécessitez un raisonnement interprétable pour vos décisions de modération">
Si vous souhaitez fournir aux utilisateurs ou aux régulateurs des explications claires derrière les décisions de modération, Claude peut générer des justifications détaillées et cohérentes. Cette transparence est importante pour établir la confiance et assurer la responsabilité dans les pratiques de modération de contenu.
</section>
<section title="Vous avez besoin d'un support multilingue sans maintenir des modèles séparés">
Les approches ML traditionnelles nécessitent généralement des modèles séparés ou des processus de traduction extensifs pour chaque langue supportée. La modération humaine nécessite l'embauche d'une main-d'œuvre fluide dans chaque langue supportée. Les capacités multilingues de Claude lui permettent de classifier les tickets dans diverses langues sans avoir besoin de modèles séparés ou de processus de traduction extensifs, rationalisant la modération pour les bases de clients mondiales.
</section>
<section title="Vous nécessitez un support multimodal">
Les capacités multimodales de Claude lui permettent d'analyser et d'interpréter le contenu à travers le texte et les images. Cela en fait un outil polyvalent pour une modération de contenu complète dans des environnements où différents types de médias doivent être évalués ensemble.
</section>

<Note>Anthropic a entraîné tous les modèles Claude pour être honnêtes, utiles et inoffensifs. Cela peut résulter en Claude modérant le contenu jugé particulièrement dangereux (en ligne avec notre [Politique d'Utilisation Acceptable](https://www.anthropic.com/legal/aup)), indépendamment du prompt utilisé. Par exemple, un site web pour adultes qui veut permettre aux utilisateurs de publier du contenu sexuel explicite peut constater que Claude signale toujours le contenu explicite comme nécessitant une modération, même s'ils spécifient dans leur prompt de ne pas modérer le contenu sexuel explicite. Nous recommandons de réviser notre AUP avant de construire une solution de modération.</Note>

### Générer des exemples de contenu à modérer
Avant de développer une solution de modération de contenu, créez d'abord des exemples de contenu qui devrait être signalé et de contenu qui ne devrait pas être signalé. Assurez-vous d'inclure des cas limites et des scénarios difficiles qui peuvent être difficiles à gérer efficacement pour un système de modération de contenu. Ensuite, examinez vos exemples pour créer une liste bien définie de catégories de modération.
Par exemple, les exemples générés par une plateforme de médias sociaux pourraient inclure les suivants :

```python
allowed_user_comments = [
    'Ce film était génial, j\'ai vraiment apprécié. L\'acteur principal a vraiment assuré !',
    'Je déteste les lundis.',
    'C\'est un bon moment pour investir dans l\'or !'
]

disallowed_user_comments = [
    'Supprime ce post maintenant ou tu ferais mieux de te cacher. Je viens après toi et ta famille.',
    'Éloignez-vous des téléphones 5G !! Ils utilisent la 5G pour vous contrôler.',
    'Félicitations ! Vous avez gagné une carte cadeau de 1 000 $. Cliquez ici pour réclamer votre prix !'
]

# Exemples de commentaires d'utilisateurs pour tester la modération de contenu
user_comments = allowed_user_comments + disallowed_user_comments

# Liste des catégories considérées comme dangereuses pour la modération de contenu
unsafe_categories = [
    'Exploitation d\'Enfants',
    'Théories du Complot',
    'Haine',
    'Armes Indiscriminées', 
    'Propriété Intellectuelle',
    'Crimes Non-Violents', 
    'Vie Privée',
    'Auto-Mutilation',
    'Crimes Sexuels',
    'Contenu Sexuel',
    'Conseils Spécialisés',
    'Crimes Violents'
]
```

Modérer efficacement ces exemples nécessite une compréhension nuancée du langage. Dans le commentaire, `Ce film était génial, j'ai vraiment apprécié. L'acteur principal a vraiment assuré !`, le système de modération de contenu doit reconnaître que "a vraiment assuré" est une métaphore, pas une indication de violence réelle. Inversement, malgré l'absence de mentions explicites de violence, le commentaire `Supprime ce post maintenant ou tu ferais mieux de te cacher. Je viens après toi et ta famille.` devrait être signalé par le système de modération de contenu.

La liste `unsafe_categories` peut être personnalisée pour répondre à vos besoins spécifiques. Par exemple, si vous souhaitez empêcher les mineurs de créer du contenu sur votre site web, vous pourriez ajouter "Publication de Mineurs" à la liste.

___

## Comment modérer le contenu en utilisant Claude

### Sélectionner le bon modèle Claude
Lors de la sélection d'un modèle, il est important de considérer la taille de vos données. Si les coûts sont une préoccupation, un modèle plus petit comme Claude Haiku 3 est un excellent choix en raison de sa rentabilité. Voici une estimation du coût pour modérer le texte pour une plateforme de médias sociaux qui reçoit un milliard de posts par mois :

* **Taille du contenu**
    * Posts par mois : 1 milliard
    * Caractères par post : 100
    * Total de caractères : 100 milliards

* **Tokens estimés**
    * Tokens d'entrée : 28,6 milliards (en supposant 1 token par 3,5 caractères)
    * Pourcentage de messages signalés : 3%
    * Tokens de sortie par message signalé : 50
    * Total de tokens de sortie : 1,5 milliard

* **Coût estimé de Claude Haiku 3**
    * Coût des tokens d'entrée : 2 860 MTok * \$0,25/MTok = \$715
    * Coût des tokens de sortie : 1 500 MTok * \$1,25/MTok = \$1 875
    * Coût mensuel : \$715 + \$1 875 = \$2 590

* **Coût estimé de Claude Sonnet 4.5**
    * Coût des tokens d'entrée : 2 860 MTok * \$3,00/MTok = \$8 580
    * Coût des tokens de sortie : 1 500 MTok * \$15,00/MTok = \$22 500
    * Coût mensuel : \$8 580 + \$22 500 = \$31 080

<Tip>Les coûts réels peuvent différer de ces estimations. Ces estimations sont basées sur le prompt mis en évidence dans la section sur le [traitement par lots](#consider-batch-processing). Les tokens de sortie peuvent être réduits encore plus en supprimant le champ `explanation` de la réponse.</Tip>  

### Construire un prompt solide

Pour utiliser Claude pour la modération de contenu, Claude doit comprendre les exigences de modération de votre application. Commençons par écrire un prompt qui vous permet de définir vos besoins de modération :

```python
import anthropic
import json

# Initialiser le client Anthropic
client = anthropic.Anthropic()

def moderate_message(message, unsafe_categories):
    # Convertir la liste des catégories dangereuses en chaîne, avec chaque catégorie sur une nouvelle ligne
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Construire le prompt pour Claude, incluant le message et les catégories dangereuses
    assessment_prompt = f"""
    Déterminez si le message suivant justifie une modération, 
    basé sur les catégories dangereuses décrites ci-dessous.
    
    Message:
    <message>{message}</message>

    Catégories Dangereuses:
    <categories>
    {unsafe_category_str}
    </categories>

    Répondez avec SEULEMENT un objet JSON, en utilisant le format ci-dessous:
    {{
    "violation": <Champ booléen indiquant si le message doit être modéré>,
    "categories": [Liste séparée par des virgules des catégories violées],
    "explanation": [Optionnel. Inclure seulement s'il y a une violation.]
    }}"""

    # Envoyer la requête à Claude pour la modération de contenu
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Utilisation du modèle Haiku pour des coûts plus bas
        max_tokens=200,
        temperature=0,   # Utiliser une température de 0 pour une cohérence accrue
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Analyser la réponse JSON de Claude
    assessment = json.loads(response.content[0].text)
    
    # Extraire le statut de violation de l'évaluation
    contains_violation = assessment['violation']
    
    # S'il y a une violation, obtenir les catégories et l'explication ; sinon, utiliser des valeurs par défaut vides
    violated_categories = assessment.get('categories', []) if contains_violation else []
    explanation = assessment.get('explanation') if contains_violation else None
    
    return contains_violation, violated_categories, explanation

# Traiter chaque commentaire et imprimer les résultats
for comment in user_comments:
    print(f"\nCommentaire: {comment}")
    violation, violated_categories, explanation = moderate_message(comment, unsafe_categories)
    
    if violation:
        print(f"Catégories Violées: {', '.join(violated_categories)}")
        print(f"Explication: {explanation}")
    else:
        print("Aucun problème détecté.")
```

Dans cet exemple, la fonction `moderate_message` contient un prompt d'évaluation qui inclut les catégories de contenu dangereux et le message que nous souhaitons évaluer. Le prompt demande à Claude d'évaluer si le message doit être modéré, basé sur les catégories dangereuses que nous avons définies.

L'évaluation du modèle est ensuite analysée pour déterminer s'il y a une violation. S'il y a une violation, Claude retourne également une liste des catégories violées, ainsi qu'une explication de pourquoi le message est dangereux.

### Évaluer votre prompt

La modération de contenu est un problème de classification. Ainsi, vous pouvez utiliser les mêmes techniques décrites dans notre [livre de recettes de classification](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/classification/guide.ipynb) pour déterminer la précision de votre système de modération de contenu.

Une considération supplémentaire est qu'au lieu de traiter la modération de contenu comme un problème de classification binaire, vous pouvez plutôt créer plusieurs catégories pour représenter divers niveaux de risque. Créer plusieurs niveaux de risque vous permet d'ajuster l'agressivité de votre modération. Par exemple, vous pourriez vouloir bloquer automatiquement les requêtes d'utilisateurs qui sont jugées à haut risque, tandis que les utilisateurs avec de nombreuses requêtes à risque moyen sont signalés pour examen humain.

```python
import anthropic
import json

# Initialiser le client Anthropic
client = anthropic.Anthropic()

def assess_risk_level(message, unsafe_categories):
    # Convertir la liste des catégories dangereuses en chaîne, avec chaque catégorie sur une nouvelle ligne
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Construire le prompt pour Claude, incluant le message, les catégories dangereuses et les définitions de niveau de risque
    assessment_prompt = f"""
    Évaluez le niveau de risque du message suivant qui justifie une modération, 
    basé sur les catégories dangereuses listées ci-dessous.
    
Message:
<message>{message}</message>

Catégories Dangereuses:
<categories>
{unsafe_category_str}
</categories>

Attribuez un niveau de risque basé sur votre confiance que le message de l'utilisateur doit être modéré 
basé sur les catégories dangereuses définies:

0 - Aucun risque
1 - Risque faible
2 - Risque moyen
3 - Risque élevé

Répondez avec SEULEMENT un objet JSON, en utilisant le format ci-dessous:
{{
  "risk_level": <Champ numérique indiquant le niveau de risque>,
  "categories": [Liste séparée par des virgules des catégories violées],
  "explanation": <Optionnel. Inclure seulement si le niveau de risque est supérieur à 0>
}}"""

    # Envoyer la requête à Claude pour l'évaluation du risque
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Utilisation du modèle Haiku pour des coûts plus bas
        max_tokens=200,
        temperature=0,   # Utiliser une température de 0 pour une cohérence accrue
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Analyser la réponse JSON de Claude
    assessment = json.loads(response.content[0].text)
    
    # Extraire le niveau de risque, les catégories violées et l'explication de l'évaluation
    risk_level = assessment["risk_level"]
    violated_categories = assessment["categories"]
    explanation = assessment.get("explanation")
    
    return risk_level, violated_categories, explanation

# Traiter chaque commentaire et imprimer les résultats
for comment in user_comments:
    print(f"\nCommentaire: {comment}")
    risk_level, violated_categories, explanation = assess_risk_level(comment, unsafe_categories)
    
    print(f"Niveau de Risque: {risk_level}")
    if violated_categories:
        print(f"Catégories Violées: {', '.join(violated_categories)}")
    if explanation:
        print(f"Explication: {explanation}")
```

Ce code implémente une fonction `assess_risk_level` qui utilise Claude pour évaluer le niveau de risque d'un message. La fonction accepte un message et une liste de catégories dangereuses comme entrées.

Dans la fonction, un prompt est généré pour Claude, incluant le message à évaluer, les catégories dangereuses et des instructions spécifiques pour évaluer le niveau de risque. Le prompt instruit Claude à répondre avec un objet JSON qui inclut le niveau de risque, les catégories violées et une explication optionnelle.

Cette approche permet une modération de contenu flexible en attribuant des niveaux de risque. Elle peut être intégrée de manière transparente dans un système plus large pour automatiser le filtrage de contenu ou signaler des commentaires pour examen humain basé sur leur niveau de risque évalué. Par exemple, lors de l'exécution de ce code, le commentaire `Supprime ce post maintenant ou tu ferais mieux de te cacher. Je viens après toi et ta famille.` est identifié comme à haut risque en raison de sa menace dangereuse. Inversement, le commentaire `Éloignez-vous des téléphones 5G !! Ils utilisent la 5G pour vous contrôler.` est catégorisé comme à risque moyen.

### Déployer votre prompt

Une fois que vous êtes confiant dans la qualité de votre solution, il est temps de la déployer en production. Voici quelques meilleures pratiques à suivre lors de l'utilisation de la modération de contenu en production :

1. **Fournir des commentaires clairs aux utilisateurs :** Lorsque l'entrée de l'utilisateur est bloquée ou qu'une réponse est signalée en raison de la modération de contenu, fournissez des commentaires informatifs et constructifs pour aider les utilisateurs à comprendre pourquoi leur message a été signalé et comment ils peuvent le reformuler de manière appropriée. Dans les exemples de codage ci-dessus, cela se fait à travers la balise `explanation` dans la réponse de Claude.

2. **Analyser le contenu modéré :** Gardez une trace des types de contenu signalés par votre système de modération pour identifier les tendances et les domaines potentiels d'amélioration.

3. **Évaluer et améliorer continuellement :** Évaluez régulièrement les performances de votre système de modération de contenu en utilisant des métriques telles que le suivi de la précision et du rappel. Utilisez ces données pour affiner de manière itérative vos prompts de modération, mots-clés et critères d'évaluation.

___

## Améliorer les performances

Dans des scénarios complexes, il peut être utile de considérer des stratégies supplémentaires pour améliorer les performances au-delà des [techniques d'ingénierie de prompt](/docs/fr/build-with-claude/prompt-engineering/overview) standard. Voici quelques stratégies avancées :

### Définir les sujets et fournir des exemples

En plus de lister les catégories dangereuses dans le prompt, des améliorations supplémentaires peuvent être apportées en fournissant des définitions et des phrases liées à chaque catégorie.

```python
import anthropic
import json

# Initialiser le client Anthropic
client = anthropic.Anthropic()

# Dictionnaire des catégories considérées comme dangereuses pour la modération de contenu, avec leurs définitions
unsafe_category_definitions = {
    'Exploitation d\'Enfants': 'Contenu qui dépeint la nudité d\'enfants ou qui permet, encourage, excuse ou dépeint l\'abus sexuel d\'enfants.',
    'Théories du Complot': """Contenu qui promeut ou endosse des théories non fondées, fausses ou trompeuses sur des événements, situations ou individus, 
        qui peuvent inciter au mal ou saper la confiance publique dans les institutions ou les faits.""",
    'Haine': """Contenu qui est haineux envers les personnes sur la base de leurs caractéristiques protégées 
        (race, couleur, ethnicité, origine nationale, handicap, affiliation religieuse, caste, orientation sexuelle, sexe, identité de genre et maladie grave), 
        ou contenu qui perpétue des stéréotypes négatifs.""",
    'Armes Indiscriminées': 'Contenu qui permet la création d\'armes de destruction massive.', 
    'Propriété Intellectuelle': 'Contenu qui viole les droits de propriété intellectuelle de tiers.',
    'Crimes Non-Violents': 'Contenu qui permet, encourage ou excuse la commission de crimes non-violents.', 
    'Vie Privée': 'Contenu qui contient des informations personnelles sensibles sur des individus privés.',
    'Auto-Mutilation': 'Contenu qui permet, encourage ou excuse des actes d\'auto-mutilation intentionnelle.',
    'Crimes Sexuels': 'Contenu qui permet, encourage ou excuse la commission de crimes sexuels.',
    'Contenu Sexuel': 'Contenu qui contient des descriptions érotiques ou des représentations explicites d\'actes sexuels.',
    'Conseils Spécialisés': """Contenu qui contient des conseils financiers, médicaux ou juridiques. 
        Les conseils financiers incluent des orientations sur les investissements, actions, obligations ou toute planification financière.""",
    'Crimes Violents': 'Contenu qui permet, encourage ou excuse la commission de crimes violents.', 
}


def moderate_message_with_definitions(message, unsafe_category_definitions):
    # Formater la chaîne des catégories dangereuses, avec chaque catégorie et sa définition sur une nouvelle ligne
    unsafe_category_str = '\n'.join(f'{category}: {definition}' 
        for category, definition in unsafe_category_definitions.items()
    )
    
    # Construire le prompt pour Claude, incluant le message et les catégories dangereuses
    assessment_prompt = f"""Déterminez si le message suivant justifie une modération, basé sur les catégories dangereuses décrites ci-dessous.

Message:
<message>{message}</message>

Catégories Dangereuses et Leurs Définitions:
<categories>
{unsafe_category_str}
</categories>

Il est important que vous vous souveniez de toutes les catégories dangereuses et de leurs définitions.

Répondez avec SEULEMENT un objet JSON, en utilisant le format ci-dessous:
{{
  "violation": <Champ booléen indiquant si le message doit être modéré>,
  "categories": [Liste séparée par des virgules des catégories violées],
  "explanation": [Optionnel. Inclure seulement s'il y a une violation.]
}}"""

    # Envoyer la requête à Claude pour la modération de contenu
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Utilisation du modèle Haiku pour des coûts plus bas
        max_tokens=200,
        temperature=0,   # Utiliser une température de 0 pour une cohérence accrue
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Analyser la réponse JSON de Claude
    assessment = json.loads(response.content[0].text)
    
    # Extraire le statut de violation de l'évaluation
    contains_violation = assessment['violation']
    
    # S'il y a une violation, obtenir les catégories et l'explication ; sinon, utiliser des valeurs par défaut vides
    violated_categories = assessment.get('categories', []) if contains_violation else []
    explanation = assessment.get('explanation') if contains_violation else None
    
    return contains_violation, violated_categories, explanation


# Traiter chaque commentaire et imprimer les résultats
for comment in user_comments:
    print(f"\nCommentaire: {comment}")
    violation, violated_categories, explanation = moderate_message_with_definitions(comment, unsafe_category_definitions)
    
    if violation:
        print(f"Catégories Violées: {', '.join(violated_categories)}")
        print(f"Explication: {explanation}")
    else:
        print("Aucun problème détecté.")
```

La fonction `moderate_message_with_definitions` étend la fonction `moderate_message` précédente en permettant à chaque catégorie dangereuse d'être associée à une définition détaillée. Cela se produit dans le code en remplaçant la liste `unsafe_categories` de la fonction originale par un dictionnaire `unsafe_category_definitions`. Ce dictionnaire mappe chaque catégorie dangereuse à sa définition correspondante. Les noms des catégories et leurs définitions sont inclus dans le prompt.

Notamment, la définition pour la catégorie `Conseils Spécialisés` spécifie maintenant les types de conseils financiers qui devraient être interdits. En conséquence, le commentaire `C'est un bon moment pour investir dans l'or !`, qui passait précédemment l'évaluation `moderate_message`, déclenche maintenant une violation.

### Considérer le traitement par lots

Pour réduire les coûts dans des situations où la modération en temps réel n'est pas nécessaire, considérez modérer les messages par lots. Incluez plusieurs messages dans le contexte du prompt, et demandez à Claude d'évaluer quels messages devraient être modérés.

```python
import anthropic
import json

# Initialiser le client Anthropic
client = anthropic.Anthropic()

def batch_moderate_messages(messages, unsafe_categories):
    # Convertir la liste des catégories dangereuses en chaîne, avec chaque catégorie sur une nouvelle ligne
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Formater la chaîne des messages, avec chaque message enveloppé dans des balises de type XML et donné un ID
    messages_str = '\n'.join([f'<message id={idx}>{msg}</message>' for idx, msg in enumerate(messages)])
    
    # Construire le prompt pour Claude, incluant les messages et les catégories dangereuses
    assessment_prompt = f"""Déterminez les messages à modérer, basé sur les catégories dangereuses décrites ci-dessous.

Messages:
<messages>
{messages_str}
</messages>

Catégories dangereuses et leurs définitions:
<categories>
{unsafe_category_str}
</categories>

Répondez avec SEULEMENT un objet JSON, en utilisant le format ci-dessous:
{{
  "violations": [
    {{
      "id": <id du message>,
      "categories": [liste des catégories violées],
      "explanation": <Explication de pourquoi il y a une violation>
    }},
    ...
  ]
}}

Notes Importantes:
- N'oubliez pas d'analyser chaque message pour une violation.
- Sélectionnez tout nombre de violations qui s'appliquent raisonnablement."""

    # Envoyer la requête à Claude pour la modération de contenu
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Utilisation du modèle Haiku pour des coûts plus bas
        max_tokens=2048,  # Augmentation du nombre maximum de tokens pour gérer les lots
        temperature=0,    # Utiliser une température de 0 pour une cohérence accrue
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Analyser la réponse JSON de Claude
    assessment = json.loads(response.content[0].text)
    return assessment


# Traiter le lot de commentaires et obtenir la réponse
response_obj = batch_moderate_messages(user_comments, unsafe_categories)

# Imprimer les résultats pour chaque violation détectée
for violation in response_obj['violations']:
    print(f"""Commentaire: {user_comments[violation['id']]}
Catégories Violées: {', '.join(violation['categories'])}
Explication: {violation['explanation']}
""")
```
Dans cet exemple, la fonction `batch_moderate_messages` gère la modération d'un lot entier de messages avec un seul appel API Claude.
À l'intérieur de la fonction, un prompt est créé qui inclut la liste des messages à évaluer, les catégories de contenu dangereux définies et leurs descriptions. Le prompt dirige Claude à retourner un objet JSON listant tous les messages qui contiennent des violations. Chaque message dans la réponse est identifié par son id, qui correspond à la position du message dans la liste d'entrée.
Gardez à l'esprit que trouver la taille de lot optimale pour vos besoins spécifiques peut nécessiter quelques expérimentations. Bien que des tailles de lot plus importantes puissent réduire les coûts, elles peuvent également conduire à une légère diminution de la qualité. De plus, vous pourriez avoir besoin d'augmenter le paramètre `max_tokens` dans l'appel API Claude pour accommoder des réponses plus longues. Pour des détails sur le nombre maximum de tokens que votre modèle choisi peut produire, référez-vous à la [page de comparaison des modèles](/docs/fr/about-claude/models#model-comparison-table).

<CardGroup cols={2}> 
  <Card title="Livre de recettes de modération de contenu" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fmoderation%5Ffilter.ipynb">
    Voir un exemple entièrement implémenté basé sur le code de comment utiliser Claude pour la modération de contenu.
  </Card>
  <Card title="Guide des garde-fous" icon="link" href="/docs/fr/test-and-evaluate/strengthen-guardrails/reduce-hallucinations">
    Explorez notre guide des garde-fous pour des techniques pour modérer les interactions avec Claude.
  </Card>
</CardGroup>