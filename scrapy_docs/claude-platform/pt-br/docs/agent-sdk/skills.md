# Agent Skills no SDK

Estenda Claude com capacidades especializadas usando Agent Skills no Claude Agent SDK

---

## Visão Geral

Agent Skills estendem Claude com capacidades especializadas que Claude invoca autonomamente quando relevante. Skills são empacotadas como arquivos `SKILL.md` contendo instruções, descrições e recursos de suporte opcionais.

Para informações abrangentes sobre Skills, incluindo benefícios, arquitetura e diretrizes de autoria, consulte a [visão geral de Agent Skills](/docs/pt-BR/agents-and-tools/agent-skills/overview).

## Como Skills Funcionam com o SDK

Ao usar o Claude Agent SDK, Skills são:

1. **Definidas como artefatos do sistema de arquivos**: Criadas como arquivos `SKILL.md` em diretórios específicos (`.claude/skills/`)
2. **Carregadas do sistema de arquivos**: Skills são carregadas de locais do sistema de arquivos configurados. Você deve especificar `settingSources` (TypeScript) ou `setting_sources` (Python) para carregar Skills do sistema de arquivos
3. **Descobertas automaticamente**: Uma vez que as configurações do sistema de arquivos são carregadas, os metadados de Skill são descobertos na inicialização a partir de diretórios de usuário e projeto; conteúdo completo carregado quando acionado
4. **Invocadas pelo modelo**: Claude escolhe autonomamente quando usá-las com base no contexto
5. **Habilitadas via allowed_tools**: Adicione `"Skill"` ao seu `allowed_tools` para habilitar Skills

Diferentemente de subagentes (que podem ser definidos programaticamente), Skills devem ser criadas como artefatos do sistema de arquivos. O SDK não fornece uma API programática para registrar Skills.

<Note>
**Comportamento padrão**: Por padrão, o SDK não carrega nenhuma configuração do sistema de arquivos. Para usar Skills, você deve configurar explicitamente `settingSources: ['user', 'project']` (TypeScript) ou `setting_sources=["user", "project"]` (Python) em suas opções.
</Note>

## Usando Skills com o SDK

Para usar Skills com o SDK, você precisa:

1. Incluir `"Skill"` em sua configuração `allowed_tools`
2. Configurar `settingSources`/`setting_sources` para carregar Skills do sistema de arquivos

Uma vez configurado, Claude descobre automaticamente Skills dos diretórios especificados e os invoca quando relevante para a solicitação do usuário.

<CodeGroup>

```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions

async def main():
    options = ClaudeAgentOptions(
        cwd="/path/to/project",  # Project with .claude/skills/
        setting_sources=["user", "project"],  # Load Skills from filesystem
        allowed_tools=["Skill", "Read", "Write", "Bash"]  # Enable Skill tool
    )

    async for message in query(
        prompt="Help me process this PDF document",
        options=options
    ):
        print(message)

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me process this PDF document",
  options: {
    cwd: "/path/to/project",  // Project with .claude/skills/
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Write", "Bash"]  // Enable Skill tool
  }
})) {
  console.log(message);
}
```

</CodeGroup>

## Locais de Skill

Skills são carregadas de diretórios do sistema de arquivos com base em sua configuração `settingSources`/`setting_sources`:

- **Project Skills** (`.claude/skills/`): Compartilhadas com sua equipe via git - carregadas quando `setting_sources` inclui `"project"`
- **User Skills** (`~/.claude/skills/`): Skills pessoais em todos os projetos - carregadas quando `setting_sources` inclui `"user"`
- **Plugin Skills**: Agrupadas com plugins Claude Code instalados

## Criando Skills

Skills são definidas como diretórios contendo um arquivo `SKILL.md` com frontmatter YAML e conteúdo Markdown. O campo `description` determina quando Claude invoca sua Skill.

**Exemplo de estrutura de diretório**:
```bash
.claude/skills/processing-pdfs/
└── SKILL.md
```

Para orientação completa sobre como criar Skills, incluindo estrutura SKILL.md, Skills com múltiplos arquivos e exemplos, consulte:
- [Agent Skills no Claude Code](https://code.claude.com/docs/skills): Guia completo com exemplos
- [Agent Skills Best Practices](/docs/pt-BR/agents-and-tools/agent-skills/best-practices): Diretrizes de autoria e convenções de nomenclatura

## Restrições de Ferramenta

<Note>
O campo frontmatter `allowed-tools` em SKILL.md é suportado apenas ao usar Claude Code CLI diretamente. **Não se aplica ao usar Skills através do SDK**.

Ao usar o SDK, controle o acesso à ferramenta através da opção principal `allowedTools` em sua configuração de consulta.
</Note>

Para restringir ferramentas para Skills em aplicações SDK, use a opção `allowedTools`:

<Note>
As instruções de importação do primeiro exemplo são assumidas nos seguintes trechos de código.
</Note>

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill", "Read", "Grep", "Glob"]  # Restricted toolset
)

