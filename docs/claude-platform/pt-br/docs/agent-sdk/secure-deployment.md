# Implantação segura de agentes de IA

Um guia para proteger implantações do Claude Code e Agent SDK com isolamento, gerenciamento de credenciais e controles de rede

---

Claude Code e o Agent SDK são ferramentas poderosas que podem executar código, acessar arquivos e interagir com serviços externos em seu nome. Como qualquer ferramenta com essas capacidades, implantá-las com cuidado garante que você obtenha os benefícios mantendo controles apropriados.

Diferentemente do software tradicional que segue caminhos de código predeterminados, essas ferramentas geram suas ações dinamicamente com base no contexto e nos objetivos. Essa flexibilidade é o que as torna úteis, mas também significa que seu comportamento pode ser influenciado pelo conteúdo que processam: arquivos, páginas da web ou entrada do usuário. Isso às vezes é chamado de injeção de prompt. Por exemplo, se o README de um repositório contiver instruções incomuns, Claude Code pode incorporá-las em suas ações de maneiras que o operador não antecipou. Este guia aborda maneiras práticas de reduzir esse risco.

A boa notícia é que proteger uma implantação de agente não requer infraestrutura exótica. Os mesmos princípios que se aplicam à execução de qualquer código semi-confiável se aplicam aqui: isolamento, privilégio mínimo e defesa em profundidade. Claude Code inclui vários recursos de segurança que ajudam com preocupações comuns, e este guia percorre esses recursos junto com opções de endurecimento adicionais para quem precisar delas.

Nem toda implantação precisa de segurança máxima. Um desenvolvedor executando Claude Code em seu laptop tem requisitos diferentes de uma empresa processando dados de clientes em um ambiente multi-tenant. Este guia apresenta opções que variam desde os recursos de segurança integrados do Claude Code até arquiteturas de produção endurecidas, para que você possa escolher o que se adequa à sua situação.

## O que estamos protegendo?

