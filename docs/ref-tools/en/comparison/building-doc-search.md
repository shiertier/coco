# Building the Best Documentation Search

> Principles for coding-agent context.

## Objectives

* Deliver precise answers with minimal tokens.
* Keep agents in flow without manual hunting.
* Blend public and private sources safely.

## Search Loop

1. Use session history to skip duplicate hits.
2. Return only the top \~5k relevant tokens per read.
3. Let agents adjust prompts while paging results.

## Token Discipline

* Favor HTTP MCP mode for fast responses.
* Trim payloads before handing context to the agent.
* Track credits per endpoint to spot waste.

## Source Coverage

* Index GitHub repos for code and docs.
* Add PDFs and Markdown libraries for gaps.
* Combine web search when private data misses.

## Agent Experience

* Verify installs with a sample MCP prompt early.
* Surface usage cards and alerts in the dashboard.
* Keep role-based guardrails tight for teams.

## Continuous Improvement

* Monitor credit logs after each feature launch.
* Re-run indexing when repos cross major releases.
* Capture wins and issues in a monthly changelog.


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt