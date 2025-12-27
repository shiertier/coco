# What is MCP?

> Understanding the Model Context Protocol and why it matters for AI coding agents.

## The Model Context Protocol

The **Model Context Protocol (MCP)** is an open standard that enables AI coding agents to connect with external tools, data sources, and services. Think of it as a universal translator that lets your AI assistant reach beyond its training data to access the specific information it needs, when it needs it.

Introduced by Anthropic in November 2024, MCP provides a standardized way for AI models to interact with the real world—reading files, querying databases, searching documentation, and executing tools—all through a common protocol.

## Why MCP Matters for Coding Agents

### The Documentation Problem

Modern development involves countless libraries, frameworks, and APIs. No AI model can memorize every API endpoint, every configuration option, or every recent update to every library. Traditional approaches either:

1. **Cram everything into the prompt** - This wastes tokens, blows up context windows, and still misses the exact information needed
2. **Hope the training data is current** - Models inevitably work with outdated information and hallucinate deprecated APIs

### The MCP Solution

MCP solves this by giving coding agents **just-in-time access** to accurate, up-to-date information:

* **Search documentation on-demand** - Find the exact code snippet or API reference needed for the current task
* **Access private resources** - Read your company's internal docs, not just public information
* **Stay current** - Always get the latest documentation, not what was known at training time
* **Preserve context** - Use tokens for reasoning instead of documentation

**The result?** Coding agents that make fewer mistakes, write better code, and never need to guess about API syntax or configuration options.

## MCP in Action with Ref

Ref is an MCP server specifically built for documentation search. It gives coding agents the ability to:

* **Search public documentation** - React, Next.js, Python, Go, and thousands of other libraries
* **Search private documentation** - Your team's internal docs, private repos, and custom APIs
* **Get precise answers** - Find the exact code snippet or configuration example needed
* **Stay up-to-date** - Always access the latest documentation versions

Instead of guessing about how to use a library, coding agents can simply ask Ref and get the authoritative answer.

## Learn More

Want to dive deeper into MCP? Check out these resources:

* [Official MCP Documentation](https://modelcontextprotocol.io) - The complete specification and guides
* [MCP Specification](https://modelcontextprotocol.io/specification/2024-11-05/index) - Technical details of the protocol
* [Visual Guide to MCP](https://block.github.io/goose/blog/2025/04/10/visual-guide-mcp/) - A beginner-friendly explanation with diagrams

<Columns cols={2}>
  <Card icon="rocket" title="Try Ref" href="/getting-started/quick-start">
    Install the Ref MCP server and see it in action.
  </Card>

  <Card icon="book" title="MCP Server Details" href="/mcp/tools">
    Explore the tools and capabilities Ref provides.
  </Card>
</Columns>


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt