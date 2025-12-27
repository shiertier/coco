# Privacy & Security

> Providing a secure search environment is important to us. This page outlines how we approach security for Ref.

Please submit potential vulnerabilities or any security-related questions to [security@ref.tools](mailto:security@ref.tools).

*While many teams and individuals already trusting Ref, please note that we are still in the journey of growing our product and improving our security posture. If you're working in a highly sensitive environment, you should be careful when using Ref (or any other AI tool). We hope this page gives insight into our progress and helps you make a proper risk assessment.*

## Overview

Ref is built with security and privacy as core principles. This page outlines our security architecture, data handling practices, and compliance efforts. Key areas include:

* **MCP Implementation**: Local and remote server protocols with API key authentication
* **Data Protection**: End-to-end encryption, isolated multi-tenant architecture, and comprehensive audit logging
* **Compliance**: SOC2 certification in process with Vanta (see our [Trust Center](https://trust.ref.tools))
* **Monitoring**: Real-time health checks and public status updates

## MCP

### Protocols

Ref provides an [open-source](https://github.com/ref-tools/ref-tools-mcp) `stdio` server that can be run locally and a `streamable-http` that is connected to remotely.

### Authentication

Ref allows use of API keys or OAuth for authentication. Ref's implementation of MCP OAuth is in beta as of Nov 15, 2025. Ref also supports SSO for organizations via [Scalekit](https://www.scalekit.com/).

The easiest way to rollout Ref to an organization is a combination of SSO and MCP OAuth. If you are interested in MCP OAuth and/or SSO, please reach out to `help@ref.tools`

## Data Handling

### Encryption

* **In Transit**: All data is encrypted during transit. Ref uses MCP streamable-http transport.
* **At Rest**: Documents and search indices are encrypted at rest in our database.
* **Customer Managed Keys**: Turbopuffer supports customer managed encryption keys (available upon request).

### Data Isolation

We take data isolation very seriously in our multi-tenant environment:

* Each team and user has their own isolated namespace in Turbopuffer.
* Indexing jobs run in single, transient, isolated containers. Your documents and credentials are never present at the same time as another team's data.
* All application data reads go through Firestore rules that enforce user access at the database level.

### Audit Logging

* Complete activity logging for users and teams available at [ref.tools/activity](https://ref.tools/activity)
* Logs include user identity, tool calls, and arguments

### Incident Response

* Internal monitoring via Sentry and Google Cloud alerting tools
* Status updates published at [ref.tools/status](https://ref.tools/status) during incidents

## Compliance

### Certifications

* **SOC2**: Currently in progress with [Vanta](https://www.vanta.com)
* For more information, see out [Trust Center](https://trust.ref.tools)

### Subprocessors & Data Access

Our security model includes the following subprocessors with specific data access patterns:

* **[Turbopuffer](https://turbopuffer.com)**: `Stores docs` Turbopuffer is the primary search store used by Ref. It is also used by Cursor and Notion. Stores documents, descriptions, and vector embeddings with encryption at rest and isolated namespaces.
* **[Firebase](https://firebase.google.com)**: `Sees and stores docs` Temporarily processes search results through Functions and temporarily caches results in Firestore with database-level access controls. All data is encrypted at rest.
* **[Google Cloud Run](https://cloud.google.com/run)**: `Sees docs` Processes indexing jobs in isolated, transient containers. User docs will be loaded
* **[Google Vertex AI](https://ai.google.dev/gemini-api/docs)**: `Sees docs` Generates document description with zero-data retention policy.
* **[VoyageAI](https://www.voyageai.com/)**: `Sees docs` Creates vector embeddings of docs with zero-data retention policy
* **[OpenAI](https://openai.com)**: `Sees docs` Powers research agent functionality with zero-data retention policy. User data will be included in prompts sent to OpenAI.
* **[Anthropic](https://www.anthropic.com)**: `Sees no docs` Powers evals on public documentation sets with zero-data retention policy.
* **[Stripe](https://stripe.com)**: `Sees no docs` Processes payment information with PCI-compliant security standards.
* **[Postmark](https://postmarkapp.com)**: `Sees no docs` Delivers transactional emails with user contact information.
* **[Mailchimp](https://mailchimp.com)**: `Sees no docs` Manages marketing communications and newsletter subscriptions.
* **[Mixpanel](https://mixpanel.com)**: `Sees no docs` Analyzes product usage analytics.
* **[Sentry](https://sentry.io)**: `Sees no docs` Monitors errors and performance with anonymized telemetry data.
* **[Google Workspace](https://workspace.google.com)**: `Sees no docs` Used for communication and coordination.
* **[Slack](https://slack.com)**: `Sees no docs` Used to communicate with partners.
* **[GitHub](https://github.com)**: `Sees no docs` Used for version control.
* **[Scalekit](https://scalekit.com)**: `Sees no docs` Used for SSO (Firebase is the IdP) and MCP OAuth.

### Monitoring & Health Checks

* Health check endpoint: `api.ref.tools/ping`
* Internal monitoring and alerting infrastructure
* Status page: [ref.tools/status](https://ref.tools/status)


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt