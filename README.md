# docs-crawler

一个使用标准库实现的简单爬虫，用于抓取文档站点的 `*.md` / `*.mdx` 文件，作为 LLM/RAG 的本地参考语料。

## 快速开始

- 抓取所有来源：`python3 -m docs_crawler crawl`
- 只抓取某个来源：`python3 -m docs_crawler crawl --source pydantic-ai`
- 删除不再被发现的旧文档：`python3 -m docs_crawler crawl --source pydantic-ai --prune`

## 日志与进度

默认输出 `INFO` 日志，包含发现、抓取进度和错误信息；可用 `--log-level` 调整（如 `WARNING` 仅显示错误）。

## 配置

配置文件为 `sources.json`，核心字段：

- `sources[].id`：来源标识（用于 `--source`）
- `sources[].output_dir`：输出目录（每个来源一个目录）
- `sources[].default_language`：默认语言（例如 `en`）
- `sources[].allowed_extensions`（可选）：覆盖全局 `defaults.allowed_extensions`；可使用 `["*"]` 接收任意后缀
- `sources[].max_workers`（可选）：覆盖全局 `defaults.max_workers`，控制该来源的并发抓取线程数
- `sources[].entrypoints[]`：入口列表，目前支持：
  - `type: "llms_txt"`：从 `llms.txt` 中解析匹配 `allowed_extensions` 的链接并抓取
  - `language`（可选）：如果链接路径里没有语言前缀，可用它给该入口的文档打语言标签
  - `type: "github_tree"`：从 GitHub 仓库的目录树中发现并抓取文档
    - `repo`：形如 `owner/name` 的仓库
    - `ref`（可选）：分支或提交，默认 `main`
    - `path`：仓库内的文档目录
    - `languages`（可选）：仅抓取指定语言（例如 `["en", "zh", "ja", "ko"]`）
    - `language`（可选）：未带语言前缀的文件使用该语言标签
    - 如遇到 GitHub API 速率限制，可设置环境变量 `GITHUB_TOKEN` 或 `GH_TOKEN`
    - API 失败时会尝试使用系统 `git` 进行回退（需本机已安装 git）
    - `git_cache_dir`（可选）：git 回退时的本地缓存目录；默认 `output_dir/.git-cache`
  - `language_variants`（可选）：需配合 `language` 使用，在 `language` 基础上生成额外语言版本；会把 URL 路径中匹配 `language` 的段替换为对应语言

## 输出结构与更新逻辑

每个来源的输出目录包含：

- `.manifest.json`：缓存/状态清单（按 URL 记录 `etag`/`last_modified`/`sha256` 等）
- `.manifest.json` 中的 `documents_order` 保留 `llms.txt` 的原始顺序
- `<lang>/.../*.md(x)`：按语言分目录存储的文档
- 无后缀的 URL 会保存为 `.html` 文件以避免目录冲突

重复执行 `crawl` 会利用 `ETag`/`If-None-Match` 等机制增量更新；如果文档未变化，会得到 `304 Not Modified` 并跳过下载。

## 路径规则

当 URL 以 `/index.md` 或 `/index.mdx` 结尾时，会折叠为同级的 `*.md(x)` 文件：

- `.../guide/index.md` -> `.../guide.md`
