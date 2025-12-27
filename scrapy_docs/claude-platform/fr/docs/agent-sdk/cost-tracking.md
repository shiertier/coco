# Suivi des Coûts et de l'Utilisation

Comprendre et suivre l'utilisation des tokens pour la facturation dans le SDK Claude Agent

---

# Suivi des Coûts du SDK

Le SDK Claude Agent fournit des informations détaillées sur l'utilisation des tokens pour chaque interaction avec Claude. Ce guide explique comment suivre correctement les coûts et comprendre les rapports d'utilisation, en particulier lors du traitement d'utilisations d'outils parallèles et de conversations multi-étapes.

Pour la documentation complète de l'API, consultez la [référence du SDK TypeScript](https://code.claude.com/docs/typescript-sdk-reference).

## Comprendre l'Utilisation des Tokens

Lorsque Claude traite les requêtes, il rapporte l'utilisation des tokens au niveau du message. Ces données d'utilisation sont essentielles pour suivre les coûts et facturer les utilisateurs de manière appropriée.

### Concepts Clés

1. **Étapes** : Une étape est une paire unique de requête/réponse entre votre application et Claude
2. **Messages** : Messages individuels au sein d'une étape (texte, utilisations d'outils, résultats d'outils)
3. **Utilisation** : Données de consommation de tokens attachées aux messages de l'assistant

## Structure des Rapports d'Utilisation

### Utilisation d'Outils Unique vs Parallèle

Lorsque Claude exécute des outils, les rapports d'utilisation diffèrent selon que les outils sont exécutés séquentiellement ou en parallèle :

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Exemple : Suivi de l'utilisation dans une conversation
const result = await query({
  prompt: "Analysez cette base de code et exécutez les tests",
  options: {
    onMessage: (message) => {
      if (message.type === 'assistant' && message.usage) {
        console.log(`ID du Message : ${message.id}`);
        console.log(`Utilisation :`, message.usage);
      }
    }
  }
});
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage
import asyncio

# Exemple : Suivi de l'utilisation dans une conversation
async def track_usage():
    # Traiter les messages au fur et à mesure qu'ils arrivent
    async for message in query(
        prompt="Analysez cette base de code et exécutez les tests"
    ):
        if isinstance(message, AssistantMessage) and hasattr(message, 'usage'):
            print(f"ID du Message : {message.id}")
            print(f"Utilisation : {message.usage}")

asyncio.run(track_usage())
```

</CodeGroup>

### Exemple de Flux de Messages

Voici comment les messages et l'utilisation sont rapportés dans une conversation multi-étapes typique :

```
<!-- Étape 1 : Requête initiale avec utilisations d'outils parallèles -->
assistant (text)      { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
user (tool_result)
user (tool_result)
user (tool_result)

<!-- Étape 2 : Réponse de suivi -->
assistant (text)      { id: "msg_2", usage: { output_tokens: 98, ... } }
```

## Règles d'Utilisation Importantes

### 1. Même ID = Même Utilisation

**Tous les messages avec le même champ `id` rapportent une utilisation identique**. Lorsque Claude envoie plusieurs messages dans le même tour (par exemple, texte + utilisations d'outils), ils partagent le même ID de message et les mêmes données d'utilisation.

```typescript
// Tous ces messages ont le même ID et la même utilisation
const messages = [
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } }
];

// Facturer seulement une fois par ID de message unique
const uniqueUsage = messages[0].usage; // Identique pour tous les messages avec cet ID
```

### 2. Facturer Une Fois Par Étape

**Vous ne devriez facturer les utilisateurs qu'une fois par étape**, pas pour chaque message individuel. Lorsque vous voyez plusieurs messages d'assistant avec le même ID, utilisez l'utilisation de n'importe lequel d'entre eux.

### 3. Le Message de Résultat Contient l'Utilisation Cumulative

Le message final `result` contient l'utilisation cumulative totale de toutes les étapes de la conversation :

```typescript
// Le résultat final inclut l'utilisation totale
const result = await query({
  prompt: "Tâche multi-étapes",
  options: { /* ... */ }
});

