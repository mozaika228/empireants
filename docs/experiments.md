# Experiment Protocol

## Objective

Provide consistent, reproducible comparison of ACO strategies and runtime behavior across predefined environments.

## Validation matrix

- Scenarios:
  - `open_field`
  - `narrow_passages`
  - `obstacle_shift`
- Strategies:
  - `basic`
  - `max_min`
  - `as_rank`
  - `ant_net`

Total default matrix size: `3 x 4 = 12` runs.

## Primary KPIs

- `food_collected`
- `first_food_step`
- `convergence_step`
- `exploration_efficiency`
- `throughput_food_per_second`
- `runtime_stability_score`

See `docs/kpi-glossary.md` for formal definitions.

## Statistical guidance

- For comparative claims, run at least 5 replicated matrices with identical software version and controlled environment.
- Report median and interquartile range where possible.
- Do not compare runs across mixed runtime configurations unless explicitly declared.

## Run recipe

```bash
cargo run --release --bin scientific_validation
python scripts/analyze_validation.py
```

Output artifact:

- `artifacts/validation_report.csv`

## Reporting template

Include the following in any report:

1. Commit SHA and date.
2. Hardware/OS summary.
3. Exact command lines.
4. KPI table by `scenario x strategy`.
5. Interpretation limits and potential confounders.
