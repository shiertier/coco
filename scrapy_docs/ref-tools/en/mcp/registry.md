# MCP Registry

> Include Ref in your MCP registry or subregistry

## What is the MCP Registry?

The **MCP Registry** ([registry.modelcontextprotocol.io](https://registry.modelcontextprotocol.io)) is the official directory of MCP servers - like an app store for AI coding agents. It's community-owned and serves as the canonical source for publicly-available MCP servers.

## Building Subregistries

Many organizations build **subregistries** that add value through curation, enhanced metadata, or focus on specific use cases. These subregistries typically:

* Pull servers from the official registry via API
* Add custom metadata, ratings, or curation
* Serve specific communities or enterprise needs

## Including Ref in Your Registry

**Ref** is available in the official MCP registry and provides essential documentation search capabilities for AI coding agents.

```title: Ref MCP registry name theme={null}
tools.ref/ref-tools-mcp
```

### Get Ref's Registry Data

```bash  theme={null}
curl "https://registry.modelcontextprotocol.io/v0/servers?search=tools.ref/ref-tools-mcp"
```

This returns Ref's complete server metadata that you can import into your own registry.

## Resources

* **Official Registry**: [registry.modelcontextprotocol.io](https://registry.modelcontextprotocol.io)
* **Registry API**: [REST API Documentation](https://github.com/modelcontextprotocol/registry/blob/main/docs/guides/consuming/use-rest-api.md)
* **Build Your Own**: [Registry Implementation Guide](https://github.com/modelcontextprotocol/registry/blob/main/docs/README.md)

<Columns cols={2}>
  <Card icon="search" title="Browse Official Registry" href="https://registry.modelcontextprotocol.io">
    Explore all available MCP servers.
  </Card>

  <Card icon="code" title="Ref on GitHub" href="https://github.com/ref-tools/ref-tools-mcp">
    View Ref's source code and documentation.
  </Card>
</Columns>


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt