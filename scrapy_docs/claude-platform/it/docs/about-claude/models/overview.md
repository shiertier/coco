# Panoramica dei modelli

Claude è una famiglia di modelli linguistici di grandi dimensioni all'avanguardia sviluppati da Anthropic. Questa guida introduce i nostri modelli e confronta le loro prestazioni.

---

## Scelta di un modello

Se non sei sicuro di quale modello utilizzare, ti consigliamo di iniziare con **Claude Sonnet 4.5**. Offre il miglior equilibrio tra intelligenza, velocità e costo per la maggior parte dei casi d'uso, con prestazioni eccezionali nei compiti di codifica e agenti.

Tutti i modelli Claude attuali supportano input di testo e immagini, output di testo, capacità multilingue e visione. I modelli sono disponibili tramite l'API Anthropic, AWS Bedrock e Google Vertex AI.

Una volta scelto un modello, [scopri come effettuare la tua prima chiamata API](/docs/it/get-started).

### Confronto dei modelli più recenti

| Feature | Claude Sonnet 4.5 | Claude Haiku 4.5 | Claude Opus 4.5 |
|:--------|:------------------|:-----------------|:----------------|
| **Description** | Il nostro modello intelligente per agenti complessi e codifica | Il nostro modello più veloce con intelligenza quasi all'avanguardia | Modello premium che combina l'intelligenza massima con prestazioni pratiche |
| **Claude API ID** | claude-sonnet-4-5-20250929 | claude-haiku-4-5-20251001 | claude-opus-4-5-20251101 |
| **Claude API alias**<sup>1</sup> | claude-sonnet-4-5 | claude-haiku-4-5 | claude-opus-4-5 |
| **AWS Bedrock ID** | anthropic.claude-sonnet-4-5-20250929-v1:0 | anthropic.claude-haiku-4-5-20251001-v1:0 | anthropic.claude-opus-4-5-20251101-v1:0 |
| **GCP Vertex AI ID** | claude-sonnet-4-5@20250929 | claude-haiku-4-5@20251001 | claude-opus-4-5@20251101 |
| **Pricing**<sup>2</sup> | \$3 / input MTok<br/>\$15 / output MTok | \$1 / input MTok<br/>\$5 / output MTok | \$5 / input MTok<br/>\$25 / output MTok |
| **[Extended thinking](/docs/it/build-with-claude/extended-thinking)** | Sì | Sì | Sì |
| **[Priority Tier](/docs/it/api/service-tiers)** | Sì | Sì | Sì |
| **Comparative latency** | Veloce | Più veloce | Moderato |
| **Context window** | <Tooltip tooltipContent="~150K words \ ~680K unicode characters">200K tokens</Tooltip> / <br/> <Tooltip tooltipContent="~750K words \ ~3.4M unicode characters">1M tokens</Tooltip> (beta)<sup>3</sup> | <Tooltip tooltipContent="~150K words \ ~680K unicode characters">200K tokens</Tooltip> | <Tooltip tooltipContent="~150K words \ ~680K unicode characters">200K tokens</Tooltip> |
| **Max output** | 64K tokens | 64K tokens | 64K tokens |
| **Reliable knowledge cutoff** | Gen 2025<sup>4</sup> | Feb 2025 | Mag 2025<sup>4</sup> |
| **Training data cutoff** | Lug 2025 | Lug 2025 | Ago 2025 |

_<sup>1 - Gli alias puntano automaticamente all'istantanea del modello più recente. Quando rilasciamo nuove istantanee del modello, migriamo gli alias per puntare alla versione più recente di un modello, in genere entro una settimana dal nuovo rilascio. Sebbene gli alias siano utili per la sperimentazione, consigliamo di utilizzare versioni specifiche del modello (ad es., `claude-sonnet-4-5-20250929`) nelle applicazioni di produzione per garantire un comportamento coerente.</sup>_

_<sup>2 - Consulta la nostra [pagina dei prezzi](/docs/it/about-claude/pricing) per informazioni complete sui prezzi, inclusi sconti API batch, tariffe di caching dei prompt, costi di extended thinking e tariffe di elaborazione della visione.</sup>_

