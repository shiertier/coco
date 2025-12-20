# Hospedando o Agent SDK

Implante e hospede o Claude Agent SDK em ambientes de produção

---

O Claude Agent SDK difere das APIs LLM tradicionais sem estado, pois mantém o estado conversacional e executa comandos em um ambiente persistente. Este guia cobre a arquitetura, considerações de hospedagem e melhores práticas para implantar agentes baseados em SDK em produção.

<Info>
Para endurecimento de segurança além da sandboxing básica—incluindo controles de rede, gerenciamento de credenciais e opções de isolamento—veja [Implantação Segura](/docs/pt-BR/agent-sdk/secure-deployment).
</Info>

## Requisitos de Hospedagem

### Sandboxing Baseado em Contêiner

Para segurança e isolamento, o SDK deve ser executado dentro de um ambiente de contêiner em sandbox. Isso fornece isolamento de processo, limites de recursos, controle de rede e sistemas de arquivos efêmeros.

O SDK também suporta [configuração de sandbox programática](/docs/pt-BR/agent-sdk/typescript#sandbox-settings) para execução de comandos.

### Requisitos do Sistema

Cada instância do SDK requer:

- **Dependências de tempo de execução**
  - Python 3.10+ (para SDK Python) ou Node.js 18+ (para SDK TypeScript)
  - Node.js (necessário pelo Claude Code CLI)
  - Claude Code CLI: `npm install -g @anthropic-ai/claude-code`

- **Alocação de recursos**
  - Recomendado: 1GiB de RAM, 5GiB de disco e 1 CPU (varie isso com base em sua tarefa conforme necessário)

- **Acesso à rede**
  - HTTPS de saída para `api.anthropic.com`
  - Opcional: Acesso a servidores MCP ou ferramentas externas

## Entendendo a Arquitetura do SDK

Diferentemente das chamadas de API sem estado, o Claude Agent SDK opera como um **processo de longa duração** que:
- **Executa comandos** em um ambiente de shell persistente
- **Gerencia operações de arquivo** dentro de um diretório de trabalho
- **Manipula execução de ferramentas** com contexto de interações anteriores

## Opções de Provedor de Sandbox

Vários provedores se especializam em ambientes de contêiner seguro para execução de código de IA:

- **[Cloudflare Sandboxes](https://github.com/cloudflare/sandbox-sdk)**
- **[Modal Sandboxes](https://modal.com/docs/guide/sandbox)**
- **[Daytona](https://www.daytona.io/)**
- **[E2B](https://e2b.dev/)**
- **[Fly Machines](https://fly.io/docs/machines/)**
- **[Vercel Sandbox](https://vercel.com/docs/functions/sandbox)**

Para opções auto-hospedadas (Docker, gVisor, Firecracker) e configuração de isolamento detalhada, veja [Tecnologias de Isolamento](/docs/pt-BR/agent-sdk/secure-deployment#isolation-technologies).

## Padrões de Implantação em Produção

### Padrão 1: Sessões Efêmeras

Crie um novo contêiner para cada tarefa do usuário e destrua-o quando concluído.

Melhor para tarefas únicas, o usuário ainda pode interagir com a IA enquanto a tarefa está sendo concluída, mas uma vez concluída, o contêiner é destruído.

**Exemplos:**
- Investigação e Correção de Bugs: Depure e resolva um problema específico com contexto relevante
- Processamento de Faturas: Extraia e estruture dados de recibos/faturas para sistemas contábeis
- Tarefas de Tradução: Traduza documentos ou lotes de conteúdo entre idiomas
- Processamento de Imagem/Vídeo: Aplique transformações, otimizações ou extraia metadados de arquivos de mídia

### Padrão 2: Sessões de Longa Duração

Mantenha instâncias de contêiner persistentes para tarefas de longa duração. Frequentemente, executando _múltiplos_ processos Claude Agent dentro do contêiner com base na demanda.

Melhor para agentes proativos que tomam ação sem a entrada do usuário, agentes que servem conteúdo ou agentes que processam grandes quantidades de mensagens.

**Exemplos:**
- Agente de Email: Monitora emails recebidos e triagem autônoma, responde ou toma ações com base no conteúdo
- Construtor de Site: Hospeda sites personalizados por usuário com capacidades de edição ao vivo servidas através de portas de contêiner
- Chatbots de Alta Frequência: Manipula fluxos contínuos de mensagens de plataformas como Slack onde tempos de resposta rápidos são críticos

### Padrão 3: Sessões Híbridas

Contêineres efêmeros que são hidratados com histórico e estado, possivelmente de um banco de dados ou dos recursos de retomada de sessão do SDK.

Melhor para contêineres com interação intermitente do usuário que inicia o trabalho e desliga quando o trabalho é concluído, mas pode ser continuado.

**Exemplos:**
- Gerenciador de Projeto Pessoal: Ajuda a gerenciar projetos contínuos com check-ins intermitentes, mantém contexto de tarefas, decisões e progresso
- Pesquisa Profunda: Conduz tarefas de pesquisa de várias horas, salva descobertas e retoma a investigação quando o usuário retorna
- Agente de Suporte ao Cliente: Manipula tickets de suporte que abrangem múltiplas interações, carrega histórico de tickets e contexto do cliente

### Padrão 4: Contêineres Únicos

Execute múltiplos processos Claude Agent SDK em um contêiner global.

Melhor para agentes que devem colaborar estreitamente. Este é provavelmente o padrão menos popular porque você terá que evitar que os agentes sobrescrevam um ao outro.

**Exemplos:**
- **Simulações**: Agentes que interagem uns com os outros em simulações, como videogames.

# Perguntas Frequentes

### Como me comunico com meus sandboxes?
Ao hospedar em contêineres, exponha portas para se comunicar com suas instâncias do SDK. Sua aplicação pode expor endpoints HTTP/WebSocket para clientes externos enquanto o SDK é executado internamente dentro do contêiner.

### Qual é o custo de hospedar um contêiner?
Descobrimos que o custo dominante de servir agentes são os tokens, contêineres variam com base no que você provisiona, mas um custo mínimo é aproximadamente 5 centavos por hora em execução.

### Quando devo desligar contêineres ociosos versus mantê-los aquecidos?
Isso provavelmente depende do provedor, diferentes provedores de sandbox permitirão que você defina critérios diferentes para tempos limite de ociosidade após os quais um sandbox pode desligar.
Você vai querer ajustar esse tempo limite com base na frequência com que você acha que a resposta do usuário pode ser.

### Com que frequência devo atualizar o Claude Code CLI?
O Claude Code CLI é versionado com semver, portanto quaisquer mudanças significativas serão versionadas.

### Como monitoro a saúde do contêiner e o desempenho do agente?
Como contêineres são apenas servidores, a mesma infraestrutura de logging que você usa para o backend funcionará para contêineres.

### Quanto tempo uma sessão de agente pode ser executada antes de atingir o tempo limite?
Uma sessão de agente não atingirá o tempo limite, mas recomendamos definir uma propriedade 'maxTurns' para evitar que Claude fique preso em um loop.

## Próximos Passos

- [Implantação Segura](/docs/pt-BR/agent-sdk/secure-deployment) - Controles de rede, gerenciamento de credenciais e endurecimento de isolamento
- [SDK TypeScript - Configurações de Sandbox](/docs/pt-BR/agent-sdk/typescript#sandbox-settings) - Configure sandbox programaticamente
- [Guia de Sessões](/docs/pt-BR/agent-sdk/sessions) - Saiba mais sobre gerenciamento de sessões
- [Permissões](/docs/pt-BR/agent-sdk/permissions) - Configure permissões de ferramentas
- [Rastreamento de Custos](/docs/pt-BR/agent-sdk/cost-tracking) - Monitore o uso da API
- [Integração MCP](/docs/pt-BR/agent-sdk/mcp) - Estenda com ferramentas personalizadas