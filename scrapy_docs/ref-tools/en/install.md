# Installation

> Install Ref in your preferred coding environment

## About Ref's MCP Server

Ref runs as a **streamable HTTP server** that implements the Model Context Protocol (MCP). This means you can connect to Ref's documentation search capabilities from any MCP-compatible client without installing additional packages locally.

### Authentication

Ref's API accepts authentication in three ways:

1. **As a header**: `x-ref-api-key: YOUR_API_KEY`
2. **As a query parameter**: `?apiKey=YOUR_API_KEY`
3. **OAuth**: Include neither header or query param and your MCP client will initiate OAuth sign-in.

The streamable HTTP endpoint is available at `https://api.ref.tools/mcp`.

You can find your API key at [ref.tools/keys](https://ref.tools/keys)

### OAuth and Teams

If you use OAuth in combination with [Teams](/usage/teams), you may need to specify the team you want Ref to access. Ref will do it's best to intelligently pick the correct team but you may want to manually force a specific team.

Here is the algorithm Ref uses to pick the user or team scope:

```
For base OAuth url https://api.ref.tools/mcp if the user belongs to:
  - no teams -> user scope
  - one team -> team scope for the users single team
  - multiple teams -> user scope, require specifying team

When specifying scope in OAuth url https://api.ref.tools/mcp?scope=(user|{team_id})
  - scope=user -> user scope
  - scope={team_id} -> team scope
```

Team ids have the format `t-<hash>`. You can find the team OAuth url at [ref.tools/install](https://ref.tools/install)

***

## Installation Guides

Choose your coding environment below to get started:

<CardGroup cols={2}>
  <Card title="Cursor" icon="arrow-pointer" href="/install/cursor">
    Connect Cursor to up-to-date and token-efficient documentation.
  </Card>

  <Card title="Claude Code" icon="asterisk" href="/install/claude-code">
    Connect Claude Code to up-to-date and token-efficient documentation.
  </Card>

  <Card title="Claude Desktop" icon="desktop" href="/install/claude-desktop">
    Connect Claude Desktop and Web to up-to-date and token-efficient documentation.
  </Card>

  <Card title="VS Code" icon="code" href="/install/vscode">
    Connect VS Code to up-to-date and token-efficient documentation.
  </Card>

  <Card title="Antigravity" icon="rocket" href="/install/antigravity">
    Connect Antigravity to up-to-date and token-efficient documentation.
  </Card>

  <Card title="Windsurf" icon="wind" href="/install/windsurf">
    Connect Windsurf to up-to-date and token-efficient documentation.
  </Card>

  <Card title="Cline" icon="robot" href="/install/cline">
    Connect Cline to up-to-date and token-efficient documentation.
  </Card>

  <Card title="Kiro" icon="ghost" href="/install/kiro">
    Connect Kiro to up-to-date and token-efficient documentation.
  </Card>

  <Card title="Roo Code" icon="paw" href="/install/roo-code">
    Connect Roo Code to up-to-date and token-efficient documentation.
  </Card>

  <Card title="Goose" icon="dove" href="/install/goose">
    Connect Goose to up-to-date and token-efficient documentation.
  </Card>

  <Card title="Amp" icon="arrow-up" href="/install/amp">
    Connect Amp to up-to-date and token-efficient documentation.
  </Card>

  <Card title="Augment" icon="wand-magic-sparkles" href="/install/augment">
    Connect Augment to up-to-date and token-efficient documentation.
  </Card>

  <Card title="ChatGPT" icon="comment" href="/install/chatgpt">
    Connect ChatGPT to up-to-date and token-efficient documentation.
  </Card>

  <Card title="Devin" icon="hexagon-nodes" href="/install/devin">
    Connect Devin to up-to-date and token-efficient documentation.
  </Card>

  <Card title="OpenCode" icon="code-branch" href="/install/opencode">
    Connect OpenCode to up-to-date and token-efficient documentation.
  </Card>

  <Card title="Codex CLI" icon="openai" href="/install/codex">
    Connect Codex CLI to up-to-date and token-efficient documentation.
  </Card>

  <Card title="Droid CLI" icon="user-robot" href="/install/droid-cli">
    Connect Factory Droid CLI to up-to-date and token-efficient documentation.
  </Card>

  <Card title="Gemini CLI" icon="gem" href="/install/gemini-cli">
    Connect Gemini CLI to up-to-date and token-efficient documentation.
  </Card>
</CardGroup>


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt