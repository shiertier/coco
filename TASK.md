# Context Core (CoCo) - 技术规范 v0.0.1 (Physical Separation)

**版本:** v0.0.1
**状态:** APPROVED (Physical Separation)
**架构范式:** Workspace Split + Static Dispatch (No-DI)
**定位:** Edge-Native Dev Tool (Local) vs Cloud-Native Indexing Service (Server)

---

## 1. 核心概述 (Executive Summary)

**Context Core (CoCo)** 是一个专门为代码与知识库设计的**语义检索引擎**。

### 1.1 定位与哲学：两种形态 (Two Forms)

v2.7 引入核心变更：**Server Mode 与 Local Mode 是两种完全不同的产品形态**，不是 Dev/Prod 的关系。
两者场景不同、数据不互通，也不需要以 Local 作为 Remote 的测试环境，仅共享核心逻辑层。

- **Server Mode (SaaS Service):**
  - **定位:** 企业级/团队协作服务。
  - **特性:** 多租户隔离、来源标识存储（`source_ref`/`doc_id`）、增量同步。
  - **用途:** 作为**远程知识库检索服务**，只接收 Pipeline 产物并返回内容片段，不承担文件导航。
  - **关键差异:** Server 侧**不接触用户文件系统**，设计上与本地 FS 无关。
    - **数据仅来自 Pipeline（Crawler/Processor/QA）**，Hydration 由上游完成，**Server 只接收 Artifacts**。
- **Local Mode (Dev Tool):**
  - **定位:** 开源极客工具 (Developer Companion)。
  - **特性:** 下放全部权限、绝对路径、TUI 仪表盘。
  - **用途:** 作为**本地开发工具**，直接索引/监听本地文件，提供实时检索与可视化操作。
  - **关键差异:** **所见即所得** —— Rust Service 直接操作 FS，体验最优化。
- **API 分叉允许:** Server/Local 的对外 API **不要求完全一致**；Local 可暴露路径相关字段与接口，Server 禁止。
- **Shared Core:** 两者仅复用纯逻辑层 (Parsing, Chunking, Protocol)。
- **物理隔离 (Physical Separation):** 拒绝使用 Feature Flags 混合代码。采用 Workspace 策略，Server 代码闭源，Local 代码开源。
- **拒绝运行时 DI (No-DI):** 不使用 `Box<dyn ...>` 与运行时注入；核心仅定义**业务语义端口**，具体实现由 Local/Server 各自编译期绑定为 **Concrete Types**。

### 1.2 核心价值

- **零构建依赖:** 计算层预编译为 Rust 二进制，避免本地 C/C++/Python 构建依赖地狱。
- **资源恒定:** Embedding 模型全局单例，内存不随项目数线性增长。
- **安全沙箱:** 仅基于 `project_id` 访问，**Server Mode Public API** 不接受任何路径参数。

### 1.3 数据流与真相定义 (Data Flows & Truth)

**🟢 Local Mode (The "Live" Flow)**

- **对外输入/输出:** 允许包含本地路径/行号等导航字段（可选）。
- **内部输入:** 本地文件系统 (FS)。触发器: `fs.watch` / Save（**不可靠，需兜底**）。
- **处理:** Rust Service 直接解析；**强制 Debounce + 批处理**，避免事件风暴。
- **兜底:** 定时 Re-scan / Re-sync（如每 5 分钟或 IDE 焦点触发），防止漏事件与分支切换导致的遗漏。
- **真相:** **磁盘文件**是唯一真相。DB 仅仅是加速缓存。
- **能力:** 具备 `grep`/LSP 级别的实时性，所见即所得。

**🔵 Server Mode (The "Pipeline" Flow)**

- **对外输入/输出:** 仅返回内容与逻辑标识（如 `doc_id`/`source_ref`），不包含路径。
- **内部输入:** 上游数据处理流水线 (Crawler -> Processor -> QA).
- **触发器:** CI/CD, Cron, Webhook.
- **处理:** 接收经过清洗、分块、测试验证的高质量 Artifacts。
- **真相:** **数据库 (DB)** 是唯一真相，结果为知识库内容片段，不涉及本地路径。
- **能力:** 提供经过验证的高质量上下文，无噪声。**不**直接操作用户本地 FS。

---

## 2. 系统架构设计 (Architecture Design)

### 2.1 代码组织 (Workspace Strategy)

采用 **Rust Workspace** + **Private 目录** 的结构（如需闭源可后续拆分独立仓库）：

```text
/coco-monorepo
├── /crates                 (Open Source)
│   ├── /coco-core          # 纯逻辑: 分词, AST, Chunking (No I/O)
│   ├── /coco-protocol      # API Defines, DTO, Error Types
│   └── /coco-local         # Local Service (Rust, SQLite + LanceDB + ort)
├── /private                (Closed Source)
│   ├── /coco-server        # SaaS API (PostgreSQL + pgvector)
│   └── /coco-worker        # 异步 Worker (解析/写入)
└── Cargo.toml
```

**拒绝**在同一个 Crate 中通过 `#[cfg(feature = "server")]` 区分，强制物理隔离。
`private/` 可在后续拆分为独立仓库或子模块；当前实现为同一 Workspace。

### 2.2 部署形态 (Deployment Profiles)

| 模式            | 目标场景    | 启动方式        | Service 拓扑        | 存储（Meta+Vector）   | 默认绑定  | 分发优先级     |
| :-------------- | :---------- | :-------------- | :------------------ | :-------------------- | :-------- | :------------- |
| **Server Mode** | 团队/服务器 | Docker/系统服务 | Rust API + Worker   | PostgreSQL + pgvector | 127.0.0.1 | Docker Image   |
| **Local Mode**  | 个人/本机   | `coco start`    | 单进程 Rust Service | SQLite + LanceDB      | 127.0.0.1 | Standalone Bin |

### 2.3 🟢 Local Mode (Hub & Spoke)

```mermaid
graph TD
    Consumer[MCP Client / Web Agent / CLI] --"HTTP/MCP"--> Service

    subgraph "Host Machine"
        subgraph "CoCo Service (Rust)"
            Service[API Gateway]
            Service --> ProjectMgr[Project Manager]
            ProjectMgr --> Registry[Project Registry (ID <-> Source)]
            Registry --> MetaDB[(SQLite: Metadata)]
            Service --> Parser[Tree-sitter]
            Service --> Embedder[ONNX Runtime (Singleton)]
            Service --> VectorDB[(LanceDB: Vectors)]
        end
    end
```

### 2.4 🔵 Server Mode (First-Class Docker)

- **Rust API Service 与 Rust Worker 独立部署**，通过队列/HTTP/gRPC 进行任务分发。
- **服务生命周期** 由容器编排系统管理。

```mermaid
graph TD
    Client[Apps / IDEs] --> API

    subgraph "Upstream Pipeline (Data Factory)"
        Crawler[Crawler/Source] --> Processor[Cleaner/Chunker]
        Processor --> Tester[QA/Verification]
    end

    Tester --"Batch Push (Verified Artifacts)"--> API

    subgraph "CoCo Server (Retrieval Endpoint)"
        API[CoCo API Service (Rust)]
        Worker[CoCo Ingest Worker (Rust)]
        API --> DB[(PostgreSQL/VectorDB)]
        Worker --> DB
    end

    Consumer[App / Agent] --"Query"--> API
    API --"Final Results (Verified)"--> Consumer
```

---

### 2.5 存储策略 (One Stack)

- **元数据统一:** 使用 SeaORM，`postgres://` 与 `sqlite://` 由 `DatabaseConnection` 自动适配，无需泛型注入。
- **向量分层:** `StorageBackend` 作为**业务语义端口**（如 `search_similar_docs`），具体实现分别在 Local/Server 中绑定：
- Server Mode (Network): PostgreSQL + pgvector 或 Specialized VDB (Qdrant)。**不支持嵌入式数据库**。
  - Local Mode (Embedded): LanceDB (嵌入式文件型)。
- **Server 限制:** 虽然支持专业 VDB，但设计上 Server Mode 必须连接**网络服务**型数据库，禁止使用进程内嵌入式库。

#### 2.5.1 双轨存储的成本与解法 (Schism of Storage)

**问题:** Local 使用 LanceDB，Server 使用 PostgreSQL/pgvector，查询语法、索引参数（如 IVF-PQ vs HNSW）与 FTS 细节差异巨大，容易导致检索逻辑漂移。

**原则:** 拒绝过度抽象（No-DI）不等于业务逻辑直写 SQL 或 LanceDB Builder。**算法意图必须统一**。

**解法:** 引入轻量级查询 DSL / 中间层：

- 定义 `SearchIntent`（检索意图结构体），描述：
  - `retrieval_mode`（vector/fts/hybrid）
  - `top_k`、`hybrid_alpha`、`filters`
  - `reranker` 等算法层参数
- 分别实现 `PgExecutor` 与 `LanceExecutor`：
  - 负责将 `SearchIntent` 映射为各自的查询语言与索引参数
  - 保持算法一致、实现可替换
- **端口语义优先:** `StorageBackend` 方法应表达业务语义（如 `search_similar_docs`），避免抽象成 `execute_sql` 等数据库语义。

**收益:** Shared Core 只维护一套检索策略与调参逻辑，Local/Server 仅在执行层分叉，避免逻辑漂移。

---

## 3. 核心功能规范 (Functional Specifications)

### 3.1 启动与生命周期 (Lifecycle)

- **显式启动 (Strictly Manual Start):**

  - Local Mode 必须用户执行 `coco start`。
  - 客户端连接失败时仅提示“请先启动 CoCo 服务”。

- **模型预配置 (Model Provisioning - Local Mode):**

  - **首次启动检测:** `coco start` 检查 `~/.coco/models/` 目录。
  - **缺失模型时:**
    1.  显示 TUI 进度条，从 HuggingFace Mirror / CDN 拉取模型 (e.g., `all-MiniLM-L6-v2.onnx`)。
    2.  支持 `--model-url <URL>` 自定义镜像源（适配国内网络）。
    3.  下载失败提供清晰指引：`coco setup --model-path /path/to/local.onnx`。
  - **零配置目标:** 对于大多数用户，首次 `coco start` 应在 30 秒内完成下载并进入服务状态。
  - **UX 关键点:** 这一步是用户流失的高风险点，必须设计得极致丝滑。

- **协议策略: Public JSON + Internal Binary:**

  - **Public API:** Control/Data Plane 统一 JSON（`application/json`）。
  - **Internal IPC:** API Service <-> Worker 允许使用 gRPC/Protobuf 或 Arrow IPC，避免 JSON 序列化开销。

- **端口幂等与双重确认:**

  - 启动时检测 3456 端口，先 Ping `/v1/sys/health`。
  - 返回 200 且包含 CoCo 签名 → 视为已运行，输出状态并成功退出。
  - Ping 失败但 listen 报 `EADDRINUSE` → 端口被其他应用占用，报错退出。

