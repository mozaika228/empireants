# ADR 0002: Observability Contract

- Status: Accepted
- Date: 2026-04-07

## Context

EmpireAnts serves both research and product-like operational use cases. Metrics must be consumable by Prometheus/Grafana and stable enough for dashboard continuity.

## Decision

Expose a Prometheus-compatible metrics contract via:

- file export artifacts (`prometheus.prom`),
- live HTTP endpoint (`/metrics`),
- stable metric naming for runtime and simulation KPIs.

## Consequences

- Enables live dashboards and automated monitoring pipelines.
- Changes to metric names become API-level breaking changes.
- Requires governance in docs and CI discipline.
