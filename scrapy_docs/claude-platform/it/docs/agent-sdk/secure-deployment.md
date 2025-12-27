# Distribuzione sicura di agenti AI

Una guida per proteggere le distribuzioni di Claude Code e Agent SDK con isolamento, gestione delle credenziali e controlli di rete

---

Claude Code e l'Agent SDK sono strumenti potenti che possono eseguire codice, accedere a file e interagire con servizi esterni per tuo conto. Come qualsiasi strumento con queste capacità, distribuirli con attenzione garantisce che tu ottenga i vantaggi mantenendo i controlli appropriati.

A differenza del software tradizionale che segue percorsi di codice predeterminati, questi strumenti generano le loro azioni dinamicamente in base al contesto e agli obiettivi. Questa flessibilità è ciò che li rende utili, ma significa anche che il loro comportamento può essere influenzato dal contenuto che elaborano: file, pagine web o input dell'utente. Questo è talvolta chiamato prompt injection. Ad esempio, se il README di un repository contiene istruzioni insolite, Claude Code potrebbe incorporarle nelle sue azioni in modi che l'operatore non aveva anticipato. Questa guida copre modi pratici per ridurre questo rischio.

La buona notizia è che proteggere una distribuzione di agenti non richiede infrastrutture esotiche. Gli stessi principi che si applicano all'esecuzione di qualsiasi codice semi-affidabile si applicano qui: isolamento, privilegio minimo e difesa in profondità. Claude Code include diverse funzioni di sicurezza che aiutano con le preoccupazioni comuni, e questa guida le illustra insieme a opzioni di hardening aggiuntive per coloro che ne hanno bisogno.

Non ogni distribuzione ha bisogno della massima sicurezza. Uno sviluppatore che esegue Claude Code sul suo laptop ha requisiti diversi da un'azienda che elabora dati dei clienti in un ambiente multi-tenant. Questa guida presenta opzioni che vanno dalle funzioni di sicurezza integrate di Claude Code alle architetture di produzione indurizzate, così puoi scegliere ciò che si adatta alla tua situazione.

## Cosa stiamo proteggendo?

Gli agenti possono intraprendere azioni indesiderate a causa di prompt injection (istruzioni incorporate nel contenuto che elaborano) o errore del modello. I modelli Claude sono progettati per resistere a questo, e come abbiamo analizzato nella nostra [model card](https://assets.anthropic.com/m/64823ba7485345a7/Claude-Opus-4-5-System-Card.pdf), crediamo che Claude Opus 4.5 sia il modello frontier più robusto disponibile.

La difesa in profondità è comunque una buona pratica. Ad esempio, se un agente elabora un file dannoso che gli istruisce di inviare dati dei clienti a un server esterno, i controlli di rete possono bloccare completamente quella richiesta.

## Funzioni di sicurezza integrate

Claude Code include diverse funzioni di sicurezza che affrontano le preoccupazioni comuni. Vedi la [documentazione sulla sicurezza](https://code.claude.com/docs/en/security) per i dettagli completi.

- **Sistema di autorizzazioni**: Ogni strumento e comando bash può essere configurato per consentire, bloccare o richiedere l'approvazione dell'utente. Usa i pattern glob per creare regole come "consenti tutti i comandi npm" o "blocca qualsiasi comando con sudo". Le organizzazioni possono impostare politiche che si applicano a tutti gli utenti. Vedi [controllo di accesso e autorizzazioni](https://code.claude.com/docs/en/iam#access-control-and-permissions).
- **Analisi statica**: Prima di eseguire i comandi bash, Claude Code esegue un'analisi statica per identificare operazioni potenzialmente rischiose. I comandi che modificano i file di sistema o accedono a directory sensibili vengono contrassegnati e richiedono l'approvazione esplicita dell'utente.
- **Riepilogo della ricerca web**: I risultati della ricerca vengono riepilogati piuttosto che passare il contenuto grezzo direttamente nel contesto, riducendo il rischio di prompt injection da contenuto web dannoso.
- **Modalità sandbox**: I comandi bash possono essere eseguiti in un ambiente sandbox che limita l'accesso al filesystem e alla rete. Vedi la [documentazione del sandboxing](https://code.claude.com/docs/en/sandboxing) per i dettagli.

## Principi di sicurezza

Per le distribuzioni che richiedono hardening aggiuntivo oltre i valori predefiniti di Claude Code, questi principi guidano le opzioni disponibili.

### Confini di sicurezza

Un confine di sicurezza separa i componenti con diversi livelli di fiducia. Per le distribuzioni ad alta sicurezza, puoi posizionare le risorse sensibili (come le credenziali) al di fuori del confine che contiene l'agente. Se qualcosa va male nell'ambiente dell'agente, le risorse al di fuori di quel confine rimangono protette.

Ad esempio, piuttosto che dare a un agente accesso diretto a una chiave API, potresti eseguire un proxy al di fuori dell'ambiente dell'agente che inietta la chiave nelle richieste. L'agente può effettuare chiamate API, ma non vede mai la credenziale stessa. Questo pattern è utile per le distribuzioni multi-tenant o quando si elabora contenuto non affidabile.

### Privilegio minimo

Quando necessario, puoi limitare l'agente solo alle capacità richieste per il suo compito specifico:

| Risorsa | Opzioni di restrizione |
|---------|------------------------|
| Filesystem | Montare solo le directory necessarie, preferire la sola lettura |
| Rete | Limitare a endpoint specifici tramite proxy |
| Credenziali | Iniettare tramite proxy piuttosto che esporre direttamente |
| Capacità di sistema | Eliminare le capacità Linux nei container |

### Difesa in profondità

Per gli ambienti ad alta sicurezza, stratificare più controlli fornisce protezione aggiuntiva. Le opzioni includono:

- Isolamento del container
- Restrizioni di rete
- Controlli del filesystem
- Convalida delle richieste a un proxy

La giusta combinazione dipende dal tuo modello di minaccia e dai requisiti operativi.

## Tecnologie di isolamento

Diverse tecnologie di isolamento offrono diversi compromessi tra forza di sicurezza, prestazioni e complessità operativa.

<Info>
In tutte queste configurazioni, Claude Code (o la tua applicazione Agent SDK) viene eseguito all'interno del confine di isolamento—la sandbox, il container o la VM. I controlli di sicurezza descritti di seguito limitano ciò che l'agente può accedere da dentro quel confine.
</Info>

| Tecnologia | Forza di isolamento | Overhead di prestazioni | Complessità |
|------------|-------------------|------------------------|------------|
| Runtime sandbox | Buona (valori predefiniti sicuri) | Molto basso | Basso |
| Container (Docker) | Dipende dalla configurazione | Basso | Medio |
| gVisor | Eccellente (con configurazione corretta) | Medio/Alto | Medio |
| VM (Firecracker, QEMU) | Eccellente (con configurazione corretta) | Alto | Medio/Alto |

### Runtime sandbox

Per l'isolamento leggero senza container, [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime) applica restrizioni di filesystem e rete a livello del sistema operativo.

Il vantaggio principale è la semplicità: nessuna configurazione Docker, immagini di container o configurazione di rete richiesta. Il proxy e le restrizioni del filesystem sono integrati. Fornisci un file di configurazione che specifica i domini e i percorsi consentiti.

**Come funziona:**
- **Filesystem**: Utilizza i primitivi del sistema operativo (`bubblewrap` su Linux, `sandbox-exec` su macOS) per limitare l'accesso in lettura/scrittura ai percorsi configurati
- **Rete**: Rimuove lo spazio dei nomi di rete (Linux) o utilizza i profili Seatbelt (macOS) per instradare il traffico di rete attraverso un proxy integrato
- **Configurazione**: Allowlist basate su JSON per domini e percorsi del filesystem

**Configurazione:**
```bash
npm install @anthropic-ai/sandbox-runtime
```

Quindi crea un file di configurazione che specifica i percorsi e i domini consentiti.

**Considerazioni sulla sicurezza:**

1. **Kernel dello stesso host**: A differenza delle VM, i processi in sandbox condividono il kernel dell'host. Una vulnerabilità del kernel potrebbe teoricamente abilitare l'escape. Per alcuni modelli di minaccia questo è accettabile, ma se hai bisogno dell'isolamento a livello del kernel, usa gVisor o una VM separata.

2. **Nessuna ispezione TLS**: Il proxy crea un allowlist dei domini ma non ispeziona il traffico crittografato. Se l'agente ha credenziali permissive per un dominio consentito, assicurati che non sia possibile utilizzare quel dominio per attivare altre richieste di rete o per estrarre dati.

Per molti casi di uso di singoli sviluppatori e CI/CD, sandbox-runtime aumenta significativamente la barra con una configurazione minima. Le sezioni seguenti coprono i container e le VM per le distribuzioni che richiedono un isolamento più forte.

### Container

I container forniscono isolamento attraverso i namespace di Linux. Ogni container ha la sua propria vista del filesystem, dell'albero dei processi e dello stack di rete, mentre condivide il kernel dell'host.

Una configurazione di container con hardening di sicurezza potrebbe assomigliare a questa:

```bash
docker run \
  --cap-drop ALL \
  --security-opt no-new-privileges \
  --security-opt seccomp=/path/to/seccomp-profile.json \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /home/agent:rw,noexec,nosuid,size=500m \
  --network none \
  --memory 2g \
  --cpus 2 \
  --pids-limit 100 \
  --user 1000:1000 \
  -v /path/to/code:/workspace:ro \
  -v /var/run/proxy.sock:/var/run/proxy.sock:ro \
  agent-image
```

Ecco cosa fa ogni opzione:

| Opzione | Scopo |
|---------|-------|
| `--cap-drop ALL` | Rimuove le capacità Linux come `NET_ADMIN` e `SYS_ADMIN` che potrebbero abilitare l'escalation dei privilegi |
| `--security-opt no-new-privileges` | Impedisce ai processi di ottenere privilegi attraverso i binari setuid |
| `--security-opt seccomp=...` | Limita le syscall disponibili; il valore predefinito di Docker blocca ~44, i profili personalizzati possono bloccarne di più |
| `--read-only` | Rende il filesystem root del container immutabile, impedendo all'agente di persistere i cambiamenti |
| `--tmpfs /tmp:...` | Fornisce una directory temporanea scrivibile che viene cancellata quando il container si arresta |
| `--network none` | Rimuove tutte le interfacce di rete; l'agente comunica attraverso il socket Unix montato di seguito |
| `--memory 2g` | Limita l'utilizzo della memoria per prevenire l'esaurimento delle risorse |
| `--pids-limit 100` | Limita il conteggio dei processi per prevenire fork bomb |
| `--user 1000:1000` | Viene eseguito come utente non root |
| `-v ...:/workspace:ro` | Monta il codice in sola lettura in modo che l'agente possa analizzarlo ma non modificarlo. **Evita di montare directory host sensibili come `~/.ssh`, `~/.aws` o `~/.config`** |
| `-v .../proxy.sock:...` | Monta un socket Unix connesso a un proxy in esecuzione al di fuori del container (vedi sotto) |

**Architettura del socket Unix:**

Con `--network none`, il container non ha interfacce di rete. L'unico modo per l'agente di raggiungere il mondo esterno è attraverso il socket Unix montato, che si connette a un proxy in esecuzione sull'host. Questo proxy può applicare allowlist di domini, iniettare credenziali e registrare tutto il traffico.

Questa è la stessa architettura utilizzata da [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime). Anche se l'agente è compromesso tramite prompt injection, non può estrarre dati a server arbitrari—può solo comunicare attraverso il proxy, che controlla quali domini sono raggiungibili. Per ulteriori dettagli, vedi il [post del blog sul sandboxing di Claude Code](https://www.anthropic.com/engineering/claude-code-sandboxing).

**Opzioni di hardening aggiuntive:**

| Opzione | Scopo |
|---------|-------|
| `--userns-remap` | Mappa il root del container all'utente host senza privilegi; richiede la configurazione del daemon ma limita i danni dall'escape del container |
| `--ipc private` | Isola la comunicazione tra processi per prevenire attacchi tra container |

### gVisor

I container standard condividono il kernel dell'host: quando il codice all'interno di un container effettua una chiamata di sistema, va direttamente allo stesso kernel che esegue l'host. Questo significa che una vulnerabilità del kernel potrebbe consentire l'escape del container. gVisor affronta questo intercettando le chiamate di sistema nello spazio utente prima che raggiungano il kernel dell'host, implementando il suo proprio livello di compatibilità che gestisce la maggior parte delle syscall senza coinvolgere il kernel reale.

Se un agente esegue codice dannoso (forse a causa di prompt injection), quel codice viene eseguito nel container e potrebbe tentare exploit del kernel. Con gVisor, la superficie di attacco è molto più piccola: il codice dannoso dovrebbe prima sfruttare l'implementazione dello spazio utente di gVisor e avrebbe accesso limitato al kernel reale.

Per usare gVisor con Docker, installa il runtime `runsc` e configura il daemon:

```json
// /etc/docker/daemon.json
{
  "runtimes": {
    "runsc": {
      "path": "/usr/local/bin/runsc"
    }
  }
}
```

Quindi esegui i container con:

```bash
docker run --runtime=runsc agent-image
```

**Considerazioni sulle prestazioni:**

| Carico di lavoro | Overhead |
|------------------|----------|
| Calcolo legato alla CPU | ~0% (nessuna intercettazione di syscall) |
| Syscall semplici | ~2× più lento |
| Intensivo di I/O su file | Fino a 10-200× più lento per pattern di apertura/chiusura pesanti |

Per gli ambienti multi-tenant o quando si elabora contenuto non affidabile, l'isolamento aggiuntivo è spesso degno dell'overhead.

### Macchine virtuali

Le VM forniscono isolamento a livello hardware attraverso le estensioni di virtualizzazione della CPU. Ogni VM esegue il suo proprio kernel, creando un confine forte—una vulnerabilità nel kernel guest non compromette direttamente l'host. Tuttavia, le VM non sono automaticamente "più sicure" di alternative come gVisor. La sicurezza della VM dipende molto dall'hypervisor e dal codice di emulazione dei dispositivi.

Firecracker è progettato per l'isolamento leggero di microVM—può avviare VM in meno di 125ms con meno di 5 MiB di overhead di memoria, eliminando l'emulazione di dispositivi non necessaria per ridurre la superficie di attacco.

Con questo approccio, la VM dell'agente non ha interfaccia di rete esterna. Invece, comunica attraverso `vsock` (socket virtuali). Tutto il traffico viene instradato attraverso vsock a un proxy sull'host, che applica allowlist e inietta credenziali prima di inoltrare le richieste.

### Distribuzioni cloud

Per le distribuzioni cloud, puoi combinare qualsiasi una delle tecnologie di isolamento di cui sopra con i controlli di rete nativi del cloud:

1. Esegui i container dell'agente in una subnet privata senza gateway internet
2. Configura le regole del firewall cloud (AWS Security Groups, GCP VPC firewall) per bloccare tutto l'egress tranne verso il tuo proxy
3. Esegui un proxy (come [Envoy](https://www.envoyproxy.io/) con il suo filtro `credential_injector`) che convalida le richieste, applica allowlist di domini, inietta credenziali e inoltra agli API esterni
4. Assegna autorizzazioni IAM minime all'account di servizio dell'agente, instradando l'accesso sensibile attraverso il proxy dove possibile
5. Registra tutto il traffico al proxy a scopo di audit

## Gestione delle credenziali

Gli agenti spesso hanno bisogno di credenziali per chiamare API, accedere ai repository o interagire con i servizi cloud. La sfida è fornire questo accesso senza esporre le credenziali stesse.

### Il pattern del proxy

L'approccio consigliato è eseguire un proxy al di fuori del confine di sicurezza dell'agente che inietta le credenziali nelle richieste in uscita. L'agente invia richieste senza credenziali, il proxy le aggiunge e inoltra la richiesta alla sua destinazione.

Questo pattern ha diversi vantaggi:

1. L'agente non vede mai le credenziali effettive
2. Il proxy può applicare un allowlist di endpoint consentiti
3. Il proxy può registrare tutte le richieste per l'audit
4. Le credenziali vengono archiviate in un'unica posizione sicura piuttosto che distribuite a ogni agente

### Configurazione di Claude Code per usare un proxy

Claude Code supporta due metodi per instradare le richieste di campionamento attraverso un proxy:

**Opzione 1: ANTHROPIC_BASE_URL (semplice ma solo per le richieste API di campionamento)**

```bash
export ANTHROPIC_BASE_URL="http://localhost:8080"
```

Questo dice a Claude Code e all'Agent SDK di inviare le richieste di campionamento al tuo proxy invece che direttamente all'API Anthropic. Il tuo proxy riceve richieste HTTP in testo semplice, può ispezionarle e modificarle (incluso l'iniezione di credenziali), quindi inoltra all'API reale.

**Opzione 2: HTTP_PROXY / HTTPS_PROXY (a livello di sistema)**

```bash
export HTTP_PROXY="http://localhost:8080"
export HTTPS_PROXY="http://localhost:8080"
```

Claude Code e l'Agent SDK rispettano queste variabili di ambiente standard, instradando tutto il traffico HTTP attraverso il proxy. Per HTTPS, il proxy crea un tunnel CONNECT crittografato: non può vedere o modificare i contenuti delle richieste senza l'intercettazione TLS.

### Implementazione di un proxy

Puoi costruire il tuo proxy o usarne uno esistente:

- [Envoy Proxy](https://www.envoyproxy.io/) — proxy di livello produzione con filtro `credential_injector` per aggiungere header di autenticazione
- [mitmproxy](https://mitmproxy.org/) — proxy che termina TLS per ispezionare e modificare il traffico HTTPS
- [Squid](http://www.squid-cache.org/) — proxy di caching con liste di controllo di accesso
- [LiteLLM](https://github.com/BerriAI/litellm) — gateway LLM con iniezione di credenziali e rate limiting

### Credenziali per altri servizi

Oltre al campionamento dall'API Anthropic, gli agenti spesso hanno bisogno di accesso autenticato ad altri servizi—repository git, database, API interne. Ci sono due approcci principali:

#### Strumenti personalizzati

Fornisci accesso attraverso un server MCP o uno strumento personalizzato che instrada le richieste a un servizio in esecuzione al di fuori del confine di sicurezza dell'agente. L'agente chiama lo strumento, ma la richiesta autenticata effettiva accade all'esterno—lo strumento chiama a un proxy che inietta le credenziali.

Ad esempio, un server MCP git potrebbe accettare comandi dall'agente ma inoltrarli a un proxy git in esecuzione sull'host, che aggiunge l'autenticazione prima di contattare il repository remoto. L'agente non vede mai le credenziali.

Vantaggi:
- **Nessuna intercettazione TLS**: Il servizio esterno effettua richieste autenticate direttamente
- **Le credenziali rimangono all'esterno**: L'agente vede solo l'interfaccia dello strumento, non le credenziali sottostanti

#### Inoltro del traffico

Per le chiamate all'API Anthropic, `ANTHROPIC_BASE_URL` ti permette di instradare le richieste a un proxy che può ispezionarle e modificarle in testo semplice. Ma per altri servizi HTTPS (GitHub, registri npm, API interne), il traffico è spesso crittografato end-to-end—anche se lo instrada attraverso un proxy tramite `HTTP_PROXY`, il proxy vede solo un tunnel TLS opaco e non può iniettare credenziali.

Per modificare il traffico HTTPS verso servizi arbitrari, senza usare uno strumento personalizzato, hai bisogno di un proxy che termina TLS che decrittografa il traffico, lo ispeziona o lo modifica, quindi lo ricripta prima di inoltrarlo. Questo richiede:

1. Eseguire il proxy al di fuori del container dell'agente
2. Installare il certificato CA del proxy nell'archivio di fiducia dell'agente (in modo che l'agente si fidi dei certificati del proxy)
3. Configurare `HTTP_PROXY`/`HTTPS_PROXY` per instradare il traffico attraverso il proxy

Questo approccio gestisce qualsiasi servizio basato su HTTP senza scrivere strumenti personalizzati, ma aggiunge complessità attorno alla gestione dei certificati.

Nota che non tutti i programmi rispettano `HTTP_PROXY`/`HTTPS_PROXY`. La maggior parte degli strumenti (curl, pip, npm, git) lo fa, ma alcuni potrebbero bypassare queste variabili e connettersi direttamente. Ad esempio, `fetch()` di Node.js ignora queste variabili per impostazione predefinita; in Node 24+ puoi impostare `NODE_USE_ENV_PROXY=1` per abilitare il supporto. Per una copertura completa, puoi usare [proxychains](https://github.com/haad/proxychains) per intercettare le chiamate di rete, o configurare iptables per reindirizzare il traffico in uscita a un proxy trasparente.

<Info>
Un **proxy trasparente** intercetta il traffico a livello di rete, quindi il client non ha bisogno di essere configurato per usarlo. I proxy regolari richiedono ai client di connettersi esplicitamente e parlare HTTP CONNECT o SOCKS. I proxy trasparenti (come Squid o mitmproxy in modalità trasparente) possono gestire connessioni TCP grezze reindirizzate.
</Info>

Entrambi gli approcci richiedono ancora il proxy che termina TLS e il certificato CA affidabile—assicurano semplicemente che il traffico raggiunga effettivamente il proxy.

## Configurazione del filesystem

I controlli del filesystem determinano quali file l'agente può leggere e scrivere.

### Montaggio del codice in sola lettura

Quando l'agente ha bisogno di analizzare il codice ma non modificarlo, monta la directory in sola lettura:

```bash
docker run -v /path/to/code:/workspace:ro agent-image
```

<Warning>
Anche l'accesso in sola lettura a una directory di codice può esporre le credenziali. File comuni da escludere o bonificare prima del montaggio:

| File | Rischio |
|------|---------|
| `.env`, `.env.local` | Chiavi API, password del database, segreti |
| `~/.git-credentials` | Password/token git in testo semplice |
| `~/.aws/credentials` | Chiavi di accesso AWS |
| `~/.config/gcloud/application_default_credentials.json` | Token ADC di Google Cloud |
| `~/.azure/` | Credenziali CLI di Azure |
| `~/.docker/config.json` | Token di autenticazione del registro Docker |
| `~/.kube/config` | Credenziali del cluster Kubernetes |
| `.npmrc`, `.pypirc` | Token del registro dei pacchetti |
| `*-service-account.json` | Chiavi dell'account di servizio GCP |
| `*.pem`, `*.key` | Chiavi private |

Considera di copiare solo i file sorgente necessari, o di usare il filtraggio in stile `.dockerignore`.
</Warning>

### Posizioni scrivibili

Se l'agente ha bisogno di scrivere file, hai alcune opzioni a seconda che tu voglia che i cambiamenti persistano:

Per gli spazi di lavoro effimeri nei container, usa i montaggi `tmpfs` che esistono solo in memoria e vengono cancellati quando il container si arresta:

```bash
docker run \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /workspace:rw,noexec,size=500m \
  agent-image
```

Se vuoi rivedere i cambiamenti prima di persistarli, un filesystem overlay permette all'agente di scrivere senza modificare i file sottostanti—i cambiamenti vengono archiviati in un livello separato che puoi ispezionare, applicare o scartare. Per l'output completamente persistente, monta un volume dedicato ma mantienilo separato dalle directory sensibili.

## Ulteriori letture

- [Documentazione sulla sicurezza di Claude Code](https://code.claude.com/docs/en/security)
- [Hosting dell'Agent SDK](/docs/it/agent-sdk/hosting)
- [Gestione delle autorizzazioni](/docs/it/agent-sdk/permissions)
- [Sandbox runtime](https://github.com/anthropic-experimental/sandbox-runtime)
- [The Lethal Trifecta for AI Agents](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/)
- [OWASP Top 10 for LLM Applications](https://owasp.org/www-project-top-10-for-large-language-model-applications/)
- [Docker Security Best Practices](https://docs.docker.com/engine/security/)
- [gVisor Documentation](https://gvisor.dev/docs/)
- [Firecracker Documentation](https://firecracker-microvm.github.io/)