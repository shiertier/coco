# Claude Desktop and Web

> Connect Claude Desktop and Web to up-to-date and token-efficient documentation.

You can find your API key at [ref.tools/keys](https://ref.tools/keys)

## Install Steps

1. Open `Settings`

2. Click `Connectors`

3. Click `Add custom connector`

4. Fill in Name as `Ref`

5. Fill in Remote MCP Server URL `https://api.ref.tools/mcp?apiKey=<YOUR_API_KEY>`

## Verify

Start a new chat and issue the following prompt.

```
what is ref tools mcp, search the docs with ref
```

This should results in a `ref_search_documentation` tool call.

## Get help

If you have any issues, please reach out to `help@ref.tools`. We're happy to help you get started!


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt