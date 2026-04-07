# KPI Glossary

## `food_collected`

- Meaning: total food units returned to nest.
- Unit: count (integer, monotonic).

## `first_food_step`

- Meaning: first tick where `food_collected > 0`.
- Unit: tick index.
- Lower is better for discovery speed.

## `convergence_step`

- Meaning: heuristic convergence proxy used in validation suite.
- Unit: tick index.
- Interpretation: lower suggests faster transition to productive foraging.

## `exploration_efficiency`

- Formula: `food_collected / max(exploration_moves, 1)`.
- Unit: food per exploration move.
- Higher is better.

## `throughput_food_per_second`

- Formula: `food_collected / simulation_elapsed_seconds`.
- Unit: food per second.
- Sensitive to machine performance and runtime configuration.

## `runtime_stability_score`

- Formula: `1 / (1 + dropped + restarts + supervision_events)`.
- Unit: dimensionless in `(0, 1]`.
- Higher is better.
- Note: this is an operational stability proxy, not a biological metric.
