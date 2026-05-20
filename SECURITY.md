# Security policy

## Reporting a vulnerability

If you've found a security issue in Kotoba, please **do not open a public issue**. Instead, email **security@kotoba.dev** (placeholder — update once maintainers register a domain) with:

- A description of the issue
- Steps to reproduce
- The version (or commit) you tested against
- Any suggested mitigations

You can expect:

- An acknowledgement within **3 business days**
- A first assessment within **7 business days**
- A patch or mitigation timeline within **14 business days**

We'll coordinate disclosure with you. We do not currently run a paid bug-bounty program but we will publicly thank you in the release notes (with your permission).

## Scope

In scope:

- The `kotoba` binary and all crates in this repository
- The optional `kotoba-daemon` HTTP API
- The `kotoba-mcp` MCP server
- Default sync backends shipped in this repo
- Reference plugins shipped in this repo

Out of scope (unless explicitly maintained here):

- Third-party plugins or integrations
- Deployments of Kotoba you don't control
- Underlying language-model providers

## Threat model summary

Kotoba is a **local-first** tool. The default deployment has no network surface and no remote authentication. Sensitive surfaces are:

1. **The local daemon's HTTP API** — binds to `127.0.0.1` by default, but exposes read/write to deck data. Authentication via a per-install token is required for any non-localhost binding.
2. **The MCP server** — exposes learner-state to AI agents via the user's MCP client. Permissions and consent live in the MCP client, not in Kotoba; we document the consent model in [`docs/ai-integration.md`](docs/ai-integration.md).
3. **Sync backends** — credentials for git remotes, S3, WebDAV, etc. are stored in the OS keychain when available, and never logged.
4. **Deck files** — plain text in the user's home directory; we treat them as user-readable but rely on filesystem permissions.

If you're operating Kotoba in a multi-user or networked context (e.g. a classroom server), please read [`docs/security-deployment.md`](docs/security-deployment.md) (TODO) before deployment.

## Supported versions

Until v1.0, only the **latest released minor version** receives security fixes. After v1.0, we'll publish a formal support policy.

| Version | Supported          |
|---------|--------------------|
| `0.x` (latest) | ✅ |
| Older `0.x`    | ❌ |
