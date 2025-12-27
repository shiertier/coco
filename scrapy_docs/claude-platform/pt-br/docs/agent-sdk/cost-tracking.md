# Rastreamento de Custos e Uso

Entenda e rastreie o uso de tokens para faturamento no Claude Agent SDK

---

# Rastreamento de Custos do SDK

O Claude Agent SDK fornece informações detalhadas sobre o uso de tokens para cada interação com Claude. Este guia explica como rastrear adequadamente os custos e entender os relatórios de uso, especialmente ao lidar com usos paralelos de ferramentas e conversas de múltiplas etapas.

Para documentação completa da API, consulte a [referência do SDK TypeScript](https://code.claude.com/docs/typescript-sdk-reference).

## Entendendo o Uso de Tokens

Quando Claude processa solicitações, ele relata o uso de tokens no nível da mensagem. Esses dados de uso são essenciais para rastrear custos e faturar usuários adequadamente.

### Conceitos Principais

1. **Etapas**: Uma etapa é um único par de solicitação/resposta entre sua aplicação e Claude
2. **Mensagens**: Mensagens individuais dentro de uma etapa (texto, usos de ferramentas, resultados de ferramentas)
3. **Uso**: Dados de consumo de tokens anexados às mensagens do assistente

## Estrutura de Relatórios de Uso

### Uso Único vs Paralelo de Ferramentas

Quando Claude executa ferramentas, os relatórios de uso diferem com base em se as ferramentas são executadas sequencialmente ou em paralelo:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Exemplo: Rastreando uso em uma conversa
const result = await query({
  prompt: "Analise esta base de código e execute testes",
  options: {
    onMessage: (message) => {
      if (message.type === 'assistant' && message.usage) {
        console.log(`ID da Mensagem: ${message.id}`);
        console.log(`Uso:`, message.usage);
      }
    }
  }
});
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage
import asyncio

# Exemplo: Rastreando uso em uma conversa
async def track_usage():
    # Processa mensagens conforme chegam
    async for message in query(
        prompt="Analise esta base de código e execute testes"
    ):
        if isinstance(message, AssistantMessage) and hasattr(message, 'usage'):
            print(f"ID da Mensagem: {message.id}")
            print(f"Uso: {message.usage}")

asyncio.run(track_usage())
```

</CodeGroup>

### Exemplo de Fluxo de Mensagens

Aqui está como as mensagens e o uso são relatados em uma conversa típica de múltiplas etapas:

```
<!-- Etapa 1: Solicitação inicial com usos paralelos de ferramentas -->
assistant (text)      { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
user (tool_result)
user (tool_result)
user (tool_result)

<!-- Etapa 2: Resposta de acompanhamento -->
assistant (text)      { id: "msg_2", usage: { output_tokens: 98, ... } }
```

## Regras Importantes de Uso

### 1. Mesmo ID = Mesmo Uso

**Todas as mensagens com o mesmo campo `id` relatam uso idêntico**. Quando Claude envia múltiplas mensagens no mesmo turno (por exemplo, texto + usos de ferramentas), elas compartilham o mesmo ID de mensagem e dados de uso.

```typescript
// Todas essas mensagens têm o mesmo ID e uso
const messages = [
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } }
];

// Cobre apenas uma vez por ID de mensagem único
const uniqueUsage = messages[0].usage; // Mesmo para todas as mensagens com este ID
```

### 2. Cobre Uma Vez Por Etapa

**Você deve cobrar dos usuários apenas uma vez por etapa**, não para cada mensagem individual. Quando você vê múltiplas mensagens do assistente com o mesmo ID, use o uso de qualquer uma delas.

### 3. Mensagem de Resultado Contém Uso Cumulativo

A mensagem final `result` contém o uso cumulativo total de todas as etapas na conversa:

```typescript
// Resultado final inclui uso total
const result = await query({
  prompt: "Tarefa de múltiplas etapas",
  options: { /* ... */ }
});

console.log("Uso total:", result.usage);
console.log("Custo total:", result.usage.total_cost_usd);
```

## Implementação: Sistema de Rastreamento de Custos

Aqui está um exemplo completo de implementação de um sistema de rastreamento de custos:

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
    // Processa apenas mensagens do assistente com uso
    if (message.type !== 'assistant' || !message.usage) {
      return;
    }
    
    // Pula se já processamos este ID de mensagem
    if (this.processedMessageIds.has(message.id)) {
      return;
    }
    
    // Marca como processado e registra uso
    this.processedMessageIds.add(message.id);
    this.stepUsages.push({
      messageId: message.id,
      timestamp: new Date().toISOString(),
      usage: message.usage,
      costUSD: this.calculateCost(message.usage)
    });
  }
  
  private calculateCost(usage: any): number {
    // Implemente seu cálculo de preços aqui
    // Este é um exemplo simplificado
    const inputCost = usage.input_tokens * 0.00003;
    const outputCost = usage.output_tokens * 0.00015;
    const cacheReadCost = (usage.cache_read_input_tokens || 0) * 0.0000075;
    
    return inputCost + outputCost + cacheReadCost;
  }
}

// Uso
const tracker = new CostTracker();
const { result, stepUsages, totalCost } = await tracker.trackConversation(
  "Analise e refatore este código"
);

console.log(`Etapas processadas: ${stepUsages.length}`);
console.log(`Custo total: $${totalCost.toFixed(4)}`);
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

        # Processa mensagens conforme chegam
        async for message in query(prompt=prompt):
            self.process_message(message)

            # Captura a mensagem de resultado final
            if isinstance(message, ResultMessage):
                result = message

        return {
            "result": result,
            "step_usages": self.step_usages,
            "total_cost": result.total_cost_usd if result else 0
        }

    def process_message(self, message):
        # Processa apenas mensagens do assistente com uso
        if not isinstance(message, AssistantMessage) or not hasattr(message, 'usage'):
            return

        # Pula se já processou este ID de mensagem
        message_id = getattr(message, 'id', None)
        if not message_id or message_id in self.processed_message_ids:
            return

        # Marca como processado e registra uso
        self.processed_message_ids.add(message_id)
        self.step_usages.append({
            "message_id": message_id,
            "timestamp": datetime.now().isoformat(),
            "usage": message.usage,
            "cost_usd": self.calculate_cost(message.usage)
        })

    def calculate_cost(self, usage):
        # Implemente seu cálculo de preços
        input_cost = usage.get("input_tokens", 0) * 0.00003
        output_cost = usage.get("output_tokens", 0) * 0.00015
        cache_read_cost = usage.get("cache_read_input_tokens", 0) * 0.0000075

        return input_cost + output_cost + cache_read_cost

# Uso
async def main():
    tracker = CostTracker()
    result = await tracker.track_conversation("Analise e refatore este código")

    print(f"Etapas processadas: {len(result['step_usages'])}")
    print(f"Custo total: ${result['total_cost']:.4f}")

asyncio.run(main())
```

</CodeGroup>

## Lidando com Casos Extremos

### Discrepâncias de Tokens de Saída

Em casos raros, você pode observar valores diferentes de `output_tokens` para mensagens com o mesmo ID. Quando isso ocorre:

1. **Use o valor mais alto** - A mensagem final em um grupo normalmente contém o total preciso
2. **Verifique contra o custo total** - O `total_cost_usd` na mensagem de resultado é autoritativo
3. **Relate inconsistências** - Registre problemas no [repositório GitHub do Claude Code](https://github.com/anthropics/claude-code/issues)

### Rastreamento de Tokens de Cache

Ao usar cache de prompt, rastreie esses tipos de tokens separadamente:

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

## Melhores Práticas

1. **Use IDs de Mensagem para Desduplicação**: Sempre rastreie IDs de mensagens processadas para evitar cobrança dupla
2. **Monitore a Mensagem de Resultado**: O resultado final contém uso cumulativo autoritativo
3. **Implemente Logging**: Registre todos os dados de uso para auditoria e depuração
4. **Lide com Falhas Graciosamente**: Rastreie uso parcial mesmo se uma conversa falhar
5. **Considere Streaming**: Para respostas em streaming, acumule uso conforme as mensagens chegam

## Referência de Campos de Uso

Cada objeto de uso contém:

- `input_tokens`: Tokens de entrada base processados
- `output_tokens`: Tokens gerados na resposta
- `cache_creation_input_tokens`: Tokens usados para criar entradas de cache
- `cache_read_input_tokens`: Tokens lidos do cache
- `service_tier`: O nível de serviço usado (por exemplo, "standard")
- `total_cost_usd`: Custo total em USD (apenas na mensagem de resultado)

## Exemplo: Construindo um Painel de Faturamento

Aqui está como agregar dados de uso para um painel de faturamento:

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
    
    // Atualiza totais do usuário
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

## Documentação Relacionada

- [Referência do SDK TypeScript](https://code.claude.com/docs/typescript-sdk-reference) - Documentação completa da API
- [Visão Geral do SDK](/docs/pt-BR/agent-sdk/overview) - Começando com o SDK
- [Permissões do SDK](/docs/pt-BR/agent-sdk/permissions) - Gerenciando permissões de ferramentas