# Strumento di controllo del computer

Claude può interagire con ambienti desktop attraverso lo strumento di controllo del computer, che fornisce capacità di screenshot e controllo del mouse/tastiera per l'interazione autonoma del desktop.

---

Claude può interagire con ambienti desktop attraverso lo strumento di controllo del computer, che fornisce capacità di screenshot e controllo del mouse/tastiera per l'interazione autonoma del desktop.

<Note>
Il controllo del computer è attualmente in beta e richiede un [header beta](/docs/it/api/beta-headers):
- `"computer-use-2025-11-24"` per Claude Opus 4.5
- `"computer-use-2025-01-24"` per Claude Sonnet 4.5, Haiku 4.5, Opus 4.1, Sonnet 4, Opus 4, e Sonnet 3.7 ([deprecato](/docs/it/about-claude/model-deprecations))
</Note>

## Panoramica

Il controllo del computer è una funzione beta che consente a Claude di interagire con ambienti desktop. Questo strumento fornisce:

- **Cattura di screenshot**: Vedi cosa è attualmente visualizzato sullo schermo
- **Controllo del mouse**: Fai clic, trascina e sposta il cursore
- **Input da tastiera**: Digita testo e utilizza scorciatoie da tastiera
- **Automazione del desktop**: Interagisci con qualsiasi applicazione o interfaccia

Sebbene il controllo del computer possa essere aumentato con altri strumenti come bash e editor di testo per flussi di lavoro di automazione più completi, il controllo del computer si riferisce specificamente alla capacità dello strumento di controllo del computer di vedere e controllare gli ambienti desktop.

## Compatibilità dei modelli

Il controllo del computer è disponibile per i seguenti modelli Claude:

| Modello | Versione dello strumento | Flag Beta |
|-------|--------------|-----------|
| Claude Opus 4.5 | `computer_20251124` | `computer-use-2025-11-24` |
| Tutti gli altri modelli supportati | `computer_20250124` | `computer-use-2025-01-24` |

<Note>
Claude Opus 4.5 introduce la versione dello strumento `computer_20251124` con nuove capacità inclusa l'azione zoom per l'ispezione dettagliata della regione dello schermo. Tutti gli altri modelli (Sonnet 4.5, Haiku 4.5, Sonnet 4, Opus 4, Opus 4.1, e Sonnet 3.7) utilizzano la versione dello strumento `computer_20250124`.
</Note>

<Warning>
Le versioni precedenti dello strumento non sono garantite essere retrocompatibili con i modelli più recenti. Utilizza sempre la versione dello strumento che corrisponde alla versione del tuo modello.
</Warning>

## Considerazioni sulla sicurezza

<Warning>
Il controllo del computer è una funzione beta con rischi unici distinti dalle funzioni API standard. Questi rischi sono aumentati quando si interagisce con Internet. Per minimizzare i rischi, considera di prendere precauzioni come:

1. Utilizza una macchina virtuale dedicata o un contenitore con privilegi minimi per prevenire attacchi diretti al sistema o incidenti.
2. Evita di dare al modello accesso a dati sensibili, come informazioni di accesso all'account, per prevenire il furto di informazioni.
3. Limita l'accesso a Internet a un elenco consentito di domini per ridurre l'esposizione a contenuti dannosi.
4. Chiedi a un umano di confermare decisioni che potrebbero avere conseguenze significative nel mondo reale, nonché qualsiasi attività che richieda consenso affermativo, come accettare cookie, eseguire transazioni finanziarie o accettare i termini di servizio.

In alcune circostanze, Claude seguirà i comandi trovati nel contenuto anche se entra in conflitto con le istruzioni dell'utente. Ad esempio, le istruzioni Claude su pagine web o contenute in immagini possono ignorare le istruzioni o causare a Claude di fare errori. Suggeriamo di prendere precauzioni per isolare Claude da dati e azioni sensibili per evitare rischi legati all'iniezione di prompt.

