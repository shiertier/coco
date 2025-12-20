# Hosting dell'Agent SDK

Distribuisci e ospita Claude Agent SDK in ambienti di produzione

---

Claude Agent SDK differisce dalle API LLM tradizionali senza stato in quanto mantiene lo stato conversazionale ed esegue comandi in un ambiente persistente. Questa guida copre l'architettura, le considerazioni di hosting e le best practice per la distribuzione di agenti basati su SDK in produzione.

<Info>
Per l'hardening della sicurezza oltre il sandboxing di base—inclusi i controlli di rete, la gestione delle credenziali e le opzioni di isolamento—vedi [Distribuzione Sicura](/docs/it/agent-sdk/secure-deployment).
</Info>

## Requisiti di Hosting

### Sandboxing Basato su Container

Per la sicurezza e l'isolamento, l'SDK dovrebbe essere eseguito all'interno di un ambiente container sandboxato. Questo fornisce isolamento dei processi, limiti di risorse, controllo di rete e filesystem effimeri.

L'SDK supporta anche la [configurazione sandbox programmatica](/docs/it/agent-sdk/typescript#sandbox-settings) per l'esecuzione dei comandi.

### Requisiti di Sistema

Ogni istanza SDK richiede:

- **Dipendenze di runtime**
  - Python 3.10+ (per Python SDK) o Node.js 18+ (per TypeScript SDK)
  - Node.js (richiesto da Claude Code CLI)
  - Claude Code CLI: `npm install -g @anthropic-ai/claude-code`

- **Allocazione di risorse**
  - Consigliato: 1GiB RAM, 5GiB di disco e 1 CPU (varia questo in base al tuo compito secondo necessità)

- **Accesso di rete**
  - HTTPS in uscita a `api.anthropic.com`
  - Facoltativo: Accesso ai server MCP o strumenti esterni

## Comprensione dell'Architettura dell'SDK

A differenza delle chiamate API senza stato, Claude Agent SDK opera come un **processo a lunga esecuzione** che:
- **Esegue comandi** in un ambiente shell persistente
- **Gestisce operazioni su file** all'interno di una directory di lavoro
- **Gestisce l'esecuzione di strumenti** con contesto dalle interazioni precedenti

## Opzioni di Provider Sandbox

Diversi provider si specializzano in ambienti container sicuri per l'esecuzione di codice AI:

