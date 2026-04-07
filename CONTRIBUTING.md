# Contributing to EmpireAnts

## Principles

- Keep the project research-oriented and production-grade at the same time.
- Prefer deterministic behavior in the simulation core unless nondeterminism is explicitly required.
- Preserve modular boundaries between `world`, `ant`, `simulation`, `render`, and `scripts`.
- Document design tradeoffs when introducing performance-oriented complexity.

## Setup

```bash
rustup toolchain install stable
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
python -m py_compile scripts/analyze.py scripts/plot_heatmap.py scripts/experiments.py scripts/analyze_validation.py
docker compose config
```

## Pull request expectations

- Describe the problem, approach, and any simulation behavior changes.
- Add or update tests for logic changes in pheromones, decision making, or simulation stepping.
- Keep observability outputs stable unless there is a clear migration reason.
- Keep `/metrics` names backward-compatible when possible; treat changes as API changes.
- For performance-sensitive changes, include `scale_benchmark` before/after numbers.
- Runtime changes should include supervision/backpressure behavior notes and tests.
- Scientific claims should include scenario names, strategy matrix, and reported KPI definitions.
- Ensure CI passes all quality gates before requesting review.
- For product-facing changes, include docker stack impact and local `docker compose` smoke notes.
- Keep README and developer docs aligned with behavior and file layout.
- Avoid unrelated refactors in the same change set.

## Code guidelines

- Rust: favor explicit types and deterministic state transitions.
- Python: keep scripts dependency-light and suitable for batch experimentation.
- Performance claims should come with a benchmark, trace, or reproducible measurement note.
- New runtime, GPU, or distributed features should be feature-gated where possible.

## Suggested commit style

Use concise, imperative commit messages, for example:

- `feat: add pheromone diffusion metrics`
- `docs: expand architecture diagrams`
- `test: cover ant return-to-nest behavior`

## CI and governance

- CI workflow: `.github/workflows/ci.yml`
- PR template: `.github/pull_request_template.md`
- Issue templates: `.github/ISSUE_TEMPLATE/*`
- Owners: `.github/CODEOWNERS`
