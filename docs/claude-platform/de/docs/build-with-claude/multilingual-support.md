# Mehrsprachige Unterstützung

Claude zeichnet sich bei Aufgaben in mehreren Sprachen aus und behält eine starke sprachübergreifende Leistung im Vergleich zu Englisch bei.

---

## Übersicht

Claude demonstriert robuste mehrsprachige Fähigkeiten mit besonders starker Leistung bei Zero-Shot-Aufgaben über Sprachen hinweg. Das Modell behält eine konsistente relative Leistung über weit verbreitete und ressourcenärmere Sprachen hinweg bei, was es zu einer zuverlässigen Wahl für mehrsprachige Anwendungen macht.

Beachten Sie, dass Claude in vielen Sprachen über die unten benchmarkten hinaus fähig ist. Wir ermutigen Sie, mit allen Sprachen zu testen, die für Ihre spezifischen Anwendungsfälle relevant sind.

## Leistungsdaten

Nachfolgend sind die Zero-Shot-Chain-of-Thought-Bewertungsergebnisse für Claude-Modelle über verschiedene Sprachen hinweg aufgeführt, dargestellt als Prozentsatz relativ zur englischen Leistung (100%):

| Sprache | Claude Opus 4.1<sup>1</sup> | Claude Opus 4<sup>1</sup> | Claude Sonnet 4.5<sup>1</sup> | Claude Sonnet 4<sup>1</sup> | Claude Haiku 4.5<sup>1</sup> |
|----------|---------------|---------------|---------------|-----------------|------------------|
| Englisch (Baseline, auf 100% festgelegt) | 100% | 100% | 100% | 100% | 100% |
| Spanisch | 98.1% | 98.0% | 98.2% | 97.5% | 96.4% |
| Portugiesisch (Brasilien) | 97.8% | 97.3% | 97.8% | 97.2% | 96.1% |
| Italienisch | 97.7% | 97.5% | 97.9% | 97.3% | 96.0% |
| Französisch | 97.9% | 97.7% | 97.5% | 97.1% | 95.7% |
| Indonesisch | 97.3% | 97.2% | 97.3% | 96.2% | 94.2% |
| Deutsch | 97.7% | 97.1% | 97.0% | 94.7% | 94.3% |
| Arabisch | 97.1% | 96.9% | 97.2% | 96.1% | 92.5% |
| Chinesisch (Vereinfacht) | 97.1% | 96.7% | 96.9% | 95.9% | 94.2% |
| Koreanisch | 96.6% | 96.4% | 96.7% | 95.9% | 93.3% |
| Japanisch | 96.9% | 96.2% | 96.8% | 95.6% | 93.5% |
| Hindi | 96.8% | 96.7% | 96.7% | 95.8% | 92.4% |
| Bengalisch | 95.7% | 95.2% | 95.4% | 94.4% | 90.4% |
| Suaheli | 89.8% | 89.5% | 91.1% | 87.1% | 78.3% |
| Yoruba | 80.3% | 78.9% | 79.7% | 76.4% | 52.7% |

<sup>1</sup> Mit [erweitertem Denken](/docs/de/build-with-claude/extended-thinking).

<Note>
Diese Metriken basieren auf [MMLU (Massive Multitask Language Understanding)](https://en.wikipedia.org/wiki/MMLU) englischen Testsätzen, die von professionellen menschlichen Übersetzern in 14 zusätzliche Sprachen übersetzt wurden, wie in [OpenAIs simple-evals Repository](https://github.com/openai/simple-evals/blob/main/multilingual_mmlu_benchmark_results.md) dokumentiert. Die Verwendung von menschlichen Übersetzern für diese Bewertung gewährleistet hochwertige Übersetzungen, besonders wichtig für Sprachen mit weniger digitalen Ressourcen.
</Note>

***

## Best Practices

Bei der Arbeit mit mehrsprachigen Inhalten:

1. **Geben Sie einen klaren Sprachkontext an**: Während Claude die Zielsprache automatisch erkennen kann, verbessert die explizite Angabe der gewünschten Ein-/Ausgabesprache die Zuverlässigkeit. Für verbesserte Flüssigkeit können Sie Claude auffordern, „idiomatische Sprache zu verwenden, als wäre es ein Muttersprachler."
2. **Verwenden Sie native Schriften**: Reichen Sie Text in seiner nativen Schrift ein, anstatt Transliteration für optimale Ergebnisse zu verwenden
3. **Berücksichtigen Sie den kulturellen Kontext**: Effektive Kommunikation erfordert oft kulturelles und regionales Bewusstsein über reine Übersetzung hinaus

Wir empfehlen auch, unsere allgemeinen [Richtlinien zur Prompt-Entwicklung](/docs/de/build-with-claude/prompt-engineering/overview) zu befolgen, um Claudes Leistung besser zu verbessern.

***

## Überlegungen zur Sprachunterstützung

- Claude verarbeitet Eingaben und generiert Ausgaben in den meisten Weltsprachen, die Standard-Unicode-Zeichen verwenden
- Die Leistung variiert je nach Sprache, mit besonders starken Fähigkeiten in weit verbreiteten Sprachen
- Auch in Sprachen mit weniger digitalen Ressourcen behält Claude aussagekräftige Fähigkeiten bei

<CardGroup cols={2}>
  <Card title="Leitfaden zur Prompt-Entwicklung" icon="edit" href="/docs/de/build-with-claude/prompt-engineering/overview">
    Beherrschen Sie die Kunst der Prompt-Erstellung, um das Beste aus Claude herauszuholen.
  </Card>
  <Card title="Prompt-Bibliothek" icon="books" href="/docs/de/resources/prompt-library">
    Finden Sie eine breite Palette vorgefertigter Prompts für verschiedene Aufgaben und Branchen. Perfekt für Inspiration oder schnelle Starts.
  </Card>
</CardGroup>