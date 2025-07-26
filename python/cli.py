#!/usr/bin/env python3
"""CLI for parsing OpenAir files using the Python bindings.

Examples:
    Parse a file and print JSON to stdout:
        python -m python.cli example_data/Switzerland.txt

    Parse a file and pretty-print JSON:
        python -m python.cli example_data/Switzerland.txt --pretty

    Parse a file and write output to a file:
        python -m python.cli example_data/Switzerland.txt -o output.json
"""

import argparse
import json
import sys

from openair import parse_file


def main():
    """Parse an OpenAir file and output the result as JSON.

    This function parses command-line arguments to determine the input OpenAir file,
    output file (optional), and whether to pretty-print the JSON output. It then
    parses the OpenAir file and writes the resulting data as JSON to either stdout
    or the specified output file.

    Args:
        None

    Raises:
        SystemExit: If an error occurs during parsing or file operations, exits with code 1.
    """
    parser = argparse.ArgumentParser(
        description="Parse an OpenAir file and print the result as JSON."
    )
    parser.add_argument(
        "filepath", metavar="FILE", type=str, help="Path to the OpenAir file to parse."
    )
    parser.add_argument(
        "-o",
        "--output",
        metavar="OUTFILE",
        type=str,
        help="Write output to file instead of stdout.",
    )
    parser.add_argument(
        "--pretty", action="store_true", help="Pretty-print JSON output."
    )
    args = parser.parse_args()

    try:
        airspaces = parse_file(args.filepath)
        if args.pretty:
            json_str = json.dumps(airspaces, indent=2, ensure_ascii=False)
        else:
            json_str = json.dumps(airspaces, separators=(",", ":"), ensure_ascii=False)
        if args.output:
            with open(args.output, "w", encoding="utf-8") as f:
                f.write(json_str)
        else:
            print(json_str)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
