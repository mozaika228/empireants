from __future__ import annotations

import csv
from pathlib import Path


def load_heatmap(path: Path) -> dict[tuple[int, int], float]:
    with path.open("r", encoding="utf-8") as handle:
        reader = csv.DictReader(handle)
        return {
            (int(row["x"]), int(row["y"])): float(row["food"]) + float(row["home"])
            for row in reader
        }


def render_ascii_heatmap(values: dict[tuple[int, int], float]) -> str:
    if not values:
        return ""

    max_x = max(x for x, _ in values.keys())
    max_y = max(y for _, y in values.keys())
    max_value = max(values.values()) or 1.0
    palette = " .:-=+*#%@"

    rows: list[str] = []
    for y in range(max_y + 1):
        row = []
        for x in range(max_x + 1):
            value = values.get((x, y), 0.0)
            idx = min(int((value / max_value) * (len(palette) - 1)), len(palette) - 1)
            row.append(palette[idx])
        rows.append("".join(row))
    return "\n".join(rows)


def main() -> None:
    heatmap = load_heatmap(Path("artifacts/pheromones.csv"))
    print(render_ascii_heatmap(heatmap))


if __name__ == "__main__":
    main()

