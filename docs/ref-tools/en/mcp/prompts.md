# Prompts

> Pre-configured prompts available through the Ref MCP Server.

The Ref MCP Server includes pre-configured prompts to help you work more effectively with your documentation. These prompts follow the [MCP Prompts specification](https://modelcontextprotocol.io/specification/2025-06-18/server/prompts) and provide structured ways to search and access your documentation.

## Available Prompts

### `search_docs`

Guides your coding agent to search public documentation for libraries, frameworks, and APIs.

**Example usage:**

```
/Ref:search_docs langchain streaming callback handler implementation python
```

### `my_docs`

Directs your coding agent to search your [private resources](/resources), including indexed GitHub repositories and uploaded documentation.

**Example usage:**

```
/Ref:my_docs how we implement authentication in our backend
```

## Using Prompts

Prompts are designed to be **user-controlled** and are typically invoked through slash commands in your coding assistant.

**To use a Ref prompt:**

1. Type `/Ref` in your coding assistant
2. Your IDE or CLI will autocomplete the available prompts
3. Select either `search_docs` or `my_docs`
4. Add your search query

## Best Practices

For more information on how to effectively prompt coding agents to use Ref, see our [Best Practices guide](/getting-started/prompting).


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt