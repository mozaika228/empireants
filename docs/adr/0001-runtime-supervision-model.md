# ADR 0001: Runtime Supervision Model

- Status: Accepted
- Date: 2026-04-07

## Context

Large ant populations may produce unstable behavior when individual decision paths fail or become invalid. A robust runtime must maintain forward progress under partial failures.

## Decision

Use a mailbox-driven actor runtime with:

- bounded queue capacity (backpressure),
- supervision event accounting,
- controlled per-tick restart limits,
- actor recovery fallback to nest.

## Consequences

- Improved operational resilience and measurable failure handling.
- Additional complexity in runtime metrics and test surface.
- Potential behavior divergence from purely biological interpretation in failure paths.
