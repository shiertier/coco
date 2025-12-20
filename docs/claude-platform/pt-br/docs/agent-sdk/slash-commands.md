# Comandos Slash no SDK

Aprenda como usar comandos slash para controlar sessões do Claude Code através do SDK

---

Os comandos slash fornecem uma maneira de controlar sessões do Claude Code com comandos especiais que começam com `/`. Esses comandos podem ser enviados através do SDK para realizar ações como limpar o histórico de conversas, compactar mensagens ou obter ajuda.

## Descobrindo Comandos Slash Disponíveis

O Claude Agent SDK fornece informações sobre comandos slash disponíveis na mensagem de inicialização do sistema. Acesse essas informações quando sua sessão iniciar:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Olá Claude",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Comandos slash disponíveis:", message.slash_commands);
    // Exemplo de saída: ["/compact", "/clear", "/help"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="Olá Claude",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            print("Comandos slash disponíveis:", message.slash_commands)
            # Exemplo de saída: ["/compact", "/clear", "/help"]

asyncio.run(main())
```

</CodeGroup>

## Enviando Comandos Slash

Envie comandos slash incluindo-os em sua string de prompt, assim como texto regular:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Enviar um comando slash
for await (const message of query({
  prompt: "/compact",
  options: { maxTurns: 1 }
})) {
  if (message.type === "result") {
    console.log("Comando executado:", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Enviar um comando slash
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if message.type == "result":
            print("Comando executado:", message.result)

asyncio.run(main())
```

</CodeGroup>

## Comandos Slash Comuns

### `/compact` - Compactar Histórico de Conversa