- **Smart Lock Check:**
  - 检查 `~/.coco/service.lock`，读取 PID 并验证进程存活。
  - 不存在则清理锁文件并继续启动。

```rust
fn is_pid_running(pid: i32) -> bool {
    match nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid), None) {
        Ok(_) => true,
        Err(nix::Error::Sys(nix::errno::Errno::EPERM)) => true,
        Err(_) => false,
    }
}
```

- **Windows 注意:** PID 检测需使用 Windows API 或 `sysinfo` 等库实现。

- **Bind & TLS 默认策略:**

  - 默认只绑定 `127.0.0.1`。
  - 需显式 `--host 0.0.0.0` 或 `COCO_HOST` 才允许对外暴露。
  - Server Mode 必须启用 TLS（内置证书或反向代理终止）。

- **认证要求:**

  - Server Mode 必须启用 Master Key / API Key。
  - `project_id` 不能替代认证。

- **进程模型:**
  - Local Mode：单一 Rust Service。
  - Server Mode：Rust API Service + Rust Worker（不由 API Spawn）。
- **版本握手:**

  - API Service 与 Worker 必须进行版本校验（严格锁定主版本）。
  - 版本不匹配必须报错并拒绝服务。

- **流式返回与内存稳定 (Service):**
  - 对大结果采用流式响应，避免全量缓冲。
  - 设计目标是长期运行的稳定内存占用。

### 3.2 项目注册与安全 (Registry & Security)

**适用范围:** Server/Local API 契约可分离。**Server Mode** Public API 严禁接受任何文件系统路径。

- **项目注册 (Admin Only):**

  - **Server Mode:** `POST /v1/sys/register`
    - Body: `{ "source_ref": "kb:project-alpha", "name": "Project", "platform": "github" }`
    - **标识约束:** `source_ref` 为逻辑来源标识，不得是本地路径或可解析为本地路径的字符串。
    - Response: `project_id`, `active_config_id`, `active_version_id`, `org_id`
  - **Local Mode:** 可接受 `path`（绝对/相对路径）作为导入来源。

- **业务查询 (Public):**
  - `POST /v1/docs/query`
  - Header: `x-coco-org-id`, `x-coco-user-id`, `x-coco-project-id`
  - **Server Mode:** 返回结果仅包含 `doc_id` / `source_ref` / `title` 等引用元数据，不包含文件路径。
  - **Local Mode:** 可返回 `path` / `line` 等本地导航字段（可选），用于编辑器跳转与定位。

### 3.3 数据摄取与计算 (Ingestion & Compute)

- **计算下沉:**

  - Embedding 与 AST 解析全部由 Rust Service/Worker 执行。
  - 数据流: `API Service` -> `Ingest Worker`.
  - **内部协议:** API Service 与 Worker 之间优先使用 gRPC/Protobuf 或 Arrow IPC，避免 JSON 在高频/大数据场景的开销。

- **控制/数据分离 (Control vs Data Plane):**

  - **API Service 负责编排:** 鉴权、策略选择、任务编排与状态查询。
  - **Worker 负责处理:** 解析、向量化、检索与写入。
  - 大数据只允许“引用传递”，避免在 API Service 内部全量缓冲。

- **引用传递 (Pass-by-Reference):**

  - 大体积内容先写入临时存储（本地文件/S3/对象存储）。
  - API Service 仅传递 `blob_ref`/`url` 与 `plan` 到任务队列。
  - Rust Worker 直接拉取数据并写入最终存储。

- **Headless & Agnostic:** CoCo 是一个**无头上下文服务 (Headless Context Service)**，与具体的编辑器（VS Code/JetBrains）完全解耦。任何能够发起 HTTP/MCP 请求的终端（CLI, Web Agent, CI Runner）皆可成为其消费者。
- **基建而非插件:** 它是一个独立运行的**基础设施进程**。IDE 仅仅是众多 Consumer 中的一种。
- **增量同步与批处理 (Ingestion Protocol):**

  - **Server Mode (Pipeline Push):**
    - **API:** `POST /v1/ingest/batch`
    - **Input:** 接收上游处理好的 JSON/Parquet 数据流，包含 content, vector, meta(quality_score)。
    - **Blue/Green Indexing:** 支持原子性版本切换。新索引构建完成前，查询仍走旧版本（详见 3.4.1）。
  - **Local Mode (Direct):** Rust Service 直接扫描用户本地 FS。

- **数据来源与时效:**

  - **Server Mode:** 数据源为 **Upstream Pipeline**。时效性取决于 Pipeline 运行频率。
  - **Local Mode:** 数据源为 **Local Codebase**。实时。

- **资源池化:**

  - Rust Service/Worker 内部维护全局单例的 Embedding 模型。
  - 多项目共享，不重复加载。

- **多客户端并发:**

  - Service 必须支持 IDE/CLI/脚本同时连接，保持单进程内资源复用。
  - 批量索引应由 Worker 直接写入存储。

- **导入任务管理 (Import Jobs):**
  - 必须提供批量导入与递归导入（例如 `coco import --recursive ./workspace`）。
  - 必须提供任务进度推送（SSE/WebSocket）与可查询的 Job 状态。

#### 3.3.1 Server Mode: 双引擎拓扑 (Query vs Ingest)

目的：隔离查询低延迟与构建高吞吐，避免批量导入抢占查询资源。

| 特性     | **Query Service (查询专用)** | **Ingest Worker (构建专用)** |
| :------- | :--------------------------- | :--------------------------- |
| 服务对象 | API Service (实时响应)       | Worker Service (离线任务)    |
| 资源侧重 | 低延迟 (Latency)            | 高吞吐 (Throughput)          |
| 批处理   | `BATCH_SIZE=1`              | `BATCH_SIZE=64`              |
| 超时策略 | 短 (e.g., 5s)               | 长 (e.g., 30m)               |
| 扩容指标 | QPS / CPU                   | 队列积压 / CPU               |

#### 3.3.2 执行计划 (Execution Plan)

目的：让 Rust 可执行“可配置策略”，API Service 负责生成计划并快速迭代业务逻辑。

- **计划输入:** `IndexingPlan` / `QueryPlan`（JSON/YAML），由 API Service 生成并传给 Worker。
- **计划内容:** 复用 3.6 的 `indexing_config` / `retrieval_config`，并可扩展额外策略字段。
- **收益:** 业务变更不必修改 Rust，仅调整 Plan 即可生效。

#### 3.3.3 可选扩展: WASM 规则注入

为少数定制逻辑提供可编程扩展能力（如特殊脱敏/预处理规则）。

- API Service 可为特定任务附带 `wasm_module_ref`。
- Rust Worker 使用 `wasmtime` 执行 WASM 规则，避免核心逻辑硬编码。

### 3.4 存储与可靠性 (Storage & Durability)

> [!CAUTION] > **PostgreSQL Extension 权限警告**
> 如果选择 PostgreSQL 作为后端，用户**必须**拥有数据库的 `CREATE EXTENSION` 权限以安装 `vector` 扩展。
> 对于许多托管数据库（如 RDS / Neon 等），需确认是否已开启 pgvector 支持。若无权限，服务启动将失败。

- **存储后端灵活度:**

  - **Server Mode (Network Only):**
    - **推荐:** PostgreSQL + pgvector (运维简单)。
    - **允许:** Specialized Vector DB (Qdrant)。
    - **禁止:** 不支持任何嵌入式数据库 (如 LanceDB)，必须使用独立服务。
  - \*\*Local Mode (Embedded Only):
    - **推荐:** LanceDB (嵌入式文件型)，无需额外服务进程。

- **Tenant Key:** 向量数据必须包含 `project_id` 字段并强制过滤。
- **Source of Truth:**
  - **Server Mode:** 只有数据库内容被视为事实。
  - **Local Mode:** 磁盘文件为真相，数据库仅为缓存与索引加速。
- **Upsert 规则:** 必须定义唯一键（默认 `project_id + doc_id`），重复导入需覆盖或版本化。
- **TTL/GC:** 必须支持 TTL 与清理接口（`coco prune` / `POST /v1/sys/prune`），防止存储无限膨胀。

#### 3.4.1 影子索引与原子切换 (Shadow Indexing & Atomic Swap)

**目标:** “完全解析完成才可用”，避免用户看到半成品索引。

- **版本化写入:** 为每次构建生成 `version_id`，写入时所有数据带 `version_id`。
- **指针切换:** `projects.active_version_id` 指向当前对外可用版本。
- **原子切换:** 构建完成后在事务中切换指针，旧版本标记为 `ARCHIVED`。

建议字段：

- `projects(active_version_id)`
- `project_versions(id, project_id, status, created_at, item_count)`
- `source_chunks(project_id, version_id, content, embedding, meta)`

#### 3.4.2 版本保留与回收 (GC / Retention)

- 保留最近 N 个版本或最近 T 小时（用于回滚与对比）。
- 低优先级任务清理 `ARCHIVED` 版本数据，避免存储无限膨胀。

### 3.5 分层检索策略 (Tiered Retrieval Strategy)

**适用范围:** 本节为 **Local Mode** 的检索策略（包含 FS 层）。Server Mode 仅使用 DB 层（关键词/语义），不涉及 FS。

基于**“元数据校验”的混合检索策略 (Hybrid Retrieval with Liveness Check)**。旨在“时效性”与“检索能力”之间寻找平衡，避免盲目的全盘扫描竞争。

#### 3.5.1 三层级定义 (The Three Layers)

| 层级            | 数据源            | 技术手段                    | 优势                    | 劣势                               | 典型场景                        |
| :-------------- | :---------------- | :-------------------------- | :---------------------- | :--------------------------------- | :------------------------------ |
| **1. 实时层**   | 本地文件系统 (FS) | Terminal CMD (grep/ripgrep) | **绝对实时**，100% 准确 | **无语义**，仅限关键词，全盘扫描慢 | 查具体变量名、Verify、Live Grep |
| **2. 关键词层** | 数据库 (DB)       | SQL (LIKE / FTS / tsvector) | 速度极快，支持复杂过滤  | 依赖索引更新，**无语义**           | 精确匹配，元数据查询            |
| **3. 语义层**   | 数据库 (DB)       | Vector (pgvector/LanceDB)   | **懂语义** (搜概念)     | 依赖索引更新，构建慢，有精度损失   | 模糊搜索，知识关联              |

#### 3.5.2 核心逻辑：依赖而非竞争

**核心原则：** 不要让 1 (FS) 与 2/3 (DB) 并行竞争。

1.  **2 & 3 (DB)** 负责 **“找文件”** (Discovery)。
2.  **1 (Local FS)** 负责 **“读真相”** (Read Truth) 和 **“校验”** (Validate)。

#### 3.5.3 检索流程 (Retrieval Pipeline)

**场景 A: Server Mode (Pipeline Retrieval)**

- **原理:** Server 提供的是经过验证的高质量数据。
- **流程:**
  1.  **Search:** 并行执行 FTS + Vector。
  2.  **Return:** 直接返回结果。**不**做任何客户端校验/Hydration。
  3.  **Metadata:** 返回包含 `quality_score`, `verified` 等 Pipeline 注入的元数据。

**场景 B: Local Mode (Live Retrieval)**

- **原理:** 必须保证 IDE 内的所见即所得。
- **流程:**
  1.  **Search:** 查库发现。
  2.  **Validation:** Service 校验 FS mtime。
  3.  **Hydration:** Service 读取 FS 覆盖旧内容 (Read-Only fix)。
  4.  **Fuzzy Anchor:** 若行号漂移，使用 snippet/hash 在前后窗口（如 ±20 行）内快速匹配并修正坐标，避免返回错误片段。
  5.  **Return:** Local Mode 可附带路径/行号字段用于导航，内容经过本地校验/补全。

**步骤 4: 兜底 (Fallback) -> Slow Path (Common)**

- 仅在 Local Mode 或 Server Mode 支持且开启特殊配置时可用。
- 在 Local Mode 下调用 Live Grep 补充结果。

---

### 3.6 可调参数与实验框架 (Tunable Parameters & Experiments)

目标：将 CoCo 从“黑盒工具”升级为**可度量、可优化的白盒平台**，支持企业级的参数对比与效果评估。

#### 3.6.1 索引时参数 (Indexing-Time Parameters)

这些参数在数据写入时生效，**改变后必须重建索引**。

- **索引策略 (Indexing Strategy):** 用 `config_id` 标识一套可复用的策略配置。
- **分块策略 (Chunking Strategy):**
  - `strategy_name`: `markdown_header` / `fixed_token` / `recursive_char`
  - `chunk_size`: 目标大小（token 或字符）
  - `chunk_overlap`: 相邻块重叠
- **嵌入模型 (Embedding Model):**
  - `model_name`: `all-MiniLM-L6-v2` / `bge-large-en-v1.5` / `text-embedding-3-small` / 自定义模型
- **存储约束:** 每条向量与元数据必须带上 `config_id`，**同项目可并行维护多套索引**。

索引配置需先通过 `/v1/sys/configs` 注册，再在导入/查询时通过 `indexing_config_id` 选择。

请求示例（导入/批量写入时传入 `indexing_config_id`）：

```jsonc
{
  "project_id": "...",
  "items": [ ... ],
  "indexing_config_id": "strategy_big_chunks_bge_model"
}
```

#### 3.6.2 查询时参数 (Query-Time Parameters)

这些参数在检索时生效，**无需重建索引**，适合在线实验。

- **检索策略 (Retrieval Strategy):**
  - `retrieval_mode`: `vector` / `fts` / `hybrid`
  - `top_k`: 初筛返回数量
  - `hybrid_alpha`: 向量与 FTS 权重（`1.0`=纯向量，`0.0`=纯关键词）
- **重排序 (Re-ranking, Optional):**
  - `model_name`: Cross-Encoder 模型
  - `rerank_top_n`: 参与重排的候选数量
- **索引选择:** `indexing_config_id` 指定查询哪套索引版本；缺省时使用项目默认策略。

请求示例（查询时传入 `retrieval_config`）：

```jsonc
{
  "query": "如何配置数据库连接池？",
  "indexing_config_id": "strategy_big_chunks_bge_model",
  "retrieval_config": {
    "retrieval_mode": "hybrid",
    "top_k": 20,
    "hybrid_alpha": 0.7,
    "reranker": {
      "model_name": "bge-reranker-large",
      "rerank_top_n": 10
    }
  }
}
```

#### 3.6.3 批量测试与评估框架 (Experiment Framework)

通过实验定义文件与 CLI，完成批量对比与指标评估。

- **实验定义文件 (`experiment.yml`):** 定义多套索引策略 + 查询策略 + 评估集（Ground Truth）。
- **CLI:**
  - `coco experiment run <experiment.yml>`: 生成多套索引，批量查询，计算 Recall@K / MRR / Hit Rate / Latency。
  - `coco experiment compare <results.json>`: 汇总并输出对比表。

示例：

```yaml
name: "Chunking Strategy vs. Embedding Model Test"
project_id: "your-project-id"

indexing_strategies:
  - config_id: "small_chunks_minilm"
    chunking:
      { strategy_name: "fixed_token", chunk_size: 256, chunk_overlap: 50 }
    embedding: { model_name: "all-MiniLM-L6-v2" }
  - config_id: "large_chunks_bge"
    chunking:
      { strategy_name: "fixed_token", chunk_size: 1024, chunk_overlap: 200 }
    embedding: { model_name: "bge-large-en-v1.5" }

evaluation_set:
  - question_id: "q1"
    query: "如何启动服务？"
    expected_doc_ids: ["doc1.md#section-2", "doc5.md#setup"]
  - question_id: "q2"
    query: "数据库配置在哪里？"
    expected_doc_ids: ["config.md#database"]

query_strategies:
  - query_config_id: "pure_vector"
    retrieval_config: { top_k: 5, hybrid_alpha: 1.0 }
  - query_config_id: "hybrid_search"
    retrieval_config: { top_k: 5, hybrid_alpha: 0.5 }
```

---

## 4. 技术栈实施细节 (Implementation Stack)

