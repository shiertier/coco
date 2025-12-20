# 托管 Agent SDK

在生产环境中部署和托管 Claude Agent SDK

---

Claude Agent SDK 与传统的无状态 LLM API 不同，它维护对话状态并在持久环境中执行命令。本指南涵盖了 SDK 代理在生产中部署的架构、托管考虑事项和最佳实践。

<Info>
有关超越基本沙箱的安全加固——包括网络控制、凭证管理和隔离选项——请参阅 [安全部署](/docs/zh-CN/agent-sdk/secure-deployment)。
</Info>

## 托管要求

### 基于容器的沙箱

为了安全和隔离，SDK 应在沙箱容器环境中运行。这提供了进程隔离、资源限制、网络控制和临时文件系统。

SDK 还支持 [程序化沙箱配置](/docs/zh-CN/agent-sdk/typescript#sandbox-settings) 用于命令执行。

### 系统要求

每个 SDK 实例需要：

- **运行时依赖**
  - Python 3.10+（用于 Python SDK）或 Node.js 18+（用于 TypeScript SDK）
  - Node.js（Claude Code CLI 必需）
  - Claude Code CLI：`npm install -g @anthropic-ai/claude-code`

- **资源分配**
  - 推荐：1GiB RAM、5GiB 磁盘和 1 个 CPU（根据您的任务需要调整）

- **网络访问**
  - 出站 HTTPS 到 `api.anthropic.com`
  - 可选：访问 MCP 服务器或外部工具

## 理解 SDK 架构

与无状态 API 调用不同，Claude Agent SDK 作为 **长运行进程** 运行，该进程：
- **在持久 shell 环境中执行命令**
- **在工作目录中管理文件操作**
- **处理工具执行**，包含来自先前交互的上下文

## 沙箱提供商选项

多个提供商专门提供用于 AI 代码执行的安全容器环境：

- **[Cloudflare Sandboxes](https://github.com/cloudflare/sandbox-sdk)**
- **[Modal Sandboxes](https://modal.com/docs/guide/sandbox)**
- **[Daytona](https://www.daytona.io/)**
- **[E2B](https://e2b.dev/)**
- **[Fly Machines](https://fly.io/docs/machines/)**
- **[Vercel Sandbox](https://vercel.com/docs/functions/sandbox)**

有关自托管选项（Docker、gVisor、Firecracker）和详细隔离配置，请参阅 [隔离技术](/docs/zh-CN/agent-sdk/secure-deployment#isolation-technologies)。

## 生产部署模式

### 模式 1：临时会话

为每个用户任务创建一个新容器，完成后销毁它。

最适合一次性任务，用户可能在任务完成时仍与 AI 交互，但完成后容器会被销毁。

**示例：**
- 错误调查和修复：使用相关上下文调试和解决特定问题
- 发票处理：从收据/发票中提取和结构化数据用于会计系统
- 翻译任务：在语言之间翻译文档或内容批次
- 图像/视频处理：对媒体文件应用转换、优化或提取元数据

### 模式 2：长运行会话

为长运行任务维护持久容器实例。通常在容器内根据需求运行 **多个** Claude Agent 进程。

最适合主动代理，它们在没有用户输入的情况下采取行动，为内容提供服务的代理或处理大量消息的代理。

**示例：**
- 电子邮件代理：监控传入电子邮件并根据内容自主分类、响应或采取行动
- 网站构建器：为每个用户托管自定义网站，具有通过容器端口提供的实时编辑功能
- 高频聊天机器人：处理来自 Slack 等平台的连续消息流，其中快速响应时间至关重要

### 模式 3：混合会话

临时容器，使用历史和状态进行补充，可能来自数据库或 SDK 的会话恢复功能。

最适合与用户间歇性交互的容器，启动工作并在工作完成时关闭，但可以继续。

**示例：**
- 个人项目经理：帮助管理正在进行的项目，进行间歇性检查，维护任务、决策和进度的上下文
- 深度研究：进行多小时研究任务，保存发现并在用户返回时恢复调查
- 客户支持代理：处理跨越多个交互的支持票证，加载票证历史和客户上下文

### 模式 4：单个容器

在一个全局容器中运行多个 Claude Agent SDK 进程。

最适合必须紧密协作的代理。这可能是最不受欢迎的模式，因为您必须防止代理相互覆盖。

**示例：**
- **模拟**：在模拟中相互交互的代理，例如视频游戏。

# 常见问题

### 我如何与我的沙箱通信？
在容器中托管时，暴露端口以与您的 SDK 实例通信。您的应用程序可以为外部客户端暴露 HTTP/WebSocket 端点，而 SDK 在容器内部运行。

### 托管容器的成本是多少？
我们发现服务代理的主要成本是令牌，容器根据您配置的内容而异，但最低成本大约是每小时 5 美分。

### 我应该何时关闭空闲容器与保持它们温暖？
这可能取决于提供商，不同的沙箱提供商将允许您为空闲超时设置不同的条件，之后沙箱可能会关闭。
您需要根据您认为用户响应可能的频率来调整此超时。

### 我应该多久更新一次 Claude Code CLI？
Claude Code CLI 使用 semver 进行版本控制，因此任何破坏性更改都将被版本化。

### 我如何监控容器健康和代理性能？
由于容器只是服务器，您用于后端的相同日志记录基础设施将适用于容器。

### 代理会话在超时前可以运行多长时间？
代理会话不会超时，但我们建议设置 'maxTurns' 属性以防止 Claude 陷入循环。

## 后续步骤

- [安全部署](/docs/zh-CN/agent-sdk/secure-deployment) - 网络控制、凭证管理和隔离加固
- [TypeScript SDK - 沙箱设置](/docs/zh-CN/agent-sdk/typescript#sandbox-settings) - 以编程方式配置沙箱
- [会话指南](/docs/zh-CN/agent-sdk/sessions) - 了解会话管理
- [权限](/docs/zh-CN/agent-sdk/permissions) - 配置工具权限
- [成本跟踪](/docs/zh-CN/agent-sdk/cost-tracking) - 监控 API 使用情况
- [MCP 集成](/docs/zh-CN/agent-sdk/mcp) - 使用自定义工具扩展