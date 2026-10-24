# Axiom Notebook: Product and Engineering Specification

**Document status:** Source-defined target specification; implementation status remains experimental until verified.
**Core axiom:** AXIOM-000, “Do not prematurely destroy potentially useful information.”

## 1. Mission

Build a local-first knowledge intelligence system that can ingest, understand, connect, evaluate, preserve, evolve, and rediscover information across research, engineering, code, experiments, and decisions.

Axiom Notebook is not merely a notes editor, chat wrapper, vector database, or folder of documents. It is a provenance-preserving knowledge workspace whose derived intelligence remains traceable to source evidence.

## 2. Non-negotiable invariants

1. **Preserve source evidence.** Original inputs are immutable by default. Normalization, extraction, chunking, summaries, embeddings, and interpretations are derived artifacts linked to their source.
2. **No silent destruction.** Deletion, compaction, replacement, and retention changes must be explicit, logged, and recoverable where technically possible.
3. **Provenance travels with knowledge.** Each claim or artifact records its source, creation time, transformation, version, and verification state where applicable.
4. **Separate evidence from interpretation.** Store source observations, model-generated interpretations, human judgments, and system-generated metadata as distinct kinds of records.
5. **Make uncertainty visible.** Conflicts, missing evidence, confidence estimates, assumptions, and unresolved questions must not be disguised as facts.
6. **Reproducible evaluation.** Tests and evaluations record inputs, method/version, environment where relevant, outputs, and pass/fail criteria.
7. **Local-first operation.** Core capture and access must not require a remote AI service. Network/model features are optional adapters, not hidden prerequisites.
8. **Fail safely.** Failed imports, migrations, indexing jobs, or model calls must not corrupt or silently erase the source record.
9. **Explicit capability status.** Documentation must distinguish source-defined, proposed, implemented-unverified, verified, blocked, and deprecated work.
10. **Security by default.** Secrets stay out of version control; sensitive data access and destructive operations are constrained and auditable.

## 3. Core lifecycle

`Source → Ingest → Preserve → Parse/Extract → Typed Knowledge → Link/Index → Evaluate → Rediscover → Human Review → Versioned Derivation`

Every derived object must point back to its parent source(s). Reprocessing creates a new version or derivation rather than silently overwriting prior results.

## 4. Logical engines

These are target boundaries, not claims that each engine is already implemented.

| Engine | Responsibility | Minimum acceptance evidence |
|---|---|---|
| Identity and Core Types | Stable IDs, timestamps, typed records, schema/version identifiers | Deterministic ID and serialization tests |
| Ingestion | Accept supported source formats and record import outcomes | Fixtures for valid, malformed, duplicate, and interrupted imports |
| Preservation Archive | Keep original bytes/text and content hashes; record revisions and tombstones | Round-trip retrieval, hash verification, recovery tests |
| Extraction | Produce structured metadata, chunks, and references | Golden fixtures and source-offset/provenance checks |
| Knowledge Graph | Represent concepts, claims, sources, artifacts, and typed relations | Referential integrity and cycle/duplicate policy tests |
| Search and Retrieval | Keyword search first; optional vector/semantic retrieval as adapters | Recall/precision fixtures, ranking checks, index rebuild test |
| Evaluation | Run explicit validators and benchmarks against versioned inputs | Reproducible reports and failure-case coverage |
| Evolution and Derivation | Create improved interpretations/indexes without destroying prior evidence | Version lineage and rollback/replay tests |
| Governance | Enforce invariants, permissions, review, and destructive-action policy | Denial-path tests and append-only audit evidence |
| Interface/API | Expose capture, browse, search, inspect, compare, and review operations | End-to-end user-flow tests |
| Observability | Record errors, job state, latency, resource use, and verification results | Structured logs and bounded retry behavior |

## 5. Minimum data model

The exact schema remains a design decision, but the system must be able to represent at least:

- **SourceRecord:** stable ID, source type, original location if applicable, content hash, capture time, and preservation state.
- **Artifact:** stable ID, artifact type, payload or payload reference, schema version, and lifecycle state.
- **Derivation:** output artifact, input artifact IDs, operation/model/version, parameters, timestamp, and result status.
- **Claim:** statement, claim type, evidence links, counter-evidence links, confidence/uncertainty, and review state.
- **Relation:** source entity, target entity, relation type, provenance, and validity state.
- **EvaluationRun:** evaluator version, input IDs, criteria, environment, measurements, outcome, and evidence.
- **DecisionRecord:** decision, rationale, alternatives, author/source, timestamp, and supersession links.
- **AuditEvent:** actor or subsystem, operation, target, timestamp, outcome, and relevant before/after references without leaking secrets.

IDs must remain stable across display-name changes. Derived data must be rebuildable from preserved sources wherever practical.

## 6. Storage strategy

**Target:** local durable storage with clear boundaries between metadata, original payloads, derived artifacts, and rebuildable indexes.

Proposed layering:

1. Metadata and relationships in a transactional local database.
2. Original and large payloads in a content-addressed or otherwise integrity-checkable archive.
3. Keyword indexes and vector indexes treated as derived/rebuildable structures.
4. Optional remote/model adapters isolated behind interfaces and disabled gracefully when unavailable.
5. Explicit backup, restore, migration, and integrity-check workflows.

**Unresolved:** exact database, content-addressing scheme, encryption-at-rest policy, vector engine, and backup format. Do not claim these decisions are settled until documented and tested.

## 7. Repository implementation direction

The repository currently contains a Rust workspace with `codex-*` crate names. Retain those modules as the current scaffold until a deliberate migration is reviewed. Candidate responsibility mapping:

- `codex-core` → identity, shared types, schema versions
- `codex-ingestion` → source adapters and import jobs
- `codex-storage` → metadata and preservation interfaces
- `codex-embedding` → optional embedding adapters
- `codex-vector` → optional semantic index interfaces
- `codex-evolution` → versioned derivation and evaluation
- `codex-governance` → invariants and audit policy
- `codex-gpu` → optional compute capability layer
- `codex-api` → application-facing contracts
- `codex-app` → initial command-line interface

Do not rename every crate or rewrite the workspace as a cosmetic first step. First establish contracts and tests, then migrate names and interfaces with compatibility checks.

## 8. Build sequence

### Phase 0: Baseline and inventory
- Run formatting, compilation, tests, and repository checks.
- Record actual results and existing failures.
- Inventory each crate as implemented, stub, proposed, or blocked.
- Preserve existing documents and archives.

### Phase 1: Core contracts
- Define IDs, timestamps, content hashes, artifact/source types, provenance, and status enums.
- Add schema versioning and serialization tests.
- Define errors and deterministic behavior.

### Phase 2: Preservation and ingestion
- Implement one small supported text/Markdown import path end to end.
- Preserve original input and verify its hash.
- Make repeated imports idempotent or explicitly versioned.
- Test malformed input and interrupted writes.

### Phase 3: Durable records and relationships
- Add transactional persistence for source records, artifacts, derivations, and relations.
- Add integrity checks, migrations, and backup/restore tests.
- Ensure derived records never replace original evidence.

### Phase 4: Retrieval and inspection
- Implement reliable keyword search and source-linked result inspection.
- Add optional embeddings/vector retrieval only behind a tested adapter.
- Test index rebuild from preserved records.

### Phase 5: Evaluation, governance, and evolution
- Add reproducible evaluation runs, audit events, and invariant checks.
- Add versioned derivations and safe rollback.
- Test conflict handling and destructive-operation safeguards.

### Phase 6: Usable application workflows
- Deliver capture, browse, inspect provenance, search, compare versions, export, backup, and restore.
- Add model-assisted workflows only with clear provenance and human review where appropriate.
- Run end-to-end tests on a clean environment.

## 9. Definition of done

A capability is **Verified** only when all applicable conditions are met:

- Its contract and supported inputs are documented.
- Implementation exists beyond a stub or placeholder.
- Unit tests cover normal and boundary behavior.
- Failure paths are tested.
- Persistence and restart behavior are tested where applicable.
- Provenance and preservation invariants are tested.
- The required build/test command passes in a recorded environment.
- The evidence, commit, and known limitations are recorded.

A passing compile alone is not product acceptance. A model response alone is not evidence of factual correctness. An index existing is not evidence that retrieval is useful.

## 10. Open decisions

- Exact supported import formats and size limits.
- Metadata database and payload archive implementation.
- Encryption-at-rest and local key management.
- Conflict-resolution policy for contradictory claims.
- Semantic search engine and embedding model policy.
- Optional synchronization and multi-device conflict handling.
- Desktop/mobile interface strategy.
- Data export format and backup retention.
- Threat model and permission granularity.

Resolve these through recorded decisions and tests rather than silently turning assumptions into requirements.

## 11. Current verified status

At the time this specification was added, repository metadata and the existing README were inspected. The repository contains a modular Rust scaffold and documentation. No runtime build or test was executed as part of this documentation change. Therefore this document defines the target; it does not certify the current implementation.