| 组件         | 角色      | 技术选型                                              | 关键说明                                                       |
| :----------- | :-------- | :---------------------------------------------------- | :------------------------------------------------------------- |
| **Service**  | 控制/业务 | Rust                                                  | Axum/Tonic, 负责 HTTP/Auth/Registry。                          |
| **Worker**   | 计算/推理 | Rust                                                  | 解析/向量化/写入，集成 `ort` 与 `tree-sitter`。                 |
| **Metadata** | 元数据    | SeaORM                                                | 统一 `postgres://` 与 `sqlite://` 适配。                       |
| **Storage**  | 向量存储  | Server: PG/VDB (Network)<br>Local: LanceDB (Embedded) | 云端必须使用网络服务 (PG/Qdrant)；<br>本地使用嵌入式 LanceDB。 |
| **Clients**  | 消费者    | SDK / MCP Protocol                                    | 任何支持 HTTP/MCP 的 Runtime (Agent, IDE, CI)。                |
| **CLI**      | 命令行    | Rust                                                  | Local 启动器与管理工具。                                       |

### 4.1 技术栈差异 (Stack Divergence)

| 组件            | Local Mode (Edge Compute)                                        | Server Mode (High Quality)                                   |
| :-------------- | :--------------------------------------------------------------- | :----------------------------------------------------------- |
| **Runtime**     | `coco-local` Binary                                              | `coco-server` Container                                      |
| **Embedding**   | **ort** (ONNX Runtime) <br> 模型: `all-MiniLM-L6-v2` (Quantized) | **HTTP Client** <br> 模型: `text-embedding-3-small` (OpenAI) |
| **Metadata**    | **SQLite** (Native)                                              | **PostgreSQL** (Relational)                                  |
| **Vector**      | **LanceDB** (Embedded, Zero-Socket)                              | **pgvector** (Transactional)                                 |
| **I/O**         | Direct FS (Watch/Read)                                           | No Access (Git-Ops Only)                                     |
| **Consistency** | Eventual Consistency                                             | ACID (Meta + Vector Transaction)                             |

> [!IMPORTANT] > **Server Worker 职责明确化:**
> 即使 Embedding 由 OpenAI API 提供，Rust Worker 在 Server Mode 下依然**不可或缺**：
>
> - **AST Parsing (Tree-sitter):** 将代码解析为语义结构（CPU Bound），必须由 Rust 承担。
> - **Smart Chunking:** 将长文件智能切分为 512 Token 的语义块，保留上下文完整性。
> - **Pipeline:** Worker 执行 `Parse -> Chunk -> Call OpenAI -> Store`。它是**计算单元**，而非 API 转发器。

#### 4.1.1 文档解析与智能分块 (Tree-sitter 选择理由)

**前提：** CoCo 的核心场景是**文档**（Markdown、Confluence 页面、Notion 导出等），同时必须兼顾**代码场景**（仓库源码、配置、脚本等）。目标是**语义级别**的智能分块，而非渲染。

- **目标差异：渲染 vs 语义理解**

  - `remark/unified/marked/cheerio` 的生态重心是渲染与格式转换（HTML、目录、链接校验等）。
  - CoCo 关心的是**结构理解**与**语义分块**：按层级切分、保留标题-段落-代码-表格的上下文关系。

- **智能分块的结构约束**

  - 典型规则：**按二级标题切分**，但**保持标题下的列表/代码块/表格不被拆开**。
  - 在 MDAST 上实现这一点需要较复杂的遍历与节点重组；而 `tree-sitter-markdown` 提供**更精确的 CST**，可用查询规则直接选中区间边界，逻辑更清晰、结果更稳定。

