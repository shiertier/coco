# Résumé de documents juridiques

Ce guide explique comment exploiter les capacités avancées de traitement du langage naturel de Claude pour résumer efficacement les documents juridiques, en extrayant les informations clés et en accélérant la recherche juridique. Avec Claude, vous pouvez rationaliser l'examen des contrats, la préparation des litiges et les travaux réglementaires, en économisant du temps et en garantissant la précision de vos processus juridiques.

---

> Visitez notre [guide de résumé](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb) pour voir un exemple d'implémentation de résumé juridique utilisant Claude.

## Avant de construire avec Claude

### Décidez si vous devez utiliser Claude pour le résumé de documents juridiques

Voici quelques indicateurs clés selon lesquels vous devriez employer un LLM comme Claude pour résumer des documents juridiques :

<section title="Vous souhaitez examiner un grand volume de documents efficacement et de manière abordable">
L'examen de documents à grande échelle peut être long et coûteux lorsqu'il est effectué manuellement. Claude peut traiter et résumer rapidement de vastes quantités de documents juridiques, réduisant considérablement le temps et les coûts associés à l'examen des documents. Cette capacité est particulièrement précieuse pour des tâches telles que la diligence raisonnable, l'analyse de contrats ou la découverte en litige, où l'efficacité est cruciale.
</section>
<section title="Vous avez besoin d'une extraction automatisée des métadonnées clés">
Claude peut extraire et catégoriser efficacement les métadonnées importantes des documents juridiques, telles que les parties impliquées, les dates, les conditions du contrat ou les clauses spécifiques. Cette extraction automatisée peut aider à organiser les informations, ce qui facilite la recherche, l'analyse et la gestion de grands ensembles de documents. C'est particulièrement utile pour la gestion des contrats, les vérifications de conformité ou la création de bases de données consultables d'informations juridiques.
</section>
<section title="Vous souhaitez générer des résumés clairs, concis et standardisés">
Claude peut générer des résumés structurés qui suivent des formats prédéterminés, ce qui facilite pour les professionnels du droit de saisir rapidement les points clés de divers documents. Ces résumés standardisés peuvent améliorer la lisibilité, faciliter la comparaison entre les documents et améliorer la compréhension globale, en particulier lorsqu'il s'agit de langage juridique complexe ou de jargon technique.
</section>
<section title="Vous avez besoin de citations précises pour vos résumés">
Lors de la création de résumés juridiques, l'attribution appropriée et la citation sont cruciales pour assurer la crédibilité et la conformité aux normes juridiques. Claude peut être invité à inclure des citations précises pour tous les points juridiques référencés, ce qui facilite pour les professionnels du droit d'examiner et de vérifier les informations résumées.
</section>
<section title="Vous souhaitez rationaliser et accélérer votre processus de recherche juridique">
Claude peut aider à la recherche juridique en analysant rapidement de grands volumes de jurisprudence, de statuts et de commentaires juridiques. Il peut identifier les précédents pertinents, extraire les principes juridiques clés et résumer les arguments juridiques complexes. Cette capacité peut accélérer considérablement le processus de recherche, permettant aux professionnels du droit de se concentrer sur l'analyse de niveau supérieur et le développement de stratégies.
</section>

### Déterminez les détails que vous souhaitez que le résumé extraie
Il n'existe pas de résumé unique et correct pour un document donné. Sans direction claire, il peut être difficile pour Claude de déterminer quels détails inclure. Pour obtenir des résultats optimaux, identifiez les informations spécifiques que vous souhaitez inclure dans le résumé.

Par exemple, lors de la résumé d'un accord de sous-location, vous pourriez souhaiter extraire les points clés suivants :

```python
details_to_extract = [
    'Parties involved (sublessor, sublessee, and original lessor)',
    'Property details (address, description, and permitted use)', 
    'Term and rent (start date, end date, monthly rent, and security deposit)',
    'Responsibilities (utilities, maintenance, and repairs)',
    'Consent and notices (landlord\'s consent, and notice requirements)',
    'Special provisions (furniture, parking, and subletting restrictions)'
]
```

### Établissez des critères de succès

L'évaluation de la qualité des résumés est une tâche notoirement difficile. Contrairement à de nombreuses autres tâches de traitement du langage naturel, l'évaluation des résumés manque souvent de métriques claires et objectives. Le processus peut être très subjectif, les différents lecteurs valorisant différents aspects d'un résumé. Voici les critères que vous pourriez souhaiter considérer lors de l'évaluation de la performance de Claude en matière de résumé juridique.