async for message in query(
    prompt="Analyze the codebase structure",
    options=options
):
    print(message)
```

```typescript TypeScript
// Skills can only use Read, Grep, and Glob tools
for await (const message of query({
  prompt: "Analyze the codebase structure",
  options: {
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Grep", "Glob"]  // Restricted toolset
  }
})) {
  console.log(message);
}
```

</CodeGroup>

## Descobrindo Skills Disponíveis

Para ver quais Skills estão disponíveis em sua aplicação SDK, simplesmente pergunte a Claude:

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill"]
)

async for message in query(
    prompt="What Skills are available?",
    options=options
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "What Skills are available?",
  options: {
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill"]
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Claude listará as Skills disponíveis com base em seu diretório de trabalho atual e plugins instalados.

## Testando Skills

Teste Skills fazendo perguntas que correspondam às suas descrições:

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    cwd="/path/to/project",
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill", "Read", "Bash"]
)

async for message in query(
    prompt="Extract text from invoice.pdf",
    options=options
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "Extract text from invoice.pdf",
  options: {
    cwd: "/path/to/project",
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Bash"]
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Claude invoca automaticamente a Skill relevante se a descrição corresponder à sua solicitação.

## Solução de Problemas

### Skills Não Encontradas

**Verifique a configuração settingSources**: Skills são carregadas apenas quando você configura explicitamente `settingSources`/`setting_sources`. Este é o problema mais comum:

<CodeGroup>

```python Python
# Wrong - Skills won't be loaded
options = ClaudeAgentOptions(
    allowed_tools=["Skill"]
)

# Correct - Skills will be loaded
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// Wrong - Skills won't be loaded
const options = {
  allowedTools: ["Skill"]
};

// Correct - Skills will be loaded
const options = {
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

Para mais detalhes sobre `settingSources`/`setting_sources`, consulte a [referência SDK TypeScript](/docs/pt-BR/agent-sdk/typescript#settingsource) ou [referência SDK Python](/docs/pt-BR/agent-sdk/python#settingsource).

**Verifique o diretório de trabalho**: O SDK carrega Skills em relação à opção `cwd`. Certifique-se de que aponta para um diretório contendo `.claude/skills/`:

<CodeGroup>

```python Python
# Ensure your cwd points to the directory containing .claude/skills/
options = ClaudeAgentOptions(
    cwd="/path/to/project",  # Must contain .claude/skills/
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// Ensure your cwd points to the directory containing .claude/skills/
const options = {
  cwd: "/path/to/project",  // Must contain .claude/skills/
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

Consulte a seção "Usando Skills com o SDK" acima para o padrão completo.

**Verifique o local do sistema de arquivos**:
```bash
# Check project Skills
ls .claude/skills/*/SKILL.md

# Check personal Skills
ls ~/.claude/skills/*/SKILL.md
```

### Skill Não Está Sendo Usada

**Verifique se a ferramenta Skill está habilitada**: Confirme que `"Skill"` está em seu `allowedTools`.

**Verifique a descrição**: Certifique-se de que é específica e inclui palavras-chave relevantes. Consulte [Agent Skills Best Practices](/docs/pt-BR/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) para orientação sobre como escrever descrições eficazes.

### Solução de Problemas Adicional

Para solução de problemas geral de Skills (sintaxe YAML, depuração, etc.), consulte a [seção de solução de problemas de Skills do Claude Code](https://code.claude.com/docs/skills#troubleshooting).

## Documentação Relacionada

### Guias de Skills
- [Agent Skills no Claude Code](https://code.claude.com/docs/skills): Guia completo de Skills com criação, exemplos e solução de problemas
- [Visão Geral de Agent Skills](/docs/pt-BR/agents-and-tools/agent-skills/overview): Visão geral conceitual, benefícios e arquitetura
- [Agent Skills Best Practices](/docs/pt-BR/agents-and-tools/agent-skills/best-practices): Diretrizes de autoria para Skills eficazes
- [Agent Skills Cookbook](https://github.com/anthropics/claude-cookbooks/tree/main/skills): Skills de exemplo e modelos

### Recursos do SDK
- [Subagentes no SDK](/docs/pt-BR/agent-sdk/subagents): Agentes baseados em sistema de arquivos semelhantes com opções programáticas
- [Slash Commands no SDK](/docs/pt-BR/agent-sdk/slash-commands): Comandos invocados pelo usuário
- [Visão Geral do SDK](/docs/pt-BR/agent-sdk/overview): Conceitos gerais do SDK
- [Referência SDK TypeScript](/docs/pt-BR/agent-sdk/typescript): Documentação completa da API
- [Referência SDK Python](/docs/pt-BR/agent-sdk/python): Documentação completa da API