- **混合内容的关键优势（代码块嵌入）**

  - 文档中的代码块是高价值信息密度区域，必须被理解而不是当作纯文本。
  - `tree-sitter` 支持**语法嵌入**：解析 Markdown 时遇到 ` ```python ` 可自动切换 `tree-sitter-python`，最终得到**单一统一语法树**。
  - 这使 CoCo 能将“函数说明 + 示例代码 + 参数表格”作为一个完整语义块提取，避免“多解析器拼装”的复杂与脆弱。

- **代码场景的直接收益**

  - 对源代码的结构化理解（函数/类/注释/块级语义）是高质量 chunking 的前提。
  - `tree-sitter` 在代码场景下天然匹配：统一解析后端、稳定 CST、跨语言一致的查询规则。
  - 结果是同一套引擎可同时处理“纯代码仓库”和“文档+代码混合仓库”，避免双栈解析与规则漂移。

- **多语言语法膨胀与可扩展性（WASM 插件化为核心）**

  - 直接静态链接多语言 Grammar 会导致二进制体积增长、`build.rs` 复杂度上升、版本发布压力增大。
  - **必须: Tree-sitter WASM 插件化**
    - Rust 通过 `wasmtime` 动态加载 `.wasm` Grammar，Service/Worker 仅内置最小 Core（如 Markdown/Plain）。
    - 语法包存放在 `~/.coco/grammars/`，按文件类型按需加载（如检测到 `.vue` 时加载 `tree-sitter-vue.wasm`）。
    - 若本地缺失，CLI 自动从 Release 拉取并缓存（可配置镜像源）。
    - 新增语言无需发布新二进制，仅发布/下载 Grammar 包。
  - **不优先:** 跨平台动态库（`.so/.dll`）装配复杂、维护成本高。

- **性能与架构一致性**
  - 大规模知识库（GB/TB）全量索引更依赖 Rust 的并行与内存效率。
  - 统一解析后端降低未来扩展成本（日志、Notebook、其他结构化文本）。

### 4.2 分发与部署 (Distribution)

- **Server First (Docker):**

  - `coco-api:v0.0.1` 与 `coco-worker:v0.0.1` 作为一等公民。
  - 镜像别名：`coco-api` 与 `coco-server` 等价（同一二进制）。
  - 推荐 docker-compose/Helm 管理。
  - Helm Chart 入口：`deploy/helm/coco`。
  - **Extension Requirement:** 若使用 PostgreSQL，部署前必须确认数据库用户拥有 `CREATE EXTENSION` 权限。
  - **Flexibility:** 虽然推荐 PG 单栈，但架构允许配置独立的 Vector DB 地址（如 Qdrant 集群）。

- **Local Optional:**
  - **优先:** 提供常用平台的预编译二进制（GitHub Releases/Homebrew/Scoop）。
  - **补充:** 提供 `coco setup` 允许手动指定本地二进制路径或重试下载。
  - **Grammar 包:** `tree-sitter` 语法以 WASM 形式按需下载到 `~/.coco/grammars/`，不随引擎二进制打包。
  - `coco start` 仅用于本地单机场景。

### 4.2 命令行体验 (CLI DX - Local Mode)

- **TUI Dashboard:** `coco start` 默认开启 TUI 模式（`ratatui`）。
  - 实时展示: QPS, 内存占用, Embedding 队列长度, 最近索引文件。
- **Headless Mode:** 支持 `coco start --headless` 或 `CI=true`，仅输出结构化日志，便于 CI 集成。

### 4.3 多租户与配额 (Server Mode)

- **Strict Isolation:** 仅 `project_id` 不足以隔离。必须引入 `org_id` (对应 GitHub Org) 和 `user_id`。
- **Rate Limiting:** 必须实现 Token Bucket 限流，防止某个项目的批量导入耗尽 Embedding 资源。
- **Quota:** 限制每个 Org 的最大文件数和存储体积。

### 4.4 配置项 (Env/Flags)

| 配置项                             | 说明                                                                 |
| :--------------------------------- | :------------------------------------------------------------------- |
| `COCO_MODE`                        | 兼容字段（`local`/`server`），当前二进制忽略                         |
| `COCO_HOST`                        | 监听地址，默认 `127.0.0.1`                                           |
| `COCO_PORT`                        | 默认 `3456`                                                          |
| `COCO_META_DB`                     | 数据库连接（Server: `postgres://`，Local: `sqlite` 路径/URL）        |
| `COCO_DB_URL`                      | `COCO_META_DB` 的兼容别名                                            |
| `COCO_VECTOR_DIR`                  | Local Mode LanceDB 存储路径                                          |
| `COCO_LANCEDB_PATH`                | `COCO_VECTOR_DIR` 的兼容别名                                         |
| `COCO_ADMIN_KEY`                   | 管理端 Master Key                                                    |
| `COCO_API_KEY`                     | 客户端 API Key                                                       |
| `COCO_WORKER_ADDR`                 | Worker gRPC 地址（Server Mode 可选）                                 |
| `COCO_QUEUE_MODE`                  | 队列模式：`postgres`/`redis`                                         |
| `COCO_REDIS_URL`                   | Redis 队列地址（`COCO_QUEUE_MODE=redis` 时必填）                     |
| `COCO_REDIS_QUEUE`                 | Redis 队列名                                                         |
| `COCO_RATE_LIMIT_PER_MIN`          | 每分钟请求限制                                                       |
| `COCO_RATE_LIMIT_BURST`            | 限流突发容量                                                         |
| `COCO_ORG_MAX_DOCUMENTS`           | 组织级文档上限                                                       |
| `COCO_ORG_MAX_CHUNKS`              | 组织级 Chunk 上限                                                    |
| `COCO_ORG_MAX_STORAGE_BYTES`       | 组织级存储上限                                                       |
| `COCO_ORG_MAX_EMBEDDINGS_PER_DAY`  | 组织级每日嵌入调用上限                                               |
| `COCO_TLS_MODE`                    | TLS 模式：`tls`/`proxy`                                              |
| `COCO_TLS_CERT`                    | TLS 证书路径                                                         |
| `COCO_TLS_KEY`                     | TLS 私钥路径                                                         |
| `COCO_VECTOR_BACKEND`              | 向量后端：`pgvector`/`qdrant`                                        |
| `COCO_VECTOR_DB_URL`               | Qdrant 地址                                                          |
| `COCO_VECTOR_DB_API_KEY`           | Qdrant API Key                                                       |
| `COCO_VECTOR_DB_COLLECTION_PREFIX` | Qdrant collection 前缀（`qdrant` 时必填）                             |

---

## 5. API 协议概要 (Protocol)

统一入口端口：默认 **3456**。

### 5.1 Public Protocol (JSON Only)

- **Control Channels:** `application/json`

  - 使用场景: System Health, Registry, Job Status, Auth.
  - 目标: Human Readable, Easy Debugging.

- **Data Channels:** `application/json`
  - 使用场景: `POST /v1/docs/query` (Retrieval Results).
  - 原则: **一律 JSON**，不引入二进制协议。

#### 5.1.1 Public JSON 统一协议 (No Binary)

**原则:** 统一 JSON，避免混合协议带来的解析/调试成本。

- **前端体验优先:** DevTools 可直接查看 Response，减少 SDK 维护成本。
- **工程一致性:** 所有 Endpoint 只需维护一套结构定义，无需内容协商。

#### 5.1.2 类型契约自动化 (Type Safety + SDK)

**风险:** 仅生成 TS 类型无法保证运行时一致性（序列化命名、可空字段等），且前端仍需手写 SDK。

**方案:** 以 OpenAPI 作为中间层，生成**类型 + SDK**：

