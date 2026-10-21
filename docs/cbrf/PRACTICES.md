# CBRF Core Engineering Practices

**Status:** Source-defined practice standard  
**Scope:** Failure reproduction, candidate evolution, verification, and evidence retention.

## 1. Controlled experimentation

Before a run, record the question being tested, hypothesis, inputs, environment, controlled variables, expected result, and failure conditions. Change one major factor at a time when practical.

## 2. Deterministic reproduction

Record source revision, dependency versions, configuration, random seeds, input artifacts, and invocation. If exact determinism is impossible, state the expected variance and define a reproducible statistical protocol.

## 3. Failure preservation

Every confirmed defect becomes a specimen and a regression test where feasible. Preserve the first failing case, logs, traces, and environment. Never overwrite the only copy of a useful failure.

## 4. Hypothesis-driven candidate generation

Define the objective and hard constraints before comparing candidates. Keep the baseline, candidate lineage, evaluator version, and random seed. A fitness score is evidence about the chosen metric, not proof of general correctness.

## 5. Independent verification

Do not validate a candidate only with the same path that generated or optimized it. Use held-out cases, independent references, invariants, property-based tests, and regression tests where appropriate.

## 6. Numerical integrity

For simulations, document units, coordinate conventions, equations, integrator, step size, tolerances, and expected invariants. Test multiple step sizes and compare against an independent trusted reference when available. Energy drift alone does not prove that the equations are correct.

## 7. Data and artifact integrity

Use canonical serialization for signed data. Specify exactly which fields are covered by a signature, normalize numeric representations, and test tampering and invalid signatures. Keep secrets out of source code, logs, and container images.

## 8. Persistence and recovery

Test fresh databases, existing databases, restart persistence, schema migration, interrupted writes, corruption, and concurrent access where supported. Document durability assumptions and failure behavior.

## 9. Observable behavior

Expose truthful health states. Separate process liveness, dependency readiness, numerical checks, data integrity, and model availability. Never report a global “verified” state when only one subcheck has run.

## 10. Reproducible delivery

Pin dependencies where appropriate. Record the source commit, toolchain, build command, test command, exit status, and relevant logs. Distinguish local checks, CI checks, and deployed-runtime checks.

## 11. Release discipline

A candidate advances only when the predefined gates pass. Failed gates block release and create follow-up evidence. A version label such as “PROD” is not proof of production readiness.

## Required record for any verification claim

- Exact repository and commit.
- Exact command and environment.
- Expected and observed result.
- Pass/fail status and logs or machine-readable report.
- Scope and known exclusions.
- Reviewer or independent check, when applicable.
