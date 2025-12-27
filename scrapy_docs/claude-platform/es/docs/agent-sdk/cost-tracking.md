# Seguimiento de Costos y Uso

Comprende y rastrea el uso de tokens para facturación en el SDK de Claude Agent

---

# Seguimiento de Costos del SDK

El SDK de Claude Agent proporciona información detallada sobre el uso de tokens para cada interacción con Claude. Esta guía explica cómo rastrear adecuadamente los costos y comprender los informes de uso, especialmente cuando se trata de usos de herramientas en paralelo y conversaciones de múltiples pasos.

Para documentación completa de la API, consulta la [referencia del SDK de TypeScript](https://code.claude.com/docs/typescript-sdk-reference).

## Comprendiendo el Uso de Tokens

Cuando Claude procesa solicitudes, reporta el uso de tokens a nivel de mensaje. Estos datos de uso son esenciales para rastrear costos y facturar a los usuarios apropiadamente.

### Conceptos Clave

1. **Pasos**: Un paso es un par único de solicitud/respuesta entre tu aplicación y Claude
2. **Mensajes**: Mensajes individuales dentro de un paso (texto, usos de herramientas, resultados de herramientas)
3. **Uso**: Datos de consumo de tokens adjuntos a mensajes del asistente

## Estructura de Informes de Uso

### Uso de Herramientas Individual vs Paralelo

Cuando Claude ejecuta herramientas, los informes de uso difieren según si las herramientas se ejecutan secuencialmente o en paralelo:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Ejemplo: Rastreando uso en una conversación
const result = await query({
  prompt: "Analiza esta base de código y ejecuta pruebas",
  options: {
    onMessage: (message) => {
      if (message.type === 'assistant' && message.usage) {
        console.log(`ID del Mensaje: ${message.id}`);
        console.log(`Uso:`, message.usage);
      }
    }
  }
});
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage
import asyncio

# Ejemplo: Rastreando uso en una conversación
async def track_usage():
    # Procesar mensajes conforme llegan
    async for message in query(
        prompt="Analiza esta base de código y ejecuta pruebas"
    ):
        if isinstance(message, AssistantMessage) and hasattr(message, 'usage'):
            print(f"ID del Mensaje: {message.id}")
            print(f"Uso: {message.usage}")

asyncio.run(track_usage())
```

</CodeGroup>

### Ejemplo de Flujo de Mensajes

Así es como se reportan los mensajes y el uso en una conversación típica de múltiples pasos:

```
<!-- Paso 1: Solicitud inicial con usos de herramientas en paralelo -->
assistant (text)      { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
user (tool_result)
user (tool_result)
user (tool_result)

<!-- Paso 2: Respuesta de seguimiento -->
assistant (text)      { id: "msg_2", usage: { output_tokens: 98, ... } }
```

## Reglas Importantes de Uso

### 1. Mismo ID = Mismo Uso

**Todos los mensajes con el mismo campo `id` reportan uso idéntico**. Cuando Claude envía múltiples mensajes en el mismo turno (ej., texto + usos de herramientas), comparten el mismo ID de mensaje y datos de uso.

```typescript
// Todos estos mensajes tienen el mismo ID y uso
const messages = [
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } }
];

// Cobrar solo una vez por ID de mensaje único
const uniqueUsage = messages[0].usage; // Igual para todos los mensajes con este ID
```

### 2. Cobrar Una Vez Por Paso

**Debes cobrar a los usuarios solo una vez por paso**, no por cada mensaje individual. Cuando veas múltiples mensajes del asistente con el mismo ID, usa el uso de cualquiera de ellos.

### 3. El Mensaje de Resultado Contiene Uso Acumulativo

El mensaje final `result` contiene el uso acumulativo total de todos los pasos en la conversación:

```typescript
// El resultado final incluye el uso total
const result = await query({
  prompt: "Tarea de múltiples pasos",
  options: { /* ... */ }
});

console.log("Uso total:", result.usage);
console.log("Costo total:", result.usage.total_cost_usd);
```

## Implementación: Sistema de Seguimiento de Costos

Aquí hay un ejemplo completo de implementación de un sistema de seguimiento de costos:

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
    // Solo procesar mensajes del asistente con uso
    if (message.type !== 'assistant' || !message.usage) {
      return;
    }
    
    // Omitir si ya hemos procesado este ID de mensaje
    if (this.processedMessageIds.has(message.id)) {
      return;
    }
    
    // Marcar como procesado y registrar uso
    this.processedMessageIds.add(message.id);
    this.stepUsages.push({
      messageId: message.id,
      timestamp: new Date().toISOString(),
      usage: message.usage,
      costUSD: this.calculateCost(message.usage)
    });
  }
  
  private calculateCost(usage: any): number {
    // Implementa tu cálculo de precios aquí
    // Este es un ejemplo simplificado
    const inputCost = usage.input_tokens * 0.00003;
    const outputCost = usage.output_tokens * 0.00015;
    const cacheReadCost = (usage.cache_read_input_tokens || 0) * 0.0000075;
    
    return inputCost + outputCost + cacheReadCost;
  }
}

// Uso
const tracker = new CostTracker();
const { result, stepUsages, totalCost } = await tracker.trackConversation(
  "Analiza y refactoriza este código"
);

console.log(`Pasos procesados: ${stepUsages.length}`);
console.log(`Costo total: $${totalCost.toFixed(4)}`);
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

        # Procesar mensajes conforme llegan
        async for message in query(prompt=prompt):
            self.process_message(message)

            # Capturar el mensaje de resultado final
            if isinstance(message, ResultMessage):
                result = message

        return {
            "result": result,
            "step_usages": self.step_usages,
            "total_cost": result.total_cost_usd if result else 0
        }

    def process_message(self, message):
        # Solo procesar mensajes del asistente con uso
        if not isinstance(message, AssistantMessage) or not hasattr(message, 'usage'):
            return

        # Omitir si ya se procesó este ID de mensaje
        message_id = getattr(message, 'id', None)
        if not message_id or message_id in self.processed_message_ids:
            return

        # Marcar como procesado y registrar uso
        self.processed_message_ids.add(message_id)
        self.step_usages.append({
            "message_id": message_id,
            "timestamp": datetime.now().isoformat(),
            "usage": message.usage,
            "cost_usd": self.calculate_cost(message.usage)
        })

    def calculate_cost(self, usage):
        # Implementa tu cálculo de precios
        input_cost = usage.get("input_tokens", 0) * 0.00003
        output_cost = usage.get("output_tokens", 0) * 0.00015
        cache_read_cost = usage.get("cache_read_input_tokens", 0) * 0.0000075

        return input_cost + output_cost + cache_read_cost

# Uso
async def main():
    tracker = CostTracker()
    result = await tracker.track_conversation("Analiza y refactoriza este código")

    print(f"Pasos procesados: {len(result['step_usages'])}")
    print(f"Costo total: ${result['total_cost']:.4f}")

asyncio.run(main())
```

</CodeGroup>

## Manejando Casos Extremos

### Discrepancias en Tokens de Salida

En casos raros, podrías observar diferentes valores de `output_tokens` para mensajes con el mismo ID. Cuando esto ocurra:

1. **Usa el valor más alto** - El mensaje final en un grupo típicamente contiene el total preciso
2. **Verifica contra el costo total** - El `total_cost_usd` en el mensaje de resultado es autoritativo
3. **Reporta inconsistencias** - Presenta problemas en el [repositorio de GitHub de Claude Code](https://github.com/anthropics/claude-code/issues)

### Seguimiento de Tokens de Caché

Cuando uses caché de prompts, rastrea estos tipos de tokens por separado:

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

## Mejores Prácticas

1. **Usa IDs de Mensaje para Deduplicación**: Siempre rastrea IDs de mensajes procesados para evitar cobros dobles
2. **Monitorea el Mensaje de Resultado**: El resultado final contiene uso acumulativo autoritativo
3. **Implementa Registro**: Registra todos los datos de uso para auditoría y depuración
4. **Maneja Fallas Graciosamente**: Rastrea uso parcial incluso si una conversación falla
5. **Considera Streaming**: Para respuestas en streaming, acumula uso conforme llegan los mensajes

## Referencia de Campos de Uso

Cada objeto de uso contiene:

- `input_tokens`: Tokens de entrada base procesados
- `output_tokens`: Tokens generados en la respuesta
- `cache_creation_input_tokens`: Tokens usados para crear entradas de caché
- `cache_read_input_tokens`: Tokens leídos del caché
- `service_tier`: El nivel de servicio usado (ej., "standard")
- `total_cost_usd`: Costo total en USD (solo en mensaje de resultado)

## Ejemplo: Construyendo un Panel de Facturación

Así es como agregar datos de uso para un panel de facturación:

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
    
    // Actualizar totales del usuario
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

## Documentación Relacionada

- [Referencia del SDK de TypeScript](https://code.claude.com/docs/typescript-sdk-reference) - Documentación completa de la API
- [Resumen del SDK](/docs/es/agent-sdk/overview) - Comenzando con el SDK
- [Permisos del SDK](/docs/es/agent-sdk/permissions) - Gestionando permisos de herramientas