- **Rust -> OpenAPI:** 使用 `aide` 或 `utoipa` 导出 OpenAPI JSON。
- **OpenAPI -> SDK:** 使用 OpenAPI Generator 生成 TypeScript Axios/Fetch Client。
- **收益:** 自动处理 URL 拼接、Query 序列化、错误处理与重试，并减少“类型幻想”导致的运行时崩溃。

#### 5.1.3 Passthrough Streaming (过境不入境)

**风险:** Server Mode 中 API Service 若解析大体积响应（100MB 级向量/文档），会导致内存膨胀与延迟抖动。

**原则:** API Service 只做**认证与路由**，对大数据流保持**全链路透传**。

- **允许:** 校验 Header / Token / Rate Limit。
- **禁止:** `await response.json()` / Buffer 全量读入 / 业务层解析。
- **实现:** 采用“智能 Pipe Wrapper”：
  - 先读取前几个 chunk 或检查响应头/状态码，若为错误则拦截并重写为标准错误格式。
  - 若为正常数据，再建立直通 pipe，避免全量缓冲。
  - 若未来出现基于 body 的鉴权/脱敏需求，需要显式走“解析路径”，避免破坏流式承诺。

### 5.2 Response Envelope (Logical Structure)

JSON 响应结构如下（以检索结果为例）：

```jsonc
{
  "meta": {
    "status": "fresh"
  },
  "data": {
    "results": [
      {
        "meta": {
          "score": 0.85,      // Vector Similarity
          "quality": 0.92,    // Pipeline QA Score
          "verified": true    // Test Passed
        },
        "chunk": { ... }
      }
    ]
  }
}
```

- `ResponseMeta.status` 只描述请求新鲜度（`fresh`/`stale`）。
- 评分字段属于 `SearchHitMeta`（`data.results[].meta`），不在 `ResponseMeta`。
- Memo 查询始终返回 `fresh`；`stale` 仅出现在 Server 显式查询非 active `indexing_config_id` 时。

### 5.2 Endpoints

#### System Domain (Admin/Infra)

- `GET /v1/sys/health` : API Service + Worker 状态与签名（含版本号）。
- `POST /v1/sys/register` : Server 注册 `source_ref`；Local 可注册 `path`，返回 `project_id`（Admin Only）。
- `POST /v1/sys/prune` : 执行 TTL/GC 清理（Admin Only）。

#### Document Domain (Docs Service)

- `POST /v1/docs/import` : 触发导入任务（需 `project_id`，使用 `indexing_config_id`）。
- `POST /v1/docs/index` : 触发刷新/重建（需 `project_id`，可指定 `indexing_config_id`）。
- `POST /v1/docs/query` : 语义检索（需 `project_id`，支持 `indexing_config_id` 与 `retrieval_config`）。
- `POST /v1/ingest/batch` : 直接批量写入嵌入向量（Server 专用）。
- `/v1/docs/import` 与 `/v1/ingest/batch` 在 Server 侧共享同一批量写入请求体。

#### Memo Domain (User Service)

- `POST /v1/memo/query` : 查询用户笔记（需 `session_token`）。

#### Job Domain (Async Tasks)

- `GET /v1/jobs/:id` : 查询任务状态与进度。
- `GET /v1/jobs/:id/events` : SSE/WS 任务进度与阶段事件。

**Headers**

- `Authorization: Bearer <api_key>` (Server Mode 必须)
- `x-coco-org-id: <org>` (Docs 必填)
- `x-coco-user-id: <user>` (Docs 必填)
- `x-coco-project-id: <project>` (Docs 必填)

### 5.3 对齐检查清单 (Alignment Checklist)

当以下内容发生变更时，必须同步更新 `TASK.md` 与 `tasks.md`：

- 环境变量命名/含义（Env/Flags）。
- API 端点列表与请求结构。
- 公共请求头约束（如 `x-coco-*-id`）。

---

## 6. 开发路线图 (Roadmap)

### Phase 1: Engine Foundation (Rust)

- [ ] 构建 `coco-core` / `coco-protocol` Rust 项目。
- [ ] 实现 Embedding 接口 (ONNX Runtime)。
- [ ] 实现 Tree-sitter 解析接口。
- [ ] 实现 Tree-sitter WASM Grammar Loader（`wasmtime`），支持按需加载与缓存。
- [ ] 实现 `StorageBackend` 端口（业务语义）并在 Local/Server 中绑定：pgvector / LanceDB。
- [ ] 建立 CI 流程，产出跨平台二进制文件。

### Phase 2: Service Core (Rust)

- [ ] 实现 API Service（HTTP/Auth/Registry）。
- [ ] 实现 Worker 队列消费与任务执行。
- [ ] 实现 Smart Lock 与端口幂等逻辑。
- [ ] 实现导入任务队列与 Job 进度 API。
- [ ] 提供 `coco import` / `coco prune` CLI。
- [ ] 提供 `coco grammar install/update`（按需下载 `~/.coco/grammars/`）。

### Phase 3: Server-First Delivery

- [ ] 发布 Docker Images (`coco-api`, `coco-worker`)。
- [ ] 提供 docker-compose / Helm 示例。
- [ ] 引入 Master Key / API Key 鉴权与 TLS 规范。

---

## 7. 总结 (Conclusion)

CoCo v0.0.1 通过 **Physical Separation** 与 **Pipeline Integration** 完成了架构收敛：

1. **控制面/计算面分离**，保障可维护性与性能。
2. **Docker First**，满足服务端可运维性与部署确定性。
3. **安全默认**（localhost 绑定 + 强制鉴权 + TLS）。

这是一个面向现代团队与个人开发者的可持续 AI Context 基建方案。
