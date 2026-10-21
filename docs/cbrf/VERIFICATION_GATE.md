# CBRF Verification Gate

**Purpose:** prevent a candidate from being described as verified or production-ready without evidence.

## Gate 0: Provenance

- [ ] Repository, source commit, and changed paths are identified.
- [ ] Inputs, configuration, dependencies, and secrets handling are documented.
- [ ] Baseline and candidate artifacts are distinguishable.
- [ ] No claim relies only on a version label or narrative assertion.

**Failure action:** stop and resolve missing source/provenance.

## Gate 1: Static and structural checks

- [ ] Syntax / formatting / type checks run where applicable.
- [ ] Dependency and configuration checks run.
- [ ] Startup configuration fails safely when required configuration is absent.
- [ ] Public interfaces and error behavior match the specification.

**Evidence to record:** exact commands, environment, exit codes, logs.

## Gate 2: Unit and regression tests

- [ ] Known defects have regression tests.
- [ ] Normal, boundary, invalid, and failure cases are covered.
- [ ] Test results are repeatable or variance is documented.
- [ ] No existing regression is silently discarded.

**Failure action:** candidate is rejected or returned to implementation.

## Gate 3: Domain-specific validation

Choose checks appropriate to the component.

### Numerical / physics code

- [ ] Equations and sign conventions reviewed against a trusted derivation.
- [ ] Units and coordinate conventions documented.
- [ ] Invariants tested across multiple step sizes and initial conditions.
- [ ] Comparison against an independent reference performed where feasible.
- [ ] Tolerances justified rather than chosen solely to make tests pass.

### Evolution / learning code

- [ ] Fitness definition and output interpretation are explicit.
- [ ] Seeded runs execute through multiple generations.
- [ ] Candidate performance is evaluated on held-out cases.
- [ ] Convergence rate is measured across multiple seeds.
- [ ] Best candidate and its score refer to the same evaluated specimen.

### Persistence / integrity

- [ ] Round-trip and restart persistence tests pass.
- [ ] Tampered data and invalid signatures are rejected.
- [ ] Signed fields use canonical serialization.
- [ ] Missing records, malformed records, and concurrent access are tested where relevant.

### API / container

- [ ] Valid and invalid requests tested.
- [ ] Empty-store and dependency-failure responses tested.
- [ ] Health status reflects checks actually performed.
- [ ] Runtime secret injection and persistent-volume paths verified.
- [ ] Container build and actual startup tested.

## Gate 4: Integration

- [ ] Components work together through their real interfaces.
- [ ] Error paths and timeouts are exercised.
- [ ] Data contracts and serialization agree across boundaries.
- [ ] Resource limits and cleanup behavior are checked.

## Gate 5: Release evidence

- [ ] Build passed on a named environment.
- [ ] Automated tests passed; report and commit are recorded.
- [ ] Required domain-specific gates passed.
- [ ] Known gaps and excluded scopes are listed.
- [ ] Artifact identity and release revision are recorded.
- [ ] Deployment verification is recorded separately from CI verification.

## Allowed final statuses

- **SOURCE-DEFINED:** source exists; execution not established.
- **PROPOSED:** design intent only or implementation not established.
- **UNRESOLVED:** evidence is missing, incomplete, or contradictory.
- **IMPLEMENTATION-VERIFIED:** specific stated scope passed recorded checks.

Do not use “production-ready” as a synonym for “tests pass.” Production readiness additionally requires operational, security, recovery, monitoring, and deployment requirements appropriate to the intended environment.

## Evidence ledger

| Check | Commit | Environment | Command / method | Result | Evidence reference |
|---|---|---|---|---|---|
| | | | | Not run | |
