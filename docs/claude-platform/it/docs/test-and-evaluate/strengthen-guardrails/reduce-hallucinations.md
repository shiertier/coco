# Ridurre le allucinazioni

---

Anche i modelli linguistici più avanzati, come Claude, possono talvolta generare testo che è fattualmente errato o incoerente con il contesto dato. Questo fenomeno, noto come "allucinazione", può minare l'affidabilità delle tue soluzioni basate sull'IA.
Questa guida esplorerà le tecniche per minimizzare le allucinazioni e garantire che gli output di Claude siano accurati e affidabili.

## Strategie di base per minimizzare le allucinazioni

- **Permettere a Claude di dire "Non lo so":** Dai esplicitamente a Claude il permesso di ammettere l'incertezza. Questa semplice tecnica può ridurre drasticamente le informazioni false.

<section title="Esempio: Analisi di un report di fusione e acquisizione">

| Ruolo | Contenuto |
| ---- | --- |
| Utente | Come nostro consulente M&A, analizza questo report sull'acquisizione potenziale di AcmeCo da parte di ExampleCorp.<br/><br/>\<report><br/>\{\{REPORT}}<br/>\</report><br/><br/>Concentrati sulle proiezioni finanziarie, i rischi di integrazione e gli ostacoli normativi. Se non sei sicuro di qualche aspetto o se il report manca di informazioni necessarie, di' "Non ho informazioni sufficienti per valutare questo con sicurezza." |

</section>

- **Usa citazioni dirette per fondamento fattuale:** Per compiti che coinvolgono documenti lunghi (>20K token), chiedi a Claude di estrarre prima citazioni parola per parola prima di eseguire il suo compito. Questo ancora le sue risposte al testo effettivo, riducendo le allucinazioni.

<section title="Esempio: Revisione di una politica sulla privacy dei dati">

| Ruolo | Contenuto |
| ---- | --- |
| Utente | Come nostro Responsabile della Protezione dei Dati, rivedi questa politica sulla privacy aggiornata per la conformità GDPR e CCPA.<br/>\<br/>\{\{POLICY}}<br/>\</policy><br/><br/>1. Estrai citazioni esatte dalla politica che sono più rilevanti per la conformità GDPR e CCPA. Se non riesci a trovare citazioni rilevanti, dichiara "Nessuna citazione rilevante trovata."<br/><br/>2. Usa le citazioni per analizzare la conformità di queste sezioni della politica, facendo riferimento alle citazioni per numero. Basa la tua analisi solo sulle citazioni estratte. |

</section>

- **Verifica con citazioni**: Rendi la risposta di Claude verificabile facendogli citare citazioni e fonti per ciascuna delle sue affermazioni. Puoi anche far verificare a Claude ogni affermazione trovando una citazione di supporto dopo che genera una risposta. Se non riesce a trovare una citazione, deve ritirare l'affermazione.

<section title="Esempio: Redazione di un comunicato stampa sul lancio di un prodotto">

| Ruolo | Contenuto |
| ---- | --- |
| Utente | Redigi un comunicato stampa per il nostro nuovo prodotto di cybersecurity, AcmeSecurity Pro, usando solo informazioni da questi brief di prodotto e report di mercato.<br/>\<documents><br/>\{\{DOCUMENTS}}<br/>\</documents><br/><br/>Dopo la redazione, rivedi ogni affermazione nel tuo comunicato stampa. Per ogni affermazione, trova una citazione diretta dai documenti che la supporta. Se non riesci a trovare una citazione di supporto per un'affermazione, rimuovi quell'affermazione dal comunicato stampa e segna dove è stata rimossa con parentesi vuote []. |

</section>

***

## Tecniche avanzate

- **Verifica della catena di pensiero**: Chiedi a Claude di spiegare il suo ragionamento passo dopo passo prima di dare una risposta finale. Questo può rivelare logica o supposizioni errate.

- **Verifica Best-of-N**: Esegui Claude attraverso lo stesso prompt più volte e confronta gli output. Incongruenze tra gli output potrebbero indicare allucinazioni.

- **Raffinamento iterativo**: Usa gli output di Claude come input per prompt di follow-up, chiedendogli di verificare o espandere le dichiarazioni precedenti. Questo può individuare e correggere incongruenze.

- **Restrizione della conoscenza esterna**: Istruisci esplicitamente Claude di utilizzare solo informazioni dai documenti forniti e non la sua conoscenza generale.

<Note>Ricorda, mentre queste tecniche riducono significativamente le allucinazioni, non le eliminano completamente. Verifica sempre le informazioni critiche, specialmente per decisioni ad alto rischio.</Note>