_<sup>3 - Claude Sonnet 4.5 supporta una [finestra di contesto di 1M token](/docs/it/build-with-claude/context-windows#1m-token-context-window) quando si utilizza l'intestazione beta `context-1m-2025-08-07`. [I prezzi del contesto lungo](/docs/it/about-claude/pricing#long-context-pricing) si applicano alle richieste che superano 200K token.</sup>_

_<sup>4 - **Reliable knowledge cutoff** indica la data fino a cui la conoscenza di un modello è più estesa e affidabile. **Training data cutoff** è l'intervallo di date più ampio dei dati di addestramento utilizzati. Ad esempio, Claude Sonnet 4.5 è stato addestrato su informazioni pubblicamente disponibili fino a luglio 2025, ma la sua conoscenza è più estesa e affidabile fino a gennaio 2025. Per ulteriori informazioni, consulta [Anthropic's Transparency Hub](https://www.anthropic.com/transparency).</sup>_

<Note>I modelli con la stessa data di istantanea (ad es., 20240620) sono identici su tutte le piattaforme e non cambiano. La data di istantanea nel nome del modello garantisce coerenza e consente agli sviluppatori di fare affidamento su prestazioni stabili in diversi ambienti.</Note>

<Note>A partire da **Claude Sonnet 4.5 e tutti i modelli futuri**, AWS Bedrock e Google Vertex AI offrono due tipi di endpoint: **endpoint globali** (routing dinamico per la massima disponibilità) e **endpoint regionali** (routing dati garantito attraverso regioni geografiche specifiche). Per ulteriori informazioni, consulta la [sezione dei prezzi della piattaforma di terze parti](/docs/it/about-claude/pricing#third-party-platform-pricing).</Note>

<section title="Modelli legacy">

I seguenti modelli sono ancora disponibili ma consigliamo di migrare ai modelli attuali per prestazioni migliori:

| Feature | Claude Opus 4.1 | Claude Sonnet 4 | Claude Sonnet 3.7 | Claude Opus 4 | Claude Haiku 3 |
|:--------|:----------------|:----------------|:------------------|:--------------|:---------------|
| **Claude API ID** | claude-opus-4-1-20250805 | claude-sonnet-4-20250514 | claude-3-7-sonnet-20250219 | claude-opus-4-20250514 | claude-3-haiku-20240307 |
| **Claude API alias** | claude-opus-4-1 | claude-sonnet-4-0 | claude-3-7-sonnet-latest | claude-opus-4-0 | — |
| **AWS Bedrock ID** | anthropic.claude-opus-4-1-20250805-v1:0 | anthropic.claude-sonnet-4-20250514-v1:0 | anthropic.claude-3-7-sonnet-20250219-v1:0 | anthropic.claude-opus-4-20250514-v1:0 | anthropic.claude-3-haiku-20240307-v1:0 |
| **GCP Vertex AI ID** | claude-opus-4-1@20250805 | claude-sonnet-4@20250514 | claude-3-7-sonnet@20250219 | claude-opus-4@20250514 | claude-3-haiku@20240307 |
| **Pricing** | \$15 / input MTok<br/>\$75 / output MTok | \$3 / input MTok<br/>\$15 / output MTok | \$3 / input MTok<br/>\$15 / output MTok | \$15 / input MTok<br/>\$75 / output MTok | \$0.25 / input MTok<br/>\$1.25 / output MTok |
| **[Extended thinking](/docs/it/build-with-claude/extended-thinking)** | Sì | Sì | Sì | Sì | No |
| **[Priority Tier](/docs/it/api/service-tiers)** | Sì | Sì | Sì | Sì | No |
| **Comparative latency** | Moderato | Veloce | Veloce | Moderato | Veloce |
| **Context window** | <Tooltip tooltipContent="~150K words \ ~680K unicode characters">200K tokens</Tooltip> | <Tooltip tooltipContent="~150K words \ ~680K unicode characters">200K tokens</Tooltip> / <br/> <Tooltip tooltipContent="~750K words \ ~3.4M unicode characters">1M tokens</Tooltip> (beta)<sup>1</sup> | <Tooltip tooltipContent="~150K words \ ~680K unicode characters">200K tokens</Tooltip> | <Tooltip tooltipContent="~150K words \ ~680K unicode characters">200K tokens</Tooltip> | <Tooltip tooltipContent="~150K words \ ~680K unicode characters">200K tokens</Tooltip> |
| **Max output** | 32K tokens | 64K tokens | 64K tokens / 128K tokens (beta)<sup>4</sup> | 32K tokens | 4K tokens |
| **Reliable knowledge cutoff** | Gen 2025<sup>2</sup> | Gen 2025<sup>2</sup> | Ott 2024<sup>2</sup> | Gen 2025<sup>2</sup> | <sup>3</sup> |
| **Training data cutoff** | Mar 2025 | Mar 2025 | Nov 2024 | Mar 2025 | Ago 2023 |

_<sup>1 - Claude Sonnet 4 supporta una [finestra di contesto di 1M token](/docs/it/build-with-claude/context-windows#1m-token-context-window) quando si utilizza l'intestazione beta `context-1m-2025-08-07`. [I prezzi del contesto lungo](/docs/it/about-claude/pricing#long-context-pricing) si applicano alle richieste che superano 200K token.</sup>_

_<sup>2 - **Reliable knowledge cutoff** indica la data fino a cui la conoscenza di un modello è più estesa e affidabile. **Training data cutoff** è l'intervallo di date più ampio dei dati di addestramento utilizzati.</sup>_

_<sup>3 - Alcuni modelli Haiku hanno una singola data di cutoff dei dati di addestramento.</sup>_

_<sup>4 - Includi l'intestazione beta `output-128k-2025-02-19` nella tua richiesta API per aumentare la lunghezza massima del token di output a 128K token per Claude Sonnet 3.7. Consigliamo vivamente di utilizzare la nostra [API Messages in streaming](/docs/it/build-with-claude/streaming) per evitare timeout durante la generazione di output più lunghi. Consulta la nostra guida su [richieste lunghe](/docs/it/api/errors#long-requests) per ulteriori dettagli.</sup>_

</section>

## Prestazioni dei prompt e dell'output

I modelli Claude 4 eccellono in:
- **Prestazioni**: Risultati di livello superiore nel ragionamento, nella codifica, nei compiti multilingue, nella gestione del contesto lungo, nell'onestà e nell'elaborazione delle immagini. Consulta il [post del blog Claude 4](http://www.anthropic.com/news/claude-4) per ulteriori informazioni.
- **Risposte coinvolgenti**: I modelli Claude sono ideali per applicazioni che richiedono interazioni ricche e simili a quelle umane.

    - Se preferisci risposte più concise, puoi regolare i tuoi prompt per guidare il modello verso la lunghezza di output desiderata. Consulta le nostre [guide di ingegneria dei prompt](/docs/it/build-with-claude/prompt-engineering) per i dettagli.
    - Per le migliori pratiche specifiche di prompt di Claude 4, consulta la nostra [guida alle migliori pratiche di Claude 4](/docs/it/build-with-claude/prompt-engineering/claude-4-best-practices).
- **Qualità dell'output**: Quando si esegue la migrazione dalle generazioni di modelli precedenti a Claude 4, potresti notare miglioramenti più significativi nelle prestazioni complessive.

## Migrazione a Claude 4.5

Se stai attualmente utilizzando modelli Claude 3, consigliamo di migrare a Claude 4.5 per sfruttare l'intelligenza migliorata e le capacità potenziate. Per istruzioni dettagliate sulla migrazione, consulta [Migrazione a Claude 4.5](/docs/it/about-claude/models/migrating-to-claude-4).

## Inizia con Claude

Se sei pronto a iniziare a esplorare cosa Claude può fare per te, tuffiamoci! Che tu sia uno sviluppatore che cerca di integrare Claude nelle tue applicazioni o un utente che vuole sperimentare la potenza dell'IA in prima persona, siamo qui per te.

<Note>Vuoi chattare con Claude? Visita [claude.ai](http://www.claude.ai)!</Note>

<CardGroup cols={3}>
  <Card title="Introduzione a Claude" icon="check" href="/docs/it/intro">
    Esplora le capacità di Claude e il flusso di sviluppo.
  </Card>
  <Card title="Quickstart" icon="lightning" href="/docs/it/get-started">
    Scopri come effettuare la tua prima chiamata API in pochi minuti.
  </Card>
  <Card title="Claude Console" icon="code" href="/">
    Crea e testa prompt potenti direttamente nel tuo browser.
  </Card>
</CardGroup>

Se hai domande o hai bisogno di assistenza, non esitare a contattare il nostro [team di supporto](https://support.claude.com/) o consulta la [comunità Discord](https://www.anthropic.com/discord).