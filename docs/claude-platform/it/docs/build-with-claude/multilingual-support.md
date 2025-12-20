# Supporto multilingue

Claude eccelle in attività in più lingue, mantenendo forti prestazioni cross-linguistiche rispetto all'inglese.

---

## Panoramica

Claude dimostra solide capacità multilingui, con prestazioni particolarmente forti in attività zero-shot in più lingue. Il modello mantiene prestazioni relative coerenti sia nelle lingue ampiamente parlate che in quelle con meno risorse, rendendolo una scelta affidabile per applicazioni multilingui.

Nota che Claude è capace in molte lingue oltre a quelle valutate di seguito. Ti incoraggiamo a testare con qualsiasi lingua rilevante per i tuoi casi d'uso specifici.

## Dati di prestazione

Di seguito sono riportati i punteggi di valutazione zero-shot chain-of-thought per i modelli Claude in diverse lingue, mostrati come percentuale relativa alle prestazioni in inglese (100%):

| Lingua | Claude Opus 4.1<sup>1</sup> | Claude Opus 4<sup>1</sup> | Claude Sonnet 4.5<sup>1</sup> | Claude Sonnet 4<sup>1</sup> | Claude Haiku 4.5<sup>1</sup> |
|----------|---------------|---------------|---------------|-----------------|------------------|
| Inglese (baseline, fissato al 100%) | 100% | 100% | 100% | 100% | 100% |
| Spagnolo | 98.1% | 98.0% | 98.2% | 97.5% | 96.4% |
| Portoghese (Brasile) | 97.8% | 97.3% | 97.8% | 97.2% | 96.1% |
| Italiano | 97.7% | 97.5% | 97.9% | 97.3% | 96.0% |
| Francese | 97.9% | 97.7% | 97.5% | 97.1% | 95.7% |
| Indonesiano | 97.3% | 97.2% | 97.3% | 96.2% | 94.2% |
| Tedesco | 97.7% | 97.1% | 97.0% | 94.7% | 94.3% |
| Arabo | 97.1% | 96.9% | 97.2% | 96.1% | 92.5% |
| Cinese (Semplificato) | 97.1% | 96.7% | 96.9% | 95.9% | 94.2% |
| Coreano | 96.6% | 96.4% | 96.7% | 95.9% | 93.3% |
| Giapponese | 96.9% | 96.2% | 96.8% | 95.6% | 93.5% |
| Hindi | 96.8% | 96.7% | 96.7% | 95.8% | 92.4% |
| Bengalese | 95.7% | 95.2% | 95.4% | 94.4% | 90.4% |
| Swahili | 89.8% | 89.5% | 91.1% | 87.1% | 78.3% |
| Yoruba | 80.3% | 78.9% | 79.7% | 76.4% | 52.7% |

<sup>1</sup> Con [extended thinking](/docs/it/build-with-claude/extended-thinking).

<Note>
Queste metriche si basano su set di test MMLU (Massive Multitask Language Understanding) in inglese che sono stati tradotti in 14 lingue aggiuntive da traduttori umani professionisti, come documentato nel [repository simple-evals di OpenAI](https://github.com/openai/simple-evals/blob/main/multilingual_mmlu_benchmark_results.md). L'uso di traduttori umani per questa valutazione garantisce traduzioni di alta qualità, particolarmente importante per le lingue con meno risorse digitali.
</Note>

***

## Migliori pratiche

Quando lavori con contenuti multilingui:

1. **Fornisci un chiaro contesto linguistico**: Sebbene Claude possa rilevare automaticamente la lingua di destinazione, dichiarare esplicitamente la lingua di input/output desiderata migliora l'affidabilità. Per una fluidità migliorata, puoi chiedere a Claude di utilizzare "un linguaggio idiomatico come se fosse un madrelingua."
2. **Usa script nativi**: Invia il testo nel suo script nativo piuttosto che in traslitterazione per risultati ottimali
3. **Considera il contesto culturale**: La comunicazione efficace spesso richiede consapevolezza culturale e regionale oltre la pura traduzione

Ti suggeriamo anche di seguire le nostre [linee guida generali di prompt engineering](/docs/it/build-with-claude/prompt-engineering/overview) per migliorare ulteriormente le prestazioni di Claude.

***

## Considerazioni sul supporto linguistico

- Claude elabora input e genera output nella maggior parte delle lingue mondiali che utilizzano caratteri Unicode standard
- Le prestazioni variano a seconda della lingua, con capacità particolarmente forti nelle lingue ampiamente parlate
- Anche nelle lingue con meno risorse digitali, Claude mantiene capacità significative

<CardGroup cols={2}>
  <Card title="Guida al Prompt Engineering" icon="edit" href="/docs/it/build-with-claude/prompt-engineering/overview">
    Padroneggia l'arte della creazione di prompt per ottenere il massimo da Claude.
  </Card>
  <Card title="Libreria di Prompt" icon="books" href="/docs/it/resources/prompt-library">
    Trova un'ampia gamma di prompt pre-creati per varie attività e settori. Perfetto per l'ispirazione o per iniziare rapidamente.
  </Card>
</CardGroup>