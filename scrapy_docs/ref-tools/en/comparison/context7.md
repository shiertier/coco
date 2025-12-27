# Ref vs Context7

> Key differences between the popular documentation search MCP servers.

## Snapshot

|                       | Context7                                 | Ref                                        |
| --------------------- | ---------------------------------------- | ------------------------------------------ |
| Approach              | Simple batch retrieval                   | Iterative search + read with sessions      |
| Token Usage           | 10K tokens per query                     | 50-70% savings on average, up to 95%       |
| Tools                 | `resolve-library-id`, `get-library-docs` | `ref_search_documentation`, `ref_read_url` |
| Advanced MCP features | None                                     | Stateful search session, pre-built prompts |
| Scrape any URL        | No                                       | Yes                                        |

## Search Philosophy

### Context7's Approach

Context7 uses a **naive RAG strategy**: given a query, fetch the most relevant documents up to a fixed number of tokens (typically 10K). This "dump and hope" approach assumes what you need is in that bundle of content.

**Strengths:**

* Simple, predictable behavior
* One of the most popular MCP servers showing why MCP is valuable

**Limitations:**

* Doesn't match iterative agent/human search patterns
* Fixed token budget may be too much or too little
* Costly to iterate (10K tokens per retry)
* Cannot avoid returning duplicate results across queries

### Ref's Approach

Ref uses **agentic search with MCP sessions**: provides search() and read() tools, allowing agents to:

1. Issue queries and get result overviews
2. Selectively read only relevant documents
3. Iterate efficiently with session state

**Session-powered improvements:**

* **Never return same link twice** - agents can access prior results from context
* **On-the-fly extraction** - automatically filter large pages (e.g., 90K token Figma docs → 5K relevant tokens)
* **Pre-fetching** - results are cached for faster reads

**Results:**

* 50-70% average token savings vs 10K baseline
* Up to 95% savings on some queries (500 tokens vs 10K)
* No drop in recall quality

## Evaluation Metrics

Ref optimizes for **token usage** because:

* Tokens cost money
* [Context rot](https://research.trychroma.com/context-rot): irrelevant tokens degrade output quality
* Agents build context over multiple searches, so session-level metrics matter more than single-query precision

## Why Ref Wins

**Dramatic token savings:** 50-70% average reduction vs naive RAG approaches, with up to 95% savings on some queries. This translates to real cost savings and better output quality through reduced context rot.

**Matches how agents work:** Ref's search + read tools align with how frontier models are trained. OpenAI explicitly requires this pattern for Deep Research integration, signaling this is the future of agentic search.

**Session-powered intelligence:**

* Never returns duplicate links across queries
* Automatically extracts relevant sections from large docs (e.g., 90K token pages → 5K relevant tokens)
* Enables efficient iteration without re-fetching prior results

**Enterprise-ready:** Built-in GitHub, PDF, and Markdown indexing with team RBAC, no custom pipelines required.

## Lean more

Learn more about [how Ref evaluates agentic search](https://ref.tools/blog/how-make-search-good) and [how Ref leverages advanced MCP features](https://ref.tools/blog/how-does-ref-mcp) from the blog.


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt