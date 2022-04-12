from typing import Dict, List, Optional

import glob
import os
import json

def find_example_sizes(parent_dir: str) -> Dict[str, int]:
    example_sizes: Dict[str, int] = {}

    for example_dir in os.listdir(f"{parent_dir}/examples"):
        path = f"{parent_dir}/examples/{example_dir}"

        if not os.path.isdir(path):
            continue

        matches = glob.glob(f"{parent_dir}/examples/{example_dir}/dist/index*.wasm")

        if not matches:
            continue

        path = matches[0]

        example_sizes[example_dir] = os.path.getsize(path)

    return example_sizes

master_sizes = find_example_sizes("yew-master")
pr_sizes = find_example_sizes("current-pr")

example_names = sorted(set([*master_sizes.keys(), *pr_sizes.keys()]))

joined_sizes = [(i, [master_sizes.get(i), pr_sizes.get(i)]) for i in example_names]

size_cmp_info = {
    "sizes": joined_sizes,
    "issue_number": os.environ["ISSUE_NUMBER"],
}

with open(".SIZE_CMP_INFO", "w+") as f:
    f.write(json.dumps(size_cmp_info))