- **[Cloudflare Sandboxes](https://github.com/cloudflare/sandbox-sdk)**
- **[Modal Sandboxes](https://modal.com/docs/guide/sandbox)**
- **[Daytona](https://www.daytona.io/)**
- **[E2B](https://e2b.dev/)**
- **[Fly Machines](https://fly.io/docs/machines/)**
- **[Vercel Sandbox](https://vercel.com/docs/functions/sandbox)**

Per opzioni self-hosted (Docker, gVisor, Firecracker) e configurazione dettagliata dell'isolamento, vedi [Tecnologie di Isolamento](/docs/it/agent-sdk/secure-deployment#isolation-technologies).

## Pattern di Distribuzione in Produzione

### Pattern 1: Sessioni Effimere

Crea un nuovo container per ogni compito dell'utente, quindi distruggilo al completamento.

Ideale per compiti una tantum, l'utente può ancora interagire con l'AI mentre il compito è in corso di completamento, ma una volta completato il container viene distrutto.

**Esempi:**
- Investigazione e Correzione di Bug: Esegui il debug e risolvi un problema specifico con il contesto rilevante
- Elaborazione di Fatture: Estrai e struttura i dati da ricevute/fatture per i sistemi contabili
- Compiti di Traduzione: Traduci documenti o batch di contenuti tra lingue
- Elaborazione di Immagini/Video: Applica trasformazioni, ottimizzazioni o estrai metadati da file multimediali

### Pattern 2: Sessioni a Lunga Esecuzione

Mantieni istanze di container persistenti per compiti a lunga esecuzione. Spesso esegui _molteplici_ processi Claude Agent all'interno del container in base alla domanda.

Ideale per agenti proattivi che agiscono senza l'input dell'utente, agenti che servono contenuti o agenti che elaborano grandi quantità di messaggi.

**Esempi:**
- Email Agent: Monitora i messaggi di posta in arrivo e autonomamente li smista, risponde o intraprende azioni in base al contenuto
- Site Builder: Ospita siti web personalizzati per utente con capacità di editing dal vivo servite attraverso le porte del container
- Chat Bot ad Alta Frequenza: Gestisce flussi di messaggi continui da piattaforme come Slack dove i tempi di risposta rapidi sono critici

### Pattern 3: Sessioni Ibride

Container effimeri che vengono idratati con cronologia e stato, possibilmente da un database o dalle funzionalità di ripresa della sessione dell'SDK.

Ideale per container con interazione intermittente dall'utente che avvia il lavoro e si spegne quando il lavoro è completato ma può essere continuato.

**Esempi:**
- Personal Project Manager: Aiuta a gestire progetti in corso con check-in intermittenti, mantiene il contesto di compiti, decisioni e progressi
- Deep Research: Conduce compiti di ricerca di più ore, salva i risultati e riprende l'investigazione quando l'utente ritorna
- Customer Support Agent: Gestisce ticket di supporto che si estendono su più interazioni, carica la cronologia dei ticket e il contesto del cliente

### Pattern 4: Container Singoli

Esegui molteplici processi Claude Agent SDK in un container globale.

Ideale per agenti che devono collaborare strettamente insieme. Questo è probabilmente il pattern meno popolare perché dovrai impedire agli agenti di sovrascrivere l'uno l'altro.

**Esempi:**
- **Simulazioni**: Agenti che interagiscono tra loro in simulazioni come videogiochi.

# FAQ

### Come comunico con i miei sandbox?
Quando ospiti in container, esponi le porte per comunicare con le tue istanze SDK. La tua applicazione può esporre endpoint HTTP/WebSocket per i client esterni mentre l'SDK viene eseguito internamente all'interno del container.

### Qual è il costo dell'hosting di un container?
Abbiamo scoperto che il costo dominante della gestione degli agenti è il token, i container variano in base a quello che fornisci ma un costo minimo è approssimativamente 5 centesimi all'ora di esecuzione.

### Quando dovrei spegnere i container inattivi rispetto a mantenerli caldi?
Questo è probabilmente dipendente dal provider, diversi provider sandbox ti permetteranno di impostare diversi criteri per i timeout di inattività dopo i quali un sandbox potrebbe spegnersi.
Vorrai sintonizzare questo timeout in base a quanto frequente pensi che la risposta dell'utente potrebbe essere.

### Con quale frequenza dovrei aggiornare Claude Code CLI?
Claude Code CLI è versionato con semver, quindi eventuali modifiche di rottura saranno versionate.

### Come monitoro la salute del container e le prestazioni dell'agente?
Poiché i container sono solo server, la stessa infrastruttura di logging che usi per il backend funzionerà per i container.

### Quanto tempo può durare una sessione di agente prima del timeout?
Una sessione di agente non avrà timeout, ma consigliamo di impostare una proprietà 'maxTurns' per impedire a Claude di rimanere bloccato in un loop.

## Passaggi Successivi

- [Distribuzione Sicura](/docs/it/agent-sdk/secure-deployment) - Controlli di rete, gestione delle credenziali e hardening dell'isolamento
- [TypeScript SDK - Impostazioni Sandbox](/docs/it/agent-sdk/typescript#sandbox-settings) - Configura il sandbox programmaticamente
- [Guida alle Sessioni](/docs/it/agent-sdk/sessions) - Scopri la gestione delle sessioni
- [Autorizzazioni](/docs/it/agent-sdk/permissions) - Configura le autorizzazioni degli strumenti
- [Tracciamento dei Costi](/docs/it/agent-sdk/cost-tracking) - Monitora l'utilizzo dell'API
- [Integrazione MCP](/docs/it/agent-sdk/mcp) - Estendi con strumenti personalizzati