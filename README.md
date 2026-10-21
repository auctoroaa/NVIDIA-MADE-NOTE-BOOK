# CODEX VERDANT v1.1

**AXIOM-000:** Do not prematurely destroy potentially useful information.

Experimental modular Rust workspace for a local-first knowledge and model experimentation system.

## Engineering corpus

- [Master Engineering Corpus](docs/MASTER_ENGINEERING_CORPUS.md): engine map, reusable component registry, provenance/status definitions, and verification ledger.
- [Knowledge Harvest: Persistent Intelligence Infrastructure](docs/KNOWLEDGE_HARVEST_PII.md): concepts, formulas, cognition/governance frameworks, hardware-memory patterns, reusable specifications, open questions, and verification queue.
- [Engineering Execution Bottleneck and Leverage Plan](docs/EXECUTION_LEVERAGE_PLAN.md): proposed end-to-end task loop, verification contract, build order, and metrics for reducing coordination overhead.

## Workspace

- `codex-core`: shared types and identity primitives
- `codex-ingestion`: ingestion interfaces
- `codex-embedding`: embedding interfaces
- `codex-storage`: persistence interfaces
- `codex-evolution`: evaluation and iteration interfaces
- `codex-governance`: policy and invariant checks
- `codex-gpu`: compute backend capability reporting
- `codex-vector`: vector-search interfaces
- `codex-api`: API-facing types
- `codex-app`: CLI entry point

## Build

Install stable Rust, then run:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo run -p codex-app
```

## Honest status

This is a **scaffold**, not a finished AI platform. GPU kernels, vector indexing, document parsers, model inference, persistent storage backends, Tauri desktop shell, and Svelte UI are not implemented yet. Crate names define intended boundaries, not completed capabilities.

Never commit API keys or private datasets.
