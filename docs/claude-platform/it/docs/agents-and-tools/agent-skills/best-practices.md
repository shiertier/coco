# Migliori pratiche per la creazione di Skills

Scopri come scrivere Skills efficaci che Claude può scoprire e utilizzare con successo.

---

Le buone Skills sono concise, ben strutturate e testate con un utilizzo reale. Questa guida fornisce decisioni pratiche di authoring per aiutarti a scrivere Skills che Claude può scoprire e utilizzare efficacemente.

Per informazioni concettuali su come funzionano le Skills, consulta la [panoramica delle Skills](/docs/it/agents-and-tools/agent-skills/overview).

## Principi fondamentali

### La concisione è fondamentale

La [finestra di contesto](/docs/it/build-with-claude/context-windows) è un bene pubblico. La tua Skill condivide la finestra di contesto con tutto il resto che Claude deve sapere, incluso:
- Il prompt di sistema
- La cronologia della conversazione
- I metadati di altre Skills
- La tua richiesta effettiva

Non ogni token nella tua Skill ha un costo immediato. All'avvio, solo i metadati (nome e descrizione) da tutte le Skills vengono precaricati. Claude legge SKILL.md solo quando la Skill diventa rilevante e legge file aggiuntivi solo se necessario. Tuttavia, essere concisi in SKILL.md è ancora importante: una volta che Claude lo carica, ogni token compete con la cronologia della conversazione e altri contesti.

**Assunzione predefinita**: Claude è già molto intelligente

Aggiungi solo il contesto che Claude non ha già. Metti in discussione ogni informazione:
- "Claude ha davvero bisogno di questa spiegazione?"
- "Posso assumere che Claude lo sappia?"
- "Questo paragrafo giustifica il suo costo in token?"

**Buon esempio: Conciso** (circa 50 token):
````markdown
## Estrai testo da PDF

Usa pdfplumber per l'estrazione del testo:

```python
import pdfplumber

with pdfplumber.open("file.pdf") as pdf:
    text = pdf.pages[0].extract_text()
```
````

**Cattivo esempio: Troppo verboso** (circa 150 token):
```markdown
## Estrai testo da PDF

PDF (Portable Document Format) sono un formato di file comune che contiene
testo, immagini e altri contenuti. Per estrarre testo da un PDF, dovrai
utilizzare una libreria. Ci sono molte librerie disponibili per l'elaborazione
dei PDF, ma consigliamo pdfplumber perché è facile da usare e gestisce bene
la maggior parte dei casi. Per prima cosa, dovrai installarla usando pip.
Quindi puoi usare il codice sottostante...
```

La versione concisa assume che Claude sappia cosa sono i PDF e come funzionano le librerie.

### Imposta gradi di libertà appropriati

Abbina il livello di specificità alla fragilità e variabilità del compito.

**Alta libertà** (istruzioni basate su testo):

Usa quando:
- Sono validi più approcci
- Le decisioni dipendono dal contesto
- L'euristica guida l'approccio

Esempio:
```markdown
## Processo di revisione del codice

1. Analizza la struttura e l'organizzazione del codice
2. Controlla i potenziali bug o casi limite
3. Suggerisci miglioramenti per la leggibilità e la manutenibilità
4. Verifica l'aderenza alle convenzioni del progetto
```

**Libertà media** (pseudocodice o script con parametri):

Usa quando:
- Esiste un modello preferito
- Alcune variazioni sono accettabili
- La configurazione influisce sul comportamento

Esempio:
````markdown
## Genera report

Usa questo modello e personalizza secondo le necessità:

```python
def generate_report(data, format="markdown", include_charts=True):
    # Elabora i dati
    # Genera output nel formato specificato
    # Opzionalmente includi visualizzazioni
```
````

**Bassa libertà** (script specifici, pochi o nessun parametro):

Usa quando:
- Le operazioni sono fragili e soggette a errori
- La coerenza è critica
- Una sequenza specifica deve essere seguita

Esempio:
````markdown
## Migrazione del database

Esegui esattamente questo script:

```bash
python scripts/migrate.py --verify --backup
```

Non modificare il comando o aggiungere flag aggiuntivi.
````

**Analogia**: Pensa a Claude come a un robot che esplora un percorso:
- **Ponte stretto con scogliere su entrambi i lati**: C'è solo un modo sicuro per andare avanti. Fornisci guardrail specifici e istruzioni esatte (bassa libertà). Esempio: migrazioni di database che devono essere eseguite in sequenza esatta.
- **Campo aperto senza pericoli**: Molti percorsi portano al successo. Dai una direzione generale e fidati che Claude troverà il percorso migliore (alta libertà). Esempio: revisioni del codice dove il contesto determina l'approccio migliore.

### Testa con tutti i modelli che intendi utilizzare

Le Skills agiscono come aggiunte ai modelli, quindi l'efficacia dipende dal modello sottostante. Testa la tua Skill con tutti i modelli che intendi utilizzare.

**Considerazioni di test per modello**:
- **Claude Haiku** (veloce, economico): La Skill fornisce abbastanza guida?
- **Claude Sonnet** (equilibrato): La Skill è chiara ed efficiente?
- **Claude Opus** (ragionamento potente): La Skill evita di spiegare troppo?

Quello che funziona perfettamente per Opus potrebbe aver bisogno di più dettagli per Haiku. Se intendi utilizzare la tua Skill su più modelli, punta a istruzioni che funzionino bene con tutti loro.

## Struttura della Skill

<Note>
**Frontmatter YAML**: Il frontmatter di SKILL.md richiede due campi:

`name`:
- Massimo 64 caratteri
- Deve contenere solo lettere minuscole, numeri e trattini
- Non può contenere tag XML
- Non può contenere parole riservate: "anthropic", "claude"

`description`:
- Deve essere non vuoto
- Massimo 1024 caratteri
- Non può contenere tag XML
- Dovrebbe descrivere cosa fa la Skill e quando usarla

