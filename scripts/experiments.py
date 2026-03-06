from __future__ import annotations

import itertools
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def run_experiment(step_count: int) -> str:
    result = subprocess.run(
        ["cargo", "run", "--quiet", "--", str(step_count)],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return result.stdout


def main() -> None:
    for step_count in itertools.product([100, 200, 400]):
        print(run_experiment(step_count[0]))


if __name__ == "__main__":
    main()