<section title="Exactitude factuelle">
Le résumé doit représenter avec précision les faits, les concepts juridiques et les points clés du document.
</section>
<section title="Précision juridique">
La terminologie et les références aux statuts, à la jurisprudence ou aux réglementations doivent être correctes et alignées sur les normes juridiques.
</section>
<section title="Concision">
Le résumé doit condenser le document juridique à ses points essentiels sans perdre les détails importants.
</section>
<section title="Cohérence">
Si vous résumez plusieurs documents, le LLM doit maintenir une structure et une approche cohérentes pour chaque résumé.
</section>
<section title="Lisibilité">
Le texte doit être clair et facile à comprendre. Si le public n'est pas composé d'experts juridiques, le résumé ne doit pas inclure de jargon juridique qui pourrait confondre le public.
</section>
<section title="Biais et équité">
Le résumé doit présenter une représentation impartiale et équitable des arguments juridiques et des positions.
</section>

Consultez notre guide sur [l'établissement de critères de succès](/docs/fr/test-and-evaluate/define-success) pour plus d'informations.

---

## Comment résumer des documents juridiques en utilisant Claude

### Sélectionnez le bon modèle Claude

La précision du modèle est extrêmement importante lors de la résumé de documents juridiques. Claude Sonnet 4.5 est un excellent choix pour les cas d'usage comme celui-ci où une haute précision est requise. Si la taille et la quantité de vos documents sont importantes au point que les coûts commencent à devenir une préoccupation, vous pouvez également essayer d'utiliser un modèle plus petit comme Claude Haiku 4.5.

Pour aider à estimer ces coûts, voici une comparaison du coût pour résumer 1 000 accords de sous-location en utilisant à la fois Sonnet et Haiku :

* **Taille du contenu**
    * Nombre d'accords : 1 000
    * Caractères par accord : 300 000
    * Nombre total de caractères : 300M

* **Jetons estimés**
    * Jetons d'entrée : 86M (en supposant 1 jeton pour 3,5 caractères)
    * Jetons de sortie par résumé : 350
    * Nombre total de jetons de sortie : 350 000
 
* **Coût estimé de Claude Sonnet 4.5**
    * Coût des jetons d'entrée : 86 MTok * \$3,00/MTok = \$258
    * Coût des jetons de sortie : 0,35 MTok * \$15,00/MTok = \$5,25
    * Coût total : \$258,00 + \$5,25 = \$263,25

* **Coût estimé de Claude Haiku 3**
    * Coût des jetons d'entrée : 86 MTok * \$0,25/MTok = \$21,50
    * Coût des jetons de sortie : 0,35 MTok * \$1,25/MTok = \$0,44
    * Coût total : \$21,50 + \$0,44 = \$21,96

<Tip>Les coûts réels peuvent différer de ces estimations. Ces estimations sont basées sur l'exemple mis en évidence dans la section sur [l'élaboration de prompts](#build-a-strong-prompt).</Tip>

### Transformez les documents en un format que Claude peut traiter

Avant de commencer à résumer des documents, vous devez préparer vos données. Cela implique d'extraire du texte à partir de fichiers PDF, de nettoyer le texte et de vous assurer qu'il est prêt à être traité par Claude.

Voici une démonstration de ce processus sur un exemple de PDF :

```python
from io import BytesIO
import re

import pypdf
import requests

def get_llm_text(pdf_file):
    reader = pypdf.PdfReader(pdf_file)
    text = "\n".join([page.extract_text() for page in reader.pages])

    # Remove extra whitespace
    text = re.sub(r'\s+', ' ', text) 

    # Remove page numbers
    text = re.sub(r'\n\s*\d+\s*\n', '\n', text) 

    return text


# Create the full URL from the GitHub repository
url = "https://raw.githubusercontent.com/anthropics/anthropic-cookbook/main/skills/summarization/data/Sample Sublease Agreement.pdf"
url = url.replace(" ", "%20")

# Download the PDF file into memory
response = requests.get(url)

# Load the PDF from memory
pdf_file = BytesIO(response.content)

document_text = get_llm_text(pdf_file) 
print(document_text[:50000]) 
```

Dans cet exemple, nous téléchargeons d'abord un PDF d'un exemple d'accord de sous-location utilisé dans le [guide de résumé](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/data/Sample%20Sublease%20Agreement.pdf). Cet accord a été obtenu à partir d'un accord de sous-location disponible publiquement sur le [site web sec.gov](https://www.sec.gov/Archives/edgar/data/1045425/000119312507044370/dex1032.htm).

Nous utilisons la bibliothèque pypdf pour extraire le contenu du PDF et le convertir en texte. Les données textuelles sont ensuite nettoyées en supprimant les espaces blancs supplémentaires et les numéros de page.

### Construisez un prompt solide

Claude peut s'adapter à différents styles de résumé. Vous pouvez modifier les détails du prompt pour guider Claude à être plus ou moins verbeux, inclure plus ou moins de terminologie technique, ou fournir un résumé de niveau supérieur ou inférieur du contexte en question.

Voici un exemple de la façon de créer un prompt qui garantit que les résumés générés suivent une structure cohérente lors de l'analyse des accords de sous-location :

```python
import anthropic

# Initialize the Anthropic client
client = anthropic.Anthropic()

def summarize_document(text, details_to_extract, model="claude-sonnet-4-5", max_tokens=1000):

    # Format the details to extract to be placed within the prompt's context
    details_to_extract_str = '\n'.join(details_to_extract)
    
    # Prompt the model to summarize the sublease agreement
    prompt = f"""Summarize the following sublease agreement. Focus on these key aspects:

    {details_to_extract_str}

    Provide the summary in bullet points nested within the XML header for each section. For example:

    <parties involved>
    - Sublessor: [Name]
    // Add more details as needed
    </parties involved>
    
    If any information is not explicitly stated in the document, note it as "Not specified". Do not preamble.

    Sublease agreement text:
    {text}
    """

    response = client.messages.create(
        model=model,
        max_tokens=max_tokens,
        system="You are a legal analyst specializing in real estate law, known for highly accurate and detailed summaries of sublease agreements.",
        messages=[
            {"role": "user", "content": prompt},
            {"role": "assistant", "content": "Here is the summary of the sublease agreement: <summary>"}
        ],
        stop_sequences=["</summary>"]
    )

    return response.content[0].text

sublease_summary = summarize_document(document_text, details_to_extract)
print(sublease_summary)
```

Ce code implémente une fonction `summarize_document` qui utilise Claude pour résumer le contenu d'un accord de sous-location. La fonction accepte une chaîne de texte et une liste de détails à extraire comme entrées. Dans cet exemple, nous appelons la fonction avec les variables `document_text` et `details_to_extract` qui ont été définies dans les extraits de code précédents.

Dans la fonction, un prompt est généré pour Claude, incluant le document à résumer, les détails à extraire et des instructions spécifiques pour résumer le document. Le prompt demande à Claude de répondre avec un résumé de chaque détail à extraire imbriqué dans des en-têtes XML.

Parce que nous avons décidé de produire chaque section du résumé dans des balises, chaque section peut facilement être analysée comme une étape de post-traitement. Cette approche permet des résumés structurés qui peuvent être adaptés à votre cas d'usage, de sorte que chaque résumé suit le même modèle.

### Évaluez votre prompt

L'élaboration de prompts nécessite souvent des tests et une optimisation pour être prête pour la production. Pour déterminer la préparation de votre solution, évaluez la qualité de vos résumés en utilisant un processus systématique combinant des méthodes quantitatives et qualitatives. La création d'une [évaluation empirique solide](/docs/fr/test-and-evaluate/develop-tests#building-evals-and-test-cases) basée sur vos critères de succès définis vous permettra d'optimiser vos prompts. Voici quelques métriques que vous pourriez souhaiter inclure dans votre évaluation empirique :

<section title="Scores ROUGE">
Cela mesure le chevauchement entre le résumé généré et un résumé de référence créé par un expert. Cette métrique se concentre principalement sur le rappel et est utile pour évaluer la couverture du contenu.
</section>
<section title="Scores BLEU">
Bien que développé à l'origine pour la traduction automatique, cette métrique peut être adaptée pour les tâches de résumé. Les scores BLEU mesurent la précision des correspondances n-grammes entre le résumé généré et les résumés de référence. Un score plus élevé indique que le résumé généré contient des phrases et une terminologie similaires au résumé de référence.
</section>
<section title="Similarité d'intégration contextuelle">
Cette métrique implique de créer des représentations vectorielles (intégrations) des résumés générés et de référence. La similarité entre ces intégrations est ensuite calculée, souvent en utilisant la similarité cosinus. Des scores de similarité plus élevés indiquent que le résumé généré capture le sens sémantique et le contexte du résumé de référence, même si la formulation exacte diffère.
</section>
<section title="Notation basée sur LLM">
Cette méthode implique d'utiliser un LLM tel que Claude pour évaluer la qualité des résumés générés par rapport à une rubrique de notation. La rubrique peut être adaptée à vos besoins spécifiques, évaluant les facteurs clés tels que la précision, l'exhaustivité et la cohérence. Pour des conseils sur la mise en œuvre de la notation basée sur LLM, consultez ces [conseils](/docs/fr/test-and-evaluate/develop-tests#tips-for-llm-based-grading).
</section>
<section title="Évaluation humaine">
En plus de créer les résumés de référence, les experts juridiques peuvent également évaluer la qualité des résumés générés. Bien que cela soit coûteux et long à grande échelle, cela est souvent fait sur quelques résumés comme vérification de cohérence avant le déploiement en production.
</section>

### Déployez votre prompt

Voici quelques considérations supplémentaires à garder à l'esprit lors du déploiement de votre solution en production.

1. **Assurez-vous qu'il n'y a pas de responsabilité :** Comprenez les implications juridiques des erreurs dans les résumés, qui pourraient entraîner une responsabilité juridique pour votre organisation ou vos clients. Fournissez des avertissements ou des avis juridiques clarifiant que les résumés sont générés par l'IA et doivent être examinés par des professionnels du droit.

2. **Gérez les types de documents divers :** Dans ce guide, nous avons discuté de la façon d'extraire du texte à partir de fichiers PDF. Dans le monde réel, les documents peuvent être dans une variété de formats (fichiers PDF, documents Word, fichiers texte, etc.). Assurez-vous que votre pipeline d'extraction de données peut convertir tous les formats de fichiers que vous vous attendez à recevoir.

3. **Parallélisez les appels API à Claude :** Les documents longs avec un grand nombre de jetons peuvent nécessiter jusqu'à une minute pour que Claude génère un résumé. Pour les grandes collections de documents, vous pourriez souhaiter envoyer les appels API à Claude en parallèle afin que les résumés puissent être complétés dans un délai raisonnable. Reportez-vous aux [limites de débit](/docs/fr/api/rate-limits#rate-limits) d'Anthropic pour déterminer le nombre maximal d'appels API qui peuvent être effectués en parallèle.

---

## Améliorer les performances

Dans les scénarios complexes, il peut être utile de considérer des stratégies supplémentaires pour améliorer les performances au-delà des [techniques standard d'élaboration de prompts](/docs/fr/build-with-claude/prompt-engineering/overview). Voici quelques stratégies avancées :

### Effectuez un méta-résumé pour résumer les documents longs

La résumé juridique implique souvent de gérer des documents longs ou plusieurs documents connexes à la fois, de sorte que vous dépassez la fenêtre de contexte de Claude. Vous pouvez utiliser une méthode de chunking connue sous le nom de méta-résumé afin de gérer ce cas d'usage. Cette technique implique de diviser les documents en petits morceaux gérables et de traiter ensuite chaque morceau séparément. Vous pouvez ensuite combiner les résumés de chaque morceau pour créer un méta-résumé du document entier.

Voici un exemple de la façon d'effectuer un méta-résumé :

```python
import anthropic

# Initialize the Anthropic client
client = anthropic.Anthropic()

def chunk_text(text, chunk_size=20000):
    return [text[i:i+chunk_size] for i in range(0, len(text), chunk_size)]

def summarize_long_document(text, details_to_extract, model="claude-sonnet-4-5", max_tokens=1000):

    # Format the details to extract to be placed within the prompt's context
    details_to_extract_str = '\n'.join(details_to_extract)

    # Iterate over chunks and summarize each one
    chunk_summaries = [summarize_document(chunk, details_to_extract, model=model, max_tokens=max_tokens) for chunk in chunk_text(text)]
    
    final_summary_prompt = f"""
    
    You are looking at the chunked summaries of multiple documents that are all related. 
    Combine the following summaries of the document from different truthful sources into a coherent overall summary:

    <chunked_summaries>
    {"".join(chunk_summaries)}
    </chunked_summaries>

    Focus on these key aspects:
    {details_to_extract_str})

    Provide the summary in bullet points nested within the XML header for each section. For example:

    <parties involved>
    - Sublessor: [Name]
    // Add more details as needed
    </parties involved>
    
    If any information is not explicitly stated in the document, note it as "Not specified". Do not preamble.
    """

    response = client.messages.create(
        model=model,
        max_tokens=max_tokens,
        system="You are a legal expert that summarizes notes on one document.",
        messages=[
            {"role": "user",  "content": final_summary_prompt},
            {"role": "assistant", "content": "Here is the summary of the sublease agreement: <summary>"}

        ],
        stop_sequences=["</summary>"]
    )
    
    return response.content[0].text

long_summary = summarize_long_document(document_text, details_to_extract)
print(long_summary)
```

La fonction `summarize_long_document` s'appuie sur la fonction `summarize_document` antérieure en divisant le document en morceaux plus petits et en résumant chaque morceau individuellement.

Le code réalise cela en appliquant la fonction `summarize_document` à chaque morceau de 20 000 caractères dans le document original. Les résumés individuels sont ensuite combinés, et un résumé final est créé à partir de ces résumés de morceaux.

Notez que la fonction `summarize_long_document` n'est pas strictement nécessaire pour notre exemple de PDF, car le document entier s'adapte à la fenêtre de contexte de Claude. Cependant, elle devient essentielle pour les documents dépassant la fenêtre de contexte de Claude ou lors de la résumé de plusieurs documents connexes ensemble. Quoi qu'il en soit, cette technique de méta-résumé capture souvent des détails importants supplémentaires dans le résumé final qui ont été manqués dans l'approche de résumé unique antérieure.

### Utilisez des documents indexés par résumé pour explorer une grande collection de documents

La recherche dans une collection de documents avec un LLM implique généralement la génération augmentée par récupération (RAG). Cependant, dans les scénarios impliquant des documents volumineux ou lorsque la récupération d'informations précises est cruciale, une approche RAG basique peut être insuffisante. Les documents indexés par résumé constituent une approche RAG avancée qui fournit un moyen plus efficace de classer les documents pour la récupération, en utilisant moins de contexte que les méthodes RAG traditionnelles. Dans cette approche, vous utilisez d'abord Claude pour générer un résumé concis pour chaque document de votre corpus, puis vous utilisez Claude pour classer la pertinence de chaque résumé par rapport à la requête posée. Pour plus de détails sur cette approche, y compris un exemple basé sur le code, consultez la section des documents indexés par résumé dans le [guide de résumé](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb).

### Affinez Claude pour apprendre de votre ensemble de données

Une autre technique avancée pour améliorer la capacité de Claude à générer des résumés est l'affinage. L'affinage implique d'entraîner Claude sur un ensemble de données personnalisé qui s'aligne spécifiquement avec vos besoins de résumé juridique, en garantissant que Claude s'adapte à votre cas d'usage. Voici un aperçu de la façon d'effectuer l'affinage :

1. **Identifiez les erreurs :** Commencez par collecter les instances où les résumés de Claude sont insuffisants - cela pourrait inclure des détails juridiques critiques manquants, une mauvaise compréhension du contexte ou l'utilisation d'une terminologie juridique inappropriée.

2. **Organisez un ensemble de données :** Une fois que vous avez identifié ces problèmes, compilez un ensemble de données de ces exemples problématiques. Cet ensemble de données doit inclure les documents juridiques originaux aux côtés de vos résumés corrigés, en garantissant que Claude apprend le comportement souhaité.

3. **Effectuez l'affinage :** L'affinage implique de réentraîner le modèle sur votre ensemble de données organisé pour ajuster ses poids et paramètres. Ce réentraînement aide Claude à mieux comprendre les exigences spécifiques de votre domaine juridique, améliorant sa capacité à résumer les documents selon vos normes.

4. **Amélioration itérative :** L'affinage n'est pas un processus unique. À mesure que Claude continue à générer des résumés, vous pouvez itérativement ajouter de nouveaux exemples où il a sous-performé, affinant davantage ses capacités. Au fil du temps, cette boucle de rétroaction continue entraînera un modèle hautement spécialisé pour vos tâches de résumé juridique.

<Tip>L'affinage n'est actuellement disponible que via Amazon Bedrock. Des détails supplémentaires sont disponibles dans le [blog de lancement AWS](https://aws.amazon.com/blogs/machine-learning/fine-tune-anthropics-claude-3-haiku-in-amazon-bedrock-to-boost-model-accuracy-and-quality/).</Tip>

<CardGroup cols={2}> 
  <Card title="Guide de résumé" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb">
    Consultez un exemple de code entièrement implémenté de la façon d'utiliser Claude pour résumer les contrats.
  </Card>
  <Card title="Guide des citations" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb">
    Explorez notre recette du guide des citations pour obtenir des conseils sur la façon d'assurer la précision et l'explicabilité des informations.
  </Card>
</CardGroup>