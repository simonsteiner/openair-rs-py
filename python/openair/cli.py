"""Command-line interface: parse an OpenAir file and print it as JSON.

Examples:
    openair example_data/Switzerland.txt
    openair example_data/Switzerland.txt --pretty
    openair example_data/Switzerland.txt -o output.json
    python -m openair example_data/Switzerland.txt
"""

import argparse
import json
import sys
from collections.abc import Sequence

from openair import __version__, parse_file


def main(argv: Sequence[str] | None = None) -> int:
    """Run the CLI and return the process exit code."""
    parser = argparse.ArgumentParser(
        prog="openair",
        description="Parse an OpenAir file and print the result as JSON.",
    )
    parser.add_argument("filepath", metavar="FILE", help="OpenAir file to parse.")
    parser.add_argument(
        "-o",
        "--output",
        metavar="OUTFILE",
        help="Write output to file instead of stdout.",
    )
    parser.add_argument(
        "--pretty", action="store_true", help="Pretty-print JSON output."
    )
    parser.add_argument(
        "--normalize-legacy-classes",
        action="store_true",
        help="Move legacy AC types (R, Q, P, CTR, ...) into AY and set class UNC.",
    )
    parser.add_argument("--version", action="version", version=__version__)
    args = parser.parse_args(argv)

    try:
        airspaces = parse_file(
            args.filepath, normalize_legacy_classes=args.normalize_legacy_classes
        )
    except (OSError, ValueError) as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1

    if args.pretty:
        json_str = json.dumps(airspaces, indent=2, ensure_ascii=False)
    else:
        json_str = json.dumps(airspaces, separators=(",", ":"), ensure_ascii=False)

    if args.output:
        with open(args.output, "w", encoding="utf-8") as f:
            f.write(json_str)
    else:
        print(json_str)
    return 0


if __name__ == "__main__":
    sys.exit(main())
