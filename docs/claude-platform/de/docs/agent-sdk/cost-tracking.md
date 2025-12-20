# Kosten und Nutzung verfolgen

Token-Nutzung für die Abrechnung im Claude Agent SDK verstehen und verfolgen

---

# SDK-Kostenverfolgung

Das Claude Agent SDK bietet detaillierte Token-Nutzungsinformationen für jede Interaktion mit Claude. Dieser Leitfaden erklärt, wie Sie Kosten ordnungsgemäß verfolgen und die Nutzungsberichterstattung verstehen, insbesondere beim Umgang mit parallelen Tool-Verwendungen und mehrstufigen Unterhaltungen.

Für die vollständige API-Dokumentation siehe die [TypeScript SDK-Referenz](https://code.claude.com/docs/typescript-sdk-reference).

## Token-Nutzung verstehen

Wenn Claude Anfragen verarbeitet, meldet es die Token-Nutzung auf Nachrichtenebene. Diese Nutzungsdaten sind wesentlich für die Kostenverfolgung und die angemessene Abrechnung von Benutzern.

### Schlüsselkonzepte

1. **Schritte**: Ein Schritt ist ein einzelnes Anfrage/Antwort-Paar zwischen Ihrer Anwendung und Claude
2. **Nachrichten**: Einzelne Nachrichten innerhalb eines Schritts (Text, Tool-Verwendungen, Tool-Ergebnisse)
3. **Nutzung**: Token-Verbrauchsdaten, die an Assistenten-Nachrichten angehängt sind

## Struktur der Nutzungsberichterstattung

### Einzelne vs. parallele Tool-Verwendung

Wenn Claude Tools ausführt, unterscheidet sich die Nutzungsberichterstattung je nachdem, ob Tools sequenziell oder parallel ausgeführt werden:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Beispiel: Nutzung in einer Unterhaltung verfolgen
const result = await query({
  prompt: "Analysiere diese Codebasis und führe Tests aus",
  options: {
    onMessage: (message) => {
      if (message.type === 'assistant' && message.usage) {
        console.log(`Nachrichten-ID: ${message.id}`);
        console.log(`Nutzung:`, message.usage);
      }
    }
  }
});
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage
import asyncio

# Beispiel: Nutzung in einer Unterhaltung verfolgen
async def track_usage():
    # Nachrichten verarbeiten, während sie eintreffen
    async for message in query(
        prompt="Analysiere diese Codebasis und führe Tests aus"
    ):
        if isinstance(message, AssistantMessage) and hasattr(message, 'usage'):
            print(f"Nachrichten-ID: {message.id}")
            print(f"Nutzung: {message.usage}")

asyncio.run(track_usage())
```

</CodeGroup>

### Beispiel für Nachrichtenfluss

So werden Nachrichten und Nutzung in einer typischen mehrstufigen Unterhaltung gemeldet:

```
<!-- Schritt 1: Erste Anfrage mit parallelen Tool-Verwendungen -->
assistant (text)      { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
user (tool_result)
user (tool_result)
user (tool_result)

<!-- Schritt 2: Folge-Antwort -->
assistant (text)      { id: "msg_2", usage: { output_tokens: 98, ... } }
```

## Wichtige Nutzungsregeln

### 1. Gleiche ID = Gleiche Nutzung

**Alle Nachrichten mit demselben `id`-Feld melden identische Nutzung**. Wenn Claude mehrere Nachrichten im selben Zug sendet (z.B. Text + Tool-Verwendungen), teilen sie dieselbe Nachrichten-ID und Nutzungsdaten.

```typescript
// Alle diese Nachrichten haben dieselbe ID und Nutzung
const messages = [
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } }
];

// Nur einmal pro eindeutiger Nachrichten-ID berechnen
const uniqueUsage = messages[0].usage; // Gleich für alle Nachrichten mit dieser ID
```

### 2. Einmal pro Schritt berechnen

**Sie sollten Benutzer nur einmal pro Schritt berechnen**, nicht für jede einzelne Nachricht. Wenn Sie mehrere Assistenten-Nachrichten mit derselben ID sehen, verwenden Sie die Nutzung von einer beliebigen davon.

### 3. Ergebnisnachricht enthält kumulative Nutzung

Die finale `result`-Nachricht enthält die gesamte kumulative Nutzung aller Schritte in der Unterhaltung:

```typescript
// Finales Ergebnis enthält Gesamtnutzung
const result = await query({
  prompt: "Mehrstufige Aufgabe",
  options: { /* ... */ }
});

console.log("Gesamtnutzung:", result.usage);
console.log("Gesamtkosten:", result.usage.total_cost_usd);
```

## Implementierung: Kostenverfolgungssystem

Hier ist ein vollständiges Beispiel für die Implementierung eines Kostenverfolgungssystems:

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
    // Nur Assistenten-Nachrichten mit Nutzung verarbeiten
    if (message.type !== 'assistant' || !message.usage) {
      return;
    }
    
    // Überspringen, wenn wir diese Nachrichten-ID bereits verarbeitet haben
    if (this.processedMessageIds.has(message.id)) {
      return;
    }
    
    // Als verarbeitet markieren und Nutzung aufzeichnen
    this.processedMessageIds.add(message.id);
    this.stepUsages.push({
      messageId: message.id,
      timestamp: new Date().toISOString(),
      usage: message.usage,
      costUSD: this.calculateCost(message.usage)
    });
  }
  
  private calculateCost(usage: any): number {
    // Implementieren Sie hier Ihre Preisberechnung
    // Dies ist ein vereinfachtes Beispiel
    const inputCost = usage.input_tokens * 0.00003;
    const outputCost = usage.output_tokens * 0.00015;
    const cacheReadCost = (usage.cache_read_input_tokens || 0) * 0.0000075;
    
    return inputCost + outputCost + cacheReadCost;
  }
}

// Verwendung
const tracker = new CostTracker();
const { result, stepUsages, totalCost } = await tracker.trackConversation(
  "Analysiere und refaktoriere diesen Code"
);

console.log(`Verarbeitete Schritte: ${stepUsages.length}`);
console.log(`Gesamtkosten: $${totalCost.toFixed(4)}`);
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

        # Nachrichten verarbeiten, während sie eintreffen
        async for message in query(prompt=prompt):
            self.process_message(message)

            # Die finale Ergebnisnachricht erfassen
            if isinstance(message, ResultMessage):
                result = message

        return {
            "result": result,
            "step_usages": self.step_usages,
            "total_cost": result.total_cost_usd if result else 0
        }

    def process_message(self, message):
        # Nur Assistenten-Nachrichten mit Nutzung verarbeiten
        if not isinstance(message, AssistantMessage) or not hasattr(message, 'usage'):
            return

        # Überspringen, wenn diese Nachrichten-ID bereits verarbeitet wurde
        message_id = getattr(message, 'id', None)
        if not message_id or message_id in self.processed_message_ids:
            return

        # Als verarbeitet markieren und Nutzung aufzeichnen
        self.processed_message_ids.add(message_id)
        self.step_usages.append({
            "message_id": message_id,
            "timestamp": datetime.now().isoformat(),
            "usage": message.usage,
            "cost_usd": self.calculate_cost(message.usage)
        })

    def calculate_cost(self, usage):
        # Implementieren Sie Ihre Preisberechnung
        input_cost = usage.get("input_tokens", 0) * 0.00003
        output_cost = usage.get("output_tokens", 0) * 0.00015
        cache_read_cost = usage.get("cache_read_input_tokens", 0) * 0.0000075

        return input_cost + output_cost + cache_read_cost

# Verwendung
async def main():
    tracker = CostTracker()
    result = await tracker.track_conversation("Analysiere und refaktoriere diesen Code")

    print(f"Verarbeitete Schritte: {len(result['step_usages'])}")
    print(f"Gesamtkosten: ${result['total_cost']:.4f}")

asyncio.run(main())
```

</CodeGroup>

## Umgang mit Sonderfällen

### Diskrepanzen bei Output-Token

In seltenen Fällen könnten Sie unterschiedliche `output_tokens`-Werte für Nachrichten mit derselben ID beobachten. Wenn dies auftritt:

1. **Verwenden Sie den höchsten Wert** - Die letzte Nachricht in einer Gruppe enthält typischerweise die genaue Gesamtsumme
2. **Gegen Gesamtkosten verifizieren** - Die `total_cost_usd` in der Ergebnisnachricht ist maßgebend
3. **Inkonsistenzen melden** - Reichen Sie Probleme im [Claude Code GitHub-Repository](https://github.com/anthropics/claude-code/issues) ein

### Cache-Token-Verfolgung

Bei Verwendung von Prompt-Caching verfolgen Sie diese Token-Typen separat:

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

## Best Practices

1. **Nachrichten-IDs für Deduplizierung verwenden**: Verfolgen Sie immer verarbeitete Nachrichten-IDs, um Doppelberechnungen zu vermeiden
2. **Die Ergebnisnachricht überwachen**: Das finale Ergebnis enthält maßgebende kumulative Nutzung
3. **Protokollierung implementieren**: Protokollieren Sie alle Nutzungsdaten für Auditing und Debugging
4. **Fehler elegant behandeln**: Verfolgen Sie partielle Nutzung auch wenn eine Unterhaltung fehlschlägt
5. **Streaming berücksichtigen**: Für Streaming-Antworten akkumulieren Sie Nutzung während Nachrichten eintreffen

## Referenz für Nutzungsfelder

Jedes Nutzungsobjekt enthält:

- `input_tokens`: Verarbeitete Basis-Input-Token
- `output_tokens`: In der Antwort generierte Token
- `cache_creation_input_tokens`: Token, die zur Erstellung von Cache-Einträgen verwendet wurden
- `cache_read_input_tokens`: Aus dem Cache gelesene Token
- `service_tier`: Die verwendete Service-Stufe (z.B. "standard")
- `total_cost_usd`: Gesamtkosten in USD (nur in Ergebnisnachricht)

## Beispiel: Aufbau eines Abrechnungs-Dashboards

So aggregieren Sie Nutzungsdaten für ein Abrechnungs-Dashboard:

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
    
    // Benutzer-Gesamtsummen aktualisieren
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

## Verwandte Dokumentation

- [TypeScript SDK-Referenz](https://code.claude.com/docs/typescript-sdk-reference) - Vollständige API-Dokumentation
- [SDK-Übersicht](/docs/de/agent-sdk/overview) - Erste Schritte mit dem SDK
- [SDK-Berechtigungen](/docs/de/agent-sdk/permissions) - Tool-Berechtigungen verwalten