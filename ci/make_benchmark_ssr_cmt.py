from typing import Dict, List, Optional, Tuple

import os
import json


header = "| Benchmark | Round | Min (ms) | Max (ms) | Mean (ms) | Standard Deviation |"
sep = "| --- | --- | --- | --- | --- | --- |"


def write_benchmark(lines: List[str], content: List[Dict[str, str]]) -> None:
    lines.append("<details>")
    lines.append("")
    lines.append(header)
    lines.append(sep)

    for i in content:
        lines.append(
            f"| {i['name']} | {i['round']} | {i['min']} | {i['max']} | {i['mean']} | {i['std_dev']} |"
        )

    lines.append("")
    lines.append("</details>")
    lines.append("")


def main() -> None:
    with open("benchmark-ssr/yew-master/tools/output.json") as f:
        master_content = json.loads(f.read())

    with open("benchmark-ssr/current-pr/tools/output.json") as f:
        pr_content = json.loads(f.read())

    lines: List[str] = []

    lines.append("### Benchmark - SSR")
    lines.append("")

    lines.append("#### Yew Master")
    lines.append("")

    write_benchmark(lines, master_content)

    lines.append("#### Pull Request")
    lines.append("")

    write_benchmark(lines, pr_content)

    output = "\n".join(lines)

    with open(os.environ["GITHUB_ENV"], "a+") as f:
        f.write(f"YEW_BENCH_SSR={json.dumps(output)}\n")


if __name__ == "__main__":
    main()
