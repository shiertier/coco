# Tracciamento di Costi e Utilizzo

Comprendi e traccia l'utilizzo dei token per la fatturazione nel Claude Agent SDK

---

# Tracciamento dei Costi SDK

Il Claude Agent SDK fornisce informazioni dettagliate sull'utilizzo dei token per ogni interazione con Claude. Questa guida spiega come tracciare correttamente i costi e comprendere il reporting dell'utilizzo, specialmente quando si gestiscono utilizzi di strumenti paralleli e conversazioni multi-step.

Per la documentazione API completa, consulta il [riferimento TypeScript SDK](https://code.claude.com/docs/typescript-sdk-reference).

## Comprensione dell'Utilizzo dei Token

Quando Claude elabora le richieste, riporta l'utilizzo dei token a livello di messaggio. Questi dati di utilizzo sono essenziali per tracciare i costi e fatturare gli utenti in modo appropriato.

### Concetti Chiave

1. **Steps**: Uno step è una singola coppia richiesta/risposta tra la tua applicazione e Claude
2. **Messaggi**: Messaggi individuali all'interno di uno step (testo, utilizzi di strumenti, risultati di strumenti)
3. **Utilizzo**: Dati di consumo dei token allegati ai messaggi dell'assistente

## Struttura del Reporting dell'Utilizzo

### Utilizzo di Strumenti Singolo vs Parallelo

Quando Claude esegue strumenti, il reporting dell'utilizzo differisce in base al fatto che gli strumenti vengano eseguiti sequenzialmente o in parallelo:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Esempio: Tracciamento dell'utilizzo in una conversazione
const result = await query({
  prompt: "Analizza questo codebase ed esegui i test",
  options: {
    onMessage: (message) => {
      if (message.type === 'assistant' && message.usage) {
        console.log(`ID Messaggio: ${message.id}`);
        console.log(`Utilizzo:`, message.usage);
      }
    }
  }
});
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage
import asyncio

# Esempio: Tracciamento dell'utilizzo in una conversazione
async def track_usage():
    # Elabora i messaggi man mano che arrivano
    async for message in query(
        prompt="Analizza questo codebase ed esegui i test"
    ):
        if isinstance(message, AssistantMessage) and hasattr(message, 'usage'):
            print(f"ID Messaggio: {message.id}")
            print(f"Utilizzo: {message.usage}")

asyncio.run(track_usage())
```

</CodeGroup>

### Esempio di Flusso di Messaggi

Ecco come vengono riportati i messaggi e l'utilizzo in una tipica conversazione multi-step:

```
<!-- Step 1: Richiesta iniziale con utilizzi di strumenti paralleli -->
assistant (text)      { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
user (tool_result)
user (tool_result)
user (tool_result)

<!-- Step 2: Risposta di follow-up -->
assistant (text)      { id: "msg_2", usage: { output_tokens: 98, ... } }
```

## Regole Importanti per l'Utilizzo

### 1. Stesso ID = Stesso Utilizzo

**Tutti i messaggi con lo stesso campo `id` riportano utilizzo identico**. Quando Claude invia più messaggi nello stesso turno (ad es., testo + utilizzi di strumenti), condividono lo stesso ID messaggio e dati di utilizzo.

```typescript
// Tutti questi messaggi hanno lo stesso ID e utilizzo
const messages = [
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } }
];

// Addebita solo una volta per ID messaggio unico
const uniqueUsage = messages[0].usage; // Uguale per tutti i messaggi con questo ID
```

### 2. Addebita Una Volta Per Step

**Dovresti addebitare agli utenti solo una volta per step**, non per ogni singolo messaggio. Quando vedi più messaggi dell'assistente con lo stesso ID, usa l'utilizzo da uno qualsiasi di essi.

### 3. Il Messaggio Risultato Contiene l'Utilizzo Cumulativo

Il messaggio finale `result` contiene l'utilizzo cumulativo totale da tutti gli step nella conversazione:

```typescript
// Il risultato finale include l'utilizzo totale
const result = await query({
  prompt: "Attività multi-step",
  options: { /* ... */ }
});

console.log("Utilizzo totale:", result.usage);
console.log("Costo totale:", result.usage.total_cost_usd);
```

## Implementazione: Sistema di Tracciamento dei Costi

Ecco un esempio completo di implementazione di un sistema di tracciamento dei costi:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

class CostTracker {
  private processedMessageIds = new Set<string>();
  private stepUsages: Array<any> = [];
  
  async trackConversation(prompt: string) {
    const result = await query({
      prompt,
      options: {
        onMessage: (message) => {
          this.processMessage(message);
        }
      }
    });
    
    return {
      result,
      stepUsages: this.stepUsages,
      totalCost: result.usage?.total_cost_usd || 0
    };
  }
  
  private processMessage(message: any) {
    // Elabora solo messaggi dell'assistente con utilizzo
    if (message.type !== 'assistant' || !message.usage) {
      return;
    }
    
    // Salta se abbiamo già elaborato questo ID messaggio
    if (this.processedMessageIds.has(message.id)) {
      return;
    }
    
    // Segna come elaborato e registra l'utilizzo
    this.processedMessageIds.add(message.id);
    this.stepUsages.push({
      messageId: message.id,
      timestamp: new Date().toISOString(),
      usage: message.usage,
      costUSD: this.calculateCost(message.usage)
    });
  }
  
  private calculateCost(usage: any): number {
    // Implementa qui il tuo calcolo dei prezzi
    // Questo è un esempio semplificato
    const inputCost = usage.input_tokens * 0.00003;
    const outputCost = usage.output_tokens * 0.00015;
    const cacheReadCost = (usage.cache_read_input_tokens || 0) * 0.0000075;
    
    return inputCost + outputCost + cacheReadCost;
  }
}

// Utilizzo
const tracker = new CostTracker();
const { result, stepUsages, totalCost } = await tracker.trackConversation(
  "Analizza e refactorizza questo codice"
);

console.log(`Step elaborati: ${stepUsages.length}`);
console.log(`Costo totale: $${totalCost.toFixed(4)}`);
```

```python Python
from claude_agent_sdk import query, AssistantMessage, ResultMessage
from datetime import datetime
import asyncio

class CostTracker:
    def __init__(self):
        self.processed_message_ids = set()
        self.step_usages = []

    async def track_conversation(self, prompt):
        result = None

        # Elabora i messaggi man mano che arrivano
        async for message in query(prompt=prompt):
            self.process_message(message)

            # Cattura il messaggio risultato finale
            if isinstance(message, ResultMessage):
                result = message

        return {
            "result": result,
            "step_usages": self.step_usages,
            "total_cost": result.total_cost_usd if result else 0
        }

    def process_message(self, message):
        # Elabora solo messaggi dell'assistente con utilizzo
        if not isinstance(message, AssistantMessage) or not hasattr(message, 'usage'):
            return

        # Salta se già elaborato questo ID messaggio
        message_id = getattr(message, 'id', None)
        if not message_id or message_id in self.processed_message_ids:
            return

        # Segna come elaborato e registra l'utilizzo
        self.processed_message_ids.add(message_id)
        self.step_usages.append({
            "message_id": message_id,
            "timestamp": datetime.now().isoformat(),
            "usage": message.usage,
            "cost_usd": self.calculate_cost(message.usage)
        })

    def calculate_cost(self, usage):
        # Implementa il tuo calcolo dei prezzi
        input_cost = usage.get("input_tokens", 0) * 0.00003
        output_cost = usage.get("output_tokens", 0) * 0.00015
        cache_read_cost = usage.get("cache_read_input_tokens", 0) * 0.0000075

        return input_cost + output_cost + cache_read_cost

# Utilizzo
async def main():
    tracker = CostTracker()
    result = await tracker.track_conversation("Analizza e refactorizza questo codice")

    print(f"Step elaborati: {len(result['step_usages'])}")
    print(f"Costo totale: ${result['total_cost']:.4f}")

asyncio.run(main())
```

</CodeGroup>

## Gestione dei Casi Limite

### Discrepanze nei Token di Output

In rari casi, potresti osservare valori diversi di `output_tokens` per messaggi con lo stesso ID. Quando questo accade:

1. **Usa il valore più alto** - L'ultimo messaggio in un gruppo tipicamente contiene il totale accurato
2. **Verifica contro il costo totale** - Il `total_cost_usd` nel messaggio risultato è autorevole
3. **Segnala inconsistenze** - Segnala problemi al [repository GitHub di Claude Code](https://github.com/anthropics/claude-code/issues)

### Tracciamento dei Token di Cache

Quando usi il prompt caching, traccia questi tipi di token separatamente:

```typescript
interface CacheUsage {
  cache_creation_input_tokens: number;
  cache_read_input_tokens: number;
  cache_creation: {
    ephemeral_5m_input_tokens: number;
    ephemeral_1h_input_tokens: number;
  };
}
```

## Migliori Pratiche

1. **Usa gli ID Messaggio per la Deduplicazione**: Traccia sempre gli ID messaggio elaborati per evitare doppi addebiti
2. **Monitora il Messaggio Risultato**: Il risultato finale contiene l'utilizzo cumulativo autorevole
3. **Implementa il Logging**: Registra tutti i dati di utilizzo per audit e debug
4. **Gestisci i Fallimenti con Grazia**: Traccia l'utilizzo parziale anche se una conversazione fallisce
5. **Considera lo Streaming**: Per risposte in streaming, accumula l'utilizzo man mano che arrivano i messaggi

## Riferimento dei Campi di Utilizzo

Ogni oggetto utilizzo contiene:

- `input_tokens`: Token di input base elaborati
- `output_tokens`: Token generati nella risposta
- `cache_creation_input_tokens`: Token usati per creare voci di cache
- `cache_read_input_tokens`: Token letti dalla cache
- `service_tier`: Il livello di servizio usato (ad es., "standard")
- `total_cost_usd`: Costo totale in USD (solo nel messaggio risultato)

## Esempio: Costruzione di una Dashboard di Fatturazione

Ecco come aggregare i dati di utilizzo per una dashboard di fatturazione:

```typescript
class BillingAggregator {
  private userUsage = new Map<string, {
    totalTokens: number;
    totalCost: number;
    conversations: number;
  }>();
  
  async processUserRequest(userId: string, prompt: string) {
    const tracker = new CostTracker();
    const { result, stepUsages, totalCost } = await tracker.trackConversation(prompt);
    
    // Aggiorna i totali utente
    const current = this.userUsage.get(userId) || {
      totalTokens: 0,
      totalCost: 0,
      conversations: 0
    };
    
    const totalTokens = stepUsages.reduce((sum, step) => 
      sum + step.usage.input_tokens + step.usage.output_tokens, 0
    );
    
    this.userUsage.set(userId, {
      totalTokens: current.totalTokens + totalTokens,
      totalCost: current.totalCost + totalCost,
      conversations: current.conversations + 1
    });
    
    return result;
  }
  
  getUserBilling(userId: string) {
    return this.userUsage.get(userId) || {
      totalTokens: 0,
      totalCost: 0,
      conversations: 0
    };
  }
}
```

## Documentazione Correlata

- [Riferimento TypeScript SDK](https://code.claude.com/docs/typescript-sdk-reference) - Documentazione API completa
- [Panoramica SDK](/docs/it/agent-sdk/overview) - Iniziare con l'SDK
- [Permessi SDK](/docs/it/agent-sdk/permissions) - Gestione dei permessi degli strumenti