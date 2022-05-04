from typing import Dict, List, Optional
from pathlib import Path

import glob
import os
import json


def find_example_sizes(parent_dir: Path) -> Dict[str, int]:
    example_sizes: Dict[str, int] = {}

    for example_dir in (parent_dir / "examples").iterdir():

        if not example_dir.is_dir():
            print(f"{example_dir} is not a directory.")
            continue

        total_size = 0

        # For examples with multiple bundles, we add them together.
        for bundle in (example_dir / "dist").glob(f"*.wasm"):
            size = bundle.stat().st_size

            print(f"{bundle} has a size of {size}.")

            total_size += size

        if total_size > 0:
            example_sizes[example_dir.name] = total_size

    return example_sizes


def main() -> None:
    master_sizes = find_example_sizes(Path("yew-master"))
    pr_sizes = find_example_sizes(Path("current-pr"))

    example_names = sorted(set([*master_sizes.keys(), *pr_sizes.keys()]))

    joined_sizes = [(i, [master_sizes.get(i), pr_sizes.get(i)]) for i in example_names]

    size_cmp_info = {
        "sizes": joined_sizes,
        "issue_number": os.environ["ISSUE_NUMBER"],
    }

    with open(".SIZE_CMP_INFO", "w+") as f:
        f.write(json.dumps(size_cmp_info, indent=4))


if __name__ == "__main__":
    main()