Agentes podem executar ações não intencionais devido a injeção de prompt (instruções incorporadas no conteúdo que processam) ou erro do modelo. Os modelos Claude são projetados para resistir a isso, e conforme analisamos em nosso [cartão de modelo](https://assets.anthropic.com/m/64823ba7485345a7/Claude-Opus-4-5-System-Card.pdf), acreditamos que Claude Opus 4.5 é o modelo de fronteira mais robusto disponível.

A defesa em profundidade ainda é uma boa prática. Por exemplo, se um agente processa um arquivo malicioso que o instrui a enviar dados de clientes para um servidor externo, controles de rede podem bloquear essa solicitação completamente.

## Recursos de segurança integrados

Claude Code inclui vários recursos de segurança que abordam preocupações comuns. Consulte a [documentação de segurança](https://code.claude.com/docs/pt-BR/security) para detalhes completos.

- **Sistema de permissões**: Cada ferramenta e comando bash pode ser configurado para permitir, bloquear ou solicitar aprovação do usuário. Use padrões glob para criar regras como "permitir todos os comandos npm" ou "bloquear qualquer comando com sudo". As organizações podem definir políticas que se aplicam a todos os usuários. Consulte [controle de acesso e permissões](https://code.claude.com/docs/pt-BR/iam#access-control-and-permissions).
- **Análise estática**: Antes de executar comandos bash, Claude Code executa análise estática para identificar operações potencialmente arriscadas. Comandos que modificam arquivos do sistema ou acessam diretórios sensíveis são sinalizados e requerem aprovação explícita do usuário.
- **Resumo de pesquisa na web**: Os resultados da pesquisa são resumidos em vez de passar o conteúdo bruto diretamente para o contexto, reduzindo o risco de injeção de prompt de conteúdo web malicioso.
- **Modo sandbox**: Comandos bash podem ser executados em um ambiente sandbox que restringe acesso ao sistema de arquivos e rede. Consulte a [documentação de sandboxing](https://code.claude.com/docs/pt-BR/sandboxing) para detalhes.

## Princípios de segurança

Para implantações que requerem endurecimento adicional além dos padrões do Claude Code, esses princípios guiam as opções disponíveis.

### Limites de segurança

Um limite de segurança separa componentes com diferentes níveis de confiança. Para implantações de alta segurança, você pode colocar recursos sensíveis (como credenciais) fora do limite contendo o agente. Se algo der errado no ambiente do agente, recursos fora desse limite permanecem protegidos.

Por exemplo, em vez de dar a um agente acesso direto a uma chave de API, você poderia executar um proxy fora do ambiente do agente que injeta a chave em solicitações. O agente pode fazer chamadas de API, mas nunca vê a credencial em si. Esse padrão é útil para implantações multi-tenant ou ao processar conteúdo não confiável.

### Privilégio mínimo

Quando necessário, você pode restringir o agente apenas às capacidades necessárias para sua tarefa específica:

| Recurso | Opções de restrição |
|----------|---------------------|
| Sistema de arquivos | Montar apenas diretórios necessários, preferir somente leitura |
| Rede | Restringir a endpoints específicos via proxy |
| Credenciais | Injetar via proxy em vez de expor diretamente |
| Capacidades do sistema | Descartar capacidades Linux em contêineres |

### Defesa em profundidade

Para ambientes de alta segurança, camadas de múltiplos controles fornecem proteção adicional. As opções incluem:

- Isolamento de contêiner
- Restrições de rede
- Controles de sistema de arquivos
- Validação de solicitação em um proxy

A combinação correta depende do seu modelo de ameaça e requisitos operacionais.

## Tecnologias de isolamento

Diferentes tecnologias de isolamento oferecem diferentes compensações entre força de segurança, desempenho e complexidade operacional.

<Info>
Em todas essas configurações, Claude Code (ou sua aplicação Agent SDK) é executado dentro do limite de isolamento—o sandbox, contêiner ou VM. Os controles de segurança descritos abaixo restringem o que o agente pode acessar de dentro desse limite.
</Info>

| Tecnologia | Força de isolamento | Sobrecarga de desempenho | Complexidade |
|------------|-------------------|---------------------|------------|
| Runtime sandbox | Boa (padrões seguros) | Muito baixa | Baixa |
| Contêineres (Docker) | Dependente da configuração | Baixa | Média |
| gVisor | Excelente (com configuração correta) | Média/Alta | Média |
| VMs (Firecracker, QEMU) | Excelente (com configuração correta) | Alta | Média/Alta |

### Runtime sandbox

Para isolamento leve sem contêineres, [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime) impõe restrições de sistema de arquivos e rede no nível do SO.

A principal vantagem é a simplicidade: nenhuma configuração Docker, imagens de contêiner ou configuração de rede necessária. O proxy e as restrições de sistema de arquivos são integrados. Você fornece um arquivo de configurações especificando domínios e caminhos permitidos.

**Como funciona:**
- **Sistema de arquivos**: Usa primitivos do SO (`bubblewrap` no Linux, `sandbox-exec` no macOS) para restringir acesso de leitura/escrita a caminhos configurados
- **Rede**: Remove namespace de rede (Linux) ou usa perfis Seatbelt (macOS) para rotear tráfego de rede através de um proxy integrado
- **Configuração**: Listas de permissão baseadas em JSON para domínios e caminhos do sistema de arquivos

**Configuração:**
```bash
npm install @anthropic-ai/sandbox-runtime
```

Em seguida, crie um arquivo de configuração especificando caminhos e domínios permitidos.

**Considerações de segurança:**

1. **Kernel no mesmo host**: Diferentemente de VMs, processos em sandbox compartilham o kernel do host. Uma vulnerabilidade de kernel poderia teoricamente permitir escape. Para alguns modelos de ameaça isso é aceitável, mas se você precisar de isolamento no nível do kernel, use gVisor ou uma VM separada.

2. **Sem inspeção TLS**: O proxy lista domínios permitidos mas não inspeciona tráfego criptografado. Se o agente tiver credenciais permissivas para um domínio permitido, certifique-se de que não seja possível usar esse domínio para disparar outras solicitações de rede ou exfiltrar dados.

Para muitos casos de uso de desenvolvedor único e CI/CD, sandbox-runtime aumenta significativamente a barra com configuração mínima. As seções abaixo cobrem contêineres e VMs para implantações que requerem isolamento mais forte.

### Contêineres

Contêineres fornecem isolamento através de namespaces Linux. Cada contêiner tem sua própria visualização do sistema de arquivos, árvore de processos e pilha de rede, enquanto compartilha o kernel do host.

Uma configuração de contêiner endurecida por segurança pode parecer assim:

```bash
docker run \
  --cap-drop ALL \
  --security-opt no-new-privileges \
  --security-opt seccomp=/path/to/seccomp-profile.json \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /home/agent:rw,noexec,nosuid,size=500m \
  --network none \
  --memory 2g \
  --cpus 2 \
  --pids-limit 100 \
  --user 1000:1000 \
  -v /path/to/code:/workspace:ro \
  -v /var/run/proxy.sock:/var/run/proxy.sock:ro \
  agent-image
```

Aqui está o que cada opção faz:

| Opção | Propósito |
|--------|---------|
| `--cap-drop ALL` | Remove capacidades Linux como `NET_ADMIN` e `SYS_ADMIN` que poderiam permitir escalação de privilégio |
| `--security-opt no-new-privileges` | Impede que processos ganhem privilégios através de binários setuid |
| `--security-opt seccomp=...` | Restringe chamadas do sistema disponíveis; o padrão do Docker bloqueia ~44, perfis personalizados podem bloquear mais |
| `--read-only` | Torna o sistema de arquivos raiz do contêiner imutável, impedindo que o agente persista mudanças |
| `--tmpfs /tmp:...` | Fornece um diretório temporário gravável que é limpo quando o contêiner para |
| `--network none` | Remove todas as interfaces de rede; o agente se comunica através do socket Unix montado abaixo |
| `--memory 2g` | Limita o uso de memória para evitar esgotamento de recursos |
| `--pids-limit 100` | Limita a contagem de processos para evitar fork bombs |
| `--user 1000:1000` | Executa como um usuário não-root |
| `-v ...:/workspace:ro` | Monta código somente leitura para que o agente possa analisar mas não modificar. **Evite montar diretórios sensíveis do host como `~/.ssh`, `~/.aws` ou `~/.config`** |
| `-v .../proxy.sock:...` | Monta um socket Unix conectado a um proxy executando fora do contêiner (veja abaixo) |

**Arquitetura de socket Unix:**

Com `--network none`, o contêiner não tem interfaces de rede. A única maneira do agente alcançar o mundo externo é através do socket Unix montado, que se conecta a um proxy executando no host. Este proxy pode impor listas de permissão de domínio, injetar credenciais e registrar todo o tráfego.

Esta é a mesma arquitetura usada por [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime). Mesmo se o agente for comprometido via injeção de prompt, ele não pode exfiltrar dados para servidores arbitrários—ele só pode se comunicar através do proxy, que controla quais domínios são alcançáveis. Para mais detalhes, consulte o [post do blog de sandboxing do Claude Code](https://www.anthropic.com/engineering/claude-code-sandboxing).

**Opções de endurecimento adicional:**

| Opção | Propósito |
|--------|---------|
| `--userns-remap` | Mapeia raiz do contêiner para usuário não-privilegiado do host; requer configuração do daemon mas limita danos de escape de contêiner |
| `--ipc private` | Isola comunicação entre processos para evitar ataques entre contêineres |

### gVisor

Contêineres padrão compartilham o kernel do host: quando código dentro de um contêiner faz uma chamada do sistema, ela vai diretamente para o mesmo kernel que executa o host. Isso significa que uma vulnerabilidade de kernel poderia permitir escape de contêiner. gVisor aborda isso interceptando chamadas do sistema em espaço de usuário antes de chegarem ao kernel do host, implementando sua própria camada de compatibilidade que manipula a maioria das chamadas do sistema sem envolver o kernel real.

Se um agente executa código malicioso (talvez devido a injeção de prompt), esse código é executado no contêiner e poderia tentar exploits de kernel. Com gVisor, a superfície de ataque é muito menor: o código malicioso precisaria explorar primeiro a implementação de espaço de usuário do gVisor e teria acesso limitado ao kernel real.

Para usar gVisor com Docker, instale o runtime `runsc` e configure o daemon:

```json
// /etc/docker/daemon.json
{
  "runtimes": {
    "runsc": {
      "path": "/usr/local/bin/runsc"
    }
  }
}
```

Em seguida, execute contêineres com:

```bash
docker run --runtime=runsc agent-image
```

**Considerações de desempenho:**

| Carga de trabalho | Sobrecarga |
|----------|----------|
| Computação vinculada a CPU | ~0% (sem interceptação de chamada do sistema) |
| Chamadas do sistema simples | ~2× mais lento |
| Intensivo em I/O de arquivo | Até 10-200× mais lento para padrões pesados de abertura/fechamento |

Para ambientes multi-tenant ou ao processar conteúdo não confiável, o isolamento adicional geralmente vale a sobrecarga.

### Máquinas virtuais

VMs fornecem isolamento no nível de hardware através de extensões de virtualização de CPU. Cada VM executa seu próprio kernel, criando um limite forte—uma vulnerabilidade no kernel convidado não compromete diretamente o host. No entanto, VMs não são automaticamente "mais seguras" que alternativas como gVisor. A segurança de VM depende muito do hipervisor e do código de emulação de dispositivo.

Firecracker é projetado para isolamento leve de microVM—pode inicializar VMs em menos de 125ms com menos de 5 MiB de sobrecarga de memória, removendo emulação de dispositivo desnecessária para reduzir a superfície de ataque.

Com essa abordagem, a VM do agente não tem interface de rede externa. Em vez disso, ela se comunica através de `vsock` (sockets virtuais). Todo o tráfego é roteado através de vsock para um proxy no host, que impõe listas de permissão e injeta credenciais antes de encaminhar solicitações.

### Implantações em nuvem

Para implantações em nuvem, você pode combinar qualquer uma das tecnologias de isolamento acima com controles de rede nativos da nuvem:

1. Execute contêineres de agente em uma sub-rede privada sem gateway de internet
2. Configure regras de firewall em nuvem (AWS Security Groups, firewall VPC do GCP) para bloquear toda saída exceto para seu proxy
3. Execute um proxy (como [Envoy](https://www.envoyproxy.io/) com seu filtro `credential_injector`) que valida solicitações, impõe listas de permissão de domínio, injeta credenciais e encaminha para APIs externas
4. Atribua permissões IAM mínimas à conta de serviço do agente, roteando acesso sensível através do proxy quando possível
5. Registre todo o tráfego no proxy para fins de auditoria

## Gerenciamento de credenciais

Agentes frequentemente precisam de credenciais para chamar APIs, acessar repositórios ou interagir com serviços em nuvem. O desafio é fornecer esse acesso sem expor as credenciais em si.

### O padrão de proxy

A abordagem recomendada é executar um proxy fora do limite de segurança do agente que injeta credenciais em solicitações de saída. O agente envia solicitações sem credenciais, o proxy as adiciona e encaminha a solicitação para seu destino.

Este padrão tem vários benefícios:

1. O agente nunca vê as credenciais reais
2. O proxy pode impor uma lista de permissão de endpoints permitidos
3. O proxy pode registrar todas as solicitações para auditoria
4. Credenciais são armazenadas em um local seguro em vez de distribuídas para cada agente

### Configurando Claude Code para usar um proxy

Claude Code suporta dois métodos para rotear solicitações de amostragem através de um proxy:

**Opção 1: ANTHROPIC_BASE_URL (simples mas apenas para solicitações de API de amostragem)**

```bash
export ANTHROPIC_BASE_URL="http://localhost:8080"
```

Isso diz ao Claude Code e ao Agent SDK para enviar solicitações de amostragem para seu proxy em vez da API Anthropic diretamente. Seu proxy recebe solicitações HTTP em texto simples, pode inspecionar e modificá-las (incluindo injetar credenciais), então encaminha para a API real.

**Opção 2: HTTP_PROXY / HTTPS_PROXY (em todo o sistema)**

```bash
export HTTP_PROXY="http://localhost:8080"
export HTTPS_PROXY="http://localhost:8080"
```

Claude Code e o Agent SDK respeitam essas variáveis de ambiente padrão, roteando todo o tráfego HTTP através do proxy. Para HTTPS, o proxy cria um túnel CONNECT criptografado: ele não pode ver ou modificar conteúdo de solicitação sem interceptação TLS.

### Implementando um proxy

Você pode construir seu próprio proxy ou usar um existente:

- [Envoy Proxy](https://www.envoyproxy.io/) — proxy de nível de produção com filtro `credential_injector` para adicionar cabeçalhos de autenticação
- [mitmproxy](https://mitmproxy.org/) — proxy que termina TLS para inspecionar e modificar tráfego HTTPS
- [Squid](http://www.squid-cache.org/) — proxy de cache com listas de controle de acesso
- [LiteLLM](https://github.com/BerriAI/litellm) — gateway LLM com injeção de credenciais e limitação de taxa

### Credenciais para outros serviços

Além de amostragem da API Anthropic, agentes frequentemente precisam de acesso autenticado a outros serviços—repositórios git, bancos de dados, APIs internas. Existem duas abordagens principais:

#### Ferramentas personalizadas

Forneça acesso através de um servidor MCP ou ferramenta personalizada que roteia solicitações para um serviço executando fora do limite de segurança do agente. O agente chama a ferramenta, mas a solicitação autenticada real acontece fora—a ferramenta chama um proxy que injeta as credenciais.

Por exemplo, um servidor MCP git poderia aceitar comandos do agente mas encaminhá-los para um proxy git executando no host, que adiciona autenticação antes de contatar o repositório remoto. O agente nunca vê as credenciais.

Vantagens:
- **Sem interceptação TLS**: O serviço externo faz solicitações autenticadas diretamente
- **Credenciais ficam fora**: O agente apenas vê a interface da ferramenta, não as credenciais subjacentes

#### Encaminhamento de tráfego

Para chamadas da API Anthropic, `ANTHROPIC_BASE_URL` permite rotear solicitações para um proxy que pode inspecionar e modificá-las em texto simples. Mas para outros serviços HTTPS (GitHub, registros npm, APIs internas), o tráfego geralmente é criptografado de ponta a ponta—mesmo se você roteá-lo através de um proxy via `HTTP_PROXY`, o proxy apenas vê um túnel TLS opaco e não pode injetar credenciais.

Para modificar tráfego HTTPS para serviços arbitrários, sem usar uma ferramenta personalizada, você precisa de um proxy que termina TLS que descriptografa tráfego, inspeciona ou modifica, então re-criptografa antes de encaminhar. Isso requer:

1. Executar o proxy fora do contêiner do agente
2. Instalar o certificado CA do proxy no armazenamento de confiança do agente (para que o agente confie nos certificados do proxy)
3. Configurar `HTTP_PROXY`/`HTTPS_PROXY` para rotear tráfego através do proxy

Esta abordagem manipula qualquer serviço baseado em HTTP sem escrever ferramentas personalizadas, mas adiciona complexidade em torno do gerenciamento de certificado.

Note que nem todos os programas respeitam `HTTP_PROXY`/`HTTPS_PROXY`. A maioria das ferramentas (curl, pip, npm, git) respeita, mas algumas podem contornar essas variáveis e conectar diretamente. Por exemplo, `fetch()` do Node.js ignora essas variáveis por padrão; no Node 24+ você pode definir `NODE_USE_ENV_PROXY=1` para habilitar suporte. Para cobertura abrangente, você pode usar [proxychains](https://github.com/haad/proxychains) para interceptar chamadas de rede, ou configurar iptables para redirecionar tráfego de saída para um proxy transparente.

<Info>
Um **proxy transparente** intercepta tráfego no nível de rede, então o cliente não precisa ser configurado para usá-lo. Proxies regulares requerem que clientes se conectem explicitamente e falem HTTP CONNECT ou SOCKS. Proxies transparentes (como Squid ou mitmproxy em modo transparente) podem manipular conexões TCP redirecionadas brutas.
</Info>

Ambas as abordagens ainda requerem o proxy que termina TLS e certificado CA confiável—elas apenas garantem que o tráfego realmente alcance o proxy.

## Configuração do sistema de arquivos

Controles de sistema de arquivos determinam quais arquivos o agente pode ler e escrever.

### Montagem de código somente leitura

Quando o agente precisa analisar código mas não modificá-lo, monte o diretório somente leitura:

```bash
docker run -v /path/to/code:/workspace:ro agent-image
```

<Warning>
Mesmo acesso somente leitura a um diretório de código pode expor credenciais. Arquivos comuns a excluir ou sanitizar antes de montar:

| Arquivo | Risco |
|------|------|
| `.env`, `.env.local` | Chaves de API, senhas de banco de dados, segredos |
| `~/.git-credentials` | Senhas/tokens git em texto simples |
| `~/.aws/credentials` | Chaves de acesso AWS |
| `~/.config/gcloud/application_default_credentials.json` | Tokens ADC do Google Cloud |
| `~/.azure/` | Credenciais da CLI Azure |
| `~/.docker/config.json` | Tokens de autenticação de registro Docker |
| `~/.kube/config` | Credenciais de cluster Kubernetes |
| `.npmrc`, `.pypirc` | Tokens de registro de pacote |
| `*-service-account.json` | Chaves de conta de serviço do GCP |
| `*.pem`, `*.key` | Chaves privadas |

Considere copiar apenas os arquivos de origem necessários, ou usar filtragem estilo `.dockerignore`.
</Warning>

### Locais graváveis

Se o agente precisa escrever arquivos, você tem algumas opções dependendo se quer que as mudanças persistam:

Para espaços de trabalho efêmeros em contêineres, use montagens `tmpfs` que existem apenas em memória e são limpas quando o contêiner para:

```bash
docker run \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /workspace:rw,noexec,size=500m \
  agent-image
```

Se você quer revisar mudanças antes de persistir, um sistema de arquivos overlay permite que o agente escreva sem modificar arquivos subjacentes—mudanças são armazenadas em uma camada separada que você pode inspecionar, aplicar ou descartar. Para saída totalmente persistente, monte um volume dedicado mas mantenha-o separado de diretórios sensíveis.

## Leitura adicional

- [Documentação de segurança do Claude Code](https://code.claude.com/docs/pt-BR/security)
- [Hospedando o Agent SDK](/docs/pt-BR/agent-sdk/hosting)
- [Manipulando permissões](/docs/pt-BR/agent-sdk/permissions)
- [Sandbox runtime](https://github.com/anthropic-experimental/sandbox-runtime)
- [The Lethal Trifecta for AI Agents](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/)
- [OWASP Top 10 for LLM Applications](https://owasp.org/www-project-top-10-for-large-language-model-applications/)
- [Docker Security Best Practices](https://docs.docker.com/engine/security/)
- [gVisor Documentation](https://gvisor.dev/docs/)
- [Firecracker Documentation](https://firecracker-microvm.github.io/)