console.log("Utilisation totale :", result.usage);
console.log("Coût total :", result.usage.total_cost_usd);
```

## Implémentation : Système de Suivi des Coûts

Voici un exemple complet d'implémentation d'un système de suivi des coûts :

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
    // Traiter seulement les messages d'assistant avec utilisation
    if (message.type !== 'assistant' || !message.usage) {
      return;
    }
    
    // Ignorer si nous avons déjà traité cet ID de message
    if (this.processedMessageIds.has(message.id)) {
      return;
    }
    
    // Marquer comme traité et enregistrer l'utilisation
    this.processedMessageIds.add(message.id);
    this.stepUsages.push({
      messageId: message.id,
      timestamp: new Date().toISOString(),
      usage: message.usage,
      costUSD: this.calculateCost(message.usage)
    });
  }
  
  private calculateCost(usage: any): number {
    // Implémentez votre calcul de prix ici
    // Ceci est un exemple simplifié
    const inputCost = usage.input_tokens * 0.00003;
    const outputCost = usage.output_tokens * 0.00015;
    const cacheReadCost = (usage.cache_read_input_tokens || 0) * 0.0000075;
    
    return inputCost + outputCost + cacheReadCost;
  }
}

// Utilisation
const tracker = new CostTracker();
const { result, stepUsages, totalCost } = await tracker.trackConversation(
  "Analysez et refactorisez ce code"
);

console.log(`Étapes traitées : ${stepUsages.length}`);
console.log(`Coût total : $${totalCost.toFixed(4)}`);
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

        # Traiter les messages au fur et à mesure qu'ils arrivent
        async for message in query(prompt=prompt):
            self.process_message(message)

            # Capturer le message de résultat final
            if isinstance(message, ResultMessage):
                result = message

        return {
            "result": result,
            "step_usages": self.step_usages,
            "total_cost": result.total_cost_usd if result else 0
        }

    def process_message(self, message):
        # Traiter seulement les messages d'assistant avec utilisation
        if not isinstance(message, AssistantMessage) or not hasattr(message, 'usage'):
            return

        # Ignorer si déjà traité cet ID de message
        message_id = getattr(message, 'id', None)
        if not message_id or message_id in self.processed_message_ids:
            return

        # Marquer comme traité et enregistrer l'utilisation
        self.processed_message_ids.add(message_id)
        self.step_usages.append({
            "message_id": message_id,
            "timestamp": datetime.now().isoformat(),
            "usage": message.usage,
            "cost_usd": self.calculate_cost(message.usage)
        })

    def calculate_cost(self, usage):
        # Implémentez votre calcul de prix
        input_cost = usage.get("input_tokens", 0) * 0.00003
        output_cost = usage.get("output_tokens", 0) * 0.00015
        cache_read_cost = usage.get("cache_read_input_tokens", 0) * 0.0000075

        return input_cost + output_cost + cache_read_cost

# Utilisation
async def main():
    tracker = CostTracker()
    result = await tracker.track_conversation("Analysez et refactorisez ce code")

    print(f"Étapes traitées : {len(result['step_usages'])}")
    print(f"Coût total : ${result['total_cost']:.4f}")

asyncio.run(main())
```

</CodeGroup>

## Gestion des Cas Particuliers

### Divergences des Tokens de Sortie

Dans de rares cas, vous pourriez observer différentes valeurs `output_tokens` pour des messages avec le même ID. Lorsque cela se produit :

1. **Utilisez la valeur la plus élevée** - Le dernier message d'un groupe contient généralement le total précis
2. **Vérifiez contre le coût total** - Le `total_cost_usd` dans le message de résultat fait autorité
3. **Signalez les incohérences** - Déposez des problèmes sur le [dépôt GitHub Claude Code](https://github.com/anthropics/claude-code/issues)

### Suivi des Tokens de Cache

Lors de l'utilisation de la mise en cache des prompts, suivez ces types de tokens séparément :

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

## Meilleures Pratiques

1. **Utilisez les ID de Messages pour la Déduplication** : Suivez toujours les ID de messages traités pour éviter la double facturation
2. **Surveillez le Message de Résultat** : Le résultat final contient l'utilisation cumulative faisant autorité
3. **Implémentez la Journalisation** : Enregistrez toutes les données d'utilisation pour l'audit et le débogage
4. **Gérez les Échecs avec Élégance** : Suivez l'utilisation partielle même si une conversation échoue
5. **Considérez le Streaming** : Pour les réponses en streaming, accumulez l'utilisation au fur et à mesure que les messages arrivent

## Référence des Champs d'Utilisation

Chaque objet d'utilisation contient :

- `input_tokens` : Tokens d'entrée de base traités
- `output_tokens` : Tokens générés dans la réponse
- `cache_creation_input_tokens` : Tokens utilisés pour créer des entrées de cache
- `cache_read_input_tokens` : Tokens lus depuis le cache
- `service_tier` : Le niveau de service utilisé (par exemple, "standard")
- `total_cost_usd` : Coût total en USD (seulement dans le message de résultat)

## Exemple : Construction d'un Tableau de Bord de Facturation

Voici comment agréger les données d'utilisation pour un tableau de bord de facturation :

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
    
    // Mettre à jour les totaux utilisateur
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

## Documentation Connexe

- [Référence du SDK TypeScript](https://code.claude.com/docs/typescript-sdk-reference) - Documentation complète de l'API
- [Aperçu du SDK](/docs/fr/agent-sdk/overview) - Commencer avec le SDK
- [Permissions du SDK](/docs/fr/agent-sdk/permissions) - Gestion des permissions d'outils