O comando `/compact` reduz o tamanho do seu histórico de conversa resumindo mensagens mais antigas enquanto preserva contexto importante:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "/compact",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "compact_boundary") {
    console.log("Compactação concluída");
    console.log("Tokens pré-compactação:", message.compact_metadata.pre_tokens);
    console.log("Gatilho:", message.compact_metadata.trigger);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if (message.type == "system" and 
            message.subtype == "compact_boundary"):
            print("Compactação concluída")
            print("Tokens pré-compactação:", 
                  message.compact_metadata.pre_tokens)
            print("Gatilho:", message.compact_metadata.trigger)

asyncio.run(main())
```

</CodeGroup>

### `/clear` - Limpar Conversa

O comando `/clear` inicia uma conversa nova limpando todo o histórico anterior:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Limpar conversa e começar do zero
for await (const message of query({
  prompt: "/clear",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Conversa limpa, nova sessão iniciada");
    console.log("ID da sessão:", message.session_id);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Limpar conversa e começar do zero
    async for message in query(
        prompt="/clear",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            print("Conversa limpa, nova sessão iniciada")
            print("ID da sessão:", message.session_id)

asyncio.run(main())
```

</CodeGroup>

## Criando Comandos Slash Personalizados

Além de usar comandos slash integrados, você pode criar seus próprios comandos personalizados que estão disponíveis através do SDK. Comandos personalizados são definidos como arquivos markdown em diretórios específicos, similar a como subagentes são configurados.

### Localizações de Arquivos

Comandos slash personalizados são armazenados em diretórios designados baseados em seu escopo:

- **Comandos de projeto**: `.claude/commands/` - Disponíveis apenas no projeto atual
- **Comandos pessoais**: `~/.claude/commands/` - Disponíveis em todos os seus projetos

### Formato de Arquivo

Cada comando personalizado é um arquivo markdown onde:
- O nome do arquivo (sem extensão `.md`) torna-se o nome do comando
- O conteúdo do arquivo define o que o comando faz
- Frontmatter YAML opcional fornece configuração

#### Exemplo Básico

Crie `.claude/commands/refactor.md`:

```markdown
Refatore o código selecionado para melhorar legibilidade e manutenibilidade.
Foque em princípios de código limpo e melhores práticas.
```

Isso cria o comando `/refactor` que você pode usar através do SDK.

#### Com Frontmatter

Crie `.claude/commands/security-check.md`:

```markdown
---
allowed-tools: Read, Grep, Glob
description: Executar verificação de vulnerabilidades de segurança
model: claude-3-5-sonnet-20241022
---

Analise a base de código para vulnerabilidades de segurança incluindo:
- Riscos de injeção SQL
- Vulnerabilidades XSS
- Credenciais expostas
- Configurações inseguras
```

### Usando Comandos Personalizados no SDK

Uma vez definidos no sistema de arquivos, comandos personalizados ficam automaticamente disponíveis através do SDK:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Usar um comando personalizado
for await (const message of query({
  prompt: "/refactor src/auth/login.ts",
  options: { maxTurns: 3 }
})) {
  if (message.type === "assistant") {
    console.log("Sugestões de refatoração:", message.message);
  }
}

// Comandos personalizados aparecem na lista slash_commands
for await (const message of query({
  prompt: "Olá",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // Incluirá tanto comandos integrados quanto personalizados
    console.log("Comandos disponíveis:", message.slash_commands);
    // Exemplo: ["/compact", "/clear", "/help", "/refactor", "/security-check"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Usar um comando personalizado
    async for message in query(
        prompt="/refactor src/auth/login.py",
        options={"max_turns": 3}
    ):
        if message.type == "assistant":
            print("Sugestões de refatoração:", message.message)
    
    # Comandos personalizados aparecem na lista slash_commands
    async for message in query(
        prompt="Olá",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            # Incluirá tanto comandos integrados quanto personalizados
            print("Comandos disponíveis:", message.slash_commands)
            # Exemplo: ["/compact", "/clear", "/help", "/refactor", "/security-check"]

asyncio.run(main())
```

</CodeGroup>

### Recursos Avançados

#### Argumentos e Placeholders

Comandos personalizados suportam argumentos dinâmicos usando placeholders:

Crie `.claude/commands/fix-issue.md`:

```markdown
---
argument-hint: [issue-number] [priority]
description: Corrigir uma issue do GitHub
---

Corrija a issue #$1 com prioridade $2.
Verifique a descrição da issue e implemente as mudanças necessárias.
```

Use no SDK:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Passar argumentos para comando personalizado
for await (const message of query({
  prompt: "/fix-issue 123 high",
  options: { maxTurns: 5 }
})) {
  // Comando processará com $1="123" e $2="high"
  if (message.type === "result") {
    console.log("Issue corrigida:", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Passar argumentos para comando personalizado
    async for message in query(
        prompt="/fix-issue 123 high",
        options={"max_turns": 5}
    ):
        # Comando processará com $1="123" e $2="high"
        if message.type == "result":
            print("Issue corrigida:", message.result)

asyncio.run(main())
```

</CodeGroup>

#### Execução de Comandos Bash

Comandos personalizados podem executar comandos bash e incluir sua saída:

Crie `.claude/commands/git-commit.md`:

```markdown
---
allowed-tools: Bash(git add:*), Bash(git status:*), Bash(git commit:*)
description: Criar um commit git
---

## Contexto

- Status atual: !`git status`
- Diff atual: !`git diff HEAD`

## Tarefa

Crie um commit git com mensagem apropriada baseada nas mudanças.
```

#### Referências de Arquivos

Inclua conteúdos de arquivos usando o prefixo `@`:

Crie `.claude/commands/review-config.md`:

```markdown
---
description: Revisar arquivos de configuração
---

Revise os seguintes arquivos de configuração para problemas:
- Configuração do pacote: @package.json
- Configuração TypeScript: @tsconfig.json
- Configuração de ambiente: @.env

Verifique problemas de segurança, dependências desatualizadas e configurações incorretas.
```

### Organização com Namespacing

Organize comandos em subdiretórios para melhor estrutura:

```bash
.claude/commands/
├── frontend/
│   ├── component.md      # Cria /component (project:frontend)
│   └── style-check.md     # Cria /style-check (project:frontend)
├── backend/
│   ├── api-test.md        # Cria /api-test (project:backend)
│   └── db-migrate.md      # Cria /db-migrate (project:backend)
└── review.md              # Cria /review (project)
```

O subdiretório aparece na descrição do comando mas não afeta o nome do comando em si.

### Exemplos Práticos

#### Comando de Revisão de Código

Crie `.claude/commands/code-review.md`:

```markdown
---
allowed-tools: Read, Grep, Glob, Bash(git diff:*)
description: Revisão abrangente de código
---

## Arquivos Alterados
!`git diff --name-only HEAD~1`

## Mudanças Detalhadas
!`git diff HEAD~1`

## Lista de Verificação da Revisão

Revise as mudanças acima para:
1. Qualidade e legibilidade do código
2. Vulnerabilidades de segurança
3. Implicações de performance
4. Cobertura de testes
5. Completude da documentação

Forneça feedback específico e acionável organizado por prioridade.
```

#### Comando Executor de Testes

Crie `.claude/commands/test.md`:

```markdown
---
allowed-tools: Bash, Read, Edit
argument-hint: [test-pattern]
description: Executar testes com padrão opcional
---

Execute testes correspondendo ao padrão: $ARGUMENTS

1. Detecte o framework de teste (Jest, pytest, etc.)
2. Execute testes com o padrão fornecido
3. Se os testes falharem, analise e corrija-os
4. Execute novamente para verificar as correções
```

Use esses comandos através do SDK:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Executar revisão de código
for await (const message of query({
  prompt: "/code-review",
  options: { maxTurns: 3 }
})) {
  // Processar feedback da revisão
}

// Executar testes específicos
for await (const message of query({
  prompt: "/test auth",
  options: { maxTurns: 5 }
})) {
  // Lidar com resultados dos testes
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Executar revisão de código
    async for message in query(
        prompt="/code-review",
        options={"max_turns": 3}
    ):
        # Processar feedback da revisão
        pass
    
    # Executar testes específicos
    async for message in query(
        prompt="/test auth",
        options={"max_turns": 5}
    ):
        # Lidar com resultados dos testes
        pass

asyncio.run(main())
```

</CodeGroup>

## Veja Também

- [Comandos Slash](https://code.claude.com/docs/slash-commands) - Documentação completa de comandos slash
- [Subagentes no SDK](/docs/pt-BR/agent-sdk/subagents) - Configuração baseada em sistema de arquivos similar para subagentes
- [Referência do SDK TypeScript](https://code.claude.com/docs/typescript-sdk-reference) - Documentação completa da API
- [Visão geral do SDK](/docs/pt-BR/agent-sdk/overview) - Conceitos gerais do SDK
- [Referência da CLI](https://code.claude.com/docs/cli-reference) - Interface de linha de comando