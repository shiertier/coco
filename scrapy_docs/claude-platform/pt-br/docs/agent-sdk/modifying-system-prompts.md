# Modificando prompts do sistema

Aprenda como personalizar o comportamento do Claude modificando prompts do sistema usando três abordagens - estilos de saída, systemPrompt com append e prompts de sistema personalizados.

---

Os prompts do sistema definem o comportamento, capacidades e estilo de resposta do Claude. O Claude Agent SDK fornece três maneiras de personalizar prompts do sistema: usando estilos de saída (configurações persistentes baseadas em arquivos), anexando ao prompt do Claude Code ou usando um prompt totalmente personalizado.

## Entendendo prompts do sistema

Um prompt do sistema é o conjunto inicial de instruções que molda como o Claude se comporta ao longo de uma conversa.

<Note>
**Comportamento padrão:** O Agent SDK usa um **prompt do sistema vazio** por padrão para máxima flexibilidade. Para usar o prompt do sistema do Claude Code (instruções de ferramentas, diretrizes de código, etc.), especifique `systemPrompt: { preset: "claude_code" }` em TypeScript ou `system_prompt="claude_code"` em Python.
</Note>

O prompt do sistema do Claude Code inclui:

- Instruções de uso de ferramentas e ferramentas disponíveis
- Diretrizes de estilo e formatação de código
- Configurações de tom de resposta e verbosidade
- Instruções de segurança e proteção
- Contexto sobre o diretório de trabalho atual e ambiente

## Métodos de modificação

### Método 1: Arquivos CLAUDE.md (instruções em nível de projeto)

Os arquivos CLAUDE.md fornecem contexto e instruções específicas do projeto que são automaticamente lidas pelo Agent SDK quando ele é executado em um diretório. Eles servem como "memória" persistente para seu projeto.

#### Como o CLAUDE.md funciona com o SDK

**Localização e descoberta:**

- **Nível de projeto:** `CLAUDE.md` ou `.claude/CLAUDE.md` em seu diretório de trabalho
- **Nível de usuário:** `~/.claude/CLAUDE.md` para instruções globais em todos os projetos

**IMPORTANTE:** O SDK só lê arquivos CLAUDE.md quando você configura explicitamente `settingSources` (TypeScript) ou `setting_sources` (Python):

- Inclua `'project'` para carregar CLAUDE.md em nível de projeto
- Inclua `'user'` para carregar CLAUDE.md em nível de usuário (`~/.claude/CLAUDE.md`)

O preset do prompt do sistema `claude_code` NÃO carrega automaticamente CLAUDE.md - você também deve especificar fontes de configuração.

**Formato do conteúdo:**
Os arquivos CLAUDE.md usam markdown simples e podem conter:

- Diretrizes e padrões de codificação
- Contexto específico do projeto
- Comandos ou fluxos de trabalho comuns
- Convenções de API
- Requisitos de teste

#### Exemplo de CLAUDE.md

```markdown
# Diretrizes do Projeto

## Estilo de Código

- Use modo estrito do TypeScript
- Prefira componentes funcionais no React
- Sempre inclua comentários JSDoc para APIs públicas

## Testes

- Execute `npm test` antes de fazer commit
- Mantenha >80% de cobertura de código
- Use jest para testes unitários, playwright para E2E

## Comandos

- Build: `npm run build`
- Servidor dev: `npm run dev`
- Verificação de tipo: `npm run typecheck`
```

#### Usando CLAUDE.md com o SDK

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// IMPORTANTE: Você deve especificar settingSources para carregar CLAUDE.md
// O preset claude_code sozinho NÃO carrega arquivos CLAUDE.md
const messages = [];

for await (const message of query({
  prompt: "Adicione um novo componente React para perfis de usuário",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code", // Use o prompt do sistema do Claude Code
    },
    settingSources: ["project"], // Necessário para carregar CLAUDE.md do projeto
  },
})) {
  messages.push(message);
}

// Agora o Claude tem acesso às suas diretrizes de projeto do CLAUDE.md
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# IMPORTANTE: Você deve especificar setting_sources para carregar CLAUDE.md
# O preset claude_code sozinho NÃO carrega arquivos CLAUDE.md
messages = []

async for message in query(
    prompt="Adicione um novo componente React para perfis de usuário",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code"  # Use o prompt do sistema do Claude Code
        },
        setting_sources=["project"]  # Necessário para carregar CLAUDE.md do projeto
    )
):
    messages.append(message)

