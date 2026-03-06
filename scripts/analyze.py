from __future__ import annotations

import csv
from pathlib import Path


def analyze_metrics(path: Path) -> dict[str, float]:
    with path.open("r", encoding="utf-8") as handle:
        reader = csv.DictReader(handle)
        rows = list(reader)

    if not rows:
        raise ValueError("metrics file is empty")

    row = rows[-1]
    ant_count = int(row["ant_count"])
    food_collected = int(row["food_collected"])
    exploration_moves = int(row["exploration_moves"])
    steps = int(row["steps"])

    return {
        "steps": steps,
        "ant_count": ant_count,
        "food_collected": food_collected,
        "food_per_ant": food_collected / ant_count if ant_count else 0.0,
        "exploration_rate": exploration_moves / (steps * ant_count) if steps and ant_count else 0.0,
        "average_decision_score": float(row["average_decision_score"]),
        "active_food_sources": int(row["active_food_sources"]),
    }


def main() -> None:
    metrics = analyze_metrics(Path("artifacts/metrics.csv"))
    for key, value in metrics.items():
        print(f"{key}: {value}")


if __name__ == "__main__":
    main()

