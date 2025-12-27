# Amp

> Connect Amp to up-to-date and token-efficient documentation.

How to set up Ref with [Amp CLI](https://ampcode.com/manual#mcp).

## Install

Run the `claude` install command. You can copy a version of the command with your API key pre-populated at [ref.tools/install](https://ref.tools/install)

```
amp mcp add Ref https://api.ref.tools/mcp --header "x-ref-api-key: <YOUR_API_KEY>"
```

You can find your API key at [ref.tools/keys](https://ref.tools/keys)

## Verify

Run the command:

```
claude mcp doctor
```

You should see:

```
➜  amp mcp doctor
User settings: <filepath>
Workspace settings: <filepath>

Ref (user settings): connected (2 tools: ref_search_documentation, ref_read_url)
```

## Get help

If you have any issues, please reach out to `help@ref.tools`. We're happy to help you get started!


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt