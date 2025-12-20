# Support multilingue

Claude excelle dans les tâches multilingues, maintenant une performance cross-linguale solide par rapport à l'anglais.

---

## Aperçu

Claude démontre des capacités multilingues robustes, avec une performance particulièrement forte dans les tâches zero-shot dans plusieurs langues. Le modèle maintient une performance relative cohérente dans les langues largement parlées et les langues à ressources limitées, ce qui en fait un choix fiable pour les applications multilingues.

Notez que Claude est capable dans de nombreuses langues au-delà de celles évaluées ci-dessous. Nous vous encourageons à tester avec toutes les langues pertinentes pour vos cas d'usage spécifiques.

## Données de performance

Ci-dessous se trouvent les scores d'évaluation zero-shot chain-of-thought pour les modèles Claude dans différentes langues, affichés en pourcentage relatif à la performance en anglais (100%) :

| Langue | Claude Opus 4.1<sup>1</sup> | Claude Opus 4<sup>1</sup> | Claude Sonnet 4.5<sup>1</sup> | Claude Sonnet 4<sup>1</sup> | Claude Haiku 4.5<sup>1</sup> |
|----------|---------------|---------------|---------------|-----------------|------------------|
| Anglais (référence, fixé à 100%) | 100% | 100% | 100% | 100% | 100% |
| Espagnol | 98.1% | 98.0% | 98.2% | 97.5% | 96.4% |
| Portugais (Brésil) | 97.8% | 97.3% | 97.8% | 97.2% | 96.1% |
| Italien | 97.7% | 97.5% | 97.9% | 97.3% | 96.0% |
| Français | 97.9% | 97.7% | 97.5% | 97.1% | 95.7% |
| Indonésien | 97.3% | 97.2% | 97.3% | 96.2% | 94.2% |
| Allemand | 97.7% | 97.1% | 97.0% | 94.7% | 94.3% |
| Arabe | 97.1% | 96.9% | 97.2% | 96.1% | 92.5% |
| Chinois (Simplifié) | 97.1% | 96.7% | 96.9% | 95.9% | 94.2% |
| Coréen | 96.6% | 96.4% | 96.7% | 95.9% | 93.3% |
| Japonais | 96.9% | 96.2% | 96.8% | 95.6% | 93.5% |
| Hindi | 96.8% | 96.7% | 96.7% | 95.8% | 92.4% |
| Bengali | 95.7% | 95.2% | 95.4% | 94.4% | 90.4% |
| Swahili | 89.8% | 89.5% | 91.1% | 87.1% | 78.3% |
| Yoruba | 80.3% | 78.9% | 79.7% | 76.4% | 52.7% |

<sup>1</sup> Avec [extended thinking](/docs/fr/build-with-claude/extended-thinking).

<Note>
Ces métriques sont basées sur les ensembles de tests MMLU (Massive Multitask Language Understanding) en anglais qui ont été traduits dans 14 langues supplémentaires par des traducteurs humains professionnels, comme documenté dans [le référentiel simple-evals d'OpenAI](https://github.com/openai/simple-evals/blob/main/multilingual_mmlu_benchmark_results.md). L'utilisation de traducteurs humains pour cette évaluation garantit des traductions de haute qualité, particulièrement importante pour les langues avec moins de ressources numériques.
</Note>

***

## Meilleures pratiques

Lorsque vous travaillez avec du contenu multilingue :

1. **Fournir un contexte linguistique clair** : Bien que Claude puisse détecter automatiquement la langue cible, énoncer explicitement la langue d'entrée/sortie souhaitée améliore la fiabilité. Pour une fluidité améliorée, vous pouvez inviter Claude à utiliser « un langage idiomatique comme s'il était un locuteur natif ».
2. **Utiliser les scripts natifs** : Soumettez le texte dans son script natif plutôt que sa translittération pour des résultats optimaux
3. **Considérer le contexte culturel** : Une communication efficace nécessite souvent une sensibilité culturelle et régionale au-delà de la simple traduction

Nous vous suggérons également de suivre nos [directives générales d'ingénierie des invites](/docs/fr/build-with-claude/prompt-engineering/overview) pour améliorer davantage la performance de Claude.

***

## Considérations relatives au support linguistique

- Claude traite l'entrée et génère la sortie dans la plupart des langues mondiales qui utilisent des caractères Unicode standard
- La performance varie selon la langue, avec des capacités particulièrement fortes dans les langues largement parlées
- Même dans les langues avec moins de ressources numériques, Claude maintient des capacités significatives

<CardGroup cols={2}>
  <Card title="Guide d'ingénierie des invites" icon="edit" href="/docs/fr/build-with-claude/prompt-engineering/overview">
    Maîtrisez l'art de la rédaction d'invites pour tirer le meilleur parti de Claude.
  </Card>
  <Card title="Bibliothèque d'invites" icon="books" href="/docs/fr/resources/prompt-library">
    Trouvez un large éventail d'invites pré-rédigées pour diverses tâches et industries. Parfait pour l'inspiration ou les démarrages rapides.
  </Card>
</CardGroup>