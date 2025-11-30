CI workflows and tooling added

This repository had the following CI and automation configuration added:

- `.github/workflows/ci.yml` (detector-only workflow)
- `.github/workflows/node-ci.yml` (Node.js CI)
- `.github/workflows/python-ci.yml` (Python CI)
- `.github/workflows/go-ci.yml` (Go CI)
- `.github/workflows/rust-ci.yml` (Rust CI)
- `.github/workflows/codeql-analysis.yml` (CodeQL analysis)
- `.github/dependabot.yml` (Dependabot config)

These changes were created to provide language-specific continuous integration, dependency updates, and static analysis.