# Agora o Claude tem acesso às suas diretrizes de projeto do CLAUDE.md
```

</CodeGroup>

#### Quando usar CLAUDE.md

**Melhor para:**

- **Contexto compartilhado da equipe** - Diretrizes que todos devem seguir
- **Convenções do projeto** - Padrões de codificação, estrutura de arquivos, padrões de nomenclatura
- **Comandos comuns** - Comandos de build, teste, deploy específicos do seu projeto
- **Memória de longo prazo** - Contexto que deve persistir em todas as sessões
- **Instruções controladas por versão** - Commit no git para que a equipe permaneça sincronizada

**Características principais:**

- ✅ Persistente em todas as sessões em um projeto
- ✅ Compartilhado com a equipe via git
- ✅ Descoberta automática (nenhuma mudança de código necessária)
- ⚠️ Requer carregamento de configurações via `settingSources`

### Método 2: Estilos de saída (configurações persistentes)

Os estilos de saída são configurações salvas que modificam o prompt do sistema do Claude. Eles são armazenados como arquivos markdown e podem ser reutilizados em sessões e projetos.

#### Criando um estilo de saída

<CodeGroup>

```typescript TypeScript
import { writeFile, mkdir } from "fs/promises";
import { join } from "path";
import { homedir } from "os";

async function createOutputStyle(
  name: string,
  description: string,
  prompt: string
) {
  // Nível de usuário: ~/.claude/output-styles
  // Nível de projeto: .claude/output-styles
  const outputStylesDir = join(homedir(), ".claude", "output-styles");

  await mkdir(outputStylesDir, { recursive: true });

  const content = `---
name: ${name}
description: ${description}
---

${prompt}`;

  const filePath = join(
    outputStylesDir,
    `${name.toLowerCase().replace(/\s+/g, "-")}.md`
  );
  await writeFile(filePath, content, "utf-8");
}

// Exemplo: Criar um especialista em revisão de código
await createOutputStyle(
  "Code Reviewer",
  "Assistente de revisão de código completa",
  `Você é um revisor de código especialista.

Para cada submissão de código:
1. Verifique bugs e problemas de segurança
2. Avalie performance
3. Sugira melhorias
4. Classifique a qualidade do código (1-10)`
);
```

```python Python
from pathlib import Path

async def create_output_style(name: str, description: str, prompt: str):
    # Nível de usuário: ~/.claude/output-styles
    # Nível de projeto: .claude/output-styles
    output_styles_dir = Path.home() / '.claude' / 'output-styles'

    output_styles_dir.mkdir(parents=True, exist_ok=True)

    content = f"""---
name: {name}
description: {description}
---

{prompt}"""

    file_name = name.lower().replace(' ', '-') + '.md'
    file_path = output_styles_dir / file_name
    file_path.write_text(content, encoding='utf-8')

