# AXIOM NOTEBOOK

**AXIOM-000:** Do not prematurely destroy potentially useful information.

Axiom Notebook is a local-first knowledge intelligence workspace for capturing, understanding, connecting, evaluating, preserving, evolving, and rediscovering information. It is intended to become a durable engineering and research environment, not merely a notes app or chat interface.

> **Status:** Experimental. The current repository contains a modular Rust scaffold and knowledge-engineering documents. The complete product described here is not yet implemented. See [the specification](docs/AXIOM_NOTEBOOK_SPEC.md) and record implementation evidence before claiming a capability is complete.

## Product contract

Axiom Notebook must:

1. **Ingest** source material without silently discarding the original.
2. **Understand** content through typed records, metadata, extraction, and optional model-assisted analysis.
3. **Connect** concepts, claims, sources, artifacts, decisions, and experiments as traceable relationships.
4. **Evaluate** claims and artifacts with explicit criteria, evidence, uncertainty, and reproducible checks.
5. **Preserve** originals, provenance, revisions, and superseded knowledge. Destructive actions require an explicit, auditable decision.
6. **Evolve** derived interpretations and indexes without rewriting source evidence.
7. **Rediscover** useful information through search, filters, links, and eventually semantic retrieval.

## Engineering corpus

- [Axiom Notebook Specification](docs/AXIOM_NOTEBOOK_SPEC.md): product contract, system boundaries, data principles, module map, acceptance gates, and unresolved decisions.
- [Master Engineering Corpus](docs/MASTER_ENGINEERING_CORPUS.md): engine map, reusable component registry, provenance/status definitions, and verification ledger.
- [Knowledge Harvest: Persistent Intelligence Infrastructure](docs/KNOWLEDGE_HARVEST_PII.md): concepts, formulas, cognition/governance frameworks, hardware-memory patterns, reusable specifications, open questions, and verification queue.
- [Engineering Execution Bottleneck and Leverage Plan](docs/EXECUTION_LEVERAGE_PLAN.md): proposed end-to-end task loop, verification contract, build order, and coordination metrics.

## Current Rust workspace

The existing crates are retained as scaffold boundaries while the product architecture is clarified:

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

These names are implementation scaffolding, not proof that the corresponding engines are production-ready. Crate renaming and deeper implementation should follow a reviewed migration plan so existing history and links are not needlessly broken.

## Build and verify

Install the Rust toolchain pinned by `rust-toolchain.toml`, then run:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo run -p codex-app
```

A successful compile only verifies the current Rust scaffold. It does not establish that ingestion, durable storage, semantic search, model inference, or the full Axiom Notebook product works end to end.

## Evidence and status rules

Every capability must be labelled as one of:

- **Source-defined:** specified in a document or contract.
- **Proposed:** a design awaiting implementation or approval.
- **Implemented, unverified:** code exists but required tests or runtime evidence are missing.
- **Verified:** acceptance tests passed with recorded evidence and environment.
- **Blocked:** a known dependency or defect prevents acceptance.
- **Deprecated:** retained for history but not the current direction.

Do not promote a capability to Verified based on filenames, comments, stubs, or a successful compile alone. Keep known limitations, test results, and unresolved decisions visible.

## Security

Never commit API keys, credentials, private datasets, or sensitive source material. Use environment variables or a local ignored secrets file, and keep `.env.example` free of real secrets.
