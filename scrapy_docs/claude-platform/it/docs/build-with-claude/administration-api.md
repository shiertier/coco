# Panoramica dell'Admin API

Gestisci programmaticamente le risorse della tua organizzazione, inclusi i membri, gli spazi di lavoro e le chiavi API.

---

<Tip>
**The Admin API is unavailable for individual accounts.** To collaborate with teammates and add members, set up your organization in **Console → Settings → Organization**.
</Tip>

L'[Admin API](/docs/it/api/admin) ti consente di gestire programmaticamente le risorse della tua organizzazione, inclusi i membri dell'organizzazione, gli spazi di lavoro e le chiavi API. Questo fornisce il controllo programmatico su attività amministrative che altrimenti richiederebbero una configurazione manuale nella [Claude Console](/).

<Check>
  **L'Admin API richiede un accesso speciale**

  L'Admin API richiede una chiave Admin API speciale (che inizia con `sk-ant-admin...`) che differisce dalle chiavi API standard. Solo i membri dell'organizzazione con il ruolo di amministratore possono fornire chiavi Admin API tramite la Claude Console.
</Check>

## Come funziona l'Admin API

Quando utilizzi l'Admin API:

1. Effettui richieste utilizzando la tua chiave Admin API nell'intestazione `x-api-key`
2. L'API ti consente di gestire:
   - I membri dell'organizzazione e i loro ruoli
   - Gli inviti dei membri dell'organizzazione
   - Gli spazi di lavoro e i loro membri
   - Le chiavi API

Questo è utile per:
- Automatizzare l'onboarding/offboarding degli utenti
- Gestire programmaticamente l'accesso allo spazio di lavoro
- Monitorare e gestire l'utilizzo delle chiavi API

## Ruoli e autorizzazioni dell'organizzazione

Ci sono cinque ruoli a livello di organizzazione. Vedi più dettagli [qui](https://support.claude.com/en/articles/10186004-api-console-roles-and-permissions).

| Ruolo | Autorizzazioni |
|------|-------------|
| user | Può utilizzare Workbench |
| claude_code_user | Può utilizzare Workbench e [Claude Code](https://code.claude.com/docs/en/overview) |
| developer | Può utilizzare Workbench e gestire le chiavi API |
| billing | Può utilizzare Workbench e gestire i dettagli di fatturazione |
| admin | Può fare tutto quanto sopra, più gestire gli utenti |

## Concetti chiave

### Membri dell'organizzazione

Puoi elencare i [membri dell'organizzazione](/docs/it/api/admin-api/users/get-user), aggiornare i ruoli dei membri e rimuovere i membri.

<CodeGroup>
```bash Shell
# Elenca i membri dell'organizzazione
curl "https://api.anthropic.com/v1/organizations/users?limit=10" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY"

# Aggiorna il ruolo del membro
curl "https://api.anthropic.com/v1/organizations/users/{user_id}" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY" \
  --data '{"role": "developer"}'

# Rimuovi membro
curl --request DELETE "https://api.anthropic.com/v1/organizations/users/{user_id}" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY"
```

</CodeGroup>

### Inviti dell'organizzazione

Puoi invitare gli utenti alle organizzazioni e gestire questi [inviti](/docs/it/api/admin-api/invites/get-invite).

<CodeGroup>

```bash Shell
# Crea invito
curl --request POST "https://api.anthropic.com/v1/organizations/invites" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY" \
  --data '{
    "email": "newuser@domain.com",
    "role": "developer"
  }'

# Elenca inviti
curl "https://api.anthropic.com/v1/organizations/invites?limit=10" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY"

# Elimina invito
curl --request DELETE "https://api.anthropic.com/v1/organizations/invites/{invite_id}" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY"
```

</CodeGroup>

### Spazi di lavoro

Crea e gestisci gli [spazi di lavoro](/docs/it/api/admin-api/workspaces/get-workspace) ([console](/settings/workspaces)) per organizzare le tue risorse:

<CodeGroup>

```bash Shell
# Crea spazio di lavoro
curl --request POST "https://api.anthropic.com/v1/organizations/workspaces" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY" \
  --data '{"name": "Production"}'

# Elenca spazi di lavoro
curl "https://api.anthropic.com/v1/organizations/workspaces?limit=10&include_archived=false" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY"

# Archivia spazio di lavoro
curl --request POST "https://api.anthropic.com/v1/organizations/workspaces/{workspace_id}/archive" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY"
```

</CodeGroup>

### Membri dello spazio di lavoro

Gestisci l'[accesso dell'utente a spazi di lavoro specifici](/docs/it/api/admin-api/workspace_members/get-workspace-member):

<CodeGroup>

```bash Shell
# Aggiungi membro allo spazio di lavoro
curl --request POST "https://api.anthropic.com/v1/organizations/workspaces/{workspace_id}/members" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY" \
  --data '{
    "user_id": "user_xxx",
    "workspace_role": "workspace_developer"
  }'

# Elenca i membri dello spazio di lavoro
curl "https://api.anthropic.com/v1/organizations/workspaces/{workspace_id}/members?limit=10" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY"

# Aggiorna il ruolo del membro
curl --request POST "https://api.anthropic.com/v1/organizations/workspaces/{workspace_id}/members/{user_id}" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY" \
  --data '{
    "workspace_role": "workspace_admin"
  }'

# Rimuovi membro dallo spazio di lavoro
curl --request DELETE "https://api.anthropic.com/v1/organizations/workspaces/{workspace_id}/members/{user_id}" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY"
```

</CodeGroup>

### Chiavi API

Monitora e gestisci le [chiavi API](/docs/it/api/admin-api/apikeys/get-api-key):

<CodeGroup>

```bash Shell
# Elenca le chiavi API
curl "https://api.anthropic.com/v1/organizations/api_keys?limit=10&status=active&workspace_id=wrkspc_xxx" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY"

# Aggiorna chiave API
curl --request POST "https://api.anthropic.com/v1/organizations/api_keys/{api_key_id}" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ANTHROPIC_ADMIN_KEY" \
  --data '{
    "status": "inactive",
    "name": "New Key Name"
  }'
```

</CodeGroup>

## Accesso alle informazioni dell'organizzazione

Ottieni informazioni sulla tua organizzazione programmaticamente con l'endpoint `/v1/organizations/me`.

Ad esempio:

```bash
curl "https://api.anthropic.com/v1/organizations/me" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ADMIN_API_KEY"
```

```json
{
  "id": "12345678-1234-5678-1234-567812345678",
  "type": "organization",
  "name": "Organization Name"
}
```

Questo endpoint è utile per determinare programmaticamente a quale organizzazione appartiene una chiave Admin API.

Per i dettagli completi dei parametri e gli schemi di risposta, vedi il [riferimento dell'API Organization Info](/docs/it/api/admin-api/organization/get-me).

## Accesso ai rapporti di utilizzo e costo

Per accedere ai rapporti di utilizzo e costo della tua organizzazione, utilizza gli endpoint dell'API di utilizzo e costo:

- L'[**endpoint di utilizzo**](/docs/it/build-with-claude/usage-cost-api#usage-api) (`/v1/organizations/usage_report/messages`) fornisce dati di utilizzo dettagliati, inclusi i conteggi dei token e le metriche delle richieste, raggruppati per varie dimensioni come spazio di lavoro, utente e modello.
- L'[**endpoint di costo**](/docs/it/build-with-claude/usage-cost-api#cost-api) (`/v1/organizations/cost_report`) fornisce dati di costo associati all'utilizzo della tua organizzazione, consentendoti di tracciare le spese e allocare i costi per spazio di lavoro o descrizione.

Questi endpoint forniscono informazioni dettagliate sull'utilizzo della tua organizzazione e sui costi associati.

## Accesso all'analisi di Claude Code

Per le organizzazioni che utilizzano Claude Code, l'[**API Claude Code Analytics**](/docs/it/build-with-claude/claude-code-analytics-api) fornisce metriche di produttività dettagliate e informazioni sull'utilizzo:

- L'[**endpoint Claude Code Analytics**](/docs/it/build-with-claude/claude-code-analytics-api) (`/v1/organizations/usage_report/claude_code`) fornisce metriche aggregate giornaliere per l'utilizzo di Claude Code, incluse sessioni, righe di codice, commit, richieste pull, statistiche di utilizzo degli strumenti e dati di costo suddivisi per utente e modello.

Questa API ti consente di tracciare la produttività degli sviluppatori, analizzare l'adozione di Claude Code e creare dashboard personalizzati per la tua organizzazione.

## Best practice

Per utilizzare efficacemente l'Admin API:

- Utilizza nomi e descrizioni significativi per gli spazi di lavoro e le chiavi API
- Implementa una gestione degli errori appropriata per le operazioni non riuscite
- Controlla regolarmente i ruoli e le autorizzazioni dei membri
- Pulisci gli spazi di lavoro inutilizzati e gli inviti scaduti
- Monitora l'utilizzo delle chiavi API e ruota le chiavi periodicamente

## FAQ

<section title="Quali autorizzazioni sono necessarie per utilizzare l'Admin API?">

Solo i membri dell'organizzazione con il ruolo di amministratore possono utilizzare l'Admin API. Devono anche avere una chiave Admin API speciale (che inizia con `sk-ant-admin`).

</section>

<section title="Posso creare nuove chiavi API tramite l'Admin API?">

No, le nuove chiavi API possono essere create solo tramite la Claude Console per motivi di sicurezza. L'Admin API può solo gestire le chiavi API esistenti.

</section>

<section title="Cosa succede alle chiavi API quando si rimuove un utente?">

Le chiavi API persistono nel loro stato attuale poiché sono limitate all'organizzazione, non ai singoli utenti.

</section>

<section title="Gli amministratori dell'organizzazione possono essere rimossi tramite l'API?">

No, i membri dell'organizzazione con il ruolo di amministratore non possono essere rimossi tramite l'API per motivi di sicurezza.

</section>

<section title="Quanto durano gli inviti dell'organizzazione?">

Gli inviti dell'organizzazione scadono dopo 21 giorni. Attualmente non c'è modo di modificare questo periodo di scadenza.

</section>

<section title="Ci sono limiti agli spazi di lavoro?">

Sì, puoi avere un massimo di 100 spazi di lavoro per organizzazione. Gli spazi di lavoro archiviati non contano verso questo limite.

</section>

<section title="Qual è lo spazio di lavoro predefinito?">

Ogni organizzazione ha uno "spazio di lavoro predefinito" che non può essere modificato o rimosso e non ha un ID. Questo spazio di lavoro non appare negli endpoint dell'elenco degli spazi di lavoro.

</section>

<section title="Come influiscono i ruoli dell'organizzazione sull'accesso allo spazio di lavoro?">

Gli amministratori dell'organizzazione ottengono automaticamente il ruolo `workspace_admin` a tutti gli spazi di lavoro. I membri della fatturazione dell'organizzazione ottengono automaticamente il ruolo `workspace_billing`. Gli utenti e gli sviluppatori dell'organizzazione devono essere aggiunti manualmente a ogni spazio di lavoro.

</section>

<section title="Quali ruoli possono essere assegnati negli spazi di lavoro?">

Gli utenti e gli sviluppatori dell'organizzazione possono essere assegnati ai ruoli `workspace_admin`, `workspace_developer` o `workspace_user`. Il ruolo `workspace_billing` non può essere assegnato manualmente - viene ereditato dall'avere il ruolo di fatturazione dell'organizzazione.

</section>

<section title="I ruoli dello spazio di lavoro dei membri amministratori o di fatturazione dell'organizzazione possono essere modificati?">

Solo i membri della fatturazione dell'organizzazione possono avere il loro ruolo dello spazio di lavoro aggiornato a un ruolo di amministratore. Altrimenti, gli amministratori e i membri della fatturazione dell'organizzazione non possono avere i loro ruoli dello spazio di lavoro modificati o essere rimossi dagli spazi di lavoro mentre mantengono quei ruoli dell'organizzazione. L'accesso allo spazio di lavoro deve essere modificato cambiando prima il loro ruolo nell'organizzazione.

</section>

<section title="Cosa succede all'accesso allo spazio di lavoro quando i ruoli dell'organizzazione cambiano?">

Se un amministratore dell'organizzazione o un membro della fatturazione viene retrocesso a utente o sviluppatore, perde l'accesso a tutti gli spazi di lavoro tranne quelli a cui è stato assegnato manualmente. Quando gli utenti vengono promossi a ruoli di amministratore o fatturazione, ottengono accesso automatico a tutti gli spazi di lavoro.

</section>