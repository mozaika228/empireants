from __future__ import annotations

import csv
from collections import defaultdict
from pathlib import Path


def load_rows(path: Path) -> list[dict[str, str]]:
    with path.open("r", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def summarize_by_strategy(rows: list[dict[str, str]]) -> list[tuple[str, float, float, float]]:
    grouped: dict[str, list[dict[str, str]]] = defaultdict(list)
    for row in rows:
        grouped[row["strategy"]].append(row)

    summary: list[tuple[str, float, float, float]] = []
    for strategy, items in grouped.items():
        n = len(items)
        mean_food = sum(float(item["food_collected"]) for item in items) / n
        mean_convergence = sum(float(item["convergence_step"]) for item in items) / n
        mean_stability = sum(float(item["runtime_stability_score"]) for item in items) / n
        summary.append((strategy, mean_food, mean_convergence, mean_stability))
    summary.sort(key=lambda row: (-row[1], row[2], -row[3]))
    return summary


def main() -> None:
    rows = load_rows(Path("artifacts/validation_report.csv"))
    for strategy, mean_food, mean_convergence, mean_stability in summarize_by_strategy(rows):
        print(
            f"strategy={strategy} mean_food={mean_food:.3f} "
            f"mean_convergence={mean_convergence:.2f} mean_stability={mean_stability:.4f}"
        )


if __name__ == "__main__":
    main()
