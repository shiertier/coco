# Dare a Claude un ruolo con un prompt di sistema

Scopri come usare il parametro system per assegnare a Claude un ruolo specifico e migliorare drasticamente le sue prestazioni.

---

<Note>
While these tips apply broadly to all Claude models, you can find prompting tips specific to extended thinking models [here](/docs/en/build-with-claude/prompt-engineering/extended-thinking-tips).
</Note>

Quando usi Claude, puoi migliorare drasticamente le sue prestazioni utilizzando il parametro `system` per assegnargli un ruolo. Questa tecnica, nota come role prompting, è il modo più potente per usare i prompt di sistema con Claude.

Il ruolo giusto può trasformare Claude da un assistente generico nel tuo esperto di dominio virtuale!

<Tip>**Suggerimenti per il prompt di sistema**: Usa il parametro `system` per impostare il ruolo di Claude. Metti tutto il resto, come le istruzioni specifiche del compito, nel turno `user` invece.</Tip>

## Perché usare il role prompting?

- **Precisione migliorata:** In scenari complessi come l'analisi legale o la modellazione finanziaria, il role prompting può aumentare significativamente le prestazioni di Claude.
- **Tono personalizzato:** Che tu abbia bisogno della brevità di un CFO o della creatività di uno copywriter, il role prompting adatta lo stile di comunicazione di Claude.
- **Messa a fuoco migliorata:** Impostando il contesto del ruolo, Claude rimane più entro i confini dei requisiti specifici del tuo compito.

***

## Come assegnare un ruolo a Claude

Usa il parametro `system` nell'[API Messages](/docs/it/api/messages) per impostare il ruolo di Claude:

```python
import anthropic

client = anthropic.Anthropic()

response = client.messages.create(
    model="claude-sonnet-4-5-20250929",
    max_tokens=2048,
    system="You are a seasoned data scientist at a Fortune 500 company.", # <-- role prompt
    messages=[
        {"role": "user", "content": "Analyze this dataset for anomalies: <dataset>{{DATASET}}</dataset>"}
    ]
)

print(response.content)
```

<Tip>**Suggerimento per il role prompting**: Sperimenta con i ruoli! Un `data scientist` potrebbe vedere intuizioni diverse rispetto a un `marketing strategist` per gli stessi dati. Un `data scientist specializing in customer insight analysis for Fortune 500 companies` potrebbe produrre risultati ancora diversi!</Tip>

***

## Esempi

### Esempio 1: Analisi di contratti legali
Senza un ruolo, Claude potrebbe perdere problemi critici:

<section title="Analisi di contratti legali senza role prompting">

| Ruolo | Contenuto |
| ---- | ------- |
| User | Analizza questo accordo di licenza software per potenziali rischi:<br/>\<contract><br/>\{\{CONTRACT\}\}<br/>\</contract><br/><br/>Concentrati su indennizzo, responsabilità e proprietà intellettuale. |
| Assistant | Ecco un riepilogo dei punti chiave nell'accordo di licenza software:<br/><br/>1. Indennizzo (Sezione 8):<br/>   - Indenizziamo il fornitore contro i reclami derivanti dal nostro utilizzo.<br/><br/>2. Limitazione della responsabilità (Sezione 9):<br/>   - Danni limitati a \$500.<br/><br/>3. Proprietà intellettuale (Sezione 12):<br/>   - Manteniamo la proprietà delle nostre modifiche.<br/><br/>Nel complesso, l'accordo sembra standard. Le clausole di indennizzo e responsabilità sono tipiche e manteniamo i nostri diritti di proprietà intellettuale. |

</section>

Con un ruolo, Claude individua problemi critici che potrebbero costare milioni:

<section title="Analisi di contratti legali con role prompting">

| Ruolo | Contenuto |
| ---- | ------- |
| User | Sei il General Counsel di un'azienda tecnologica Fortune 500. Stiamo considerando questo accordo di licenza software per la nostra infrastruttura dati principale:<br/>\<contract><br/>\{\{CONTRACT\}\}<br/>\</contract><br/><br/>Analizzalo per potenziali rischi, concentrandoti su indennizzo, responsabilità e proprietà intellettuale. Dai la tua opinione professionale. |
| Assistant | Ho revisionato l'accordo di licenza software e ho serie preoccupazioni che potrebbero esporre la nostra azienda a rischi significativi:<br/><br/>1. Indennizzo (Sezione 8):<br/>   - Problema critico: Indenizziamo il fornitore contro "qualsiasi e tutti i reclami" derivanti dal nostro utilizzo.<br/>   - Rischio: Questo potrebbe renderci responsabili anche per la negligenza del fornitore o i difetti del prodotto.<br/>   - Raccomandazione: Rifiuta. Limita il nostro indennizzo ai reclami derivanti dalla nostra violazione dell'accordo.<br/><br/>2. Limitazione della responsabilità (Sezione 9):<br/>   - Problema critico: Danni limitati a \$500, inclusi i danni diretti.<br/>   - Rischio: Grossolanamente inadeguato. Un guasto critico nella nostra infrastruttura dati potrebbe costarci milioni in perdite di entrate e danno reputazionale.<br/>   - Raccomandazione: Rifiuta. Negozia un limite di 12 mesi di commissioni per i danni diretti e esclusioni per le violazioni dei dati.<br/><br/>3. Proprietà intellettuale (Sezione 12):<br/>   - Problema critico: Il fornitore acquisisce la proprietà congiunta delle nostre modifiche e integrazioni.<br/>   - Rischio: Potrebbero usare o vendere i nostri algoritmi proprietari o le intuizioni sui dati ai concorrenti.<br/>   - Raccomandazione: Rifiuta. Dobbiamo mantenere la piena proprietà del nostro lavoro.<br/><br/>Opinione professionale: Non firmare questo accordo. È estremamente sfavorevole e ci espone a livelli inaccettabili di rischio. Se il fornitore rifiuta di rinegoziare questi termini, consiglio vivamente di esplorare fornitori alternativi o accelerare la nostra soluzione interna. I costi potenziali di procedere con questo accordo superano di gran lunga i guadagni a breve termine. |

