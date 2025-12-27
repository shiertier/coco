# Livelli di servizio

Diversi livelli di servizio ti permettono di bilanciare disponibilità, prestazioni e costi prevedibili in base alle esigenze della tua applicazione.

---

Offriamo tre livelli di servizio:
- **Priority Tier:** Ideale per i flussi di lavoro distribuiti in produzione dove il tempo, la disponibilità e i prezzi prevedibili sono importanti
- **Standard:** Livello predefinito sia per i progetti pilota che per il ridimensionamento dei casi d'uso quotidiani
- **Batch:** Ideale per i flussi di lavoro asincroni che possono attendere o beneficiare di essere al di fuori della tua capacità normale

## Standard Tier

Il livello standard è il livello di servizio predefinito per tutte le richieste API. Le richieste in questo livello sono prioritizzate insieme a tutte le altre richieste e osservano la disponibilità best-effort.

## Priority Tier

Le richieste in questo livello sono prioritizzate rispetto a tutte le altre richieste ad Anthropic. Questa prioritizzazione aiuta a minimizzare gli errori ["server overloaded"](/docs/it/api/errors#http-errors), anche durante i periodi di picco.

Per ulteriori informazioni, consulta [Inizia con Priority Tier](#get-started-with-priority-tier)

## Come le richieste vengono assegnate ai livelli

Quando gestisce una richiesta, Anthropic decide di assegnare una richiesta a Priority Tier nei seguenti scenari:
- La tua organizzazione ha una capacità Priority Tier sufficiente di token **input** al minuto
- La tua organizzazione ha una capacità Priority Tier sufficiente di token **output** al minuto

Anthropic conta l'utilizzo rispetto alla capacità Priority Tier come segue:

**Token di input**
- Le letture della cache come 0,1 token per token letto dalla cache
- Le scritture della cache come 1,25 token per token scritto nella cache con TTL di 5 minuti
- Le scritture della cache come 2,00 token per token scritto nella cache con TTL di 1 ora
- Per le richieste [long-context](/docs/it/build-with-claude/context-windows) (>200k token di input), i token di input sono 2 token per token
- Tutti gli altri token di input sono 1 token per token

**Token di output**
- Per le richieste [long-context](/docs/it/build-with-claude/context-windows) (>200k token di input), i token di output sono 1,5 token per token
- Tutti gli altri token di output sono 1 token per token

Altrimenti, le richieste procedono al livello standard.

<Note>
Le richieste assegnate a Priority Tier attingono sia dalla capacità Priority Tier che dai limiti di velocità regolari.
Se la gestione della richiesta superasse i limiti di velocità, la richiesta viene rifiutata.
</Note>

## Utilizzo dei livelli di servizio

Puoi controllare quali livelli di servizio possono essere utilizzati per una richiesta impostando il parametro `service_tier`:

```python
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello, Claude!"}],
    service_tier="auto"  # Automatically use Priority Tier when available, fallback to standard
)
```

Il parametro `service_tier` accetta i seguenti valori:

- `"auto"` (predefinito) - Utilizza la capacità Priority Tier se disponibile, ricadendo sulla tua altra capacità se non disponibile
- `"standard_only"` - Utilizza solo la capacità del livello standard, utile se non desideri utilizzare la tua capacità Priority Tier

L'oggetto `usage` della risposta include anche il livello di servizio assegnato alla richiesta:

```json
{
  "usage": {
    "input_tokens": 410,
    "cache_creation_input_tokens": 0,
    "cache_read_input_tokens": 0,
    "output_tokens": 585,
    "service_tier": "priority"
  }
}
```
Questo ti consente di determinare quale livello di servizio è stato assegnato alla richiesta.

Quando richiedi `service_tier="auto"` con un modello con un impegno Priority Tier, questi header di risposta forniscono informazioni:
```
anthropic-priority-input-tokens-limit: 10000
anthropic-priority-input-tokens-remaining: 9618
anthropic-priority-input-tokens-reset: 2025-01-12T23:11:59Z
anthropic-priority-output-tokens-limit: 10000
anthropic-priority-output-tokens-remaining: 6000
anthropic-priority-output-tokens-reset: 2025-01-12T23:12:21Z
```
Puoi utilizzare la presenza di questi header per rilevare se la tua richiesta era idonea per Priority Tier, anche se era oltre il limite.

## Inizia con Priority Tier

Potresti voler impegnarti nella capacità Priority Tier se sei interessato a:
- **Maggiore disponibilità**: Obiettivo di uptime del 99,5% con risorse computazionali prioritizzate
- **Controllo dei costi**: Spesa prevedibile e sconti per impegni più lunghi
- **Overflow flessibile**: Ricade automaticamente al livello standard quando superi la tua capacità impegnata

L'impegno a Priority Tier comporterà la decisione di:
- Un numero di token di input al minuto
- Un numero di token di output al minuto
- Una durata dell'impegno (1, 3, 6 o 12 mesi)
- Una versione di modello specifica

<Note>
Il rapporto tra i token di input e output che acquisti è importante. Dimensionare la tua capacità Priority Tier per allinearsi ai tuoi modelli di traffico effettivi ti aiuta a massimizzare l'utilizzo dei tuoi token acquistati.
</Note>

### Modelli supportati

Priority Tier è supportato da:

- Claude Opus 4.5
- Claude Sonnet 4.5
- Claude Haiku 4.5
- Claude Opus 4.1
- Claude Opus 4
- Claude Sonnet 4
- Claude Sonnet 3.7 ([deprecato](/docs/it/about-claude/model-deprecations))
- Claude Haiku 3.5 ([deprecato](/docs/it/about-claude/model-deprecations))

Consulta la [pagina di panoramica dei modelli](/docs/it/about-claude/models/overview) per ulteriori dettagli sui nostri modelli.

### Come accedere a Priority Tier

Per iniziare a utilizzare Priority Tier:

1. [Contatta il team di vendita](https://claude.com/contact-sales/priority-tier) per completare il provisioning
2. (Facoltativo) Aggiorna le tue richieste API per impostare facoltativamente il parametro `service_tier` su `auto`
3. Monitora il tuo utilizzo attraverso gli header di risposta e la Claude Console