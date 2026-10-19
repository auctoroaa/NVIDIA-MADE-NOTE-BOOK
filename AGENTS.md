# Agent Instructions

This repository uses reusable skills in `.agents/skills/`.

## Required workflow

1. Before command execution, read `.agents/skills/terminal-runner/SKILL.md`. Use only an execution environment confirmed available in the current session.
2. Before repository edits, read `.agents/skills/repo-engineer/SKILL.md`. Inspect existing source and preserve the Rust workspace and crate names.
3. Before reporting status, read `.agents/skills/verification-engine/SKILL.md`. Label claims as verified, source-defined, proposed, unresolved, or failed.
4. For Rust changes, use the existing GitHub Actions workflow at `.github/workflows/rust.yml` as the remote build/test route. It runs formatting, workspace compilation, tests, and a CLI smoke test on pushes to `main` and pull requests.
5. Do not claim that tests passed until the relevant run has completed successfully and its result has been inspected. A successful build does not mean scaffolded features are implemented.

## Project truth

CODEX VERDANT is currently an experimental modular Rust workspace scaffold. Keep its existing architecture and report unimplemented capabilities accurately.