</section>

### Esempio 2: Analisi finanziaria
Senza un ruolo, l'analisi di Claude manca di profondità:

<section title="Analisi finanziaria senza role prompting">

| Ruolo | Contenuto |
| ---- | ------- |
| User | Analizza questo dataset dei nostri dati finanziari Q2:<br/>\<data><br/>\{\{FINANCIALS\}\}<br/>\</data><br/><br/>Evidenzia i trend chiave e consiglia azioni. |
| Assistant | Ecco un riepilogo dei tuoi dati finanziari Q2:<br/><br/>1. Ricavi: \$15.2M, in aumento del 22% YoY. La base di clienti aziendali è cresciuta del 30%.<br/>2. Margine lordo: 72%, in aumento del 3% rispetto a Q1.<br/>3. Margine EBITDA: 18%, in calo del 2% a causa degli investimenti in R&D.<br/>4. Flusso di cassa: \$4.1M operativo, \$28M in riserve.<br/><br/>Nel complesso, i dati finanziari sembrano forti con crescita dei ricavi e margini sani. L'aumento della spesa in R&D sta impattando la redditività ma potrebbe ripagare a lungo termine. Mantieni il corso attuale. |

</section>

Con un ruolo, Claude fornisce intuizioni attuabili:

<section title="Analisi finanziaria con role prompting">

| Ruolo | Contenuto |
| ---- | ------- |
| User | Sei il CFO di un'azienda B2B SaaS in rapida crescita. Siamo in una riunione del consiglio di amministrazione che discute i nostri dati finanziari Q2:<br/>\<data><br/>\{\{FINANCIALS\}\}<br/>\</data><br/><br/>Analizza i trend chiave, segnala le preoccupazioni e consiglia azioni strategiche. I nostri investitori vogliono una crescita aggressiva ma sono cauti riguardo al nostro burn rate. |
| Assistant | Membri del consiglio, ecco la mia analisi dei nostri dati finanziari Q2:<br/><br/>1. Ricavi (\$15.2M, +22% YoY):<br/>   - Intuizione: La crescita aziendale (+30%) sta superando la crescita complessiva.<br/>   - Segnalazione: Il segmento PMB è diminuito del 5%.<br/>   - Azione: Riallocare il 20% del budget di marketing PMB alle vendite aziendali. Incarica il prodotto di semplificare l'onboarding PMB.<br/><br/>2. Margini (Lordo: 72%, +3% QoQ; EBITDA: 18%, -2% QoQ):<br/>   - Intuizione: Le ottimizzazioni dell'infrastruttura hanno aumentato i margini lordi.<br/>   - Segnalazione: La spesa in R&D per le funzioni AI ha intaccato l'EBITDA.<br/>   - Azione: Nessun cambiamento. La nostra roadmap AI è critica per la fedeltà aziendale.<br/><br/>3. Cassa (\$4.1M OCF, \$28M riserve):<br/>   - Intuizione: 15 mesi di runway al burn attuale.<br/>   - Segnalazione: CAC in aumento del 20%, erodendo LTV/CAC.<br/>   - Azione: Congela le assunzioni non essenziali. Implementa l'analisi dell'imbuto di vendita per ridurre il CAC del 15% in Q3.<br/><br/>Posizione strategica: Stiamo navigando tra crescita e burn. La focalizzazione aziendale è il nostro motore di crescita—investi lì. Ma monitora il churn PMB; potrebbe estendersi all'azienda. La nostra scommessa AI è solida, ma dobbiamo convertirla in ARR entro Q4, altrimenti riduciamo la R&D del 30%.<br/><br/>In sintesi: Raddoppia l'impegno sull'azienda, semplifica PMB, ottimizza le vendite e monetizza AI. Domande? |

</section>

---

<CardGroup cols={3}>
  <Card title="Libreria di prompt" icon="link" href="/docs/it/resources/prompt-library/library">
    Ispirati da una selezione curata di prompt per vari compiti e casi d'uso.
  </Card>
  <Card title="Tutorial di prompting GitHub" icon="link" href="https://github.com/anthropics/prompt-eng-interactive-tutorial">
    Un tutorial ricco di esempi che copre i concetti di prompt engineering trovati nella nostra documentazione.
  </Card>
  <Card title="Tutorial di prompting Google Sheets" icon="link" href="https://docs.google.com/spreadsheets/d/19jzLgRruG9kjUQNKtCg1ZjdD6l6weA6qRXG5zLIAhC8">
    Una versione più leggera del nostro tutorial di prompt engineering tramite un foglio di calcolo interattivo.
  </Card>
</CardGroup>