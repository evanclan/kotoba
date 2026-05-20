# AI integration

Kotoba's relationship with AI is a deliberate design choice, not an afterthought. This document explains how AI fits in today and where it's heading.

## Core stance

> **Kotoba is the substrate. AI is one tool that plugs into it.**

We do not build "ChatGPT for Japanese." We build the local-first, plain-text, owned learning state that *every* AI tool — present and future — should plug into.

## Three layers of AI use

### 1. Optional, in-product AI (BYO model)

Some commands can use a language model when one is configured:

| Command                              | What AI does                                                                |
|--------------------------------------|------------------------------------------------------------------------------|
| `kotoba add 留学 --enrich`           | Generate two example sentences calibrated to your level.                    |
| `kotoba review --talk-back`          | After review, narrate one of the cards in a 2-line story for context.       |
| `kotoba reading --topic "..."`       | Generate comprehensible input using only vocabulary you know + N new words. |
| `kotoba conversation`                | Free-chat practice with weak-grammar emphasis (planned).                    |

**Bring your own model.** Configure in `~/.config/kotoba/config.toml`:

```toml
[ai]
provider = "ollama"           # or "openai", "anthropic", "openrouter", "none"
model    = "qwen2.5:14b"
endpoint = "http://localhost:11434"
api_key_env = "OLLAMA_API_KEY"   # optional
```

Default: `provider = "none"`. AI features fail closed with a clear error message.

### 2. MCP server (`kotoba-mcp`)

Kotoba ships an [MCP](https://modelcontextprotocol.io) server so AI agents (Claude Desktop, Cursor, etc.) can read your learning state and write back to it — with your consent.

**Resources** (read-only context for the agent):

- `kotoba://learner-state` — JLPT level estimate, target retention, totals
- `kotoba://due-cards?limit=20` — what's due now
- `kotoba://recent-mistakes?days=7` — last week's lapses
- `kotoba://decks` — your deck list
- `kotoba://decks/<slug>/cards` — cards in a deck

**Tools** (agent can take action, with consent):

- `lookup(term)` — dictionary lookup
- `add_card(deck, front, back?, context?)` — add a card
- `record_review(card_id, grade)` — log a review the user did externally
- `generate_example_sentence(term, target_level)` — calls the configured LLM
- `suggest_next_card(criteria)` — agent-orchestrated next-card selection

The agent never sees personal data outside the resources you grant. Consent lives in the MCP client (e.g. Claude Desktop's tool-permissions UI), not in Kotoba.

### 3. Programmable substrate

The daemon's HTTP / JSON-RPC API ([plugin-api.md](plugin-api.md)) lets *any* tool — your own scripts, third-party agents, future products that don't exist yet — read and write Kotoba state. This is the most important interface in the long run.

## Privacy & consent principles

1. **Default off.** No AI feature is enabled until the user configures a model.
2. **Default local.** When suggesting a model, we suggest a local one (Ollama / llama.cpp) first.
3. **No silent uploads.** No card data, review log, or user input is sent to a remote model unless the user explicitly configured a remote provider.
4. **Explicit prompts.** Every prompt sent to a remote model is loggable via `RUST_LOG=kotoba_ai=debug`.
5. **Per-feature toggles.** A user can enable enrichment but disable conversation; enable lookups but disable review-time generation.
6. **No data brokering.** We never share or sell user data, and we never will. This is enshrined in [governance.md](governance.md).

## What we will not build

- **A captive, hosted AI tutor.** That's a different product, owned by someone else.
- **Vendor lock-in.** Any feature that requires a specific provider must have a fallback to "none" or to a local model.
- **AI features that hide their inputs.** Every prompt and every output is inspectable.

## Future directions (research)

- **Federated weight tuning for FSRS.** Across consenting users, find better default scheduler weights without anyone's data leaving their machine.
- **Embedding-based card similarity.** Use a local embedding model (e.g. `multilingual-e5-small`) to detect duplicates and cluster decks.
- **Real-time error feedback.** When the user types Japanese, generate corrections that flow back into the SRS as targeted weak points.
- **Cross-language transfer learning.** When you start a new language, seed your initial deck from cognates and shared vocabulary you already know.

These are research directions, not commitments. They will live behind feature flags and ship with documentation explaining trade-offs.
