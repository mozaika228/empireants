# EmpireAnts Research Docs

This documentation layer defines EmpireAnts as a research platform with reproducible experiments, explicit architecture constraints, and trackable engineering decisions.

## Documentation map

- `docs/architecture.md`
  System-level architecture, data flow, and runtime boundaries.
- `docs/experiments.md`
  Experiment design protocol, scenario matrix, and KPI interpretation.
- `docs/reproducibility.md`
  Reproducibility checklist and exact run recipes.
- `docs/kpi-glossary.md`
  Formal metric definitions with units and caveats.
- `docs/adr/`
  Architecture Decision Records for high-impact design choices.

## Normative status

- The docs in this folder are normative for experimental claims.
- Any PR changing scientific results should update relevant docs.
- Any PR changing runtime semantics should update ADRs and architecture.