Abbiamo addestrato il modello a resistere a queste iniezioni di prompt e abbiamo aggiunto un ulteriore livello di difesa. Se utilizzi i nostri strumenti di controllo del computer, eseguiremo automaticamente classificatori sui tuoi prompt per segnalare potenziali istanze di iniezioni di prompt. Quando questi classificatori identificano potenziali iniezioni di prompt negli screenshot, indirizzeranno automaticamente il modello a chiedere la conferma dell'utente prima di procedere con l'azione successiva. Riconosciamo che questa protezione extra non sarà ideale per ogni caso d'uso (ad esempio, casi d'uso senza un umano nel ciclo), quindi se desideri rinunciare e disattivarla, per favore [contattaci](https://support.claude.com/en/).

Suggeriamo comunque di prendere precauzioni per isolare Claude da dati e azioni sensibili per evitare rischi legati all'iniezione di prompt.

Infine, per favore informa gli utenti finali dei rischi rilevanti e ottieni il loro consenso prima di abilitare il controllo del computer nei tuoi prodotti.

</Warning>

<Card
  title="Implementazione di riferimento del controllo del computer"
  icon="computer"
  href="https://github.com/anthropics/anthropic-quickstarts/tree/main/computer-use-demo"
>

Inizia rapidamente con la nostra implementazione di riferimento del controllo del computer che include un'interfaccia web, un contenitore Docker, implementazioni di strumenti di esempio e un ciclo di agenti.

**Nota:** L'implementazione è stata aggiornata per includere nuovi strumenti sia per i modelli Claude 4 che per Claude Sonnet 3.7. Assicurati di estrarre la versione più recente del repository per accedere a queste nuove funzioni.

</Card>

<Tip>
  Per favore utilizza [questo modulo](https://forms.gle/BT1hpBrqDPDUrCqo7) per fornire
  feedback sulla qualità delle risposte del modello, l'API stessa, o la qualità
  della documentazione - non vediamo l'ora di sentirti!
</Tip>

## Avvio rapido

Ecco come iniziare con il controllo del computer:

<CodeGroup>
```python Python
import anthropic

client = anthropic.Anthropic()

response = client.beta.messages.create(
    model="claude-sonnet-4-5",  # o un altro modello compatibile
    max_tokens=1024,
    tools=[
        {
          "type": "computer_20250124",
          "name": "computer",
          "display_width_px": 1024,
          "display_height_px": 768,
          "display_number": 1,
        },
        {
          "type": "text_editor_20250728",
          "name": "str_replace_based_edit_tool"
        },
        {
          "type": "bash_20250124",
          "name": "bash"
        }
    ],
    messages=[{"role": "user", "content": "Save a picture of a cat to my desktop."}],
    betas=["computer-use-2025-01-24"]
)
print(response)
```

```bash Shell
curl https://api.anthropic.com/v1/messages \
  -H "content-type: application/json" \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "anthropic-beta: computer-use-2025-01-24" \
  -d '{
    "model": "claude-sonnet-4-5",
    "max_tokens": 1024,
    "tools": [
      {
        "type": "computer_20250124",
        "name": "computer",
        "display_width_px": 1024,
        "display_height_px": 768,
        "display_number": 1
      },
      {
        "type": "text_editor_20250728",
        "name": "str_replace_based_edit_tool"
      },
      {
        "type": "bash_20250124",
        "name": "bash"
      }
    ],
    "messages": [
      {
        "role": "user",
        "content": "Save a picture of a cat to my desktop."
      }
    ]
  }'
```
</CodeGroup>

<Note>
Un header beta è richiesto solo per lo strumento di controllo del computer.

L'esempio sopra mostra tutti e tre gli strumenti utilizzati insieme, il che richiede l'header beta perché include lo strumento di controllo del computer.
</Note>

---

## Come funziona il controllo del computer

<Steps>
  <Step
    title="1. Fornisci a Claude lo strumento di controllo del computer e un prompt dell'utente"
    icon="tool"
  >
    - Aggiungi lo strumento di controllo del computer (e facoltativamente altri strumenti) alla tua richiesta API.
    - Includi un prompt dell'utente che richiede l'interazione con il desktop, ad esempio, "Salva un'immagine di un gatto sul mio desktop."
  </Step>
  <Step title="2. Claude decide di utilizzare lo strumento di controllo del computer" icon="wrench">
    - Claude valuta se lo strumento di controllo del computer può aiutare con la query dell'utente.
    - Se sì, Claude costruisce una richiesta di utilizzo dello strumento correttamente formattata.
    - La risposta API ha un `stop_reason` di `tool_use`, segnalando l'intenzione di Claude.
  </Step>
  <Step
    title="3. Estrai l'input dello strumento, valuta lo strumento su un computer e restituisci i risultati"
    icon="computer"
  >
    - Da parte tua, estrai il nome dello strumento e l'input dalla richiesta di Claude.
    - Utilizza lo strumento su un contenitore o una macchina virtuale.
    - Continua la conversazione con un nuovo messaggio `user` contenente un blocco di contenuto `tool_result`.
  </Step>
  <Step
    title="4. Claude continua a chiamare gli strumenti di controllo del computer fino al completamento dell'attività"
    icon="arrows-clockwise"
  >
    - Claude analizza i risultati dello strumento per determinare se è necessario un ulteriore utilizzo dello strumento o se l'attività è stata completata.
    - Se Claude decide che ha bisogno di un altro strumento, risponde con un altro `stop_reason` di `tool_use` e dovresti tornare al passaggio 3.
    - Altrimenti, elabora una risposta di testo all'utente.
  </Step>
</Steps>

Ci riferiamo alla ripetizione dei passaggi 3 e 4 senza input dell'utente come il "ciclo di agenti" - cioè Claude che risponde con una richiesta di utilizzo dello strumento e la tua applicazione che risponde a Claude con i risultati della valutazione di quella richiesta.

### L'ambiente informatico

Il controllo del computer richiede un ambiente informatico sandbox in cui Claude possa interagire in sicurezza con applicazioni e il web. Questo ambiente include:

1. **Display virtuale**: Un server di display X11 virtuale (utilizzando Xvfb) che renderizza l'interfaccia desktop che Claude vedrà attraverso gli screenshot e controllerà con azioni del mouse/tastiera.

2. **Ambiente desktop**: Un'interfaccia UI leggera con window manager (Mutter) e pannello (Tint2) in esecuzione su Linux, che fornisce un'interfaccia grafica coerente per Claude per interagire.

3. **Applicazioni**: Applicazioni Linux preinstallate come Firefox, LibreOffice, editor di testo e gestori di file che Claude può utilizzare per completare le attività.

4. **Implementazioni di strumenti**: Codice di integrazione che traduce le richieste di strumenti astratti di Claude (come "sposta il mouse" o "prendi uno screenshot") in operazioni effettive nell'ambiente virtuale.

5. **Ciclo di agenti**: Un programma che gestisce la comunicazione tra Claude e l'ambiente, inviando le azioni di Claude all'ambiente e restituendo i risultati (screenshot, output dei comandi) a Claude.

Quando utilizzi il controllo del computer, Claude non si connette direttamente a questo ambiente. Invece, la tua applicazione:

1. Riceve le richieste di utilizzo dello strumento di Claude
2. Le traduce in azioni nel tuo ambiente informatico
3. Cattura i risultati (screenshot, output dei comandi, ecc.)
4. Restituisce questi risultati a Claude

Per la sicurezza e l'isolamento, l'implementazione di riferimento esegue tutto questo all'interno di un contenitore Docker con mappature di porta appropriate per visualizzare e interagire con l'ambiente.

---

## Come implementare il controllo del computer

### Inizia con la nostra implementazione di riferimento

Abbiamo costruito un'[implementazione di riferimento](https://github.com/anthropics/anthropic-quickstarts/tree/main/computer-use-demo) che include tutto ciò di cui hai bisogno per iniziare rapidamente con il controllo del computer:

- Un [ambiente containerizzato](https://github.com/anthropics/anthropic-quickstarts/blob/main/computer-use-demo/Dockerfile) adatto al controllo del computer con Claude
- Implementazioni degli [strumenti di controllo del computer](https://github.com/anthropics/anthropic-quickstarts/tree/main/computer-use-demo/computer_use_demo/tools)
- Un [ciclo di agenti](https://github.com/anthropics/anthropic-quickstarts/blob/main/computer-use-demo/computer_use_demo/loop.py) che interagisce con l'API Claude ed esegue gli strumenti di controllo del computer
- Un'interfaccia web per interagire con il contenitore, il ciclo di agenti e gli strumenti.

### Comprendere il ciclo multi-agente

Il nucleo del controllo del computer è il "ciclo di agenti" - un ciclo in cui Claude richiede azioni degli strumenti, la tua applicazione le esegue e restituisce i risultati a Claude. Ecco un esempio semplificato:

```python
async def sampling_loop(
    *,
    model: str,
    messages: list[dict],
    api_key: str,
    max_tokens: int = 4096,
    tool_version: str,
    thinking_budget: int | None = None,
    max_iterations: int = 10,  # Aggiungi limite di iterazione per prevenire cicli infiniti
):
    """
    Un semplice ciclo di agenti per le interazioni di controllo del computer di Claude.

    Questa funzione gestisce il dialogo tra:
    1. Invio di messaggi dell'utente a Claude
    2. Claude che richiede di utilizzare gli strumenti
    3. La tua app che esegue quegli strumenti
    4. Invio dei risultati dello strumento a Claude
    """
    # Configura gli strumenti e i parametri API
    client = Anthropic(api_key=api_key)
    beta_flag = "computer-use-2025-01-24" if "20250124" in tool_version else "computer-use-2024-10-22"

    # Configura gli strumenti - dovresti già averli inizializzati altrove
    tools = [
        {"type": f"computer_{tool_version}", "name": "computer", "display_width_px": 1024, "display_height_px": 768},
        {"type": f"text_editor_{tool_version}", "name": "str_replace_editor"},
        {"type": f"bash_{tool_version}", "name": "bash"}
    ]

    # Ciclo principale di agenti (con limite di iterazione per prevenire costi API incontrollati)
    iterations = 0
    while True and iterations < max_iterations:
        iterations += 1
        # Configura il parametro di thinking opzionale (per Claude Sonnet 3.7)
        thinking = None
        if thinking_budget:
            thinking = {"type": "enabled", "budget_tokens": thinking_budget}

        # Chiama l'API Claude
        response = client.beta.messages.create(
            model=model,
            max_tokens=max_tokens,
            messages=messages,
            tools=tools,
            betas=[beta_flag],
            thinking=thinking
        )

        # Aggiungi la risposta di Claude alla cronologia della conversazione
        response_content = response.content
        messages.append({"role": "assistant", "content": response_content})

        # Controlla se Claude ha utilizzato degli strumenti
        tool_results = []
        for block in response_content:
            if block.type == "tool_use":
                # In un'app reale, eseguiresti lo strumento qui
                # Ad esempio: result = run_tool(block.name, block.input)
                result = {"result": "Tool executed successfully"}

                # Formatta il risultato per Claude
                tool_results.append({
                    "type": "tool_result",
                    "tool_use_id": block.id,
                    "content": result
                })

        # Se nessuno strumento è stato utilizzato, Claude ha finito - restituisci i messaggi finali
        if not tool_results:
            return messages

        # Aggiungi i risultati dello strumento ai messaggi per la prossima iterazione con Claude
        messages.append({"role": "user", "content": tool_results})
```

Il ciclo continua fino a quando Claude non risponde senza richiedere alcuno strumento (completamento dell'attività) o fino al raggiungimento del limite massimo di iterazione. Questo salvaguardia previene potenziali cicli infiniti che potrebbero risultare in costi API inaspettati.

Ti consigliamo di provare l'implementazione di riferimento prima di leggere il resto di questa documentazione.

### Ottimizza le prestazioni del modello con i prompt

Ecco alcuni suggerimenti su come ottenere i migliori output di qualità:

1. Specifica attività semplici e ben definite e fornisci istruzioni esplicite per ogni passaggio.
2. Claude a volte assume i risultati delle sue azioni senza controllare esplicitamente i loro risultati. Per prevenire questo, puoi richiedere a Claude con `After each step, take a screenshot and carefully evaluate if you have achieved the right outcome. Explicitly show your thinking: "I have evaluated step X..." If not correct, try again. Only when you confirm a step was executed correctly should you move on to the next one.`
3. Alcuni elementi dell'interfaccia utente (come menu a discesa e barre di scorrimento) potrebbero essere difficili per Claude da manipolare utilizzando i movimenti del mouse. Se riscontri questo, prova a richiedere al modello di utilizzare scorciatoie da tastiera.
4. Per attività ripetibili o interazioni dell'interfaccia utente, includi screenshot di esempio e chiamate di strumenti di risultati riusciti nel tuo prompt.
5. Se hai bisogno che il modello acceda, forniscigli il nome utente e la password nel tuo prompt all'interno di tag xml come `<robot_credentials>`. L'utilizzo del controllo del computer all'interno di applicazioni che richiedono l'accesso aumenta il rischio di cattivi risultati a causa dell'iniezione di prompt. Per favore rivedi la nostra [guida sulla mitigazione delle iniezioni di prompt](/docs/it/test-and-evaluate/strengthen-guardrails/mitigate-jailbreaks) prima di fornire al modello le credenziali di accesso.

<Tip>
  Se incontri ripetutamente un insieme chiaro di problemi o conosci in anticipo le attività
  che Claude dovrà completare, utilizza il prompt di sistema per fornire a Claude
  suggerimenti espliciti o istruzioni su come completare le attività con successo.
</Tip>

### Prompt di sistema

Quando uno degli strumenti definiti da Anthropic viene richiesto tramite l'API Claude, viene generato un prompt di sistema specifico per il controllo del computer. È simile al [prompt di sistema per l'utilizzo dello strumento](/docs/it/agents-and-tools/tool-use/implement-tool-use#tool-use-system-prompt) ma inizia con:

> You have access to a set of functions you can use to answer the user's question. This includes access to a sandboxed computing environment. You do NOT currently have the ability to inspect files or interact with external resources, except by invoking the below functions.

Come con l'utilizzo regolare dello strumento, il campo `system_prompt` fornito dall'utente è ancora rispettato e utilizzato nella costruzione del prompt di sistema combinato.

### Azioni disponibili

Lo strumento di controllo del computer supporta queste azioni:

**Azioni di base (tutte le versioni)**
- **screenshot** - Cattura il display corrente
- **left_click** - Fai clic alle coordinate `[x, y]`
- **type** - Digita una stringa di testo
- **key** - Premi un tasto o una combinazione di tasti (ad esempio, "ctrl+s")
- **mouse_move** - Sposta il cursore alle coordinate

**Azioni migliorate (`computer_20250124`)**
Disponibili nei modelli Claude 4 e Claude Sonnet 3.7:
- **scroll** - Scorri in qualsiasi direzione con controllo della quantità
- **left_click_drag** - Fai clic e trascina tra le coordinate
- **right_click**, **middle_click** - Pulsanti del mouse aggiuntivi
- **double_click**, **triple_click** - Clic multipli
- **left_mouse_down**, **left_mouse_up** - Controllo fine del clic
- **hold_key** - Tieni premuto un tasto mentre esegui altre azioni
- **wait** - Pausa tra le azioni

**Azioni migliorate (`computer_20251124`)**
Disponibili in Claude Opus 4.5:
- Tutte le azioni da `computer_20250124`
- **zoom** - Visualizza una regione specifica dello schermo a risoluzione completa. Richiede `enable_zoom: true` nella definizione dello strumento. Accetta un parametro `region` con coordinate `[x1, y1, x2, y2]` che definiscono gli angoli in alto a sinistra e in basso a destra dell'area da ispezionare.

<section title="Azioni di esempio">

```json
// Prendi uno screenshot
{
  "action": "screenshot"
}

// Fai clic alla posizione
{
  "action": "left_click",
  "coordinate": [500, 300]
}

// Digita testo
{
  "action": "type",
  "text": "Hello, world!"
}

// Scorri verso il basso (Claude 4/3.7)
{
  "action": "scroll",
  "coordinate": [500, 400],
  "scroll_direction": "down",
  "scroll_amount": 3
}

// Zoom per visualizzare la regione in dettaglio (Opus 4.5)
{
  "action": "zoom",
  "region": [100, 200, 400, 350]
}
```

</section>

### Parametri dello strumento

| Parametro | Obbligatorio | Descrizione |
|-----------|----------|-------------|
| `type` | Sì | Versione dello strumento (`computer_20251124`, `computer_20250124`, o `computer_20241022`) |
| `name` | Sì | Deve essere "computer" |
| `display_width_px` | Sì | Larghezza del display in pixel |
| `display_height_px` | Sì | Altezza del display in pixel |
| `display_number` | No | Numero di display per ambienti X11 |
| `enable_zoom` | No | Abilita l'azione zoom (`computer_20251124` solo). Imposta su `true` per consentire a Claude di ingrandire regioni specifiche dello schermo. Predefinito: `false` |

<Note>
**Importante**: Lo strumento di controllo del computer deve essere eseguito esplicitamente dalla tua applicazione - Claude non può eseguirlo direttamente. Sei responsabile dell'implementazione della cattura dello screenshot, dei movimenti del mouse, degli input da tastiera e di altre azioni in base alle richieste di Claude.
</Note>

### Abilita la capacità di thinking nei modelli Claude 4 e Claude Sonnet 3.7

Claude Sonnet 3.7 ha introdotto una nuova capacità di "thinking" che ti consente di vedere il processo di ragionamento del modello mentre lavora su attività complesse. Questa funzione ti aiuta a capire come Claude sta affrontando un problema e può essere particolarmente preziosa per il debug o scopi educativi.

Per abilitare il thinking, aggiungi un parametro `thinking` alla tua richiesta API:

```json
"thinking": {
  "type": "enabled",
  "budget_tokens": 1024
}
```

Il parametro `budget_tokens` specifica quanti token Claude può utilizzare per il thinking. Questo viene sottratto dal tuo budget complessivo di `max_tokens`.

Quando il thinking è abilitato, Claude restituirà il suo processo di ragionamento come parte della risposta, che può aiutarti a:

1. Comprendere il processo decisionale del modello
2. Identificare potenziali problemi o malintesi
3. Imparare dall'approccio di Claude alla risoluzione dei problemi
4. Ottenere maggiore visibilità nelle operazioni multi-step complesse

Ecco un esempio di come potrebbe apparire l'output del thinking:

```
[Thinking]
Devo salvare un'immagine di un gatto sul desktop. Dividiamo questo in passaggi:

1. Per prima cosa, prenderò uno screenshot per vedere cosa c'è sul desktop
2. Poi cercherò un browser web per cercare immagini di gatti
3. Dopo aver trovato un'immagine adatta, dovrò salvarla sul desktop

Iniziamo prendendo uno screenshot per vedere cosa è disponibile...
```

### Aumentare il controllo del computer con altri strumenti

Lo strumento di controllo del computer può essere combinato con altri strumenti per creare flussi di lavoro di automazione più potenti. Questo è particolarmente utile quando hai bisogno di:
- Eseguire comandi di sistema ([strumento bash](/docs/it/agents-and-tools/tool-use/bash-tool))
- Modificare file di configurazione o script ([strumento editor di testo](/docs/it/agents-and-tools/tool-use/text-editor-tool))
- Integrarsi con API personalizzate o servizi (strumenti personalizzati)

<CodeGroup>
  ```bash Shell
  curl https://api.anthropic.com/v1/messages \
    -H "content-type: application/json" \
    -H "x-api-key: $ANTHROPIC_API_KEY" \
    -H "anthropic-version: 2023-06-01" \
    -H "anthropic-beta: computer-use-2025-01-24" \
    -d '{
      "model": "claude-sonnet-4-5",
      "max_tokens": 2000,
      "tools": [
        {
          "type": "computer_20250124",
          "name": "computer",
          "display_width_px": 1024,
          "display_height_px": 768,
          "display_number": 1
        },
        {
          "type": "text_editor_20250728",
          "name": "str_replace_based_edit_tool"
        },
        {
          "type": "bash_20250124",
          "name": "bash"
        },
        {
          "name": "get_weather",
          "description": "Get the current weather in a given location",
          "input_schema": {
            "type": "object",
            "properties": {
              "location": {
                "type": "string",
                "description": "The city and state, e.g. San Francisco, CA"
              },
              "unit": {
                "type": "string",
                "enum": ["celsius", "fahrenheit"],
                "description": "The unit of temperature, either 'celsius' or 'fahrenheit'"
              }
            },
            "required": ["location"]
          }
        }
      ],
      "messages": [
        {
          "role": "user",
          "content": "Find flights from San Francisco to a place with warmer weather."
        }
      ],
      "thinking": {
        "type": "enabled",
        "budget_tokens": 1024
      }
    }'
  ```

```python Python
import anthropic

client = anthropic.Anthropic()

response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    tools=[
        {
          "type": "computer_20250124",
          "name": "computer",
          "display_width_px": 1024,
          "display_height_px": 768,
          "display_number": 1,
        },
        {
          "type": "text_editor_20250728",
          "name": "str_replace_based_edit_tool"
        },
        {
          "type": "bash_20250124",
          "name": "bash"
        },
        {
          "name": "get_weather",
          "description": "Get the current weather in a given location",
          "input_schema": {
            "type": "object",
            "properties": {
              "location": {
                "type": "string",
                "description": "The city and state, e.g. San Francisco, CA"
              },
              "unit": {
                "type": "string",
                "enum": ["celsius", "fahrenheit"],
                "description": "The unit of temperature, either 'celsius' or 'fahrenheit'"
              }
            },
            "required": ["location"]
          }
        },
    ],
    messages=[{"role": "user", "content": "Find flights from San Francisco to a place with warmer weather."}],
    betas=["computer-use-2025-01-24"],
    thinking={"type": "enabled", "budget_tokens": 1024},
)
print(response)
```

```typescript TypeScript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

const message = await anthropic.beta.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  tools: [
      {
        type: "computer_20250124",
        name: "computer",
        display_width_px: 1024,
        display_height_px: 768,
        display_number: 1,
      },
      {
        type: "text_editor_20250728",
        name: "str_replace_based_edit_tool"
      },
      {
        type: "bash_20250124",
        name: "bash"
      },
      {
        name: "get_weather",
        description: "Get the current weather in a given location",
        input_schema: {
          type: "object",
          properties: {
            location: {
              type: "string",
              description: "The city and state, e.g. San Francisco, CA"
            },
            unit: {
              type: "string",
              enum: ["celsius", "fahrenheit"],
              description: "The unit of temperature, either 'celsius' or 'fahrenheit'"
            }
          },
          required: ["location"]
        }
      },
  ],
  messages: [{ role: "user", content: "Find flights from San Francisco to a place with warmer weather." }],
  betas: ["computer-use-2025-01-24"],
  thinking: { type: "enabled", budget_tokens: 1024 },
});
console.log(message);
```
```java Java
import java.util.List;
import java.util.Map;

import com.anthropic.client.AnthropicClient;
import com.anthropic.client.okhttp.AnthropicOkHttpClient;
import com.anthropic.core.JsonValue;
import com.anthropic.models.beta.messages.BetaMessage;
import com.anthropic.models.beta.messages.MessageCreateParams;
import com.anthropic.models.beta.messages.BetaToolBash20250124;
import com.anthropic.models.beta.messages.BetaToolComputerUse20250124;
import com.anthropic.models.beta.messages.BetaToolTextEditor20250124;
import com.anthropic.models.beta.messages.BetaThinkingConfigEnabled;
import com.anthropic.models.beta.messages.BetaThinkingConfigParam;
import com.anthropic.models.beta.messages.BetaTool;

public class MultipleToolsExample {

    public static void main(String[] args) {
        AnthropicClient client = AnthropicOkHttpClient.fromEnv();

        MessageCreateParams params = MessageCreateParams.builder()
                .model("claude-sonnet-4-5")
                .maxTokens(1024)
                .addTool(BetaToolComputerUse20250124.builder()
                        .displayWidthPx(1024)
                        .displayHeightPx(768)
                        .displayNumber(1)
                        .build())
                .addTool(BetaToolTextEditor20250124.builder()
                        .build())
                .addTool(BetaToolBash20250124.builder()
                        .build())
                .addTool(BetaTool.builder()
                        .name("get_weather")
                        .description("Get the current weather in a given location")
                        .inputSchema(BetaTool.InputSchema.builder()
                                .properties(
                                        JsonValue.from(
                                                Map.of(
                                                        "location", Map.of(
                                                                "type", "string",
                                                                "description", "The city and state, e.g. San Francisco, CA"
                                                        ),
                                                        "unit", Map.of(
                                                                "type", "string",
                                                                "enum", List.of("celsius", "fahrenheit"),
                                                                "description", "The unit of temperature, either 'celsius' or 'fahrenheit'"
                                                        )
                                                )
                                        ))
                                .build()
                        )
                        .build())
                .thinking(BetaThinkingConfigParam.ofEnabled(
                        BetaThinkingConfigEnabled.builder()
                                .budgetTokens(1024)
                                .build()
                ))
                .addUserMessage("Find flights from San Francisco to a place with warmer weather.")
                .addBeta("computer-use-2025-01-24")
                .build();

        BetaMessage message = client.beta().messages().create(params);
        System.out.println(message);
    }
}
```
</CodeGroup>

### Costruisci un ambiente di computer use personalizzato

L'[implementazione di riferimento](https://github.com/anthropics/anthropic-quickstarts/tree/main/computer-use-demo) è pensata per aiutarti a iniziare con il computer use. Include tutti i componenti necessari affinché Claude utilizzi un computer. Tuttavia, puoi costruire il tuo ambiente per il computer use in base alle tue esigenze. Avrai bisogno di:

- Un ambiente virtualizzato o containerizzato adatto al computer use con Claude
- Un'implementazione di almeno uno degli strumenti di computer use definiti da Anthropic
- Un agent loop che interagisce con l'API Claude ed esegue i risultati di `tool_use` utilizzando le tue implementazioni di strumenti
- Un'API o un'interfaccia utente che consenta l'input dell'utente per avviare l'agent loop

#### Implementa lo strumento di computer use

Lo strumento di computer use è implementato come uno strumento senza schema. Quando utilizzi questo strumento, non è necessario fornire uno schema di input come con altri strumenti; lo schema è integrato nel modello di Claude e non può essere modificato.

<Steps>
  <Step title="Configura il tuo ambiente di calcolo">
    Crea un display virtuale o connettiti a un display esistente con cui Claude interagirà. Questo in genere comporta la configurazione di Xvfb (X Virtual Framebuffer) o tecnologia simile.
  </Step>
  <Step title="Implementa i gestori di azioni">
    Crea funzioni per gestire ogni tipo di azione che Claude potrebbe richiedere:
    ```python
    def handle_computer_action(action_type, params):
        if action_type == "screenshot":
            return capture_screenshot()
        elif action_type == "left_click":
            x, y = params["coordinate"]
            return click_at(x, y)
        elif action_type == "type":
            return type_text(params["text"])
        # ... gestisci altre azioni
    ```
  </Step>
  <Step title="Elabora le chiamate di strumenti di Claude">
    Estrai ed esegui le chiamate di strumenti dalle risposte di Claude:
    ```python
    for content in response.content:
        if content.type == "tool_use":
            action = content.input["action"]
            result = handle_computer_action(action, content.input)
            
            # Restituisci il risultato a Claude
            tool_result = {
                "type": "tool_result",
                "tool_use_id": content.id,
                "content": result
            }
    ```
  </Step>
  <Step title="Implementa l'agent loop">
    Crea un loop che continua fino a quando Claude completa l'attività:
    ```python
    while True:
        response = client.beta.messages.create(...)
        
        # Controlla se Claude ha utilizzato strumenti
        tool_results = process_tool_calls(response)
        
        if not tool_results:
            # Nessun altro tool use, attività completata
            break
            
        # Continua la conversazione con i risultati degli strumenti
        messages.append({"role": "user", "content": tool_results})
    ```
  </Step>
</Steps>

#### Gestisci gli errori

Quando implementi lo strumento di computer use, possono verificarsi vari errori. Ecco come gestirli:

<section title="Errore di acquisizione dello screenshot">

Se l'acquisizione dello screenshot non riesce, restituisci un messaggio di errore appropriato:

```json
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": "Error: Failed to capture screenshot. Display may be locked or unavailable.",
      "is_error": true
    }
  ]
}
```

</section>

<section title="Coordinate non valide">

Se Claude fornisce coordinate al di fuori dei limiti del display:

```json
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": "Error: Coordinates (1200, 900) are outside display bounds (1024x768).",
      "is_error": true
    }
  ]
}
```

</section>

<section title="Errore di esecuzione dell'azione">

Se un'azione non riesce a essere eseguita:

```json
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01A09q90qw90lq917835lq9",
      "content": "Error: Failed to perform click action. The application may be unresponsive.",
      "is_error": true
    }
  ]
}
```

</section>

#### Gestisci il ridimensionamento delle coordinate per risoluzioni più elevate

L'API vincola le immagini a un massimo di 1568 pixel sul lato più lungo e circa 1,15 megapixel totali (vedi [ridimensionamento delle immagini](/docs/it/build-with-claude/vision#evaluate-image-size) per i dettagli). Ad esempio, uno schermo 1512x982 viene sottocampionato a circa 1330x864. Claude analizza questa immagine più piccola e restituisce coordinate in quello spazio, ma il tuo strumento esegue i clic nello spazio dello schermo originale.

Questo può causare il mancato raggiungimento dei target da parte delle coordinate di clic di Claude a meno che tu non gestisca la trasformazione delle coordinate.

Per risolvere questo problema, ridimensiona gli screenshot tu stesso e ridimensiona le coordinate di Claude:

<CodeGroup>
```python Python
import math

def get_scale_factor(width, height):
    """Calcola il fattore di scala per soddisfare i vincoli dell'API."""
    long_edge = max(width, height)
    total_pixels = width * height

    long_edge_scale = 1568 / long_edge
    total_pixels_scale = math.sqrt(1_150_000 / total_pixels)

    return min(1.0, long_edge_scale, total_pixels_scale)

# Quando acquisisci lo screenshot
scale = get_scale_factor(screen_width, screen_height)
scaled_width = int(screen_width * scale)
scaled_height = int(screen_height * scale)

# Ridimensiona l'immagine alle dimensioni ridimensionate prima di inviarla a Claude
screenshot = capture_and_resize(scaled_width, scaled_height)

# Quando gestisci le coordinate di Claude, ridimensionale
def execute_click(x, y):
    screen_x = x / scale
    screen_y = y / scale
    perform_click(screen_x, screen_y)
```

```typescript TypeScript
const MAX_LONG_EDGE = 1568;
const MAX_PIXELS = 1_150_000;

function getScaleFactor(width: number, height: number): number {
  const longEdge = Math.max(width, height);
  const totalPixels = width * height;

  const longEdgeScale = MAX_LONG_EDGE / longEdge;
  const totalPixelsScale = Math.sqrt(MAX_PIXELS / totalPixels);

  return Math.min(1.0, longEdgeScale, totalPixelsScale);
}

// Quando acquisisci lo screenshot
const scale = getScaleFactor(screenWidth, screenHeight);
const scaledWidth = Math.floor(screenWidth * scale);
const scaledHeight = Math.floor(screenHeight * scale);

// Ridimensiona l'immagine alle dimensioni ridimensionate prima di inviarla a Claude
const screenshot = captureAndResize(scaledWidth, scaledHeight);

// Quando gestisci le coordinate di Claude, ridimensionale
function executeClick(x: number, y: number): void {
  const screenX = x / scale;
  const screenY = y / scale;
  performClick(screenX, screenY);
}
```
</CodeGroup>

#### Segui le best practice di implementazione

<section title="Utilizza una risoluzione di display appropriata">

Imposta le dimensioni del display che corrispondono al tuo caso d'uso mantenendoti entro i limiti consigliati:
- Per attività desktop generali: 1024x768 o 1280x720
- Per applicazioni web: 1280x800 o 1366x768
- Evita risoluzioni superiori a 1920x1080 per prevenire problemi di prestazioni

</section>

<section title="Implementa la gestione corretta degli screenshot">

Quando restituisci screenshot a Claude:
- Codifica gli screenshot come PNG o JPEG in base64
- Considera la compressione di screenshot di grandi dimensioni per migliorare le prestazioni
- Includi metadati rilevanti come timestamp o stato del display
- Se utilizzi risoluzioni più elevate, assicurati che le coordinate siano accuratamente ridimensionate

</section>

<section title="Aggiungi ritardi di azione">

Alcune applicazioni hanno bisogno di tempo per rispondere alle azioni:
```python
def click_and_wait(x, y, wait_time=0.5):
    click_at(x, y)
    time.sleep(wait_time)  # Consenti all'interfaccia utente di aggiornarsi
```

</section>

<section title="Convalida le azioni prima dell'esecuzione">

Controlla che le azioni richieste siano sicure e valide:
```python
def validate_action(action_type, params):
    if action_type == "left_click":
        x, y = params.get("coordinate", (0, 0))
        if not (0 <= x < display_width and 0 <= y < display_height):
            return False, "Coordinates out of bounds"
    return True, None
```

</section>

<section title="Registra le azioni per il debug">

Mantieni un registro di tutte le azioni per la risoluzione dei problemi:
```python
import logging

def log_action(action_type, params, result):
    logging.info(f"Action: {action_type}, Params: {params}, Result: {result}")
```

</section>

---

## Comprendi le limitazioni del computer use

La funzionalità di computer use è in beta. Sebbene le capacità di Claude siano all'avanguardia, gli sviluppatori dovrebbero essere consapevoli delle sue limitazioni:

1. **Latenza**: la latenza attuale del computer use per le interazioni uomo-IA potrebbe essere troppo lenta rispetto alle normali azioni di computer dirette dall'uomo. Consigliamo di concentrarsi su casi d'uso in cui la velocità non è critica (ad es. raccolta di informazioni di background, test automatizzato del software) in ambienti affidabili.
2. **Accuratezza e affidabilità della visione artificiale**: Claude potrebbe fare errori o allucinare quando genera coordinate specifiche durante la generazione di azioni. Claude Sonnet 3.7 introduce la capacità di thinking che può aiutarti a comprendere il ragionamento del modello e identificare potenziali problemi.
3. **Accuratezza e affidabilità della selezione degli strumenti**: Claude potrebbe fare errori o allucinare quando seleziona strumenti durante la generazione di azioni o intraprendere azioni inaspettate per risolvere i problemi. Inoltre, l'affidabilità potrebbe essere inferiore quando si interagisce con applicazioni di nicchia o più applicazioni contemporaneamente. Consigliamo agli utenti di richiedere attentamente il modello quando si richiedono attività complesse.
4. **Affidabilità dello scorrimento**: Claude Sonnet 3.7 ha introdotto azioni di scorrimento dedicate con controllo della direzione che migliora l'affidabilità. Il modello può ora scorrere esplicitamente in qualsiasi direzione (su/giù/sinistra/destra) di un importo specificato.
5. **Interazione con fogli di calcolo**: I clic del mouse per l'interazione con fogli di calcolo sono migliorati in Claude Sonnet 3.7 con l'aggiunta di azioni di controllo del mouse più precise come `left_mouse_down`, `left_mouse_up` e nuovo supporto dei tasti modificatori. La selezione delle celle può essere più affidabile utilizzando questi controlli granulari e combinando i tasti modificatori con i clic.
6. **Creazione di account e generazione di contenuti su piattaforme di social media e comunicazioni**: Sebbene Claude visiterà i siti web, stiamo limitando la sua capacità di creare account o generare e condividere contenuti o altrimenti impersonare persone su siti web e piattaforme di social media. Potremmo aggiornare questa capacità in futuro.
7. **Vulnerabilità**: Vulnerabilità come jailbreaking o prompt injection possono persistere nei sistemi di IA frontier, inclusa l'API beta di computer use. In alcune circostanze, Claude seguirà i comandi trovati nei contenuti, a volte anche in conflitto con le istruzioni dell'utente. Ad esempio, le istruzioni di Claude su pagine web o contenute in immagini possono ignorare le istruzioni o causare errori di Claude. Consigliamo:
   a. Limitare il computer use ad ambienti affidabili come macchine virtuali o container con privilegi minimi
   b. Evitare di dare accesso al computer use ad account o dati sensibili senza una supervisione rigorosa
   c. Informare gli utenti finali dei rischi rilevanti e ottenere il loro consenso prima di abilitare o richiedere autorizzazioni necessarie per le funzioni di computer use nelle tue applicazioni
8. **Azioni inappropriate o illegittime**: Secondo i termini di servizio di Anthropic, non devi utilizzare il computer use per violare leggi o la nostra Acceptable Use Policy.

Rivedi e verifica sempre attentamente le azioni e i registri del computer use di Claude. Non utilizzare Claude per attività che richiedono una precisione perfetta o informazioni sensibili dell'utente senza supervisione umana.

---

## Prezzi

Computer use follows the standard [tool use pricing](/docs/en/agents-and-tools/tool-use/overview#pricing). When using the computer use tool:

**System prompt overhead**: The computer use beta adds 466-499 tokens to the system prompt

**Computer use tool token usage**:
| Model | Input tokens per tool definition |
| ----- | -------------------------------- |
| Claude 4.x models | 735 tokens |
| Claude Sonnet 3.7 ([deprecated](/docs/en/about-claude/model-deprecations)) | 735 tokens |

**Additional token consumption**:
- Screenshot images (see [Vision pricing](/docs/en/build-with-claude/vision))
- Tool execution results returned to Claude

<Note>
If you're also using bash or text editor tools alongside computer use, those tools have their own token costs as documented in their respective pages.
</Note>

## Passaggi successivi

<CardGroup cols={2}>
  <Card
    title="Implementazione di riferimento"
    icon="github-logo"
    href="https://github.com/anthropics/anthropic-quickstarts/tree/main/computer-use-demo"
  >
    Inizia rapidamente con la nostra implementazione completa basata su Docker
  </Card>
  <Card
    title="Documentazione dello strumento"
    icon="tool"
    href="/docs/it/agents-and-tools/tool-use/overview"
  >
    Scopri di più sul tool use e sulla creazione di strumenti personalizzati
  </Card>
</CardGroup>