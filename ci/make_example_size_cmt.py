from typing import Dict, List, Optional, Tuple

import os
import json


header = "| examples | master (KB) | pull request (KB) | diff (KB) | diff (%) |"
sep = "| --- | --- | --- | --- | --- |"


def format_size(size: Optional[int]) -> str:
    if size is None:
        return "N/A"

    if size == 0:
        return "0"

    return f"{size / 1024:.3f}"


def format_diff_size(
    master_size: Optional[int], pr_size: Optional[int]
) -> Tuple[str, str, bool]:
    if master_size is None or pr_size is None:
        return ("N/A", "N/A", False)

    diff = pr_size - master_size

    if diff == 0:
        return ("0", "0.000%", False)

    diff_percent = diff / master_size

    return (f"{diff / 1024:+.3f}", f"{diff_percent:+.3%}", abs(diff_percent) >= 0.01)


def main() -> None:
    with open("size-cmp-info/.SIZE_CMP_INFO") as f:
        content = json.loads(f.read())

    joined_sizes = content["sizes"]
    issue_number = content["issue_number"]

    lines: List[str] = []
    significant_lines: List[str] = []

    lines.append("### Size Comparison")
    lines.append("")
    lines.append("<details>")
    lines.append("")
    lines.append(header)
    lines.append(sep)

    for (i, sizes) in joined_sizes:
        (master_size, pr_size) = sizes

        master_size_str = format_size(master_size)
        pr_size_str = format_size(pr_size)

        (diff_str, diff_percent, diff_significant) = format_diff_size(
            master_size, pr_size
        )

        line_str = (
            f"| {i} | {master_size_str} | {pr_size_str} | "
            f"{diff_str} | {diff_percent} |"
        )

        lines.append(line_str)

        if diff_significant:
            significant_lines.append(line_str)

    lines.append("")
    lines.append("</details>")
    lines.append("")

    if significant_lines:

        if len(significant_lines) == 1:
            lines.append("⚠️ The following example has changed its size significantly:")
        else:
            lines.append(
                "⚠️ The following examples have changed their size significantly:"
            )
        lines.append("")

        lines.append(header)
        lines.append(sep)
        lines.extend(significant_lines)

    else:
        lines.append("✅ None of the examples has changed their size significantly.")

    output = "\n".join(lines)

    with open(os.environ["GITHUB_ENV"], "a+") as f:
        f.write(f"YEW_EXAMPLE_SIZES={json.dumps(output)}\n")
        f.write(f"PR_NUMBER={issue_number}\n")


if __name__ == "__main__":
    main()
