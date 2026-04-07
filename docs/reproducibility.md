# Reproducibility Guide

## Reproducibility checklist

- Pin to a specific commit SHA.
- Use the repository Rust toolchain (`rust-toolchain.toml`).
- Run with clean working tree.
- Record environment:
  - OS and kernel
  - CPU model and core count
  - RAM
  - Rust and Python versions
- Keep scenario and strategy matrix unchanged unless documented.

## Deterministic run steps

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo run --release --bin scientific_validation
python scripts/analyze_validation.py
```

## Artifact retention

Retain at minimum:

- `artifacts/validation_report.csv`
- `artifacts/metrics.csv`
- `artifacts/prometheus.prom`
- UI/Grafana screenshots for qualitative support (optional but recommended)

## Threats to reproducibility

- Running different branches/tags without explicit declaration.
- Comparing debug and release binaries.
- Changing runtime mailbox capacity between compared runs.
- Mixing CI cloud runs and local runs without hardware disclosure.
