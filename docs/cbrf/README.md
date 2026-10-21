# CBRF Engineering Practice Assets

**Framework:** Chaos Bug Replication Framework (CBRF)  
**Status:** Engineering practice pack / specification assets  
**Verification state:** These documents define workflows and evidence requirements. They do not claim that a CBRF runtime or any candidate implementation has passed tests.

## Assets

- [Core Practices](PRACTICES.md): operational principles and minimum engineering habits.
- [Bug Specimen Template](BUG_SPECIMEN_TEMPLATE.md): capture a reproducible failure without losing provenance.
- [Experiment Record Template](EXPERIMENT_RECORD_TEMPLATE.md): track hypotheses, candidate changes, metrics, and outcomes.
- [Verification Gate](VERIFICATION_GATE.md): evidence required before a fix or release can advance.

## Lifecycle

Observe → Reproduce → Isolate → Generate candidate → Evaluate → Independently verify → Preserve evidence → Release or reject.

A failed run is still a result. Preserve its inputs, environment, logs, and failure signature.

## Status vocabulary

- **SOURCE-DEFINED:** source or specification exists; execution is not implied.
- **PROPOSED:** intended behavior or design is not yet established in implementation.
- **UNRESOLVED:** evidence is missing or conflicting.
- **IMPLEMENTATION-VERIFIED:** the specific implementation and stated scope passed recorded checks. Include commit, commands, environment, and results.

## Invariant

Names are not implementations. A successful demonstration is not a regression suite. A passing test suite verifies only the scope it actually exercises.
