# Subagentes no SDK

Trabalhando com subagentes no Claude Agent SDK

---

Subagentes no Claude Agent SDK são IAs especializadas que são orquestradas pelo agente principal.
Use subagentes para gerenciamento de contexto e paralelização.

Este guia explica como definir e usar subagentes no SDK usando o parâmetro `agents`.

## Visão Geral

Subagentes podem ser definidos de duas maneiras ao usar o SDK:

1. **Programaticamente** - Usando o parâmetro `agents` nas suas opções de `query()` (recomendado para aplicações SDK)
2. **Baseado em sistema de arquivos** - Colocando arquivos markdown com frontmatter YAML em diretórios designados (`.claude/agents/`)

Este guia foca principalmente na abordagem programática usando o parâmetro `agents`, que fornece uma experiência de desenvolvimento mais integrada para aplicações SDK.

## Benefícios de Usar Subagentes

### Gerenciamento de Contexto
Subagentes mantêm contexto separado do agente principal, prevenindo sobrecarga de informações e mantendo interações focadas. Este isolamento garante que tarefas especializadas não poluam o contexto da conversa principal com detalhes irrelevantes.

**Exemplo**: Um subagente `research-assistant` pode explorar dezenas de arquivos e páginas de documentação sem bagunçar a conversa principal com todos os resultados de busca intermediários - retornando apenas os achados relevantes.

### Paralelização
Múltiplos subagentes podem executar simultaneamente, acelerando dramaticamente fluxos de trabalho complexos.

**Exemplo**: Durante uma revisão de código, você pode executar subagentes `style-checker`, `security-scanner`, e `test-coverage` simultaneamente, reduzindo o tempo de revisão de minutos para segundos.

### Instruções e Conhecimento Especializados
Cada subagente pode ter prompts de sistema personalizados com expertise específica, melhores práticas e restrições.

**Exemplo**: Um subagente `database-migration` pode ter conhecimento detalhado sobre melhores práticas SQL, estratégias de rollback e verificações de integridade de dados que seriam ruído desnecessário nas instruções do agente principal.

### Restrições de Ferramentas
Subagentes podem ser limitados a ferramentas específicas, reduzindo o risco de ações não intencionais.

**Exemplo**: Um subagente `doc-reviewer` pode ter acesso apenas às ferramentas Read e Grep, garantindo que ele possa analisar mas nunca modificar acidentalmente seus arquivos de documentação.

## Criando Subagentes

### Definição Programática (Recomendada)

Defina subagentes diretamente no seu código usando o parâmetro `agents`:

```typescript
import { query } from '@anthropic-ai/claude-agent-sdk';

const result = query({
  prompt: "Revise o módulo de autenticação para problemas de segurança",
  options: {
    agents: {
      'code-reviewer': {
        description: 'Especialista em revisão de código. Use para revisões de qualidade, segurança e manutenibilidade.',
        prompt: `Você é um especialista em revisão de código com expertise em segurança, performance e melhores práticas.

Ao revisar código:
- Identifique vulnerabilidades de segurança
- Verifique problemas de performance
- Verifique aderência aos padrões de codificação
- Sugira melhorias específicas

Seja minucioso mas conciso no seu feedback.`,
        tools: ['Read', 'Grep', 'Glob'],
        model: 'sonnet'
      },
      'test-runner': {
        description: 'Executa e analisa suítes de teste. Use para execução de testes e análise de cobertura.',
        prompt: `Você é um especialista em execução de testes. Execute testes e forneça análise clara dos resultados.

Foque em:
- Executar comandos de teste
- Analisar saída de testes
- Identificar testes que falharam
- Sugerir correções para falhas`,
        tools: ['Bash', 'Read', 'Grep'],
      }
    }
  }
});

for await (const message of result) {
  console.log(message);
}
```

### Configuração AgentDefinition

| Campo | Tipo | Obrigatório | Descrição |
|:---|:---|:---|:---|
| `description` | `string` | Sim | Descrição em linguagem natural de quando usar este agente |
| `prompt` | `string` | Sim | O prompt de sistema do agente definindo seu papel e comportamento |
| `tools` | `string[]` | Não | Array de nomes de ferramentas permitidas. Se omitido, herda todas as ferramentas |
| `model` | `'sonnet' \| 'opus' \| 'haiku' \| 'inherit'` | Não | Substituição de modelo para este agente. Padrão para modelo principal se omitido |

### Definição Baseada em Sistema de Arquivos (Alternativa)

Você também pode definir subagentes como arquivos markdown em diretórios específicos:

- **Nível de projeto**: `.claude/agents/*.md` - Disponível apenas no projeto atual
- **Nível de usuário**: `~/.claude/agents/*.md` - Disponível em todos os projetos

Cada subagente é um arquivo markdown com frontmatter YAML:

```markdown
---
name: code-reviewer
description: Especialista em revisão de código. Use para revisões de qualidade, segurança e manutenibilidade.
tools: Read, Grep, Glob, Bash
---

O prompt de sistema do seu subagente vai aqui. Isso define o papel do subagente,
capacidades e abordagem para resolver problemas.
```

**Nota:** Agentes definidos programaticamente (via parâmetro `agents`) têm precedência sobre agentes baseados em sistema de arquivos com o mesmo nome.

## Como o SDK Usa Subagentes

Ao usar o Claude Agent SDK, subagentes podem ser definidos programaticamente ou carregados do sistema de arquivos. Claude irá:

1. **Carregar agentes programáticos** do parâmetro `agents` nas suas opções
2. **Auto-detectar agentes do sistema de arquivos** dos diretórios `.claude/agents/` (se não substituído)
3. **Invocá-los automaticamente** baseado na correspondência de tarefas e na `description` do agente
4. **Usar seus prompts especializados** e restrições de ferramentas
5. **Manter contexto separado** para cada invocação de subagente

Agentes definidos programaticamente (via parâmetro `agents`) têm precedência sobre agentes baseados em sistema de arquivos com o mesmo nome.

## Exemplo de Subagentes

Para exemplos abrangentes de subagentes incluindo revisores de código, executores de teste, debuggers e auditores de segurança, veja o [guia principal de Subagentes](https://code.claude.com/docs/sub-agents#example-subagents). O guia inclui configurações detalhadas e melhores práticas para criar subagentes eficazes.

## Padrões de Integração SDK

### Invocação Automática

O SDK irá automaticamente invocar subagentes apropriados baseado no contexto da tarefa. Garanta que o campo `description` do seu agente indique claramente quando ele deve ser usado:

```typescript
const result = query({
  prompt: "Otimize as consultas de banco de dados na camada de API",
  options: {
    agents: {
      'performance-optimizer': {
        description: 'Use PROATIVAMENTE quando mudanças de código podem impactar performance. DEVE SER USADO para tarefas de otimização.',
        prompt: 'Você é um especialista em otimização de performance...',
        tools: ['Read', 'Edit', 'Bash', 'Grep'],
        model: 'sonnet'
      }
    }
  }
});
```

### Invocação Explícita

Usuários podem solicitar subagentes específicos nos seus prompts:

```typescript
const result = query({
  prompt: "Use o agente code-reviewer para verificar o módulo de autenticação",
  options: {
    agents: {
      'code-reviewer': {
        description: 'Especialista em revisão de código',
        prompt: 'Você é um revisor de código focado em segurança...',
        tools: ['Read', 'Grep', 'Glob']
      }
    }
  }
});
```

### Configuração Dinâmica de Agente

Você pode configurar agentes dinamicamente baseado nas necessidades da sua aplicação:

```typescript
import { query, type AgentDefinition } from '@anthropic-ai/claude-agent-sdk';

function createSecurityAgent(securityLevel: 'basic' | 'strict'): AgentDefinition {
  return {
    description: 'Revisor de código de segurança',
    prompt: `Você é um revisor de segurança ${securityLevel === 'strict' ? 'rigoroso' : 'equilibrado'}...`,
    tools: ['Read', 'Grep', 'Glob'],
    model: securityLevel === 'strict' ? 'opus' : 'sonnet'
  };
}

const result = query({
  prompt: "Revise este PR para problemas de segurança",
  options: {
    agents: {
      'security-reviewer': createSecurityAgent('strict')
    }
  }
});
```

## Restrições de Ferramentas

Subagentes podem ter acesso restrito a ferramentas via campo `tools`:

- **Omitir o campo** - Agente herda todas as ferramentas disponíveis (padrão)
- **Especificar ferramentas** - Agente pode usar apenas ferramentas listadas

Exemplo de um agente de análise somente leitura:

```typescript
const result = query({
  prompt: "Analise a arquitetura desta base de código",
  options: {
    agents: {
      'code-analyzer': {
        description: 'Análise estática de código e revisão de arquitetura',
        prompt: `Você é um analista de arquitetura de código. Analise estrutura de código,
identifique padrões e sugira melhorias sem fazer mudanças.`,
        tools: ['Read', 'Grep', 'Glob']  // Sem permissões de escrita ou execução
      }
    }
  }
});
```

### Combinações Comuns de Ferramentas

**Agentes somente leitura** (análise, revisão):
```typescript
tools: ['Read', 'Grep', 'Glob']
```

**Agentes de execução de teste**:
```typescript
tools: ['Bash', 'Read', 'Grep']
```

**Agentes de modificação de código**:
```typescript
tools: ['Read', 'Edit', 'Write', 'Grep', 'Glob']
```

## Documentação Relacionada

- [Guia Principal de Subagentes](https://code.claude.com/docs/sub-agents) - Documentação abrangente de subagentes
- [Visão Geral do SDK](/docs/pt-BR/agent-sdk/overview) - Visão geral do Claude Agent SDK
- [Configurações](https://code.claude.com/docs/settings) - Referência de arquivo de configuração
- [Comandos Slash](https://code.claude.com/docs/slash-commands) - Criação de comandos personalizados