Per i dettagli completi della struttura della Skill, consulta la [panoramica delle Skills](/docs/it/agents-and-tools/agent-skills/overview#skill-structure).
</Note>

### Convenzioni di denominazione

Usa modelli di denominazione coerenti per rendere le Skills più facili da referenziare e discutere. Consigliamo di usare la **forma gerundiale** (verbo + -ing) per i nomi delle Skills, poiché descrive chiaramente l'attività o la capacità che la Skill fornisce.

Ricorda che il campo `name` deve usare solo lettere minuscole, numeri e trattini.

**Buoni esempi di denominazione (forma gerundiale)**:
- `processing-pdfs`
- `analyzing-spreadsheets`
- `managing-databases`
- `testing-code`
- `writing-documentation`

**Alternative accettabili**:
- Frasi nominali: `pdf-processing`, `spreadsheet-analysis`
- Orientate all'azione: `process-pdfs`, `analyze-spreadsheets`

**Evita**:
- Nomi vaghi: `helper`, `utils`, `tools`
- Troppo generici: `documents`, `data`, `files`
- Parole riservate: `anthropic-helper`, `claude-tools`
- Modelli incoerenti all'interno della tua raccolta di skills

La denominazione coerente rende più facile:
- Referenziare le Skills nella documentazione e nelle conversazioni
- Capire cosa fa una Skill a colpo d'occhio
- Organizzare e cercare tra più Skills
- Mantenere una libreria di skills professionale e coesa

### Scrivere descrizioni efficaci

Il campo `description` abilita la scoperta della Skill e dovrebbe includere sia cosa fa la Skill che quando usarla.

<Warning>
**Scrivi sempre in terza persona**. La descrizione viene iniettata nel prompt di sistema e un punto di vista incoerente può causare problemi di scoperta.

- **Buono:** "Elabora file Excel e genera report"
- **Evita:** "Posso aiutarti a elaborare file Excel"
- **Evita:** "Puoi usare questo per elaborare file Excel"
</Warning>

**Sii specifico e includi termini chiave**. Includi sia cosa fa la Skill che trigger/contesti specifici per quando usarla.

Ogni Skill ha esattamente un campo di descrizione. La descrizione è critica per la selezione della skill: Claude la usa per scegliere la Skill giusta da potenzialmente 100+ Skills disponibili. La tua descrizione deve fornire abbastanza dettagli affinché Claude sappia quando selezionare questa Skill, mentre il resto di SKILL.md fornisce i dettagli di implementazione.

Esempi efficaci:

**Skill di elaborazione PDF:**
```yaml
description: Estrai testo e tabelle da file PDF, compila moduli, unisci documenti. Usa quando lavori con file PDF o quando l'utente menziona PDF, moduli o estrazione di documenti.
```

**Skill di analisi Excel:**
```yaml
description: Analizza fogli di calcolo Excel, crea tabelle pivot, genera grafici. Usa quando analizzi file Excel, fogli di calcolo, dati tabulari o file .xlsx.
```

**Skill di aiuto per commit Git:**
```yaml
description: Genera messaggi di commit descrittivi analizzando i diff di git. Usa quando l'utente chiede aiuto per scrivere messaggi di commit o per rivedere le modifiche in staging.
```

Evita descrizioni vaghe come queste:

```yaml
description: Aiuta con i documenti
```
```yaml
description: Elabora i dati
```
```yaml
description: Fa cose con i file
```

### Modelli di divulgazione progressiva

SKILL.md serve come panoramica che indirizza Claude a materiali dettagliati secondo le necessità, come un indice in una guida di onboarding. Per una spiegazione di come funziona la divulgazione progressiva, consulta [Come funzionano le Skills](/docs/it/agents-and-tools/agent-skills/overview#how-skills-work) nella panoramica.

**Guida pratica:**
- Mantieni il corpo di SKILL.md sotto 500 righe per prestazioni ottimali
- Dividi il contenuto in file separati quando ti avvicini a questo limite
- Usa i modelli sottostanti per organizzare istruzioni, codice e risorse in modo efficace

#### Panoramica visiva: Da semplice a complesso

Una Skill di base inizia con solo un file SKILL.md contenente metadati e istruzioni:

![File SKILL.md semplice che mostra frontmatter YAML e corpo markdown](/docs/images/agent-skills-simple-file.png)

Man mano che la tua Skill cresce, puoi raggruppare contenuti aggiuntivi che Claude carica solo quando necessario:

![Raggruppamento di file di riferimento aggiuntivi come reference.md e forms.md.](/docs/images/agent-skills-bundling-content.png)

La struttura completa della directory della Skill potrebbe assomigliare a questa:

```
pdf/
├── SKILL.md              # Istruzioni principali (caricate quando attivate)
├── FORMS.md              # Guida alla compilazione di moduli (caricata secondo le necessità)
├── reference.md          # Riferimento API (caricato secondo le necessità)
├── examples.md           # Esempi di utilizzo (caricati secondo le necessità)
└── scripts/
    ├── analyze_form.py   # Script di utilità (eseguito, non caricato)
    ├── fill_form.py      # Script di compilazione moduli
    └── validate.py       # Script di convalida
```

#### Modello 1: Guida di alto livello con riferimenti

````markdown
---
name: pdf-processing
description: Estrae testo e tabelle da file PDF, compila moduli e unisce documenti. Usa quando lavori con file PDF o quando l'utente menziona PDF, moduli o estrazione di documenti.
---

# Elaborazione PDF

## Avvio rapido

Estrai testo con pdfplumber:
```python
import pdfplumber
with pdfplumber.open("file.pdf") as pdf:
    text = pdf.pages[0].extract_text()
```

## Funzionalità avanzate

**Compilazione moduli**: Consulta [FORMS.md](FORMS.md) per la guida completa
**Riferimento API**: Consulta [REFERENCE.md](REFERENCE.md) per tutti i metodi
**Esempi**: Consulta [EXAMPLES.md](EXAMPLES.md) per i modelli comuni
````

Claude carica FORMS.md, REFERENCE.md o EXAMPLES.md solo quando necessario.

#### Modello 2: Organizzazione specifica del dominio

Per Skills con più domini, organizza il contenuto per dominio per evitare di caricare contesti irrilevanti. Quando un utente chiede informazioni sulle metriche di vendita, Claude ha bisogno solo di leggere gli schemi relativi alle vendite, non i dati finanziari o di marketing. Questo mantiene l'utilizzo dei token basso e il contesto focalizzato.

```
bigquery-skill/
├── SKILL.md (panoramica e navigazione)
└── reference/
    ├── finance.md (metriche di ricavi, fatturazione)
    ├── sales.md (opportunità, pipeline)
    ├── product.md (utilizzo API, funzionalità)
    └── marketing.md (campagne, attribuzione)
```

````markdown SKILL.md
# Analisi dati BigQuery

## Dataset disponibili

**Finanza**: Ricavi, ARR, fatturazione → Consulta [reference/finance.md](reference/finance.md)
**Vendite**: Opportunità, pipeline, account → Consulta [reference/sales.md](reference/sales.md)
**Prodotto**: Utilizzo API, funzionalità, adozione → Consulta [reference/product.md](reference/product.md)
**Marketing**: Campagne, attribuzione, email → Consulta [reference/marketing.md](reference/marketing.md)

## Ricerca rapida

Trova metriche specifiche usando grep:

```bash
grep -i "revenue" reference/finance.md
grep -i "pipeline" reference/sales.md
grep -i "api usage" reference/product.md
```
````

#### Modello 3: Dettagli condizionali

Mostra contenuto di base, collega a contenuto avanzato:

```markdown
# Elaborazione DOCX

## Creazione di documenti

Usa docx-js per nuovi documenti. Consulta [DOCX-JS.md](DOCX-JS.md).

## Modifica di documenti

Per modifiche semplici, modifica direttamente l'XML.

**Per modifiche tracciate**: Consulta [REDLINING.md](REDLINING.md)
**Per dettagli OOXML**: Consulta [OOXML.md](OOXML.md)
```

Claude legge REDLINING.md o OOXML.md solo quando l'utente ha bisogno di quelle funzionalità.

### Evita riferimenti profondamente annidati

Claude potrebbe leggere parzialmente i file quando vengono referenziati da altri file referenziati. Quando incontra riferimenti annidati, Claude potrebbe usare comandi come `head -100` per visualizzare l'anteprima del contenuto piuttosto che leggere interi file, risultando in informazioni incomplete.

**Mantieni i riferimenti a un livello di profondità da SKILL.md**. Tutti i file di riferimento dovrebbero collegarsi direttamente da SKILL.md per garantire che Claude legga file completi quando necessario.

**Cattivo esempio: Troppo profondo**:
```markdown
# SKILL.md
Consulta [advanced.md](advanced.md)...

# advanced.md
Consulta [details.md](details.md)...

# details.md
Ecco l'informazione effettiva...
```

**Buon esempio: Un livello di profondità**:
```markdown
# SKILL.md

**Utilizzo di base**: [istruzioni in SKILL.md]
**Funzionalità avanzate**: Consulta [advanced.md](advanced.md)
**Riferimento API**: Consulta [reference.md](reference.md)
**Esempi**: Consulta [examples.md](examples.md)
```

### Struttura file di riferimento più lunghi con indice dei contenuti

Per file di riferimento più lunghi di 100 righe, includi un indice dei contenuti in cima. Questo garantisce che Claude possa vedere l'ambito completo delle informazioni disponibili anche quando visualizza in anteprima con letture parziali.

**Esempio**:
```markdown
# Riferimento API

## Contenuti
- Autenticazione e configurazione
- Metodi principali (create, read, update, delete)
- Funzionalità avanzate (operazioni batch, webhook)
- Modelli di gestione degli errori
- Esempi di codice

## Autenticazione e configurazione
...

## Metodi principali
...
```

Claude può quindi leggere il file completo o saltare a sezioni specifiche secondo le necessità.

Per i dettagli su come questa architettura basata su filesystem abilita la divulgazione progressiva, consulta la sezione [Ambiente di runtime](#runtime-environment) nella sezione Avanzate sottostante.

## Flussi di lavoro e cicli di feedback

### Usa flussi di lavoro per compiti complessi

Suddividi le operazioni complesse in passaggi chiari e sequenziali. Per flussi di lavoro particolarmente complessi, fornisci una checklist che Claude può copiare nella sua risposta e spuntare man mano che procede.

**Esempio 1: Flusso di lavoro di sintesi della ricerca** (per Skills senza codice):

````markdown
## Flusso di lavoro di sintesi della ricerca

Copia questa checklist e traccia il tuo progresso:

```
Progresso della ricerca:
- [ ] Passaggio 1: Leggi tutti i documenti di origine
- [ ] Passaggio 2: Identifica i temi chiave
- [ ] Passaggio 3: Fai riferimenti incrociati alle affermazioni
- [ ] Passaggio 4: Crea un riassunto strutturato
- [ ] Passaggio 5: Verifica le citazioni
```

**Passaggio 1: Leggi tutti i documenti di origine**

Rivedi ogni documento nella directory `sources/`. Nota gli argomenti principali e le prove di supporto.

**Passaggio 2: Identifica i temi chiave**

Cerca modelli tra le fonti. Quali temi appaiono ripetutamente? Dove le fonti concordano o discordano?

**Passaggio 3: Fai riferimenti incrociati alle affermazioni**

Per ogni affermazione principale, verifica che appaia nel materiale di origine. Nota quale fonte supporta ogni punto.

**Passaggio 4: Crea un riassunto strutturato**

Organizza i risultati per tema. Includi:
- Affermazione principale
- Prove di supporto dalle fonti
- Punti di vista conflittuali (se presenti)

**Passaggio 5: Verifica le citazioni**

Controlla che ogni affermazione faccia riferimento al documento di origine corretto. Se le citazioni sono incomplete, torna al Passaggio 3.
````

Questo esempio mostra come i flussi di lavoro si applicano a compiti di analisi che non richiedono codice. Il modello di checklist funziona per qualsiasi processo complesso e multi-passaggio.

**Esempio 2: Flusso di lavoro di compilazione moduli PDF** (per Skills con codice):

````markdown
## Flusso di lavoro di compilazione moduli PDF

Copia questa checklist e spunta gli elementi man mano che li completi:

```
Progresso dell'attività:
- [ ] Passaggio 1: Analizza il modulo (esegui analyze_form.py)
- [ ] Passaggio 2: Crea mappatura dei campi (modifica fields.json)
- [ ] Passaggio 3: Convalida la mappatura (esegui validate_fields.py)
- [ ] Passaggio 4: Compila il modulo (esegui fill_form.py)
- [ ] Passaggio 5: Verifica l'output (esegui verify_output.py)
```

**Passaggio 1: Analizza il modulo**

Esegui: `python scripts/analyze_form.py input.pdf`

Questo estrae i campi del modulo e le loro posizioni, salvando in `fields.json`.

**Passaggio 2: Crea mappatura dei campi**

Modifica `fields.json` per aggiungere valori per ogni campo.

**Passaggio 3: Convalida la mappatura**

Esegui: `python scripts/validate_fields.py fields.json`

Correggi eventuali errori di convalida prima di continuare.

**Passaggio 4: Compila il modulo**

Esegui: `python scripts/fill_form.py input.pdf fields.json output.pdf`

**Passaggio 5: Verifica l'output**

Esegui: `python scripts/verify_output.py output.pdf`

Se la verifica fallisce, torna al Passaggio 2.
````

I passaggi chiari impediscono a Claude di saltare la convalida critica. La checklist aiuta sia Claude che te a tracciare il progresso attraverso flussi di lavoro multi-passaggio.

### Implementa cicli di feedback

**Modello comune**: Esegui validatore → correggi errori → ripeti

Questo modello migliora notevolmente la qualità dell'output.

**Esempio 1: Conformità alla guida di stile** (per Skills senza codice):

```markdown
## Processo di revisione dei contenuti

1. Bozza dei tuoi contenuti seguendo le linee guida in STYLE_GUIDE.md
2. Rivedi rispetto alla checklist:
   - Controlla la coerenza della terminologia
   - Verifica che gli esempi seguano il formato standard
   - Conferma che tutte le sezioni richieste siano presenti
3. Se vengono trovati problemi:
   - Nota ogni problema con riferimento a sezione specifica
   - Rivedi il contenuto
   - Rivedi di nuovo la checklist
4. Procedi solo quando tutti i requisiti sono soddisfatti
5. Finalizza e salva il documento
```

Questo mostra il modello di ciclo di convalida usando documenti di riferimento invece di script. Il "validatore" è STYLE_GUIDE.md e Claude esegue il controllo leggendo e confrontando.

**Esempio 2: Processo di modifica di documenti** (per Skills con codice):

```markdown
## Processo di modifica di documenti

1. Fai le tue modifiche a `word/document.xml`
2. **Convalida immediatamente**: `python ooxml/scripts/validate.py unpacked_dir/`
3. Se la convalida fallisce:
   - Rivedi attentamente il messaggio di errore
   - Correggi i problemi nell'XML
   - Esegui di nuovo la convalida
4. **Procedi solo quando la convalida passa**
5. Ricostruisci: `python ooxml/scripts/pack.py unpacked_dir/ output.docx`
6. Testa il documento di output
```

Il ciclo di convalida cattura gli errori presto.

## Linee guida sui contenuti

### Evita informazioni sensibili al tempo

Non includere informazioni che diventeranno obsolete:

**Cattivo esempio: Sensibile al tempo** (diventerà sbagliato):
```markdown
Se stai facendo questo prima di agosto 2025, usa l'API vecchia.
Dopo agosto 2025, usa la nuova API.
```

**Buon esempio** (usa la sezione "modelli vecchi"):
```markdown
## Metodo attuale

Usa l'endpoint API v2: `api.example.com/v2/messages`

## Modelli vecchi

<details>
<summary>API v1 legacy (deprecato 2025-08)</summary>

L'API v1 usava: `api.example.com/v1/messages`

Questo endpoint non è più supportato.
</details>
```

La sezione dei modelli vecchi fornisce contesto storico senza ingombrare il contenuto principale.

### Usa terminologia coerente

Scegli un termine e usalo in tutta la Skill:

**Buono - Coerente**:
- Sempre "endpoint API"
- Sempre "campo"
- Sempre "estrai"

**Cattivo - Incoerente**:
- Mescola "endpoint API", "URL", "rotta API", "percorso"
- Mescola "campo", "casella", "elemento", "controllo"
- Mescola "estrai", "tira", "ottieni", "recupera"

La coerenza aiuta Claude a capire e seguire le istruzioni.

## Modelli comuni

### Modello di template

Fornisci template per il formato di output. Abbina il livello di rigidità alle tue necessità.

**Per requisiti rigorosi** (come risposte API o formati di dati):

````markdown
## Struttura del report

UTILIZZA SEMPRE questa struttura di template esatta:

```markdown
# [Titolo dell'analisi]

## Riassunto esecutivo
[Panoramica di una riga dei risultati chiave]

## Risultati chiave
- Risultato 1 con dati di supporto
- Risultato 2 con dati di supporto
- Risultato 3 con dati di supporto

## Raccomandazioni
1. Raccomandazione specifica e attuabile
2. Raccomandazione specifica e attuabile
```
````

**Per guida flessibile** (quando l'adattamento è utile):

````markdown
## Struttura del report

Ecco un formato predefinito sensato, ma usa il tuo miglior giudizio in base all'analisi:

```markdown
# [Titolo dell'analisi]

## Riassunto esecutivo
[Panoramica]

## Risultati chiave
[Adatta le sezioni in base a quello che scopri]

## Raccomandazioni
[Personalizza al contesto specifico]
```

Adatta le sezioni secondo le necessità per il tipo di analisi specifico.
````

### Modello di esempi

Per Skills dove la qualità dell'output dipende dalla visualizzazione di esempi, fornisci coppie input/output proprio come nel prompting regolare:

````markdown
## Formato del messaggio di commit

Genera messaggi di commit seguendo questi esempi:

**Esempio 1:**
Input: Aggiunto autenticazione utente con token JWT
Output:
```
feat(auth): implementa autenticazione basata su JWT

Aggiungi endpoint di login e middleware di convalida token
```

**Esempio 2:**
Input: Corretto bug dove le date venivano visualizzate in modo errato nei report
Output:
```
fix(reports): correggi la formattazione della data nella conversione del fuso orario

Usa timestamp UTC in modo coerente in tutta la generazione dei report
```

**Esempio 3:**
Input: Aggiornate le dipendenze e refactored la gestione degli errori
Output:
```
chore: aggiorna dipendenze e refactor gestione degli errori

- Aggiorna lodash a 4.17.21
- Standardizza il formato di risposta degli errori su tutti gli endpoint
```

Segui questo stile: type(scope): breve descrizione, quindi spiegazione dettagliata.
````

Gli esempi aiutano Claude a capire lo stile desiderato e il livello di dettaglio più chiaramente rispetto alle sole descrizioni.

### Modello di flusso di lavoro condizionale

Guida Claude attraverso i punti decisionali:

```markdown
## Flusso di lavoro di modifica di documenti

1. Determina il tipo di modifica:

   **Creazione di nuovo contenuto?** → Segui il "Flusso di lavoro di creazione" sottostante
   **Modifica di contenuto esistente?** → Segui il "Flusso di lavoro di modifica" sottostante

2. Flusso di lavoro di creazione:
   - Usa la libreria docx-js
   - Costruisci il documento da zero
   - Esporta nel formato .docx

3. Flusso di lavoro di modifica:
   - Estrai il documento esistente
   - Modifica direttamente l'XML
   - Convalida dopo ogni modifica
   - Ricompatta quando completato
```

<Tip>
Se i flussi di lavoro diventano grandi o complicati con molti passaggi, considera di spostarli in file separati e di dire a Claude di leggere il file appropriato in base al compito in questione.
</Tip>

## Valutazione e iterazione

### Costruisci valutazioni per prime

**Crea valutazioni PRIMA di scrivere documentazione estesa.** Questo garantisce che la tua Skill risolva problemi reali piuttosto che documentare quelli immaginati.

**Sviluppo guidato dalla valutazione:**
1. **Identifica i gap**: Esegui Claude su compiti rappresentativi senza una Skill. Documenta i fallimenti specifici o il contesto mancante
2. **Crea valutazioni**: Costruisci tre scenari che testano questi gap
3. **Stabilisci la baseline**: Misura le prestazioni di Claude senza la Skill
4. **Scrivi istruzioni minime**: Crea solo abbastanza contenuto per affrontare i gap e superare le valutazioni
5. **Itera**: Esegui le valutazioni, confronta con la baseline e affina

Questo approccio garantisce che stai risolvendo problemi effettivi piuttosto che anticipare requisiti che potrebbero non materializzarsi mai.

**Struttura di valutazione**:
```json
{
  "skills": ["pdf-processing"],
  "query": "Estrai tutto il testo da questo file PDF e salvalo in output.txt",
  "files": ["test-files/document.pdf"],
  "expected_behavior": [
    "Legge con successo il file PDF usando una libreria appropriata di elaborazione PDF o uno strumento da riga di comando",
    "Estrae il contenuto di testo da tutte le pagine del documento senza perdere alcuna pagina",
    "Salva il testo estratto in un file denominato output.txt in un formato chiaro e leggibile"
  ]
}
```

<Note>
Questo esempio dimostra una valutazione guidata dai dati con una semplice rubrica di test. Attualmente non forniamo un modo integrato per eseguire queste valutazioni. Gli utenti possono creare il proprio sistema di valutazione. Le valutazioni sono la tua fonte di verità per misurare l'efficacia della Skill.
</Note>

### Sviluppa Skills in modo iterativo con Claude

Il processo di sviluppo della Skill più efficace coinvolge Claude stesso. Lavora con un'istanza di Claude ("Claude A") per creare una Skill che sarà utilizzata da altre istanze ("Claude B"). Claude A ti aiuta a progettare e affinare le istruzioni, mentre Claude B le testa in compiti reali. Questo funziona perché i modelli Claude capiscono sia come scrivere istruzioni efficaci per gli agenti che quali informazioni gli agenti hanno bisogno.

**Creazione di una nuova Skill:**

1. **Completa un compito senza una Skill**: Lavora attraverso un problema con Claude A usando il prompting normale. Mentre lavori, fornirai naturalmente contesto, spiegherai preferenze e condividerai conoscenze procedurali. Nota quali informazioni fornisci ripetutamente.

2. **Identifica il modello riutilizzabile**: Dopo aver completato il compito, identifica quale contesto hai fornito che sarebbe utile per compiti futuri simili.

   **Esempio**: Se hai lavorato attraverso un'analisi BigQuery, potresti aver fornito nomi di tabelle, definizioni di campi, regole di filtraggio (come "escludi sempre gli account di test") e modelli di query comuni.

3. **Chiedi a Claude A di creare una Skill**: "Crea una Skill che catturi questo modello di analisi BigQuery che abbiamo appena usato. Includi gli schemi delle tabelle, le convenzioni di denominazione e la regola su come filtrare gli account di test."

   <Tip>
   I modelli Claude capiscono il formato e la struttura della Skill in modo nativo. Non hai bisogno di prompt di sistema speciali o di una "skill di scrittura di skills" per far sì che Claude aiuti a creare Skills. Semplicemente chiedi a Claude di creare una Skill e genererà contenuto SKILL.md correttamente strutturato con frontmatter e corpo appropriati.
   </Tip>

4. **Rivedi per concisione**: Controlla che Claude A non abbia aggiunto spiegazioni non necessarie. Chiedi: "Rimuovi la spiegazione su cosa significa il tasso di vincita - Claude lo sa già."

5. **Migliora l'architettura dell'informazione**: Chiedi a Claude A di organizzare il contenuto più efficacemente. Ad esempio: "Organizza questo in modo che lo schema della tabella sia in un file di riferimento separato. Potremmo aggiungere più tabelle in seguito."

6. **Testa su compiti simili**: Usa la Skill con Claude B (un'istanza fresca con la Skill caricata) su casi d'uso correlati. Osserva se Claude B trova le informazioni giuste, applica le regole correttamente e gestisce il compito con successo.

7. **Itera in base all'osservazione**: Se Claude B ha difficoltà o perde qualcosa, torna a Claude A con specifiche: "Quando Claude ha usato questa Skill, ha dimenticato di filtrare per data per Q4. Dovremmo aggiungere una sezione sui modelli di filtraggio per data?"

**Iterazione su Skills esistenti:**

Lo stesso modello gerarchico continua quando si migliorano le Skills. Alterna tra:
- **Lavorare con Claude A** (l'esperto che aiuta a affinare la Skill)
- **Testare con Claude B** (l'agente che usa la Skill per eseguire il lavoro reale)
- **Osservare il comportamento di Claude B** e portare gli insegnamenti a Claude A

1. **Usa la Skill in flussi di lavoro reali**: Dai a Claude B (con la Skill caricata) compiti effettivi, non scenari di test

2. **Osserva il comportamento di Claude B**: Nota dove ha difficoltà, riesce o fa scelte inaspettate

   **Esempio di osservazione**: "Quando ho chiesto a Claude B un report di vendita regionale, ha scritto la query ma ha dimenticato di filtrare gli account di test, anche se la Skill menziona questa regola."

3. **Torna a Claude A per miglioramenti**: Condividi il SKILL.md attuale e descrivi cosa hai osservato. Chiedi: "Ho notato che Claude B ha dimenticato di filtrare gli account di test quando ho chiesto un report regionale. La Skill menziona il filtraggio, ma forse non è abbastanza prominente?"

4. **Rivedi i suggerimenti di Claude A**: Claude A potrebbe suggerire di riorganizzare per rendere le regole più prominenti, usare un linguaggio più forte come "DEVE filtrare" invece di "sempre filtrare", o ristrutturare la sezione del flusso di lavoro.

5. **Applica e testa i cambiamenti**: Aggiorna la Skill con i perfezionamenti di Claude A, quindi testa di nuovo con Claude B su richieste simili

6. **Ripeti in base all'utilizzo**: Continua questo ciclo di osservazione-affinamento-test mentre incontri nuovi scenari. Ogni iterazione migliora la Skill in base al comportamento effettivo dell'agente, non alle assunzioni.

**Raccolta di feedback dal team:**

1. Condividi le Skills con i colleghi e osserva il loro utilizzo
2. Chiedi: La Skill si attiva quando previsto? Le istruzioni sono chiare? Cosa manca?
3. Incorpora il feedback per affrontare i punti ciechi nei tuoi modelli di utilizzo

**Perché questo approccio funziona**: Claude A capisce le esigenze dell'agente, tu fornisci l'expertise del dominio, Claude B rivela i gap attraverso l'utilizzo reale e il perfezionamento iterativo migliora le Skills in base al comportamento osservato piuttosto che alle assunzioni.

### Osserva come Claude naviga le Skills

Man mano che iteri sulle Skills, presta attenzione a come Claude le utilizza effettivamente nella pratica. Guarda:

- **Percorsi di esplorazione inaspettati**: Claude legge i file in un ordine che non hai anticipato? Questo potrebbe indicare che la tua struttura non è intuitiva come pensavi
- **Connessioni mancate**: Claude non riesce a seguire i riferimenti a file importanti? I tuoi link potrebbero aver bisogno di essere più espliciti o prominenti
- **Affidamento eccessivo a determinate sezioni**: Se Claude legge ripetutamente lo stesso file, considera se quel contenuto dovrebbe essere nel SKILL.md principale invece
- **Contenuto ignorato**: Se Claude non accede mai a un file raggruppato, potrebbe essere non necessario o segnalato male nelle istruzioni principali

Itera in base a queste osservazioni piuttosto che alle assunzioni. Il 'name' e la 'description' nei metadati della tua Skill sono particolarmente critici. Claude li usa quando decide se attivare la Skill in risposta al compito attuale. Assicurati che descrivano chiaramente cosa fa la Skill e quando dovrebbe essere usata.

## Anti-modelli da evitare

### Evita percorsi in stile Windows

Usa sempre barre oblique in avanti nei percorsi dei file, anche su Windows:

- ✓ **Buono**: `scripts/helper.py`, `reference/guide.md`
- ✗ **Evita**: `scripts\helper.py`, `reference\guide.md`

I percorsi in stile Unix funzionano su tutte le piattaforme, mentre i percorsi in stile Windows causano errori sui sistemi Unix.

### Evita di offrire troppe opzioni

Non presentare più approcci a meno che non sia necessario:

````markdown
**Cattivo esempio: Troppe scelte** (confuso):
"Puoi usare pypdf, o pdfplumber, o PyMuPDF, o pdf2image, o..."

**Buon esempio: Fornisci un predefinito** (con via di fuga):
"Usa pdfplumber per l'estrazione del testo:
```python
import pdfplumber
```

Per PDF scansionati che richiedono OCR, usa pdf2image con pytesseract invece."
````

## Avanzate: Skills con codice eseguibile

Le sezioni sottostanti si concentrano su Skills che includono script eseguibili. Se la tua Skill usa solo istruzioni markdown, salta a [Checklist per Skills efficaci](#checklist-per-skills-efficaci).

### Risolvi, non rimandare

Quando scrivi script per le Skills, gestisci le condizioni di errore piuttosto che rimandare a Claude.

**Buon esempio: Gestisci gli errori in modo esplicito**:
```python
def process_file(path):
    """Elabora un file, creandolo se non esiste."""
    try:
        with open(path) as f:
            return f.read()
    except FileNotFoundError:
        # Crea il file con contenuto predefinito invece di fallire
        print(f"File {path} non trovato, creazione predefinito")
        with open(path, 'w') as f:
            f.write('')
        return ''
    except PermissionError:
        # Fornisci un'alternativa invece di fallire
        print(f"Impossibile accedere a {path}, utilizzo predefinito")
        return ''
```

**Cattivo esempio: Rimanda a Claude**:
```python
def process_file(path):
    # Semplicemente fallisci e lascia che Claude lo capisca
    return open(path).read()
```

I parametri di configurazione dovrebbero anche essere giustificati e documentati per evitare "costanti voodoo" (legge di Ousterhout). Se non conosci il valore giusto, come Claude lo determinerà?

**Buon esempio: Auto-documentante**:
```python
# Le richieste HTTP in genere si completano entro 30 secondi
# Un timeout più lungo tiene conto delle connessioni lente
REQUEST_TIMEOUT = 30

# Tre tentativi bilanciano l'affidabilità rispetto alla velocità
# La maggior parte dei fallimenti intermittenti si risolvono al secondo tentativo
MAX_RETRIES = 3
```

**Cattivo esempio: Numeri magici**:
```python
TIMEOUT = 47  # Perché 47?
RETRIES = 5   # Perché 5?
```

### Fornisci script di utilità

Anche se Claude potrebbe scrivere uno script, gli script pre-realizzati offrono vantaggi:

**Vantaggi degli script di utilità**:
- Più affidabili del codice generato
- Risparmiano token (non è necessario includere il codice nel contesto)
- Risparmiano tempo (non è richiesta la generazione di codice)
- Garantiscono coerenza tra gli usi

![Raggruppamento di script eseguibili insieme ai file di istruzioni](/docs/images/agent-skills-executable-scripts.png)

Il diagramma sopra mostra come gli script eseguibili funzionano insieme ai file di istruzioni. Il file di istruzioni (forms.md) fa riferimento allo script e Claude può eseguirlo senza caricare i suoi contenuti nel contesto.

**Distinzione importante**: Rendi chiaro nelle tue istruzioni se Claude dovrebbe:
- **Eseguire lo script** (più comune): "Esegui `analyze_form.py` per estrarre i campi"
- **Leggerlo come riferimento** (per logica complessa): "Consulta `analyze_form.py` per l'algoritmo di estrazione dei campi"

Per la maggior parte degli script di utilità, l'esecuzione è preferita perché è più affidabile ed efficiente. Consulta la sezione [Ambiente di runtime](#runtime-environment) sottostante per i dettagli su come funziona l'esecuzione dello script.

**Esempio**:
````markdown
## Script di utilità

**analyze_form.py**: Estrai tutti i campi del modulo da PDF

```bash
python scripts/analyze_form.py input.pdf > fields.json
```

Formato di output:
```json
{
  "field_name": {"type": "text", "x": 100, "y": 200},
  "signature": {"type": "sig", "x": 150, "y": 500}
}
```

**validate_boxes.py**: Controlla i riquadri di delimitazione sovrapposti

```bash
python scripts/validate_boxes.py fields.json
# Restituisce: "OK" o elenca i conflitti
```

**fill_form.py**: Applica i valori dei campi al PDF

```bash
python scripts/fill_form.py input.pdf fields.json output.pdf
```
````

### Usa analisi visiva

Quando gli input possono essere renderizzati come immagini, fai analizzare a Claude:

````markdown
## Analisi del layout del modulo

1. Converti PDF in immagini:
   ```bash
   python scripts/pdf_to_images.py form.pdf
   ```

2. Analizza ogni immagine di pagina per identificare i campi del modulo
3. Claude può vedere visivamente le posizioni e i tipi di campo
````

<Note>
In questo esempio, dovresti scrivere lo script `pdf_to_images.py`.
</Note>

Le capacità di visione di Claude aiutano a capire layout e strutture.

### Crea output intermedi verificabili

Quando Claude esegue compiti complessi e aperti, può fare errori. Il modello "piano-convalida-esecuzione" cattura gli errori presto facendo sì che Claude crei prima un piano in un formato strutturato, quindi convalidi quel piano con uno script prima di eseguirlo.

**Esempio**: Immagina di chiedere a Claude di aggiornare 50 campi di modulo in un PDF in base a un foglio di calcolo. Senza convalida, Claude potrebbe fare riferimento a campi inesistenti, creare valori conflittuali, perdere campi obbligatori o applicare gli aggiornamenti in modo errato.

**Soluzione**: Usa il modello di flusso di lavoro mostrato sopra (compilazione moduli PDF), ma aggiungi un file intermedio `changes.json` che viene convalidato prima di applicare le modifiche. Il flusso di lavoro diventa: analizza → **crea file di piano** → **convalida il piano** → esegui → verifica.

**Perché questo modello funziona:**
- **Cattura gli errori presto**: La convalida trova i problemi prima che le modifiche vengano applicate
- **Verificabile da macchina**: Gli script forniscono una verifica oggettiva
- **Pianificazione reversibile**: Claude può iterare sul piano senza toccare gli originali
- **Debug chiaro**: I messaggi di errore puntano a problemi specifici

**Quando usare**: Operazioni batch, modifiche distruttive, regole di convalida complesse, operazioni ad alto rischio.

**Suggerimento di implementazione**: Rendi gli script di convalida dettagliati con messaggi di errore specifici come "Campo 'signature_date' non trovato. Campi disponibili: customer_name, order_total, signature_date_signed" per aiutare Claude a correggere i problemi.

### Pacchetto dipendenze

Le Skills vengono eseguite nell'ambiente di esecuzione del codice con limitazioni specifiche della piattaforma:

- **claude.ai**: Può installare pacchetti da npm e PyPI e tirare da repository GitHub
- **API Anthropic**: Non ha accesso alla rete e nessuna installazione di pacchetti di runtime

Elenca i pacchetti richiesti nel tuo SKILL.md e verifica che siano disponibili nella [documentazione dello strumento di esecuzione del codice](/docs/it/agents-and-tools/tool-use/code-execution-tool).

### Ambiente di runtime

Le Skills vengono eseguite in un ambiente di esecuzione del codice con accesso al filesystem, comandi bash e capacità di esecuzione del codice. Per la spiegazione concettuale di questa architettura, consulta [L'architettura delle Skills](/docs/it/agents-and-tools/agent-skills/overview#the-skills-architecture) nella panoramica.

**Come questo influisce sulla tua authoring:**

**Come Claude accede alle Skills:**

1. **Metadati precaricati**: All'avvio, il nome e la descrizione dal frontmatter YAML di tutte le Skills vengono caricati nel prompt di sistema
2. **File letti su richiesta**: Claude usa strumenti di lettura bash per accedere a SKILL.md e altri file dal filesystem quando necessario
3. **Script eseguiti in modo efficiente**: Gli script di utilità possono essere eseguiti tramite bash senza caricare i loro contenuti completi nel contesto. Solo l'output dello script consuma token
4. **Nessuna penalità di contesto per file di grandi dimensioni**: File di riferimento, dati o documentazione non consumano token di contesto fino a quando non vengono effettivamente letti

- **I percorsi dei file contano**: Claude naviga la directory della tua skill come un filesystem. Usa barre oblique in avanti (`reference/guide.md`), non barre oblique inverse
- **Nomina i file in modo descrittivo**: Usa nomi che indicano il contenuto: `form_validation_rules.md`, non `doc2.md`
- **Organizza per scoperta**: Struttura le directory per dominio o funzionalità
  - Buono: `reference/finance.md`, `reference/sales.md`
  - Cattivo: `docs/file1.md`, `docs/file2.md`
- **Raggruppa risorse complete**: Includi documentazione API completa, esempi estesi, set di dati di grandi dimensioni; nessuna penalità di contesto fino a quando non vengono accessibili
- **Preferisci script per operazioni deterministiche**: Scrivi `validate_form.py` piuttosto che chiedere a Claude di generare codice di convalida
- **Rendi chiaro l'intento di esecuzione**:
  - "Esegui `analyze_form.py` per estrarre i campi" (esegui)
  - "Consulta `analyze_form.py` per l'algoritmo di estrazione" (leggi come riferimento)
- **Testa i modelli di accesso ai file**: Verifica che Claude possa navigare la struttura della tua directory testando con richieste reali

**Esempio:**

```
bigquery-skill/
├── SKILL.md (panoramica, punta ai file di riferimento)
└── reference/
    ├── finance.md (metriche di ricavi)
    ├── sales.md (dati della pipeline)
    └── product.md (analitiche di utilizzo)
```

Quando l'utente chiede informazioni sui ricavi, Claude legge SKILL.md, vede il riferimento a `reference/finance.md` e invoca bash per leggere solo quel file. I file sales.md e product.md rimangono sul filesystem, consumando zero token di contesto fino a quando non vengono necessari. Questo modello basato su filesystem è ciò che abilita la divulgazione progressiva. Claude può navigare e caricare selettivamente esattamente quello che ogni compito richiede.

Per i dettagli tecnici completi sull'architettura, consulta [Come funzionano le Skills](/docs/it/agents-and-tools/agent-skills/overview#how-skills-work) nella panoramica delle Skills.

### Riferimenti a strumenti MCP

Se la tua Skill usa strumenti MCP (Model Context Protocol), usa sempre nomi di strumenti completamente qualificati per evitare errori "strumento non trovato".

**Formato**: `ServerName:tool_name`

**Esempio**:
```markdown
Usa lo strumento BigQuery:bigquery_schema per recuperare gli schemi delle tabelle.
Usa lo strumento GitHub:create_issue per creare problemi.
```

Dove:
- `BigQuery` e `GitHub` sono nomi di server MCP
- `bigquery_schema` e `create_issue` sono i nomi degli strumenti all'interno di quei server

Senza il prefisso del server, Claude potrebbe non riuscire a individuare lo strumento, specialmente quando sono disponibili più server MCP.

### Evita di assumere che gli strumenti siano installati

Non assumere che i pacchetti siano disponibili:

````markdown
**Cattivo esempio: Assume l'installazione**:
"Usa la libreria pdf per elaborare il file."

**Buon esempio: Esplicito sulle dipendenze**:
"Installa il pacchetto richiesto: `pip install pypdf`

Quindi usalo:
```python
from pypdf import PdfReader
reader = PdfReader("file.pdf")
```"
````

## Note tecniche

### Requisiti del frontmatter YAML

Il frontmatter di SKILL.md richiede campi `name` e `description` con regole di convalida specifiche:
- `name`: Massimo 64 caratteri, solo lettere minuscole/numeri/trattini, nessun tag XML, nessuna parola riservata
- `description`: Massimo 1024 caratteri, non vuoto, nessun tag XML

Consulta la [panoramica delle Skills](/docs/it/agents-and-tools/agent-skills/overview#skill-structure) per i dettagli completi della struttura.

### Budget di token

Mantieni il corpo di SKILL.md sotto 500 righe per prestazioni ottimali. Se il tuo contenuto supera questo, dividilo in file separati usando i modelli di divulgazione progressiva descritti in precedenza. Per i dettagli architettonici, consulta la [panoramica delle Skills](/docs/it/agents-and-tools/agent-skills/overview#how-skills-work).

## Checklist per Skills efficaci

Prima di condividere una Skill, verifica:

### Qualità di base
- [ ] La descrizione è specifica e include termini chiave
- [ ] La descrizione include sia cosa fa la Skill che quando usarla
- [ ] Il corpo di SKILL.md è sotto 500 righe
- [ ] I dettagli aggiuntivi sono in file separati (se necessario)
- [ ] Nessuna informazione sensibile al tempo (o nella sezione "modelli vecchi")
- [ ] Terminologia coerente in tutto
- [ ] Gli esempi sono concreti, non astratti
- [ ] I riferimenti ai file sono a un livello di profondità
- [ ] La divulgazione progressiva è usata in modo appropriato
- [ ] I flussi di lavoro hanno passaggi chiari

### Codice e script
- [ ] Gli script risolvono i problemi piuttosto che rimandare a Claude
- [ ] La gestione degli errori è esplicita e utile
- [ ] Nessuna "costante voodoo" (tutti i valori giustificati)
- [ ] I pacchetti richiesti sono elencati nelle istruzioni e verificati come disponibili
- [ ] Gli script hanno una documentazione chiara
- [ ] Nessun percorso in stile Windows (tutte le barre oblique in avanti)
- [ ] Passaggi di convalida/verifica per operazioni critiche
- [ ] Cicli di feedback inclusi per compiti critici per la qualità

### Test
- [ ] Almeno tre valutazioni create
- [ ] Testato con Haiku, Sonnet e Opus
- [ ] Testato con scenari di utilizzo reali
- [ ] Feedback del team incorporato (se applicabile)

## Prossimi passaggi

<CardGroup cols={2}>
  <Card
    title="Inizia con Agent Skills"
    icon="rocket"
    href="/docs/it/agents-and-tools/agent-skills/quickstart"
  >
    Crea la tua prima Skill
  </Card>
  <Card
    title="Usa Skills in Claude Code"
    icon="terminal"
    href="https://code.claude.com/docs/skills"
  >
    Crea e gestisci Skills in Claude Code
  </Card>
  <Card
    title="Usa Skills in Agent SDK"
    icon="cube"
    href="/docs/it/agent-sdk/skills"
  >
    Usa le Skills a livello di programmazione in TypeScript e Python
  </Card>
  <Card
    title="Usa Skills con l'API"
    icon="code"
    href="/docs/it/build-with-claude/skills-guide"
  >
    Carica e usa le Skills a livello di programmazione
  </Card>
</CardGroup>