# Exemplo: Criar um especialista em revisão de código
await create_output_style(
    'Code Reviewer',
    'Assistente de revisão de código completa',
    """Você é um revisor de código especialista.

Para cada submissão de código:
1. Verifique bugs e problemas de segurança
2. Avalie performance
3. Sugira melhorias
4. Classifique a qualidade do código (1-10)"""
)
```

</CodeGroup>

#### Usando estilos de saída

Uma vez criados, ative estilos de saída via:

- **CLI**: `/output-style [nome-do-estilo]`
- **Configurações**: `.claude/settings.local.json`
- **Criar novo**: `/output-style:new [descrição]`

**Nota para usuários do SDK:** Os estilos de saída são carregados quando você inclui `settingSources: ['user']` ou `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` ou `setting_sources=["project"]` (Python) em suas opções.

### Método 3: Usando `systemPrompt` com append

Você pode usar o preset do Claude Code com uma propriedade `append` para adicionar suas instruções personalizadas enquanto preserva toda a funcionalidade integrada.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const messages = [];

for await (const message of query({
  prompt: "Me ajude a escrever uma função Python para calcular números fibonacci",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append:
        "Sempre inclua docstrings detalhadas e type hints no código Python.",
    },
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

messages = []

async for message in query(
    prompt="Me ajude a escrever uma função Python para calcular números fibonacci",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": "Sempre inclua docstrings detalhadas e type hints no código Python."
        }
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

### Método 4: Prompts de sistema personalizados

Você pode fornecer uma string personalizada como `systemPrompt` para substituir completamente o padrão por suas próprias instruções.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const customPrompt = `Você é um especialista em codificação Python.
Siga estas diretrizes:
- Escreva código limpo e bem documentado
- Use type hints para todas as funções
- Inclua docstrings abrangentes
- Prefira padrões de programação funcional quando apropriado
- Sempre explique suas escolhas de código`;

const messages = [];

for await (const message of query({
  prompt: "Crie um pipeline de processamento de dados",
  options: {
    systemPrompt: customPrompt,
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

custom_prompt = """Você é um especialista em codificação Python.
Siga estas diretrizes:
- Escreva código limpo e bem documentado
- Use type hints para todas as funções
- Inclua docstrings abrangentes
- Prefira padrões de programação funcional quando apropriado
- Sempre explique suas escolhas de código"""

messages = []

async for message in query(
    prompt="Crie um pipeline de processamento de dados",
    options=ClaudeAgentOptions(
        system_prompt=custom_prompt
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

## Comparação de todas as quatro abordagens

| Recurso                 | CLAUDE.md           | Estilos de Saída   | `systemPrompt` com append | `systemPrompt` Personalizado |
| --- | --- | --- | --- | --- |
| **Persistência**        | Arquivo por projeto | Salvos como arquivos | Apenas sessão            | Apenas sessão           |
| **Reutilização**        | Por projeto         | Entre projetos      | Duplicação de código     | Duplicação de código    |
| **Gerenciamento**       | No sistema de arquivos | CLI + arquivos   | No código                | No código               |
| **Ferramentas padrão**  | Preservadas         | Preservadas         | Preservadas              | Perdidas (a menos que incluídas) |
| **Segurança integrada** | Mantida             | Mantida             | Mantida                  | Deve ser adicionada     |
| **Contexto do ambiente** | Automático         | Automático          | Automático               | Deve ser fornecido      |
| **Nível de personalização** | Apenas adições  | Substituir padrão   | Apenas adições           | Controle completo       |
| **Controle de versão**  | Com projeto         | Sim                 | Com código               | Com código              |
| **Escopo**              | Específico do projeto | Usuário ou projeto | Sessão de código         | Sessão de código        |

**Nota:** "Com append" significa usar `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` em TypeScript ou `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` em Python.

## Casos de uso e melhores práticas

### Quando usar CLAUDE.md

**Melhor para:**

- Padrões e convenções de codificação específicos do projeto
- Documentar estrutura e arquitetura do projeto
- Listar comandos comuns (build, teste, deploy)
- Contexto compartilhado da equipe que deve ser controlado por versão
- Instruções que se aplicam a todo uso do SDK em um projeto

**Exemplos:**

- "Todos os endpoints da API devem usar padrões async/await"
- "Execute `npm run lint:fix` antes de fazer commit"
- "Migrações de banco de dados estão no diretório `migrations/`"

**Importante:** Para carregar arquivos CLAUDE.md, você deve definir explicitamente `settingSources: ['project']` (TypeScript) ou `setting_sources=["project"]` (Python). O preset do prompt do sistema `claude_code` NÃO carrega automaticamente CLAUDE.md sem esta configuração.

### Quando usar estilos de saída

**Melhor para:**

- Mudanças de comportamento persistentes entre sessões
- Configurações compartilhadas da equipe
- Assistentes especializados (revisor de código, cientista de dados, DevOps)
- Modificações de prompt complexas que precisam de versionamento

**Exemplos:**

- Criar um assistente dedicado de otimização SQL
- Construir um revisor de código focado em segurança
- Desenvolver um assistente de ensino com pedagogia específica

### Quando usar `systemPrompt` com append

**Melhor para:**

- Adicionar padrões ou preferências de codificação específicas
- Personalizar formatação de saída
- Adicionar conhecimento específico do domínio
- Modificar verbosidade da resposta
- Melhorar o comportamento padrão do Claude Code sem perder instruções de ferramentas

### Quando usar `systemPrompt` personalizado

**Melhor para:**

- Controle completo sobre o comportamento do Claude
- Tarefas especializadas de sessão única
- Testar novas estratégias de prompt
- Situações onde ferramentas padrão não são necessárias
- Construir agentes especializados com comportamento único

## Combinando abordagens

Você pode combinar esses métodos para máxima flexibilidade:

### Exemplo: Estilo de saída com adições específicas da sessão

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Assumindo que o estilo de saída "Code Reviewer" está ativo (via /output-style)
// Adicionar áreas de foco específicas da sessão
const messages = [];

for await (const message of query({
  prompt: "Revise este módulo de autenticação",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: `
        Para esta revisão, priorize:
        - Conformidade com OAuth 2.0
        - Segurança de armazenamento de token
        - Gerenciamento de sessão
      `,
    },
  },
})) {
  messages.push(message);
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# Assumindo que o estilo de saída "Code Reviewer" está ativo (via /output-style)
# Adicionar áreas de foco específicas da sessão
messages = []

async for message in query(
    prompt="Revise este módulo de autenticação",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": """
            Para esta revisão, priorize:
            - Conformidade com OAuth 2.0
            - Segurança de armazenamento de token
            - Gerenciamento de sessão
            """
        }
    )
):
    messages.append(message)
```

</CodeGroup>

## Veja também

- [Estilos de saída](https://code.claude.com/docs/output-styles) - Documentação completa de estilos de saída
- [Guia do SDK TypeScript](/docs/pt-BR/agent-sdk/typescript) - Guia completo de uso do SDK
- [Referência do SDK TypeScript](https://code.claude.com/docs/typescript-sdk-reference) - Documentação completa da API
- [Guia de configuração](https://code.claude.com/docs/configuration) - Opções gerais de configuração