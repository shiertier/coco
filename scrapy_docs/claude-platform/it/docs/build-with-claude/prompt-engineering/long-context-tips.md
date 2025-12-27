# Suggerimenti per il prompting con contesto lungo

Suggerimenti per sfruttare efficacemente la finestra di contesto estesa di Claude per gestire compiti complessi e ricchi di dati.

---

<Note>
While these tips apply broadly to all Claude models, you can find prompting tips specific to extended thinking models [here](/docs/en/build-with-claude/prompt-engineering/extended-thinking-tips).
</Note>

La finestra di contesto estesa di Claude (200K token per i modelli Claude 3) consente di gestire compiti complessi e ricchi di dati. Questa guida ti aiuterà a sfruttare questo potere in modo efficace.

## Suggerimenti essenziali per i prompt con contesto lungo

- **Posiziona i dati in formato lungo in alto**: Posiziona i tuoi documenti lunghi e input (~20K+ token) vicino alla parte superiore del tuo prompt, sopra la tua query, istruzioni ed esempi. Questo può migliorare significativamente le prestazioni di Claude su tutti i modelli.

    <Note>Le query alla fine possono migliorare la qualità della risposta fino al 30% nei test, soprattutto con input complessi e multi-documento.</Note>

- **Struttura il contenuto del documento e i metadati con tag XML**: Quando utilizzi più documenti, racchiudi ogni documento in tag `<document>` con sottotag `<document_content>` e `<source>` (e altri metadati) per chiarezza.

    <section title="Esempio di struttura multi-documento">

    ```xml
    <documents>
      <document index="1">
        <source>annual_report_2023.pdf</source>
        <document_content>
          {{ANNUAL_REPORT}}
        </document_content>
      </document>
      <document index="2">
        <source>competitor_analysis_q2.xlsx</source>
        <document_content>
          {{COMPETITOR_ANALYSIS}}
        </document_content>
      </document>
    </documents>

    Analizza il rapporto annuale e l'analisi della concorrenza. Identifica i vantaggi strategici e consiglia le aree di focus per il Q3.
    ```
    
</section>

- **Basa le risposte su citazioni**: Per compiti con documenti lunghi, chiedi a Claude di citare prima le parti rilevanti dei documenti prima di eseguire il suo compito. Questo aiuta Claude a superare il "rumore" del resto del contenuto del documento.

    <section title="Esempio di estrazione di citazioni">

    ```xml
    Sei un assistente medico AI. Il tuo compito è aiutare i medici a diagnosticare possibili malattie dei pazienti.

    <documents>
      <document index="1">
        <source>patient_symptoms.txt</source>
        <document_content>
          {{PATIENT_SYMPTOMS}}
        </document_content>
      </document>
      <document index="2">
        <source>patient_records.txt</source>
        <document_content>
          {{PATIENT_RECORDS}}
        </document_content>
      </document>
      <document index="3">
        <source>patient01_appt_history.txt</source>
        <document_content>
          {{PATIENT01_APPOINTMENT_HISTORY}}
        </document_content>
      </document>
    </documents>

    Trova citazioni dai registri dei pazienti e dalla cronologia degli appuntamenti che sono rilevanti per diagnosticare i sintomi segnalati dal paziente. Posiziona questi in tag <quotes>. Quindi, in base a queste citazioni, elenca tutte le informazioni che aiuterebbero il medico a diagnosticare i sintomi del paziente. Posiziona le tue informazioni diagnostiche in tag <info>.
    ```
    
</section>

***

<CardGroup cols={3}>
  <Card title="Libreria di prompt" icon="link" href="/docs/it/resources/prompt-library/library">
    Ispirati da una selezione curata di prompt per vari compiti e casi d'uso.
  </Card>
  <Card title="Tutorial di prompting su GitHub" icon="link" href="https://github.com/anthropics/prompt-eng-interactive-tutorial">
    Un tutorial ricco di esempi che copre i concetti di prompt engineering trovati nella nostra documentazione.
  </Card>
  <Card title="Tutorial di prompting su Google Sheets" icon="link" href="https://docs.google.com/spreadsheets/d/19jzLgRruG9kjUQNKtCg1ZjdD6l6weA6qRXG5zLIAhC8">
    Una versione più leggera del nostro tutorial di prompt engineering tramite un foglio di calcolo interattivo.
  </Card>
</CardGroup>