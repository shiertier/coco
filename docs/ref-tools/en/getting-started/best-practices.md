# Best Practices

> How to prompt coding agents to use Ref effectively.

## TLDR

1. **Use the provided `search_docs` and `my_docs` prompts** - When you know your prompt will require working with a tricky library, use Ref's default prompts.
2. **Guide the model with rules files** - Set up a project rules file so the model always knows when you want it to use Ref.

# How coding agents use Ref

Coding agents leverage Ref's MCP tools to access documentation at the right moment. Understanding when and how agents use Ref helps you guide them more effectively.

### 1. When explicitly guided by the user

Agents use Ref when you explicitly instruct them to search documentation. This is the most reliable way to ensure agents access the latest docs.

**Example prompts:**

* `implement cors in my firebase functions, check docs with ref`
* `/Ref:search_docs langchain streaming callback handler implementation python`
* `look up how i explained ref.tools in my yc application in my private docs`

**Ref's built-in prompts:**
Ref provides two autocomplete prompts in compatible clients:

* **`search_docs`** - Guides agents to search public documentation
* **`my_docs`** - Directs agents to search your [private resources](/resources)

Typing `/Ref` and your IDE or CLI will autocomplete these prompts.

### 2. When encountering lint errors

When a coding agent detects errors related to library or API usage, they will search Ref for documentation without user guidance.

**Example flow:**

1. Agent hallucinates a function name that doesn't exist.
2. Linter flags the error.
3. Agent searches relevant documentation and applies the correct pattern.

The best way to encourage this behavior is with a simple prompt in an `AGENTS.md` or similar file.

# Guiding Agents with Rules Files

Rules files let you provide consistent instructions to coding agents. They're especially powerful for directing agents to use Ref automatically.

### Example Rules File Content

Rules files typically include project-specific instructions that guide AI agents in how to work with your codebase. For Ref integration, they should specify when and how to search documentation. They should generally be simple and direct.

**✅ Recommended for everyone**

It's helpful to provide a basic level of guidance to the agent that you want it to check docs with Ref.

```
When working with libraries, check the docs with Ref.
```

**✅ For sharing context between repos**

Ref is great for sharing context between repos. For example if you have microservices or client-server in different repos.

```markdown  theme={null}
When working with <my-internal-service> APIs, search my private docs with Ref.
```

**❌ Do not over-prompt**

It's very unlikely your agent would need to search with Ref on every prompt.

```markdown  theme={null}
Always start by checking the docs with Ref.
```

### Rules Files

Different AI coding tools support different rules file formats:

| File                      | Location                   | Supported Tools                                                  | Description                                                                      |
| ------------------------- | -------------------------- | ---------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| `AGENTS.md`               | Project root               | Cline, Windsurf, Roo Code, Goose, and other MCP-compatible tools | Vendor-neutral instructions file that works across multiple AI coding assistants |
| `.cursorrules`            | Project root               | Cursor (legacy)                                                  | Legacy rules file for Cursor IDE                                                 |
| `.cursor/rules/`          | `.cursor/rules/` directory | Cursor                                                           | Recommended location for Cursor rules files                                      |
| `CLAUDE.md`               | Project root               | Claude Code, Claude API integrations                             | Claude-specific instructions with tool-specific guidance                         |
| `.windsurfrules`          | Project root               | Windsurf                                                         | Windsurf Cascade-specific rules                                                  |
| `copilot-instructions.md` | `.github/` directory       | GitHub Copilot                                                   | GitHub Copilot-specific instructions                                             |

**Recommendation:** For maximum compatibility, use `AGENTS.md` as your primary rules file. It's supported by most modern AI coding tools that implement MCP or read standard project files.

## Next Steps

<Card icon="cubes" title="Add Private Resources" href="/resources/index">
  Build your own custom index of private documentation sources for you or your team.
</Card>


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt