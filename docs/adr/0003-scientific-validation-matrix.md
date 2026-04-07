# ADR 0003: Scientific Validation Matrix

- Status: Accepted
- Date: 2026-04-07

## Context

Ad hoc experiments are hard to compare over time and across strategy variants.

## Decision

Adopt a fixed validation matrix:

- scenarios: `open_field`, `narrow_passages`, `obstacle_shift`,
- strategies: `basic`, `max_min`, `as_rank`, `ant_net`,
- consistent KPI export to `validation_report.csv`.

## Consequences

- Comparable baselines across commits and releases.
- Easier CI smoke validation and regression triage.
- Matrix may need future expansion; such expansion requires a new ADR.
