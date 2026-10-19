---
name: verification-engine
description: Audit claims about code, builds, tests, deployments, and capabilities against direct evidence; separate verified facts from assumptions and proposals.
---

# Verification Engine

## Goal
Prevent unsupported success claims. Every material status statement must be tied to inspectable evidence.

## Evidence levels
- **VERIFIED**: directly confirmed by a tool result, source file, completed command, test log, or deployment response.
- **SOURCE-DEFINED**: stated in repository documentation or source, but not independently exercised.
- **PROPOSED**: recommended design or planned work that has not been implemented.
- **UNRESOLVED**: evidence is missing, contradictory, or incomplete.
- **FAILED**: an attempted operation returned an error or a check failed.

Do not upgrade evidence levels without new evidence.

## Checks
1. Identify the precise claim being assessed.
2. Prefer primary evidence: source, manifest, workflow definition, command output, test results, deployment status, or API response.
3. Distinguish compilation from tests; distinguish tests being present from tests actually executing; distinguish mocked tests from real integration tests.
4. For CI, record repository, commit SHA, workflow/run ID, job result, and relevant logs when available.
5. For deployment, verify the resulting URL or provider deployment state. A successful commit is not a deployment.
6. For product/provider capabilities, distinguish official documentation from account-specific access, quota, and successful use.
7. If evidence is insufficient, say exactly what remains unverified and what single check would resolve it.

## Output format
- **Claim**
- **Status**: one evidence level above
- **Evidence**
- **Gap / next check**

Keep conclusions proportional to the evidence. Never fabricate logs, test counts, benchmark numbers, hardware access, persistence, or provider quotas.
