# Panoramica dell'ingegneria dei prompt

Scopri come ottimizzare i tuoi prompt per Claude con tecniche di ingegneria dei prompt efficaci.

---

<Note>
While these tips apply broadly to all Claude models, you can find prompting tips specific to extended thinking models [here](/docs/en/build-with-claude/prompt-engineering/extended-thinking-tips).
</Note>

## Prima dell'ingegneria dei prompt

Questa guida presuppone che tu abbia:
1. Una definizione chiara dei criteri di successo per il tuo caso d'uso
2. Alcuni modi per testare empiricamente rispetto a questi criteri
3. Una bozza iniziale di prompt che desideri migliorare

Se non è così, ti consigliamo vivamente di dedicare tempo a stabilire prima questo. Consulta [Definisci i tuoi criteri di successo](/docs/it/test-and-evaluate/define-success) e [Crea valutazioni empiriche forti](/docs/it/test-and-evaluate/develop-tests) per suggerimenti e orientamenti.

<Card title="Generatore di prompt" icon="link" href="/dashboard">
  Non hai una bozza iniziale di prompt? Prova il generatore di prompt nella Claude Console!
</Card>

***

## Quando fare ingegneria dei prompt

  Questa guida si concentra sui criteri di successo che sono controllabili attraverso l'ingegneria dei prompt.
  Non tutti i criteri di successo o le valutazioni fallite sono meglio risolti dall'ingegneria dei prompt. Ad esempio, la latenza e il costo possono a volte essere migliorati più facilmente selezionando un modello diverso.

<section title="Prompting vs. fine-tuning">

  L'ingegneria dei prompt è molto più veloce di altri metodi di controllo del comportamento del modello, come il fine-tuning, e spesso può produrre salti di prestazioni in molto meno tempo. Ecco alcuni motivi per considerare l'ingegneria dei prompt rispetto al fine-tuning:<br/>
  - **Efficienza delle risorse**: Il fine-tuning richiede GPU di fascia alta e molta memoria, mentre l'ingegneria dei prompt richiede solo input di testo, rendendola molto più efficiente dal punto di vista delle risorse.
  - **Convenienza economica**: Per i servizi di IA basati su cloud, il fine-tuning comporta costi significativi. L'ingegneria dei prompt utilizza il modello base, che è tipicamente più economico.
  - **Mantenimento degli aggiornamenti del modello**: Quando i provider aggiornano i modelli, le versioni con fine-tuning potrebbero necessitare di un nuovo addestramento. I prompt di solito funzionano tra le versioni senza modifiche.
  - **Risparmio di tempo**: Il fine-tuning può richiedere ore o addirittura giorni. Al contrario, l'ingegneria dei prompt fornisce risultati quasi istantanei, consentendo una risoluzione rapida dei problemi.
  - **Esigenze di dati minime**: Il fine-tuning necessita di dati etichettati e specifici per l'attività in quantità sostanziale, che possono essere scarsi o costosi. L'ingegneria dei prompt funziona con apprendimento few-shot o addirittura zero-shot.
  - **Flessibilità e iterazione rapida**: Prova rapidamente vari approcci, modifica i prompt e visualizza i risultati immediati. Questa sperimentazione rapida è difficile con il fine-tuning.
  - **Adattamento del dominio**: Adatta facilmente i modelli a nuovi domini fornendo contesto specifico del dominio nei prompt, senza necessità di riaddestrare.
  - **Miglioramenti della comprensione**: L'ingegneria dei prompt è molto più efficace del fine-tuning nell'aiutare i modelli a comprendere e utilizzare meglio contenuti esterni come documenti recuperati
  - **Preserva la conoscenza generale**: Il fine-tuning rischia l'oblio catastrofico, dove il modello perde la conoscenza generale. L'ingegneria dei prompt mantiene le ampie capacità del modello.
  - **Trasparenza**: I prompt sono leggibili dall'uomo, mostrando esattamente quali informazioni riceve il modello. Questa trasparenza aiuta nella comprensione e nel debug.

</section>

***

## Come fare ingegneria dei prompt

Le pagine di ingegneria dei prompt in questa sezione sono state organizzate dalle tecniche più ampiamente efficaci alle tecniche più specializzate. Quando risolvi i problemi di prestazioni, ti consigliamo di provare queste tecniche in ordine, anche se l'impatto effettivo di ogni tecnica dipenderà dal tuo caso d'uso.
1. [Generatore di prompt](/docs/it/build-with-claude/prompt-engineering/prompt-generator)
2. [Sii chiaro e diretto](/docs/it/build-with-claude/prompt-engineering/be-clear-and-direct)
3. [Usa esempi (multishot)](/docs/it/build-with-claude/prompt-engineering/multishot-prompting)
4. [Lascia che Claude pensi (chain of thought)](/docs/it/build-with-claude/prompt-engineering/chain-of-thought)
5. [Usa tag XML](/docs/it/build-with-claude/prompt-engineering/use-xml-tags)
6. [Dai a Claude un ruolo (system prompts)](/docs/it/build-with-claude/prompt-engineering/system-prompts)
7. [Precompila la risposta di Claude](/docs/it/build-with-claude/prompt-engineering/prefill-claudes-response)
8. [Concatena prompt complessi](/docs/it/build-with-claude/prompt-engineering/chain-prompts)
9. [Suggerimenti per il contesto lungo](/docs/it/build-with-claude/prompt-engineering/long-context-tips)

***

## Tutorial di ingegneria dei prompt

Se sei un discente interattivo, puoi invece tuffarti nei nostri tutorial interattivi!

<CardGroup cols={2}>
  <Card title="Tutorial di prompting GitHub" icon="link" href="https://github.com/anthropics/prompt-eng-interactive-tutorial">
    Un tutorial ricco di esempi che copre i concetti di ingegneria dei prompt trovati nella nostra documentazione.
  </Card>
  <Card title="Tutorial di prompting Google Sheets" icon="link" href="https://docs.google.com/spreadsheets/d/19jzLgRruG9kjUQNKtCg1ZjdD6l6weA6qRXG5zLIAhC8">
    Una versione più leggera del nostro tutorial di ingegneria dei prompt tramite un foglio di calcolo interattivo.
  </Card>
</CardGroup>