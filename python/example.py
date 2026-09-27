#!/usr/bin/env python3
"""Example usage of the OpenAir Python bindings.

Run from the repository root: ``uv run python python/example.py``
"""

from openair import parse_file, parse_string, write_string

OPENAIR_DATA = """
AC D
AN EXAMPLE CTR
AL GND
AH 5000 ft
DP 46:57:13 N 008:27:52 E
DP 46:57:46 N 008:30:41 E
DP 46:57:55 N 008:28:40 E
DP 46:57:13 N 008:27:52 E

AC R
AN EXAMPLE RESTRICTED
AL 1371.6m
AH FL100
V X=46:57:30 N 008:29:00 E
DC 2.5
"""


def main() -> None:
    """Parse from a string and a file, then write OpenAir back out."""
    airspaces = parse_string(OPENAIR_DATA)
    print(f"Parsed {len(airspaces)} airspace(s) from string:")
    for i, airspace in enumerate(airspaces, start=1):
        geom = airspace["geom"]
        print(f"  {i}. {airspace['name']} (class {airspace['class']})")
        print(f"     {airspace['lowerBound']} -> {airspace['upperBound']}")
        if geom["type"] == "Polygon":
            print(f"     Polygon with {len(geom['segments'])} segments")
        else:
            print(f"     Circle, radius {geom['radius']} NM")

    # Legacy AC tokens like R/Q/P/CTR can be moved into AY (class becomes UNC).
    normalized = parse_string(OPENAIR_DATA, normalize_legacy_classes=True)
    print(f"\nNormalized: class={normalized[1]['class']} type={normalized[1]['type']}")

    print("\nWritten back as OpenAir:\n")
    print(write_string(airspaces))

    airspaces_file = parse_file("example_data/Switzerland.txt")
    print(f"Parsed {len(airspaces_file)} airspace(s) from example_data/Switzerland.txt")


if __name__ == "__main__":
    main()
